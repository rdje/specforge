use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::extractor::{
    ExtractionContext, ExtractionManifest, Extractor, run_surface, run_surface_concat,
};
use crate::ir::prior_memory::{
    ActorTaxonomyRole, CorpusMemory, ProtocolFamily, is_meaningful_actor_term,
    normalize_actor_term, normalized_text_contains_term,
};
use crate::ir::semantic::InterfaceSignalSemanticRole;
use crate::ir::source::{
    ActorSignalRelation, ConditionalRuleRecord, RegisterFieldEnumRecord, RegisterFieldRecord,
    RegisterRecord, RelationKind, SignalConstraintKind, SignalConstraintRecord,
    StructuredTableCellRecord, StructuredTableRecord, TimingConstraintRecord,
    ValidationReportRecord, WidthHint,
};
use crate::ir::source::{
    AutomationConfidence, DiagramKind, NormalizationStatus, SectionKind, SourceIr, TableKind,
    VisualAsset, VisualAssetKind, document_key,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StatementClass {
    SourceFact,
    /// Sentence explicitly constraining a hardware signal to a specific logic value or
    /// protocol state. These are the most precise and directly actionable constraints.
    /// Examples (shapes, not any spec's vocabulary):
    ///   `<signal> must be <value> when <condition>`,
    ///   `<signal> shall remain <value> throughout the burst`,
    ///   `<signal> must be asserted when transfer is accepted`.
    SignalValueConstraint,
    /// Sentence with `shall`/`must`/`shall not`/`required`/`prohibited` in a non-boilerplate
    /// section. General normative behavioral requirements not covered by a more specific class.
    NormativeStatement,
    /// Sentence containing cycle counts, setup/hold time references, or latency bounds.
    /// Examples: "within 2 cycles", "tSU setup time", "at least N clock periods".
    TimingConstraint,
    /// Conditional behavioral sentence: `when X, Y shall...` / `if A then B`.
    ConditionalRule,
    DerivedRule,
    LocalDesignDecision,
    ExplicitAbstraction,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceModality {
    Text,
    Visual,
    Mixed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VisualEvidenceRole {
    Normative,
    Explanatory,
    Illustrative,
    Ambiguous,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VisualObservationKind {
    Caption,
    FigureReference,
    Description,
    Classification,
    ChartExtraction,
    OcrTranscription,
    TableTranscription,
    FormulaTranscription,
    /// Structured extraction from a timing diagram via VLM.
    /// `text` field contains JSON: `{"signals":[{"name":str,"values":[{"cycle":str,"state":str}]}],"annotations":[str]}`
    TimingDiagramExtraction,
    /// Structured extraction from a state machine diagram via VLM.
    /// `text` field contains JSON: `{"states":[{"name":str,"is_initial":bool}],"transitions":[{"from":str,"to":str,"guard":str}]}`
    StateMachineExtraction,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLinkKind {
    Cites,
    Describes,
    DerivedFrom,
    Supports,
    Refines,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub source_ir_path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_memory_path: Option<PathBuf>,
    pub artifact_layout: EvidenceArtifactLayout,
    pub document_identity: EvidenceDocumentIdentity,
    pub section_anchors: Vec<SectionAnchor>,
    pub evidence_spans: Vec<EvidenceSpan>,
    pub visual_evidence: Vec<VisualEvidenceItem>,
    pub evidence_links: Vec<EvidenceLink>,
    pub extracted_statements: Vec<ExtractedStatement>,
    /// Register records synthesized from `register_map` tables in `SourceIR`.
    #[serde(default)]
    pub register_records: Vec<RegisterRecord>,
    /// Timing constraint records synthesized from `timing_parameter` tables in `SourceIR`.
    #[serde(default)]
    pub timing_constraints: Vec<TimingConstraintRecord>,
    /// Level 2 NLP: structured records extracted from `SignalValueConstraint` sentences.
    #[serde(default)]
    pub signal_constraints: Vec<SignalConstraintRecord>,
    /// Level 2 NLP: structured records extracted from `ConditionalRule` sentences.
    #[serde(default)]
    pub conditional_rules: Vec<ConditionalRuleRecord>,
    /// CVE-PROSE-EXTRACTION: `ActorContract`s extracted from prose by the
    /// `extract-contracts` command (Qwen via Ollama), each fails-closed parsed
    /// (`parse_constrained_contract`) + entailment-gated
    /// (`apply_entailment_to_contract`). Additive + serde-skip-if-empty ⇒ zero
    /// artifact churn until the producer runs; `SemanticIr::build` folds these
    /// into `actor_contracts` before fusion/fidelity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extracted_contracts: Vec<crate::ir::contract::ActorContract>,
    /// CVE-PROSE-EXTRACTION: producer-time extraction stats (the
    /// `schema_rejects` count cannot be derived from survivors). Carried
    /// `EvidenceIR → SemanticIR → IntentIR` for the `validate` `constrained:`
    /// block. `None` until the producer runs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constrained_extraction_stats: Option<crate::ir::cve::ConstrainedExtractionStats>,
    /// Provenance links from table-synthesized signal declarations back to SourceIR tables.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub table_signal_declaration_provenance: Vec<TableSignalDeclarationProvenanceRecord>,
    /// Resolved signal polarity facts recovered from prose/table evidence.
    /// These remain explicit so downstream layers can interpret asserted/deasserted
    /// semantics without blindly collapsing them to HIGH/LOW.
    #[serde(default)]
    pub signal_polarities: Vec<SignalPolarityRecord>,
    /// Explicit conflicts where polarity evidence disagrees across prose/table sources.
    /// These conflicts stay visible instead of silently collapsing into a neutral fallback.
    #[serde(default)]
    pub signal_polarity_conflicts: Vec<SignalPolarityConflictRecord>,
    /// Typed semantic hints mined from tables, prose, and alias-grounded prose descriptions,
    /// kept explicit so later semantic stages can use meaning-based roles instead of literal
    /// signal spelling alone.
    #[serde(default)]
    pub signal_semantic_hints: Vec<SignalSemanticHintRecord>,
    /// Explicit conflicts where semantic-role evidence assigns incompatible roles to the same
    /// signal. These conflicts stay visible instead of silently collapsing into an ambiguous
    /// dual-tag fallback.
    #[serde(default)]
    pub signal_semantic_conflicts: Vec<SignalSemanticConflictRecord>,
    /// Tier 2 Knowledge Graph: actor–signal relation triples extracted from prose verb phrases.
    /// Each record encodes (actor, drives|reads, signal) derived from sentences like
    /// "PREADY is driven by the slave" or "The Manager drives HTRANS".
    /// Together these form the structural knowledge graph of the specification.
    #[serde(default)]
    pub actor_signal_relations: Vec<ActorSignalRelation>,
    /// Form 2 signal alias map
    /// extracts a constraint from a sentence where the signal name doesn't appear
    /// literally (e.g. "address bus" → "HADDR").  Persisted across runs so the
    /// alias vocabulary accumulates.  Applied at the start of each enrichment pass
    /// to reclassify remaining NormativeStatements without LLM calls.
    #[serde(default)]
    pub signal_alias_map: BTreeMap<String, String>,
    #[serde(default)]
    pub validation_reports: Vec<ValidationReportRecord>,
    /// R15c: report of the monotone anchored-rescan loop that built this
    /// EvidenceIR. Makes the convergent extraction first-class — how many passes
    /// ran, how many *genuinely new* (deduplicated) facts each pass recovered,
    /// and whether the loop stabilized (`converged`) or stopped at its pass cap.
    /// `None` only for artifacts built before this field existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convergence_report: Option<EvidenceConvergenceReport>,
    /// PER-EXTRACTOR-FACT-TAGGING: per-extractor fact observations (which tier
    /// found each fact, under a canonical key) — the capture–recapture recall
    /// gauge precondition. Pattern finds are tagged at `build`; Nlp finds at
    /// `nlp-enrich` (pre-dedup, so overlaps are recorded). Empty for artifacts
    /// built before this field existed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fact_provenance: Vec<FactProvenanceRecord>,
    /// SWD-SERIAL-EXTRACTION.3: typed serial-frame fields recovered from a serial protocol's
    /// frame description (the SWD packet request / acknowledge / data phases). Each field carries
    /// its bit-width (from a `NAME[hi:lo]` range or a stated bit count) and, for the ACK field, the
    /// response values (OK/WAIT/FAULT). Scoped to serial-protocol context so parallel-bus specs are
    /// untouched. Empty (serde-skipped) for non-serial documents.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub serial_frame_fields: Vec<SerialFrameField>,
    /// SWD-SERIAL-EXTRACTION.4: the protocol FSM states (the JTAG TAP / SWD line state machine). The
    /// FSM is critical to understanding/implementing SWD/JTAG and is what FSMGen ultimately builds.
    /// Empty (serde-skipped) for documents without a described state machine.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_states: Vec<ProtocolStateRecord>,
    /// PDF-VARIANT-DIGESTION.3b: protocol ACTORS/AGENTS a spec defines in prose (controller, target, …).
    /// Empty (serde-skipped) for documents that do not define agents in prose.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_actors: Vec<ProtocolActorRecord>,
    /// SWD-SERIAL-EXTRACTION.4b: the SWD packet-protocol operations — the response-branched phase
    /// sequences (OK → 3-phase request/ack/data; WAIT/FAULT → 2-phase request/ack) + turnaround model.
    /// Empty (serde-skipped) for non-serial documents.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub swd_operations: Vec<SwdOperation>,
    /// EXTRACTOR-ARCHITECTURE.8: the per-surface extraction run manifest — which extractors were eligible /
    /// fired / produced / kept, for each surface that runs through the unified `run_surface(_concat)` driver
    /// (FSM, semantic-hints, registers, actors). A per-document **behavioral fingerprint** (the substrate
    /// `CORPUS-PATTERN-REUSE` clusters on), and the inspectable "which extractors fired" view the `.1` audit
    /// found missing. Additive + serde-default for backward compatibility with older persisted artifacts.
    #[serde(default)]
    pub extraction_manifest: ExtractionManifest,
}

/// SWD-SERIAL-EXTRACTION.4b: one SWD packet-protocol operation variant — a response branch of the packet
/// FSM. OK responses carry a data phase (3 phases: request → acknowledge → data); WAIT/FAULT do not
/// (2 phases). Derived from "a successful `<read|write>` operation consists of three phases" /
/// "A `<WAIT|FAULT>` response … consists of two phases" prose (B4.2).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SwdOperation {
    /// Stable id, e.g. `swd_operation_0001`.
    pub operation_id: String,
    /// The acknowledge response that selects this branch: `OK` / `WAIT` / `FAULT`.
    pub response: String,
    /// `read` or `write`; `None` when the operation applies to "a read or write" request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// Number of packet phases (2 = request+acknowledge; 3 = request+acknowledge+data).
    pub phase_count: u32,
    /// Whether a data-transfer phase follows the acknowledge (true for OK; false for WAIT/FAULT
    /// unless overrun detection is enabled).
    pub has_data_phase: bool,
    /// Whether a turnaround period sits between the acknowledge and data phases (true for write —
    /// host drives WDATA; false for read — target drives both ack and RDATA). `None` for 2-phase.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turnaround_before_data: Option<bool>,
    /// Statements that evidenced this operation.
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
}

/// SWD-SERIAL-EXTRACTION.3: one field of a serial protocol frame (e.g. SWD `ACK[2:0]`, `WDATA[0:31]`,
/// the `APnDP`/`RnW` request bits). The serial frame is a SEQUENCE protocol, not a clocked-edge rule,
/// so it is a distinct typed surface from `signal_constraints`/`temporal_rules`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SerialFrameField {
    /// Stable id, e.g. `serial_field_0003`.
    pub field_id: String,
    /// The field name as written (`ACK`, `WDATA`, `APnDP`, `RnW`, `A`, `DATAIN`).
    pub name: String,
    /// Bit-width in the frame (from `NAME[hi:lo]` ⇒ |hi-lo|+1, or a stated count). `None` if unstated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_width: Option<u32>,
    /// The literal bit range `[high, low]` when the field was written as `NAME[hi:lo]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_range: Option<(u32, u32)>,
    /// The frame phase this field belongs to (request / acknowledge / data), inferred from context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<SerialFramePhase>,
    /// Which actor drives the bidirectional data wire (SWDIO) during this field — derived from the
    /// spec's "from the `<actor>` to the `<actor>`" / "`<actor>` to `<actor>`, following a read/write
    /// request" prose (the host samples whatever the target drives). `None` if not stated. (`.4c`)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swdio_direction: Option<SwdioDirection>,
    /// Order of this field within the frame sequence (request bits → acknowledge → data), assigned
    /// by phase rank then first appearance. `None` if the field has no resolved phase. (`.3b`)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
    /// Response values for a response field — the ACK field carries OK / WAIT / FAULT. (`.3b`)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub response_values: Vec<String>,
    /// Statements that evidenced this field.
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
}

/// The phase of a serial transaction a frame field belongs to.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SerialFramePhase {
    /// The host-to-target packet request (APnDP, RnW, address, parity).
    Request,
    /// The target-to-host acknowledge response (ACK: OK/WAIT/FAULT).
    Acknowledge,
    /// The data transfer phase (read/write data + parity).
    Data,
}

/// Which actor drives the bidirectional serial data wire (SWDIO) during a frame field/phase. The
/// other actor samples it. SWD-SERIAL-EXTRACTION.4c.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SwdioDirection {
    /// The host (external debugger) drives the wire; the target samples (request + write data).
    HostDrives,
    /// The target (DP) drives the wire; the host samples (acknowledge + read data).
    TargetDrives,
}

/// SWD-SERIAL-EXTRACTION.4: one state of a protocol FSM. SWD and JTAG are *defined* by a state machine
/// (the JTAG TAP controller / SWD line protocol) — and the FSM is the heart of SpecForge's purpose
/// (IntentIR → `.isf` → FSMGen builds the `.fsm`). Each record is a named state with its machine and
/// per-state action. Transitions (the TMS-driven edges) are `SWD-SERIAL-EXTRACTION.4b`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolStateRecord {
    /// Stable id, e.g. `protocol_state_0003`.
    pub state_id: String,
    /// The state machine this state belongs to (e.g. `DBGTAPSM`), when named.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    /// The state name as written (e.g. `Shift-DR`, `Run-Test/Idle`, `Test-Logic-Reset`).
    pub state_name: String,
    /// What happens in this state ("data is transferred from DBGTDI to DBGTDO …"), when stated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Statements that evidenced this state.
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
}

/// PDF-VARIANT-DIGESTION.3b — a protocol ACTOR/AGENT a spec DEFINES in prose ("A controller is the device
/// which initiates a data transfer …", "any device addressed is considered a target"). Grounds the agent
/// model from prose, not only as the inferred subject of a relation. General agent-definition grammar; no
/// chip-spec names (ADR 0006).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolActorRecord {
    /// Stable id, e.g. `protocol_actor_0001`.
    pub actor_id: String,
    /// The agent / role name as written (e.g. `controller`, `target`, `host`).
    pub name: String,
    /// The defining clause, when captured ("the device which initiates a data transfer …").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// Statements that evidenced this actor.
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
}

/// R15c: typed accounting for the EvidenceIR convergent anchored-rescan loop.
///
/// The loop seeds from one-shot extraction, then repeatedly uses known signals /
/// values as anchors to rescan tables (and prose-derived relations) for more
/// facts, stopping when a pass discovers no new *deduplicated* statement texts.
/// This record exposes that behavior instead of leaving it implicit, so users
/// can see genuine knowledge growth (not duplicate vector growth) and whether
/// convergence was actually reached.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceConvergenceReport {
    /// Number of loop passes actually executed (includes the terminal pass that
    /// discovered nothing new, when the loop converged).
    pub passes_run: usize,
    /// The pass cap for this document (`structured_tables.len().max(1) + 4`).
    pub max_passes: usize,
    /// Genuinely-new (deduplicated) statements discovered per pass, in order.
    #[serde(default)]
    pub new_facts_per_pass: Vec<usize>,
    /// Sum of `new_facts_per_pass` — total anchored facts recovered beyond the seed.
    pub total_new_facts: usize,
    /// `true` if the loop stopped because a pass found nothing new (stabilized);
    /// `false` if it exhausted `max_passes` while still discovering facts
    /// (convergence not proven — the anchored rescan may be incomplete).
    pub converged: bool,
}

/// The independent extractor families a fact can come from. Recorded per fact so
/// the capture–recapture recall gauge (`INTENT-COMPLETENESS-RESEARCH.5`) can
/// estimate the unseen population from the overlap of what each found.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtractorTier {
    /// Tier-1/2 structural pattern extraction (prose patterns + table synthesis)
    /// run during `EvidenceIR` build.
    Pattern,
    /// Tier-3 NLP via a local LLM (`nlp-enrich`).
    Nlp,
    /// Tier-3 visual via a VLM (`enrich`).
    Vlm,
}

/// The kind of fact a [`FactProvenanceRecord`] refers to (extensible; the first
/// tagged type is `SignalConstraint`, which two independent tiers both produce).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FactKind {
    SignalConstraint,
    ActorSignalRelation,
}

/// One per-extractor fact observation: "extractor `producer` found a fact of
/// `fact_kind` whose canonical identity is `canonical_key`." Recorded *before*
/// merge/dedup so a fact found by two tiers appears once per tier (the overlap
/// capture–recapture needs). The canonical key is normalized so the SAME fact
/// found by different tiers yields the SAME key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FactProvenanceRecord {
    pub producer: ExtractorTier,
    pub fact_kind: FactKind,
    pub canonical_key: String,
}

/// Canonical identity of a signal constraint for cross-extractor overlap
/// detection: normalized subject signal + constraint kind + target value, so the
/// same constraint extracted by the pattern tier and the NLP tier produces the
/// same key regardless of id / source-text differences.
pub fn signal_constraint_fact_key(constraint: &SignalConstraintRecord) -> String {
    format!(
        "{}|{:?}|{}",
        constraint.subject_signal.trim().to_ascii_uppercase(),
        constraint.constraint_kind,
        constraint
            .target_value
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_ascii_uppercase(),
    )
}

/// Canonical identity of an actor-signal relation for cross-extractor overlap
/// detection: normalized actor + relation kind + signal, so the same edge
/// extracted by the pattern tier and the `signal-resolve` LLM produces the same
/// key regardless of id / source differences.
pub fn actor_signal_relation_fact_key(relation: &ActorSignalRelation) -> String {
    format!(
        "{}|{:?}|{}",
        relation.actor_name.trim().to_ascii_uppercase(),
        relation.relation,
        relation.signal_name.trim().to_ascii_uppercase(),
    )
}

/// Count the DISTINCT extractor tiers (`Pattern`/`Nlp`/`Vlm`) that recorded each fact, keyed by
/// its provenance `canonical_key` — the agreement-confidence signal (a fact two tiers independently
/// found is more trustworthy than one). A key absent from the map was seen by a single tier.
pub fn tier_count_by_fact_key(
    provenance: &[FactProvenanceRecord],
) -> std::collections::HashMap<String, usize> {
    let idx = |t: &ExtractorTier| match t {
        ExtractorTier::Pattern => 0,
        ExtractorTier::Nlp => 1,
        ExtractorTier::Vlm => 2,
    };
    let mut seen: std::collections::HashMap<String, [bool; 3]> = std::collections::HashMap::new();
    for p in provenance {
        seen.entry(p.canonical_key.clone()).or_default()[idx(&p.producer)] = true;
    }
    seen.into_iter()
        .map(|(k, flags)| (k, flags.iter().filter(|f| **f).count().max(1)))
        .collect()
}

#[derive(Debug, Clone)]
struct EvidencePriorGuidance {
    prior_memory_path: PathBuf,
    corpus_memory: CorpusMemory,
    protocol_family: ProtocolFamily,
}

impl EvidenceIr {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(AppError::MissingPath(path.to_path_buf()));
        }

        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn build(source_ir_path: &Path, artifact_base_root: &Path) -> Result<Self> {
        Self::build_with_prior_memory(source_ir_path, artifact_base_root, None)
    }

    pub fn build_with_prior_memory(
        source_ir_path: &Path,
        artifact_base_root: &Path,
        prior_memory_path: Option<&Path>,
    ) -> Result<Self> {
        let source_ir_path = canonicalize_existing_path(source_ir_path)?;
        let source_ir = SourceIr::load_from_path(&source_ir_path)?;
        let prior_guidance = load_evidence_prior_guidance(prior_memory_path, &source_ir)?;

        if !matches!(
            source_ir.normalization_plan.status,
            NormalizationStatus::Ready
        ) {
            return Err(AppError::InvalidStageArtifact(format!(
                "SourceIR at {} must have normalization status `ready` before building EvidenceIR",
                source_ir_path.display()
            )));
        }

        let promoted_markdown_path = source_ir
            .normalization_plan
            .promoted_markdown_path
            .as_ref()
            .ok_or_else(|| {
                AppError::InvalidStageArtifact(format!(
                    "SourceIR at {} is missing a promoted_markdown_path",
                    source_ir_path.display()
                ))
            })?;
        let promoted_markdown_path = canonicalize_existing_path(promoted_markdown_path)?;
        let parsed_markdown = parse_markdown(&promoted_markdown_path)?;

        let artifact_root = artifact_base_root.join(&source_ir.document_identity.document_key);
        let evidence_ir_path = artifact_root.join("evidence_ir.json");
        let artifact_layout = EvidenceArtifactLayout {
            artifact_root,
            evidence_ir_path,
        };
        let document_identity = EvidenceDocumentIdentity {
            document_key: source_ir.document_identity.document_key.clone(),
            display_name: source_ir.document_identity.display_name.clone(),
        };

        let mut section_anchors = build_section_anchors(
            &parsed_markdown.headings,
            parsed_markdown.total_lines,
            &document_identity.display_name,
        );
        for anchor in &mut section_anchors {
            anchor.source_path = promoted_markdown_path.clone();
        }
        let mut section_pages: Vec<Vec<u32>> = vec![Vec::new(); section_anchors.len()];

        let visual_motif_signal_names =
            collect_signal_names_from_tables(&source_ir, prior_guidance.as_ref())
                .into_iter()
                .collect::<BTreeSet<_>>();
        let visual_motif_actor_names =
            collect_known_actor_names_for_semantic_hints(&source_ir, &[], prior_guidance.as_ref());
        let mut visual_evidence = build_visual_evidence_items(
            &source_ir.visual_assets,
            prior_guidance.as_ref(),
            &visual_motif_signal_names,
            &visual_motif_actor_names,
        );
        let asset_id_to_visual_index: HashMap<String, usize> = visual_evidence
            .iter()
            .enumerate()
            .map(|(idx, item)| (item.asset_id.clone(), idx))
            .collect();

        // Inject VLM-derived observations from SourceIR visual asset notes.
        // These are written by `specforge enrich --vlm-provider <provider>` and carry
        // typed diagram extractions that downstream SemanticIR parses into records.
        inject_vlm_observations(
            &source_ir.visual_assets,
            &mut visual_evidence,
            &asset_id_to_visual_index,
        );
        let asset_id_to_visual_evidence_id: HashMap<String, String> = visual_evidence
            .iter()
            .map(|item| (item.asset_id.clone(), item.evidence_id.clone()))
            .collect();
        let caption_key_to_asset_id = build_caption_key_index(&source_ir.visual_assets);
        let reference_patterns = build_reference_patterns(&source_ir.visual_assets);
        let asset_id_to_page: HashMap<String, u32> = source_ir
            .visual_assets
            .iter()
            .filter_map(|asset| {
                asset
                    .page_id
                    .as_deref()
                    .and_then(page_number_from_page_id)
                    .map(|page| (asset.asset_id.clone(), page))
            })
            .collect();

        let mut evidence_spans = Vec::new();
        let mut evidence_links = Vec::new();
        let mut extracted_statements = Vec::new();
        let mut caption_support: HashMap<String, Vec<String>> = HashMap::new();
        let mut reference_support: HashMap<String, Vec<ReferenceSupport>> = HashMap::new();

        let mut link_counter = 1usize;
        let mut statement_counter = 1usize;

        for (span_counter, block) in (1usize..).zip(parsed_markdown.blocks.iter()) {
            let span_id = format!("span_{span_counter:04}");

            let section_index = section_index_for_line(&section_anchors, block.line_start);
            let caption_asset_id = caption_key_to_asset_id
                .get(&normalize_text_key(&block.text))
                .cloned();

            let mut linked_asset_ids = Vec::new();
            let mut source_page = None;
            let mut note = None;

            if let Some(asset_id) = caption_asset_id.clone() {
                linked_asset_ids.push(asset_id.clone());
                source_page = asset_id_to_page.get(&asset_id).copied();
                note = Some("caption".to_string());
            } else {
                for reference_hit in extract_reference_hits(&block.text, &reference_patterns) {
                    linked_asset_ids.push(reference_hit.asset_id.clone());
                    reference_support
                        .entry(reference_hit.asset_id.clone())
                        .or_default()
                        .push(ReferenceSupport {
                            supporting_span_id: span_id.clone(),
                            display_reference_text: reference_hit.display_reference_text,
                        });
                }
            }

            let modality = if linked_asset_ids.is_empty() {
                EvidenceModality::Text
            } else {
                EvidenceModality::Mixed
            };

            if let Some(asset_id) = caption_asset_id {
                let visual_evidence_id = asset_id_to_visual_evidence_id.get(&asset_id).ok_or_else(
                    || {
                        AppError::InvalidStageArtifact(format!(
                            "visual asset `{asset_id}` referenced by caption span is missing a matching visual evidence item"
                        ))
                    },
                )?;
                caption_support
                    .entry(asset_id)
                    .or_default()
                    .push(span_id.clone());
                evidence_links.push(EvidenceLink {
                    link_id: format!("link_{link_counter:04}"),
                    from_evidence_span_id: span_id.clone(),
                    to_visual_evidence_id: visual_evidence_id.clone(),
                    relation: EvidenceLinkKind::Describes,
                });
                link_counter += 1;
            } else {
                for asset_id in &linked_asset_ids {
                    let visual_evidence_id =
                        asset_id_to_visual_evidence_id.get(asset_id).ok_or_else(|| {
                            AppError::InvalidStageArtifact(format!(
                                "visual asset `{asset_id}` referenced by text span is missing a matching visual evidence item"
                            ))
                        })?;
                    evidence_links.push(EvidenceLink {
                        link_id: format!("link_{link_counter:04}"),
                        from_evidence_span_id: span_id.clone(),
                        to_visual_evidence_id: visual_evidence_id.clone(),
                        relation: EvidenceLinkKind::Cites,
                    });
                    link_counter += 1;
                }
            }

            let visual_asset_id = if linked_asset_ids.len() == 1 {
                linked_asset_ids.first().cloned()
            } else {
                None
            };
            evidence_spans.push(EvidenceSpan {
                span_id: span_id.clone(),
                modality,
                source_path: promoted_markdown_path.clone(),
                source_page,
                line_start: Some(block.line_start),
                line_end: Some(block.line_end),
                visual_asset_id,
                note,
            });

            if let Some(section_index) = section_index
                && let Some(source_page) = source_page
            {
                section_pages[section_index].push(source_page);
            }

            let related_visual_evidence_ids: Vec<String> = linked_asset_ids
                .iter()
                .filter_map(|asset_id| asset_id_to_visual_evidence_id.get(asset_id).cloned())
                .collect();
            // Layer A: suppress NormativeStatement for sentences inside boilerplate sections
            // (introduction, legal, revision history, etc.) — these are compliance obligations,
            // not hardware behavioral constraints.
            let section_is_boilerplate = section_index
                .map(|idx| is_boilerplate_section_title(&section_anchors[idx].title))
                .unwrap_or(false);
            let raw_class = classify_statement(&block.text);
            let class = if section_is_boilerplate
                && matches!(raw_class, StatementClass::NormativeStatement)
            {
                StatementClass::SourceFact
            } else {
                raw_class
            };

            extracted_statements.push(ExtractedStatement {
                statement_id: format!("statement_{statement_counter:04}"),
                class,
                modality: if related_visual_evidence_ids.is_empty() {
                    EvidenceModality::Text
                } else {
                    EvidenceModality::Mixed
                },
                text: block.text.clone(),
                evidence_span_ids: vec![span_id],
                related_visual_evidence_ids,
            });
            statement_counter += 1;
        }

        for (asset_id, span_ids) in caption_support {
            if let Some(visual_index) = asset_id_to_visual_index.get(&asset_id).copied() {
                let visual_item = &mut visual_evidence[visual_index];
                let caption_text = visual_item.caption_text.clone();
                if let Some(caption_text) = caption_text {
                    let observation_id = format!(
                        "obs_{}_{}",
                        visual_item.asset_id,
                        visual_item.observations.len() + 1
                    );
                    visual_item.observations.push(VisualObservation {
                        observation_id,
                        kind: VisualObservationKind::Caption,
                        created_by: "source_document".to_string(),
                        text: caption_text,
                        supporting_span_ids: span_ids,
                        automation_confidence: AutomationConfidence::High,
                    });
                }
            }
        }

        for (asset_id, supports) in reference_support {
            if let Some(visual_index) = asset_id_to_visual_index.get(&asset_id).copied() {
                let mut support_span_ids = Vec::new();
                let mut figure_reference_text = None;
                for support in supports {
                    support_span_ids.push(support.supporting_span_id);
                    if figure_reference_text.is_none() {
                        figure_reference_text = Some(support.display_reference_text);
                    }
                }
                let visual_item = &mut visual_evidence[visual_index];
                visual_item.figure_reference_text = figure_reference_text.clone();
                if let Some(reference_text) = figure_reference_text {
                    let observation_id = format!(
                        "obs_{}_{}",
                        visual_item.asset_id,
                        visual_item.observations.len() + 1
                    );
                    visual_item.observations.push(VisualObservation {
                        observation_id,
                        kind: VisualObservationKind::FigureReference,
                        created_by: "specforge_evidence_builder".to_string(),
                        text: reference_text,
                        supporting_span_ids: support_span_ids,
                        automation_confidence: AutomationConfidence::Medium,
                    });
                }
            }
        }

        for (index, pages) in section_pages.into_iter().enumerate() {
            if let (Some(min_page), Some(max_page)) =
                (pages.iter().min().copied(), pages.iter().max().copied())
            {
                section_anchors[index].page_start = Some(min_page);
                section_anchors[index].page_end = Some(max_page);
            }
        }

        // Synthesize typed declarations from structured table data in SourceIR.
        // This provides the first seed set for the convergent loop:
        //   1. direct signal declarations from signal-description tables
        //   2. direct enum facts from tables already classified as encodings
        // EXTRACTOR-ARCHITECTURE.5 — the signal-declaration SEED (table-declaration strategy + a
        // sparse-catalog prose fallback) is now one cohesive assembly-phase function instead of inline
        // orchestration in this ~500-line builder. It produces seed statements for the convergence loop plus
        // the table-declaration provenance side-output. This is the statement-ASSEMBLY phase, not the typed
        // surface-extraction phase, so it keeps its own orchestrator rather than the `run_surface` merge
        // driver (see the two-phase / two-category note in `ir/extractor.rs`).
        let (synthesized, table_signal_declaration_provenance) = synthesize_signal_declaration_seed(
            &source_ir,
            &extracted_statements,
            &mut statement_counter,
            prior_guidance.as_ref(),
        );

        // Extract system contract (clock + reset) from signal-description prose in tables.
        let contract_stmts = synthesize_system_contract_from_table_descriptions(
            &source_ir,
            &mut statement_counter,
            prior_guidance.as_ref(),
        );

        // EXTRACTOR-ARCHITECTURE.8 — collect each framework surface's run manifest into a per-document
        // extraction fingerprint (which extractors fired / produced / kept), surfaced on `EvidenceIr`.
        let mut extraction_manifest = ExtractionManifest::default();

        // EXTRACTOR-ARCHITECTURE.6 — the register-record surface (register-map + `unknown` field-table
        // strategies, concatenated via `run_surface_concat`, then the width / bit-layout-grid / fragment
        // post-passes) is now one cohesive surface function instead of inline orchestration. Behavior-identical.
        let register_records = register_record_surface(
            &source_ir,
            prior_guidance.as_ref(),
            &mut extraction_manifest,
        );
        let timing_constraints = synthesize_timing_constraints(&source_ir, prior_guidance.as_ref());

        // Replace the previous one-shot extraction with a monotone convergent loop:
        // discovered signals unlock anchored encoding tables, which unlock new value atoms,
        // which unlock additional prose-derived constraints.
        let (
            extracted_statements,
            signal_constraints,
            conditional_rules,
            signal_polarities,
            signal_polarity_conflicts,
            actor_signal_relations,
            convergence_report,
        ) = converge_evidence_extractions(
            &source_ir,
            extracted_statements,
            synthesized,
            contract_stmts,
            &mut statement_counter,
            prior_guidance.as_ref(),
            &mut extraction_manifest,
        );

        // SWD-SERIAL-EXTRACTION.3 + PDF-VARIANT-DIGESTION.9.3b: recover the serial-frame fields — the SWD
        // packet/ack/data frame, or a frame described as a prose COMPOSITION LIST (CAN's "composed of seven
        // different bit fields: SOF, ARBITRATION FIELD, …"). EXTRACTOR-ARCHITECTURE.9a — both strategies now
        // run through the unified `run_surface` driver; first-wins key-merge on the field name reproduces
        // the prior "composition defers to bit-range names" policy exactly because each strategy already
        // emits a name-unique list. No-op for parallel buses.
        let serial_frame_fields =
            serial_frame_field_surface(&extracted_statements, &mut extraction_manifest);

        // SWD-SERIAL-EXTRACTION.4/.4d + PDF-VARIANT-DIGESTION.9.3a/.9.7: recover the protocol FSM states.
        // EXTRACTOR-ARCHITECTURE.3 — the four FSM-state grammars (JTAG/SWD-hyphen, SWD line, quoted-mode,
        // transition-bound single-word) now run through the unified `Extractor`/`run_surface` driver instead
        // of four inline dedup loops here. Order = legacy precedence, key = uppercased state name → the
        // merged inventory is byte-identical. The FSM is the heart of SWD/JTAG and what FSMGen builds; no-op
        // for non-FSM/non-serial docs.
        let protocol_states =
            protocol_state_surface(&extracted_statements, &mut extraction_manifest);
        // PDF-VARIANT-DIGESTION.3b — protocol actors/agents defined in prose.
        // EXTRACTOR-ARCHITECTURE.7 — protocol actors via the unified framework (single-strategy surface, run
        // through the concat driver for a uniform manifest entry; output byte-identical).
        let protocol_actors_run = run_surface_concat(
            "protocol_actors",
            &ExtractionContext {
                statements: &extracted_statements,
            },
            &[&ProtocolActorExtractor],
        );
        extraction_manifest.record(&protocol_actors_run);
        let protocol_actors = protocol_actors_run.records;

        // SWD-SERIAL-EXTRACTION.4b: recover the SWD packet operations (response branching: OK→3-phase,
        // WAIT/FAULT→2-phase, + turnaround model). EXTRACTOR-ARCHITECTURE.9a — single-strategy surface run
        // through the concat driver for a uniform manifest entry. No-op for non-serial docs.
        let swd_operations = swd_operation_surface(&extracted_statements, &mut extraction_manifest);

        // PER-EXTRACTOR-FACT-TAGGING: tag every fact produced by the structural
        // pattern tier (the convergent build loop above) as `Pattern`, computed
        // before the move into the struct literal. The LLM tiers tag their finds
        // later (`nlp-enrich` for constraints, `signal-resolve` for relations).
        let fact_provenance: Vec<FactProvenanceRecord> = signal_constraints
            .iter()
            .map(|c| FactProvenanceRecord {
                producer: ExtractorTier::Pattern,
                fact_kind: FactKind::SignalConstraint,
                canonical_key: signal_constraint_fact_key(c),
            })
            .chain(actor_signal_relations.iter().map(|r| FactProvenanceRecord {
                producer: ExtractorTier::Pattern,
                fact_kind: FactKind::ActorSignalRelation,
                canonical_key: actor_signal_relation_fact_key(r),
            }))
            .collect();

        let mut evidence_ir = Self {
            schema_version: 1,
            stage: IrStage::EvidenceIr,
            source_ir_path,
            prior_memory_path: prior_guidance
                .as_ref()
                .map(|guidance| guidance.prior_memory_path.clone()),
            artifact_layout,
            document_identity,
            section_anchors,
            evidence_spans,
            visual_evidence,
            evidence_links,
            extracted_statements,
            register_records,
            timing_constraints,
            signal_constraints,
            conditional_rules,
            extracted_contracts: Vec::new(),
            constrained_extraction_stats: None,
            table_signal_declaration_provenance,
            signal_polarities,
            signal_polarity_conflicts,
            signal_semantic_hints: Vec::new(),
            signal_semantic_conflicts: Vec::new(),
            actor_signal_relations,
            signal_alias_map: BTreeMap::new(),
            validation_reports: Vec::new(),
            convergence_report: Some(convergence_report),
            fact_provenance,
            serial_frame_fields,
            protocol_states,
            protocol_actors,
            swd_operations,
            extraction_manifest,
        };
        evidence_ir.carry_forward_existing_knowledge()?;
        // refresh_signal_semantic_hints records the `signal_semantic_hints` surface into the manifest too.
        evidence_ir.refresh_signal_semantic_hints()?;

        Ok(evidence_ir)
    }

    /// Form 2: Signal alias learning feedback loop.
    ///
    /// Applies the accumulated `signal_alias_map` to re-classify remaining
    /// `NormativeStatement` sentences WITHOUT an LLM call.  For each sentence
    /// containing a known prose alias (e.g. "address bus"), the alias is substituted
    /// by the canonical signal name (e.g. "HADDR") and `is_signal_value_constraint()`
    /// is re-run on the substituted text.  If it now qualifies, the statement is
    /// reclassified to `SignalValueConstraint` and a `SignalConstraintRecord` is
    /// synthesised at `AutomationConfidence::Low` (alias-derived).
    ///
    /// Returns `(reclassified_count, new_records)`.  The caller is responsible for
    /// extending `self.signal_constraints` with `new_records` and writing to disk.
    pub fn apply_alias_reclassification(
        &mut self,
        constraint_counter: &mut usize,
    ) -> (usize, Vec<SignalConstraintRecord>) {
        if self.signal_alias_map.is_empty() {
            return (0, Vec::new());
        }

        // Build a quick-lookup set of texts already covered by existing records.
        let existing_texts: HashSet<&str> = self
            .signal_constraints
            .iter()
            .map(|r| r.source_text.as_str())
            .collect();

        let alias_map = self.signal_alias_map.clone();
        let mut reclassified = 0usize;
        let mut new_records = Vec::new();

        for stmt in &mut self.extracted_statements {
            if !matches!(stmt.class, StatementClass::NormativeStatement) {
                continue;
            }
            if existing_texts.contains(stmt.text.as_str()) {
                continue;
            }

            let lowered = stmt.text.to_ascii_lowercase();

            for (alias_phrase, signal_name) in &alias_map {
                if !lowered.contains(alias_phrase.as_str()) {
                    continue;
                }

                // Substitute the alias phrase with the uppercase signal name.
                // Result is mixed-case, e.g. "the HADDR shall remain stable when hready is low".
                // `is_signal_value_constraint()` handles this correctly:
                //   • lowercases for phrase-binding check  ("shall remain stable" found)
                //   • scans original-case for uppercase tokens  ("HADDR" found)
                let substituted = lowered.replace(alias_phrase.as_str(), signal_name.as_str());

                if !is_signal_value_constraint(&substituted) {
                    continue;
                }

                // Reclassify the statement (Form 2 backannotation).
                stmt.class = StatementClass::SignalValueConstraint;
                reclassified += 1;

                // Synthesise a constraint record for the reclassified statement.
                let constraint_kind = detect_constraint_kind_from_substituted(&substituted);
                let condition_text = extract_condition_clause(&stmt.text);
                let negated = lowered.contains(" not ") || lowered.contains("cannot");

                *constraint_counter += 1;
                new_records.push(SignalConstraintRecord {
                    constraint_id: format!("alias2_sigcon_{constraint_counter:04}"),
                    subject_signal: signal_name.clone(),
                    constraint_kind,
                    target_value: None,
                    condition_text,
                    negated,
                    source_text: stmt.text.clone(),
                    supporting_statement_ids: vec![stmt.statement_id.clone()],
                    automation_confidence: AutomationConfidence::Low,
                });

                break; // Apply at most one alias per statement.
            }
        }

        (reclassified, new_records)
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn refresh_signal_semantic_hints(&mut self) -> Result<()> {
        let source_ir = SourceIr::load_from_path(&self.source_ir_path)?;
        let prior_guidance =
            load_evidence_prior_guidance(self.prior_memory_path.as_deref(), &source_ir)?;
        // EXTRACTOR-ARCHITECTURE.8 — record the semantic-hints surface manifest into the per-document
        // fingerprint. Disjoint self-field borrows: the inputs are `&self.<field>` (shared) and the manifest
        // is `&mut self.extraction_manifest` (a distinct field) — allowed. `record` is idempotent per surface
        // name, so re-running a refresh replaces (not duplicates) the `signal_semantic_hints` entry.
        let (signal_semantic_hints, signal_semantic_conflicts) = synthesize_signal_semantic_hints(
            &source_ir,
            &self.extracted_statements,
            &self.actor_signal_relations,
            &self.signal_alias_map,
            &self.visual_evidence,
            prior_guidance.as_ref(),
            &mut self.extraction_manifest,
        );
        self.signal_semantic_hints = signal_semantic_hints;
        self.signal_semantic_conflicts = signal_semantic_conflicts;
        Ok(())
    }

    pub fn write_to_disk(&self) -> Result<()> {
        fs::create_dir_all(&self.artifact_layout.artifact_root)?;
        fs::write(
            &self.artifact_layout.evidence_ir_path,
            self.to_pretty_json()?,
        )?;
        Ok(())
    }

    pub fn dedup_loopback_records(&mut self) -> bool {
        let mut changed = false;
        changed |= dedup_signal_constraints_in_place(&mut self.signal_constraints);
        changed |= dedup_conditional_rules_in_place(&mut self.conditional_rules);
        changed
    }

    fn carry_forward_existing_knowledge(&mut self) -> Result<()> {
        if !self.artifact_layout.evidence_ir_path.exists() {
            return Ok(());
        }

        let existing = Self::load_from_path(&self.artifact_layout.evidence_ir_path)?;
        if existing.source_ir_path != self.source_ir_path
            || existing.document_identity != self.document_identity
            || existing.section_anchors.len() != self.section_anchors.len()
            || existing.extracted_statements.len() != self.extracted_statements.len()
        {
            return Ok(());
        }

        // Idempotency guard: the carry-forward merge exists to PRESERVE LLM enrichments
        // (`nlp_enrich`/`signal-resolve`, tagged `ExtractorTier::Nlp`) across re-builds. If the
        // existing artifact carries no such facts, the fresh deterministic build fully
        // supersedes it — skip the merge so re-running `evidence` after an extractor change does
        // not ACCUMULATE stale deterministic facts (which bit a V2 re-measurement: 13 → 21 with
        // duplicate ids). With LLM facts present the merge still runs to keep them.
        if !existing
            .fact_provenance
            .iter()
            .any(|prov| prov.producer == ExtractorTier::Nlp)
        {
            return Ok(());
        }

        self.signal_alias_map.extend(existing.signal_alias_map);
        carry_forward_statement_classes(
            &mut self.extracted_statements,
            &existing.extracted_statements,
        );
        merge_signal_constraints(&mut self.signal_constraints, &existing.signal_constraints);
        merge_conditional_rules(&mut self.conditional_rules, &existing.conditional_rules);
        self.dedup_loopback_records();

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceArtifactLayout {
    pub artifact_root: PathBuf,
    pub evidence_ir_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceDocumentIdentity {
    pub document_key: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SignalPolarity {
    ActiveHigh,
    ActiveLow,
}

impl SignalPolarity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ActiveHigh => "active_high",
            Self::ActiveLow => "active_low",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SignalPolarityEvidenceSourceKind {
    ProseStatement,
    SignalDescriptionTable,
}

impl SignalPolarityEvidenceSourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProseStatement => "prose_statement",
            Self::SignalDescriptionTable => "signal_description_table",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalPolarityObservationRecord {
    pub polarity: SignalPolarity,
    pub source_kind: SignalPolarityEvidenceSourceKind,
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
    #[serde(default)]
    pub supporting_table_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalPolarityRecord {
    pub signal_name: String,
    pub polarity: SignalPolarity,
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
    #[serde(default)]
    pub supporting_table_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalPolarityConflictRecord {
    pub conflict_id: String,
    pub signal_name: String,
    pub observations: Vec<SignalPolarityObservationRecord>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SignalSemanticTag {
    HandshakeValidLike,
    HandshakeReadyLike,
}

impl SignalSemanticTag {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HandshakeValidLike => "handshake_valid_like",
            Self::HandshakeReadyLike => "handshake_ready_like",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SignalSemanticHintSourceKind {
    SignalDescriptionTable,
    ProseStatement,
    AliasGroundedProseStatement,
    VisualCaption,
    VlmTimingDiagramAnnotation,
}

impl SignalSemanticHintSourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SignalDescriptionTable => "signal_description_table",
            Self::ProseStatement => "prose_statement",
            Self::AliasGroundedProseStatement => "alias_grounded_prose_statement",
            Self::VisualCaption => "visual_caption",
            Self::VlmTimingDiagramAnnotation => "vlm_timing_diagram_annotation",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalSemanticHintRecord {
    pub signal_name: String,
    #[serde(default)]
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
pub struct SignalSemanticConflictObservationRecord {
    #[serde(default)]
    pub semantic_tags: Vec<SignalSemanticTag>,
    pub source_kind: SignalSemanticHintSourceKind,
    pub source_text: String,
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
    #[serde(default)]
    pub supporting_table_ids: Vec<String>,
    #[serde(default)]
    pub supporting_visual_evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalSemanticConflictRecord {
    pub conflict_id: String,
    pub signal_name: String,
    pub observations: Vec<SignalSemanticConflictObservationRecord>,
    pub automation_confidence: AutomationConfidence,
}

fn carry_forward_statement_classes(
    current: &mut [ExtractedStatement],
    existing: &[ExtractedStatement],
) {
    let existing_by_id: HashMap<&str, StatementClass> = existing
        .iter()
        .filter_map(|statement| match statement.class {
            StatementClass::SignalValueConstraint | StatementClass::ConditionalRule => {
                Some((statement.statement_id.as_str(), statement.class))
            }
            _ => None,
        })
        .collect();
    let existing_by_text: HashMap<&str, StatementClass> = existing
        .iter()
        .filter_map(|statement| match statement.class {
            StatementClass::SignalValueConstraint | StatementClass::ConditionalRule => {
                Some((statement.text.as_str(), statement.class))
            }
            _ => None,
        })
        .collect();

    for statement in current {
        if !matches!(statement.class, StatementClass::NormativeStatement) {
            continue;
        }

        if let Some(class) = existing_by_id
            .get(statement.statement_id.as_str())
            .or_else(|| existing_by_text.get(statement.text.as_str()))
            .copied()
        {
            statement.class = class;
        }
    }
}

fn merge_signal_constraints(
    current: &mut Vec<SignalConstraintRecord>,
    existing: &[SignalConstraintRecord],
) {
    let mut known_keys = current
        .iter()
        .map(signal_constraint_merge_key)
        .collect::<HashSet<_>>();
    for record in existing {
        let key = signal_constraint_merge_key(record);
        if known_keys.insert(key) {
            current.push(record.clone());
        }
    }
}

fn merge_conditional_rules(
    current: &mut Vec<ConditionalRuleRecord>,
    existing: &[ConditionalRuleRecord],
) {
    let mut known_keys = current
        .iter()
        .map(conditional_rule_merge_key)
        .collect::<HashSet<_>>();
    for record in existing {
        let key = conditional_rule_merge_key(record);
        if known_keys.insert(key) {
            current.push(record.clone());
        }
    }
}

fn dedup_signal_constraints_in_place(records: &mut Vec<SignalConstraintRecord>) -> bool {
    let original_len = records.len();
    let mut seen = HashSet::new();
    records.retain(|record| seen.insert(signal_constraint_merge_key(record)));
    records.len() != original_len
}

fn dedup_conditional_rules_in_place(records: &mut Vec<ConditionalRuleRecord>) -> bool {
    let original_len = records.len();
    let mut seen = HashSet::new();
    records.retain(|record| seen.insert(conditional_rule_merge_key(record)));
    records.len() != original_len
}

fn signal_constraint_merge_key(record: &SignalConstraintRecord) -> String {
    format!(
        "{}|{:?}|{:?}|{:?}|{}|{}",
        record.subject_signal,
        record.constraint_kind,
        record.target_value,
        record.condition_text,
        record.negated,
        record.source_text
    )
}

fn conditional_rule_merge_key(record: &ConditionalRuleRecord) -> String {
    format!(
        "{}|{:?}|{}|{}",
        record.antecedent_text,
        record.consequent_signal,
        record.consequent_action,
        record.source_text
    )
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SectionAnchor {
    pub section_id: String,
    pub title: String,
    pub source_path: PathBuf,
    pub page_start: Option<u32>,
    pub page_end: Option<u32>,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceSpan {
    pub span_id: String,
    pub modality: EvidenceModality,
    pub source_path: PathBuf,
    pub source_page: Option<u32>,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub visual_asset_id: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VisualEvidenceItem {
    pub evidence_id: String,
    pub asset_id: String,
    pub asset_kind: VisualAssetKind,
    pub role: VisualEvidenceRole,
    pub source_path: Option<PathBuf>,
    pub source_page: Option<u32>,
    pub caption_text: Option<String>,
    pub figure_reference_text: Option<String>,
    pub observations: Vec<VisualObservation>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VisualObservation {
    pub observation_id: String,
    pub kind: VisualObservationKind,
    pub created_by: String,
    pub text: String,
    pub supporting_span_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceLink {
    pub link_id: String,
    pub from_evidence_span_id: String,
    pub to_visual_evidence_id: String,
    pub relation: EvidenceLinkKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtractedStatement {
    pub statement_id: String,
    pub class: StatementClass,
    pub modality: EvidenceModality,
    pub text: String,
    pub evidence_span_ids: Vec<String>,
    pub related_visual_evidence_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TableSignalDeclarationProvenanceRecord {
    pub statement_id: String,
    pub signal_name: String,
    pub table_id: String,
}

#[derive(Debug, Clone)]
struct MarkdownHeading {
    title: String,
    line_start: u32,
}

#[derive(Debug, Clone)]
struct MarkdownBlock {
    text: String,
    line_start: u32,
    line_end: u32,
}

#[derive(Debug, Clone)]
struct ParsedMarkdown {
    headings: Vec<MarkdownHeading>,
    blocks: Vec<MarkdownBlock>,
    total_lines: u32,
}

#[derive(Debug, Clone)]
struct VisualReferencePattern {
    asset_id: String,
    display_reference_text: String,
    tokens: Vec<String>,
}

#[derive(Debug, Clone)]
struct ReferenceSupport {
    supporting_span_id: String,
    display_reference_text: String,
}

fn build_visual_evidence_items(
    visual_assets: &[VisualAsset],
    prior_guidance: Option<&EvidencePriorGuidance>,
    signal_names: &BTreeSet<String>,
    actor_names: &BTreeSet<String>,
) -> Vec<VisualEvidenceItem> {
    visual_assets
        .iter()
        .enumerate()
        .map(|(index, asset)| {
            let prior_guided_diagram_kind =
                prior_guided_visual_diagram_kind(asset, prior_guidance, signal_names, actor_names);
            let effective_diagram_kind = prior_guided_diagram_kind.unwrap_or(asset.diagram_kind);
            let observations = prior_guided_diagram_kind
                .map(|diagram_kind| VisualObservation {
                    observation_id: format!("obs_prior_visual_motif_{}", asset.asset_id),
                    kind: VisualObservationKind::Classification,
                    created_by: "specforge_prior_memory".to_string(),
                    text: format!("diagram_kind={}", diagram_kind_key(diagram_kind)),
                    supporting_span_ids: Vec::new(),
                    automation_confidence: AutomationConfidence::Medium,
                })
                .into_iter()
                .collect();

            VisualEvidenceItem {
                evidence_id: format!("visual_{:04}", index + 1),
                asset_id: asset.asset_id.clone(),
                asset_kind: asset.asset_kind,
                role: infer_visual_role(
                    asset.asset_kind,
                    asset.caption_text.as_deref(),
                    effective_diagram_kind,
                ),
                source_path: asset
                    .image_path
                    .clone()
                    .or_else(|| asset.caption_source_path.clone()),
                source_page: asset.page_id.as_deref().and_then(page_number_from_page_id),
                caption_text: asset.caption_text.clone(),
                figure_reference_text: None,
                observations,
                automation_confidence: if asset.caption_text.is_some() {
                    AutomationConfidence::High
                } else {
                    AutomationConfidence::Medium
                },
            }
        })
        .collect()
}

fn prior_guided_visual_diagram_kind(
    asset: &VisualAsset,
    prior_guidance: Option<&EvidencePriorGuidance>,
    signal_names: &BTreeSet<String>,
    actor_names: &BTreeSet<String>,
) -> Option<DiagramKind> {
    if !matches!(asset.diagram_kind, DiagramKind::Unknown) {
        return None;
    }
    let caption_text = asset.caption_text.as_deref()?;
    let prior_guidance = prior_guidance?;
    prior_guidance
        .corpus_memory
        .diagram_kind_for_visual_caption(
            Some(prior_guidance.protocol_family),
            caption_text,
            signal_names,
            actor_names,
        )
}

fn diagram_kind_key(diagram_kind: DiagramKind) -> &'static str {
    match diagram_kind {
        DiagramKind::TimingDiagram => "timing_diagram",
        DiagramKind::StateMachineDiagram => "state_machine_diagram",
        DiagramKind::BlockDiagram => "block_diagram",
        DiagramKind::RegisterBitfield => "register_bitfield",
        DiagramKind::TruthTable => "truth_table",
        DiagramKind::FlowChart => "flow_chart",
        DiagramKind::Unknown => "unknown",
    }
}

fn build_caption_key_index(visual_assets: &[VisualAsset]) -> HashMap<String, String> {
    visual_assets
        .iter()
        .filter_map(|asset| {
            asset
                .caption_text
                .as_ref()
                .map(|caption| (normalize_text_key(caption), asset.asset_id.clone()))
        })
        .collect()
}

fn build_reference_patterns(visual_assets: &[VisualAsset]) -> Vec<VisualReferencePattern> {
    visual_assets
        .iter()
        .filter_map(|asset| {
            parse_caption_reference(asset.caption_text.as_deref()?).map(|reference| {
                let display_reference_text =
                    format!("{} {}", reference.display_label, reference.number);
                let tokens = match reference.reference_kind {
                    ReferenceKind::Figure => vec![
                        format!("figure {}", reference.number),
                        format!("fig. {}", reference.number),
                        format!("fig {}", reference.number),
                    ],
                    ReferenceKind::Table => vec![format!("table {}", reference.number)],
                };
                VisualReferencePattern {
                    asset_id: asset.asset_id.clone(),
                    display_reference_text,
                    tokens,
                }
            })
        })
        .collect()
}

fn extract_reference_hits(
    text: &str,
    patterns: &[VisualReferencePattern],
) -> Vec<VisualReferencePattern> {
    let lowered_text = text.to_ascii_lowercase();
    let mut seen_asset_ids = HashSet::new();
    let mut hits = Vec::new();

    for pattern in patterns {
        if seen_asset_ids.contains(&pattern.asset_id) {
            continue;
        }

        if pattern
            .tokens
            .iter()
            .any(|token| contains_reference_token(&lowered_text, token))
        {
            seen_asset_ids.insert(pattern.asset_id.clone());
            hits.push(pattern.clone());
        }
    }

    hits
}

fn parse_markdown(path: &Path) -> Result<ParsedMarkdown> {
    let markdown = fs::read_to_string(path)?;
    let lines: Vec<&str> = markdown.lines().collect();
    let total_lines = lines.len() as u32;

    let mut headings = Vec::new();
    let mut blocks = Vec::new();
    let mut current_lines = Vec::new();
    let mut current_line_start = 0u32;
    let mut current_line_end = 0u32;
    let mut in_code_fence = false;

    for (index, raw_line) in lines.iter().enumerate() {
        let line_number = (index + 1) as u32;
        let trimmed = raw_line.trim();

        if trimmed.starts_with("```") {
            flush_markdown_block(
                &mut blocks,
                &mut current_lines,
                &mut current_line_start,
                &mut current_line_end,
            );
            in_code_fence = !in_code_fence;
            continue;
        }

        if in_code_fence {
            continue;
        }

        if let Some(title) = heading_title(trimmed) {
            flush_markdown_block(
                &mut blocks,
                &mut current_lines,
                &mut current_line_start,
                &mut current_line_end,
            );
            headings.push(MarkdownHeading {
                title,
                line_start: line_number,
            });
            continue;
        }

        if trimmed.is_empty() {
            flush_markdown_block(
                &mut blocks,
                &mut current_lines,
                &mut current_line_start,
                &mut current_line_end,
            );
            continue;
        }

        if is_image_line(trimmed) {
            flush_markdown_block(
                &mut blocks,
                &mut current_lines,
                &mut current_line_start,
                &mut current_line_end,
            );
            continue;
        }

        if is_standalone_markdown_block(trimmed) {
            flush_markdown_block(
                &mut blocks,
                &mut current_lines,
                &mut current_line_start,
                &mut current_line_end,
            );
            blocks.push(MarkdownBlock {
                text: normalize_block_text(&[trimmed.to_string()]),
                line_start: line_number,
                line_end: line_number,
            });
            continue;
        }

        if current_lines.is_empty() {
            current_line_start = line_number;
        }
        current_line_end = line_number;
        current_lines.push(trimmed.to_string());
    }

    flush_markdown_block(
        &mut blocks,
        &mut current_lines,
        &mut current_line_start,
        &mut current_line_end,
    );

    Ok(ParsedMarkdown {
        headings,
        blocks,
        total_lines,
    })
}

fn flush_markdown_block(
    blocks: &mut Vec<MarkdownBlock>,
    current_lines: &mut Vec<String>,
    current_line_start: &mut u32,
    current_line_end: &mut u32,
) {
    if current_lines.is_empty() {
        return;
    }

    blocks.push(MarkdownBlock {
        text: normalize_block_text(current_lines),
        line_start: *current_line_start,
        line_end: *current_line_end,
    });
    current_lines.clear();
    *current_line_start = 0;
    *current_line_end = 0;
}

fn normalize_block_text(lines: &[String]) -> String {
    lines
        .iter()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn build_section_anchors(
    headings: &[MarkdownHeading],
    total_lines: u32,
    display_name: &str,
) -> Vec<SectionAnchor> {
    if headings.is_empty() {
        return vec![SectionAnchor {
            section_id: "section_0001_document_root".to_string(),
            title: display_name.to_string(),
            source_path: PathBuf::new(),
            page_start: None,
            page_end: None,
            line_start: if total_lines > 0 { Some(1) } else { None },
            line_end: if total_lines > 0 {
                Some(total_lines)
            } else {
                None
            },
        }];
    }

    headings
        .iter()
        .enumerate()
        .map(|(index, heading)| SectionAnchor {
            section_id: format!("section_{:04}_{}", index + 1, document_key(&heading.title)),
            title: heading.title.clone(),
            source_path: PathBuf::new(),
            page_start: None,
            page_end: None,
            line_start: Some(heading.line_start),
            line_end: Some(
                headings
                    .get(index + 1)
                    .map(|next| next.line_start.saturating_sub(1))
                    .unwrap_or(total_lines),
            ),
        })
        .collect()
}

fn section_index_for_line(section_anchors: &[SectionAnchor], line_number: u32) -> Option<usize> {
    section_anchors.iter().rposition(|anchor| {
        anchor
            .line_start
            .map(|start| line_number >= start)
            .unwrap_or(false)
    })
}

fn heading_title(trimmed_line: &str) -> Option<String> {
    let heading_level = trimmed_line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if heading_level == 0 {
        return None;
    }

    let title = trimmed_line[heading_level..].trim();
    if title.is_empty() {
        None
    } else {
        Some(title.to_string())
    }
}

fn is_image_line(trimmed_line: &str) -> bool {
    trimmed_line.starts_with("![")
}

fn is_standalone_markdown_block(trimmed_line: &str) -> bool {
    trimmed_line.starts_with("- ")
        || trimmed_line.starts_with("* ")
        || trimmed_line.starts_with("+ ")
        || trimmed_line.starts_with("|")
        || numbered_list_prefix(trimmed_line)
}

fn numbered_list_prefix(trimmed_line: &str) -> bool {
    let mut chars = trimmed_line.chars().peekable();
    let mut saw_digit = false;
    while let Some(character) = chars.peek() {
        if character.is_ascii_digit() {
            saw_digit = true;
            chars.next();
        } else {
            break;
        }
    }

    saw_digit && chars.next() == Some('.') && chars.next() == Some(' ')
}

fn infer_visual_role(
    asset_kind: VisualAssetKind,
    caption_text: Option<&str>,
    diagram_kind: DiagramKind,
) -> VisualEvidenceRole {
    let lowered_caption = caption_text.map(|text| text.to_ascii_lowercase());

    if matches!(
        diagram_kind,
        DiagramKind::TimingDiagram | DiagramKind::StateMachineDiagram | DiagramKind::TruthTable
    ) {
        return VisualEvidenceRole::Normative;
    }

    if let Some(lowered_caption) = lowered_caption.as_deref() {
        if contains_any(
            lowered_caption,
            &["example", "examples", "sample", "illustration"],
        ) {
            return VisualEvidenceRole::Illustrative;
        }
        if contains_any(
            lowered_caption,
            &[
                "timing",
                "waveform",
                "handshake",
                "state",
                "transition",
                "truth table",
                "protocol",
            ],
        ) {
            return VisualEvidenceRole::Normative;
        }
    }

    match asset_kind {
        VisualAssetKind::Chart | VisualAssetKind::TableRegion => VisualEvidenceRole::Explanatory,
        VisualAssetKind::Figure | VisualAssetKind::Diagram => VisualEvidenceRole::Ambiguous,
        VisualAssetKind::FormulaRegion => VisualEvidenceRole::Normative,
        VisualAssetKind::Screenshot => VisualEvidenceRole::Illustrative,
        VisualAssetKind::Unknown => VisualEvidenceRole::Unknown,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tier 2 Knowledge Graph: actor–signal relation extraction
// ─────────────────────────────────────────────────────────────────────────────

/// Collect all hardware signal names that have been formally declared via
/// synthesized `Signal X is input/output` statements.  These come from signal
/// description tables (High confidence) and are the known universe of signals
/// we should look for in prose.
pub(crate) fn collect_known_signal_names(
    statements: &[ExtractedStatement],
) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    for stmt in statements {
        let text = &stmt.text;
        let lowered = text.to_ascii_lowercase();
        // Scan for ALL "Signal X is..." occurrences.
        // Handles both single-line declarations and merged multi-signal blocks
        // (consecutive non-empty lines are concatenated into one statement during markdown parsing).
        // A valid declaration starts either at position 0 or after ". " (sentence boundary).
        for (idx, _) in lowered.match_indices("signal ") {
            let is_declaration_start = idx == 0 || (idx >= 2 && &lowered[idx - 2..idx] == ". ");
            if !is_declaration_start {
                continue;
            }
            let name_start = idx + 7; // past "signal "
            if name_start > text.len() {
                continue;
            }
            let name: String = text[name_start..]
                .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .next()
                .unwrap_or("")
                .to_ascii_uppercase();
            // Only keep plausible hardware signal names: uppercase, 2-30 chars
            if name.len() >= 2
                && name.len() <= 30
                && name
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            {
                names.insert(name);
            }
        }
    }
    names
}

pub(crate) fn collect_signals_with_explicit_direction_declarations(
    statements: &[ExtractedStatement],
) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    for stmt in statements {
        let text = &stmt.text;
        let lowered = text.to_ascii_lowercase();
        for (idx, _) in lowered.match_indices("signal ") {
            let is_declaration_start = idx == 0 || (idx >= 2 && &lowered[idx - 2..idx] == ". ");
            if !is_declaration_start {
                continue;
            }
            let tail = &lowered[idx..];
            let sentence = tail.find('.').map(|end| &tail[..end]).unwrap_or(tail);
            if !(sentence.contains(" is input")
                || sentence.contains(" is output")
                || sentence.contains(" is internal")
                || sentence.contains(" is local"))
            {
                continue;
            }
            let name_start = idx + 7;
            if name_start > text.len() {
                continue;
            }
            let name: String = text[name_start..]
                .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .next()
                .unwrap_or("")
                .to_ascii_uppercase();
            if name.len() >= 2
                && name.len() <= 30
                && name
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            {
                names.insert(name);
            }
        }
    }
    names
}

/// Collect hardware signal names from ALL signal-description table rows (first column),
/// regardless of whether direction could be determined.  This covers specs like APB and AXI
/// where the Source/Direction column uses non-standard values ("Requester", "Completer")
/// or is absent entirely.
fn collect_signal_names_from_tables(
    source_ir: &SourceIr,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    for table in &source_ir.structured_tables {
        if !should_treat_table_as_top_level_signal_description(source_ir, table, prior_guidance) {
            continue;
        }
        for row in &table.body_rows {
            let Some(first_cell) = row.first() else {
                continue;
            };
            let token = first_cell
                .text
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_ascii_uppercase();
            if is_hardware_signal_token(&token) && !is_signal_synthesis_non_signal(&token) {
                names.insert(token);
            }
        }
    }
    names
}

fn load_evidence_prior_guidance(
    prior_memory_path: Option<&Path>,
    source_ir: &SourceIr,
) -> Result<Option<EvidencePriorGuidance>> {
    let Some(prior_memory_path) = prior_memory_path else {
        return Ok(None);
    };
    if !prior_memory_path.exists() {
        return Ok(None);
    }
    let prior_memory_path = canonicalize_existing_path(prior_memory_path)?;

    let corpus_memory =
        serde_json::from_str::<CorpusMemory>(&fs::read_to_string(&prior_memory_path)?)?;
    Ok(Some(EvidencePriorGuidance {
        prior_memory_path: prior_memory_path.clone(),
        protocol_family: ProtocolFamily::infer(
            &source_ir.document_identity.document_key,
            &source_ir.document_identity.display_name,
        ),
        corpus_memory,
    }))
}

/// Extract actor–signal relation triples directly from signal-description table structure.
///
/// The Source/Driver column of a signal-description table encodes the same information
/// as a prose sentence: `PADDR | Requester | ...` means `(Requester, Drives, PADDR)`.
/// This is part of the knowledge graph — the table is part of the document, and the
/// Source column directly records which actor drives each signal.
///
/// Unlike prose extraction, no verb-pattern matching is needed here: the table cell
/// value names either the driving or receiving actor, and the table structure implies
/// the relation kind.
///
/// Only plausible actor labels become KG relations. Direction placeholders ("input",
/// "output"), infrastructure labels ("Clock", "Reset"), and similar non-actor values
/// are filtered out so the canonical graph does not invent bogus actors from
/// signal-description metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RelationTableColumnKind {
    SourceLike,
    DestinationLike,
}

fn normalize_table_actor_name(value: &str) -> Option<String> {
    let actor = value.trim();
    if actor.is_empty() {
        return None;
    }

    let lowered = actor
        .split_whitespace()
        .map(|token| token.to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    if lowered.is_empty() || !lowered.chars().any(|ch| ch.is_ascii_alphabetic()) {
        return None;
    }

    if matches!(
        lowered.as_str(),
        "input"
            | "output"
            | "inout"
            | "bidirectional"
            | "bidir"
            | "reserved"
            | "n/a"
            | "na"
            | "none"
            | "tbd"
            | "see note"
            | "-"
    ) {
        return None;
    }

    if lowered.contains("clock")
        || lowered.contains("reset")
        || lowered.contains("global")
        || lowered.contains("system bus")
        || lowered.contains("power")
        || lowered.contains("ground")
        || lowered.contains("supply")
        || lowered.contains("vdd")
        || lowered.contains("vss")
    {
        return None;
    }

    Some(actor.to_string())
}

fn normalize_relation_actor_name(value: &str) -> Option<String> {
    let actor = normalize_table_actor_name(value)?;
    if !is_meaningful_actor_term(&actor) {
        return None;
    }

    Some(actor)
}

fn is_tie_off_actor_text(value: &str) -> bool {
    matches!(normalize_actor_term(value).as_str(), "tie off" | "tieoff")
}

fn effective_table_kind(
    table: &crate::ir::source::StructuredTableRecord,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> TableKind {
    if !matches!(table.table_kind, TableKind::Unknown) {
        return table.table_kind;
    }

    prior_guidance
        .and_then(|prior_guidance| {
            prior_guidance
                .corpus_memory
                .table_kind_for_structured_table(Some(prior_guidance.protocol_family), table)
        })
        .unwrap_or(TableKind::Unknown)
}

fn nearest_section_title_for_table(
    source_ir: &SourceIr,
    table: &crate::ir::source::StructuredTableRecord,
) -> Option<String> {
    nearest_section_title_original(source_ir, table).map(|t| t.to_ascii_lowercase())
}

/// The nearest preceding section heading for a table, by page number, in its ORIGINAL case. Used where
/// the heading's casing carries meaning (e.g. recovering a register name token); the lowercased variant
/// above is for case-insensitive section matching. EXTRACTION-GAP-FIX.3.
fn nearest_section_title_original(
    source_ir: &SourceIr,
    table: &crate::ir::source::StructuredTableRecord,
) -> Option<String> {
    let table_page = table
        .page_id
        .as_deref()
        .and_then(page_number_from_page_id)?;
    source_ir
        .document_sections
        .iter()
        .filter_map(|section| {
            let section_page = section
                .page_id
                .as_deref()
                .and_then(page_number_from_page_id)?;
            (section_page <= table_page).then_some((section_page, section.title.as_str()))
        })
        .max_by_key(|(section_page, _)| *section_page)
        .map(|(_, title)| title.to_string())
}

fn should_treat_table_as_top_level_signal_description(
    source_ir: &SourceIr,
    table: &crate::ir::source::StructuredTableRecord,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> bool {
    if !matches!(
        effective_table_kind(table, prior_guidance),
        TableKind::SignalDescription
    ) {
        return false;
    }

    let header_texts: Vec<String> = table
        .header_rows
        .first()
        .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
        .unwrap_or_default();
    let first_header = header_texts.first().cloned().unwrap_or_default();
    let caption_text = table
        .caption_text
        .as_deref()
        .unwrap_or("")
        .to_ascii_lowercase();
    let section_title = nearest_section_title_for_table(source_ir, table).unwrap_or_default();
    let has_explicit_signal_header = header_texts.iter().any(|header| {
        header.contains("signal") || header.contains("port") || header.contains("pin")
    });
    let has_direction_or_relation_header = header_texts.iter().any(|header| {
        header.contains("source")
            || header.contains("driver")
            || header.contains("direction")
            || header.contains("destination")
            || header.contains("dest")
    });
    let name_col = header_texts.iter().position(|header| {
        header.contains("name")
            || header.contains("signal")
            || header.contains("port")
            || header.contains("pin")
    });
    let caption_looks_field_like = caption_text.contains(" field")
        || caption_text.contains(" fields")
        || caption_text.contains("bit assignment")
        || caption_text.contains("bit assignments")
        || caption_text.contains("bit definition")
        || caption_text.contains("bit definitions")
        || caption_text.contains("bitfield")
        || caption_text.contains("bit field");
    let section_looks_field_like = section_title.contains(" field")
        || section_title.contains(" fields")
        || section_title.contains("bit assignment")
        || section_title.contains("bit assignments")
        || section_title.contains("bit definition")
        || section_title.contains("bit definitions")
        || section_title.contains("bitfield")
        || section_title.contains("bit field");
    let first_header_looks_field_like = first_header.contains("bit")
        || first_header.contains("field")
        || first_header.contains("offset");

    if first_header_looks_field_like {
        return false;
    }

    if (caption_looks_field_like || section_looks_field_like)
        && name_col.map(|index| index > 0).unwrap_or(false)
    {
        return false;
    }

    if (caption_looks_field_like || section_looks_field_like)
        && !has_direction_or_relation_header
        && !has_explicit_signal_header
        && first_header.contains("name")
    {
        return false;
    }

    if table_looks_like_abstract_transport_signal_table(table) {
        return false;
    }

    true
}

fn table_looks_like_abstract_transport_signal_table(
    table: &crate::ir::source::StructuredTableRecord,
) -> bool {
    let header_texts: Vec<String> = table
        .header_rows
        .first()
        .map(|row| {
            row.iter()
                .map(|cell| cell.text.to_ascii_lowercase())
                .collect()
        })
        .unwrap_or_default();
    let relation_col = header_texts.iter().position(|header| {
        header.contains("source")
            || header.contains("driver")
            || header.contains("destination")
            || header.contains("dest")
    });
    let Some(relation_col) = relation_col else {
        return false;
    };

    let mut signal_tokens = Vec::new();
    let mut actor_terms = BTreeSet::new();
    for row in &table.body_rows {
        let Some(name_cell) = row.first() else {
            continue;
        };
        let signal_token = name_cell
            .text
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_ascii_uppercase();
        if !is_hardware_signal_token(&signal_token) || is_signal_synthesis_non_signal(&signal_token)
        {
            continue;
        }

        let Some(actor_cell) = row.get(relation_col) else {
            continue;
        };
        let actor_term = actor_cell.text.trim();
        if actor_term.is_empty() {
            continue;
        }

        signal_tokens.push(signal_token);
        actor_terms.insert(actor_term.to_string());
    }

    signal_tokens.len() >= 2
        && signal_tokens
            .iter()
            .all(|token| is_abstract_transport_signal_token(token))
        && !actor_terms.is_empty()
        && actor_terms
            .iter()
            .all(|term| is_abstract_transport_actor_term(term))
}

fn is_abstract_transport_signal_token(token: &str) -> bool {
    matches!(
        token,
        "VALID" | "READY" | "PENDING" | "CRDT" | "CRDTSH" | "SHAREDCRD" | "RP"
    )
}

fn is_abstract_transport_actor_term(term: &str) -> bool {
    matches!(
        normalize_actor_term(term).as_str(),
        "tx" | "rx" | "transmitter" | "receiver"
    )
}

fn actor_name_and_role_from_section_heading(
    title: &str,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Option<(String, ActorTaxonomyRole)> {
    let lowered = title.to_ascii_lowercase();
    for suffix in [
        " signals", " signal", " inputs", " input", " outputs", " output",
    ] {
        if lowered.ends_with(suffix) {
            let trimmed = title
                .get(..title.len().saturating_sub(suffix.len()))
                .unwrap_or("")
                .trim();
            let actor_name = normalize_table_actor_name(trimmed)?;
            let role = actor_taxonomy_role_in_text(trimmed, prior_guidance)?;
            return Some((actor_name, role));
        }
    }

    None
}

fn opposite_actor_taxonomy_role(role: ActorTaxonomyRole) -> ActorTaxonomyRole {
    match role {
        ActorTaxonomyRole::RequesterLike => ActorTaxonomyRole::CompleterLike,
        ActorTaxonomyRole::CompleterLike => ActorTaxonomyRole::RequesterLike,
    }
}

fn collect_local_actor_names_by_taxonomy_role(
    source_ir: &SourceIr,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> BTreeMap<ActorTaxonomyRole, BTreeSet<String>> {
    let mut actor_names_by_role = BTreeMap::<ActorTaxonomyRole, BTreeSet<String>>::new();

    for table in &source_ir.structured_tables {
        if !should_treat_table_as_top_level_signal_description(source_ir, table, prior_guidance) {
            continue;
        }

        let header_texts: Vec<String> = table
            .header_rows
            .first()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.text.to_ascii_lowercase())
                    .collect()
            })
            .unwrap_or_default();
        let relation_cols = header_texts
            .iter()
            .enumerate()
            .filter_map(|(idx, header)| {
                if header.contains("source")
                    || header.contains("driver")
                    || header.contains("destination")
                    || header.contains("dest")
                {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for row in &table.body_rows {
            for relation_col in &relation_cols {
                if let Some(cell) = row.get(*relation_col)
                    && let Some(actor_name) = normalize_relation_actor_name(&cell.text)
                    && let Some(role) = actor_taxonomy_role_in_text(&actor_name, prior_guidance)
                {
                    actor_names_by_role
                        .entry(role)
                        .or_default()
                        .insert(actor_name);
                }
            }
        }
    }

    for section in &source_ir.document_sections {
        if let Some((actor_name, role)) =
            actor_name_and_role_from_section_heading(&section.title, prior_guidance)
            && let Some(actor_name) = normalize_relation_actor_name(&actor_name)
        {
            actor_names_by_role
                .entry(role)
                .or_default()
                .insert(actor_name);
        }
    }

    actor_names_by_role
}

fn unique_complementary_reader_actor_name(
    actor_name: &str,
    actor_role: ActorTaxonomyRole,
    local_actor_names_by_role: &BTreeMap<ActorTaxonomyRole, BTreeSet<String>>,
) -> Option<String> {
    let opposite_role = opposite_actor_taxonomy_role(actor_role);
    let candidates = local_actor_names_by_role.get(&opposite_role)?;
    if candidates.len() != 1 {
        return None;
    }

    let candidate = candidates.iter().next()?.clone();
    if normalize_actor_term(&candidate) == normalize_actor_term(actor_name) {
        return None;
    }

    Some(candidate)
}

#[cfg(test)]
fn extract_relations_from_signal_tables(source_ir: &SourceIr) -> Vec<ActorSignalRelation> {
    extract_relations_from_signal_tables_with_prior_guidance(source_ir, None)
}

fn extract_relations_from_signal_tables_with_prior_guidance(
    source_ir: &SourceIr,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<ActorSignalRelation> {
    let mut records = Vec::new();
    let mut counter = 1usize;
    let mut page_to_section: BTreeMap<u32, (SectionKind, String)> = BTreeMap::new();
    let local_actor_names_by_role =
        collect_local_actor_names_by_taxonomy_role(source_ir, prior_guidance);

    for section in &source_ir.document_sections {
        if let Some(page_num) = section
            .page_id
            .as_deref()
            .and_then(page_number_from_page_id)
        {
            page_to_section.insert(page_num, (section.section_kind, section.title.clone()));
        }
    }

    for table in &source_ir.structured_tables {
        if !should_treat_table_as_top_level_signal_description(source_ir, table, prior_guidance) {
            continue;
        }

        // Find the first relation-bearing column.
        let header_texts: Vec<String> = table
            .header_rows
            .first()
            .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
            .unwrap_or_default();
        let relation_col = header_texts.iter().enumerate().find_map(|(idx, header)| {
            if header.contains("source") || header.contains("driver") {
                Some((idx, RelationTableColumnKind::SourceLike))
            } else if header.contains("destination") || header.contains("dest") {
                Some((idx, RelationTableColumnKind::DestinationLike))
            } else {
                None
            }
        });

        let table_page = table
            .page_id
            .as_deref()
            .and_then(page_number_from_page_id)
            .unwrap_or(0);
        let section_context = page_to_section
            .range(..=table_page)
            .next_back()
            .map(|(_, context)| context.clone());

        for row in &table.body_rows {
            // Signal name from first column
            let Some(name_cell) = row.first() else {
                continue;
            };
            let signal_token = name_cell
                .text
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_ascii_uppercase();
            if !is_hardware_signal_token(&signal_token)
                || is_signal_synthesis_non_signal(&signal_token)
            {
                continue;
            }

            let Some((actor, relation)) = relation_col
                .and_then(|(relation_col_idx, relation_col_kind)| {
                    row.get(relation_col_idx).and_then(|source_cell| {
                        normalize_relation_actor_name(&source_cell.text).map(|actor| {
                            let relation = match relation_col_kind {
                                RelationTableColumnKind::SourceLike => RelationKind::Drives,
                                RelationTableColumnKind::DestinationLike => RelationKind::Reads,
                            };
                            (actor, relation)
                        })
                    })
                })
                .or_else(|| {
                    section_context
                        .as_ref()
                        .and_then(|(section_kind, section_title)| {
                            if !matches!(section_kind, SectionKind::SignalDescription) {
                                return None;
                            }
                            actor_name_and_role_from_section_heading(section_title, prior_guidance)
                                .map(|(actor_name, _role)| (actor_name, RelationKind::Drives))
                        })
                })
            else {
                continue;
            };

            let complementary_reader = if matches!(relation, RelationKind::Drives) {
                actor_taxonomy_role_in_text(&actor, prior_guidance).and_then(|actor_role| {
                    unique_complementary_reader_actor_name(
                        &actor,
                        actor_role,
                        &local_actor_names_by_role,
                    )
                })
            } else {
                None
            };

            records.push(ActorSignalRelation {
                relation_id: format!("tbl_asr_{counter:04}"),
                actor_name: actor.clone(),
                signal_name: signal_token.clone(),
                relation,
                source_statement_ids: vec![table.table_id.clone()],
                automation_confidence: AutomationConfidence::Medium,
            });
            counter += 1;

            if let Some(reader_actor) = complementary_reader {
                records.push(ActorSignalRelation {
                    relation_id: format!("tbl_asr_{counter:04}"),
                    actor_name: reader_actor,
                    signal_name: signal_token.clone(),
                    relation: RelationKind::Reads,
                    source_statement_ids: vec![table.table_id.clone()],
                    automation_confidence: AutomationConfidence::Medium,
                });
                counter += 1;
            }
        }
    }

    records
}

fn augment_check_signal_relations_from_tables(
    source_ir: &SourceIr,
    actor_signal_relations: &[ActorSignalRelation],
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<ActorSignalRelation> {
    let mut augmented = actor_signal_relations.to_vec();
    let mut existing_keys = actor_signal_relations
        .iter()
        .map(|relation| {
            (
                relation.actor_name.clone(),
                relation.signal_name.clone(),
                matches!(relation.relation, RelationKind::Drives) as u8,
            )
        })
        .collect::<HashSet<_>>();

    let mut relations_by_signal = HashMap::<String, Vec<(String, RelationKind)>>::new();
    for relation in actor_signal_relations {
        relations_by_signal
            .entry(relation.signal_name.clone())
            .or_default()
            .push((relation.actor_name.clone(), relation.relation));
    }

    let mut counter = 1usize;
    for table in &source_ir.structured_tables {
        if !should_treat_table_as_top_level_signal_description(source_ir, table, prior_guidance) {
            continue;
        }

        let header_texts: Vec<String> = table
            .header_rows
            .first()
            .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
            .unwrap_or_default();
        let check_signal_col = header_texts
            .iter()
            .position(|header| header.contains("check signal"));
        let covered_signal_col = header_texts.iter().position(|header| {
            header.contains("signals covered")
                || (header.contains("covered") && header.contains("signal"))
        });
        let check_enable_col = header_texts.iter().position(|header| {
            header.contains("check enable")
                || (header.contains("enable") && header.contains("check"))
        });
        let granularity_col = header_texts
            .iter()
            .position(|header| header.contains("granularity"));
        let (Some(check_signal_col), Some(covered_signal_col)) =
            (check_signal_col, covered_signal_col)
        else {
            continue;
        };

        for row in &table.body_rows {
            let Some(check_signal_cell) = row.get(check_signal_col) else {
                continue;
            };
            let check_signal = check_signal_cell
                .text
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_ascii_uppercase();
            if !is_hardware_signal_token(&check_signal)
                || is_signal_synthesis_non_signal(&check_signal)
                || relations_by_signal.contains_key(&check_signal)
            {
                continue;
            }

            let Some(covered_signal_cell) = row.get(covered_signal_col) else {
                continue;
            };
            let covered_signals = collect_related_check_table_signal_tokens(
                &covered_signal_cell.text,
                check_enable_col.and_then(|col| row.get(col).map(|cell| cell.text.as_str())),
                granularity_col.and_then(|col| row.get(col).map(|cell| cell.text.as_str())),
                &relations_by_signal,
            );
            if covered_signals.is_empty() {
                continue;
            }

            let mut inherited_pairs = covered_signals
                .iter()
                .flat_map(|signal_name| relations_by_signal.get(signal_name).into_iter().flatten())
                .cloned()
                .collect::<Vec<_>>();
            inherited_pairs.sort_by(|left, right| {
                let left_relation_rank = matches!(left.1, RelationKind::Reads) as u8;
                let right_relation_rank = matches!(right.1, RelationKind::Reads) as u8;
                left.0
                    .cmp(&right.0)
                    .then_with(|| left_relation_rank.cmp(&right_relation_rank))
            });
            inherited_pairs.dedup();
            if inherited_pairs.is_empty() {
                continue;
            }

            for (actor_name, relation) in inherited_pairs {
                let key = (
                    actor_name.clone(),
                    check_signal.clone(),
                    matches!(relation, RelationKind::Drives) as u8,
                );
                if !existing_keys.insert(key) {
                    continue;
                }
                augmented.push(ActorSignalRelation {
                    relation_id: format!("chk_asr_{counter:04}"),
                    actor_name,
                    signal_name: check_signal.clone(),
                    relation,
                    source_statement_ids: vec![table.table_id.clone()],
                    automation_confidence: AutomationConfidence::Medium,
                });
                counter += 1;
            }
        }
    }

    augmented
}

/// Collect hardware signal widths from signal-description table Width columns.
/// Returns a map of signal_name → WidthHint.
/// Used to enrich KG-synthesized direction declarations with width information
/// (e.g. "Signal PADDR is output width ADDR_WIDTH.") even when direction
/// must come from prose rather than the table.
fn collect_signal_widths_from_tables(
    source_ir: &SourceIr,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> std::collections::HashMap<String, WidthHint> {
    let mut widths = std::collections::HashMap::new();
    for table in &source_ir.structured_tables {
        if !should_treat_table_as_top_level_signal_description(source_ir, table, prior_guidance) {
            continue;
        }
        let header_texts: Vec<String> = table
            .header_rows
            .first()
            .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
            .unwrap_or_default();
        let width_col = header_texts
            .iter()
            .position(|h| h.contains("width") || h.contains("bits") || h.contains("size"));
        let Some(w_col) = width_col else {
            continue;
        };
        for row in &table.body_rows {
            let Some(name_cell) = row.first() else {
                continue;
            };
            let signal = name_cell
                .text
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_ascii_uppercase();
            if !is_hardware_signal_token(&signal) || is_signal_synthesis_non_signal(&signal) {
                continue;
            }
            if let Some(width_hint) =
                infer_signal_table_row_width_hint(row, &header_texts, Some(w_col))
            {
                widths.insert(signal, width_hint);
            }
        }
    }
    widths
}

fn infer_signal_table_row_width_hint(
    row: &[StructuredTableCellRecord],
    header_texts: &[String],
    width_col: Option<usize>,
) -> Option<WidthHint> {
    if let Some(width_hint) = width_col
        .and_then(|col| row.get(col))
        .and_then(|cell| parse_table_width_hint_text(&cell.text))
    {
        return Some(width_hint);
    }

    let check_signal_col = header_texts
        .iter()
        .position(|header| header.contains("check signal"));
    let covered_signal_col = header_texts.iter().position(|header| {
        header.contains("signals covered")
            || (header.contains("covered") && header.contains("signal"))
    });
    if check_signal_col.is_some()
        && let Some(width_hint) = covered_signal_col
            .and_then(|col| row.get(col))
            .and_then(|cell| parse_width_hint_from_covered_signal_cell(&cell.text))
    {
        return Some(width_hint);
    }

    None
}

fn parse_width_hint_from_covered_signal_cell(text: &str) -> Option<WidthHint> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    let tokens = trimmed.split_whitespace().collect::<Vec<_>>();
    if let Some(first) = tokens.first() {
        let first_token = first.to_ascii_uppercase();
        if is_hardware_signal_token(&first_token) && tokens.len() > 1 {
            let remainder = tokens[1..].join(" ");
            if let Some(width_hint) = parse_table_width_hint_text(&remainder) {
                return Some(width_hint);
            }
        }
    }

    parse_table_width_hint_text(trimmed)
}

fn parse_table_width_hint_text(text: &str) -> Option<WidthHint> {
    let trimmed = text.trim();
    if trimmed.is_empty()
        || matches!(trimmed, "-" | "N/A" | "n/a")
        || trimmed.chars().all(|ch| ch == '.')
    {
        return None;
    }

    if let Ok(bits) = trimmed.parse::<u32>() {
        return (bits > 0).then_some(WidthHint::Numeric(bits));
    }

    if trimmed.chars().any(|ch| ch.is_ascii_alphabetic()) {
        return Some(WidthHint::Parametric(trimmed.to_string()));
    }

    None
}

/// Tier 2: Extract actor–signal relation triples from prose sentences using
/// verb-pattern matching.  Only sentences that mention a known signal name are
/// processed, keeping precision high.
///
/// Two pattern families are recognised:
///
/// **Passive drives** (signal is subject, actor is after a preposition):
///   `"PREADY is driven by the slave"`  →  (slave, Drives, PREADY)
///   `"RDATA is returned from the Completer"`  →  (Completer, Drives, RDATA)
///
/// **Active drives** (actor is sentence subject before the verb):
///   `"The Manager drives HTRANS"`  →  (Manager, Drives, HTRANS)
///   `"The Requester must drive PSEL"`  →  (Requester, Drives, PSEL)
///
/// **Passive reads** (signal is subject, actor samples after a preposition):
///   `"HREADY is sampled by the Manager"`  →  (Manager, Reads, HREADY)
///
/// **Active reads** (actor is sentence subject):
///   `"The Manager samples HREADY"`  →  (Manager, Reads, HREADY)
fn extract_actor_signal_relations(
    statements: &[ExtractedStatement],
    known_signals: &std::collections::HashSet<String>,
) -> Vec<ActorSignalRelation> {
    if known_signals.is_empty() {
        return Vec::new();
    }

    // The actor→signal relation verb vocabulary (the KG edge labels) is centralized
    // in `normative_vocab` — the single place to add a relation verb
    // (VERB-COVERAGE-CORPUS). Passive = `{signal} is {verb} by {actor}`; active =
    // `{actor} {verb} {signal}`. Verbs are grammar, not names (ADR 0006).
    use crate::ir::normative_vocab::{
        ACTIVE_DRIVES_VERBS, ACTIVE_READS_VERBS, PASSIVE_DRIVES_VERBS, PASSIVE_READS_VERBS,
    };

    let mut records = Vec::new();
    let mut counter = 1usize;
    let mut seen: std::collections::HashSet<(String, String, u8)> =
        std::collections::HashSet::new();

    // EVIDENCE-DETERMINISM.2 — iterate the signals in a DETERMINISTIC (sorted) order. Iterating the
    // `known_signals` HashSet directly made the relation `asr_NNNN` ids, the record order, and the
    // first-seen dedup representative depend on hash-iteration order, so the build was non-deterministic
    // run-to-run (`[[evidence-build-nondeterminism]]`). Sorting once here leaves the relation SET (the
    // `(actor, signal, kind)` dedup keys) unchanged — only the order / ids / attribution become stable.
    let mut sorted_signals: Vec<&String> = known_signals.iter().collect();
    sorted_signals.sort_unstable();

    for stmt in statements {
        // Skip synthesized declarations and table rows
        if stmt.text.starts_with("Signal ")
            || stmt.text.starts_with("Enum ")
            || stmt.text.starts_with('|')
            || stmt.text.starts_with('-')
        {
            continue;
        }

        let text = &stmt.text;
        let lowered = text.to_ascii_lowercase();

        for &signal in &sorted_signals {
            let sig_lower = signal.to_ascii_lowercase();
            if !lowered.contains(&sig_lower) {
                continue;
            }

            // ── Passive drives: "{sig} is {verb} by|from {actor}" ────────────
            // Direct full-pattern search: find the complete phrase then extract what follows.
            for verb in PASSIVE_DRIVES_VERBS {
                for prep in &["by", "from"] {
                    // Pattern: "{signal} is {verb} {prep} " with trailing space so actor starts right after
                    let full_pat = format!("{} is {} {} ", sig_lower, verb, prep);
                    if let Some(actor_start_in_lower) = lowered.find(&full_pat) {
                        let actor_start = actor_start_in_lower + full_pat.len();
                        if actor_start <= text.len() {
                            let after = &text[actor_start..];
                            if let Some(actor) = extract_actor_phrase(after) {
                                let key = (actor.clone(), signal.clone(), 0u8);
                                if seen.insert(key) {
                                    records.push(ActorSignalRelation {
                                        relation_id: format!("asr_{counter:04}"),
                                        actor_name: actor,
                                        signal_name: signal.clone(),
                                        relation: RelationKind::Drives,
                                        source_statement_ids: vec![stmt.statement_id.clone()],
                                        automation_confidence: AutomationConfidence::Medium,
                                    });
                                    counter += 1;
                                }
                            }
                        }
                    }
                }
            }

            // ── Passive reads: "{sig} is {verb} by|from {actor}" ─────────────
            for verb in PASSIVE_READS_VERBS {
                for preposition in &[" by ", " from "] {
                    let by_pat = format!("{} is {} {}", sig_lower, verb, preposition.trim());
                    if let Some(pat_pos) = lowered.find(&by_pat) {
                        let actor_start = pat_pos + by_pat.len();
                        if actor_start <= text.len()
                            && let Some(actor) = extract_actor_phrase(&text[actor_start..])
                        {
                            let key = (actor.clone(), signal.clone(), 1u8);
                            if seen.insert(key) {
                                records.push(ActorSignalRelation {
                                    relation_id: format!("asr_{counter:04}"),
                                    actor_name: actor,
                                    signal_name: signal.clone(),
                                    relation: RelationKind::Reads,
                                    source_statement_ids: vec![stmt.statement_id.clone()],
                                    automation_confidence: AutomationConfidence::Medium,
                                });
                                counter += 1;
                            }
                        }
                    }
                }
            }

            // ── Active drives: "{actor} {verb} {signal}" ────────────────────
            for verb in ACTIVE_DRIVES_VERBS {
                let active_verb_pat = format!(" {} ", verb);
                for (verb_pos, _) in lowered.match_indices(&active_verb_pat) {
                    let object_start = verb_pos + active_verb_pat.len();
                    if !active_object_contains_signal(&lowered, object_start, &sig_lower) {
                        continue;
                    }
                    let before = &text[..verb_pos];
                    if let Some(actor) = extract_subject_phrase(before) {
                        let key = (actor.clone(), signal.clone(), 0u8);
                        if seen.insert(key) {
                            records.push(ActorSignalRelation {
                                relation_id: format!("asr_{counter:04}"),
                                actor_name: actor,
                                signal_name: signal.clone(),
                                relation: RelationKind::Drives,
                                source_statement_ids: vec![stmt.statement_id.clone()],
                                automation_confidence: AutomationConfidence::Medium,
                            });
                            counter += 1;
                        }
                    }
                }

                // Look for " {verb} {signal}" in the lowered text
                let active_pat = format!(" {} {}", verb, sig_lower);
                if let Some(verb_end_pos) = lowered.find(&active_pat) {
                    // Subject is the text before verb_end_pos
                    let before = &text[..verb_end_pos];
                    if let Some(actor) = extract_subject_phrase(before) {
                        let key = (actor.clone(), signal.clone(), 0u8);
                        if seen.insert(key) {
                            records.push(ActorSignalRelation {
                                relation_id: format!("asr_{counter:04}"),
                                actor_name: actor,
                                signal_name: signal.clone(),
                                relation: RelationKind::Drives,
                                source_statement_ids: vec![stmt.statement_id.clone()],
                                automation_confidence: AutomationConfidence::Medium,
                            });
                            counter += 1;
                        }
                    }
                }
                // Also try "must {verb}" pattern: "the Requester must drive PSEL"
                let must_pat = format!(" must {} {}", verb, sig_lower);
                if let Some(verb_end_pos) = lowered.find(&must_pat) {
                    let before = &text[..verb_end_pos];
                    if let Some(actor) = extract_subject_phrase(before) {
                        let key = (actor.clone(), signal.clone(), 0u8);
                        if seen.insert(key) {
                            records.push(ActorSignalRelation {
                                relation_id: format!("asr_{counter:04}"),
                                actor_name: actor,
                                signal_name: signal.clone(),
                                relation: RelationKind::Drives,
                                source_statement_ids: vec![stmt.statement_id.clone()],
                                automation_confidence: AutomationConfidence::Medium,
                            });
                            counter += 1;
                        }
                    }
                }
            }

            // ── Active reads: "{actor} {verb} {signal}" ─────────────────────
            for verb in ACTIVE_READS_VERBS {
                let active_verb_pat = format!(" {} ", verb);
                for (verb_pos, _) in lowered.match_indices(&active_verb_pat) {
                    let object_start = verb_pos + active_verb_pat.len();
                    if !active_object_contains_signal(&lowered, object_start, &sig_lower) {
                        continue;
                    }
                    let before = &text[..verb_pos];
                    if let Some(actor) = extract_subject_phrase(before) {
                        let key = (actor.clone(), signal.clone(), 1u8);
                        if seen.insert(key) {
                            records.push(ActorSignalRelation {
                                relation_id: format!("asr_{counter:04}"),
                                actor_name: actor,
                                signal_name: signal.clone(),
                                relation: RelationKind::Reads,
                                source_statement_ids: vec![stmt.statement_id.clone()],
                                automation_confidence: AutomationConfidence::Medium,
                            });
                            counter += 1;
                        }
                    }
                }

                let active_pat = format!(" {} {}", verb, sig_lower);
                if let Some(verb_end_pos) = lowered.find(&active_pat) {
                    let before = &text[..verb_end_pos];
                    if let Some(actor) = extract_subject_phrase(before) {
                        let key = (actor.clone(), signal.clone(), 1u8);
                        if seen.insert(key) {
                            records.push(ActorSignalRelation {
                                relation_id: format!("asr_{counter:04}"),
                                actor_name: actor,
                                signal_name: signal.clone(),
                                relation: RelationKind::Reads,
                                source_statement_ids: vec![stmt.statement_id.clone()],
                                automation_confidence: AutomationConfidence::Medium,
                            });
                            counter += 1;
                        }
                    }
                }
            }
        }
    }

    records
}

fn active_object_contains_signal(lowered: &str, object_start: usize, sig_lower: &str) -> bool {
    let Some(object_text) = lowered.get(object_start..) else {
        return false;
    };
    let mut clause_end = object_text
        .find(['.', ';', '\n'])
        .unwrap_or(object_text.len());
    for marker in [
        " when ",
        " if ",
        " while ",
        " once ",
        " before ",
        " after ",
        " unless ",
        " according to ",
        " provided ",
    ] {
        if let Some(marker_start) = object_text[..clause_end].find(marker) {
            clause_end = marker_start;
        }
    }
    contains_signal_token(&object_text[..clause_end], sig_lower)
}

fn contains_signal_token(text: &str, sig_lower: &str) -> bool {
    text.match_indices(sig_lower).any(|(start, _)| {
        let before_is_token = text[..start]
            .chars()
            .next_back()
            .is_some_and(is_signal_name_char);
        let after_start = start + sig_lower.len();
        let after_is_token = text[after_start..]
            .chars()
            .next()
            .is_some_and(is_signal_name_char);
        !before_is_token && !after_is_token
    })
}

fn is_signal_name_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

/// Synthesize `Signal X is output.` declarations from the Drives triples in the
/// knowledge graph.  These are added to `extracted_statements` so they flow into
/// `SemanticIr::build_interfaces()` exactly like table-synthesized declarations.
///
/// Only unique (signal_name) entries are produced — duplicate Drives triples for
/// the same signal (different actor names) produce a single declaration.
fn synthesize_directions_from_relations(
    relations: &[ActorSignalRelation],
    already_declared: &std::collections::HashSet<String>,
    width_map: &std::collections::HashMap<String, WidthHint>,
    counter: &mut usize,
) -> Vec<ExtractedStatement> {
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut statements = Vec::new();

    for rel in relations {
        if !matches!(rel.relation, RelationKind::Drives) {
            continue;
        }
        // Skip signals already declared from signal description tables.
        // Table declarations are authoritative and must not be overwritten by
        // KG-derived declarations.
        if already_declared.contains(&rel.signal_name) {
            continue;
        }
        // One declaration per unique signal name — direction = output (from the driving actor).
        if seen.insert(rel.signal_name.clone()) {
            *counter += 1;
            // Include width from the table Width column if available.
            let text = match width_map.get(&rel.signal_name) {
                Some(WidthHint::Numeric(bits)) => {
                    format!("Signal {} is output width {bits}.", rel.signal_name)
                }
                Some(WidthHint::Parametric(expr)) => {
                    format!("Signal {} is output width {expr}.", rel.signal_name)
                }
                None => format!("Signal {} is output.", rel.signal_name),
            };
            statements.push(ExtractedStatement {
                statement_id: format!("statement_{counter:04}"),
                class: StatementClass::SourceFact,
                modality: EvidenceModality::Text,
                text,
                evidence_span_ids: rel.source_statement_ids.clone(),
                related_visual_evidence_ids: vec![],
            });
        }
    }
    statements
}

/// Extract the actor name from the text that follows a passive verb phrase
/// like `"is driven by "` or `"is asserted from "`.
/// Returns the first 1–3 meaningful words stripped of leading articles.
///
/// Examples:
///   `"the slave, which ..."` → `Some("slave")`
///   `"the Completer to indicate"` → `Some("Completer")`
///   `"a Manager or Subordinate"` → `Some("Manager")`
fn extract_actor_phrase(text: &str) -> Option<String> {
    let trimmed = text.trim_start();
    // Strip leading article determiners (the, a, an, this, its, each, all, every, any)
    let stripped = {
        let lowered = trimmed.to_ascii_lowercase();
        let mut result = trimmed;
        for prefix in &[
            "the ", "a ", "an ", "this ", "its ", "each ", "all ", "every ", "any ",
        ] {
            if lowered.starts_with(prefix) {
                result = &trimmed[prefix.len()..];
                break;
            }
        }
        result
    };
    // Collect words until a delimiter or stop-word
    const STOP_DELIMITERS: &[char] = &['.', ',', ';', '(', ')'];
    const STOP_WORDS: &[&str] = &[
        "to", "for", "and", "or", "in", "at", "on", "with", "when", "if", "by", "from", "that",
        "which", "where", "as", "is", "are", "has", "have", "will", "shall", "can", "may", "might",
        "must", "should", "could", "would",
    ];
    let mut words: Vec<&str> = Vec::new();
    for word in stripped.split_whitespace() {
        // Stop at punctuation
        let clean: &str = word.trim_end_matches(STOP_DELIMITERS);
        if clean.is_empty() {
            break;
        }
        // Stop at stop-words (but only after collecting at least one word)
        if !words.is_empty() && STOP_WORDS.contains(&clean.to_ascii_lowercase().as_str()) {
            break;
        }
        words.push(clean);
        if words.len() >= 2 {
            break; // two words is enough for compound actor names
        }
    }
    if words.is_empty() {
        return None;
    }
    let actor = words.join(" ");
    // Reject trivial / single-character results
    if actor.len() < 2 {
        return None;
    }
    normalize_relation_actor_name(&actor)
}

/// Extract the actor name from the text BEFORE an active verb phrase like
/// `"drives HTRANS"`.  Returns the last 1–2 meaningful words of the subject,
/// stripped of trailing articles and punctuation.
///
/// Examples:
///   `"The Manager"` → `Some("Manager")`
/// Generic protocol actor-role terms (universal role grammar, NOT chip-spec names — ADR 0006),
/// used to resolve a pronoun subject's antecedent. Mirrors the role vocabulary already treated as
/// non-signals in `is_signal_synthesis_non_signal`, broadened to the common cross-protocol roles.
fn is_canonical_actor_role(word: &str) -> bool {
    matches!(
        word.to_ascii_lowercase().as_str(),
        "manager"
            | "subordinate"
            | "requester"
            | "completer"
            | "initiator"
            | "target"
            | "master"
            | "slave"
            | "responder"
            | "decoder"
            | "arbiter"
            | "interconnect"
            | "bridge"
            | "transmitter"
            | "receiver"
            | "producer"
            | "consumer"
            | "controller"
            | "peripheral"
            | "host"
            | "device"
            | "agent"
    )
}

/// Anaphora: when the subject immediately governing the verb is a bare pronoun (`it`/`they`), the
/// real agent is the clause's main subject — resolve it to the FIRST canonical actor role appearing
/// before the pronoun. Returns `None` when the closest subject is not such a pronoun, or no actor
/// role precedes it (so nothing is invented). Universal pronoun/role grammar, not a chip name
/// (ADR 0006). `WIRE-BASED-100.5g` — fixes e.g. "After the Subordinate has sampled the address … it
/// can start to drive HREADYOUT" (previously mis-read the noun "address" as the actor).
fn resolve_pronoun_subject_anaphora(clause: &str) -> Option<String> {
    // Auxiliary / modal / infinitive helpers between the subject and the verb (skipped to find the
    // subject head closest to the verb). NB: the pronouns themselves are NOT listed here.
    const HELPERS: &[&str] = &[
        "can",
        "may",
        "might",
        "must",
        "should",
        "could",
        "would",
        "will",
        "shall",
        "start",
        "starts",
        "started",
        "then",
        "also",
        "to",
        "be",
        "is",
        "are",
        "has",
        "have",
        "had",
        "now",
        "only",
        "first",
        "immediately",
        "always",
        "subsequently",
        "begin",
        "begins",
    ];
    let tokens: Vec<&str> = clause.split_whitespace().collect();
    let clean = |t: &str| {
        t.trim_matches(|c: char| !c.is_ascii_alphabetic())
            .to_ascii_lowercase()
    };
    // Subject head = the last content token (skipping helpers), i.e. closest to the verb.
    let mut head_idx = None;
    for idx in (0..tokens.len()).rev() {
        let w = clean(tokens[idx]);
        if w.is_empty() || HELPERS.contains(&w.as_str()) {
            continue;
        }
        head_idx = Some(idx);
        break;
    }
    let head_idx = head_idx?;
    if !matches!(clean(tokens[head_idx]).as_str(), "it" | "they") {
        return None;
    }
    // Antecedent = the first canonical actor role appearing before the pronoun (the main subject).
    for token in &tokens[..head_idx] {
        let w = clean(token);
        if is_canonical_actor_role(&w) {
            return normalize_relation_actor_name(
                token.trim_matches(|c: char| !c.is_ascii_alphabetic()),
            );
        }
    }
    None
}

fn extract_subject_phrase(text: &str) -> Option<String> {
    const SKIP_WORDS: &[&str] = &[
        "the", "a", "an", "this", "that", "and", "or", "when", "if", ".", ",", ";", "(", ")", ":",
        "can", "may", "might", "must", "should", "could", "would", "will", "shall", "once",
        "before", "after", "while",
    ];
    const DETERMINERS: &[&str] = &[
        "the", "a", "an", "this", "that", "these", "those", "each", "every", "any",
    ];
    const SUBJECT_FOLLOWER_VERBS: &[&str] = &[
        "accept",
        "accepts",
        "activate",
        "activates",
        "apply",
        "applies",
        "assert",
        "asserts",
        "capture",
        "captures",
        "check",
        "checks",
        "control",
        "controls",
        "detect",
        "detects",
        "drive",
        "drives",
        "generate",
        "generates",
        "indicate",
        "indicates",
        "issue",
        "issues",
        "latch",
        "latches",
        "monitor",
        "monitors",
        "observe",
        "observes",
        "output",
        "outputs",
        "place",
        "places",
        "present",
        "presents",
        "produce",
        "produces",
        "provide",
        "provides",
        "read",
        "reads",
        "receive",
        "receives",
        "return",
        "returns",
        "sample",
        "samples",
        "send",
        "sends",
        "set",
        "sets",
        "source",
        "sources",
        "specify",
        "specifies",
        "supply",
        "supplies",
        "take",
        "takes",
        "transfer",
        "transfers",
        "transmit",
        "transmits",
    ];
    let mut subject_text = text;
    if let Some(sentence_start) = subject_text.rfind(['.', ';', '\n']) {
        subject_text = &subject_text[sentence_start + 1..];
    }

    let lowered_subject = subject_text.to_ascii_lowercase();
    for marker in [" which ", " that ", " who ", " whose "] {
        if let Some(relative_start) = lowered_subject.find(marker) {
            subject_text = &subject_text[..relative_start];
            break;
        }
    }

    // Pronoun-subject anaphora: if the subject closest to the verb is a bare pronoun ("it"/"they"),
    // resolve it to the clause's main actor before the determiner/backward heuristics pick a nearer
    // noun by mistake (WIRE-BASED-100.5g).
    if let Some(actor) = resolve_pronoun_subject_anaphora(subject_text) {
        return Some(actor);
    }

    let words: Vec<&str> = subject_text.split_whitespace().collect();

    for idx in (0..words.len()).rev() {
        let determiner = words[idx].trim_matches(|c: char| !c.is_ascii_alphabetic());
        let lower = determiner.to_ascii_lowercase();
        if !DETERMINERS.contains(&lower.as_str()) {
            continue;
        }

        let mut actor_words: Vec<&str> = Vec::new();
        for word in &words[idx + 1..] {
            let clean = word.trim_matches(|c: char| !c.is_ascii_alphabetic());
            if clean.is_empty() {
                break;
            }
            let lower = clean.to_ascii_lowercase();
            if SKIP_WORDS.contains(&lower.as_str()) {
                break;
            }
            if !actor_words.is_empty() && SUBJECT_FOLLOWER_VERBS.contains(&lower.as_str()) {
                break;
            }
            actor_words.push(clean);
            if actor_words.len() >= 2 {
                break;
            }
        }

        if !actor_words.is_empty()
            && let Some(actor) = normalize_relation_actor_name(&actor_words.join(" "))
        {
            return Some(actor);
        }
    }

    // Work backwards from the end to find the last meaningful word(s)
    let mut actor_words: Vec<&str> = Vec::new();
    for word in words.iter().rev() {
        let clean = word.trim_matches(|c: char| !c.is_ascii_alphabetic());
        if clean.is_empty() {
            break;
        }
        let lower = clean.to_ascii_lowercase();
        if SKIP_WORDS.contains(&lower.as_str()) {
            if !actor_words.is_empty() {
                break; // stop collecting after hitting an article
            }
            continue; // skip leading articles at the front of our backward scan
        }
        if actor_words.is_empty() && SUBJECT_FOLLOWER_VERBS.contains(&lower.as_str()) {
            continue;
        }
        actor_words.push(clean);
        if actor_words.len() >= 2 {
            break;
        }
    }
    if actor_words.is_empty() {
        return None;
    }
    actor_words.reverse();
    let actor = actor_words.join(" ");
    if actor.len() < 2 {
        return None;
    }
    normalize_relation_actor_name(&actor)
}

/// Returns `true` if the section title indicates boilerplate content
/// introduction, revision history, references, etc.) where normative language is used
/// for compliance purposes rather than hardware behavior constraints.
fn is_boilerplate_section_title(title: &str) -> bool {
    let lowered = title.to_ascii_lowercase();
    contains_any(
        &lowered,
        &[
            // Document preamble / meta content
            "introduction",
            "preface",
            "foreword",
            "scope",
            "about this",
            "how to read",
            "document organization",
            "document conventions",
            "document structure",
            // Legal / intellectual property
            "legal notice",
            "legal notices",
            "copyright",
            "patent",
            "license",
            "licence",
            "proprietary",
            "confidential",
            // Versioning / change tracking
            "revision history",
            "change history",
            "version history",
            "change log",
            "changelog",
            // Normative reference boilerplate
            "normative references",
            "informative references",
            "bibliography",
            "references",
            // Terminology / abbreviation glossaries
            "glossary",
            "acronyms",
            "abbreviations",
            "definitions",
            "terms and definitions",
            // Related document indexes
            "related documents",
            "related specifications",
            "related standards",
        ],
    )
}

fn classify_statement(text: &str) -> StatementClass {
    let lowered_text = text.to_ascii_lowercase();

    if contains_any(
        &lowered_text,
        &[
            "assume",
            "assuming",
            "modeled as",
            "modelled as",
            "abstracted",
            "treated as",
        ],
    ) {
        return StatementClass::ExplicitAbstraction;
    }
    if contains_any(
        &lowered_text,
        &[
            "therefore",
            "thus",
            "hence",
            "derived",
            "implies",
            "as a result",
        ],
    ) {
        return StatementClass::DerivedRule;
    }
    if contains_any(
        &lowered_text,
        &[
            "implementation note",
            "design choice",
            "local decision",
            "we choose",
            "we selected",
        ],
    ) {
        return StatementClass::LocalDesignDecision;
    }
    // Signal value constraints — most specific class; check before normative and conditional.
    // Pattern: HARDWARE_SIGNAL (must|shall) (be|remain|stay|become|not change) LOGIC_VALUE
    // or: HARDWARE_SIGNAL is (HIGH|LOW|asserted|deasserted) [when CONDITION]
    if is_signal_value_constraint(text) {
        return StatementClass::SignalValueConstraint;
    }

    // Timing constraints — check before normative so "shall be asserted within 2 cycles"
    // gets the more specific TimingConstraint class.
    if contains_any(
        &lowered_text,
        &[
            " cycle",
            "cycles",
            // Standard timing parameter abbreviations
            "tsu",
            "thd",
            "tckh",
            "tckl",
            "tco",
            "tpd",
            "toh",
            "tih",
            "setup time",
            "hold time",
            "clock period",
            "within n",
            "within one clock",
            "within two clock",
            "at least",
            "maximum latency",
            "propagation delay",
            // Edge-referenced timing
            "rising edge",
            "falling edge",
            "clock edge",
            "positive edge",
            "negative edge",
        ],
    ) && contains_any(
        &lowered_text,
        &["shall", "must", "cycle", "ns", "ps", "time", "edge"],
    ) {
        return StatementClass::TimingConstraint;
    }

    // Conditional behavioral rules.
    // Triggers on leading conditionals (When X, Y) and embedded conditionals (X when Y).
    // Also handles inverted conditionals (unless), duration (while/during/as long as),
    // and temporal ordering (after/before) when combined with a normative consequent.
    if (lowered_text.starts_with("when ")
        || lowered_text.starts_with("if ")
        || lowered_text.starts_with("unless ")
        || lowered_text.starts_with("while ")
        || lowered_text.starts_with("during ")
        || lowered_text.starts_with("after ")
        || lowered_text.starts_with("before ")
        || lowered_text.starts_with("provided that ")
        || lowered_text.starts_with("as long as ")
        || lowered_text.contains(" when ")
        || lowered_text.contains(" unless ")
        || lowered_text.contains("whenever ")
        || lowered_text.contains("in the event")
        || lowered_text.contains(" provided that ")
        || lowered_text.contains(" as long as "))
        && contains_any(
            &lowered_text,
            &["shall", "must", "cannot", "will", "assert", "deassert"],
        )
    {
        return StatementClass::ConditionalRule;
    }

    // Normative behavioral requirements — most important class for protocol specs.
    // Covers RFC 2119 modal verbs (shall/must) and common prohibition vocabulary
    // found in hardware specification documents.
    if contains_any(
        &lowered_text,
        &[
            // RFC 2119 obligation / prohibition
            "shall not",
            "must not",
            "shall ",
            "must ",
            "required to",
            "is required",
            "are required",
            "prohibited",
            // Common prohibition vocabulary in chip specs (not covered by shall/must)
            "cannot ",
            "can not ",
            "is not permitted",
            "are not permitted",
            "is not allowed",
            "are not allowed",
            "is not legal",
            "is not valid",
            "is forbidden",
            "is illegal",
            "may not ",
            "must never",
            "shall never",
            "will not ",
            "it is mandatory",
            "is not supported",
        ],
    ) {
        return StatementClass::NormativeStatement;
    }

    if text.trim().is_empty() {
        return StatementClass::Unknown;
    }

    StatementClass::SourceFact
}

fn normalize_text_key(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn contains_any(text: &str, candidates: &[&str]) -> bool {
    candidates.iter().any(|candidate| text.contains(candidate))
}

fn contains_reference_token(text: &str, token: &str) -> bool {
    for (match_index, _) in text.match_indices(token) {
        let prefix_ok = text[..match_index]
            .chars()
            .next_back()
            .map(|character| !character.is_ascii_alphanumeric())
            .unwrap_or(true);
        let suffix_index = match_index + token.len();
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

#[derive(Debug, Clone, Default)]
struct SignalPolarityFactCollection {
    resolved: HashMap<String, SignalPolarity>,
    resolved_records: Vec<SignalPolarityRecord>,
    conflicts: Vec<SignalPolarityConflictRecord>,
}

#[derive(Debug, Clone)]
struct SignalPolarityObservationCandidate {
    signal_name: String,
    polarity: SignalPolarity,
    source_kind: SignalPolarityEvidenceSourceKind,
    supporting_statement_ids: Vec<String>,
    supporting_table_ids: Vec<String>,
}

fn extract_enum_member_name(text: &str) -> Option<String> {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let normalized = normalized.trim().trim_end_matches('.');
    let tokens: Vec<&str> = normalized.split_whitespace().collect();
    if tokens.len() < 5 || !tokens[0].eq_ignore_ascii_case("enum") || tokens[3] != "=" {
        return None;
    }

    let member_name = tokens[2].trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_');
    if member_name.is_empty() {
        None
    } else {
        Some(member_name.to_ascii_uppercase())
    }
}

fn collect_discovered_enum_values(statement_groups: &[&[ExtractedStatement]]) -> HashSet<String> {
    let mut values = HashSet::new();
    for group in statement_groups {
        for statement in *group {
            if let Some(member_name) = extract_enum_member_name(&statement.text) {
                values.insert(member_name);
            }
        }
    }
    values
}

fn derive_encoding_enum_name(
    table: &crate::ir::source::StructuredTableRecord,
    section_title: &str,
    known_signals: Option<&HashSet<String>>,
) -> Option<String> {
    if let Some(known_signals) = known_signals {
        let caption_lower = table
            .caption_text
            .as_deref()
            .unwrap_or("")
            .to_ascii_lowercase();
        let section_lower = section_title.to_ascii_lowercase();
        let header_lower = table
            .header_rows
            .iter()
            .flatten()
            .map(|cell| cell.text.as_str())
            .collect::<Vec<_>>()
            .join(" ")
            .to_ascii_lowercase();
        // EVIDENCE-DETERMINISM.2 — sort longest-first with an alphabetical tie-break. Sorting by length
        // ALONE is a partial order: same-length candidates (e.g. `TDO`/`TDI`) stay tied and a stable sort
        // then preserves the non-deterministic `known_signals` HashSet order, so the first matching name
        // returned below was non-deterministic. A total order (length desc, then name) makes the chosen
        // enum name reproducible without changing which names are eligible (`[[evidence-build-nondeterminism]]`).
        let mut ordered_signals: Vec<&String> = known_signals.iter().collect();
        ordered_signals.sort_by(|a, b| b.len().cmp(&a.len()).then_with(|| a.cmp(b)));

        for signal in ordered_signals {
            let signal_lower = signal.to_ascii_lowercase();
            if contains_reference_token(&caption_lower, &signal_lower)
                || contains_reference_token(&section_lower, &signal_lower)
                || contains_reference_token(&header_lower, &signal_lower)
                || header_lower.contains(&format!("{signal_lower}["))
            {
                return Some(signal.clone());
            }
        }
    }

    let enum_name_source = table.caption_text.as_deref().unwrap_or(section_title);
    enum_name_source
        .split_whitespace()
        .find(|token| is_hardware_signal_token(&token.to_ascii_uppercase()))
        .map(|token| token.to_ascii_uppercase())
}

fn infer_encoding_column_indices(
    table: &crate::ir::source::StructuredTableRecord,
    enum_name: &str,
) -> (usize, usize) {
    let header_texts: Vec<String> = table
        .header_rows
        .first()
        .map(|row| {
            row.iter()
                .map(|cell| cell.text.to_ascii_lowercase())
                .collect()
        })
        .unwrap_or_default();
    let enum_name_lower = enum_name.to_ascii_lowercase();

    let mut name_col = header_texts.iter().position(|header| {
        header.contains("name")
            || header.contains("meaning")
            || header.contains("description")
            || header.contains("state")
            || header.contains("transfer")
            || header.contains("response")
            || header.contains("type")
    });
    let mut value_col = header_texts.iter().position(|header| {
        header.contains("value")
            || header.contains("encoding")
            || header.contains("code")
            || header.contains("binary")
            || header.contains("hex")
            || header.contains("bit")
            || contains_reference_token(header, &enum_name_lower)
            || header.contains(&format!("{enum_name_lower}["))
    });

    if value_col.is_none()
        && table
            .body_rows
            .iter()
            .filter_map(|row| row.first())
            .any(|cell| looks_like_encoding_literal(&cell.text))
    {
        value_col = Some(0);
    }

    let value_col = value_col.unwrap_or(0);
    if name_col.is_none() || name_col == Some(value_col) {
        name_col = (0..table.col_count as usize).find(|index| *index != value_col);
    }

    (name_col.unwrap_or(0), value_col)
}

fn parse_encoding_numeric_literal(text: &str) -> Option<u32> {
    let trimmed = text.trim().trim_matches(|c: char| c == '[' || c == ']');
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(value) = trimmed.parse::<u32>() {
        return Some(value);
    }

    let lowered = trimmed.to_ascii_lowercase();
    if let Some(bits) = lowered.strip_prefix("0b") {
        return u32::from_str_radix(bits, 2).ok();
    }
    if let Some(hex) = lowered.strip_prefix("0x") {
        return u32::from_str_radix(hex, 16).ok();
    }
    if let Some((_, bits)) = lowered.split_once("'b") {
        return u32::from_str_radix(bits, 2).ok();
    }
    if let Some((_, hex)) = lowered.split_once("'h") {
        return u32::from_str_radix(hex, 16).ok();
    }
    if lowered.chars().all(|c| matches!(c, '0' | '1')) {
        return u32::from_str_radix(&lowered, 2).ok();
    }

    None
}

fn looks_like_encoding_literal(text: &str) -> bool {
    let lowered = text.trim().to_ascii_lowercase().replace(' ', "");
    if lowered.is_empty() {
        return false;
    }
    if lowered.starts_with("0b")
        || lowered.starts_with("0x")
        || lowered.contains("'b")
        || lowered.contains("'h")
    {
        return true;
    }

    lowered
        .chars()
        .all(|c| matches!(c, '0' | '1' | 'x' | 'z' | '_' | '?'))
        && lowered.chars().any(|c| matches!(c, '0' | '1'))
}

fn table_looks_like_encoding(
    table: &crate::ir::source::StructuredTableRecord,
    anchor_signal: &str,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> bool {
    if matches!(
        effective_table_kind(table, prior_guidance),
        TableKind::Encoding
    ) {
        return true;
    }

    let header_texts: Vec<String> = table
        .header_rows
        .first()
        .map(|row| {
            row.iter()
                .map(|cell| cell.text.to_ascii_lowercase())
                .collect()
        })
        .unwrap_or_default();
    let has_name_column = header_texts.iter().any(|header| {
        header.contains("name")
            || header.contains("meaning")
            || header.contains("description")
            || header.contains("state")
            || header.contains("transfer")
            || header.contains("response")
            || header.contains("type")
    });
    let anchor_lower = anchor_signal.to_ascii_lowercase();
    let has_value_column = header_texts.iter().any(|header| {
        header.contains("value")
            || header.contains("encoding")
            || header.contains("code")
            || header.contains("binary")
            || header.contains("hex")
            || header.contains("bit")
            || contains_reference_token(header, &anchor_lower)
            || header.contains(&format!("{anchor_lower}["))
    });
    if has_name_column && has_value_column {
        return true;
    }

    let evidence_hits = table
        .body_rows
        .iter()
        .filter(|row| {
            row.iter()
                .any(|cell| looks_like_encoding_literal(&cell.text))
                || row.iter().any(|cell| {
                    let lowered = cell.text.to_ascii_lowercase();
                    lowered.contains(&format!("{anchor_lower}["))
                        || contains_reference_token(&lowered, &anchor_lower)
                })
        })
        .count();
    evidence_hits >= 2
}

fn scan_encoding_tables_by_signal_anchor(
    source_ir: &SourceIr,
    known_signals: &HashSet<String>,
    statement_counter: &mut usize,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<ExtractedStatement> {
    if known_signals.is_empty() || source_ir.structured_tables.is_empty() {
        return Vec::new();
    }

    let mut page_to_section: BTreeMap<u32, String> = BTreeMap::new();
    for section in &source_ir.document_sections {
        if let Some(page_num) = section
            .page_id
            .as_deref()
            .and_then(page_number_from_page_id)
        {
            page_to_section.insert(page_num, section.title.clone());
        }
    }

    let mut statements = Vec::new();
    for table in &source_ir.structured_tables {
        if matches!(
            effective_table_kind(table, prior_guidance),
            TableKind::SignalDescription | TableKind::RegisterMap | TableKind::TimingParameter
        ) {
            continue;
        }

        let table_page = table
            .page_id
            .as_deref()
            .and_then(page_number_from_page_id)
            .unwrap_or(0);
        let section_title = page_to_section
            .range(..=table_page)
            .next_back()
            .map(|(_, title)| title.clone())
            .unwrap_or_default();

        let Some(anchor_signal) =
            derive_encoding_enum_name(table, &section_title, Some(known_signals))
        else {
            continue;
        };
        if !table_looks_like_encoding(table, &anchor_signal, prior_guidance) {
            continue;
        }

        statements.extend(synthesize_encoding_declarations_for_enum(
            table,
            &anchor_signal,
            statement_counter,
        ));
    }

    statements
}

fn collect_subject_signal_tokens_with_discovered_values(
    text: &str,
    discovered_values: &HashSet<String>,
) -> Vec<String> {
    collect_subject_signal_tokens(text)
        .into_iter()
        .filter(|token| !discovered_values.contains(token))
        .collect()
}

fn extract_discovered_state_value_from_text(
    lowered: &str,
    discovered_values: &HashSet<String>,
) -> Option<String> {
    let mut ordered_values: Vec<&String> = discovered_values.iter().collect();
    ordered_values.sort_by_key(|value| std::cmp::Reverse(value.len()));
    for value in ordered_values {
        let value_lower = value.to_ascii_lowercase();
        if contains_any(
            lowered,
            &[
                &format!("must be {value_lower}"),
                &format!("shall be {value_lower}"),
                &format!("must remain {value_lower}"),
                &format!("shall remain {value_lower}"),
                &format!("is {value_lower} when"),
            ],
        ) {
            return Some(value.clone());
        }
    }
    None
}

fn extract_signal_polarity_from_prose(
    statements: &[ExtractedStatement],
    known_signals: &HashSet<String>,
) -> Vec<SignalPolarityObservationCandidate> {
    let mut observations = Vec::new();
    let mut ordered_signals: Vec<&String> = known_signals.iter().collect();
    ordered_signals.sort();

    for statement in statements {
        let lowered = statement.text.to_ascii_lowercase();
        let mut signal_polarities = Vec::new();

        if let Some(polarity) = detect_signal_polarity(&lowered) {
            let mentioned_signals = known_signals_referenced_in_text(&lowered, &ordered_signals);
            let signal_names = if mentioned_signals.len() == 1 {
                mentioned_signals
            } else {
                collective_polarity_subject_signals(&lowered, polarity, &ordered_signals)
            };

            signal_polarities.extend(
                signal_names
                    .into_iter()
                    .map(|signal_name| (signal_name, polarity)),
            );
        }

        if signal_polarities.is_empty() {
            signal_polarities = clause_local_polarity_subject_signals(&lowered, &ordered_signals);
        }

        for (signal_name, polarity) in signal_polarities {
            observations.push(SignalPolarityObservationCandidate {
                signal_name,
                polarity,
                source_kind: SignalPolarityEvidenceSourceKind::ProseStatement,
                supporting_statement_ids: vec![statement.statement_id.clone()],
                supporting_table_ids: Vec::new(),
            });
        }
    }

    observations
}

fn extract_signal_polarity_from_signal_tables(
    source_ir: &SourceIr,
    known_signals: &HashSet<String>,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<SignalPolarityObservationCandidate> {
    let mut observations = Vec::new();

    for table in &source_ir.structured_tables {
        if !should_treat_table_as_top_level_signal_description(source_ir, table, prior_guidance) {
            continue;
        }

        for row in &table.body_rows {
            let row_text = row
                .iter()
                .map(|cell| cell.text.trim())
                .filter(|text| !text.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            if row_text.is_empty() {
                continue;
            }

            let lowered = row_text.to_ascii_lowercase();
            let Some(polarity) = detect_signal_polarity(&lowered) else {
                continue;
            };
            let Some(signal_name) = signal_name_from_signal_table_row(row, known_signals) else {
                continue;
            };

            observations.push(SignalPolarityObservationCandidate {
                signal_name,
                polarity,
                source_kind: SignalPolarityEvidenceSourceKind::SignalDescriptionTable,
                supporting_statement_ids: Vec::new(),
                supporting_table_ids: vec![table.table_id.clone()],
            });
        }
    }

    observations
}

/// EXTRACTOR-ARCHITECTURE.9b — the prose active-level strategy (explicit asserted-when-level prose,
/// collective active-LOW/HIGH prose, safe clause-local mixed forms) as a registered `Extractor`. The
/// grammar self-gates: it emits nothing when no known signal carries polarity prose.
struct SignalPolarityProseExtractor<'a> {
    known_signals: &'a HashSet<String>,
}
impl Extractor<SignalPolarityObservationCandidate> for SignalPolarityProseExtractor<'_> {
    fn name(&self) -> &'static str {
        "signal_polarity.prose"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<SignalPolarityObservationCandidate> {
        extract_signal_polarity_from_prose(cx.statements, self.known_signals)
    }
}

/// EXTRACTOR-ARCHITECTURE.9b — the signal-table polarity strategy (active-level columns/cells in
/// signal-description tables) as a registered `Extractor`. Reads the structured tables, not statements.
struct SignalPolarityTableExtractor<'a> {
    source_ir: &'a SourceIr,
    known_signals: &'a HashSet<String>,
    prior_guidance: Option<&'a EvidencePriorGuidance>,
}
impl Extractor<SignalPolarityObservationCandidate> for SignalPolarityTableExtractor<'_> {
    fn name(&self) -> &'static str {
        "signal_polarity.tables"
    }
    fn run(&self, _cx: &ExtractionContext<'_>) -> Vec<SignalPolarityObservationCandidate> {
        extract_signal_polarity_from_signal_tables(
            self.source_ir,
            self.known_signals,
            self.prior_guidance,
        )
    }
}

/// Run the signal-polarity surface through the unified concat driver (EXTRACTOR-ARCHITECTURE.9b), then
/// arbitrate. Polarity is an OBSERVATION surface: the two strategies' candidates are concatenated in
/// strategy order (prose, then tables — exactly the legacy collection order) and the per-signal
/// accumulate-and-arbitrate post-pass decides consensus vs conflict; first-wins key-dedup would be wrong
/// here because a same-polarity observation from a second source STRENGTHENS the record (merged ids) and
/// a different-polarity observation must surface as an explicit conflict, never be dropped.
///
/// Called per pass of the convergence loop; `ExtractionManifest::record` replaces per surface name, so
/// the manifest ends up holding exactly the FINAL (converged) pass's run.
fn signal_polarity_surface(
    source_ir: &SourceIr,
    statements: &[ExtractedStatement],
    known_signals: &HashSet<String>,
    prior_guidance: Option<&EvidencePriorGuidance>,
    manifest: &mut ExtractionManifest,
) -> SignalPolarityFactCollection {
    let cx = ExtractionContext { statements };
    let prose = SignalPolarityProseExtractor { known_signals };
    let tables = SignalPolarityTableExtractor {
        source_ir,
        known_signals,
        prior_guidance,
    };
    let extractors: [&dyn Extractor<SignalPolarityObservationCandidate>; 2] = [&prose, &tables];
    let run = run_surface_concat("signal_polarities", &cx, &extractors);
    manifest.record(&run);
    arbitrate_signal_polarity_observations(run.records)
}

/// The polarity arbitration post-pass: group the concatenated observation candidates per signal (merging
/// same `(polarity, source_kind)` observations), then resolve — a single observed polarity becomes a
/// `SignalPolarityRecord`; disagreement becomes an explicit `SignalPolarityConflictRecord` (ids minted in
/// signal-name order). This is the unchanged body of the pre-`.9b` `collect_signal_polarity_facts`.
fn arbitrate_signal_polarity_observations(
    observations: Vec<SignalPolarityObservationCandidate>,
) -> SignalPolarityFactCollection {
    let mut observations_by_signal =
        BTreeMap::<String, Vec<SignalPolarityObservationRecord>>::new();

    for observation in observations {
        record_signal_polarity_observation(&mut observations_by_signal, observation);
    }

    let mut resolved = HashMap::new();
    let mut resolved_records = Vec::new();
    let mut conflicts = Vec::new();
    let mut conflict_counter = 1usize;

    for (signal_name, observations) in observations_by_signal {
        let polarities = observations
            .iter()
            .map(|observation| observation.polarity)
            .collect::<BTreeSet<_>>();
        if polarities.len() == 1 {
            if let Some(polarity) = polarities.iter().next().copied() {
                let mut supporting_statement_ids = observations
                    .iter()
                    .flat_map(|observation| observation.supporting_statement_ids.iter().cloned())
                    .collect::<Vec<_>>();
                let mut supporting_table_ids = observations
                    .iter()
                    .flat_map(|observation| observation.supporting_table_ids.iter().cloned())
                    .collect::<Vec<_>>();
                supporting_statement_ids.sort();
                supporting_statement_ids.dedup();
                supporting_table_ids.sort();
                supporting_table_ids.dedup();

                resolved.insert(signal_name.clone(), polarity);
                resolved_records.push(SignalPolarityRecord {
                    signal_name,
                    polarity,
                    supporting_statement_ids,
                    supporting_table_ids,
                    automation_confidence: AutomationConfidence::Medium,
                });
            }
            continue;
        }

        conflicts.push(SignalPolarityConflictRecord {
            conflict_id: format!("polarity_conflict_{conflict_counter:04}"),
            signal_name,
            observations,
            automation_confidence: AutomationConfidence::Medium,
        });
        conflict_counter += 1;
    }

    SignalPolarityFactCollection {
        resolved,
        resolved_records,
        conflicts,
    }
}

fn signal_name_from_signal_table_row(
    row: &[crate::ir::source::StructuredTableCellRecord],
    known_signals: &HashSet<String>,
) -> Option<String> {
    for cell in row {
        let trimmed = cell.text.trim();
        if trimmed.is_empty() {
            continue;
        }

        let candidate = trimmed
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_matches(|character: char| !character.is_ascii_alphanumeric() && character != '_')
            .to_ascii_uppercase();
        if candidate.is_empty() {
            continue;
        }
        if known_signals.contains(&candidate) {
            return Some(candidate);
        }
    }

    None
}

fn detect_signal_polarity(text_lower: &str) -> Option<SignalPolarity> {
    let has_active_low = text_lower.contains("active low")
        || text_lower.contains("active-low")
        || text_lower.contains("asserted low")
        || text_lower.contains("low asserted")
        || text_lower.contains("asserted when low")
        || text_lower.contains("low when asserted")
        || text_lower.contains("asserted by driving low")
        || text_lower.contains("driven low to assert");
    let has_active_high = text_lower.contains("active high")
        || text_lower.contains("active-high")
        || text_lower.contains("asserted high")
        || text_lower.contains("high asserted")
        || text_lower.contains("asserted when high")
        || text_lower.contains("high when asserted")
        || text_lower.contains("asserted by driving high")
        || text_lower.contains("driven high to assert");

    match (has_active_low, has_active_high) {
        (true, false) => Some(SignalPolarity::ActiveLow),
        (false, true) => Some(SignalPolarity::ActiveHigh),
        _ => None,
    }
}

fn known_signals_referenced_in_text(text_lower: &str, ordered_signals: &[&String]) -> Vec<String> {
    ordered_signals
        .iter()
        .filter_map(|signal| {
            let signal_lower = signal.to_ascii_lowercase();
            contains_reference_token(text_lower, &signal_lower).then_some((*signal).clone())
        })
        .collect()
}

fn collective_polarity_subject_signals(
    text_lower: &str,
    polarity: SignalPolarity,
    ordered_signals: &[&String],
) -> Vec<String> {
    let markers = match polarity {
        SignalPolarity::ActiveLow => [
            " are active low",
            " are active-low",
            " are asserted low",
            " are low asserted",
            " are asserted when low",
            " are low when asserted",
            " are asserted by driving low",
            " are driven low to assert",
        ],
        SignalPolarity::ActiveHigh => [
            " are active high",
            " are active-high",
            " are asserted high",
            " are high asserted",
            " are asserted when high",
            " are high when asserted",
            " are asserted by driving high",
            " are driven high to assert",
        ],
    };

    for marker in markers {
        let Some(index) = text_lower.find(marker) else {
            continue;
        };
        let subject = &text_lower[..index];
        let signals = known_signals_referenced_in_text(subject, ordered_signals);
        if signals.len() >= 2 {
            return signals;
        }
    }

    Vec::new()
}

fn clause_local_polarity_subject_signals(
    text_lower: &str,
    ordered_signals: &[&String],
) -> Vec<(String, SignalPolarity)> {
    let mentioned_signals = known_signals_referenced_in_text(text_lower, ordered_signals);
    if mentioned_signals.len() < 2 {
        return Vec::new();
    }

    let mut recovered_polarity_by_signal = BTreeMap::<String, SignalPolarity>::new();
    for clause in polarity_clause_segments(text_lower) {
        let clause = clause.trim();
        if clause.is_empty() {
            continue;
        }
        let polarity = detect_signal_polarity(clause);
        let clause_signals = known_signals_referenced_in_text(clause, ordered_signals);

        let Some(polarity) = polarity else {
            continue;
        };
        if clause_signals.len() != 1 {
            return Vec::new();
        }

        let signal_name = clause_signals[0].clone();
        match recovered_polarity_by_signal.get(&signal_name) {
            Some(existing) if *existing != polarity => return Vec::new(),
            Some(_) => {}
            None => {
                recovered_polarity_by_signal.insert(signal_name, polarity);
            }
        }
    }

    if recovered_polarity_by_signal.len() != mentioned_signals.len() {
        return Vec::new();
    }

    mentioned_signals
        .into_iter()
        .filter_map(|signal_name| {
            recovered_polarity_by_signal
                .get(&signal_name)
                .copied()
                .map(|polarity| (signal_name, polarity))
        })
        .collect()
}

fn polarity_clause_segments(text_lower: &str) -> Vec<String> {
    let mut clauses = vec![text_lower.to_string()];

    for delimiter in [";", ".", ",", " and ", " but ", " while ", " whereas "] {
        clauses = clauses
            .into_iter()
            .flat_map(|clause| {
                clause
                    .split(delimiter)
                    .map(str::trim)
                    .filter(|segment| !segment.is_empty())
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .collect();
    }

    clauses
}

fn record_signal_polarity_observation(
    observations_by_signal: &mut BTreeMap<String, Vec<SignalPolarityObservationRecord>>,
    observation: SignalPolarityObservationCandidate,
) {
    let entry = observations_by_signal
        .entry(observation.signal_name)
        .or_default();
    if let Some(existing) = entry.iter_mut().find(|existing| {
        existing.polarity == observation.polarity && existing.source_kind == observation.source_kind
    }) {
        merge_observation_ids(
            &mut existing.supporting_statement_ids,
            observation.supporting_statement_ids,
        );
        merge_observation_ids(
            &mut existing.supporting_table_ids,
            observation.supporting_table_ids,
        );
        return;
    }

    let mut record = SignalPolarityObservationRecord {
        polarity: observation.polarity,
        source_kind: observation.source_kind,
        supporting_statement_ids: observation.supporting_statement_ids,
        supporting_table_ids: observation.supporting_table_ids,
    };
    record.supporting_statement_ids.sort();
    record.supporting_statement_ids.dedup();
    record.supporting_table_ids.sort();
    record.supporting_table_ids.dedup();
    entry.push(record);
    entry.sort_by_key(|record| (record.source_kind, record.polarity));
}

fn merge_observation_ids(target: &mut Vec<String>, new_ids: Vec<String>) {
    target.extend(new_ids);
    target.sort();
    target.dedup();
}

fn apply_signal_polarity_to_constraints(
    constraints: &mut [SignalConstraintRecord],
    signal_polarity: &HashMap<String, SignalPolarity>,
) {
    for constraint in constraints {
        let Some(polarity) = signal_polarity.get(&constraint.subject_signal).copied() else {
            continue;
        };
        constraint.constraint_kind = match (&constraint.constraint_kind, polarity) {
            (SignalConstraintKind::MustBeAsserted, SignalPolarity::ActiveLow) => {
                SignalConstraintKind::MustBeLow
            }
            (SignalConstraintKind::MustBeAsserted, SignalPolarity::ActiveHigh) => {
                SignalConstraintKind::MustBeHigh
            }
            (SignalConstraintKind::MustBeDeasserted, SignalPolarity::ActiveLow) => {
                SignalConstraintKind::MustBeHigh
            }
            (SignalConstraintKind::MustBeDeasserted, SignalPolarity::ActiveHigh) => {
                SignalConstraintKind::MustBeLow
            }
            _ => constraint.constraint_kind.clone(),
        };
    }
}

/// Recognize a logic-level VALUE BINDING in an active construction the discovered-value /
/// "must be `<value>`" path misses — e.g. "the Requester must drive PSTRB LOW", "X is tied
/// HIGH". Returns the kind (`MustBeHigh`/`MustBeLow`) when a logic-level **word** (the
/// universal "how", LOGIC-LEVEL-BOUNDARY) is the object of a value-binding verb. Gated two
/// ways against over-generation: (1) a binding verb must be present, and (2) only alphabetic
/// word forms (`high`/`low`/`hi`/`lo`/`true`/`false`) count — never the numeric `1`/`0`, which
/// are ambiguous with bit indices. The last such word wins (the object position). `lowered`
/// is the lowercased subject clause (the condition clause is already stripped by the caller).
fn logic_level_binding_kind_from_text(lowered: &str) -> Option<SignalConstraintKind> {
    const BIND_VERBS: &[&str] = &[
        "drive", "driven", "drives", "set", "sets", "tied", "held", "pulled", "forced",
    ];
    // The bound level is the verb's OBJECT — it must sit within a few words *after* the bind
    // verb, not a distant condition clause ("… driven correctly every cycle in which X is True").
    const MAX_GAP: usize = 6;
    let words: Vec<&str> = lowered
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .collect();
    // Whole-word bind verb (so "set" does NOT match the substring in "reset").
    let bind_pos = words.iter().position(|w| BIND_VERBS.contains(w))?;
    let mut kind = None;
    for (i, word) in words.iter().enumerate().skip(bind_pos + 1) {
        if i - bind_pos > MAX_GAP {
            break;
        }
        // Alphabetic word forms only (exclude numeric 1/0 — bit indices).
        if word.len() < 2 || !word.chars().all(|c| c.is_ascii_alphabetic()) {
            continue;
        }
        // "active low" / "active high" describes polarity, not a must-be constraint.
        if i >= 1 && words[i - 1] == "active" {
            continue;
        }
        if crate::ir::normative_vocab::LOGIC_HIGH_VALUES.contains(word) {
            kind = Some(SignalConstraintKind::MustBeHigh);
        } else if crate::ir::normative_vocab::LOGIC_LOW_VALUES.contains(word) {
            kind = Some(SignalConstraintKind::MustBeLow);
        }
    }
    kind
}

fn extract_dynamic_signal_constraints(
    statements: &[ExtractedStatement],
    counter: &mut usize,
    discovered_values: &HashSet<String>,
) -> Vec<SignalConstraintRecord> {
    // NOTE: no early-return on empty `discovered_values` — the logic-level binding path
    // ("drive <signal> LOW/HIGH") finds constraints even when a doc declares no enum values.

    let mut records = Vec::new();
    // Same catalog gate as the pattern path: a constraint subject must be a DECLARED signal, so
    // property/config/doc-meta prose ("RME_Support must be False", "MPAM_WIDTH must be 11") is not
    // mined as a signal constraint (WIRE-BASED-100.5i). Skipped when no signals are declared.
    let declared_signals = collect_known_signal_names(statements);
    for statement in statements {
        if matches!(statement.class, StatementClass::SignalValueConstraint) {
            continue;
        }

        let lowered = statement.text.to_ascii_lowercase();
        let subject_part = text_before_condition_marker(&statement.text);

        // The bound value is either a discovered enum value (`must be <value>` → MustBeValue),
        // OR — in an active "drive/set/tied <signal> LOW/HIGH" construction the discovered-value
        // path misses — a logic level (→ MustBeHigh/Low; the universal "how",
        // LOGIC-LEVEL-BOUNDARY). The logic-level path is gated to binding verbs + the subject
        // clause so a bare mention of HIGH/LOW does not over-generate
        // (CONSTRAINT-EXTRACTION-V2 / drive-level recall).
        let (constraint_kind, target_value) = if let Some(value) =
            extract_discovered_state_value_from_text(&lowered, discovered_values)
        {
            (
                SignalConstraintKind::MustBeValue {
                    value: value.clone(),
                },
                Some(value),
            )
        } else if let Some(kind) =
            logic_level_binding_kind_from_text(&subject_part.to_ascii_lowercase())
        {
            (kind, None)
        } else {
            continue;
        };

        let mut subject_signals =
            collect_subject_signal_tokens_with_discovered_values(subject_part, discovered_values);
        if subject_signals.is_empty() {
            subject_signals = collect_subject_signal_tokens_with_discovered_values(
                &statement.text,
                discovered_values,
            );
        }
        if !declared_signals.is_empty() {
            subject_signals.retain(|s| declared_signals.contains(s));
        }
        if subject_signals.is_empty() {
            continue;
        }

        let condition_text = extract_condition_clause(&statement.text);
        let negated = contains_any(
            &lowered,
            &[
                "must not",
                "shall not",
                "must never",
                "shall never",
                "cannot",
                "will not",
            ],
        );

        for subject_signal in subject_signals {
            *counter += 1;
            records.push(SignalConstraintRecord {
                constraint_id: format!("dyn_sigcon_{counter:04}"),
                subject_signal,
                constraint_kind: constraint_kind.clone(),
                target_value: target_value.clone(),
                condition_text: condition_text.clone(),
                negated,
                source_text: statement.text.clone(),
                supporting_statement_ids: vec![statement.statement_id.clone()],
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    records
}

/// Synthesizes formal typed declarations from the structured table data captured in `SourceIR`.
///
/// This is the correct architectural layer for table-to-declaration conversion:
/// `SourceIR` holds the raw cell grids; `EvidenceIR` produces typed statements;
/// `SemanticIR` lifts from those statements using its existing parsers.
///
/// EXTRACTOR-ARCHITECTURE.5 — the signal-declaration SEED stage of the statement-ASSEMBLY phase, lifted out
/// of the ~500-line `EvidenceIr::build()` into one cohesive function. It runs the table-declaration strategy,
/// then the prose-declaration strategy as a FALLBACK only when the table catalog is sparse (`< 8` signals),
/// concatenating the results (no key-dedup — duplicates collapse downstream in the convergence loop) and
/// returning the synthesized seed statements plus the table-declaration provenance side-output.
///
/// This is deliberately NOT a `run_surface` merge-surface extractor: it produces seed `ExtractedStatement`s
/// (not typed surface records), mints synthetic statement ids via the build-wide counter, has a side-output,
/// and uses a cross-strategy fallback gate — the signature of the assembly phase, distinct from the typed
/// surface-extraction phase the `Extractor`/`run_surface` framework serves (see `ir/extractor.rs`). Forcing
/// it through the dedup driver would lose the provenance output and the fallback gate; keeping it a cohesive
/// assembly-phase function is the honest consolidation (shrinks the orchestrator, one home for the seed).
fn synthesize_signal_declaration_seed(
    source_ir: &SourceIr,
    statements: &[ExtractedStatement],
    statement_counter: &mut usize,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> (
    Vec<ExtractedStatement>,
    Vec<TableSignalDeclarationProvenanceRecord>,
) {
    let mut table_signal_declaration_provenance = Vec::new();
    // SWD-SERIAL-EXTRACTION.2: the table strategy seeds direct signal declarations + enum facts from
    // structured tables; the prose strategy (below) additionally captures interface signals declared in
    // PROSE (e.g. an appositive "a clock pin, SWCLK"), so serial specs get SWCLK/SWDIO into the catalog.
    let mut synthesized = synthesize_declarations_from_tables(
        source_ir,
        statement_counter,
        prior_guidance,
        &mut table_signal_declaration_provenance,
    );
    // The parenthetical prose form (`.3`) runs only as a FALLBACK when the table catalog is sparse
    // (few/no signal-description tables) — so table-rich specs are untouched (no regression).
    let table_signal_count = synthesized
        .iter()
        .filter(|s| s.text.starts_with("Signal "))
        .count();
    synthesized.extend(synthesize_signal_declarations_from_prose(
        statements,
        statement_counter,
        table_signal_count < 8,
    ));
    (synthesized, table_signal_declaration_provenance)
}

/// Currently handles two table kinds:
/// - `SignalDescription` → `Signal X is output/input [width N].` declarations
/// - `Encoding` → `Enum <name> <member> = <value>.` declarations
fn synthesize_declarations_from_tables(
    source_ir: &SourceIr,
    statement_counter: &mut usize,
    prior_guidance: Option<&EvidencePriorGuidance>,
    table_signal_declaration_provenance: &mut Vec<TableSignalDeclarationProvenanceRecord>,
) -> Vec<ExtractedStatement> {
    let mut statements = Vec::new();
    if source_ir.structured_tables.is_empty() {
        return statements;
    }

    // Build a page-number → (section_kind, section_title) lookup so we can infer
    // the semantic context of a table from the last section heading before it.
    let mut page_to_section: BTreeMap<u32, (SectionKind, String)> = BTreeMap::new();
    for section in &source_ir.document_sections {
        if let Some(page_num) = section
            .page_id
            .as_deref()
            .and_then(page_number_from_page_id)
        {
            page_to_section.insert(page_num, (section.section_kind, section.title.clone()));
        }
    }

    for table in &source_ir.structured_tables {
        let table_page = table
            .page_id
            .as_deref()
            .and_then(page_number_from_page_id)
            .unwrap_or(0);

        // Find the most recent section heading at or before this table's page.
        let (section_kind, section_title) = page_to_section
            .range(..=table_page)
            .next_back()
            .map(|(_, v)| v.clone())
            .unwrap_or((SectionKind::Unknown, String::new()));

        match effective_table_kind(table, prior_guidance) {
            TableKind::SignalDescription
                if should_treat_table_as_top_level_signal_description(
                    source_ir,
                    table,
                    prior_guidance,
                ) =>
            {
                statements.extend(synthesize_signal_declarations(
                    table,
                    section_kind,
                    &section_title,
                    statement_counter,
                    prior_guidance,
                    table_signal_declaration_provenance,
                ));
            }
            TableKind::Encoding => {
                statements.extend(synthesize_encoding_declarations(
                    table,
                    &section_title,
                    statement_counter,
                ));
            }
            _ => {}
        }
    }

    statements
}

/// Extract system contract declarations (clock signal, reset signal) from the
/// Description column of signal-description tables.
///
/// Pattern: the Description column of any `signal_description` table in AMBA
/// specs always has the first sentence identify the signal role:
///   - "Clock. PCLK is a clock signal..." → synthesise `"Clock PCLK."`
///   - "Reset. PRESETn is the reset signal and is active-LOW." → `"Reset PRESETn is asynchronous active low."`
///
/// The synthesized statements are processed by `parse_explicit_system_clock()`
/// and `parse_explicit_system_reset()` in SemanticIR without any downstream changes.
fn synthesize_system_contract_from_table_descriptions(
    source_ir: &SourceIr,
    statement_counter: &mut usize,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<ExtractedStatement> {
    let mut statements = Vec::new();
    let mut clock_found = false;
    let mut reset_found = false;

    'outer: for table in &source_ir.structured_tables {
        if !should_treat_table_as_top_level_signal_description(source_ir, table, prior_guidance) {
            continue;
        }

        for row in &table.body_rows {
            // Scan ALL cells in the row rather than relying on fixed column indices.
            //
            // Docling sometimes mis-assigns body cells to wrong column buckets when a
            // table has visually distinctive cells (bold/boxed signal names) whose
            // internal structure causes span-count arithmetic to shift.  The AHB
            // "Global signals" table is a confirmed instance: Docling places HCLK /
            // HRESETn in the last column even though they are in the first column of
            // the PDF.  Column-independent scanning is immune to this class of bug.
            //
            // Signal-name candidates: cells with ≤2 whitespace tokens where the first
            // token is a valid hardware signal name (not a role word like CLOCK/RESET).
            // This excludes description cells (many words) and role cells like
            // "Clock source" / "Reset controller" (first token in exclusion list).
            //
            // For clock/reset descriptions: among all cells whose text contains a
            // matching keyword, keep the longest one so that a rich description cell
            // ("The bus clock times all bus transfers …") wins over a short role cell
            // ("Clock source"), giving accurate polarity/kind inference for resets.
            let mut row_signal: Option<String> = None;
            let mut row_clock_desc: Option<String> = None;
            let mut row_reset_desc: Option<String> = None;

            for cell in row {
                let cell_text = cell.text.trim();
                if cell_text.is_empty() {
                    continue;
                }
                let cell_lower = cell_text.to_ascii_lowercase();
                let word_count = cell_text.split_whitespace().count();
                let first_token = cell_text
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_ascii_uppercase();

                // Signal name: short cell (name + optional footnote marker), valid token.
                if row_signal.is_none()
                    && word_count <= 2
                    && is_hardware_signal_token(&first_token)
                    && !is_signal_synthesis_non_signal(&first_token)
                {
                    row_signal = Some(first_token);
                }

                // Clock description — prefer longer / more informative text.
                if (cell_lower.starts_with("clock")
                    || cell_lower.contains("clock signal")
                    || cell_lower.contains("bus clock")
                    || cell_lower.contains("is a clock")
                    || cell_lower.contains("times all bus transfers")
                    || cell_lower.contains("timed against the rising edge")
                    || cell_lower.contains("sampled on the rising edge of")
                    || cell_lower.contains("related to the rising edge")
                    || cell_lower.contains("all signals are sampled")
                    || cell_lower.contains("all signal timings"))
                    && row_clock_desc
                        .as_ref()
                        .map(|d: &String| d.len())
                        .unwrap_or(0)
                        < cell_lower.len()
                {
                    row_clock_desc = Some(cell_lower.clone());
                }

                // Reset description — prefer longer / more informative text.
                if (cell_lower.starts_with("reset")
                    || cell_lower.contains("reset signal")
                    || cell_lower.contains("is the reset")
                    || cell_lower.contains("is a reset")
                    || cell_lower.contains("bus reset")
                    || (cell_lower.contains("is an active") && cell_lower.contains("reset")))
                    && row_reset_desc
                        .as_ref()
                        .map(|d: &String| d.len())
                        .unwrap_or(0)
                        < cell_lower.len()
                {
                    row_reset_desc = Some(cell_lower.clone());
                }
            }

            let Some(signal) = row_signal else {
                continue;
            };

            // ── Clock detection ────────────────────────────────────────────────
            if !clock_found && row_clock_desc.is_some() {
                *statement_counter += 1;
                statements.push(ExtractedStatement {
                    statement_id: format!("statement_{statement_counter:04}"),
                    class: StatementClass::SourceFact,
                    modality: EvidenceModality::Text,
                    text: format!("Clock {signal}."),
                    evidence_span_ids: vec![],
                    related_visual_evidence_ids: vec![],
                });
                clock_found = true;
            }

            // ── Reset detection ────────────────────────────────────────────────
            if !reset_found && let Some(desc) = row_reset_desc {
                // Polarity: explicit keyword wins; signal ending with N or B is
                // a secondary indicator (AMBA naming convention).
                let polarity = if desc.contains("active-low")
                    || desc.contains("active low")
                    || desc.contains("active_low")
                    || (!desc.contains("active-high")
                        && !desc.contains("active high")
                        && (signal.ends_with('N') || signal.ends_with('B')))
                {
                    "active low"
                } else {
                    "active high"
                };
                // Kind: explicit keyword wins; active-low AMBA resets are
                // conventionally asserted asynchronously.
                let explicit_async = desc.contains("asynchronous") || desc.contains("async");
                let explicit_sync = desc.contains("synchronous");
                let kind = if explicit_async || (!explicit_sync && polarity == "active low") {
                    "asynchronous"
                } else {
                    "synchronous"
                };
                *statement_counter += 1;
                statements.push(ExtractedStatement {
                    statement_id: format!("statement_{statement_counter:04}"),
                    class: StatementClass::SourceFact,
                    modality: EvidenceModality::Text,
                    text: format!("Reset {signal} is {kind} {polarity}."),
                    evidence_span_ids: vec![],
                    related_visual_evidence_ids: vec![],
                });
                reset_found = true;
            }

            if clock_found && reset_found {
                break 'outer;
            }
        }
    }

    statements
}

// EXTRACTOR-ARCHITECTURE.4 — the signal-semantic-hint cluster as a unified surface registry. The three
// meaning-inference strategies (signal-description tables, prose / alias-grounded prose, and visual captions /
// VLM timing-diagram annotations — the LLM/VLM "read the meaning" surfaces) are now
// `Extractor<SignalSemanticHintRecord>` units run by the shared `run_surface` driver, replacing the prior
// hand-rolled "run tables, then dedup-append prose, then dedup-append visual" merge. Registry ORDER is the
// legacy precedence (tables → prose → visual) and the surface key is `signal_semantic_hint_key` (the legacy
// dedup key), so the merged inventory is behavior-identical. Each strategy reads exactly the inputs it needs,
// carried on its extractor struct (the derived `known_signals`/`known_actor_names`, alias map, visual
// evidence, source, prior guidance); the shared `ExtractionContext` supplies `statements` to the prose path.

/// Meaning hints from signal-description tables — see `synthesize_signal_semantic_hints_from_tables`.
struct SemanticHintTableExtractor<'a> {
    source_ir: &'a SourceIr,
    known_signals: &'a HashSet<String>,
    known_actor_names: &'a BTreeSet<String>,
    prior_guidance: Option<&'a EvidencePriorGuidance>,
}
impl Extractor<SignalSemanticHintRecord> for SemanticHintTableExtractor<'_> {
    fn name(&self) -> &'static str {
        "semantic_hints.tables"
    }
    fn run(&self, _cx: &ExtractionContext<'_>) -> Vec<SignalSemanticHintRecord> {
        synthesize_signal_semantic_hints_from_tables(
            self.source_ir,
            self.known_signals,
            self.known_actor_names,
            self.prior_guidance,
        )
    }
}

/// Meaning hints from prose / alias-grounded prose — see `synthesize_signal_semantic_hints_from_prose`.
struct SemanticHintProseExtractor<'a> {
    signal_alias_map: &'a BTreeMap<String, String>,
    known_signals: &'a HashSet<String>,
    known_actor_names: &'a BTreeSet<String>,
    prior_guidance: Option<&'a EvidencePriorGuidance>,
}
impl Extractor<SignalSemanticHintRecord> for SemanticHintProseExtractor<'_> {
    fn name(&self) -> &'static str {
        "semantic_hints.prose"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<SignalSemanticHintRecord> {
        synthesize_signal_semantic_hints_from_prose(
            cx.statements,
            self.signal_alias_map,
            self.known_signals,
            self.known_actor_names,
            self.prior_guidance,
        )
    }
}

/// Meaning hints from visual captions / VLM timing-diagram annotations — see
/// `synthesize_signal_semantic_hints_from_visual_evidence`.
struct SemanticHintVisualExtractor<'a> {
    visual_evidence: &'a [VisualEvidenceItem],
    signal_alias_map: &'a BTreeMap<String, String>,
    known_signals: &'a HashSet<String>,
    known_actor_names: &'a BTreeSet<String>,
    prior_guidance: Option<&'a EvidencePriorGuidance>,
}
impl Extractor<SignalSemanticHintRecord> for SemanticHintVisualExtractor<'_> {
    fn name(&self) -> &'static str {
        "semantic_hints.visual"
    }
    fn run(&self, _cx: &ExtractionContext<'_>) -> Vec<SignalSemanticHintRecord> {
        synthesize_signal_semantic_hints_from_visual_evidence(
            self.visual_evidence,
            self.signal_alias_map,
            self.known_signals,
            self.known_actor_names,
            self.prior_guidance,
        )
    }
}

/// Run the signal-semantic-hint surface registry through the unified driver. Behavior-identical to the
/// legacy tables → prose → visual merge: first-wins dedup by `signal_semantic_hint_key`.
/// (`EXTRACTOR-ARCHITECTURE.4`)
#[allow(clippy::too_many_arguments)]
fn signal_semantic_hint_surface(
    statements: &[ExtractedStatement],
    source_ir: &SourceIr,
    visual_evidence: &[VisualEvidenceItem],
    signal_alias_map: &BTreeMap<String, String>,
    known_signals: &HashSet<String>,
    known_actor_names: &BTreeSet<String>,
    prior_guidance: Option<&EvidencePriorGuidance>,
    manifest: &mut ExtractionManifest,
) -> Vec<SignalSemanticHintRecord> {
    let cx = ExtractionContext { statements };
    let tables = SemanticHintTableExtractor {
        source_ir,
        known_signals,
        known_actor_names,
        prior_guidance,
    };
    let prose = SemanticHintProseExtractor {
        signal_alias_map,
        known_signals,
        known_actor_names,
        prior_guidance,
    };
    let visual = SemanticHintVisualExtractor {
        visual_evidence,
        signal_alias_map,
        known_signals,
        known_actor_names,
        prior_guidance,
    };
    let extractors: [&dyn Extractor<SignalSemanticHintRecord>; 3] = [&tables, &prose, &visual];
    let run = run_surface(
        "signal_semantic_hints",
        &cx,
        &extractors,
        signal_semantic_hint_key,
    );
    manifest.record(&run);
    run.records
}

#[allow(clippy::too_many_arguments)]
fn synthesize_signal_semantic_hints(
    source_ir: &SourceIr,
    statements: &[ExtractedStatement],
    actor_signal_relations: &[ActorSignalRelation],
    signal_alias_map: &BTreeMap<String, String>,
    visual_evidence: &[VisualEvidenceItem],
    prior_guidance: Option<&EvidencePriorGuidance>,
    manifest: &mut ExtractionManifest,
) -> (
    Vec<SignalSemanticHintRecord>,
    Vec<SignalSemanticConflictRecord>,
) {
    let known_actor_names = collect_known_actor_names_for_semantic_hints(
        source_ir,
        actor_signal_relations,
        prior_guidance,
    );
    let known_signals =
        collect_known_signal_names_for_semantic_hints(source_ir, statements, prior_guidance);
    // EXTRACTOR-ARCHITECTURE.4 — the three meaning-inference strategies run through the unified `run_surface`
    // driver (tables → prose → visual, first-wins dedup by `signal_semantic_hint_key`); conflict detection
    // stays a post-merge step on the merged inventory.
    let hints = signal_semantic_hint_surface(
        statements,
        source_ir,
        visual_evidence,
        signal_alias_map,
        &known_signals,
        &known_actor_names,
        prior_guidance,
        manifest,
    );
    let conflicts = detect_signal_semantic_conflicts(&hints);
    (hints, conflicts)
}

fn signal_semantic_hint_key(hint: &SignalSemanticHintRecord) -> String {
    let mut visual_ids = hint.supporting_visual_evidence_ids.clone();
    visual_ids.sort();
    visual_ids.dedup();
    format!(
        "{}:{}:{}:{}:{}",
        hint.signal_name,
        match hint.source_kind {
            SignalSemanticHintSourceKind::SignalDescriptionTable => "signal_description_table",
            SignalSemanticHintSourceKind::ProseStatement => "prose_statement",
            SignalSemanticHintSourceKind::AliasGroundedProseStatement =>
                "alias_grounded_prose_statement",
            SignalSemanticHintSourceKind::VisualCaption => "visual_caption",
            SignalSemanticHintSourceKind::VlmTimingDiagramAnnotation =>
                "vlm_timing_diagram_annotation",
        },
        hint.source_text,
        visual_ids.join(","),
        hint.semantic_tags
            .iter()
            .map(|tag| tag.as_str())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn collect_known_signal_names_for_semantic_hints(
    source_ir: &SourceIr,
    statements: &[ExtractedStatement],
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> HashSet<String> {
    let mut known_signals = collect_known_signal_names(statements);
    known_signals.extend(collect_signal_names_from_tables(source_ir, prior_guidance));
    known_signals
}

fn collect_known_actor_names_for_semantic_hints(
    source_ir: &SourceIr,
    actor_signal_relations: &[ActorSignalRelation],
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> BTreeSet<String> {
    let mut actor_names = actor_signal_relations
        .iter()
        .map(|relation| relation.actor_name.clone())
        .collect::<BTreeSet<_>>();

    for table in &source_ir.structured_tables {
        if !should_treat_table_as_top_level_signal_description(source_ir, table, prior_guidance) {
            continue;
        }

        let header_texts: Vec<String> = table
            .header_rows
            .first()
            .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
            .unwrap_or_default();
        let relation_cols = header_texts
            .iter()
            .enumerate()
            .filter_map(|(idx, header)| {
                if header.contains("source")
                    || header.contains("driver")
                    || header.contains("destination")
                    || header.contains("dest")
                {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for row in &table.body_rows {
            for relation_col in &relation_cols {
                if let Some(cell) = row.get(*relation_col)
                    && let Some(actor_name) = normalize_table_actor_name(&cell.text)
                {
                    actor_names.insert(actor_name);
                }
            }
        }
    }

    for section in &source_ir.document_sections {
        let lowered = section.title.to_ascii_lowercase();
        for suffix in [
            " signals", " signal", " inputs", " input", " outputs", " output",
        ] {
            if lowered.ends_with(suffix) {
                let trimmed = section
                    .title
                    .get(..section.title.len().saturating_sub(suffix.len()))
                    .unwrap_or("")
                    .trim();
                if !trimmed.is_empty() {
                    actor_names.insert(trimmed.to_string());
                }
            }
        }
    }

    actor_names
}

fn synthesize_signal_semantic_hints_from_tables(
    source_ir: &SourceIr,
    known_signals: &HashSet<String>,
    known_actor_names: &BTreeSet<String>,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<SignalSemanticHintRecord> {
    let mut hints = Vec::new();
    let mut seen = BTreeSet::<String>::new();

    for table in &source_ir.structured_tables {
        if !should_treat_table_as_top_level_signal_description(source_ir, table, prior_guidance) {
            continue;
        }

        let header_texts: Vec<String> = table
            .header_rows
            .first()
            .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
            .unwrap_or_default();
        let name_col = header_texts
            .iter()
            .position(|h| {
                h.contains("signal")
                    || h.contains("name")
                    || h.contains("port")
                    || h.contains("pin")
            })
            .unwrap_or(0);
        let description_col = header_texts.iter().position(|h| {
            h.contains("description") || h.contains("meaning") || h.contains("function")
        });
        let Some(description_col) = description_col else {
            continue;
        };

        for row in &table.body_rows {
            let Some(name_cell) = row.get(name_col) else {
                continue;
            };
            let signal_name = name_cell
                .text
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_ascii_uppercase();
            if !is_hardware_signal_token(&signal_name)
                || is_signal_synthesis_non_signal(&signal_name)
            {
                continue;
            }

            let Some(description_cell) = row.get(description_col) else {
                continue;
            };
            let description = description_cell.text.trim();
            if description.is_empty() {
                continue;
            }

            let sanitized_description = strip_signal_mentions_from_semantic_hint_text(
                description,
                known_signals.iter().map(String::as_str),
            );
            let semantic_tags = infer_signal_semantic_tags_from_description(
                &sanitized_description,
                description,
                SignalSemanticHintSourceKind::SignalDescriptionTable,
                &BTreeSet::from([signal_name.clone()]),
                known_actor_names,
                prior_guidance,
            );
            if semantic_tags.is_empty() {
                continue;
            }

            let key = format!(
                "{}:{}:{}",
                signal_name,
                table.table_id,
                semantic_tags
                    .iter()
                    .map(|tag| tag.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            if !seen.insert(key) {
                continue;
            }

            hints.push(SignalSemanticHintRecord {
                signal_name,
                semantic_tags,
                source_kind: SignalSemanticHintSourceKind::SignalDescriptionTable,
                source_text: description.to_string(),
                supporting_statement_ids: Vec::new(),
                supporting_table_ids: vec![table.table_id.clone()],
                supporting_visual_evidence_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    hints
}

fn synthesize_signal_semantic_hints_from_prose(
    statements: &[ExtractedStatement],
    signal_alias_map: &BTreeMap<String, String>,
    known_signals: &HashSet<String>,
    known_actor_names: &BTreeSet<String>,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<SignalSemanticHintRecord> {
    let mut hints = Vec::new();
    let mut seen = BTreeSet::<String>::new();

    for statement in statements {
        if !matches!(statement.class, StatementClass::SourceFact) {
            continue;
        }

        for target_context in extract_signal_semantic_target_contexts(
            &statement.text,
            known_signals,
            signal_alias_map,
        ) {
            let source_kind = if target_context.alias_grounded {
                SignalSemanticHintSourceKind::AliasGroundedProseStatement
            } else {
                SignalSemanticHintSourceKind::ProseStatement
            };
            let sanitized_text = strip_signal_mentions_from_semantic_hint_text(
                &target_context.context_text,
                known_signals.iter().map(String::as_str),
            );
            let semantic_tags = infer_signal_semantic_tags_from_description(
                &sanitized_text,
                &target_context.context_text,
                source_kind,
                &BTreeSet::from([target_context.signal_name.clone()]),
                known_actor_names,
                prior_guidance,
            );
            if semantic_tags.is_empty() {
                continue;
            }

            let key = format!(
                "{}:{}:{}:{}",
                target_context.signal_name,
                statement.statement_id,
                source_kind.as_str(),
                semantic_tags
                    .iter()
                    .map(|tag| tag.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            if !seen.insert(key) {
                continue;
            }

            hints.push(SignalSemanticHintRecord {
                signal_name: target_context.signal_name,
                semantic_tags,
                source_kind,
                source_text: statement.text.clone(),
                supporting_statement_ids: vec![statement.statement_id.clone()],
                supporting_table_ids: Vec::new(),
                supporting_visual_evidence_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Low,
            });
        }
    }

    hints
}

fn synthesize_signal_semantic_hints_from_visual_evidence(
    visual_evidence: &[VisualEvidenceItem],
    signal_alias_map: &BTreeMap<String, String>,
    known_signals: &HashSet<String>,
    known_actor_names: &BTreeSet<String>,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<SignalSemanticHintRecord> {
    let mut hints = Vec::new();
    let mut seen = BTreeSet::<String>::new();

    for visual_item in visual_evidence {
        if let Some(caption_text) = visual_item.caption_text.as_deref() {
            push_visual_signal_semantic_hint(
                &mut hints,
                &mut seen,
                caption_text,
                SignalSemanticHintSourceKind::VisualCaption,
                &visual_item.evidence_id,
                known_signals,
                known_actor_names,
                signal_alias_map,
                AutomationConfidence::Low,
                prior_guidance,
            );
        }

        for observation in &visual_item.observations {
            if !matches!(
                observation.kind,
                VisualObservationKind::TimingDiagramExtraction
            ) {
                continue;
            }
            let Some(json_value) = parse_visual_observation_json(&observation.text) else {
                continue;
            };
            let Some(annotations) = json_value
                .get("annotations")
                .and_then(|value| value.as_array())
            else {
                continue;
            };
            for annotation in annotations {
                let Some(annotation_text) = annotation.as_str() else {
                    continue;
                };
                push_visual_signal_semantic_hint(
                    &mut hints,
                    &mut seen,
                    annotation_text,
                    SignalSemanticHintSourceKind::VlmTimingDiagramAnnotation,
                    &visual_item.evidence_id,
                    known_signals,
                    known_actor_names,
                    signal_alias_map,
                    AutomationConfidence::Low,
                    prior_guidance,
                );
            }
        }
    }

    hints
}

#[expect(
    clippy::too_many_arguments,
    reason = "visual semantic hint emission keeps evidence provenance, modality, and prior guidance explicit"
)]
fn push_visual_signal_semantic_hint(
    hints: &mut Vec<SignalSemanticHintRecord>,
    seen: &mut BTreeSet<String>,
    source_text: &str,
    source_kind: SignalSemanticHintSourceKind,
    visual_evidence_id: &str,
    known_signals: &HashSet<String>,
    known_actor_names: &BTreeSet<String>,
    signal_alias_map: &BTreeMap<String, String>,
    automation_confidence: AutomationConfidence,
    prior_guidance: Option<&EvidencePriorGuidance>,
) {
    for target_context in
        extract_signal_semantic_target_contexts(source_text, known_signals, signal_alias_map)
    {
        let sanitized_text = strip_signal_mentions_from_semantic_hint_text(
            &target_context.context_text,
            known_signals.iter().map(String::as_str),
        );
        let semantic_tags = infer_signal_semantic_tags_from_description(
            &sanitized_text,
            &target_context.context_text,
            source_kind,
            &BTreeSet::from([target_context.signal_name.clone()]),
            known_actor_names,
            prior_guidance,
        );
        if semantic_tags.is_empty() {
            continue;
        }

        let key = format!(
            "{}:{}:{}:{}",
            target_context.signal_name,
            visual_evidence_id,
            source_kind.as_str(),
            semantic_tags
                .iter()
                .map(|tag| tag.as_str())
                .collect::<Vec<_>>()
                .join(",")
        );
        if !seen.insert(key) {
            continue;
        }

        hints.push(SignalSemanticHintRecord {
            signal_name: target_context.signal_name,
            semantic_tags,
            source_kind,
            source_text: source_text.to_string(),
            supporting_statement_ids: Vec::new(),
            supporting_table_ids: Vec::new(),
            supporting_visual_evidence_ids: vec![visual_evidence_id.to_string()],
            automation_confidence,
        });
    }
}

fn detect_signal_semantic_conflicts(
    hints: &[SignalSemanticHintRecord],
) -> Vec<SignalSemanticConflictRecord> {
    let mut hints_by_signal = BTreeMap::<String, Vec<&SignalSemanticHintRecord>>::new();
    for hint in hints {
        hints_by_signal
            .entry(hint.signal_name.clone())
            .or_default()
            .push(hint);
    }

    let mut conflicts = Vec::new();
    let mut conflict_counter = 1usize;
    for (signal_name, observations) in hints_by_signal {
        let distinct_tags = observations
            .iter()
            .flat_map(|hint| hint.semantic_tags.iter().copied())
            .collect::<BTreeSet<_>>();
        if distinct_tags.len() < 2 {
            continue;
        }

        conflicts.push(SignalSemanticConflictRecord {
            conflict_id: format!("semantic_conflict_{conflict_counter:04}"),
            signal_name,
            observations: observations
                .into_iter()
                .map(|hint| SignalSemanticConflictObservationRecord {
                    semantic_tags: hint.semantic_tags.clone(),
                    source_kind: hint.source_kind,
                    source_text: hint.source_text.clone(),
                    supporting_statement_ids: hint.supporting_statement_ids.clone(),
                    supporting_table_ids: hint.supporting_table_ids.clone(),
                    supporting_visual_evidence_ids: hint.supporting_visual_evidence_ids.clone(),
                })
                .collect(),
            automation_confidence: AutomationConfidence::Medium,
        });
        conflict_counter += 1;
    }

    conflicts
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SignalSemanticTargetContext {
    signal_name: String,
    alias_grounded: bool,
    context_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SignalSemanticTargetMention {
    signal_name: String,
    alias_grounded: bool,
    start: usize,
    end: usize,
}

fn extract_signal_semantic_target_contexts(
    text: &str,
    known_signals: &HashSet<String>,
    signal_alias_map: &BTreeMap<String, String>,
) -> Vec<SignalSemanticTargetContext> {
    let mentions = collect_signal_semantic_target_mentions(text, known_signals, signal_alias_map);
    if mentions.is_empty() {
        return Vec::new();
    }

    let mut contexts = Vec::new();
    let distinct_signals = mentions
        .iter()
        .map(|mention| mention.signal_name.as_str())
        .collect::<BTreeSet<_>>();

    if distinct_signals.len() == 1 {
        let signal_name = mentions[0].signal_name.clone();
        contexts.push(SignalSemanticTargetContext {
            signal_name,
            alias_grounded: mentions.iter().any(|mention| mention.alias_grounded),
            context_text: text.to_string(),
        });
        return contexts;
    }

    let mut seen = BTreeSet::new();
    for (index, mention) in mentions.iter().enumerate() {
        let context_text = extract_signal_semantic_context_window(text, &mentions, index);
        let context_text = context_text.trim();
        if context_text.is_empty() {
            continue;
        }
        let key = format!(
            "{}:{}:{}",
            mention.signal_name,
            mention.alias_grounded,
            normalize_text_key(context_text)
        );
        if !seen.insert(key) {
            continue;
        }
        contexts.push(SignalSemanticTargetContext {
            signal_name: mention.signal_name.clone(),
            alias_grounded: mention.alias_grounded,
            context_text: context_text.to_string(),
        });
    }

    contexts
}

fn collect_signal_semantic_target_mentions(
    text: &str,
    known_signals: &HashSet<String>,
    signal_alias_map: &BTreeMap<String, String>,
) -> Vec<SignalSemanticTargetMention> {
    let lowered = text.to_ascii_lowercase();
    let mut mentions = Vec::new();

    let mut ordered_signals: Vec<&String> = known_signals.iter().collect();
    ordered_signals.sort_by_key(|signal_name| std::cmp::Reverse(signal_name.len()));
    for signal_name in ordered_signals {
        let token = signal_name.to_ascii_lowercase();
        for (start, end) in find_reference_spans(&lowered, &token) {
            mentions.push(SignalSemanticTargetMention {
                signal_name: signal_name.clone(),
                alias_grounded: false,
                start,
                end,
            });
        }
    }

    let mut ordered_aliases: Vec<(&String, &String)> = signal_alias_map.iter().collect();
    ordered_aliases.sort_by_key(|(alias_phrase, _)| std::cmp::Reverse(alias_phrase.len()));
    for (alias_phrase, signal_name) in ordered_aliases {
        let alias_lower = alias_phrase.to_ascii_lowercase();
        for (start, end) in find_reference_spans(&lowered, &alias_lower) {
            mentions.push(SignalSemanticTargetMention {
                signal_name: signal_name.clone(),
                alias_grounded: true,
                start,
                end,
            });
        }
    }

    let signals_with_direct_mentions = mentions
        .iter()
        .filter(|mention| !mention.alias_grounded)
        .map(|mention| mention.signal_name.clone())
        .collect::<HashSet<_>>();
    if !signals_with_direct_mentions.is_empty() {
        mentions.retain(|mention| {
            !mention.alias_grounded || !signals_with_direct_mentions.contains(&mention.signal_name)
        });
    }

    mentions.sort_by(|left, right| {
        left.start
            .cmp(&right.start)
            .then_with(|| (right.end - right.start).cmp(&(left.end - left.start)))
            .then_with(|| right.alias_grounded.cmp(&left.alias_grounded))
            .then_with(|| left.signal_name.cmp(&right.signal_name))
    });

    let mut collapsed: Vec<SignalSemanticTargetMention> = Vec::new();
    for mention in mentions {
        if let Some(previous) = collapsed.last()
            && mention.start < previous.end
        {
            continue;
        }
        collapsed.push(mention);
    }
    collapsed
}

fn find_reference_spans(text: &str, token: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    for (match_index, _) in text.match_indices(token) {
        let prefix_ok = text[..match_index]
            .chars()
            .next_back()
            .map(|character| !character.is_ascii_alphanumeric())
            .unwrap_or(true);
        let suffix_index = match_index + token.len();
        let suffix_ok = text[suffix_index..]
            .chars()
            .next()
            .map(|character| !character.is_ascii_alphanumeric())
            .unwrap_or(true);
        if prefix_ok && suffix_ok {
            spans.push((match_index, suffix_index));
        }
    }
    spans
}

fn extract_signal_semantic_context_window<'a>(
    text: &'a str,
    mentions: &[SignalSemanticTargetMention],
    index: usize,
) -> &'a str {
    let lowered = text.to_ascii_lowercase();
    let current = &mentions[index];

    let start = if let Some(previous) = index.checked_sub(1).and_then(|idx| mentions.get(idx)) {
        if let Some((separator_offset, separator_len)) =
            find_last_clause_separator(&lowered[previous.end..current.start])
        {
            previous.end + separator_offset + separator_len
        } else {
            current.start
        }
    } else {
        0
    };

    let end = if let Some(next) = mentions.get(index + 1) {
        if let Some((separator_offset, _separator_len)) =
            find_first_clause_separator(&lowered[current.end..next.start])
        {
            current.end + separator_offset
        } else {
            current.end
        }
    } else {
        text.len()
    };

    text[start..end].trim()
}

fn find_first_clause_separator(text: &str) -> Option<(usize, usize)> {
    clause_separator_offsets(text)
        .into_iter()
        .min_by_key(|(start, _)| *start)
}

fn find_last_clause_separator(text: &str) -> Option<(usize, usize)> {
    clause_separator_offsets(text)
        .into_iter()
        .max_by_key(|(start, _)| *start)
}

fn clause_separator_offsets(text: &str) -> Vec<(usize, usize)> {
    const SEPARATORS: &[&str] = &[
        " and ",
        " but ",
        " while ",
        " whereas ",
        ";",
        ",",
        "\n",
        ".",
    ];
    let mut offsets = Vec::new();
    for separator in SEPARATORS {
        for (start, _) in text.match_indices(separator) {
            offsets.push((start, separator.len()));
        }
    }
    offsets
}

fn infer_signal_semantic_tags_from_description(
    description: &str,
    source_text_for_prior_matching: &str,
    source_kind: SignalSemanticHintSourceKind,
    signal_names: &BTreeSet<String>,
    actor_names: &BTreeSet<String>,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<SignalSemanticTag> {
    if !matches!(
        source_kind,
        SignalSemanticHintSourceKind::SignalDescriptionTable
    ) && looks_like_structural_contents_entry_for_semantic_hint(source_text_for_prior_matching)
    {
        return Vec::new();
    }

    let lowered = description.to_ascii_lowercase();
    let mut tags = BTreeSet::new();

    if contains_any(
        &lowered,
        &[
            " valid",
            "valid ",
            "information is available",
            "data is available",
            "address is available",
            "control is available",
            "request is pending",
            "request pending",
            "request present",
            "transaction request",
            "transfer request",
        ],
    ) {
        tags.insert(SignalSemanticTag::HandshakeValidLike);
    }

    if contains_any(
        &lowered,
        &[
            " ready",
            "ready ",
            "can accept",
            "able to accept",
            "accept the transfer",
            "accept transfer",
            "accept data",
            "accept address",
            "complete the transfer",
            "transfer can complete",
        ],
    ) || contains_ready_like_acknowledgment_phrase(&lowered)
    {
        tags.insert(SignalSemanticTag::HandshakeReadyLike);
    }

    if let Some(role) = prior_guidance.and_then(|prior_guidance| {
        prior_guidance.corpus_memory.semantic_phrase_role_in_text(
            Some(prior_guidance.protocol_family),
            source_kind,
            source_text_for_prior_matching,
            signal_names,
            actor_names,
        )
    }) {
        tags.insert(semantic_tag_for_role(role));
    }

    tags.into_iter().collect()
}

fn looks_like_structural_contents_entry_for_semantic_hint(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();
    lowered.contains(". . .")
        || lowered.contains("table of contents")
        || lowered.contains("| |")
            && text.matches('|').count() >= 4
            && text.chars().any(|character| character.is_ascii_digit())
}

fn contains_ready_like_acknowledgment_phrase(lowered: &str) -> bool {
    contains_any(
        lowered,
        &[
            "acknowledge the request",
            "acknowledges the request",
            "acknowledged the request",
            "acknowledge request",
            "acknowledges request",
            "acknowledged request",
            "acknowledge the transfer",
            "acknowledges the transfer",
            "acknowledged the transfer",
            "acknowledge transfer",
            "acknowledges transfer",
            "acknowledged transfer",
            "acknowledge receipt",
            "acknowledges receipt",
            "acknowledged receipt",
            "request acknowledged",
            "transfer acknowledged",
        ],
    )
}

fn semantic_tag_for_role(role: InterfaceSignalSemanticRole) -> SignalSemanticTag {
    match role {
        InterfaceSignalSemanticRole::HandshakeValidLike => SignalSemanticTag::HandshakeValidLike,
        InterfaceSignalSemanticRole::HandshakeReadyLike => SignalSemanticTag::HandshakeReadyLike,
    }
}

fn strip_signal_mentions_from_semantic_hint_text<'a>(
    text: &str,
    signal_names: impl IntoIterator<Item = &'a str>,
) -> String {
    let signal_tokens = signal_names
        .into_iter()
        .map(|name| name.trim().to_ascii_uppercase())
        .filter(|name| !name.is_empty())
        .collect::<HashSet<_>>();
    if signal_tokens.is_empty() {
        return text.to_string();
    }

    let mut output = String::with_capacity(text.len());
    let mut token = String::new();

    let flush_token = |token: &mut String, output: &mut String, signal_tokens: &HashSet<String>| {
        if token.is_empty() {
            return;
        }
        if signal_tokens.contains(&token.to_ascii_uppercase()) {
            output.push(' ');
        } else {
            output.push_str(token);
        }
        token.clear();
    };

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            token.push(ch);
        } else {
            flush_token(&mut token, &mut output, &signal_tokens);
            output.push(ch);
        }
    }
    flush_token(&mut token, &mut output, &signal_tokens);

    output
}

fn collect_hardware_signal_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    let flush_token = |current: &mut String, tokens: &mut Vec<String>| {
        if current.is_empty() {
            return;
        }
        let token = current.to_ascii_uppercase();
        if is_hardware_signal_token(&token)
            && !is_signal_synthesis_non_signal(&token)
            && !tokens.contains(&token)
        {
            tokens.push(token);
        }
        current.clear();
    };

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            current.push(ch);
        } else {
            flush_token(&mut current, &mut tokens);
        }
    }
    flush_token(&mut current, &mut tokens);

    tokens
}

fn collect_related_check_table_signal_tokens(
    covered_signal_text: &str,
    check_enable_text: Option<&str>,
    granularity_text: Option<&str>,
    relations_by_signal: &HashMap<String, Vec<(String, RelationKind)>>,
) -> Vec<String> {
    let covered_signals = collect_hardware_signal_tokens(covered_signal_text)
        .into_iter()
        .filter(|signal_name| relations_by_signal.contains_key(signal_name))
        .collect::<Vec<_>>();
    if !covered_signals.is_empty() {
        return covered_signals;
    }

    [check_enable_text, granularity_text]
        .into_iter()
        .flatten()
        .flat_map(collect_hardware_signal_tokens)
        .filter(|signal_name| relations_by_signal.contains_key(signal_name))
        .fold(Vec::new(), |mut signals, signal_name| {
            if !signals.contains(&signal_name) {
                signals.push(signal_name);
            }
            signals
        })
}

/// Infer signal direction from a section kind + title for signal description tables.
fn direction_for_actor_taxonomy_role(
    role: ActorTaxonomyRole,
    column_kind: RelationTableColumnKind,
) -> &'static str {
    match (column_kind, role) {
        (RelationTableColumnKind::SourceLike, ActorTaxonomyRole::RequesterLike) => "output",
        (RelationTableColumnKind::SourceLike, ActorTaxonomyRole::CompleterLike) => "input",
        (RelationTableColumnKind::DestinationLike, ActorTaxonomyRole::RequesterLike) => "input",
        (RelationTableColumnKind::DestinationLike, ActorTaxonomyRole::CompleterLike) => "output",
    }
}

fn builtin_actor_taxonomy_role_in_text(text: &str) -> Option<ActorTaxonomyRole> {
    let normalized = normalize_actor_term(text);
    if normalized.is_empty() {
        return None;
    }

    let requester_like = ["manager", "initiator", "master", "requester"]
        .iter()
        .any(|term| normalized_text_contains_term(&normalized, term));
    let completer_like = [
        "subordinate",
        "slave",
        "responder",
        "multiplexor",
        "completer",
        "target",
    ]
    .iter()
    .any(|term| normalized_text_contains_term(&normalized, term));

    match (requester_like, completer_like) {
        (true, false) => Some(ActorTaxonomyRole::RequesterLike),
        (false, true) => Some(ActorTaxonomyRole::CompleterLike),
        _ => None,
    }
}

fn actor_taxonomy_role_in_text(
    text: &str,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Option<ActorTaxonomyRole> {
    builtin_actor_taxonomy_role_in_text(text).or_else(|| {
        prior_guidance.and_then(|prior_guidance| {
            prior_guidance
                .corpus_memory
                .actor_taxonomy_role_in_text(Some(prior_guidance.protocol_family), text)
        })
    })
}

fn infer_signal_direction_from_actor_text(
    actor_text: &str,
    column_kind: RelationTableColumnKind,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Option<&'static str> {
    let lowered = actor_text.to_ascii_lowercase();
    if is_tie_off_actor_text(actor_text) {
        return Some("input");
    }
    if lowered.contains("output") {
        return Some("output");
    }
    if lowered.contains("input") {
        return Some("input");
    }
    if lowered.contains("clock")
        || lowered.contains("reset")
        || lowered.contains("system bus")
        || lowered.contains("global")
    {
        return Some("input");
    }

    actor_taxonomy_role_in_text(actor_text, prior_guidance)
        .map(|role| direction_for_actor_taxonomy_role(role, column_kind))
}

/// Infer interface direction from a signal-description cell's prose, for tables that
/// carry NO direction / source / width column (e.g. CHI's two-column
/// `Signal | Description` channel tables).
///
/// AMBA/CHI channel descriptions name the *driver* of the signal in prose:
///   - "Request Flit Valid. The transmitter sets this signal HIGH …" → transmitter-driven
///   - "Request L-Credit Valid. The receiver sets this signal HIGH …" → receiver-driven
///
/// Framed relative to the channel transmitter (the subject of a "<X> channel interface
/// signals" table): a transmitter-driven signal is an `output`; a receiver-driven
/// signal is an `input`. Returns `None` when no driver is stated, or when both are
/// (ambiguous) — an honest residual rather than a fabricated direction.
fn infer_signal_direction_from_description_prose(description: &str) -> Option<&'static str> {
    let lowered = description.to_ascii_lowercase();
    let driver_stated = |actor: &str| -> bool {
        [
            "sets this signal",
            "drives this signal",
            "asserts this signal",
            "generates this signal",
        ]
        .iter()
        .any(|verb| lowered.contains(&format!("{actor} {verb}")))
            || lowered.contains(&format!("driven by the {actor}"))
            || lowered.contains(&format!("asserted by the {actor}"))
            || lowered.contains(&format!("set by the {actor}"))
    };
    let transmitter_driven = driver_stated("transmitter") || driver_stated("source");
    let receiver_driven = driver_stated("receiver") || driver_stated("destination");
    match (transmitter_driven, receiver_driven) {
        (true, false) => Some("output"),
        (false, true) => Some("input"),
        // No driver stated, or both stated (ambiguous): leave as an honest residual.
        _ => None,
    }
}

fn infer_signal_direction_from_section(
    kind: SectionKind,
    title: &str,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Option<&'static str> {
    // Explicit section kind takes priority.
    match kind {
        SectionKind::SignalDescription => {}
        SectionKind::Boilerplate | SectionKind::Glossary | SectionKind::TableOfContents => {
            return None;
        }
        _ => {}
    }
    let lowered = title.to_ascii_lowercase();
    // Infrastructure signals (clock, reset, global decoder) are distributed
    // into all blocks — treat as input.
    if lowered.contains("global")
        || lowered.contains("system")
        || lowered.contains("clock")
        || lowered.contains("reset")
        || lowered.contains("decoder")
    {
        return Some("input");
    }

    actor_name_and_role_from_section_heading(title, prior_guidance).map(|(_actor_name, role)| {
        direction_for_actor_taxonomy_role(role, RelationTableColumnKind::SourceLike)
    })
}

/// Returns true if the token looks like a hardware signal name:
/// all-uppercase with optional digits and underscores, at least 2 chars, and STARTING WITH A
/// LETTER. The leading-letter rule drops binary/number literals like `0B0`/`0B1` (which pass the
/// uppercase-or-digit test via the `B`) that a value cell otherwise mis-declares as a signal —
/// real signal identifiers always start with a letter (WIRE-BASED-100.5j).
pub(crate) fn is_hardware_signal_token(token: &str) -> bool {
    token.len() >= 2
        && token
            .chars()
            .next()
            .map(|c| c.is_ascii_uppercase())
            .unwrap_or(false)
        && token
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

/// Level 2 NLP — Extract `SignalConstraintRecord` entries from `SignalValueConstraint` sentences.
/// Operates only on already-classified sentences to keep precision high.
///
/// Multi-signal support: if a sentence constrains multiple signals simultaneously
/// (e.g. "Both HTRANS and HADDR shall be stable"), a separate record is created
/// for each. The condition clause is stripped first so that signal names appearing
/// in the condition (e.g. HREADY in "...when HREADY is LOW") are not confused
/// with subjects.
fn extract_signal_constraints(
    statements: &[ExtractedStatement],
    counter: &mut usize,
) -> Vec<SignalConstraintRecord> {
    let mut records = Vec::new();
    // The document's own declared-signal catalog decides what is a signal: a constraint subject
    // must be a declared signal. This drops property/config/doc-meta prose mis-read as signal
    // constraints (e.g. AXI "RME_Support must be False" → "RME"; "MPAM_WIDTH must be 11" → "MPAM";
    // "granted to LICENSEE" → "LICENSEE") without any hardcoded list (ADR 0006). WIRE-BASED-100.5i.
    let declared_signals = collect_known_signal_names(statements);

    for statement in statements {
        if !matches!(statement.class, StatementClass::SignalValueConstraint) {
            continue;
        }
        let text = &statement.text;
        let lowered = text.to_ascii_lowercase();

        // Narrow to the sentence carrying the constraint verb (so unrelated earlier
        // sentences/clauses don't contribute false subjects), drop any leading antecedent
        // ("<cond>, which means <obligation>"), THEN strip the trailing condition clause
        // ("when X" / "until X" / "if X" / …).
        let subject_part = text_before_condition_marker(consequent_after_inference_marker(
            constraint_bearing_sentence(text),
        ));

        // Collect ALL valid signal tokens from the subject part, creating one record each.
        // Fall back to scanning the full text if no signals found in the subject part.
        let mut subject_signals = collect_subject_signal_tokens(subject_part);
        if subject_signals.is_empty() {
            // The full-text fallback can otherwise grab a signal that appears ONLY in
            // the stripped condition clause ("The following signals must be valid when
            // PSEL is asserted") and mis-attribute the constraint to it. Exclude any
            // token from the condition clause so the condition signal never becomes the
            // subject (CONSTRAINT-CONDITION-SUBJECT; real-APB NLI finding).
            let sentence = constraint_bearing_sentence(text);
            let condition_clause = &sentence[text_before_condition_marker(sentence).len()..];
            let condition_signals: std::collections::HashSet<String> =
                collect_subject_signal_tokens(condition_clause)
                    .into_iter()
                    .collect();
            subject_signals = collect_subject_signal_tokens(text)
                .into_iter()
                .filter(|tok| !condition_signals.contains(tok))
                .collect();
        }

        if subject_signals.is_empty() {
            continue;
        }
        // Determine constraint kind and negation from the value-binding phrase.
        let negated = contains_any(
            &lowered,
            &[
                "must not",
                "shall not",
                "must never",
                "shall never",
                "cannot",
                "will not",
            ],
        );

        let constraint_kind = if contains_any(
            &lowered,
            &[
                "must not change",
                "shall not change",
                "must remain stable",
                "shall remain stable",
            ],
        ) {
            SignalConstraintKind::MustNotChange
        } else if contains_any(
            &lowered,
            &[
                "must be stable",
                "shall be stable",
                "must hold",
                "shall hold",
            ],
        ) {
            SignalConstraintKind::MustBeStable
        } else if contains_any(
            &lowered,
            &[
                "must be high",
                "shall be high",
                "must remain high",
                "shall remain high",
                "must be driven high",
            ],
        ) {
            SignalConstraintKind::MustBeHigh
        } else if contains_any(
            &lowered,
            &[
                "must be low",
                "shall be low",
                "must remain low",
                "shall remain low",
                "must be driven low",
            ],
        ) {
            SignalConstraintKind::MustBeLow
        } else if contains_any(
            &lowered,
            &[
                "must be asserted",
                "shall be asserted",
                "must remain asserted",
                "shall remain asserted",
            ],
        ) {
            SignalConstraintKind::MustBeAsserted
        } else if contains_any(
            &lowered,
            &[
                "must be deasserted",
                "shall be deasserted",
                "must remain deasserted",
                "shall remain deasserted",
            ],
        ) {
            SignalConstraintKind::MustBeDeasserted
        } else if contains_any(&lowered, &["must be valid", "shall be valid"]) {
            // Look for a specific protocol state value after "must be" / "shall be"
            if let Some(value) = extract_protocol_state_value(&lowered) {
                SignalConstraintKind::MustBeValue { value }
            } else {
                SignalConstraintKind::MustBeStable
            }
        } else {
            // Generic: try to find a protocol state value
            if let Some(value) = extract_protocol_state_value(&lowered) {
                SignalConstraintKind::MustBeValue { value }
            } else {
                SignalConstraintKind::MustBeStable
            }
        };

        // The constraint's own value (the token after the normative verb) is not a
        // subject signal — exclude it positionally so value *names* need never be
        // denylisted (ADR 0006). "<signal> must be <value>" → the subject is the
        // signal, never the value.
        if let Some(value) = extract_protocol_state_value(&lowered) {
            subject_signals.retain(|s| !s.eq_ignore_ascii_case(&value));
        }
        // Keep only subjects that are DECLARED signals — a property/config name or doc-meta token
        // (AXI "RME_Support", "MPAM_WIDTH", "LICENSEE") is not in the catalog and is dropped
        // (WIRE-BASED-100.5i). Skipped when the document declares no signals at all (e.g. a tiny
        // fixture), so a no-catalog corpus is not silently emptied.
        if !declared_signals.is_empty() {
            subject_signals.retain(|s| declared_signals.contains(s));
        }
        if subject_signals.is_empty() {
            continue;
        }

        // A kind that already encodes its own negation (`MustNotChange`, `MustBeDeasserted`)
        // must NOT also carry `negated = true`: the "not"/"de-" is part of the obligation, so a
        // `negated` flag on top would read as a double negative ("must not change" → "may
        // change"). `negated` is reserved for kinds whose plain form is affirmative
        // (`MustBeAsserted`/`MustBeHigh`/…) inverted by an explicit "not" (WIRE-BASED-100.5b).
        let negated = negated
            && !matches!(
                constraint_kind,
                SignalConstraintKind::MustNotChange | SignalConstraintKind::MustBeDeasserted
            );

        // Extract condition clause: text after "when", "while", "during", "unless" — from the
        // SAME bounded obligation the subject came from, so a later table-cell bullet does not
        // bleed into the condition (CONSTRAINT-EXTRACTION-V2.2).
        let condition_text = extract_condition_clause(consequent_after_inference_marker(
            constraint_bearing_sentence(text),
        ));

        // Create one record per subject signal (multi-signal sentences).
        for subject_signal in subject_signals {
            *counter += 1;
            records.push(SignalConstraintRecord {
                constraint_id: format!("sigcon_{counter:04}"),
                subject_signal,
                constraint_kind: constraint_kind.clone(),
                target_value: None,
                condition_text: condition_text.clone(),
                negated,
                source_text: text.clone(),
                supporting_statement_ids: vec![statement.statement_id.clone()],
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    records
}

/// Return the portion of `text` before the first condition-clause marker
/// (" when ", " while ", " during ", " unless ", " provided ", " after ", " before ").
/// Returns the full text if no marker is found.
/// Narrow a (possibly multi-sentence) statement to the sentence that carries the
/// constraint verb (`must`/`shall`), so signal names from unrelated earlier sentences
/// or clauses are not swept in as false subjects. Falls back to the whole text when no
/// sentence carries a constraint verb.
///
/// Example: in "… PREADY is asserted … at the rising edge of PCLK …. PADDR, PWDATA …
/// must be stable …" only the second sentence (the one with `must`) is the subject
/// source, so PREADY/PCLK are not minted as `must_be_stable` subjects.
fn constraint_bearing_sentence(text: &str) -> &str {
    // Split on bullets (`•`) and newlines as well as `.`/`;`, so a multi-obligation table
    // cell ("… • PAUSER must be valid when … • PAUSER must have the same value …") yields one
    // obligation per clause rather than one constraint carrying the whole cell
    // (CONSTRAINT-EXTRACTION-V2.2).
    for sentence in text.split(['.', ';', '•', '\n']) {
        let lowered = sentence.to_ascii_lowercase();
        if lowered.contains("must") || lowered.contains("shall") {
            return sentence;
        }
    }
    text
}

/// If the sentence states an antecedent and then *infers* an obligation
/// (`"<antecedent>, which means <obligation>"`), return the consequent — the obligation's
/// subject lives there, not in the antecedent. Otherwise the text is returned unchanged.
/// (CONSTRAINT-EXTRACTION-V2.3: real-APB `"PSEL is asserted, which means PADDR, PWRITE, and
/// PWDATA must be valid"` previously yielded a bogus `"PSEL must be VALID"`.)
fn consequent_after_inference_marker(text: &str) -> &str {
    let lowered = text.to_ascii_lowercase();
    for marker in [
        " which means that ",
        " which means ",
        " which implies that ",
        " which implies ",
        ", meaning that ",
    ] {
        if let Some(pos) = lowered.find(marker) {
            return text[pos + marker.len()..].trim_start();
        }
    }
    text
}

fn text_before_condition_marker(text: &str) -> &str {
    let lowered_bytes = text.to_ascii_lowercase();
    // Cut at the EARLIEST condition marker (not the first in list order), so signals in
    // any trailing condition clause are excluded from the subject. `until`/`if` are
    // included because constraints like "X must remain asserted until Y ... if Z ..."
    // otherwise leak Y/Z as false subjects.
    let mut cut = text.len();
    for marker in &[
        " when ",
        " while ",
        " during ",
        " unless ",
        " provided ",
        " after ",
        " before ",
        " until ",
        " if ",
    ] {
        if let Some(pos) = lowered_bytes.find(marker) {
            cut = cut.min(pos);
        }
    }
    &text[..cut]
}

/// Collect all uppercase hardware signal tokens from a text fragment.
/// Excludes logic-level values (HIGH/LOW), protocol state names (NONSEQ/SEQ/...),
/// protocol family names (AHB/AXI/...), and document structure words.
/// A logic-level word (`HIGH`/`LOW`/`TRUE`/`FALSE`/…) is a universal binary-logic VALUE — the
/// centralized, owner-confirmed `normative_vocab` "how" — never a signal name, so it must not be
/// collected as a constraint subject (ADR 0006; CONSTRAINT-EXTRACTION-V2.1).
fn is_logic_level_token(tok: &str) -> bool {
    let lower = tok.to_ascii_lowercase();
    crate::ir::normative_vocab::LOGIC_HIGH_VALUES.contains(&lower.as_str())
        || crate::ir::normative_vocab::LOGIC_LOW_VALUES.contains(&lower.as_str())
}

fn collect_subject_signal_tokens(text: &str) -> Vec<String> {
    text.split(|c: char| !(c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
        .filter(|tok| {
            tok.len() >= 3
                && tok
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_uppercase())
                    .unwrap_or(false)
                && tok
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                // Width parameters (e.g. DATA_WIDTH, USER_RESP_WIDTH) are integrator
                // constants, not constrained signals — they appear in table width columns.
                && !tok.ends_with("_WIDTH")
                // A logic-level word (HIGH/LOW/…) is a universal value, never a signal —
                // closes the "tied/driven LOW" → "LOW must be stable" leak that the
                // positional "must be <value>" exclusion misses (CONSTRAINT-EXTRACTION-V2.1).
                && !is_logic_level_token(tok)
                && !matches!(
                    *tok,
                    // Constraint VALUE names (HIGH/LOW/IDLE/NONSEQ/…) are no longer
                    // listed here — they are excluded positionally (the value is the
                    // token after the normative verb), so no value vocabulary is
                    // hardcoded (ADR 0006; PDF-AGNOSTIC-EXTRACTION.2).
                    // Only document-independent tokens are listed here — no
                    // protocol/vendor names (those were removed in
                    // PDF-AGNOSTIC-EXTRACTION.3; ADR 0006).
                    // English quantifiers (never signal subjects).
                    "NONE" | "ALL" | "ANY" | "BOTH"
                        // Generic technology abbreviations (document-independent).
                        | "RISC" | "IP" | "SoC"
                        // Document structure terms (document-independent).
                        | "NOTE" | "TABLE" | "FIGURE" | "CHAPTER" | "SECTION" | "REF"
                        // Interface role/component terms (generic across protocols).
                        | "MANAGER" | "SUBORDINATE" | "DECODER" | "INITIATOR"
                        | "MASTER" | "SLAVE" | "TARGET" | "SOURCE"
                )
        })
        .map(|s| s.to_string())
        // Deduplicate while preserving order (same signal can appear twice in a sentence).
        .fold(Vec::new(), |mut acc, s| {
            if !acc.contains(&s) {
                acc.push(s);
            }
            acc
        })
}

/// Level 2 NLP — Extract `ConditionalRuleRecord` entries from `ConditionalRule` sentences.
fn extract_conditional_rules(
    statements: &[ExtractedStatement],
    counter: &mut usize,
) -> Vec<ConditionalRuleRecord> {
    let mut records = Vec::new();

    for statement in statements {
        if !matches!(statement.class, StatementClass::ConditionalRule) {
            continue;
        }
        let text = &statement.text;
        let lowered = text.to_ascii_lowercase();

        // Split on "when", "if", "while", "during", "after", "before".
        let (antecedent, consequent) = split_conditional_sentence(text);
        if antecedent.is_empty() || consequent.is_empty() {
            continue;
        }

        // Try to find the consequent signal (uppercase token in the consequent clause).
        let consequent_signal = consequent
            .split(|c: char| !(c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
            .find(|tok| {
                tok.len() >= 3
                    && tok
                        .chars()
                        .next()
                        .map(|c| c.is_ascii_uppercase())
                        .unwrap_or(false)
                    && tok
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                    && !matches!(
                        // Protocol encoding values (NONSEQ/SEQ/OKAY) removed — ADR 0006;
                        // a value mis-picked as the consequent signal is filtered
                        // downstream by `declared_signal_names`.
                        *tok,
                        "HIGH" | "LOW" | "IDLE" | "BUSY" | "ERROR"
                    )
            })
            .map(|s| s.to_string());

        // Extract the action verb phrase from the consequent.
        let consequent_action = extract_action_phrase(&lowered, &consequent.to_ascii_lowercase());

        *counter += 1;
        records.push(ConditionalRuleRecord {
            rule_id: format!("condrule_{counter:04}"),
            antecedent_text: antecedent.trim().to_string(),
            consequent_signal,
            consequent_action,
            source_text: text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
            automation_confidence: AutomationConfidence::Medium,
        });
    }

    records
}

/// Extract the condition clause from a sentence ("when X", "while X", "during X", "unless X").
fn extract_condition_clause(text: &str) -> Option<String> {
    let lowered = text.to_ascii_lowercase();
    for marker in &[" when ", " while ", " during ", " unless ", " provided "] {
        if let Some(pos) = lowered.find(marker) {
            let clause = &text[pos + marker.len()..].trim_end_matches('.');
            if !clause.is_empty() {
                return Some(clause.to_string());
            }
        }
    }
    None
}

/// Extract a protocol state value from lowered text (IDLE, NONSEQ, SEQ, OKAY, etc.).
/// Extract the value a constraint binds a signal to, **positionally** — the word
/// the document places immediately after the normative "be"/"remain" verb
/// (`"… must be <value>"` → `"<value>"`). Derived from the document at hand, with no
/// hardcoded value vocabulary, so it works for any spec's value names (ADR 0006 —
/// remember the *how*, not the names). Leading articles/binding fillers
/// (`a`/`the`/`set`/`driven`/`to`/…) are skipped. Returns the value uppercased, or
/// `None` if there is no binding phrase or no value word follows it.
fn extract_protocol_state_value(lowered: &str) -> Option<String> {
    const BINDERS: &[&str] = &["must be ", "shall be ", "must remain ", "shall remain "];
    // Words that are grammar/binding scaffolding, never the value itself.
    const FILLERS: &[&str] = &[
        "a", "an", "the", "set", "driven", "to", "equal", "held", "kept", "in", "at", "its",
    ];
    for binder in BINDERS {
        let Some(pos) = lowered.find(binder) else {
            continue;
        };
        let rest = &lowered[pos + binder.len()..];
        let value = rest
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .find(|word| !word.is_empty() && !FILLERS.contains(word));
        if let Some(value) = value {
            return Some(value.to_ascii_uppercase());
        }
    }
    None
}

/// Split a conditional sentence into (antecedent, consequent) based on leading conditional words.
fn split_conditional_sentence(text: &str) -> (String, String) {
    let lowered = text.to_ascii_lowercase();
    // Leading conditional: "When X, Y" / "While X, Y" / "If X, Y" / "Unless X, Y"
    for marker in &[
        "when ",
        "while ",
        "if ",
        "unless ",
        "during ",
        "after ",
        "before ",
        "whenever ",
        "provided that ",
        "as long as ",
    ] {
        if lowered.starts_with(marker) {
            // Find the comma or second clause boundary.
            let rest = &text[marker.len()..];
            // Look for ", the", ", a ", ", SIGNAL", or just " , "
            if let Some(comma_pos) = rest.find(',') {
                let antecedent = rest[..comma_pos].trim().to_string();
                let consequent = rest[comma_pos + 1..].trim().to_string();
                if !antecedent.is_empty() && !consequent.is_empty() {
                    return (antecedent, consequent);
                }
            }
            // No comma: try splitting at " then "
            if let Some(then_pos) = rest.to_ascii_lowercase().find(" then ") {
                return (
                    rest[..then_pos].trim().to_string(),
                    rest[then_pos + 6..].trim().to_string(),
                );
            }
        }
    }
    // Embedded conditional: "X [must/shall] Y when Z"
    let lowered = text.to_ascii_lowercase();
    for marker in &[" when ", " while ", " during ", " unless "] {
        if let Some(pos) = lowered.find(marker) {
            let consequent = text[..pos].trim().to_string();
            let antecedent = text[pos + marker.len()..].trim_end_matches('.').to_string();
            if !antecedent.is_empty() && !consequent.is_empty() {
                return (antecedent, consequent);
            }
        }
    }
    (String::new(), String::new())
}

/// Extract a normalized action phrase from the consequent clause of a conditional sentence.
fn extract_action_phrase(full_lowered: &str, consequent_lowered: &str) -> String {
    // Normative/behavioral grammar only (the "how"), longest-specific first. No
    // value names — a specific value is read positionally and appended below, so
    // no value vocabulary is hardcoded (ADR 0006).
    for phrase in &[
        "must not change",
        "shall not change",
        "must remain",
        "shall remain",
        "must be asserted",
        "shall be asserted",
        "must be deasserted",
        "shall be deasserted",
        "must be stable",
        "shall be stable",
        "must be valid",
        "shall be valid",
        "must be high",
        "shall be high",
        "must be low",
        "shall be low",
        "must not",
        "shall not",
        "must be",
        "shall be",
        "must",
        "shall",
    ] {
        if consequent_lowered.contains(phrase) || full_lowered.contains(phrase) {
            // For a bare value-binding verb, append the value the document names
            // (positionally): "… must be <value>" → "must be <value>", for ANY
            // value, without listing one.
            let appended_value = if matches!(*phrase, "must be" | "shall be") {
                extract_protocol_state_value(consequent_lowered)
                    .or_else(|| extract_protocol_state_value(full_lowered))
            } else {
                None
            };
            if let Some(value) = appended_value {
                return format!("{phrase} {}", value.to_ascii_lowercase());
            }
            return phrase.to_string();
        }
    }
    "(see source_text)".to_string()
}

/// Detect `SignalConstraintKind` from an alias-substituted sentence.
/// The input is a mixed-case string where the alias phrase has been replaced by the
/// signal name in uppercase (e.g. `"the HADDR shall remain stable when hready is low"`).
fn detect_constraint_kind_from_substituted(text: &str) -> SignalConstraintKind {
    let lowered = text.to_ascii_lowercase();
    if contains_any(
        &lowered,
        &[
            "shall not change",
            "must not change",
            "cannot change",
            "will not change",
        ],
    ) {
        SignalConstraintKind::MustNotChange
    } else if contains_any(
        &lowered,
        &[
            "shall remain stable",
            "must remain stable",
            "shall be stable",
            "must be stable",
        ],
    ) {
        SignalConstraintKind::MustBeStable
    } else if contains_any(
        &lowered,
        &[
            "shall be high",
            "must be high",
            "shall remain high",
            "must remain high",
            "is tied high",
            "is held high",
            "is kept high",
        ],
    ) {
        SignalConstraintKind::MustBeHigh
    } else if contains_any(
        &lowered,
        &[
            "shall be low",
            "must be low",
            "shall remain low",
            "must remain low",
            "is tied low",
            "is held low",
        ],
    ) {
        SignalConstraintKind::MustBeLow
    } else if contains_any(
        &lowered,
        &[
            "shall be asserted",
            "must be asserted",
            "shall remain asserted",
            "must remain asserted",
        ],
    ) {
        SignalConstraintKind::MustBeAsserted
    } else if contains_any(
        &lowered,
        &[
            "shall be deasserted",
            "must be deasserted",
            "shall remain deasserted",
        ],
    ) {
        SignalConstraintKind::MustBeDeasserted
    } else {
        // Fall back to stable — the sentence is a constraint but kind is ambiguous.
        SignalConstraintKind::MustBeStable
    }
}

/// Returns `true` if the sentence explicitly constrains a hardware signal to a specific
/// logic value or protocol state.
///
/// Detection uses two complementary patterns:
///
/// **Value-binding phrases** — the sentence explicitly binds a signal to a value:
///   `must/shall be HIGH/LOW/asserted/deasserted/stable/IDLE/SEQ/...`
///   `must/shall remain HIGH/LOW/asserted/deasserted`
///   `must/shall not change`
///   `is HIGH/LOW when`  (a signal state conditional)
///
/// **Hardware signal reference** — there must also be an uppercase token of 3+ chars
/// that plausibly names a hardware signal. This filters out pure prose like
/// "code quality must be high" from matching.
fn is_signal_value_constraint(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();

    // Step 1: Check for a value-binding phrase. These phrases capture logic levels,
    // stability, assertion, and passive forms. Protocol *encoding* values
    // (a signal bound to a document-named value) are deliberately NOT listed here — those sentences fall
    // through to the dynamic extractor, which discovers the value from the
    // document's own enums/tables (ADR 0006: derive names, never hardcode them).
    let has_value_binding = contains_any(
        &lowered,
        &[
            // Logic levels — explicit must/shall
            "must be high",
            "shall be high",
            "must be low",
            "shall be low",
            "must remain high",
            "shall remain high",
            "must remain low",
            "shall remain low",
            "must stay high",
            "shall stay high",
            "must stay low",
            "shall stay low",
            "is high when",
            "is low when",
            // Tied / driven / held — hardware-specific passive forms that imply
            // a permanent or phase-locked logic level without using shall/must.
            // Very common in chip specs: "HWRITE is tied HIGH for the entire burst".
            "is tied high",
            "is tied low",
            "is tied to",
            "is driven high",
            "is driven low",
            "is held high",
            "is held low",
            "is held stable",
            "is kept high",
            "is kept low",
            "is kept stable",
            "is kept asserted",
            "remains high",
            "remains low",
            "remains asserted",
            "remains deasserted",
            "remains stable",
            // Prohibition forms
            "cannot change",
            "cannot be changed",
            "will not change",
            "must not be changed",
            "shall not be changed",
            // Assertion / de-assertion
            "must be asserted",
            "shall be asserted",
            "must be deasserted",
            "shall be deasserted",
            "must remain asserted",
            "shall remain asserted",
            "must remain deasserted",
            "shall remain deasserted",
            "must be driven high",
            "shall be driven high",
            "must be driven low",
            "shall be driven low",
            "must not be deasserted",
            "shall not be deasserted",
            "must not be asserted",
            "shall not be asserted",
            // Stability
            "must be stable",
            "shall be stable",
            "must not change",
            "shall not change",
            "must remain stable",
            "shall remain stable",
            // Protocol encoding/state values (a signal bound to a document-named value) are
            // detected positionally by `binds_uppercase_value` below — no value
            // names are hardcoded here (ADR 0006).
            "must be valid",
            "shall be valid",
            "must be invalid",
            "shall be invalid",
            "must indicate",
            "shall indicate",
            // Valid/ready handshake patterns
            "must be held",
            "shall be held",
            "must hold the",
            "shall hold the",
        ],
    );

    if !has_value_binding {
        return false;
    }

    // Step 2: The sentence must also contain at least one token that looks like a
    // hardware signal name: all-uppercase, 3+ chars, starts with a letter.
    // This prevents "values must be high quality" from matching.
    text.split(|c: char| !(c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
        .any(|token| {
            token.len() >= 3
                && token
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_uppercase())
                    .unwrap_or(false)
                && token
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
        })
}

/// Tokens that pass `is_hardware_signal_token` but are component names, role names,
/// or descriptive words rather than hardware signal names. These appear as first
/// cells in some table formats (e.g. AMBA Table 2-1 / Table 2-5 where signal
/// names are in the last column rather than the first).
pub(crate) fn is_signal_synthesis_non_signal(token: &str) -> bool {
    matches!(
        token,
        "MANAGER"
            | "SUBORDINATE"
            | "INITIATOR"
            | "TARGET"
            | "SOURCE"
            | "DECODER"
            | "MASTER"
            | "SLAVE"
            | "RESPONDER"
            | "CLOCK"
            | "RESET"
            | "NAME"
            | "SIGNAL"
            | "PORT"
            | "PIN"
            // Common English / description / logic words that pass `is_hardware_signal_token`
            // (all-uppercase) but are never signal NAMES. They leak in when a scrambled
            // signal-description table puts description prose in the name column (AXI
            // table_0059/0187: "Signal THE is width AWPROT, ARPROT"). Universal-language words,
            // not chip-spec names (ADR 0006) — same category as the role terms above.
            // WIRE-BASED-100.5k.
            | "THE"
            | "THIS"
            | "THAT"
            | "WHEN"
            | "WHERE"
            | "WHICH"
            | "AND"
            | "OR"
            | "FOR"
            | "IF"
            | "THEN"
            | "WITH"
            | "TRUE"
            | "FALSE"
            | "HIGH"
            | "LOW"
            | "ASSERTED"
            | "DEASSERTED"
            | "SECURE"
            | "PHYSICAL"
            | "PROTECTED"
            | "INDICATES"
            | "STREAM"
            // Direction / generic words seen leaking from the ADI/SWD architecture spec.
            | "IN"
            | "OUT"
            | "LEVEL"
            // Cross-reference word ("… pin, see Figure B4-3") — not a signal name.
            | "SEE"
            // Operation / access verbs that appear as parenthetical acronyms near a signal descriptor
            // (PDF-VARIANT-DIGESTION.3) — modes/operations, never signal names. Universal vocabulary.
            | "READ"
            | "WRITE"
            | "MODE"
    )
}

fn synthesize_signal_declarations(
    table: &crate::ir::source::StructuredTableRecord,
    section_kind: SectionKind,
    section_title: &str,
    statement_counter: &mut usize,
    prior_guidance: Option<&EvidencePriorGuidance>,
    table_signal_declaration_provenance: &mut Vec<TableSignalDeclarationProvenanceRecord>,
) -> Vec<ExtractedStatement> {
    let mut statements = Vec::new();
    if table.body_rows.is_empty() || table.col_count < 2 {
        return statements;
    }

    // ── Column detection: use headers as a clue, fall back to positional convention ─
    // By convention across all bus protocol specs the signal name is in the leftmost
    // column and the description in the rightmost column.  Headers, when present,
    // are used to find width and source/direction columns more precisely.
    let header_texts: Vec<String> = table
        .header_rows
        .first()
        .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
        .unwrap_or_default();

    // Name column: keyword match on headers; fall back to col 0 (leftmost).
    let name_col: usize = header_texts
        .iter()
        .position(|h| {
            h.contains("signal") || h.contains("name") || h.contains("port") || h.contains("pin")
        })
        .unwrap_or(0);

    // Width column: "width" or "size" are the standard header names (case-insensitive);
    // "bits" is accepted as an alias.
    let width_col = header_texts
        .iter()
        .position(|h| h.contains("width") || h.contains("size") || h.contains("bits"));

    // Direction is expressed in one of three ways across specs:
    //
    //   1. Explicit "Direction" column   → literal "input"/"output" values.
    //   2. "Source" / "Driver" column    → names the DRIVING actor.
    //        driving actor = output from that actor's port; input to all others.
    //   3. "Destination" column          → names the RECEIVING actor (inverted semantics).
    //        signal flows TO that actor → output from the driver's port.
    //
    // All three are detected from headers; only the first matching column type is used.
    let explicit_dir_col = header_texts.iter().position(|h| h.contains("direction"));
    let source_col = header_texts
        .iter()
        .position(|h| h.contains("source") || h.contains("driver"));
    let dest_col = header_texts
        .iter()
        .position(|h| h.contains("destination") || h.contains("dest"));

    // ── Content-based name-column detection (rotation-aware) ────────────────────────
    // The PDF backend sometimes rotates a signal table's body so the name column is NOT
    // where the header says — e.g. AHB `table_0009` and APB `table_0016` put the Signal
    // name in the LAST column with Width/Source/Destination shifted left. When a DIFFERENT
    // column carries more distinct hardware-signal tokens than the header-designated name
    // column, trust the content and remap the other header-derived columns by the same
    // rotation offset, so a misaligned table that is the SOLE source of a signal (e.g. AHB
    // HREADY) still gets extracted. Purely positional/structural — no signal name hardcoded
    // (ADR 0006). WIRE-BASED-100.5h (the .3a-deferred extractor fix).
    let col_count = table.body_rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let signal_token_distinct = |col: usize| -> usize {
        let mut toks: Vec<String> = table
            .body_rows
            .iter()
            .filter_map(|row| row.get(col))
            .map(|cell| {
                cell.text
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_ascii_uppercase()
            })
            .filter(|t| is_hardware_signal_token(t) && !is_signal_synthesis_non_signal(t))
            .collect();
        toks.sort();
        toks.dedup();
        toks.len()
    };
    let header_name_distinct = signal_token_distinct(name_col);
    let (best_col, best_distinct) = (0..col_count)
        .map(|col| (col, signal_token_distinct(col)))
        .max_by_key(|&(_, distinct)| distinct)
        .unwrap_or((name_col, header_name_distinct));
    // Override only on a clear content disagreement (a different column is the real name
    // column): aligned tables keep best_col == name_col → offset 0 → no behavior change.
    let (name_col, offset) =
        if best_col != name_col && best_distinct >= 2 && best_distinct > header_name_distinct {
            (best_col, best_col as isize - name_col as isize)
        } else {
            (name_col, 0isize)
        };
    let remap = |col: Option<usize>| -> Option<usize> {
        match col {
            Some(c) if offset != 0 && col_count > 0 => {
                Some(((c as isize + offset).rem_euclid(col_count as isize)) as usize)
            }
            other => other,
        }
    };
    let width_col = remap(width_col);
    let explicit_dir_col = remap(explicit_dir_col);
    let source_col = remap(source_col);
    let dest_col = remap(dest_col);

    let default_dir =
        infer_signal_direction_from_section(section_kind, section_title, prior_guidance);

    for row in &table.body_rows {
        let Some(name_cell) = row.get(name_col) else {
            continue;
        };
        // Strip footnote markers (e.g. "HSELx a" → use "HSELX").
        let raw_name = name_cell.text.trim();
        let token = raw_name
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_ascii_uppercase();
        if !is_hardware_signal_token(&token) || is_signal_synthesis_non_signal(&token) {
            continue;
        }

        // Determine direction — try each column type in priority order.
        let direction = explicit_dir_col
            .and_then(|col| row.get(col))
            .and_then(|cell| {
                let t = cell.text.to_ascii_lowercase();
                if t.contains("output") {
                    Some("output")
                } else if t.contains("input") {
                    Some("input")
                } else {
                    None
                }
            })
            .or_else(|| {
                source_col.and_then(|col| row.get(col)).and_then(|cell| {
                    infer_signal_direction_from_actor_text(
                        &cell.text,
                        RelationTableColumnKind::SourceLike,
                        prior_guidance,
                    )
                })
            })
            .or_else(|| {
                dest_col.and_then(|col| row.get(col)).and_then(|cell| {
                    infer_signal_direction_from_actor_text(
                        &cell.text,
                        RelationTableColumnKind::DestinationLike,
                        prior_guidance,
                    )
                })
            })
            .or_else(|| {
                // No direction/source/dest column (e.g. CHI's two-column
                // `Signal | Description` channel tables): the driver is named in the
                // Description prose. Pick the longest non-name cell (the description)
                // and infer direction from "the transmitter/receiver sets this signal".
                row.iter()
                    .enumerate()
                    .filter(|(col, _)| *col != name_col)
                    .map(|(_, cell)| cell.text.as_str())
                    .max_by_key(|text| text.len())
                    .and_then(infer_signal_direction_from_description_prose)
            })
            .or(default_dir);

        // Extract width: numeric (e.g. 32) or parametric (e.g. ADDR_WIDTH, DATA_WIDTH/8).
        // Both are valid RTL port widths; parametric means the integrator sets the value.
        let width: Option<WidthHint> =
            infer_signal_table_row_width_hint(row, &header_texts, width_col);

        let text = match (direction, &width) {
            (Some(dir), Some(WidthHint::Numeric(bits))) => {
                format!("Signal {token} is {dir} width {bits}.")
            }
            (Some(dir), Some(WidthHint::Parametric(expr))) => {
                format!("Signal {token} is {dir} width {expr}.")
            }
            (Some(dir), None) => format!("Signal {token} is {dir}."),
            // Direction unknown but width is known: emit a width-only declaration.
            // Downstream scoring still benefits from knowing the signal exists and its width.
            (None, Some(WidthHint::Numeric(bits))) => format!("Signal {token} is width {bits}."),
            (None, Some(WidthHint::Parametric(expr))) => {
                format!("Signal {token} is width {expr}.")
            }
            _ => continue, // No direction AND no width — not enough info to synthesize
        };

        *statement_counter += 1;
        let statement_id = format!("statement_{statement_counter:04}");
        table_signal_declaration_provenance.push(TableSignalDeclarationProvenanceRecord {
            statement_id: statement_id.clone(),
            signal_name: token.clone(),
            table_id: table.table_id.clone(),
        });
        statements.push(ExtractedStatement {
            statement_id,
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        });
    }

    statements
}

/// SWD-SERIAL-EXTRACTION.2 — capture interface signals that a serial/architecture spec declares in
/// PROSE rather than a signal-description table. The Arm Debug Interface introduces its wire contract
/// in an appositive: "requires a clock pin, SWCLK", "a single bidirectional data pin, SWDIO". The
/// pattern is "`<role>` pin, `<SIGNAL>`" — the noun "pin" immediately naming the signal across a comma.
/// General/ADR-0006 (grammar, not names). Emits a width-1 declaration (a pin is a single wire) so the
/// signal enters the declared catalog; duplicates of table-declared signals dedupe downstream.
fn synthesize_signal_declarations_from_prose(
    statements: &[ExtractedStatement],
    statement_counter: &mut usize,
    enable_parenthetical: bool,
) -> Vec<ExtractedStatement> {
    let mut out = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for statement in statements {
        let words: Vec<&str> = statement.text.split_whitespace().collect();
        for i in 0..words.len() {
            // The noun "pin", with or without a fused trailing comma.
            if !words[i].trim_end_matches(',').eq_ignore_ascii_case("pin") {
                continue;
            }
            // The signal token follows the comma: fused ("pin," SIG) or separate ("pin" "," SIG).
            let cand = if words[i].ends_with(',') {
                words.get(i + 1)
            } else if words.get(i + 1).map(|w| *w == ",").unwrap_or(false) {
                words.get(i + 2)
            } else {
                continue;
            };
            let Some(cand) = cand else { continue };
            let token: String = cand
                .trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .to_ascii_uppercase();
            if !is_hardware_signal_token(&token)
                || is_signal_synthesis_non_signal(&token)
                || !seen.insert(token.clone())
            {
                continue;
            }
            *statement_counter += 1;
            out.push(ExtractedStatement {
                statement_id: format!("statement_{statement_counter:04}"),
                class: StatementClass::SourceFact,
                modality: EvidenceModality::Text,
                text: format!("Signal {token} is width 1."),
                evidence_span_ids: statement.evidence_span_ids.clone(),
                related_visual_evidence_ids: vec![],
            });
        }
        // PDF-VARIANT-DIGESTION.3 — parenthetical abbreviation form: a signal introduced in prose as
        // "<descriptor> (NAME)", e.g. "a serial data line (SDA)", "serial clock (USCL)". The noun-phrase
        // HEAD (the word immediately before the abbreviation) must be a single-wire noun
        // (line/signal/clock/data/wire/bus/pin) so arbitrary acronyms — and non-wire heads like a clock
        // PULSE (ACK), a Data CHANNEL (DDC), or a data RATE (SDR) — are not captured (EXTRACTION-GAP-FIX.1).
        // Run only as a FALLBACK for sparse-catalog docs (`enable_parenthetical`):
        // table-rich specs (AXI etc.) get their signals from tables, and prose capture there is redundant
        // noise that can admit garbage constraints. General prose capture, universal vocabulary (ADR 0006).
        if !enable_parenthetical {
            continue;
        }
        for (i, w) in words.iter().enumerate() {
            let Some(open) = w.find('(') else { continue };
            let name: String = w[open + 1..]
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            // Must be an UPPERCASE acronym (2–10 chars, ≥1 letter, no lowercase) — a signal abbreviation
            // like SDA/SCL, not a lowercase word grabbed from "(resulting from …)".
            if name.len() < 2
                || name.len() > 10
                || !name.chars().any(|c| c.is_ascii_alphabetic())
                || name.chars().any(|c| c.is_ascii_lowercase())
            {
                continue;
            }
            let token = name.to_ascii_uppercase();
            if !is_hardware_signal_token(&token) || is_signal_synthesis_non_signal(&token) {
                continue;
            }
            // EXTRACTION-GAP-FIX.1 — the noun-phrase HEAD (the word immediately before the "(NAME)", or
            // the fused prefix in "line(NAME)") must itself be a single-wire noun — not merely SOME
            // descriptor anywhere in a preceding window. A window admits non-signals whose head is not a
            // wire: "An acknowledge clock pulse (ACK)" / "Display Data Channel (DDC)" / "standard data rate
            // (SDR)" all contain a descriptor (clock/data) but their HEAD is pulse/channel/rate — a
            // condition, another bus, a rate — not a wire. Requiring the head to be the wire noun keeps the
            // real lines ("serial data line (SDA)", "serial clock (USCL)", "high-speed data (SDAH)") and
            // drops those over-captures. General grammar, universal vocabulary (ADR 0006).
            let head_word: Option<String> = if open > 0 {
                Some(w[..open].to_string())
            } else {
                i.checked_sub(1)
                    .and_then(|h| words.get(h))
                    .map(|s| s.to_string())
            };
            let head_is_wire_noun = head_word
                .as_deref()
                .map(|p| {
                    matches!(
                        p.trim_matches(|c: char| !c.is_ascii_alphabetic())
                            .to_ascii_lowercase()
                            .as_str(),
                        "line"
                            | "lines"
                            | "signal"
                            | "signals"
                            | "clock"
                            | "data"
                            | "wire"
                            | "wires"
                            | "bus"
                            | "pin"
                            | "pins"
                    )
                })
                .unwrap_or(false);
            if !head_is_wire_noun || !seen.insert(token.clone()) {
                continue;
            }
            *statement_counter += 1;
            out.push(ExtractedStatement {
                statement_id: format!("statement_{statement_counter:04}"),
                class: StatementClass::SourceFact,
                modality: EvidenceModality::Text,
                text: format!("Signal {token} is width 1."),
                evidence_span_ids: statement.evidence_span_ids.clone(),
                related_visual_evidence_ids: vec![],
            });
        }
        // PDF-VARIANT-DIGESTION.9.8 — definitional single-wire signal form: a signal a spec DEFINES in
        // PROSE (not a table) via a copula "<NAME> is a/an signal …" or a glossary colon "<NAME>: signal
        // …". SWP (ETSI TS 102 613) names its two single-wire signals only this way — "S1 is a signal in
        // the voltage domain …", "S2 is a signal in the current domain …", "S1: signal from the master to a
        // slave" — so the pin-appositive ("pin," anchor) and parenthetical ("(NAME)") forms above miss them.
        // The definitional anchor is precise AND the candidate must be an all-uppercase identifier token
        // (`is_hardware_signal_token` on the ORIGINAL token, so lowercase English subjects like "an interrupt
        // is a signal" / "it is a signal" can never qualify): a probe over ALL persisted evidence docs yields
        // EXACTLY S1/S2 with zero garbage. General grammar, universal vocabulary (ADR 0006). Runs under the
        // same sparse-catalog fallback gate as the parenthetical form, so table-rich specs are untouched.
        for name in definitional_signal_names(&statement.text) {
            if is_signal_synthesis_non_signal(&name) || !seen.insert(name.clone()) {
                continue;
            }
            *statement_counter += 1;
            out.push(ExtractedStatement {
                statement_id: format!("statement_{statement_counter:04}"),
                class: StatementClass::SourceFact,
                modality: EvidenceModality::Text,
                text: format!("Signal {name} is width 1."),
                evidence_span_ids: statement.evidence_span_ids.clone(),
                related_visual_evidence_ids: vec![],
            });
        }
    }
    out
}

/// PDF-VARIANT-DIGESTION.9.8 — recover the NAME(s) of single-wire signal(s) a spec DEFINES in prose via a
/// definitional copula (`<NAME> is a|an signal …`) or a glossary colon (`<NAME>: signal …`). Both anchors
/// require the candidate to be an all-uppercase identifier token (`is_hardware_signal_token` on the ORIGINAL,
/// un-cased token), so a lowercase English subject ("an interrupt is a signal", "it is a signal", "Note:
/// signal …") can never qualify — the definitional structure plus the identifier shape keep it garbage-free
/// (corpus-probed over all persisted evidence docs: SWP → S1/S2 only, zero garbage). General grammar, no chip
/// names (ADR 0006).
fn definitional_signal_names(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    // Copula: "<NAME> is a|an signal" — the token immediately before "is a/an signal".
    for i in 1..words.len() {
        if !words[i].eq_ignore_ascii_case("is") {
            continue;
        }
        let article = words
            .get(i + 1)
            .map(|w| w.eq_ignore_ascii_case("a") || w.eq_ignore_ascii_case("an"))
            .unwrap_or(false);
        let signal_noun = words
            .get(i + 2)
            .map(|w| {
                w.trim_matches(|c: char| !c.is_ascii_alphanumeric())
                    .eq_ignore_ascii_case("signal")
            })
            .unwrap_or(false);
        if !(article && signal_noun) {
            continue;
        }
        let tok = words[i - 1].trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_');
        if is_hardware_signal_token(tok) {
            out.push(tok.to_string());
        }
    }
    // Glossary colon: "<NAME>: signal …" — the single identifier token before the first colon, immediately
    // followed by the descriptor noun "signal". A multi-word or lowercase head ("master: entity which …")
    // fails `is_hardware_signal_token`, so only a clean glossary signal definition fires.
    if let Some((head, rest)) = text.split_once(':') {
        let head_tok = head.trim();
        let rest_is_signal = rest
            .split_whitespace()
            .next()
            .map(|w| w.eq_ignore_ascii_case("signal"))
            .unwrap_or(false);
        if rest_is_signal && is_hardware_signal_token(head_tok) {
            out.push(head_tok.to_string());
        }
    }
    out
}

/// SWD-SERIAL-EXTRACTION.3 — recover serial-frame fields (`NAME[hi:lo]` ⇒ width |hi-lo|+1) from a
/// serial protocol's frame description (SWD packet request / acknowledge / data phases). Gated to
/// serial documents (markers: "serial wire" / "packet request" / "shift-dr" / SWDIO / SWCLK) so
/// parallel-bus specs — which also use WDATA/RDATA and the phrase "data phase" — produce nothing.
/// Protocol vocabulary, not chip names (ADR 0006).
fn extract_serial_frame_fields(statements: &[ExtractedStatement]) -> Vec<SerialFrameField> {
    let is_serial_doc = statements.iter().any(|s| {
        let l = s.text.to_ascii_lowercase();
        l.contains("serial wire")
            || l.contains("packet request")
            || l.contains("shift-dr")
            || l.contains("swdio")
            || l.contains("swclk")
    });
    if !is_serial_doc {
        return Vec::new();
    }
    let mut out: Vec<SerialFrameField> = Vec::new();
    let mut index_by_name: BTreeMap<String, usize> = BTreeMap::new();
    let mut counter = 0usize;
    for statement in statements {
        let lower = statement.text.to_ascii_lowercase();
        // Data phase is checked BEFORE request: a statement that names a data field (WDATA/RDATA/
        // DATAIN) is about the data phase even when it also mentions "write requests"/RnW — e.g.
        // "For write requests … the value in DATAIN[31:0] is written" is the DATA phase (`.5` fix).
        let phase = if lower.contains("acknowledge") || lower.contains("ack[") {
            Some(SerialFramePhase::Acknowledge)
        } else if lower.contains("data bits")
            || lower.contains("data phase")
            || lower.contains("wdata")
            || lower.contains("rdata")
            || lower.contains("datain")
            || lower.contains("dataout")
        {
            Some(SerialFramePhase::Data)
        } else if lower.contains("packet request")
            || lower.contains("apndp")
            || lower.contains("rnw")
        {
            Some(SerialFramePhase::Request)
        } else {
            None
        };
        // Only mine fields from statements that are actually in a frame phase. A serial doc also
        // cites unrelated bit-fields (register fields, bridged-bus signals like AxCACHE); the phase
        // gate keeps the SWD FRAME fields and drops that noise.
        if phase.is_none() {
            continue;
        }
        // Bit-range fields: ACK[2:0], WDATA[0:31], A[2:3], …
        for (name, hi, lo) in parse_bit_range_fields(&statement.text) {
            let width = (hi as i64 - lo as i64).unsigned_abs() as u32 + 1;
            upsert_serial_field(
                &mut out,
                &mut index_by_name,
                &mut counter,
                &name,
                Some(width),
                Some((hi, lo)),
                phase,
                &statement.statement_id,
            );
        }
        // Named single-bit request fields (`.3b`): "the four bits APnDP, RnW and A[2:3]" — APnDP/RnW
        // are 1-bit fields the prose explicitly labels "bits" (grammar, not names — ADR 0006).
        for name in parse_named_bit_list(&statement.text) {
            upsert_serial_field(
                &mut out,
                &mut index_by_name,
                &mut counter,
                &name,
                Some(1),
                None,
                phase,
                &statement.statement_id,
            );
        }
    }
    // Single-bit CONTROL fields (`.4b`): Start/Stop/Parity/Park complete the packet request frame.
    // Their definitions ("A single start bit …", "the Park bit …") carry no phase keyword, so they are
    // mined OUTSIDE the phase gate and assigned the request phase (host-driven control bits).
    for statement in statements {
        for name in parse_control_bit_fields(&statement.text) {
            upsert_serial_field(
                &mut out,
                &mut index_by_name,
                &mut counter,
                &name,
                Some(1),
                None,
                Some(SerialFramePhase::Request),
                &statement.statement_id,
            );
        }
    }
    // ACK response values (`.3b`): the ACK field carries OK / WAIT / FAULT, recovered from the
    // "<value> response to a DPACC/APACC access" grammar. Empty when there is no ACK field or no
    // such statements (serde-skipped).
    if let Some(&idx) = index_by_name.get("ACK") {
        out[idx].response_values = extract_ack_response_values(statements);
    }
    // Field ordering (`.3b`): the frame sequence is request bits → acknowledge → data. Order by phase
    // rank, then first appearance (stable: `out` is already in appearance order).
    let phase_rank = |p: &Option<SerialFramePhase>| match p {
        Some(SerialFramePhase::Request) => 0,
        Some(SerialFramePhase::Acknowledge) => 1,
        Some(SerialFramePhase::Data) => 2,
        None => 3,
    };
    let mut ordered: Vec<usize> = (0..out.len()).collect();
    ordered.sort_by_key(|&i| (phase_rank(&out[i].phase), i));
    for (rank, &i) in ordered.iter().enumerate() {
        out[i].order = Some(rank as u32);
    }
    // Per-phase SWDIO direction (`.4c`): who DRIVES the wire per field/phase, derived from the spec's
    // "from the <A> to the <B>" / "<A> to <B>, following a read/write request" prose. The data phase
    // is direction-by-field (WDATA host-driven, RDATA target-driven), so resolve field-level first;
    // request/acknowledge are phase-level.
    let mut field_dir: BTreeMap<String, SwdioDirection> = BTreeMap::new();
    let mut request_dir: Option<SwdioDirection> = None;
    let mut acknowledge_dir: Option<SwdioDirection> = None;
    for statement in statements {
        let Some(actor) = swdio_source_actor(&statement.text) else {
            continue;
        };
        let Some(dir) = swdio_direction_from_actor(&actor) else {
            continue;
        };
        let lower = statement.text.to_ascii_lowercase();
        // Field-level (data phase): a directional statement naming WDATA / RDATA (token-boundary).
        for field in out.iter() {
            if field.name.len() >= 3 && text_has_word(&statement.text, &field.name) {
                field_dir.entry(field.name.clone()).or_insert(dir);
            }
        }
        // Phase-level: the "packet request" / "acknowledge" descriptions.
        if lower.contains("packet request") {
            request_dir.get_or_insert(dir);
        }
        if lower.contains("acknowledge") {
            acknowledge_dir.get_or_insert(dir);
        }
    }
    for field in out.iter_mut() {
        field.swdio_direction = field_dir.get(&field.name).copied().or(match field.phase {
            Some(SerialFramePhase::Request) => request_dir,
            Some(SerialFramePhase::Acknowledge) => acknowledge_dir,
            _ => None,
        });
    }
    out
}

/// PDF-VARIANT-DIGESTION.9.3b — recover a protocol's FRAME STRUCTURE from a prose COMPOSITION LIST ("A DATA
/// FRAME is composed of seven different bit fields: START OF FRAME, ARBITRATION FIELD, …") plus per-field
/// widths stated directly in prose ("CONTROL FIELD consists of six bits", "ACK FIELD is two bits long"). The
/// composition list SCOPES which fields are captured, so scattered "N bits" mentions of non-frame items (ERROR
/// FLAG / OVERLOAD DELIMITER / INTERMISSION) are excluded. A width is recorded ONLY when the field name is the
/// direct subject of a PLURAL "<num> bits" count, NEVER when "<num> bit" modifies a sub-field ("the 11 bit
/// IDENTIFIER") — so a width is never fabricated/mis-attributed (the honesty guardrail: a residual `None`
/// beats a wrong value). Field names are multi-word ALL-CAPS noun phrases; the frame SEQUENCE is preserved as
/// `order`; `phase` stays `None` (this frame model is a generic field sequence, not SWD's request/ack/data).
/// General grammar, no chip names (ADR 0006). Self-gating on the composition shape — corpus-probed to fire
/// only on the CAN-style frame description, zero false positives on the wire-based or other serial specs.
fn extract_composition_frame_fields(statements: &[ExtractedStatement]) -> Vec<SerialFrameField> {
    let mut fields: Vec<SerialFrameField> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for (i, statement) in statements.iter().enumerate() {
        let lower = statement.text.to_ascii_lowercase();
        if !(lower.contains("composed of") && lower.contains("bit field")) {
            continue;
        }
        // The field list sits after the colon — in this statement, or in the next one when the composition
        // sentence ends at the colon.
        let Some(colon) = statement.text.find(':') else {
            continue;
        };
        let inline = statement.text[colon + 1..].trim();
        let (list_text, list_stmt_id): (&str, String) = if inline.is_empty() {
            match statements.get(i + 1) {
                Some(next) => (next.text.as_str(), next.statement_id.clone()),
                None => continue,
            }
        } else {
            (inline, statement.statement_id.clone())
        };
        // The list ends at the first sentence boundary after the colon.
        let head = list_text
            .split_once(". ")
            .map(|(h, _)| h)
            .unwrap_or(list_text);
        for raw in head.split(',') {
            let name = raw.trim().trim_end_matches('.').trim();
            if !is_frame_field_name(name) || !seen.insert(name.to_string()) {
                continue;
            }
            let idx = fields.len();
            let (bit_width, width_stmt_id) = stated_frame_field_bit_width(name, statements);
            let mut supporting = vec![list_stmt_id.clone()];
            if let Some(id) = width_stmt_id {
                supporting.push(id);
            }
            fields.push(SerialFrameField {
                field_id: format!("frame_field_{idx:04}"),
                name: name.to_string(),
                bit_width,
                bit_range: None,
                phase: None,
                swdio_direction: None,
                order: Some(idx as u32),
                response_values: Vec::new(),
                supporting_statement_ids: supporting,
            });
        }
    }
    fields
}

/// EXTRACTOR-ARCHITECTURE.9a — the SWD-style bit-range/named-bit frame strategy as a registered
/// `Extractor`. Its grammar self-gates on serial-doc markers, so no separate `applies_to` is needed.
struct SerialFrameBitRangeExtractor;
impl Extractor<SerialFrameField> for SerialFrameBitRangeExtractor {
    fn name(&self) -> &'static str {
        "serial_frame.bit_range"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<SerialFrameField> {
        extract_serial_frame_fields(cx.statements)
    }
}

/// EXTRACTOR-ARCHITECTURE.9a — the prose composition-list frame strategy (the CAN shape) as a registered
/// `Extractor`. Disjoint from the bit-range strategy by construction (CAN lacks the SWD markers; SWD lacks
/// the composition sentence), so the surface's key-merge is a defensive guarantee, not a load-bearing fix.
struct SerialFrameCompositionExtractor;
impl Extractor<SerialFrameField> for SerialFrameCompositionExtractor {
    fn name(&self) -> &'static str {
        "serial_frame.composition"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<SerialFrameField> {
        extract_composition_frame_fields(cx.statements)
    }
}

/// Run the serial-frame surface through the unified key-merge driver (EXTRACTOR-ARCHITECTURE.9a). Key =
/// field name: both strategies emit name-unique lists (the bit-range form upserts by name; the composition
/// form keeps a per-list `seen` set), so first-wins dedup is a no-op WITHIN each strategy and reproduces the
/// prior cross-strategy policy — a composition field is dropped only when the bit-range form already
/// produced that name — byte-for-byte.
fn serial_frame_field_surface(
    statements: &[ExtractedStatement],
    manifest: &mut ExtractionManifest,
) -> Vec<SerialFrameField> {
    let cx = ExtractionContext { statements };
    let extractors: [&dyn Extractor<SerialFrameField>; 2] = [
        &SerialFrameBitRangeExtractor,
        &SerialFrameCompositionExtractor,
    ];
    let run = run_surface("serial_frame_fields", &cx, &extractors, |field| {
        field.name.clone()
    });
    manifest.record(&run);
    run.records
}

/// A multi-word ALL-CAPS frame-field name like "START OF FRAME" / "ARBITRATION FIELD" — uppercase letters
/// plus a few joiners only, with at least two uppercase letters (so a comma fragment, a lowercase word, or a
/// stray "and" is rejected). (PDF-VARIANT-DIGESTION.9.3b)
fn is_frame_field_name(s: &str) -> bool {
    let s = s.trim();
    if s.len() < 2 || !s.starts_with(|c: char| c.is_ascii_uppercase()) {
        return false;
    }
    let mut uppercase = 0usize;
    for c in s.chars() {
        if c.is_ascii_uppercase() {
            uppercase += 1;
        } else if !matches!(c, ' ' | '/' | '\'' | '-') {
            return false;
        }
    }
    uppercase >= 2
}

/// Recover a frame field's bit width when prose states it DIRECTLY as a plural "<num> bits" count whose
/// subject is the field name ("CONTROL FIELD consists of six bits", "ACK FIELD is two bits long"). Rejects
/// "<num> bit <noun>" ("consists of the 11 bit IDENTIFIER", where the count modifies a sub-field) and singular
/// "a single … bit" forms, so a width is never mis-attributed (honesty guardrail). Returns the width and the
/// supporting statement id. (PDF-VARIANT-DIGESTION.9.3b)
fn stated_frame_field_bit_width(
    name: &str,
    statements: &[ExtractedStatement],
) -> (Option<u32>, Option<String>) {
    for statement in statements {
        let Some(pos) = statement.text.find(name) else {
            continue;
        };
        // Restrict to the clause that names the field, up to the first sentence end.
        let clause = statement.text[pos + name.len()..]
            .split_once(". ")
            .map(|(h, _)| h)
            .unwrap_or(&statement.text[pos + name.len()..]);
        let words: Vec<&str> = clause.split_whitespace().collect();
        for w in 0..words.len() {
            if !(words[w].eq_ignore_ascii_case("of") || words[w].eq_ignore_ascii_case("is")) {
                continue;
            }
            // Skip an optional determiner after the count verb.
            let mut j = w + 1;
            if words
                .get(j)
                .map(|t| matches!(t.to_ascii_lowercase().as_str(), "the" | "a" | "an"))
                .unwrap_or(false)
            {
                j += 1;
            }
            let (Some(num_tok), Some(unit_tok)) = (words.get(j), words.get(j + 1)) else {
                continue;
            };
            // The unit MUST be the plural "bits": a singular "bit" or a following noun means the count
            // modifies a sub-field, not the frame field — reject to avoid a wrong width.
            if unit_tok.trim_matches(|c: char| !c.is_ascii_alphabetic()) != "bits" {
                continue;
            }
            if let Some(n) = parse_count_word(num_tok) {
                return (Some(n), Some(statement.statement_id.clone()));
            }
        }
    }
    (None, None)
}

/// Parse an English count word or a digit string into a number ("single"/"one" → 1, "six" → 6, "11" → 11).
/// Used for prose-stated frame-field widths (PDF-VARIANT-DIGESTION.9.3b).
fn parse_count_word(tok: &str) -> Option<u32> {
    let t = tok
        .trim_matches(|c: char| !c.is_ascii_alphanumeric())
        .to_ascii_lowercase();
    match t.as_str() {
        "single" | "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        "eleven" => Some(11),
        "twelve" => Some(12),
        other => other.parse::<u32>().ok(),
    }
}

/// Upsert a serial-frame field by name: widen the bit-width / fill phase / record the supporting
/// statement on an existing field, or create a new one. (`SWD-SERIAL-EXTRACTION.3`/`.3b`)
#[allow(clippy::too_many_arguments)]
fn upsert_serial_field(
    out: &mut Vec<SerialFrameField>,
    index_by_name: &mut BTreeMap<String, usize>,
    counter: &mut usize,
    name: &str,
    width: Option<u32>,
    bit_range: Option<(u32, u32)>,
    phase: Option<SerialFramePhase>,
    stmt_id: &str,
) {
    if let Some(&idx) = index_by_name.get(name) {
        let field = &mut out[idx];
        if width.is_some_and(|w| w > field.bit_width.unwrap_or(0)) {
            field.bit_width = width;
            field.bit_range = bit_range;
        }
        if field.phase.is_none() {
            field.phase = phase;
        }
        if !field.supporting_statement_ids.iter().any(|s| s == stmt_id) {
            field.supporting_statement_ids.push(stmt_id.to_string());
        }
        return;
    }
    *counter += 1;
    index_by_name.insert(name.to_string(), out.len());
    out.push(SerialFrameField {
        field_id: format!("serial_field_{counter:04}"),
        name: name.to_string(),
        bit_width: width,
        bit_range,
        phase,
        swdio_direction: None,
        order: None,
        response_values: Vec::new(),
        supporting_statement_ids: vec![stmt_id.to_string()],
    });
}

/// SWD-SERIAL-EXTRACTION.4c — the actor that SOURCES (drives) the wire in a directional statement:
/// "from the `<A>` to the `<B>`" → `<A>`; "`<A>` to `<B>`, following a read/write request (FIELD)" →
/// `<A>`. Returns the lowercase source actor token (host/debugger/target/dp), else `None`.
fn swdio_source_actor(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let clean = |t: &str| {
        t.trim_matches(|c: char| !c.is_ascii_alphanumeric())
            .to_ascii_lowercase()
    };
    if let Some(p) = lower.find("from the ") {
        let rest = &lower[p + "from the ".len()..];
        if rest.contains(" to the ") || rest.contains(" to ") {
            return rest.split_whitespace().next().map(clean);
        }
    }
    // "Target to host, following a read request (RDATA)." — the leading token is the source.
    if lower.contains("following a") && lower.contains("request") && lower.contains(" to ") {
        return text
            .trim_start_matches(['-', ' '])
            .split_whitespace()
            .next()
            .map(clean);
    }
    None
}

/// Map a source-actor token to who drives SWDIO (the host/debugger, or the target/DP). `.4c`.
fn swdio_direction_from_actor(actor: &str) -> Option<SwdioDirection> {
    match actor {
        "host" | "debugger" => Some(SwdioDirection::HostDrives),
        "target" | "dp" => Some(SwdioDirection::TargetDrives),
        _ => None,
    }
}

/// True if `word` occurs in `text` not surrounded by alphanumerics (a token-boundary match), so
/// "WDATA" matches "WDATA[0:31]" but not a substring of a larger identifier. `.4c`.
fn text_has_word(text: &str, word: &str) -> bool {
    let bytes = text.as_bytes();
    let wbytes = word.as_bytes();
    if wbytes.is_empty() {
        return false;
    }
    let mut i = 0;
    while let Some(rel) = text[i..].find(word) {
        let start = i + rel;
        let end = start + word.len();
        let before_ok = start == 0 || !bytes[start - 1].is_ascii_alphanumeric();
        let after_ok = end >= bytes.len() || !bytes[end].is_ascii_alphanumeric();
        if before_ok && after_ok {
            return true;
        }
        i = start + 1;
    }
    false
}

/// SWD-SERIAL-EXTRACTION.4b — parse single-bit CONTROL fields defined as "A single `<name>` bit …"
/// (Start/Stop/Parity) or "the `<Name>` bit …" (Park) → the field name. These complete the SWD packet
/// request frame and are not bit-ranges or "the N bits …" lists. Grammar (a glossary definition), not
/// names (ADR 0006); the name is taken from the definition itself.
fn parse_control_bit_fields(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    for i in 1..words.len() {
        if !words[i]
            .trim_matches([',', '.'])
            .eq_ignore_ascii_case("bit")
        {
            continue;
        }
        let name = words[i - 1].trim_matches(|c: char| !c.is_ascii_alphanumeric());
        if name.len() < 3 || name.len() > 7 || !name.chars().all(|c| c.is_ascii_alphabetic()) {
            continue;
        }
        // Two specific, high-precision glossary/protocol-error phrasings only (the broad "<Word> bit"
        // form over-matches register names in this register-heavy doc):
        //   1. "A single <name> bit …"            (Start / Stop / Parity definitions)
        //   2. "the <Name> bit is not 0b…"        (Stop / Park protocol-error conditions, B4.2.5)
        let definition = i >= 2
            && words[i - 2].eq_ignore_ascii_case("single")
            && !name.eq_ignore_ascii_case("single");
        let error_condition = i >= 2
            && words[i - 2].eq_ignore_ascii_case("the")
            && words
                .get(i + 1)
                .is_some_and(|w| w.eq_ignore_ascii_case("is"))
            && words
                .get(i + 2)
                .is_some_and(|w| w.eq_ignore_ascii_case("not"));
        if !(definition || error_condition) {
            continue;
        }
        let mut cap = name.to_ascii_lowercase();
        if let Some(first) = cap.get_mut(0..1) {
            first.make_ascii_uppercase();
        }
        if is_signal_synthesis_non_signal(&cap.to_ascii_uppercase()) {
            continue;
        }
        if !out.contains(&cap) {
            out.push(cap);
        }
    }
    out
}

/// Parse named single-bit fields from "the N bits X, Y and Z" prose → [X, Y, …]. Each list item the
/// prose calls a "bit" is a field (grammar, not names — ADR 0006). Items written as a bit-range
/// (`A[2:3]`) are skipped here (the range parser captures them). Plausible field tokens only: start
/// with a letter, ≤16 alphanumerics.
fn parse_named_bit_list(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let lower = text.to_ascii_lowercase();
    let mut search_from = 0usize;
    while let Some(rel) = lower[search_from..].find("bits ") {
        let start = search_from + rel + "bits ".len();
        // The list runs to the clause end (':' or '.').
        let end = text[start..]
            .find([':', '.'])
            .map(|e| start + e)
            .unwrap_or(text.len());
        let clause = &text[start..end];
        for piece in clause.split([',']).flat_map(|p| p.split(" and ")) {
            let token = piece.trim();
            if token.contains('[') || token.is_empty() {
                continue;
            }
            let token = token.split_whitespace().next().unwrap_or("");
            let cleaned: String = token
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            if cleaned.len() >= 2
                && cleaned
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_uppercase())
                    .unwrap_or(false)
                && !is_signal_synthesis_non_signal(&cleaned.to_ascii_uppercase())
            {
                out.push(cleaned);
            }
        }
        search_from = end;
    }
    out
}

/// Recover the response values of a serial acknowledge field. SWD states them as "<value> response to
/// a DPACC or APACC access" (e.g. "WAIT response to a DPACC …", "OK or FAULT response to a …"), so the
/// extraction is gated to DP/AP-access-response statements and reads the 1–2 value tokens immediately
/// before "response" (handling the "X or Y" / "X/Y" list), excluding the access-type tokens themselves.
/// Grammar, not a hardcoded value list (ADR 0006).
fn extract_ack_response_values(statements: &[ExtractedStatement]) -> Vec<String> {
    let mut values: Vec<String> = Vec::new();
    let is_value_token = |t: &str| -> bool {
        (2..=7).contains(&t.len())
            && t.chars().all(|c| c.is_ascii_uppercase())
            && !matches!(t, "DPACC" | "APACC" | "DP" | "AP")
    };
    let mut push = |t: &str| {
        if is_value_token(t) && !values.iter().any(|v| v == t) {
            values.push(t.to_string());
        }
    };
    for statement in statements {
        let lower = statement.text.to_ascii_lowercase();
        // Only DP/AP access-response statements describe the ACK responses.
        if !(lower.contains("dpacc") || lower.contains("apacc")) {
            continue;
        }
        let words: Vec<&str> = statement.text.split_whitespace().collect();
        for (i, w) in words.iter().enumerate() {
            if !w.eq_ignore_ascii_case("response") || i == 0 {
                continue;
            }
            // The value(s) directly before "response": "WAIT response", "OK or FAULT response",
            // "OK/FAULT response". Walk back over value tokens joined by "or" / "/".
            let mut j = i;
            while j >= 1 {
                let raw = words[j - 1];
                if raw.eq_ignore_ascii_case("or") {
                    j -= 1;
                    continue;
                }
                for part in raw.split('/') {
                    let tok = part.trim_matches(|c: char| !c.is_ascii_alphanumeric());
                    push(tok);
                }
                // stop unless the token before is an "or" joiner
                if j >= 2 && words[j - 2].eq_ignore_ascii_case("or") {
                    j -= 1;
                } else {
                    break;
                }
            }
        }
    }
    values.sort();
    values
}

/// SWD-SERIAL-EXTRACTION.4 — extract the protocol FSM states (the JTAG TAP / SWD line state machine).
/// Gated to documents that describe a state machine ("state machine" / DBGTAPSM / "TAP controller"), so
/// non-FSM specs produce nothing. States are recognized by the "`<StateName>` state" grammar where the
/// name is a hyphen/slash-joined capitalized token (Shift-DR, Run-Test/Idle, Test-Logic-Reset) — grammar,
/// not names (ADR 0006). The per-state action is the clause that follows ("In the Shift-DR state, <action>").
fn extract_protocol_states(statements: &[ExtractedStatement]) -> Vec<ProtocolStateRecord> {
    let has_state_machine = statements.iter().any(|s| {
        let l = s.text.to_ascii_lowercase();
        l.contains("state machine") || l.contains("dbgtapsm") || l.contains("tap controller")
    });
    if !has_state_machine {
        return Vec::new();
    }
    let machine_name = statements
        .iter()
        .any(|s| s.text.contains("DBGTAPSM"))
        .then(|| "DBGTAPSM".to_string());
    let mut out: Vec<ProtocolStateRecord> = Vec::new();
    let mut index_by_name: BTreeMap<String, usize> = BTreeMap::new();
    let mut counter = 0usize;
    for statement in statements {
        let lower = statement.text.to_ascii_lowercase();
        // State-machine context: a statement that talks about the TAP / state machine / scan chain.
        let in_context = lower.contains("dbgtapsm")
            || lower.contains("tap")
            || lower.contains("state machine")
            || lower.contains("scan chain")
            || lower.contains(" tck")
            || lower.contains("instruction register")
            || lower.contains("data register");
        if !in_context {
            continue;
        }
        for (name, action) in find_states_with_actions(&statement.text) {
            if let Some(&idx) = index_by_name.get(&name) {
                let state = &mut out[idx];
                if state.action.is_none() {
                    state.action = action;
                }
                if !state
                    .supporting_statement_ids
                    .contains(&statement.statement_id)
                {
                    state
                        .supporting_statement_ids
                        .push(statement.statement_id.clone());
                }
                continue;
            }
            counter += 1;
            index_by_name.insert(name.clone(), out.len());
            out.push(ProtocolStateRecord {
                state_id: format!("protocol_state_{counter:04}"),
                machine_name: machine_name.clone(),
                state_name: name,
                action,
                supporting_statement_ids: vec![statement.statement_id.clone()],
            });
        }
    }
    // Dedup separator variants (`.5` fix): a docling-rendered "Test-Logic/Reset" is the same state as
    // the canonical hyphenated "Test-Logic-Reset". Group by separator-normalized key ('/' → '-') and
    // keep the variant with the most supporting statements (canonical), merging the rest's supports.
    let mut by_norm: BTreeMap<String, usize> = BTreeMap::new();
    let mut merged: Vec<ProtocolStateRecord> = Vec::new();
    for rec in out {
        let norm = rec.state_name.replace('/', "-").to_ascii_lowercase();
        if let Some(&idx) = by_norm.get(&norm) {
            let keep = &mut merged[idx];
            for sid in &rec.supporting_statement_ids {
                if !keep.supporting_statement_ids.contains(sid) {
                    keep.supporting_statement_ids.push(sid.clone());
                }
            }
            // Prefer the variant with more support as the canonical name.
            if rec.supporting_statement_ids.len() > keep.supporting_statement_ids.len() {
                keep.state_name = rec.state_name.clone();
            }
        } else {
            by_norm.insert(norm, merged.len());
            merged.push(rec);
        }
    }
    merged
}

/// PDF-VARIANT-DIGESTION.9.3a — a NEW, agnostic FSM path for protocols that define their states as
/// single-quoted operational MODES of a generic actor ("a unit may be in one of three states: 'error
/// active' / 'error passive' / 'bus off'"; "A node is 'error passive' when …"). The SWD/JTAG path
/// (`extract_protocol_states` / `find_states_with_actions`) only recognizes `Capitalized-Hyphen state`
/// names behind a TAP/scan-chain doc-gate, so it yields nothing on this shape.
///
/// Grammar (ADR 0006 — universal, no chip/protocol literals): a state is a single-quoted name of 1–3
/// alphabetic words BOUND to a generic actor-noun (node/unit/station/device) in either the adjective
/// form (`'<name>' <actor>`) or the predicate form (`<actor> <link-verb> '<name>'`). That actor-noun
/// binding is exactly what separates a STATE (a *node* is 'error passive') from a quoted bit value (a
/// *bit* is 'dominant') or a bus condition (the *bus* is 'idle'). Emitted only when ≥2 distinct names
/// each recur in ≥2 statements (an FSM has several states; a lone quoted phrase is not one). A trailing
/// `when <condition>` clause becomes the state's `action`. Nothing is fabricated — every state is a
/// node-mode the prose literally quotes.
fn extract_quoted_mode_states(statements: &[ExtractedStatement]) -> Vec<ProtocolStateRecord> {
    let mut support: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut action_by_name: BTreeMap<String, String> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    for statement in statements {
        for (name, action) in quoted_mode_states_in(&statement.text) {
            let ids = support.entry(name.clone()).or_default();
            if !ids.iter().any(|s| s == &statement.statement_id) {
                ids.push(statement.statement_id.clone());
            }
            if !order.iter().any(|n| n == &name) {
                order.push(name.clone());
            }
            if let Some(a) = action {
                action_by_name.entry(name).or_insert(a);
            }
        }
    }
    // A real FSM state is referenced repeatedly; require ≥2 supporting statements per state.
    let kept: Vec<String> = order
        .into_iter()
        .filter(|n| support.get(n).map(|v| v.len()).unwrap_or(0) >= 2)
        .collect();
    // An FSM has at least two states — a single recurring quoted phrase is not a state machine.
    if kept.len() < 2 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(kept.len());
    for (i, name) in kept.iter().enumerate() {
        out.push(ProtocolStateRecord {
            state_id: format!("mode_state_{:04}", i + 1),
            machine_name: None,
            state_name: name.clone(),
            action: action_by_name.get(name).cloned(),
            supporting_statement_ids: support.remove(name).unwrap_or_default(),
        });
    }
    out
}

/// PDF-VARIANT-DIGESTION.9.3a — find single-quoted operational-MODE names bound to a generic actor in one
/// statement → `(normalized_name, optional "when …" transition clause)`. Grammar only (ADR 0006); the
/// actor-noun binding keeps node-modes and drops quoted bit values / bus conditions.
fn quoted_mode_states_in(text: &str) -> Vec<(String, Option<String>)> {
    const ACTOR_NOUNS: &[&str] = &[
        "node", "nodes", "unit", "units", "station", "stations", "device", "devices",
    ];
    const LINK_VERBS: &[&str] = &[
        "is",
        "are",
        "be",
        "becomes",
        "become",
        "called",
        "named",
        "considered",
    ];
    // Fold typographic single quotes to ASCII so one scan handles both renderings.
    let norm: String = text
        .chars()
        .map(|c| {
            if c == '\u{2018}' || c == '\u{2019}' {
                '\''
            } else {
                c
            }
        })
        .collect();
    let lower_word = |w: &str| {
        w.trim_matches(|c: char| !c.is_ascii_alphabetic())
            .to_ascii_lowercase()
    };
    let mut out: Vec<(String, Option<String>)> = Vec::new();
    let mut search = 0usize;
    while let Some(open_rel) = norm[search..].find('\'') {
        let open = search + open_rel;
        let Some(close_rel) = norm[open + 1..].find('\'') else {
            break;
        };
        let close = open + 1 + close_rel;
        let name: String = norm[open + 1..close]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !is_quoted_mode_state_name(&name) {
            // Spurious opening quote (e.g. a contraction apostrophe) — re-scan from just past it so a
            // real pair after it is still found.
            search = open + 1;
            continue;
        }
        let after = norm[close + 1..].trim_start();
        let adjective = after
            .split_whitespace()
            .next()
            .map(lower_word)
            .is_some_and(|w| ACTOR_NOUNS.contains(&w.as_str()));
        // Predicate: the word right before the quote is a link verb whose SUBJECT (the word before it)
        // is an actor-noun — "a node is 'error passive'" (subject `node`), not "the bus to be 'bus
        // idle'" (subject `to`).
        let before_words: Vec<String> = norm[..open]
            .split_whitespace()
            .rev()
            .take(2)
            .map(lower_word)
            .collect();
        let predicate = before_words
            .first()
            .is_some_and(|v| LINK_VERBS.contains(&v.as_str()))
            && before_words
                .get(1)
                .is_some_and(|s| ACTOR_NOUNS.contains(&s.as_str()));
        if adjective || predicate {
            let action = after.find("when ").map(|p| {
                let clause = &after[p..];
                let end = clause.find(". ").unwrap_or(clause.len());
                clause[..end]
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            });
            out.push((name, action));
        }
        search = close + 1;
    }
    out
}

/// A quoted operational-mode state name is 1–3 alphabetic words (hyphens allowed) — e.g. `error active`,
/// `bus off`. Rejects sentence fragments, numbers, and empty spans. (`PDF-VARIANT-DIGESTION.9.3a`)
fn is_quoted_mode_state_name(name: &str) -> bool {
    let words: Vec<&str> = name.split_whitespace().collect();
    if words.is_empty() || words.len() > 3 {
        return false;
    }
    words.iter().all(|w| {
        !w.is_empty()
            && w.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
            && w.chars().all(|c| c.is_ascii_alphabetic() || c == '-')
    })
}

/// PDF-VARIANT-DIGESTION.9.7 — a THIRD agnostic FSM path for protocols that name their states as a single
/// ALL-CAPS word followed by "state" (SWP's "the DEACTIVATED state", "into ACTIVATED state", "in the
/// SUSPENDED state"). The SWD/JTAG path (`find_states_with_actions` / `looks_like_state_name`) requires a
/// hyphen/slash-joined name behind a TAP/scan-chain doc-gate, and `.9.3a`'s quoted-mode path requires single
/// quotes plus an actor-noun — so neither captures this shape.
///
/// Grammar (ADR 0006 — universal, no chip/protocol literals): a state is an ALL-CAPS token (≥2 chars, ≥1
/// letter, hyphens allowed) appearing in `<TRIGGER> [the|a|an] <NAME> state`, where TRIGGER is a
/// transition/locative word — a state one ENTERS, EXITS, or is IN. That binding is exactly what separates a
/// STATE ("the interface moves into the SETUP state") from a machine name ("the JTAG TAP state machine":
/// the word before the candidate is not a transition trigger, and an after-guard drops `<X> state machine`).
///
/// Two self-gates make the single-word match safe WITHOUT a keyword doc-gate (rejected because SWP never
/// says "state machine"/"FSM"): each name must recur in ≥2 statements, and a doc must yield ≥2 distinct such
/// states (one mode is not an FSM) — identical in spirit to `extract_quoted_mode_states`. Emitted only then.
/// Nothing is fabricated — every state is an all-caps mode the prose literally enters/exits/holds.
fn extract_transition_bound_states(statements: &[ExtractedStatement]) -> Vec<ProtocolStateRecord> {
    let mut support: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    for statement in statements {
        for name in transition_bound_state_names_in(&statement.text) {
            let ids = support.entry(name.clone()).or_default();
            if !ids.iter().any(|s| s == &statement.statement_id) {
                ids.push(statement.statement_id.clone());
            }
            if !order.iter().any(|n| n == &name) {
                order.push(name.clone());
            }
        }
    }
    // A real FSM state is referenced repeatedly; require ≥2 supporting statements per state.
    let kept: Vec<String> = order
        .into_iter()
        .filter(|n| support.get(n).map(|v| v.len()).unwrap_or(0) >= 2)
        .collect();
    // An FSM has at least two states — a single recurring named state is not a state machine.
    if kept.len() < 2 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(kept.len());
    for (i, name) in kept.iter().enumerate() {
        out.push(ProtocolStateRecord {
            state_id: format!("named_state_{:04}", i + 1),
            machine_name: None,
            state_name: name.clone(),
            action: None,
            supporting_statement_ids: support.remove(name).unwrap_or_default(),
        });
    }
    out
}

/// PDF-VARIANT-DIGESTION.9.7 — find ALL-CAPS single-word state names bound to a transition/locative trigger
/// in one statement. Grammar only (ADR 0006); the trigger binding plus the "state machine|diagram"
/// after-guard keep machine names and incidental "state" mentions out.
fn transition_bound_state_names_in(text: &str) -> Vec<String> {
    const TRIGGERS: &[&str] = &[
        "enter",
        "enters",
        "entered",
        "entering",
        "into",
        "leave",
        "leaves",
        "left",
        "leaving",
        "exit",
        "exits",
        "exited",
        "exiting",
        "to",
        "in",
        "from",
        "reach",
        "reaches",
        "reached",
        "reaching",
        "remain",
        "remains",
        "remained",
        "remaining",
        "stay",
        "stays",
        "stayed",
        "staying",
        "move",
        "moves",
        "moved",
        "moving",
        "transition",
        "transitions",
        "transitioned",
        "transitioning",
        "return",
        "returns",
        "returned",
        "returning",
        "put",
        "puts",
        "place",
        "places",
        "placed",
    ];
    const ARTICLES: &[&str] = &["the", "a", "an", "its", "this", "that"];
    // "<X> state machine|diagram" names the MACHINE, not a state.
    const AFTER_GUARD: &[&str] = &["machine", "machines", "diagram", "diagrams"];
    fn strip(w: &str) -> &str {
        w.trim_matches(|c: char| !c.is_ascii_alphanumeric())
    }
    fn lower(w: &str) -> String {
        strip(w).to_ascii_lowercase()
    }
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut out = Vec::new();
    for i in 2..words.len() {
        if !strip(words[i]).eq_ignore_ascii_case("state") {
            continue;
        }
        if words
            .get(i + 1)
            .is_some_and(|w| AFTER_GUARD.contains(&lower(w).as_str()))
        {
            continue;
        }
        let cand = strip(words[i - 1]);
        if !is_bare_state_name(cand) {
            continue;
        }
        // The trigger sits at i-2, or at i-3 when one article ("the"/"a"/…) separates it from the name.
        let mut t = i - 2;
        if ARTICLES.contains(&lower(words[t]).as_str()) {
            if t == 0 {
                continue;
            }
            t -= 1;
        }
        if TRIGGERS.contains(&lower(words[t]).as_str()) {
            out.push(cand.to_string());
        }
    }
    out
}

/// A bare (unquoted, single-word) FSM state name is an ALL-CAPS token of ≥2 chars (hyphens allowed, ≥1
/// letter, first char a letter) — e.g. `ACTIVATED`, `DEACTIVATED`, `CL0`. Rejects lowercase/mixed-case words
/// (those are `the <X> state` descriptions, not named states), logic levels / booleans, and the universal
/// architectural pseudo-values UNKNOWN / UNPREDICTABLE. (`PDF-VARIANT-DIGESTION.9.7`)
fn is_bare_state_name(tok: &str) -> bool {
    const DENY: &[&str] = &[
        "high",
        "low",
        "on",
        "off",
        "set",
        "clear",
        "true",
        "false",
        "none",
        "all",
        "any",
        "each",
        "both",
        "same",
        "current",
        "next",
        "previous",
        "new",
        "unknown",
        "unpredictable",
    ];
    if tok.len() < 2 {
        return false;
    }
    // ALL-CAPS: equal to its own uppercase (digits/hyphens map to themselves), with ≥1 ASCII letter.
    if tok.to_ascii_uppercase() != tok || !tok.chars().any(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    if !tok.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    if !tok.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return false;
    }
    !DENY.contains(&tok.to_ascii_lowercase().as_str())
}

// EXTRACTOR-ARCHITECTURE.3 — the FSM-state cluster as a unified surface registry. The four state-grammar
// readers are now `Extractor<ProtocolStateRecord>` units run by the shared `run_surface` driver instead of
// four inline dedup-by-name loops at the `build()` call site. Each unit just calls its existing (unchanged)
// grammar function — a refactor of the WIRING, not the grammars. Registry ORDER is the legacy precedence
// (jtag → swd_line → quoted → transition) and the surface key is the uppercased state name (the legacy
// `eq_ignore_ascii_case` cross-dedup), so the merged inventory is byte-identical to the previous call site.

/// JTAG/SWD-hyphen states (`Shift-DR state`) behind the TAP/scan-chain doc-gate — see `extract_protocol_states`.
struct JtagTapStateExtractor;
impl Extractor<ProtocolStateRecord> for JtagTapStateExtractor {
    fn name(&self) -> &'static str {
        "fsm.jtag_tap"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<ProtocolStateRecord> {
        extract_protocol_states(cx.statements)
    }
}

/// SWD LINE states (reset / operating / protocol-error / lockout / dormant) — see `extract_swd_line_states`.
struct SwdLineStateExtractor;
impl Extractor<ProtocolStateRecord> for SwdLineStateExtractor {
    fn name(&self) -> &'static str {
        "fsm.swd_line"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<ProtocolStateRecord> {
        extract_swd_line_states(cx.statements)
    }
}

/// Quoted operational modes of a generic actor (`a node is 'error passive'`) — see `extract_quoted_mode_states`.
struct QuotedModeStateExtractor;
impl Extractor<ProtocolStateRecord> for QuotedModeStateExtractor {
    fn name(&self) -> &'static str {
        "fsm.quoted_mode"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<ProtocolStateRecord> {
        extract_quoted_mode_states(cx.statements)
    }
}

/// Single ALL-CAPS word + "state" in a transition/locative binding (`the DEACTIVATED state`) — see
/// `extract_transition_bound_states`.
struct TransitionBoundStateExtractor;
impl Extractor<ProtocolStateRecord> for TransitionBoundStateExtractor {
    fn name(&self) -> &'static str {
        "fsm.transition_bound"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<ProtocolStateRecord> {
        extract_transition_bound_states(cx.statements)
    }
}

/// Run the full FSM-state surface registry through the unified driver. Behavior-identical to the legacy
/// four-inline-loop merge at the `build()` call site: order jtag → swd_line → quoted → transition,
/// first-wins dedup by uppercased state name. (`EXTRACTOR-ARCHITECTURE.3`)
fn protocol_state_surface(
    statements: &[ExtractedStatement],
    manifest: &mut ExtractionManifest,
) -> Vec<ProtocolStateRecord> {
    let cx = ExtractionContext { statements };
    let extractors: [&dyn Extractor<ProtocolStateRecord>; 4] = [
        &JtagTapStateExtractor,
        &SwdLineStateExtractor,
        &QuotedModeStateExtractor,
        &TransitionBoundStateExtractor,
    ];
    let run = run_surface("protocol_states", &cx, &extractors, |state| {
        state.state_name.to_ascii_uppercase()
    });
    manifest.record(&run);
    run.records
}

/// SWD-SERIAL-EXTRACTION.4d — extract the SWD LINE state machine (reset / operating / protocol-error /
/// lockout / dormant). Unlike the JTAG TAP states, these are lowercase 1–2-word names introduced by a
/// transition verb: "(enter|enters|into|leave|leaves) [the] `<name>` state". The verb gate keeps real
/// state transitions and rejects generic "the current/same state" mentions. Gated to serial documents.
/// Grammar, not names (ADR 0006).
fn extract_swd_line_states(statements: &[ExtractedStatement]) -> Vec<ProtocolStateRecord> {
    let is_serial_doc = statements.iter().any(|s| {
        let l = s.text.to_ascii_lowercase();
        l.contains("serial wire") || l.contains("packet request") || l.contains("swdio")
    });
    if !is_serial_doc {
        return Vec::new();
    }
    let generic = |w: &str| {
        matches!(
            w,
            // logic levels / verbs are not state names ("to the HIGH state", "to maintain the state")
            "high"
                | "low"
                | "maintain"
                | "this"
                | "that"
                | "current"
                | "same"
                | "known"
                | "correct"
                | "next"
                | "previous"
                | "given"
                | "right"
                | "wrong"
                | "following"
                | "above"
                | "below"
                | "new"
                | "other"
        )
    };
    let mut out: Vec<ProtocolStateRecord> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut counter = 0usize;
    for statement in statements {
        // Per-statement SWD-interface context: the ADI doc also describes the PROCESSOR "Debug state"
        // (execution mode) — gate to SWD/SW-DP line context so that is not mistaken for a line state.
        let lower = statement.text.to_ascii_lowercase();
        if !(lower.contains("swd")
            || lower.contains("sw-dp")
            || lower.contains("line")
            || lower.contains("target")
            || lower.contains("interface")
            || lower.contains("protocol"))
        {
            continue;
        }
        let words: Vec<String> = statement
            .text
            .split_whitespace()
            .map(|w| {
                w.trim_matches(|c: char| !c.is_ascii_alphanumeric())
                    .to_string()
            })
            .collect();
        for i in 0..words.len() {
            if !matches!(
                words[i].to_ascii_lowercase().as_str(),
                "enter" | "enters" | "into" | "leave" | "leaves" | "to"
            ) {
                continue;
            }
            let is_to = words[i].eq_ignore_ascii_case("to");
            let mut k = i + 1;
            let had_article = words
                .get(k)
                .is_some_and(|w| w.eq_ignore_ascii_case("the") || w.eq_ignore_ascii_case("a"));
            if had_article {
                k += 1;
            }
            // The broad "to" trigger requires "to the/a <name> state" (so "to maintain the state" and
            // bare "to <verb>" are not mistaken for a state transition); the directional verbs do not.
            if is_to && !had_article {
                continue;
            }
            let mut name: Vec<String> = Vec::new();
            while k < words.len() && name.len() < 2 {
                let t = words[k].to_ascii_lowercase();
                if t == "state" {
                    break;
                }
                if t.len() >= 3 && t.chars().all(|c| c.is_ascii_alphabetic()) && !generic(&t) {
                    name.push(t);
                    k += 1;
                } else {
                    break;
                }
            }
            if name.is_empty()
                || !words
                    .get(k)
                    .is_some_and(|w| w.eq_ignore_ascii_case("state"))
            {
                continue;
            }
            // Strip a leading QUALIFIER from a 2-word name so adjectives/sequence-words collapse to the
            // canonical state head: "line reset" → "reset" (dedups the Reset state), "required operating"
            // → "operating". A genuine 2-word state name ("protocol error") has a non-qualifier head.
            if name.len() == 2
                && matches!(
                    name[0].as_str(),
                    "line"
                        | "required"
                        | "relevant"
                        | "powerup"
                        | "sel"
                        | "normal"
                        | "valid"
                        | "default"
                        | "initial"
                        | "single"
                        | "certain"
                        | "particular"
                        | "specific"
                        | "appropriate"
                )
            {
                name.remove(0);
            }
            let mut state_name = name.join(" ");
            if let Some(first) = state_name.get_mut(0..1) {
                first.make_ascii_uppercase();
            }
            if !seen.insert(state_name.clone()) {
                continue;
            }
            counter += 1;
            out.push(ProtocolStateRecord {
                state_id: format!("swd_line_state_{counter:04}"),
                machine_name: Some("SWD line state machine".to_string()),
                state_name,
                action: None,
                supporting_statement_ids: vec![statement.statement_id.clone()],
            });
        }
    }
    out
}

/// Find FSM states in text via "`<StateName>` state" → (state_name, optional action clause). The state
/// name is the hyphen/slash-joined capitalized token immediately before the word "state"; the action is
/// the clause that follows a comma after it, up to the sentence end.
fn find_states_with_actions(text: &str) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    for i in 1..words.len() {
        if words[i].trim_end_matches([',', '.', ':', ';']) != "state" {
            continue;
        }
        let cand = words[i - 1]
            .trim_matches(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '/'));
        if !looks_like_state_name(cand) {
            continue;
        }
        let marker = format!("{cand} state");
        let action = text.find(&marker).and_then(|p| {
            let after = text[p + marker.len()..].trim_start();
            let after = after.strip_prefix(',').unwrap_or(after).trim_start();
            let end = after.find(". ").unwrap_or(after.len());
            let clause = after[..end].trim().trim_end_matches('.').trim();
            (clause.len() >= 4).then(|| clause.to_string())
        });
        out.push((cand.to_string(), action));
    }
    out
}

/// A protocol state name is a hyphen/slash-joined sequence of capitalized parts (e.g. `Shift-DR`,
/// `Run-Test/Idle`, `Test-Logic-Reset`). Single plain words ("reset", "this") are not state names.
fn looks_like_state_name(tok: &str) -> bool {
    if !(tok.contains('-') || tok.contains('/')) {
        return false;
    }
    let parts: Vec<&str> = tok.split(['-', '/']).collect();
    parts.len() >= 2
        && parts.iter().all(|p| {
            !p.is_empty()
                && p.chars()
                    .next()
                    .map(|c| c.is_ascii_uppercase())
                    .unwrap_or(false)
                && p.chars().all(|c| c.is_ascii_alphanumeric())
        })
}

/// Parse `NAME[hi:lo]` bit-range fields from text → (name, hi, lo) for each. NAME is the identifier
/// immediately before `[` (first char a letter); hi/lo are decimal indices (spaces tolerated).
fn parse_bit_range_fields(text: &str) -> Vec<(String, u32, u32)> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    for (i, &b) in bytes.iter().enumerate() {
        if b != b'[' {
            continue;
        }
        let name_end = i;
        let mut name_start = i;
        while name_start > 0 {
            let c = bytes[name_start - 1] as char;
            if c.is_ascii_alphanumeric() || c == '_' {
                name_start -= 1;
            } else {
                break;
            }
        }
        let name = &text[name_start..name_end];
        if name.is_empty()
            || !name
                .chars()
                .next()
                .map(|c| c.is_ascii_alphabetic())
                .unwrap_or(false)
        {
            continue;
        }
        let Some(close) = text[i + 1..].find(']') else {
            continue;
        };
        let inner = &text[i + 1..i + 1 + close];
        let parts: Vec<&str> = inner.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        if let (Ok(hi), Ok(lo)) = (
            parts[0].trim().parse::<u32>(),
            parts[1].trim().parse::<u32>(),
        ) {
            out.push((name.to_string(), hi, lo));
        }
    }
    out
}

/// SWD-SERIAL-EXTRACTION.4b — recover the SWD packet-protocol operations (response branching): each
/// EXTRACTOR-ARCHITECTURE.9a — the SWD packet-operations surface as a registered `Extractor` (currently a
/// single strategy; the concat driver is identity for one producer, as with the actors surface in `.7`).
/// Registering it gives serial/debug documents a uniform manifest entry — a behavioral fingerprint token
/// the CORPUS-PATTERN-REUSE profile plane consumes — and readies the surface for multi-strategy growth.
struct SwdOperationExtractor;
impl Extractor<SwdOperation> for SwdOperationExtractor {
    fn name(&self) -> &'static str {
        "operations.prose"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<SwdOperation> {
        extract_swd_operations(cx.statements)
    }
}

/// Run the SWD-operations surface through the concat driver for a uniform manifest entry
/// (EXTRACTOR-ARCHITECTURE.9a). One producer → concat is identity → output unchanged.
fn swd_operation_surface(
    statements: &[ExtractedStatement],
    manifest: &mut ExtractionManifest,
) -> Vec<SwdOperation> {
    let run = run_surface_concat(
        "swd_operations",
        &ExtractionContext { statements },
        &[&SwdOperationExtractor],
    );
    manifest.record(&run);
    run.records
}

/// "a successful `<read|write>` operation consists of three phases" / "A `<WAIT|FAULT>` response …
/// consists of two phases" header becomes a `SwdOperation`. OK → 3-phase (has data); WAIT/FAULT →
/// 2-phase. The turnaround-before-data flag comes from the write ("turnaround between the acknowledge
/// phase and the WDATA") vs read ("no turnaround … between the acknowledge phase and the data") prose.
/// Gated to serial documents. Grammar, not names (ADR 0006).
fn extract_swd_operations(statements: &[ExtractedStatement]) -> Vec<SwdOperation> {
    let is_serial_doc = statements.iter().any(|s| {
        let l = s.text.to_ascii_lowercase();
        l.contains("serial wire") || l.contains("packet request") || l.contains("swdio")
    });
    if !is_serial_doc {
        return Vec::new();
    }
    let mut write_trn: Option<bool> = None;
    let mut read_trn: Option<bool> = None;
    for s in statements {
        let l = s.text.to_ascii_lowercase();
        if l.contains("turnaround period between the acknowledge phase and the") {
            write_trn = Some(true);
        }
        if l.contains("no turnaround period between the acknowledge phase and the data") {
            read_trn = Some(false);
        }
    }
    let mut out = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut counter = 0usize;
    for s in statements {
        let l = s.text.to_ascii_lowercase();
        let phase_count: u32 = if l.contains("consists of three phases") {
            3
        } else if l.contains("consists of two phases") {
            2
        } else {
            continue;
        };
        let response = if l.contains("successful") || l.contains("ok response") {
            "OK"
        } else if l.contains("wait response") {
            "WAIT"
        } else if l.contains("fault response") {
            "FAULT"
        } else {
            continue;
        };
        let has_read = l.contains("read");
        let has_write = l.contains("write");
        let access = match (has_read, has_write) {
            (true, false) => Some("read"),
            (false, true) => Some("write"),
            _ => None,
        };
        let has_data_phase = phase_count == 3;
        let turnaround_before_data = if has_data_phase {
            match access {
                Some("write") => write_trn,
                Some("read") => read_trn,
                _ => None,
            }
        } else {
            None
        };
        let key = format!("{response}-{access:?}-{phase_count}");
        if !seen.insert(key) {
            continue;
        }
        counter += 1;
        out.push(SwdOperation {
            operation_id: format!("swd_operation_{counter:04}"),
            response: response.to_string(),
            access: access.map(|a| a.to_string()),
            phase_count,
            has_data_phase,
            turnaround_before_data,
            supporting_statement_ids: vec![s.statement_id.clone()],
        });
    }
    out
}

fn synthesize_encoding_declarations(
    table: &crate::ir::source::StructuredTableRecord,
    section_title: &str,
    statement_counter: &mut usize,
) -> Vec<ExtractedStatement> {
    if table.body_rows.is_empty() {
        return Vec::new();
    }

    let Some(enum_name) = derive_encoding_enum_name(table, section_title, None) else {
        return Vec::new();
    };
    synthesize_encoding_declarations_for_enum(table, &enum_name, statement_counter)
}

fn synthesize_encoding_declarations_for_enum(
    table: &crate::ir::source::StructuredTableRecord,
    enum_name: &str,
    statement_counter: &mut usize,
) -> Vec<ExtractedStatement> {
    let mut statements = Vec::new();
    let (name_col, value_col) = infer_encoding_column_indices(table, enum_name);

    for (row_idx, row) in table.body_rows.iter().enumerate() {
        let Some(name_cell) = row.get(name_col) else {
            continue;
        };
        let raw_name = name_cell.text.trim();
        if raw_name.is_empty() {
            continue;
        }
        // Sanitize enum member name: keep alphanumeric + underscore, uppercase.
        let member_name: String = raw_name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '_' {
                    c.to_ascii_uppercase()
                } else {
                    '_'
                }
            })
            .collect::<String>()
            .trim_matches('_')
            .to_string();
        if member_name.is_empty() {
            continue;
        }
        // Numeric value: use value_col if available and parseable, otherwise use row index.
        let value: u32 = row
            .get(value_col)
            .and_then(|cell| parse_encoding_numeric_literal(&cell.text))
            .unwrap_or(row_idx as u32);

        // Synthesize: "Enum HTRANS IDLE = 0."
        let text = format!("Enum {enum_name} {member_name} = {value}.");

        *statement_counter += 1;
        statements.push(ExtractedStatement {
            statement_id: format!("statement_{statement_counter:04}"),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        });
    }

    statements
}

// EXTRACTOR-ARCHITECTURE.6 — the register-record surface as a unified registry. Its two strategies
// (register-map tables, and `unknown` register-FIELD tables) are `Extractor<RegisterRecord>` units run by the
// `run_surface_concat` driver: registers are a CONCAT surface — disjoint records from different table kinds
// merged by plain concatenation, NOT a key-dedup. The three register-specific post-passes (width fill-in,
// bit-layout-grid drop, fragment de-fragmentation) then run on the merged set in `register_record_surface`.
// Each strategy carries its inputs (source + prior guidance) on its struct and does not read `statements`.

/// Register records from `register_map`-classified tables — see `synthesize_register_records`.
struct RegisterMapExtractor<'a> {
    source_ir: &'a SourceIr,
    prior_guidance: Option<&'a EvidencePriorGuidance>,
}
impl Extractor<RegisterRecord> for RegisterMapExtractor<'_> {
    fn name(&self) -> &'static str {
        "registers.register_map"
    }
    fn run(&self, _cx: &ExtractionContext<'_>) -> Vec<RegisterRecord> {
        synthesize_register_records(self.source_ir, self.prior_guidance)
    }
}

/// Register records recovered from `unknown` register-FIELD tables — see `synthesize_register_field_tables`.
struct RegisterFieldTableExtractor<'a> {
    source_ir: &'a SourceIr,
    prior_guidance: Option<&'a EvidencePriorGuidance>,
}
impl Extractor<RegisterRecord> for RegisterFieldTableExtractor<'_> {
    fn name(&self) -> &'static str {
        "registers.field_table"
    }
    fn run(&self, _cx: &ExtractionContext<'_>) -> Vec<RegisterRecord> {
        synthesize_register_field_tables(self.source_ir, self.prior_guidance)
    }
}

/// Run the register-record surface: concat the two strategies via the unified `run_surface_concat` driver,
/// then apply the register-specific post-passes (width fill-in from field extents, bit-layout-grid drop,
/// fragment merge). Behavior-identical to the previous inline `build()` block. (`EXTRACTOR-ARCHITECTURE.6`)
fn register_record_surface(
    source_ir: &SourceIr,
    prior_guidance: Option<&EvidencePriorGuidance>,
    manifest: &mut ExtractionManifest,
) -> Vec<RegisterRecord> {
    let cx = ExtractionContext { statements: &[] };
    let map = RegisterMapExtractor {
        source_ir,
        prior_guidance,
    };
    let field_table = RegisterFieldTableExtractor {
        source_ir,
        prior_guidance,
    };
    let extractors: [&dyn Extractor<RegisterRecord>; 2] = [&map, &field_table];
    let run = run_surface_concat("register_records", &cx, &extractors);
    manifest.record(&run);
    let mut records = run.records;
    // Flexible-register-model (PDF-VARIANT-DIGESTION.2c): fill register width from field bit extents where the
    // register-map path created the record incrementally without a size.
    for reg in &mut records {
        if reg.size_bits.is_none() {
            reg.size_bits = register_size_from_fields(&reg.fields);
        }
    }
    // Drop bit-LAYOUT grids mis-read as registers (see `register_is_bit_layout_grid`).
    records.retain(|reg| !register_is_bit_layout_grid(reg));
    // EXTRACTION-GAP-FIX.4c — de-fragment a register whose field table a PDF backend split across several
    // tables. Conservative: only merges same-name fragments whose field names are ALL distinct (honesty
    // guardrail — never fabricate a field set).
    consolidate_register_field_fragments(&mut records);
    records
}

/// Synthesize `RegisterRecord` entries from `register_map` tables captured in `SourceIR`.
/// Each table row becomes either a register-level record or, if the table has bit-field
/// columns, a field within the preceding register.
fn synthesize_register_records(
    source_ir: &SourceIr,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<RegisterRecord> {
    let mut records: Vec<RegisterRecord> = Vec::new();
    let register_tables: Vec<_> = source_ir
        .structured_tables
        .iter()
        .filter(|t| {
            matches!(
                effective_table_kind(t, prior_guidance),
                TableKind::RegisterMap
            )
        })
        .collect();

    for table in register_tables {
        if table.body_rows.is_empty() {
            continue;
        }

        // Identify column indices from header row.
        let header: Vec<String> = table
            .header_rows
            .first()
            .map(|r| r.iter().map(|c| c.text.to_ascii_lowercase()).collect())
            .unwrap_or_default();

        let explicit_name_col = header
            .iter()
            .position(|h| h.contains("name") || h.contains("register") || h.contains("field"));
        let name_col = explicit_name_col.unwrap_or(0);
        let offset_col = header
            .iter()
            .position(|h| h.contains("offset") || h.contains("address") || h.contains("addr"));
        let access_col = header
            .iter()
            .position(|h| h.contains("access") || h.contains("r/w"));
        let reset_col = header
            .iter()
            .position(|h| h.contains("reset") || h.contains("default"));
        let desc_col = header.iter().position(|h| h.contains("description"));
        let bits_col = header
            .iter()
            .position(|h| h.contains("bits") || h.contains("bit") || h.contains("field"));

        let table_id = table.table_id.clone();
        for (row_idx, row) in table.body_rows.iter().enumerate() {
            let name = row
                .get(name_col)
                .map(|c| c.text.trim().to_string())
                .unwrap_or_default();
            if name.is_empty() {
                continue;
            }

            let offset = offset_col
                .and_then(|col| row.get(col))
                .map(|c| c.text.trim().to_string())
                .filter(|s| !s.is_empty());
            let access = access_col
                .and_then(|col| row.get(col))
                .map(|c| c.text.trim().to_string())
                .filter(|s| !s.is_empty());
            let reset = reset_col
                .and_then(|col| row.get(col))
                .map(|c| c.text.trim().to_string())
                .filter(|s| !s.is_empty());
            let desc = desc_col
                .and_then(|col| row.get(col))
                .map(|c| c.text.trim().to_string())
                .filter(|s| !s.is_empty());

            // If there's a bits column, treat this as a register with one field.
            if bits_col.is_some() {
                let bits_text = bits_col
                    .and_then(|col| row.get(col))
                    .map(|c| c.text.trim().to_string())
                    .unwrap_or_default();
                // Parse "7:0" or "[7:0]" into bits_high, bits_low.
                let (bits_high, bits_low) = parse_bit_range(&bits_text);
                let enumerated_values = desc
                    .as_deref()
                    .map(parse_inline_field_enums)
                    .unwrap_or_default();
                // EXTRACTION-GAP-FIX.2 — with no dedicated name column the row's "name" is the bit-range;
                // recover the field mnemonic from the description (`Full Name (MNEMONIC):`), else keep the
                // bit-range as an honest residual (never fabricated).
                let field_name = if explicit_name_col.is_none() && is_bit_range_token(&name) {
                    desc.as_deref()
                        .and_then(field_mnemonic_from_description)
                        .unwrap_or_else(|| name.clone())
                } else {
                    name.clone()
                };
                let field = RegisterFieldRecord {
                    field_name,
                    bits_high,
                    bits_low,
                    bit_width: bit_width_from_range(bits_high, bits_low),
                    access_type: access,
                    reset_value: reset,
                    description: desc,
                    enumerated_values,
                };
                // Try to attach to the last register, or create a new one.
                if let Some(last) = records.last_mut() {
                    last.fields.push(field);
                    continue;
                }
            }

            records.push(RegisterRecord {
                register_id: format!("reg_{}_{row_idx:03}", document_key(&table_id)),
                register_name: name,
                offset_address: offset,
                size_bits: None,
                fields: Vec::new(),
                supporting_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    records
}

/// PDF-VARIANT-DIGESTION.2 — does this header row name a REGISTER-FIELD table? Such tables define the
/// bit-fields of one register, one row per field: `Field|Description|Access|Reset`, `Bits|Name|Access|…`,
/// `Bit|Field|Type|Reset`. Identity = a field/bit-name column AND an access OR reset column — universal
/// register vocabulary (ADR 0006), specific enough to exclude signal/encoding/feature tables (which lack
/// access+reset). Header cells are pre-lowercased.
pub(crate) fn is_register_field_header(header: &[String]) -> bool {
    let has_field = header.iter().any(|h| {
        h.contains("field") || h == "name" || h == "bits" || h == "bit" || h.contains("bit name")
    });
    let has_access = header.iter().any(|h| {
        h.contains("access") || h == "r/w" || h == "rw" || h == "type" || h.contains("attribut")
    });
    let has_reset = header
        .iter()
        .any(|h| h.contains("reset") || h.contains("default"));
    has_field && (has_access || has_reset)
}

/// Strip "Table 6.1:" / "Figure" prefixes and trailing "register"/"fields" boilerplate from a caption to
/// recover a register name; `None` when nothing usable remains.
fn register_name_from_caption(caption: &str) -> Option<String> {
    let mut s = caption.trim();
    // drop a leading "Table N[.-]N :" / "Figure N :" label
    if let Some(idx) = s.find(':') {
        let head = s[..idx].to_ascii_lowercase();
        if head.starts_with("table") || head.starts_with("figure") {
            s = s[idx + 1..].trim();
        }
    }
    let lowered = s.to_ascii_lowercase();
    let trimmed = lowered
        .trim_end_matches(|c: char| !c.is_alphanumeric())
        .trim_end_matches("fields")
        .trim_end_matches("field")
        .trim_end_matches("register")
        .trim();
    // map back to the original-case slice of the same length-ish: just title-keep the original token run.
    let kept = s
        .split_whitespace()
        .filter(|w| {
            let wl = w.to_ascii_lowercase();
            wl != "register" && wl != "fields" && wl != "field" && wl != "the"
        })
        .collect::<Vec<_>>()
        .join(" ");
    let result = if kept.trim().is_empty() {
        trimmed.to_string()
    } else {
        kept.trim().to_string()
    };
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

/// True when `s` contains a hex ADDRESS token `0x<hex>` (case-insensitive `x`) — the universal "this names
/// a register" signal: a register definition states the register's address/offset. Used to gate register-name
/// recovery from a heading so an arbitrary prose parenthetical cannot mint a register name. General digital
/// convention, not a chip name (ADR 0006). EXTRACTION-GAP-FIX.3.
fn contains_hex_address(s: &str) -> bool {
    let b = s.as_bytes();
    let mut i = 0;
    while i + 2 < b.len() {
        if b[i] == b'0' && (b[i + 1] == b'x' || b[i + 1] == b'X') && b[i + 2].is_ascii_hexdigit() {
            return true;
        }
        i += 1;
    }
    false
}

/// A plausible REGISTER NAME token: starts with a letter, then letters/digits/underscore, length 2–40.
/// General; carries no specific names (ADR 0006). EXTRACTION-GAP-FIX.3.
fn is_register_name_token(s: &str) -> bool {
    let len = s.len();
    (2..=40).contains(&len)
        && s.starts_with(|c: char| c.is_ascii_alphabetic())
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Recover a REGISTER NAME from a section heading that DEFINES a register. The universal register-map
/// convention is a heading like `Debug Module Status (dmstatus, at 0x11)` / `Hart Info (hartinfo, at 0x12)`:
/// a parenthetical whose leading token is the register identifier and which also carries the register's hex
/// ADDRESS. We scan parenthetical groups and accept the first whose content carries a `0x<hex>` address
/// (`contains_hex_address`) and whose leading token is a register identifier (`is_register_name_token`) —
/// the required address is what separates a register-definition heading from an arbitrary prose
/// parenthetical, so a register name is NEVER fabricated from a non-defining heading (honesty guardrail).
/// Returns `None` otherwise (the caller keeps the synthetic `register_<table_id>` name). Pure grammar
/// (ADR 0006); EXTRACTION-GAP-FIX.3.
fn register_name_from_heading(title: &str) -> Option<String> {
    let bytes = title.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        // `(` is ASCII so it only appears at a char boundary; slicing at `i + 1` is safe.
        if bytes[i] == b'(' {
            let rest = &title[i + 1..];
            let Some(close_rel) = rest.find(')') else {
                break;
            };
            let inner = &rest[..close_rel];
            if contains_hex_address(inner) {
                let name: String = inner
                    .trim_start()
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                if is_register_name_token(&name) {
                    return Some(name);
                }
            }
            i += 1 + close_rel + 1;
            continue;
        }
        i += 1;
    }
    None
}

/// PDF-VARIANT-DIGESTION.2 (Lever A, deterministic strategy) — recover REGISTER-FIELD tables that the
/// ingest classifier left `unknown`, most often because Docling did not mark the column-title row as a
/// header so it lands in `body_rows[0]`. These field-definition tables are ubiquitous in TRMs / architecture
/// / register specs (RISC-V, CoreSight, OpenCAPI). ADDITIVE: emits extra `RegisterRecord`s (one per table,
/// rows → fields); never touches signal/constraint/relation extraction, so the wire-based specs are
/// unaffected. Skips tables already handled by [`synthesize_register_records`]. Header GRAMMAR only.
fn synthesize_register_field_tables(
    source_ir: &SourceIr,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<RegisterRecord> {
    let mut records: Vec<RegisterRecord> = Vec::new();
    for table in &source_ir.structured_tables {
        if matches!(
            effective_table_kind(table, prior_guidance),
            TableKind::RegisterMap
        ) {
            continue; // the row-per-register path owns these
        }
        // Resolve the header row: a Docling-marked header, else body_rows[0] (the common miss).
        let header_in_body = table.header_rows.is_empty();
        let header: Vec<String> = if header_in_body {
            table.body_rows.first()
        } else {
            table.header_rows.first()
        }
        .map(|r| {
            r.iter()
                .map(|c| c.text.trim().to_ascii_lowercase())
                .collect()
        })
        .unwrap_or_default();
        if !is_register_field_header(&header) {
            continue;
        }
        let body: Vec<&Vec<StructuredTableCellRecord>> = if header_in_body {
            table.body_rows.iter().skip(1).collect()
        } else {
            table.body_rows.iter().collect()
        };
        if body.is_empty() {
            continue;
        }

        let bits_col = header.iter().position(|h| {
            h == "bits" || h == "bit" || h.contains("bit range") || h.contains("position")
        });
        // Name the field by its name column; for bits-only field tables (`Bits|Type|Reset|Description`)
        // there is none, so fall back to the bit-range column, then column 0.
        let explicit_field_col = header.iter().position(|h| {
            h.contains("field") || h == "name" || h.contains("identifier") || h.contains("bit name")
        });
        let field_col = explicit_field_col.or(bits_col).unwrap_or(0);
        // EXTRACTION-GAP-FIX.2 — with no dedicated name/field column the "name" is the bit-range string;
        // the field MNEMONIC then lives in the description's defined-term prefix (`Full Name (MNEMONIC):`).
        let name_is_bit_range_column = explicit_field_col.is_none() && bits_col.is_some();
        let access_col = header.iter().position(|h| {
            h.contains("access") || h == "r/w" || h == "rw" || h == "type" || h.contains("attribut")
        });
        let reset_col = header
            .iter()
            .position(|h| h.contains("reset") || h.contains("default"));
        let desc_col = header.iter().position(|h| {
            h.contains("description")
                || h.contains("function")
                || h.contains("meaning")
                || h.contains("notes")
                || h.contains("full name")
        });

        let mut fields: Vec<RegisterFieldRecord> = Vec::new();
        for row in &body {
            let cell = |i: usize| {
                row.get(i)
                    .map(|c| c.text.trim().to_string())
                    .filter(|s| !s.is_empty())
            };
            let field_name = match cell(field_col) {
                Some(n) => n,
                None => continue,
            };
            // A field row needs a real name token, not a sentence (legend/wrapped rows).
            if field_name.split_whitespace().count() > 4 {
                continue;
            }
            // Drop rows that merely echo a column title (a repeated/legend header landing in the body,
            // e.g. a "Field|Description|Access|Reset" format-legend table) — general, not chip-specific.
            if matches!(
                field_name.to_ascii_lowercase().as_str(),
                "field"
                    | "bits"
                    | "bit"
                    | "reset"
                    | "access"
                    | "type"
                    | "description"
                    | "name"
                    | "attributes"
                    | "default"
                    | "offset"
                    | "identifier"
            ) {
                continue;
            }
            let (bits_high, bits_low) = bits_col
                .and_then(|c| row.get(c))
                .map(|c| parse_bit_range(&c.text))
                .unwrap_or((None, None));
            let description = desc_col.and_then(cell);
            // EXTRACTION-GAP-FIX.2 — recover the field mnemonic from the description when the "name" is a
            // bit-range (no dedicated name column); if none is present, the bit-range stays as an honest
            // residual — a mnemonic is never fabricated.
            let field_name = if name_is_bit_range_column && is_bit_range_token(&field_name) {
                description
                    .as_deref()
                    .and_then(field_mnemonic_from_description)
                    .unwrap_or(field_name)
            } else {
                field_name
            };
            let enumerated_values = description
                .as_deref()
                .map(parse_inline_field_enums)
                .unwrap_or_default();
            fields.push(RegisterFieldRecord {
                field_name,
                bits_high,
                bits_low,
                bit_width: bit_width_from_range(bits_high, bits_low),
                access_type: access_col.and_then(cell),
                reset_value: reset_col.and_then(cell),
                description,
                enumerated_values,
            });
        }
        if fields.is_empty() {
            continue;
        }
        // Register name: prefer the table caption; EXTRACTION-GAP-FIX.3 — else recover it from the nearest
        // preceding section heading that DEFINES a register (`<Title> (<name>, at 0x..)`); else keep the
        // synthetic `register_<table_id>` placeholder (an honest residual — a name is never fabricated).
        let register_name = table
            .caption_text
            .as_deref()
            .and_then(register_name_from_caption)
            .or_else(|| {
                nearest_section_title_original(source_ir, table)
                    .as_deref()
                    .and_then(register_name_from_heading)
            })
            .unwrap_or_else(|| format!("register_{}", table.table_id));
        let size_bits = register_size_from_fields(&fields);
        records.push(RegisterRecord {
            register_id: format!("regfld_{}", table.table_id),
            register_name,
            offset_address: None,
            size_bits,
            fields,
            supporting_statement_ids: Vec::new(),
            automation_confidence: AutomationConfidence::Medium,
        });
    }
    records
}

/// Register width = the maximum field bit extent + 1, when any field carries a high bit.
fn register_size_from_fields(fields: &[RegisterFieldRecord]) -> Option<u32> {
    fields
        .iter()
        .filter_map(|f| f.bits_high)
        .max()
        .map(|h| h + 1)
}

/// De-fragment register-FIELD tables a PDF backend split across several tables, so ONE register's
/// fields live in ONE `RegisterRecord` instead of several (EXTRACTION-GAP-FIX.4c). Conservative by
/// construction: a same-`register_name` group is merged ONLY when its field names are ALL distinct
/// (case-insensitively) across the whole group. That single test is exactly the two safety gates —
/// (1) no fragment repeats a field name internally (rejects a garbled bit-row read as one field
/// repeated, e.g. `sizelo`×13) and (2) the fragments are pairwise disjoint (rejects distinct
/// registers an upstream heading-association collapsed to one name, e.g. an array `sbaddressN` all
/// named `sbaddress3`, each carrying the same `address` field). So it never fabricates a field set
/// (honesty guardrail): an ambiguous or garbled group is left exactly as it was. The merged record is
/// emitted at the FIRST fragment's original position, so the output order stays deterministic.
fn consolidate_register_field_fragments(records: &mut Vec<RegisterRecord>) {
    use std::collections::HashMap;
    // Group record indices by register name (insertion order is the records' own order).
    let mut by_name: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, rec) in records.iter().enumerate() {
        by_name
            .entry(rec.register_name.clone())
            .or_default()
            .push(i);
    }
    let mut out: Vec<RegisterRecord> = Vec::with_capacity(records.len());
    let mut consumed: Vec<bool> = vec![false; records.len()];
    for i in 0..records.len() {
        if consumed[i] {
            continue;
        }
        let idxs = &by_name[&records[i].register_name];
        if idxs.len() > 1 && register_fragments_are_safe_to_merge(idxs.iter().map(|&j| &records[j]))
        {
            out.push(merge_register_fragments(idxs.iter().map(|&j| &records[j])));
            for &j in idxs {
                consumed[j] = true;
            }
        } else {
            out.push(records[i].clone());
            consumed[i] = true;
        }
    }
    *records = out;
}

/// True when the field names across a same-name register group are all distinct (case-insensitively)
/// — the conservative safety test for [`consolidate_register_field_fragments`]. Any repeated field
/// name (within one fragment or across them) means the group is garbled or collapses distinct
/// registers, so it must NOT be merged.
fn register_fragments_are_safe_to_merge<'a>(
    fragments: impl Iterator<Item = &'a RegisterRecord>,
) -> bool {
    let mut seen = std::collections::HashSet::new();
    for fragment in fragments {
        for field in &fragment.fields {
            if !seen.insert(field.field_name.to_ascii_lowercase()) {
                return false;
            }
        }
    }
    true
}

/// Merge a verified-safe same-name register group into one record: the first fragment's identity and
/// metadata, the UNION of fields in fragment order, the register width recomputed from the full field
/// set (falling back to the first resolved fragment width), the first resolved `offset_address`, and
/// the de-duplicated UNION of supporting statement ids.
fn merge_register_fragments<'a>(
    fragments: impl Iterator<Item = &'a RegisterRecord>,
) -> RegisterRecord {
    let mut iter = fragments;
    let first = iter.next().expect("a group has at least one fragment");
    let mut merged = first.clone();
    let mut statement_ids: std::collections::BTreeSet<String> =
        first.supporting_statement_ids.iter().cloned().collect();
    for fragment in iter {
        merged.fields.extend(fragment.fields.iter().cloned());
        if merged.offset_address.is_none() {
            merged.offset_address = fragment.offset_address.clone();
        }
        if merged.size_bits.is_none() {
            merged.size_bits = fragment.size_bits;
        }
        statement_ids.extend(fragment.supporting_statement_ids.iter().cloned());
    }
    // The full field set can resolve a width the partial fragments could not.
    merged.size_bits = register_size_from_fields(&merged.fields).or(merged.size_bits);
    merged.supporting_statement_ids = statement_ids.into_iter().collect();
    merged
}

#[cfg(test)]
mod register_fragment_consolidation_4c {
    use super::*;
    use crate::ir::source::{AutomationConfidence, RegisterFieldRecord, RegisterRecord};

    fn field(name: &str) -> RegisterFieldRecord {
        RegisterFieldRecord {
            field_name: name.to_string(),
            bits_high: None,
            bits_low: None,
            bit_width: None,
            access_type: None,
            reset_value: None,
            description: None,
            enumerated_values: vec![],
        }
    }
    fn frag(id: &str, name: &str, fields: &[&str]) -> RegisterRecord {
        RegisterRecord {
            register_id: id.to_string(),
            register_name: name.to_string(),
            offset_address: None,
            size_bits: None,
            fields: fields.iter().map(|f| field(f)).collect(),
            supporting_statement_ids: vec![format!("stmt_{id}")],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn disjoint_same_name_fragments_merge_into_one_record() {
        // Real RISC-V dmcontrol shape: fields split across 3 Docling tables, all distinct → merge.
        let mut recs = vec![
            frag("regfld_table_0023", "dmcontrol", &["version"]),
            frag(
                "regfld_table_0024",
                "dmcontrol",
                &["haltreq", "resumereq", "hartreset"],
            ),
            frag(
                "regfld_table_0025",
                "dmcontrol",
                &["hasel", "hartsello", "hartselhi"],
            ),
        ];
        consolidate_register_field_fragments(&mut recs);
        assert_eq!(recs.len(), 1);
        assert_eq!(recs[0].register_name, "dmcontrol");
        assert_eq!(recs[0].fields.len(), 7);
        // Provenance is unioned, not lost.
        assert_eq!(recs[0].supporting_statement_ids.len(), 3);
    }

    #[test]
    fn internally_duplicated_fragment_blocks_the_merge() {
        // Real RISC-V mcontrol shape: a garbled bit-row read as `sizelo` repeated → NOT merged
        // (never fabricate a field set — honesty guardrail).
        let mut recs = vec![
            frag("regfld_table_0072", "mcontrol", &["maskmax", "sizehi"]),
            frag(
                "regfld_table_0075",
                "mcontrol",
                &["sizelo", "sizelo", "sizelo"],
            ),
        ];
        let before = recs.clone();
        consolidate_register_field_fragments(&mut recs);
        assert_eq!(
            recs, before,
            "a garbled (internally duplicated) fragment must block the merge"
        );
    }

    #[test]
    fn array_collapsed_to_one_name_is_not_merged() {
        // Real RISC-V shape: sbaddress0..3 each carry one `address` field but were all named
        // `sbaddress3`; the repeated field name means distinct registers → NOT merged.
        let mut recs = vec![
            frag("regfld_table_0039", "sbaddress3", &["address"]),
            frag("regfld_table_0040", "sbaddress3", &["address"]),
        ];
        let before = recs.clone();
        consolidate_register_field_fragments(&mut recs);
        assert_eq!(recs, before);
    }

    #[test]
    fn distinct_named_registers_and_order_are_preserved() {
        // Different names are never merged; output order follows first appearance, and a merged
        // register lands at its FIRST fragment's position.
        let mut recs = vec![
            frag("r1", "CAP", &["mqes"]),
            frag("r2", "dmcontrol", &["version"]),
            frag("r3", "CC", &["en"]),
            frag("r4", "dmcontrol", &["haltreq"]),
        ];
        consolidate_register_field_fragments(&mut recs);
        let names: Vec<&str> = recs.iter().map(|r| r.register_name.as_str()).collect();
        assert_eq!(names, vec!["CAP", "dmcontrol", "CC"]);
        let dm = recs
            .iter()
            .find(|r| r.register_name == "dmcontrol")
            .unwrap();
        assert_eq!(dm.fields.len(), 2);
    }

    #[test]
    fn singletons_are_untouched() {
        let mut recs = vec![frag("r1", "CAP", &["mqes", "cqr"])];
        let before = recs.clone();
        consolidate_register_field_fragments(&mut recs);
        assert_eq!(recs, before);
    }

    #[test]
    fn case_insensitive_repeat_blocks_the_merge() {
        // `EN` and `en` are the same field; a cross-fragment case-only repeat must block the merge.
        let mut recs = vec![
            frag("r1", "REG", &["EN", "MODE"]),
            frag("r2", "REG", &["en"]),
        ];
        let before = recs.clone();
        consolidate_register_field_fragments(&mut recs);
        assert_eq!(recs, before);
    }
}

/// A register-DIAGRAM/bit-layout grid (table columns are bit POSITIONS `15|14|…|0`) mis-read as a
/// register: every "field" is named by a bare number. A real register definition always has at least one
/// named (non-numeric) field, so an all-numeric-field record is a layout grid, not a field definition.
/// PDF-VARIANT-DIGESTION.2c no-garbage filter (general; no chip names).
fn register_is_bit_layout_grid(reg: &RegisterRecord) -> bool {
    !reg.fields.is_empty()
        && reg
            .fields
            .iter()
            .all(|f| f.field_name.trim().bytes().all(|b| b.is_ascii_digit()))
}

/// PDF-VARIANT-DIGESTION.2d — a NON-DATA noise table (table of contents, list of tables/figures, revision
/// history, section index) that should stay unextracted. Recognized by GENERAL structural signals — dotted
/// page-leaders, a contents/revision caption-or-header, or rows mostly prefixed by a section number — never
/// chip names (ADR 0006). Used to skip wasted VLM work and to report a clean noise count.
pub fn table_is_noise(table: &StructuredTableRecord) -> bool {
    let all_cells = || {
        table
            .header_rows
            .iter()
            .chain(table.body_rows.iter())
            .flat_map(|r| r.iter().map(|c| c.text.trim()))
    };
    if all_cells().next().is_none() {
        return false;
    }
    // 1. Dotted page-leaders ("Preface . . . . . 1", "BOOT_BUS [177]....184").
    let dotted = all_cells()
        .filter(|c| c.contains("....") || c.matches(". .").count() >= 3)
        .count();
    if dotted >= 2 {
        return true;
    }
    // 2. A contents / revision-history caption or header.
    let mut blob = table
        .caption_text
        .clone()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if let Some(h) = table.header_rows.first() {
        for c in h {
            blob.push(' ');
            blob.push_str(&c.text.to_ascii_lowercase());
        }
    }
    if [
        "table of contents",
        "list of tables",
        "list of figures",
        "revision history",
        "document history",
    ]
    .iter()
    .any(|k| blob.contains(k))
    {
        return true;
    }
    let has = |k: &str| blob.contains(k);
    if has("version")
        && (has("issue date")
            || has("comments")
            || has("changes")
            || (has("date") && has("description")))
    {
        return true;
    }
    // 3. Section-index rows: most first-column body cells start with a section number ("1.1.", "B4.2").
    let firsts: Vec<&str> = table
        .body_rows
        .iter()
        .filter_map(|r| r.first())
        .map(|c| c.text.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if firsts.len() >= 3 {
        let idx = firsts
            .iter()
            .filter(|s| looks_like_section_number(s))
            .count();
        if idx * 2 >= firsts.len() {
            return true;
        }
    }
    false
}

/// A leading section-number token like `1`, `1.1`, `5.3.`, `B4.2` — digits/dots (an optional leading letter),
/// requiring at least one digit AND one dot so single numbers (a bit index) and bit ranges are not matched.
fn looks_like_section_number(s: &str) -> bool {
    let token = s
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches('.');
    if token.is_empty() {
        return false;
    }
    let (mut has_digit, mut has_dot) = (false, false);
    for ch in token.chars() {
        match ch {
            c if c.is_ascii_digit() => has_digit = true,
            '.' => has_dot = true,
            c if c.is_ascii_alphabetic() => {}
            _ => return false,
        }
    }
    has_digit && has_dot
}

/// Field width from a `[high:low]` range: `high - low + 1` when both bounds are present and ordered.
fn bit_width_from_range(bits_high: Option<u32>, bits_low: Option<u32>) -> Option<u32> {
    match (bits_high, bits_low) {
        (Some(h), Some(l)) if h >= l => Some(h - l + 1),
        _ => None,
    }
}

/// Parse inline field value ENUMERATIONS from a field description — conservatively, only
/// binary/hex/Verilog value literals (`0b00: Idle`, `0x1 = Busy`, `2'b01: …`) so bit references and
/// counts are not mis-read as enums. General (no chip names); empty when none. PDF-VARIANT-DIGESTION.2c.
fn parse_inline_field_enums(desc: &str) -> Vec<RegisterFieldEnumRecord> {
    let mut out = Vec::new();
    for piece in desc.split([',', ';', '\n']) {
        let piece = piece.trim();
        let Some(sep) = piece.find([':', '=']) else {
            continue;
        };
        let value = piece[..sep].trim();
        let meaning = piece[sep + 1..].trim();
        if meaning.is_empty() || meaning.len() > 80 || !is_register_value_literal(value) {
            continue;
        }
        out.push(RegisterFieldEnumRecord {
            value: value.to_string(),
            meaning: meaning.to_string(),
        });
    }
    out
}

/// Is `s` a register value literal: `0b<bin>`, `0x<hex>`, or `<width>'b<bin>`?
fn is_register_value_literal(s: &str) -> bool {
    let s = s.trim();
    let lower = s.to_ascii_lowercase();
    if let Some(rest) = lower.strip_prefix("0b") {
        return !rest.is_empty() && rest.bytes().all(|b| b == b'0' || b == b'1');
    }
    if let Some(rest) = lower.strip_prefix("0x") {
        return !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_hexdigit());
    }
    if let Some(idx) = lower.find("'b") {
        let (width, bin) = (&lower[..idx], &lower[idx + 2..]);
        return !width.is_empty()
            && width.bytes().all(|b| b.is_ascii_digit())
            && !bin.is_empty()
            && bin.bytes().all(|b| b == b'0' || b == b'1');
    }
    false
}

/// Generic AGENT-CLASS nouns a spec uses to DEFINE an agent in prose:
/// `"<NAME> is a/an/the <agent-class> that/which <capability>"`. Curated to clear agent/
/// component-class nouns only — no chip/vendor/protocol names (ADR 0006) — so a non-agent
/// `"is a <X> that …"` (e.g. "is a signal that", "is a register that") cannot mint a false
/// actor. `"device"` keeps the original `.3b` form working unchanged.
const AGENT_CLASS_NOUNS: &[&str] = &[
    "device",
    "component",
    "agent",
    "module",
    "entity",
    "controller",
    "manager",
    "master",
    "initiator",
    "peripheral",
    "bridge",
    "engine",
    "processor",
    "host",
    "node",
    "subsystem",
];

/// The alphabetic core of a token, lowercased (punctuation stripped) — used to match the
/// article / agent-class noun / relativizer tokens of an agent definition.
fn alpha_lower(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_ascii_alphabetic())
        .to_ascii_lowercase()
}

/// Strip a single trailing `"(…)"` group from `s` — e.g. a `"(refer to section 3.1.2.1)"`
/// cross-reference that would otherwise make the NAME extraction pick the parenthetical's last
/// word ("section") instead of the real agent ("An I/O controller (refer to …) is a controller
/// that …" → the agent is "controller", not "section"). Returns the slice before the group.
fn strip_trailing_parenthetical(s: &str) -> &str {
    let trimmed = s.trim_end();
    if trimmed.ends_with(')')
        && let Some(open) = trimmed.rfind('(')
    {
        return &trimmed[..open];
    }
    trimmed
}

/// Parse `"<NAME> is a/an/the <agent-class> that/which <capability>"` agent definitions from
/// prose (`PDF-VARIANT-DIGESTION.8`, generalizing the `.3b` literal `"is the device that/which"`
/// to a curated set of generic agent-class nouns). One single forward pass over `" is "`
/// occurrences; returns `(name, definition?)` per match. The caller gates NAME through
/// `is_agent_noun` and dedups, so this stays a permissive grammar with the safety in the gate.
/// General grammar, no chip/vendor names (ADR 0006).
fn agent_definitions(text: &str) -> Vec<(String, Option<String>)> {
    let lower = text.to_ascii_lowercase();
    let mut out = Vec::new();
    for (idx, _) in lower.match_indices(" is ") {
        // NAME = the last alphabetic word before " is ", after (a) dropping a trailing
        // "(refer to section …)" cross-reference and (b) confining the search to the current
        // sentence — so an anaphoric "… host system. It is the entity that …" cannot reach back
        // across the period and mis-name the antecedent ("system"). Gated by the caller.
        let pre = strip_trailing_parenthetical(&text[..idx]);
        let sentence = &pre[pre.rfind(['.', '!', '?']).map_or(0, |p| p + 1)..];
        // A genuine definitional subject is a simple noun phrase ("A controller is …", "An I/O
        // controller is …"). When a PREPOSITION sits between the sentence start and " is "
        // (e.g. "A typical use case FOR multiple HSELx signals is a peripheral that …"), the
        // last noun before " is " is a prepositional-phrase OBJECT ("signals"), not the
        // subject — so the heuristic would mis-name it. Prepositions are a CLOSED grammatical
        // class, so this is a principled structural guard, not an open-ended noun denylist.
        if sentence.split_whitespace().any(|w| {
            matches!(
                alpha_lower(w).as_str(),
                "of" | "for"
                    | "in"
                    | "on"
                    | "with"
                    | "to"
                    | "from"
                    | "by"
                    | "at"
                    | "per"
                    | "into"
                    | "onto"
                    | "over"
                    | "under"
                    | "via"
                    | "within"
                    | "between"
                    | "across"
                    | "among"
                    | "through"
                    | "during"
                    | "against"
            )
        }) {
            continue;
        }
        let Some(name) = last_alpha_word(sentence) else {
            continue;
        };
        let mut words = text[idx + 4..].split_whitespace().peekable();
        // Optional leading article.
        if words
            .peek()
            .is_some_and(|first| matches!(alpha_lower(first).as_str(), "a" | "an" | "the"))
        {
            words.next();
        }
        // An agent-class noun must follow — otherwise this is not an agent definition.
        let Some(class_word) = words.next() else {
            continue;
        };
        let class_l = alpha_lower(class_word);
        if !AGENT_CLASS_NOUNS.contains(&class_l.as_str()) {
            continue;
        }
        // A defining relative clause (`that` / `which`) confirms a definition, not a passing
        // "is a component of …" part-of mention.
        let Some(rel) = words.next() else {
            continue;
        };
        if !matches!(alpha_lower(rel).as_str(), "that" | "which") {
            continue;
        }
        let rest = words.collect::<Vec<_>>().join(" ");
        let rest = rest.trim().trim_end_matches('.').trim().to_string();
        let def = (!rest.is_empty()).then(|| format!("the {class_l} that {rest}"));
        out.push((name, def));
    }
    out
}

/// EXTRACTOR-ARCHITECTURE.7 — the protocol-actors surface as a registered `Extractor` (currently a single
/// strategy). Running it through the `run_surface_concat` driver gives it a uniform run-manifest entry and
/// readies it for future multi-strategy growth (more prose agent-definition forms plug in as additional
/// units), consistent with the "digest more PDF variants" aim. One producer → concat is identity → output is
/// byte-identical to the prior direct call.
struct ProtocolActorExtractor;
impl Extractor<ProtocolActorRecord> for ProtocolActorExtractor {
    fn name(&self) -> &'static str {
        "actors.prose"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<ProtocolActorRecord> {
        extract_protocol_actors(cx.statements)
    }
}

/// PDF-VARIANT-DIGESTION.3b/.8 — capture protocol ACTORS/AGENTS a spec DEFINES in prose. Two general forms:
/// "<NAME> is a/an/the <agent-class> which/that <capability>" (`.8` generalizes `.3b`'s literal "device" anchor
/// to `AGENT_CLASS_NOUNS`) and "considered a/the/an <NAME>". NAME must be a plausible agent noun (alphabetic,
/// not a function/structural word) — general agent-definition grammar, no chip names.
fn extract_protocol_actors(statements: &[ExtractedStatement]) -> Vec<ProtocolActorRecord> {
    let mut out = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut counter = 0usize;
    for statement in statements {
        let lower = statement.text.to_ascii_lowercase();
        // Form 1: "<NAME> is a/an/the <agent-class> which/that <capability>".
        let mut found: Vec<(String, Option<String>)> = agent_definitions(&statement.text);
        // Form 2: "considered a/the/an <NAME>".
        for marker in [" considered a ", " considered the ", " considered an "] {
            let Some(pos) = lower.find(marker) else {
                continue;
            };
            let Some(name) = first_alpha_word(&statement.text[pos + marker.len()..]) else {
                continue;
            };
            found.push((name, None));
        }
        for (name, def) in found {
            let key = name.to_ascii_lowercase();
            if !is_agent_noun(&key) || !seen.insert(key.clone()) {
                continue;
            }
            counter += 1;
            out.push(ProtocolActorRecord {
                actor_id: format!("protocol_actor_{counter:04}"),
                name,
                definition: def,
                supporting_statement_ids: vec![statement.statement_id.clone()],
            });
        }
    }
    out
}

/// The last alphabetic word of `s` (the agent noun before "is the device …"), stripped of punctuation.
fn last_alpha_word(s: &str) -> Option<String> {
    s.split_whitespace()
        .rev()
        .map(|w| {
            w.trim_matches(|c: char| !c.is_ascii_alphabetic())
                .to_string()
        })
        .find(|w| w.len() >= 3)
}

/// The first alphabetic word of `s` (the agent noun after "considered a …"), stripped of punctuation.
fn first_alpha_word(s: &str) -> Option<String> {
    s.split_whitespace()
        .map(|w| {
            w.trim_matches(|c: char| !c.is_ascii_alphabetic())
                .to_string()
        })
        .find(|w| w.len() >= 3)
}

/// Is `word` (lowercased) a plausible agent/role noun and not a function/structural word? Permissive enough
/// to admit vendor-specific agents (smmu, requester, completer) but rejects articles/connectives and the
/// non-agent nouns the patterns can otherwise pick up.
fn is_agent_noun(word: &str) -> bool {
    if word.len() < 3 || word.len() > 24 || !word.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    !matches!(
        word,
        "the"
            | "and"
            | "for"
            | "that"
            | "which"
            | "this"
            | "these"
            | "those"
            | "each"
            | "any"
            | "all"
            | "one"
            | "two"
            | "device"
            | "devices"
            | "bus"
            | "data"
            | "clock"
            | "signal"
            | "line"
            | "same"
            | "other"
            | "such"
            | "first"
            | "second"
            | "only"
            | "both"
            | "single"
    )
}

/// Parse a bit-range string like "7:0", "[7:0]", or "31" into (bits_high, bits_low).
fn parse_bit_range(text: &str) -> (Option<u32>, Option<u32>) {
    let cleaned: String = text
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ':')
        .collect();
    if let Some(colon) = cleaned.find(':') {
        let high = cleaned[..colon].parse::<u32>().ok();
        let low = cleaned[colon + 1..].parse::<u32>().ok();
        (high, low)
    } else if let Ok(bit) = cleaned.parse::<u32>() {
        (Some(bit), Some(bit))
    } else {
        (None, None)
    }
}

/// True when `s` is a BIT-RANGE token — only digits, `:`, brackets and whitespace, with at least one
/// digit (`60:59`, `15:00`, `58`, `[7:0]`). Used to detect register-field tables whose "name" column is
/// actually the bit specification (NVMe-style `Bits|Type|Reset|Description`), so the real field identity
/// must be recovered from elsewhere. Pure grammar (ADR 0006); EXTRACTION-GAP-FIX.2.
fn is_bit_range_token(s: &str) -> bool {
    let s = s.trim();
    !s.is_empty()
        && s.bytes().any(|b| b.is_ascii_digit())
        && s.chars()
            .all(|c| c.is_ascii_digit() || matches!(c, ':' | '[' | ']' | ' ' | '\t'))
}

/// A register-field MNEMONIC: an uppercase alphanumeric abbreviation token (2–12 chars, starting with a
/// letter, only ASCII uppercase letters and digits — e.g. `MQES`, `CSS`, `TO`, `MPSMAX`). General; carries
/// no specific chip/protocol names (ADR 0006). EXTRACTION-GAP-FIX.2.
fn is_field_mnemonic_token(s: &str) -> bool {
    let s = s.trim();
    let len = s.chars().count();
    (2..=12).contains(&len)
        && s.chars()
            .next()
            .map(|c| c.is_ascii_uppercase())
            .unwrap_or(false)
        && s.chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
}

/// Recover a register-field MNEMONIC from its DESCRIPTION when the field's "name" column is actually a
/// bit-range (NVMe-style `Bits|Type|Reset|Description` tables). The mnemonic is the parenthesized
/// abbreviation in the universal defined-term prefix `Full Field Name (MNEMONIC): …` — the first
/// `(<MNEMONIC>):` group where `<MNEMONIC>` is an uppercase alphanumeric token (the `:` is required so a
/// passing reference like `(CC.MPS)` is not mistaken for the field's own definition). Returns `None` when
/// the description carries no such defined term — then the caller keeps the bit-range as an honest
/// residual; a mnemonic is NEVER fabricated (honesty guardrail). Pure grammar (ADR 0006);
/// EXTRACTION-GAP-FIX.2.
fn field_mnemonic_from_description(desc: &str) -> Option<String> {
    // The field's own `(MNEMONIC):` marker sits at the start of the description (modulo cell-bleed from a
    // prior row, which carries no `(X):`), so a bounded leading scan is enough and avoids matching an
    // unrelated `(X):` deep in a long description.
    let cap = desc.len().min(256);
    let bytes = desc.as_bytes();
    let mut i = 0;
    while i < cap {
        // `(` is ASCII, so it only ever appears at a char boundary; slicing at `i + 1` is safe.
        if bytes[i] == b'(' {
            let rest = &desc[i + 1..];
            let Some(close_rel) = rest.find(')') else {
                break;
            };
            let inner = rest[..close_rel].trim();
            let after = rest[close_rel + 1..].trim_start();
            if after.starts_with(':') && is_field_mnemonic_token(inner) {
                return Some(inner.to_string());
            }
            i += 1 + close_rel + 1;
            continue;
        }
        i += 1;
    }
    None
}

/// Synthesize `TimingConstraintRecord` entries from `timing_parameter` tables in `SourceIR`.
fn synthesize_timing_constraints(
    source_ir: &SourceIr,
    prior_guidance: Option<&EvidencePriorGuidance>,
) -> Vec<TimingConstraintRecord> {
    let mut records: Vec<TimingConstraintRecord> = Vec::new();
    let timing_tables: Vec<_> = source_ir
        .structured_tables
        .iter()
        .filter(|t| {
            matches!(
                effective_table_kind(t, prior_guidance),
                TableKind::TimingParameter
            )
        })
        .collect();

    for table in timing_tables {
        let header: Vec<String> = table
            .header_rows
            .first()
            .map(|r| r.iter().map(|c| c.text.to_ascii_lowercase()).collect())
            .unwrap_or_default();

        // PDF-VARIANT-DIGESTION.9.11 — recover data rows Docling trapped in `header_rows` by marking
        // the row-LABEL cell `is_header=true` (e.g. I2S `table_0004`, SMBus `table_0012`, which leave
        // `body_rows` empty so the table yields nothing). Past the first (column-header) row, a
        // `header_rows` entry whose VALUE cells are all `is_header=false` and whose first cell is a
        // non-empty label is really a DATA row. A genuine multi-row column header (a nested cross-tab
        // whose value cells stay `is_header=true`, e.g. I2S `table_0005`) fails this and is left an
        // honest residual. Purely structural — no parameter-name list, no case dependence (ADR 0006).
        let recovered_rows = table.header_rows.iter().skip(1).filter(|row| {
            row.len() >= 2
                && !row[0].text.trim().is_empty()
                && row.iter().skip(1).all(|cell| !cell.is_header)
        });
        let effective_rows: Vec<&Vec<StructuredTableCellRecord>> =
            table.body_rows.iter().chain(recovered_rows).collect();
        if effective_rows.is_empty() {
            continue;
        }

        let name_col = header
            .iter()
            .position(|h| h.contains("parameter") || h.contains("symbol") || h.contains("name"))
            .unwrap_or(0);
        let min_col = header.iter().position(|h| h.contains("min"));
        let typ_col = header
            .iter()
            .position(|h| h.contains("typ") || h.contains("typical"));
        let max_col = header.iter().position(|h| h.contains("max"));
        let unit_col = header
            .iter()
            .position(|h| h.contains("unit") || h.contains("ns") || h.contains("ps"));
        let desc_col = header.iter().position(|h| h.contains("description"));

        let table_id = table.table_id.clone();
        for (row_idx, row) in effective_rows.iter().enumerate() {
            let name = row
                .get(name_col)
                .map(|c| c.text.trim().to_string())
                .unwrap_or_default();
            if name.is_empty() {
                continue;
            }

            let get_cell = |col: Option<usize>| -> Option<String> {
                col.and_then(|c| row.get(c))
                    .map(|cell| cell.text.trim().to_string())
                    .filter(|s| !s.is_empty() && s != "-")
            };

            records.push(TimingConstraintRecord {
                constraint_id: format!("timing_{}_{row_idx:03}", document_key(&table_id)),
                parameter_name: name,
                min_value: get_cell(min_col),
                typ_value: get_cell(typ_col),
                max_value: get_cell(max_col),
                unit: get_cell(unit_col),
                description: get_cell(desc_col),
                supporting_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    records
}

fn dedup_actor_signal_relations(relations: Vec<ActorSignalRelation>) -> Vec<ActorSignalRelation> {
    let mut deduped = Vec::new();
    let mut seen = HashSet::new();
    for relation in relations {
        let key = (
            relation.actor_name.clone(),
            relation.signal_name.clone(),
            matches!(relation.relation, RelationKind::Drives) as u8,
        );
        if seen.insert(key) {
            deduped.push(relation);
        }
    }
    deduped
}

/// EXTRACTOR-ARCHITECTURE.9c — the prose actor→signal relation strategy (the active/passive
/// normative-verb KG-edge grammar from `normative_vocab` over the per-pass statements) as a registered
/// `Extractor`. Reads the shared context's statements plus the per-pass known-signal set.
struct ActorSignalRelationProseExtractor<'a> {
    known_signals: &'a HashSet<String>,
}
impl Extractor<ActorSignalRelation> for ActorSignalRelationProseExtractor<'_> {
    fn name(&self) -> &'static str {
        "relations.prose"
    }
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<ActorSignalRelation> {
        extract_actor_signal_relations(cx.statements, self.known_signals)
    }
}

/// EXTRACTOR-ARCHITECTURE.9c — the signal-table `Source` / `Destination` column strategy as a registered
/// `Extractor`. The relation list depends only on `SourceIr`, so the build precomputes it ONCE outside
/// the fixed-point loop and this unit re-emits it per pass — exactly the legacy per-pass cloned extend.
struct ActorSignalRelationTableExtractor<'a> {
    table_relations: &'a [ActorSignalRelation],
}
impl Extractor<ActorSignalRelation> for ActorSignalRelationTableExtractor<'_> {
    fn name(&self) -> &'static str {
        "relations.tables"
    }
    fn run(&self, _cx: &ExtractionContext<'_>) -> Vec<ActorSignalRelation> {
        self.table_relations.to_vec()
    }
}

/// Run the actor-signal-relations surface through the unified concat driver (EXTRACTOR-ARCHITECTURE.9c),
/// then apply the two ORDERED legacy post-passes: check-signal augmentation FIRST (it inherits relations
/// for check signals from the full, still-undeduped merged list — `relations_by_signal` must see every
/// strategy's records), THEN first-wins dedup by `(actor, signal, is_drives)`. The dedup deliberately
/// stays a post-pass rather than the driver's key-merge: key-merging would dedup BEFORE augmentation and
/// change what the augment step reads.
///
/// Called per pass of the convergence loop; `ExtractionManifest::record` replaces per surface name, so
/// the manifest ends up holding exactly the FINAL (converged) pass's run.
fn actor_signal_relation_surface(
    source_ir: &SourceIr,
    statements: &[ExtractedStatement],
    known_signals: &HashSet<String>,
    table_relations: &[ActorSignalRelation],
    prior_guidance: Option<&EvidencePriorGuidance>,
    manifest: &mut ExtractionManifest,
) -> Vec<ActorSignalRelation> {
    let cx = ExtractionContext { statements };
    let prose = ActorSignalRelationProseExtractor { known_signals };
    let tables = ActorSignalRelationTableExtractor { table_relations };
    let extractors: [&dyn Extractor<ActorSignalRelation>; 2] = [&prose, &tables];
    let run = run_surface_concat("actor_signal_relations", &cx, &extractors);
    manifest.record(&run);
    let augmented =
        augment_check_signal_relations_from_tables(source_ir, &run.records, prior_guidance);
    dedup_actor_signal_relations(augmented)
}

#[expect(
    clippy::type_complexity,
    reason = "evidence convergence returns the synchronized extraction families that must remain aligned"
)]
fn converge_evidence_extractions(
    source_ir: &SourceIr,
    base_extracted_statements: Vec<ExtractedStatement>,
    seed_synthesized_statements: Vec<ExtractedStatement>,
    contract_statements: Vec<ExtractedStatement>,
    statement_counter: &mut usize,
    prior_guidance: Option<&EvidencePriorGuidance>,
    extraction_manifest: &mut ExtractionManifest,
) -> (
    Vec<ExtractedStatement>,
    Vec<SignalConstraintRecord>,
    Vec<ConditionalRuleRecord>,
    Vec<SignalPolarityRecord>,
    Vec<SignalPolarityConflictRecord>,
    Vec<ActorSignalRelation>,
    EvidenceConvergenceReport,
) {
    let signal_names_from_tables = collect_signal_names_from_tables(source_ir, prior_guidance);
    let signal_widths_from_tables = collect_signal_widths_from_tables(source_ir, prior_guidance);
    let table_relations =
        extract_relations_from_signal_tables_with_prior_guidance(source_ir, prior_guidance);
    let mut dynamic_synthesized_statements = Vec::new();
    let mut final_extracted_statements = Vec::new();
    let mut final_signal_constraints = Vec::new();
    let mut final_conditional_rules = Vec::new();
    let mut final_signal_polarities = Vec::new();
    let mut final_signal_polarity_conflicts = Vec::new();
    let mut final_actor_signal_relations = Vec::new();
    let max_passes = source_ir.structured_tables.len().max(1) + 4;
    let mut new_facts_per_pass: Vec<usize> = Vec::new();
    let mut converged = false;

    for _pass in 0..max_passes {
        let mut extracted_statements = base_extracted_statements.clone();
        extracted_statements.extend(seed_synthesized_statements.iter().cloned());
        extracted_statements.extend(contract_statements.iter().cloned());
        extracted_statements.extend(dynamic_synthesized_statements.iter().cloned());

        let mut known_signals = signal_names_from_tables.clone();
        known_signals.extend(collect_known_signal_names(&extracted_statements));
        let discovered_values = collect_discovered_enum_values(&[extracted_statements.as_slice()]);
        // EXTRACTOR-ARCHITECTURE.9b — the polarity observation surface runs through the unified concat
        // driver; recorded per pass, the manifest keeps the final (converged) pass's run.
        let signal_polarity = signal_polarity_surface(
            source_ir,
            &extracted_statements,
            &known_signals,
            prior_guidance,
            extraction_manifest,
        );

        let mut constraint_counter = 1usize;
        let mut signal_constraints =
            extract_signal_constraints(&extracted_statements, &mut constraint_counter);
        signal_constraints.extend(extract_dynamic_signal_constraints(
            &extracted_statements,
            &mut constraint_counter,
            &discovered_values,
        ));
        apply_signal_polarity_to_constraints(&mut signal_constraints, &signal_polarity.resolved);
        let conditional_rules =
            extract_conditional_rules(&extracted_statements, &mut constraint_counter);

        // EXTRACTOR-ARCHITECTURE.9c — the relations surface runs through the unified concat driver with
        // its two ordered post-passes (check-signal augmentation reads the full pre-dedup list, THEN
        // first-wins dedup); recorded per pass, the manifest keeps the final (converged) pass's run.
        let actor_signal_relations = actor_signal_relation_surface(
            source_ir,
            &extracted_statements,
            &known_signals,
            &table_relations,
            prior_guidance,
            extraction_manifest,
        );

        let already_declared =
            collect_signals_with_explicit_direction_declarations(&extracted_statements);
        let mut candidate_statements = scan_encoding_tables_by_signal_anchor(
            source_ir,
            &known_signals,
            statement_counter,
            prior_guidance,
        );
        candidate_statements.extend(synthesize_directions_from_relations(
            &actor_signal_relations,
            &already_declared,
            &signal_widths_from_tables,
            statement_counter,
        ));

        let mut known_statement_texts = extracted_statements
            .iter()
            .map(|statement| statement.text.clone())
            .collect::<HashSet<_>>();
        let mut new_dynamic_statements = Vec::new();
        for statement in candidate_statements {
            if known_statement_texts.insert(statement.text.clone()) {
                new_dynamic_statements.push(statement);
            }
        }

        final_extracted_statements = extracted_statements;
        final_signal_constraints = signal_constraints;
        final_conditional_rules = conditional_rules;
        final_signal_polarities = signal_polarity.resolved_records;
        final_signal_polarity_conflicts = signal_polarity.conflicts;
        final_actor_signal_relations = actor_signal_relations;

        new_facts_per_pass.push(new_dynamic_statements.len());
        if new_dynamic_statements.is_empty() {
            converged = true;
            break;
        }
        dynamic_synthesized_statements.extend(new_dynamic_statements);
    }

    let convergence_report = EvidenceConvergenceReport {
        passes_run: new_facts_per_pass.len(),
        max_passes,
        total_new_facts: new_facts_per_pass.iter().sum(),
        new_facts_per_pass,
        converged,
    };

    (
        final_extracted_statements,
        final_signal_constraints,
        final_conditional_rules,
        final_signal_polarities,
        final_signal_polarity_conflicts,
        final_actor_signal_relations,
        convergence_report,
    )
}

/// Inject VLM-derived observations from `SourceIR.visual_assets[*].note` into the
/// corresponding `VisualEvidenceItem.observations` entries.
///
/// `specforge enrich` writes structured VLM extraction into the note field:
/// - `"vlm_timing_diagram_extraction: {json}"` → `TimingDiagramExtraction` observation
/// - `"vlm_state_machine_extraction: {json}"` → `StateMachineExtraction` observation
///
/// These observations are then available to `SemanticIR` for parsing into typed records.
fn inject_vlm_observations(
    assets: &[crate::ir::source::VisualAsset],
    visual_evidence: &mut [VisualEvidenceItem],
    asset_id_to_visual_index: &HashMap<String, usize>,
) {
    for asset in assets {
        let Some(note) = &asset.note else {
            continue;
        };
        let Some(visual_idx) = asset_id_to_visual_index.get(&asset.asset_id).copied() else {
            continue;
        };

        let (kind, json_str) = if let Some(s) = note.strip_prefix("vlm_timing_diagram_extraction: ")
        {
            (VisualObservationKind::TimingDiagramExtraction, s)
        } else if let Some(s) = note.strip_prefix("vlm_state_machine_extraction: ") {
            (VisualObservationKind::StateMachineExtraction, s)
        } else {
            continue;
        };

        let observation_id = format!("obs_vlm_{kind:?}_{}", &asset.asset_id);
        visual_evidence[visual_idx]
            .observations
            .push(VisualObservation {
                observation_id,
                kind,
                created_by: "specforge_vlm_enrich".to_string(),
                text: json_str.to_string(),
                supporting_span_ids: vec![],
                automation_confidence: AutomationConfidence::High,
            });

        // Upgrade VLM-enriched figures to Normative role —
        // timing and state machine diagrams are the most normative content in chip specs.
        if matches!(
            kind,
            VisualObservationKind::TimingDiagramExtraction
                | VisualObservationKind::StateMachineExtraction
        ) {
            visual_evidence[visual_idx].role = crate::ir::evidence::VisualEvidenceRole::Normative;
        }
    }
}

pub(crate) fn parse_visual_observation_json(text: &str) -> Option<serde_json::Value> {
    let trimmed = text.trim();
    serde_json::from_str::<serde_json::Value>(trimmed)
        .ok()
        .or_else(|| {
            extract_markdown_code_block(trimmed)
                .and_then(|candidate| serde_json::from_str::<serde_json::Value>(candidate).ok())
        })
        .or_else(|| {
            extract_first_json_object(trimmed)
                .and_then(|candidate| serde_json::from_str::<serde_json::Value>(candidate).ok())
        })
}

fn extract_markdown_code_block(text: &str) -> Option<&str> {
    let (fence_start, fence_len) = text
        .find("```json")
        .map(|index| (index, "```json".len()))
        .or_else(|| text.find("```JSON").map(|index| (index, "```JSON".len())))
        .or_else(|| text.find("```").map(|index| (index, "```".len())))?;

    let mut inner = &text[fence_start + fence_len..];
    inner = inner.trim_start_matches(|ch: char| ch.is_ascii_whitespace());
    if let Some(stripped) = inner.strip_prefix("json") {
        inner = stripped.trim_start_matches(|ch: char| ch.is_ascii_whitespace());
    } else if let Some(stripped) = inner.strip_prefix("JSON") {
        inner = stripped.trim_start_matches(|ch: char| ch.is_ascii_whitespace());
    }

    if let Some(end) = inner.find("```") {
        return Some(inner[..end].trim());
    }

    Some(inner.trim())
}

fn extract_first_json_object(text: &str) -> Option<&str> {
    let start = text.find(['{', '['])?;
    let opening = text[start..].chars().next()?;
    let closing = match opening {
        '{' => '}',
        '[' => ']',
        _ => return None,
    };

    let mut depth = 0usize;
    let mut in_string = false;
    let mut escape = false;
    for (offset, ch) in text[start..].char_indices() {
        if in_string {
            if escape {
                escape = false;
                continue;
            }
            match ch {
                '\\' => escape = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            ch if ch == opening => depth += 1,
            ch if ch == closing => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(&text[start..start + offset + ch.len_utf8()]);
                }
            }
            _ => {}
        }
    }

    None
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
}

fn page_number_from_page_id(page_id: &str) -> Option<u32> {
    page_id.rsplit('_').next()?.parse().ok()
}

#[derive(Debug, Clone, Copy)]
enum ReferenceKind {
    Figure,
    Table,
}

#[derive(Debug, Clone)]
struct ParsedCaptionReference {
    reference_kind: ReferenceKind,
    display_label: &'static str,
    number: String,
}

fn parse_caption_reference(caption_text: &str) -> Option<ParsedCaptionReference> {
    let trimmed_caption = caption_text.trim();
    let lowered_caption = trimmed_caption.to_ascii_lowercase();

    if let Some(number) = parse_prefixed_number(trimmed_caption, &lowered_caption, "figure ") {
        return Some(ParsedCaptionReference {
            reference_kind: ReferenceKind::Figure,
            display_label: "Figure",
            number,
        });
    }
    if let Some(number) = parse_prefixed_number(trimmed_caption, &lowered_caption, "fig. ") {
        return Some(ParsedCaptionReference {
            reference_kind: ReferenceKind::Figure,
            display_label: "Figure",
            number,
        });
    }
    if let Some(number) = parse_prefixed_number(trimmed_caption, &lowered_caption, "table ") {
        return Some(ParsedCaptionReference {
            reference_kind: ReferenceKind::Table,
            display_label: "Table",
            number,
        });
    }

    None
}

fn parse_prefixed_number(original_text: &str, lowered_text: &str, prefix: &str) -> Option<String> {
    if !lowered_text.starts_with(prefix) {
        return None;
    }

    let rest = original_text[prefix.len()..].trim_start();
    let number: String = rest
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    if number.is_empty() {
        None
    } else {
        Some(number)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::prior_memory::{
        ActorTaxonomyPriorRecord, ActorTaxonomyRole, CorpusMemory, CorpusMemoryUpdatePolicyRecord,
        PriorSourceArtifactRecord, ProtocolFamily, SemanticPhrasePriorRecord,
        VisualMotifPriorRecord,
    };
    use crate::ir::semantic::{InterfaceSignalSemanticRole, SemanticGroundingStrength};
    use crate::ir::source::{
        AutomationConfidence, DiagramKind, RelationKind, SectionKind, SourceIr,
        StructuredTableCellRecord, StructuredTableRecord, TableKind, VisualAsset, VisualAssetKind,
    };

    use super::{
        EvidenceIr, EvidenceLinkKind, EvidenceModality, ExtractorTier, FactKind,
        SignalSemanticHintSourceKind, SignalSemanticTag, StatementClass, VisualObservationKind,
        actor_signal_relation_fact_key, canonicalize_existing_path, contains_any,
        contains_reference_token, diagram_kind_key, is_abstract_transport_actor_term,
        is_abstract_transport_signal_token, is_hardware_signal_token, is_image_line,
        is_signal_name_char, is_signal_synthesis_non_signal, is_standalone_markdown_block,
        is_tie_off_actor_text, looks_like_encoding_literal,
        looks_like_structural_contents_entry_for_semantic_hint, numbered_list_prefix,
        parse_encoding_numeric_literal, signal_constraint_fact_key,
    };

    // ── PDF-VARIANT-DIGESTION.2 — flexible register-field-table extraction ───────────────────
    #[test]
    fn register_field_header_recognizer_is_specific() {
        let lower = |cols: &[&str]| {
            cols.iter()
                .map(|s| s.to_ascii_lowercase())
                .collect::<Vec<_>>()
        };
        // register-field tables across the surveyed shapes: a field/bits column + access OR reset
        assert!(super::is_register_field_header(&lower(&[
            "Field",
            "Description",
            "Access",
            "Reset"
        ])));
        assert!(super::is_register_field_header(&lower(&[
            "Bits",
            "Type",
            "Reset",
            "Description"
        ])));
        assert!(super::is_register_field_header(&lower(&[
            "Offset",
            "Bits",
            "Field name",
            "Description",
            "Attributes"
        ])));
        // signal / encoding / feature tables must NOT match (no access/reset pairing)
        assert!(!super::is_register_field_header(&lower(&[
            "Signal",
            "Source",
            "Width",
            "Description"
        ])));
        assert!(!super::is_register_field_header(&lower(&[
            "Value",
            "Description"
        ])));
        assert!(!super::is_register_field_header(&lower(&[
            "Feature",
            "Mandatory"
        ])));
    }

    #[test]
    fn register_name_from_caption_strips_label_and_boilerplate() {
        assert_eq!(
            super::register_name_from_caption("Table 6.1: DTMControl register fields").as_deref(),
            Some("DTMControl")
        );
        assert_eq!(
            super::register_name_from_caption("DMSTATUS Register").as_deref(),
            Some("DMSTATUS")
        );
        assert!(super::register_name_from_caption("   ").is_none());
    }

    #[test]
    fn register_field_table_header_in_body_extracts_fields() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let base = tempdir.path().join("generated").join("source_ir");
        fs::write(
            &source,
            "# Registers\nThe DTMControl register controls the DTM.\n",
        )?;
        let mut source_ir = SourceIr::build(&source, &base)?;
        // An UNKNOWN table whose column-title row Docling left in body_rows[0] (the common miss).
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_regfld".to_string(),
            asset_id: "asset_regfld".to_string(),
            page_id: None,
            caption_text: Some("Table 6.1: DTMControl register fields".to_string()),
            source_ref: None,
            table_kind: TableKind::Unknown,
            header_rows: vec![],
            body_rows: vec![
                vec![
                    make_table_cell("Field", false),
                    make_table_cell("Description", false),
                    make_table_cell("Access", false),
                    make_table_cell("Reset", false),
                ],
                vec![
                    make_table_cell("dmireset", false),
                    make_table_cell("Resets the DMI", false),
                    make_table_cell("R/W", false),
                    make_table_cell("0", false),
                ],
                vec![
                    make_table_cell("idle", false),
                    make_table_cell("Hint for idle cycles", false),
                    make_table_cell("R", false),
                    make_table_cell("0", false),
                ],
            ],
            row_count: 3,
            col_count: 4,
        });
        let recs = super::synthesize_register_field_tables(&source_ir, None);
        assert_eq!(
            recs.len(),
            1,
            "one register synthesized from the field table"
        );
        let reg = &recs[0];
        assert_eq!(reg.register_name, "DTMControl");
        let names: Vec<_> = reg.fields.iter().map(|f| f.field_name.as_str()).collect();
        assert_eq!(
            names,
            vec!["dmireset", "idle"],
            "field rows extracted, header-in-body skipped"
        );
        assert_eq!(reg.fields[0].access_type.as_deref(), Some("R/W"));
        assert_eq!(reg.fields[0].reset_value.as_deref(), Some("0"));
        assert_eq!(reg.fields[0].description.as_deref(), Some("Resets the DMI"));
        Ok(())
    }

    // PDF-VARIANT-DIGESTION.9.11 — a `timing_parameter` table whose row-label cell Docling marked
    // `is_header=true` traps every data row in `header_rows`, leaving `body_rows` empty (I2S `table_0004`,
    // SMBus `table_0012`). The structural recovery treats a `header_rows` entry past the column header
    // whose VALUE cells are `is_header=false` as a data row, so the parameters + MIN/TYP/MAX are recovered.
    #[test]
    fn timing_table_data_trapped_in_header_rows_is_recovered() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Timing\nTarget receiver timing.\n")?;
        let mut source_ir = SourceIr::build(&source, &base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_trapped".to_string(),
            asset_id: "asset_trapped".to_string(),
            page_id: None,
            caption_text: Some("Target receiver timing (all values in ns)".to_string()),
            source_ref: None,
            table_kind: TableKind::TimingParameter,
            // The real column header (blank leading cell; value labels are `is_header=true`) plus two
            // data rows whose label cell is `is_header=true` and whose value cells are `is_header=false`.
            header_rows: vec![
                vec![
                    make_table_cell("", false),
                    make_table_cell("MIN", true),
                    make_table_cell("TYP", true),
                    make_table_cell("MAX", true),
                ],
                vec![
                    make_table_cell("clock period T", true),
                    make_table_cell("360", false),
                    make_table_cell("400", false),
                    make_table_cell("440", false),
                ],
                vec![
                    make_table_cell("clock HIGH t HC", true),
                    make_table_cell("110", false),
                    make_table_cell("", false),
                    make_table_cell("", false),
                ],
            ],
            body_rows: vec![],
            row_count: 3,
            col_count: 4,
        });
        let recs = super::synthesize_timing_constraints(&source_ir, None);
        assert_eq!(recs.len(), 2, "both trapped data rows recovered");
        assert_eq!(recs[0].parameter_name, "clock period T");
        assert_eq!(recs[0].min_value.as_deref(), Some("360"));
        assert_eq!(recs[0].typ_value.as_deref(), Some("400"));
        assert_eq!(recs[0].max_value.as_deref(), Some("440"));
        assert_eq!(recs[1].parameter_name, "clock HIGH t HC");
        assert_eq!(recs[1].min_value.as_deref(), Some("110"));
        assert_eq!(
            recs[1].typ_value, None,
            "empty value cell stays None, not fabricated"
        );
        Ok(())
    }

    // PDF-VARIANT-DIGESTION.9.11 — a genuine multi-row COLUMN header (nested cross-tab, e.g. I2S
    // `table_0005`'s TRANSMITTER/RECEIVER × LOWER/UPPER) keeps its value cells `is_header=true`, so the
    // recovery must NOT mistake those header rows for data — the table stays an honest residual (0 records),
    // never a fabricated parameter named "TRANSMITTER".
    #[test]
    fn timing_table_nested_column_header_is_not_recovered() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Timing\nCross-tab timing.\n")?;
        let mut source_ir = SourceIr::build(&source, &base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_nested".to_string(),
            asset_id: "asset_nested".to_string(),
            page_id: None,
            caption_text: None,
            source_ref: None,
            table_kind: TableKind::TimingParameter,
            header_rows: vec![
                vec![
                    make_table_cell("", false),
                    make_table_cell("TRANSMITTER", true),
                    make_table_cell("RECEIVER", true),
                ],
                vec![
                    make_table_cell("", false),
                    make_table_cell("LOWER LIMIT", true),
                    make_table_cell("UPPER LIMIT", true),
                ],
            ],
            body_rows: vec![],
            row_count: 2,
            col_count: 3,
        });
        let recs = super::synthesize_timing_constraints(&source_ir, None);
        assert!(
            recs.is_empty(),
            "nested column header stays an honest residual, no fabricated parameters"
        );
        Ok(())
    }

    // PDF-VARIANT-DIGESTION.9.11 — a normal timing table (data in `body_rows`, labelled header) is
    // unchanged: the header-row recovery is purely additive and adds nothing here.
    #[test]
    fn timing_table_normal_body_rows_unchanged() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Timing\nBus timing.\n")?;
        let mut source_ir = SourceIr::build(&source, &base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_normal".to_string(),
            asset_id: "asset_normal".to_string(),
            page_id: None,
            caption_text: None,
            source_ref: None,
            table_kind: TableKind::TimingParameter,
            header_rows: vec![vec![
                make_table_cell("Symbol", true),
                make_table_cell("Min", true),
                make_table_cell("Max", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("t BUF", false),
                    make_table_cell("1.3", false),
                    make_table_cell("", false),
                ],
                vec![
                    make_table_cell("t HD", false),
                    make_table_cell("0.6", false),
                    make_table_cell("", false),
                ],
            ],
            row_count: 2,
            col_count: 3,
        });
        let recs = super::synthesize_timing_constraints(&source_ir, None);
        assert_eq!(recs.len(), 2, "both body rows extracted, nothing added");
        assert_eq!(recs[0].parameter_name, "t BUF");
        assert_eq!(recs[0].min_value.as_deref(), Some("1.3"));
        assert_eq!(recs[1].parameter_name, "t HD");
        Ok(())
    }

    #[test]
    fn register_field_table_bits_only_shape_extracts_fields() -> Result<()> {
        // `Bits|Type|Reset|Description` — no name column; the bit-range identifies the field, and
        // zero-padded ranges ("02:00") must parse. Type is the access column.
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Registers\nThe CONTROL register.\n")?;
        let mut source_ir = SourceIr::build(&source, &base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_bits".to_string(),
            asset_id: "asset_bits".to_string(),
            page_id: None,
            caption_text: Some("CONTROL".to_string()),
            source_ref: None,
            table_kind: TableKind::Unknown,
            header_rows: vec![vec![
                make_table_cell("Bits", true),
                make_table_cell("Type", true),
                make_table_cell("Reset", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("02:00", false),
                make_table_cell("WARL", false),
                make_table_cell("0", false),
                make_table_cell("Mode select", false),
            ]],
            row_count: 2,
            col_count: 4,
        });
        let recs = super::synthesize_register_field_tables(&source_ir, None);
        assert_eq!(recs.len(), 1);
        assert_eq!(
            recs[0].size_bits,
            Some(3),
            "register width = max field MSb + 1"
        );
        let f = &recs[0].fields[0];
        assert_eq!(f.field_name, "02:00");
        assert_eq!(
            (f.bits_high, f.bits_low),
            (Some(2), Some(0)),
            "zero-padded range parses"
        );
        assert_eq!(f.bit_width, Some(3), "field width = high - low + 1");
        assert_eq!(
            f.access_type.as_deref(),
            Some("WARL"),
            "free-form access kept"
        );
        Ok(())
    }

    #[test]
    fn register_field_bits_only_recovers_mnemonic_from_description() -> Result<()> {
        // EXTRACTION-GAP-FIX.2 — NVMe-style `Bits|Type|Reset|Description` table: the "name" column is a
        // bit-range, and the field MNEMONIC lives in the description's universal defined-term prefix
        // `Full Field Name (MNEMONIC): …`. The mnemonic must become the field identity while the bit
        // structure (from the Bits column) is preserved; a row with no defined term keeps the bit-range as
        // an honest residual (never fabricated). Prose lifted from the real NVMe Base Spec 2.0a (CAP).
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Registers\nThe CAP register.\n")?;
        let mut source_ir = SourceIr::build(&source, &base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_cap".to_string(),
            asset_id: "asset_cap".to_string(),
            page_id: None,
            caption_text: Some("Offset 0h: CAP".to_string()),
            source_ref: None,
            table_kind: TableKind::Unknown,
            header_rows: vec![vec![
                make_table_cell("Bits", true),
                make_table_cell("Type", true),
                make_table_cell("Reset", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("15:00", false),
                    make_table_cell("RO", false),
                    make_table_cell("Impl Spec", false),
                    make_table_cell(
                        "Maximum Queue Entries Supported (MQES): This field indicates the maximum \
                         individual queue size that the controller supports.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("44:37", false),
                    make_table_cell("RO", false),
                    make_table_cell("Impl Spec", false),
                    make_table_cell(
                        "Command Sets Supported (CSS): This field indicates the I/O Command Set(s) \
                         that the controller supports.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("31:24", false),
                    make_table_cell("RO", false),
                    make_table_cell("Impl Spec", false),
                    make_table_cell(
                        "Timeout (TO): worst-case time host software should wait.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("23:19", false),
                    make_table_cell("RO", false),
                    make_table_cell("0h", false),
                    // No defined-term prefix → mnemonic unrecoverable → bit-range stays a residual.
                    make_table_cell("Reserved.", false),
                ],
            ],
            row_count: 5,
            col_count: 4,
        });
        let recs = super::synthesize_register_field_tables(&source_ir, None);
        assert_eq!(
            recs.len(),
            1,
            "one register synthesized from the field table"
        );
        let reg = &recs[0];
        let names: Vec<_> = reg.fields.iter().map(|f| f.field_name.as_str()).collect();
        assert_eq!(
            names,
            vec!["MQES", "CSS", "TO", "23:19"],
            "mnemonics recovered from the description defined-term; the term-less row stays a residual"
        );
        assert_eq!(
            (reg.fields[0].bits_high, reg.fields[0].bits_low),
            (Some(15), Some(0)),
            "bit structure preserved (recovery only renames the field)"
        );
        assert_eq!(
            (reg.fields[1].bits_high, reg.fields[1].bits_low),
            (Some(44), Some(37)),
        );
        Ok(())
    }

    #[test]
    fn field_mnemonic_recovery_is_grammar_only() {
        // EXTRACTION-GAP-FIX.2 — the recovery is pure defined-term grammar (ADR 0006): it requires a
        // parenthesized uppercase token IMMEDIATELY followed by a colon, and never fabricates.
        assert_eq!(
            super::field_mnemonic_from_description("Timeout (TO): worst case"),
            Some("TO".to_string())
        );
        assert_eq!(
            super::field_mnemonic_from_description(
                "Region is not supported. Memory Page Size Maximum (MPSMAX): the max page size"
            ),
            Some("MPSMAX".to_string()),
            "leading cell-bleed before the defined term is tolerated"
        );
        // a passing reference `(X)` not followed by a colon is not the field's own definition
        assert_eq!(
            super::field_mnemonic_from_description("set per the value in (CC.MPS) and others"),
            None
        );
        // lowercase / multi-word parentheticals and bare numbers are not mnemonics
        assert_eq!(
            super::field_mnemonic_from_description("see note (1): refer to the table"),
            None
        );
        assert_eq!(
            super::field_mnemonic_from_description("the field (see below): description"),
            None
        );
        assert_eq!(super::field_mnemonic_from_description("Reserved."), None);
        // is_bit_range_token separates a bit spec from a real token
        assert!(super::is_bit_range_token("15:00"));
        assert!(super::is_bit_range_token("58"));
        assert!(super::is_bit_range_token("[7:0]"));
        assert!(!super::is_bit_range_token("MQES"));
        assert!(!super::is_bit_range_token("EN0"));
    }

    #[test]
    fn register_name_from_heading_is_grammar_only() {
        // EXTRACTION-GAP-FIX.3 — a register-defining heading names its register in a parenthetical bound to
        // the register's hex address; the required address is what gates against fabricating a name from an
        // arbitrary prose parenthetical (ADR 0006). Headings lifted from the real RISC-V Debug spec.
        assert_eq!(
            super::register_name_from_heading("3.14.1. Debug Module Status (dmstatus, at 0x11)"),
            Some("dmstatus".to_string())
        );
        assert_eq!(
            super::register_name_from_heading("3.14.3. Hart Info (hartinfo, at 0x12)"),
            Some("hartinfo".to_string())
        );
        // no hex address in the parenthetical → not a register-definition heading → no name minted
        assert_eq!(
            super::register_name_from_heading("3.2. Overview (see Section 3 for details)"),
            None
        );
        assert_eq!(super::register_name_from_heading("Introduction"), None);
        // an address with no leading identifier in its parenthetical does not fabricate one
        assert_eq!(
            super::register_name_from_heading("Memory map (region at 0x4000)"),
            Some("region".to_string()),
            "the leading identifier of the address-bearing parenthetical is the name"
        );
    }

    #[test]
    fn register_name_recovered_from_defining_section_heading() -> Result<()> {
        // EXTRACTION-GAP-FIX.3 — a caption-less `Field|Description|Access|Reset` register table (RISC-V
        // shape) gets its register name from the nearest preceding section heading that DEFINES a register
        // (`<Title> (<name>, at 0x..)`), via page-based association — not a synthetic `register_<table_id>`.
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Debug\nThe Debug Module.\n")?;
        let mut source_ir = SourceIr::build(&source, &base)?;
        source_ir
            .document_sections
            .push(crate::ir::source::ContentSectionRecord {
                section_id: "section_dmstatus".to_string(),
                title: "3.14.1. Debug Module Status (dmstatus, at 0x11)".to_string(),
                heading_level: 3,
                page_id: Some("page_0033".to_string()),
                source_ref: None,
                reading_order: 1,
                section_kind: SectionKind::Unknown,
            });
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_0021".to_string(),
            asset_id: "asset_0021".to_string(),
            page_id: Some("page_0033".to_string()),
            caption_text: None, // RISC-V field tables carry no caption — the name is in the heading
            source_ref: None,
            table_kind: TableKind::Unknown,
            header_rows: vec![vec![
                make_table_cell("Field", true),
                make_table_cell("Description", true),
                make_table_cell("Access", true),
                make_table_cell("Reset", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("allhalted", false),
                make_table_cell("All harts are halted", false),
                make_table_cell("R", false),
                make_table_cell("0", false),
            ]],
            row_count: 2,
            col_count: 4,
        });
        let recs = super::synthesize_register_field_tables(&source_ir, None);
        assert_eq!(recs.len(), 1);
        assert_eq!(
            recs[0].register_name, "dmstatus",
            "register name recovered from the defining heading, not synthetic register_<table_id>"
        );

        // Negative: with a NON-defining heading (no register address) the name stays synthetic — no
        // fabrication (honesty guardrail).
        let mut src2 = SourceIr::build(&source, &base)?;
        src2.document_sections
            .push(crate::ir::source::ContentSectionRecord {
                section_id: "section_overview".to_string(),
                title: "3.2. Register Overview".to_string(),
                heading_level: 2,
                page_id: Some("page_0033".to_string()),
                source_ref: None,
                reading_order: 1,
                section_kind: SectionKind::Unknown,
            });
        src2.structured_tables
            .push(source_ir.structured_tables[0].clone());
        let recs2 = super::synthesize_register_field_tables(&src2, None);
        assert_eq!(recs2.len(), 1);
        assert!(
            recs2[0].register_name.starts_with("register_table_"),
            "no register-definition heading → synthetic name kept, not fabricated (got {:?})",
            recs2[0].register_name
        );
        Ok(())
    }

    #[test]
    fn inline_field_enums_parse_only_value_literals() {
        // binary/hex/Verilog literals become enums; bare integers / bit references / prose do not.
        let e = super::parse_inline_field_enums("0b00: Idle, 0b01: Busy; 0x2 = Error");
        let pairs: Vec<_> = e
            .iter()
            .map(|v| (v.value.as_str(), v.meaning.as_str()))
            .collect();
        assert_eq!(
            pairs,
            vec![("0b00", "Idle"), ("0b01", "Busy"), ("0x2", "Error")]
        );
        assert!(super::parse_inline_field_enums("0: disabled, 1: enabled").is_empty());
        assert!(super::parse_inline_field_enums("set bit 0 to enable the channel").is_empty());
        assert!(super::is_register_value_literal("2'b01"));
        assert!(!super::is_register_value_literal("0"));
    }

    #[test]
    fn bit_layout_grids_are_detected_as_garbage() {
        use crate::ir::source::{RegisterFieldRecord, RegisterRecord};
        let mk = |name: &str| RegisterFieldRecord {
            field_name: name.to_string(),
            bits_high: None,
            bits_low: None,
            bit_width: None,
            access_type: None,
            reset_value: None,
            description: None,
            enumerated_values: vec![],
        };
        let reg = |fields: Vec<RegisterFieldRecord>| RegisterRecord {
            register_id: "r".to_string(),
            register_name: "R".to_string(),
            offset_address: None,
            size_bits: None,
            fields,
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        };
        // all-numeric field names = a bit-position grid (garbage)
        assert!(super::register_is_bit_layout_grid(&reg(vec![
            mk("14"),
            mk("13"),
            mk("12")
        ])));
        // a real register keeps at least one named field
        assert!(!super::register_is_bit_layout_grid(&reg(vec![
            mk("14"),
            mk("haltreq")
        ])));
        // bit-RANGE names ("63:61") are not pure numbers → a legitimate bits-only field table
        assert!(!super::register_is_bit_layout_grid(&reg(vec![mk("63:61")])));
        // an empty register is not a grid
        assert!(!super::register_is_bit_layout_grid(&reg(vec![])));
    }

    #[test]
    fn noise_tables_detected_data_tables_not() {
        let mk = |rows: Vec<Vec<&str>>, cap: Option<&str>| StructuredTableRecord {
            table_id: "t".to_string(),
            asset_id: "a".to_string(),
            page_id: None,
            caption_text: cap.map(String::from),
            source_ref: None,
            table_kind: TableKind::Unknown,
            header_rows: vec![],
            body_rows: rows
                .iter()
                .map(|r| r.iter().map(|c| make_table_cell(c, false)).collect())
                .collect(),
            row_count: rows.len() as u32,
            col_count: rows.first().map(|r| r.len() as u32).unwrap_or(0),
        };
        // dotted page-leaders (table of contents)
        assert!(super::table_is_noise(&mk(
            vec![vec!["Preface........1"], vec!["1.1. Scope.....8"]],
            None
        )));
        // section-index rows
        assert!(super::table_is_noise(&mk(
            vec![
                vec!["1.1.", "Scope"],
                vec!["1.2.", "Terms"],
                vec!["5.3.", "Setup"]
            ],
            None
        )));
        // revision history via caption
        assert!(super::table_is_noise(&mk(
            vec![vec!["1.0", "First release"]],
            Some("Revision History")
        )));
        // a real register-field table is NOT noise
        assert!(!super::table_is_noise(&mk(
            vec![
                vec!["Field", "Description", "Access", "Reset"],
                vec!["haltreq", "halt the hart", "R/W", "0"],
            ],
            None
        )));
    }

    #[test]
    fn register_field_extraction_ignores_signal_tables() -> Result<()> {
        // A signal-description table must NOT be mis-read as a register-field table.
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Signals\nSignal PCLK.\n")?;
        let mut source_ir = SourceIr::build(&source, &base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_sig".to_string(),
            asset_id: "asset_sig".to_string(),
            page_id: None,
            caption_text: Some("Signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Width", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("PCLK", false),
                make_table_cell("1", false),
                make_table_cell("Clock", false),
            ]],
            row_count: 2,
            col_count: 3,
        });
        assert!(super::synthesize_register_field_tables(&source_ir, None).is_empty());
        Ok(())
    }

    fn make_table_cell(text: &str, is_header: bool) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        }
    }

    fn write_actor_taxonomy_prior_memory(root: &Path) -> Result<PathBuf> {
        write_prior_memory(root, |corpus_memory| {
            corpus_memory.actor_taxonomy_priors = vec![
                ActorTaxonomyPriorRecord {
                    prior_id: "actor_taxonomy_prior_0001".to_string(),
                    normalized_actor_term: "producer".to_string(),
                    taxonomy_role: ActorTaxonomyRole::RequesterLike,
                    protocol_family: ProtocolFamily::AmbaGeneric,
                    support_count: 3,
                    supporting_document_keys: vec!["fixture".to_string()],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::CrossModality,
                },
                ActorTaxonomyPriorRecord {
                    prior_id: "actor_taxonomy_prior_0002".to_string(),
                    normalized_actor_term: "consumer".to_string(),
                    taxonomy_role: ActorTaxonomyRole::CompleterLike,
                    protocol_family: ProtocolFamily::AmbaGeneric,
                    support_count: 3,
                    supporting_document_keys: vec!["fixture".to_string()],
                    strongest_automation_confidence: AutomationConfidence::High,
                    strongest_grounding_strength: SemanticGroundingStrength::CrossModality,
                },
            ];
        })
    }

    fn write_semantic_phrase_prior_memory(root: &Path) -> Result<PathBuf> {
        write_prior_memory(root, |corpus_memory| {
            corpus_memory.semantic_phrase_priors = vec![SemanticPhrasePriorRecord {
                prior_id: "semantic_phrase_prior_0001".to_string(),
                normalized_phrase: "<signal> can receive the transfer".to_string(),
                role: InterfaceSignalSemanticRole::HandshakeReadyLike,
                protocol_family: ProtocolFamily::AmbaGeneric,
                source_kind: SignalSemanticHintSourceKind::ProseStatement,
                support_count: 2,
                supporting_document_keys: vec!["fixture".to_string()],
                strongest_automation_confidence: AutomationConfidence::High,
                strongest_grounding_strength: SemanticGroundingStrength::SingleSource,
            }];
        })
    }

    fn write_visual_motif_prior_memory(root: &Path) -> Result<PathBuf> {
        write_prior_memory(root, |corpus_memory| {
            corpus_memory.visual_motif_priors = vec![VisualMotifPriorRecord {
                prior_id: "visual_motif_prior_0001".to_string(),
                normalized_caption_phrase: Some("<signal> cycle trace".to_string()),
                diagram_kind: DiagramKind::TimingDiagram,
                asset_kind: VisualAssetKind::Diagram,
                protocol_family: ProtocolFamily::AmbaGeneric,
                support_count: 2,
                supporting_document_keys: vec!["fixture".to_string()],
                strongest_automation_confidence: AutomationConfidence::High,
            }];
        })
    }

    fn write_prior_memory(
        root: &Path,
        populate: impl FnOnce(&mut CorpusMemory),
    ) -> Result<PathBuf> {
        let prior_memory_path = root
            .join("generated")
            .join("prior_memory")
            .join("corpus_memory.json");
        if let Some(parent) = prior_memory_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut corpus_memory = CorpusMemory {
            schema_version: 6,
            update_policy: CorpusMemoryUpdatePolicyRecord {
                advisory_only: true,
                requires_validated_intent_ir: true,
                rejects_error_findings: true,
                excludes_alias_dependent_semantic_consensus: true,
                local_grounding_required_for_canonical_promotion: true,
            },
            source_artifacts: vec![PriorSourceArtifactRecord {
                artifact_path: root.join("fixture_intent_ir.json"),
                document_key: "fixture".to_string(),
                display_name: "Fixture".to_string(),
                protocol_family: ProtocolFamily::AmbaGeneric,
                overall_score: Some(100),
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
            negative_knowledge_priors: Vec::new(),
            extraction_profile_priors: Vec::new(),
        };
        populate(&mut corpus_memory);
        fs::write(
            &prior_memory_path,
            serde_json::to_string_pretty(&corpus_memory)?,
        )?;

        Ok(prior_memory_path)
    }

    #[test]
    fn builds_evidence_ir_from_markdown_source_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            "# Rules\nVALID must stay asserted until handshake.\n\nREADY may deassert while idle.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert_eq!(evidence_ir.stage.as_str(), "evidence_ir");
        assert_eq!(evidence_ir.section_anchors.len(), 1);
        assert_eq!(evidence_ir.evidence_spans.len(), 2);
        assert_eq!(evidence_ir.extracted_statements.len(), 2);
        assert!(evidence_ir.visual_evidence.is_empty());
        assert!(evidence_ir.evidence_links.is_empty());
        // "VALID must stay asserted" contains "must " → now correctly classified as
        // NormativeStatement (a behavioral requirement), not generic SourceFact.
        assert_eq!(
            evidence_ir.extracted_statements[0].class,
            StatementClass::NormativeStatement
        );
        assert_eq!(
            evidence_ir.extracted_statements[0].modality,
            EvidenceModality::Text
        );
        assert!(
            evidence_ir.extracted_statements[0]
                .related_visual_evidence_ids
                .is_empty()
        );
        assert_eq!(
            evidence_ir.extracted_statements[1].modality,
            EvidenceModality::Text
        );
        assert!(
            evidence_ir.extracted_statements[1]
                .related_visual_evidence_ids
                .is_empty()
        );

        Ok(())
    }

    #[test]
    fn vlm_enrichment_note_becomes_timing_diagram_observation() -> Result<()> {
        // Tests the VLM wiring chain:
        //   VisualAsset.note = "vlm_timing_diagram_extraction: {json}"
        //     → EvidenceIr.visual_evidence[i].observations contains TimingDiagramExtraction
        //     → (SemanticIR test separately verifies it parses into TimingConstraintRecord)
        let tempdir = tempdir()?;
        let source = tempdir.path().join("diagram_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

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
            // Simulate what `specforge enrich --vlm-provider ollama` would write.
            note: Some(
                "vlm_timing_diagram_extraction: {\"signals\":[{\"name\":\"HCLK\",\"values\":[{\"cycle\":\"T1\",\"state\":\"HIGH\"}]}],\"annotations\":[\"Address phase: T1-T2\",\"tSU = 2 ns\"]}"
                    .to_string(),
            ),
            diagram_kind: crate::ir::source::DiagramKind::TimingDiagram,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        // The VLM note must produce a TimingDiagramExtraction observation.
        assert_eq!(evidence_ir.visual_evidence.len(), 1);
        assert!(
            evidence_ir.visual_evidence[0].observations.iter().any(|o| {
                matches!(o.kind, VisualObservationKind::TimingDiagramExtraction)
                    && o.text.contains("Address phase")
                    && o.created_by == "specforge_vlm_enrich"
                    && o.automation_confidence == AutomationConfidence::High
            }),
            "expected TimingDiagramExtraction observation from VLM note"
        );
        // The figure's role should be upgraded to Normative.
        assert_eq!(
            evidence_ir.visual_evidence[0].role,
            super::VisualEvidenceRole::Normative
        );
        assert_eq!(
            evidence_ir.visual_evidence[0].automation_confidence,
            AutomationConfidence::High
        );

        Ok(())
    }

    /// Unit tests for Level 1 and Level 2 NLP classification and extraction.
    /// These lock in the expanded vocabulary so regressions are caught immediately.
    mod nlp_classification {
        use super::super::{
            StatementClass, classify_statement, collect_subject_signal_tokens,
            extract_protocol_state_value, is_signal_value_constraint, text_before_condition_marker,
        };

        #[test]
        fn constraint_value_is_derived_positionally_not_from_a_hardcoded_list() {
            // ADR 0006: the value is whatever the document places after the normative
            // verb, never matched against a baked-in vocabulary. A value the code has
            // never seen (not AMBA, not any known protocol) is still extracted.
            assert_eq!(
                extract_protocol_state_value("psel must be foobarbaz"),
                Some("FOOBARBAZ".to_string())
            );
            // Known protocol values still work — now via grammar position, not a list.
            assert_eq!(
                extract_protocol_state_value("htrans must be nonseq"),
                Some("NONSEQ".to_string())
            );
            // Binding fillers ("set to") are skipped to reach the real value.
            assert_eq!(
                extract_protocol_state_value("x must be set to active"),
                Some("ACTIVE".to_string())
            );
            // No normative binding phrase → no value.
            assert_eq!(extract_protocol_state_value("the bus is idle"), None);
        }

        // ── Level 1: NormativeStatement new vocabulary ──────────────────────

        #[test]
        fn cannot_classifies_as_normative_statement() {
            // Note: "HTRANS cannot change" hits SignalValueConstraint (more specific — correct).
            // This test uses a sentence with no value-binding phrase to isolate the
            // NormativeStatement path triggered by "cannot ".
            assert_eq!(
                classify_statement("Transfers cannot overlap with outstanding error responses"),
                StatementClass::NormativeStatement
            );
        }

        #[test]
        fn is_not_permitted_classifies_as_normative_statement() {
            assert_eq!(
                classify_statement("Early termination is not permitted on locked transfers"),
                StatementClass::NormativeStatement
            );
        }

        #[test]
        fn may_not_classifies_as_normative_statement() {
            assert_eq!(
                classify_statement("HMASTER may not change while HMASTLOCK is asserted"),
                StatementClass::NormativeStatement
            );
        }

        #[test]
        fn will_not_without_condition_classifies_as_normative_statement() {
            // "HADDR will not change" → SignalValueConstraint (correct, more specific).
            // Use a sentence with no value-binding phrase to isolate the "will not " trigger.
            assert_eq!(
                classify_statement("The response will not indicate an OKAY during error states"),
                StatementClass::NormativeStatement
            );
        }

        // ── Level 1: SignalValueConstraint new vocabulary ────────────────────

        #[test]
        fn is_tied_high_is_signal_value_constraint() {
            // Very common in AHB specs: "HWRITE is tied HIGH for the entire burst".
            assert!(is_signal_value_constraint(
                "HWRITE is tied HIGH for the entire burst"
            ));
        }

        #[test]
        fn is_held_stable_is_signal_value_constraint() {
            assert!(is_signal_value_constraint(
                "HWDATA is held stable throughout the data phase"
            ));
        }

        #[test]
        fn cannot_change_is_signal_value_constraint() {
            assert!(is_signal_value_constraint(
                "HTRANS cannot change during a waited transfer"
            ));
        }

        #[test]
        fn remains_stable_is_signal_value_constraint() {
            assert!(is_signal_value_constraint(
                "HADDR remains stable throughout the burst"
            ));
        }

        // ── Level 1: ConditionalRule new vocabulary ──────────────────────────

        #[test]
        fn unless_conditional_classifies_as_conditional_rule() {
            assert_eq!(
                classify_statement("HTRANS must remain NONSEQ unless HREADY is asserted"),
                StatementClass::ConditionalRule
            );
        }

        #[test]
        fn provided_that_classifies_as_conditional_rule() {
            // "HADDR shall be valid" → SignalValueConstraint (correct, more specific).
            // Use a sentence whose consequent has no value-binding phrase.
            assert_eq!(
                classify_statement(
                    "The transfer shall proceed provided that the address phase completes"
                ),
                StatementClass::ConditionalRule
            );
        }

        #[test]
        fn before_with_must_classifies_as_conditional_rule() {
            // "HREADY must be asserted" → SignalValueConstraint (correct, more specific).
            // Use a sentence whose consequent has no value-binding phrase.
            assert_eq!(
                classify_statement(
                    "Before the transfer phase, the decoder must enable the peripheral select"
                ),
                StatementClass::ConditionalRule
            );
        }

        // ── Level 1: TimingConstraint new vocabulary ─────────────────────────

        #[test]
        fn rising_edge_classifies_as_timing_constraint() {
            // "HADDR must be stable" → SignalValueConstraint (correct, more specific).
            // Use a sentence where the rising edge IS the timing parameter, not the condition.
            assert_eq!(
                classify_statement(
                    "HCLK must have a rising edge period of at least one nanosecond"
                ),
                StatementClass::TimingConstraint
            );
        }

        // ── Level 2: multi-signal extraction ────────────────────────────────

        #[test]
        fn collect_subject_signal_tokens_finds_all_signals_before_condition() {
            // "Both HTRANS and HADDR shall be stable" → [HTRANS, HADDR]
            // The condition clause stripping is not applied here (no condition marker).
            let signals = collect_subject_signal_tokens("Both HTRANS and HADDR shall be stable");
            assert!(signals.contains(&"HTRANS".to_string()), "expected HTRANS");
            assert!(signals.contains(&"HADDR".to_string()), "expected HADDR");
        }

        #[test]
        fn text_before_condition_marker_strips_when_clause() {
            let pre = text_before_condition_marker("HTRANS must remain NONSEQ when HREADY is LOW");
            assert!(pre.contains("HTRANS"), "subject part should include HTRANS");
            assert!(
                !pre.contains("HREADY"),
                "condition-clause signal HREADY should be stripped"
            );
        }

        #[test]
        fn constraint_value_names_are_excluded_positionally_not_by_a_denylist() {
            // A value (IDLE/HIGH/any) must never be a subject signal — but it is now
            // excluded *positionally* (the token after the normative verb) in
            // `extract_signal_constraints`, NOT by a hardcoded value denylist, so no
            // value vocabulary is baked into the code (ADR 0006).
            let s = constraint_subjects("HTRANS must be IDLE");
            assert!(
                s.contains(&"HTRANS".to_string()),
                "HTRANS is the subject; got {s:?}"
            );
            assert!(
                !s.contains(&"IDLE".to_string()),
                "IDLE is the value, not a subject; got {s:?}"
            );
            let s2 = constraint_subjects("PSEL must be HIGH");
            assert!(
                s2.contains(&"PSEL".to_string()),
                "PSEL is the subject; got {s2:?}"
            );
            assert!(
                !s2.contains(&"HIGH".to_string()),
                "HIGH is the value, not a subject; got {s2:?}"
            );
        }

        #[test]
        fn logic_level_value_not_a_subject_without_must_be() {
            // CONSTRAINT-EXTRACTION-V2.1: the real APB construction "driven LOW when … are
            // LOW" produced a bogus "LOW must be stable" — LOW is a universal logic-level
            // value, never a subject. The positional "must be <value>" exclusion does not
            // fire here (no "must be"), so a logic-level word is excluded as a value.
            let s = constraint_subjects(
                "It is recommended that PSLVERR is driven LOW when PSEL, PENABLE, or PREADY are LOW",
            );
            assert!(
                !s.contains(&"LOW".to_string()),
                "LOW is a logic-level value, never a subject; got {s:?}"
            );
        }

        #[test]
        fn inference_antecedent_signal_is_not_the_obligation_subject() {
            // CONSTRAINT-EXTRACTION-V2.3: "PSEL is asserted, which means PADDR/PWRITE/PWDATA
            // must be valid" — PSEL is the antecedent (trigger), not the subject; the
            // obligation is on the consequent signals.
            let s = constraint_subjects(
                "The select signal, PSEL, is asserted, which means that PADDR, PWRITE, and PWDATA must be valid",
            );
            assert!(
                !s.contains(&"PSEL".to_string()),
                "PSEL is the antecedent trigger, not the obligation subject; got {s:?}"
            );
            assert!(
                s.contains(&"PADDR".to_string())
                    || s.contains(&"PWRITE".to_string())
                    || s.contains(&"PWDATA".to_string()),
                "the consequent signals are the subjects; got {s:?}"
            );
        }

        #[test]
        fn multi_bullet_table_cell_condition_does_not_bleed() {
            // CONSTRAINT-EXTRACTION-V2.2: a multi-obligation table cell must not carry a later
            // bullet into one constraint's condition. The first obligation's condition is just
            // "when PSELx is asserted", not the whole rest of the cell.
            use crate::ir::evidence::{
                EvidenceModality, ExtractedStatement, StatementClass, extract_signal_constraints,
            };
            let cell = "User-defined request attribute. • PAUSER must be valid when PSELx is asserted. • PAUSER must have the same value in the Setup and Access phase of a transfer.";
            let stmt = ExtractedStatement {
                statement_id: "s".to_string(),
                text: cell.to_string(),
                class: StatementClass::SignalValueConstraint,
                modality: EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            };
            let mut counter = 0usize;
            let records = extract_signal_constraints(&[stmt], &mut counter);
            assert!(
                records.iter().any(|r| r.subject_signal == "PAUSER"),
                "PAUSER extracted; got {records:?}"
            );
            for r in &records {
                if let Some(cond) = &r.condition_text {
                    let lc = cond.to_ascii_lowercase();
                    assert!(
                        !lc.contains("must have") && !cond.contains('•'),
                        "condition must not bleed into the next bullet; got {cond:?}"
                    );
                }
            }
        }

        #[test]
        fn drive_signal_level_binds_must_be_low_but_gates_false_positives() {
            // The active "drive <signal> LOW" construction → a value constraint (the dynamic
            // tier's discovered-value path misses it; logic levels are the universal "how").
            // Gated: a distant condition level, a `set`-in-`reset` substring, and `active low`
            // polarity must NOT bind (CONSTRAINT-EXTRACTION-V2 / drive-level recall).
            use crate::ir::evidence::{
                EvidenceModality, ExtractedStatement, StatementClass,
                extract_dynamic_signal_constraints,
            };
            use crate::ir::source::SignalConstraintKind;
            let mk = |id: &str, t: &str| ExtractedStatement {
                statement_id: id.to_string(),
                text: t.to_string(),
                class: StatementClass::NormativeStatement,
                modality: EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            };
            let stmts = vec![
                mk(
                    "s1",
                    "For read transfers, the Requester must drive all bits of PSTRB LOW.",
                ),
                mk(
                    "s2",
                    "Check signals are synchronous to PCLK and must be driven correctly every cycle in which the Check Enable term is True.",
                ),
                mk("s3", "Reset PRESETN is asynchronous active low."),
            ];
            let discovered: std::collections::HashSet<String> = std::collections::HashSet::new();
            let mut counter = 0usize;
            let recs = extract_dynamic_signal_constraints(&stmts, &mut counter, &discovered);
            assert!(
                recs.iter().any(|r| r.subject_signal == "PSTRB"
                    && matches!(r.constraint_kind, SignalConstraintKind::MustBeLow)),
                "drive PSTRB LOW -> PSTRB must_be_low; got {recs:?}"
            );
            assert!(
                !recs.iter().any(|r| r.subject_signal == "PCLK"),
                "PCLK 'True' is a distant condition, not a binding; got {recs:?}"
            );
            assert!(
                !recs.iter().any(|r| r.subject_signal.starts_with("PRESET")),
                "active-low polarity / 'set'-in-'reset' must not bind; got {recs:?}"
            );
        }

        // ── CONSTRAINT-SUBJECT-PRECISION: the 3 over-extraction classes the
        //    LLM-EXTRACTION-EVAL harness caught on the APB seed. ──────────────
        fn constraint_subjects(text: &str) -> Vec<String> {
            use crate::ir::evidence::{
                EvidenceModality, ExtractedStatement, StatementClass, extract_signal_constraints,
            };
            let stmt = ExtractedStatement {
                statement_id: "s".to_string(),
                text: text.to_string(),
                class: StatementClass::SignalValueConstraint,
                modality: EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            };
            let mut counter = 0usize;
            extract_signal_constraints(&[stmt], &mut counter)
                .into_iter()
                .map(|r| r.subject_signal)
                .collect()
        }

        #[test]
        fn constraint_subject_excludes_until_and_if_condition_signals() {
            // statement_0322: subject is PWAKEUP; PREADY ("until PREADY…") and PSEL
            // ("if PWAKEUP and PSELx are HIGH") are condition-clause signals, not subjects.
            let s = constraint_subjects(
                "PWAKEUP must remain asserted until PREADY is asserted if PWAKEUP and PSELx are HIGH in the same cycle.",
            );
            assert!(
                s.contains(&"PWAKEUP".to_string()),
                "PWAKEUP is the subject; got {s:?}"
            );
            assert!(
                !s.contains(&"PREADY".to_string()),
                "PREADY is in an 'until' clause; got {s:?}"
            );
            assert!(
                !s.contains(&"PSEL".to_string()),
                "PSEL is in an 'if' clause; got {s:?}"
            );
        }

        #[test]
        fn constraint_subject_excludes_width_parameter_tokens() {
            // statement_0339 (table row): PBUSER is the subject; USER_RESP_WIDTH is the
            // width-column parameter, not a constrained signal.
            let s = constraint_subjects(
                "| PBUSER | USER_RESP_WIDTH | Completer | User-defined response attribute. PBUSER must be valid when PSEL, PENABLE, and PREADY are asserted. |",
            );
            assert!(
                s.contains(&"PBUSER".to_string()),
                "PBUSER is the subject; got {s:?}"
            );
            assert!(
                !s.contains(&"USER_RESP_WIDTH".to_string()),
                "width param must not be a subject; got {s:?}"
            );
        }

        #[test]
        fn constraint_subject_does_not_sweep_other_sentence_signals() {
            // statement_0202: only PADDR/PWDATA are "must be stable"; PENABLE/PREADY/PCLK
            // live in earlier sentences and must not be swept into the stability clause.
            let s = constraint_subjects(
                "The Access phase is shown at T2 where PENABLE is asserted. PREADY is asserted by the Completer at the rising edge of PCLK. PADDR, PWDATA, and any other control signals, must be stable until the transfer completes.",
            );
            assert!(
                s.contains(&"PADDR".to_string()),
                "PADDR is a subject; got {s:?}"
            );
            assert!(
                s.contains(&"PWDATA".to_string()),
                "PWDATA is a subject; got {s:?}"
            );
            assert!(
                !s.contains(&"PCLK".to_string()),
                "PCLK (clock, other sentence) is not a subject; got {s:?}"
            );
            assert!(
                !s.contains(&"PENABLE".to_string()),
                "PENABLE (other sentence) is not a subject; got {s:?}"
            );
            assert!(
                !s.contains(&"PREADY".to_string()),
                "PREADY (other sentence) is not a subject; got {s:?}"
            );
        }

        #[test]
        fn constraint_subject_excludes_condition_signal_on_forward_reference() {
            // "The following signals must be valid when PSEL is asserted:" — the real
            // subject is a forward-referenced list the extractor cannot resolve; PSEL
            // is the CONDITION, not the subject. The empty-subject full-text fallback
            // must not grab it (CONSTRAINT-CONDITION-SUBJECT; real-APB NLI finding).
            let s =
                constraint_subjects("The following signals must be valid when PSEL is asserted:");
            assert!(
                !s.contains(&"PSEL".to_string()),
                "PSEL is the condition, not the subject; got {s:?}"
            );
            // A normal conditional constraint still resolves its real subject.
            let s2 = constraint_subjects("PADDR must be stable when HREADY is LOW.");
            assert!(
                s2.contains(&"PADDR".to_string()),
                "PADDR is the subject; got {s2:?}"
            );
            assert!(
                !s2.contains(&"HREADY".to_string()),
                "HREADY is the condition; got {s2:?}"
            );
        }
    }

    // ── Tier 2 Knowledge Graph: actor–signal relation extraction ────────────

    #[test]
    fn passive_drive_pattern_extracts_actor_and_signal() {
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };

        let signals = ["PREADY".to_string()]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let stmts = vec![ExtractedStatement {
            statement_id: "s1".to_string(),
            text: "PREADY is driven by the slave.".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];
        let relations = extract_actor_signal_relations(&stmts, &signals);
        assert!(
            relations.iter().any(|r| r.signal_name == "PREADY"
                && matches!(r.relation, RelationKind::Drives)
                && r.actor_name == "slave"),
            "passive 'is driven by' must extract (slave, Drives, PREADY), got: {:?}",
            relations
        );
    }

    #[test]
    fn active_drive_pattern_extracts_actor_and_signal() {
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };

        let signals = ["HTRANS".to_string()]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let stmts = vec![ExtractedStatement {
            statement_id: "s2".to_string(),
            text: "The Manager drives HTRANS to indicate the transfer type.".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];
        let relations = extract_actor_signal_relations(&stmts, &signals);
        assert!(
            relations.iter().any(|r| r.signal_name == "HTRANS"
                && matches!(r.relation, RelationKind::Drives)
                && r.actor_name == "Manager"),
            "active 'drives SIGNAL' must extract (Manager, Drives, HTRANS), got: {:?}",
            relations
        );
    }

    #[test]
    fn corpus_mined_actor_verbs_extract_relations() {
        // VERB-COVERAGE-CORPUS.2: corpus-mined actor→signal verbs — an actor that
        // changes a signal is a Drives relation; one that observes it is Reads.
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };
        let signals = [
            "PSTRB".to_string(),
            "PWAKE".to_string(),
            "PWUSER".to_string(),
        ]
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
        let mk = |id: &str, t: &str| ExtractedStatement {
            statement_id: id.to_string(),
            text: t.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        };
        let stmts = vec![
            mk("s1", "The Manager clears PSTRB before the read."),
            mk("s2", "The Manager polls PWAKE every cycle."),
            // passive form of a mined verb (the gap centralization closed)
            mk(
                "s3",
                "PWUSER is masked by the Manager during the idle phase.",
            ),
        ];
        let r = extract_actor_signal_relations(&stmts, &signals);
        assert!(
            r.iter().any(|x| x.actor_name == "Manager"
                && matches!(x.relation, RelationKind::Drives)
                && x.signal_name == "PSTRB"),
            "actor 'clears SIGNAL' -> Drives; got {r:?}"
        );
        assert!(
            r.iter().any(|x| x.actor_name == "Manager"
                && matches!(x.relation, RelationKind::Reads)
                && x.signal_name == "PWAKE"),
            "actor 'polls SIGNAL' -> Reads; got {r:?}"
        );
        assert!(
            r.iter()
                .any(|x| matches!(x.relation, RelationKind::Drives) && x.signal_name == "PWUSER"),
            "passive 'SIGNAL is masked by ACTOR' -> Drives; got {r:?}"
        );
    }

    #[test]
    fn passive_read_pattern_extracts_actor_and_signal() {
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };

        let signals = ["HREADY".to_string()]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let stmts = vec![ExtractedStatement {
            statement_id: "s3".to_string(),
            text: "HREADY is sampled by the Manager on every rising edge.".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];
        let relations = extract_actor_signal_relations(&stmts, &signals);
        assert!(
            relations.iter().any(|r| r.signal_name == "HREADY"
                && matches!(r.relation, RelationKind::Reads)
                && r.actor_name == "Manager"),
            "passive 'is sampled by' must extract (Manager, Reads, HREADY), got: {:?}",
            relations
        );
    }

    #[test]
    fn must_drive_pattern_extracts_actor_from_requester_sentence() {
        // Validates APB-style sentence: "The Requester must drive PSEL"
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };

        let signals = ["PSEL".to_string()]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let stmts = vec![ExtractedStatement {
            statement_id: "s4".to_string(),
            text: "The Requester must drive PSEL before asserting PENABLE.".to_string(),
            class: StatementClass::NormativeStatement,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];
        let relations = extract_actor_signal_relations(&stmts, &signals);
        assert!(
            relations.iter().any(|r| r.signal_name == "PSEL"
                && matches!(r.relation, RelationKind::Drives)
                && r.actor_name == "Requester"),
            "'must drive SIGNAL' must extract (Requester, Drives, PSEL), got: {:?}",
            relations
        );
    }

    #[test]
    fn coordinated_active_drive_extracts_real_actor_not_payload_phrase() {
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };

        let signals = ["TVALID".to_string(), "TREADY".to_string()]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let stmts = vec![ExtractedStatement {
            statement_id: "s5".to_string(),
            text: "In Figure 2-1, the Transmitter presents the data and control information and asserts TVALID as HIGH. The transfer takes place once the Receiver asserts TREADY HIGH.".to_string(),
            class: StatementClass::SignalValueConstraint,
            modality: EvidenceModality::Mixed,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];

        let relations = extract_actor_signal_relations(&stmts, &signals);

        assert!(
            relations.iter().any(|r| r.signal_name == "TVALID"
                && matches!(r.relation, RelationKind::Drives)
                && r.actor_name == "Transmitter"),
            "coordinated active clause must keep the real subject actor for TVALID, got: {:?}",
            relations
        );
        assert!(
            relations.iter().any(|r| r.signal_name == "TREADY"
                && matches!(r.relation, RelationKind::Drives)
                && r.actor_name == "Receiver"),
            "explicit receiver assertion must still recover (Receiver, Drives, TREADY), got: {:?}",
            relations
        );
        assert!(
            !relations
                .iter()
                .any(|r| r.actor_name == "control information"),
            "payload phrases must not be promoted into actors, got: {:?}",
            relations
        );
    }

    #[test]
    fn relative_clause_active_drive_extracts_head_subject_not_mixture_phrase() {
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };

        let signals = ["ARCHUNKEN".to_string(), "RCHUNKV".to_string()]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let stmts = vec![ExtractedStatement {
            statement_id: "s6".to_string(),
            text: "An interconnect which connects to components with a mixture of chunking support can drive ARCHUNKEN and RCHUNKV according to the capabilities of the attached components.".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];

        let relations = extract_actor_signal_relations(&stmts, &signals);

        assert!(
            relations.iter().any(|r| r.signal_name == "ARCHUNKEN"
                && matches!(r.relation, RelationKind::Drives)
                && r.actor_name == "interconnect"),
            "relative-clause subject parsing must keep interconnect as the ARCHUNKEN actor, got: {:?}",
            relations
        );
        assert!(
            relations.iter().any(|r| r.signal_name == "RCHUNKV"
                && matches!(r.relation, RelationKind::Drives)
                && r.actor_name == "interconnect"),
            "coordinated relative-clause object parsing must keep interconnect as the RCHUNKV actor, got: {:?}",
            relations
        );
        assert!(
            !relations.iter().any(|r| r.actor_name == "mixture of"),
            "descriptive phrases like `mixture of` must not become actors, got: {:?}",
            relations
        );
    }

    #[test]
    fn coordinated_active_drive_object_scan_stops_before_guard_clause() {
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };

        let signals = ["DATA".to_string(), "AWVALID".to_string()]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let stmts = vec![ExtractedStatement {
            statement_id: "s7".to_string(),
            text: "The Manager drives DATA when AWVALID is HIGH.".to_string(),
            class: StatementClass::SignalValueConstraint,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];

        let relations = extract_actor_signal_relations(&stmts, &signals);

        assert!(
            relations.iter().any(|r| r.signal_name == "DATA"
                && matches!(r.relation, RelationKind::Drives)
                && r.actor_name == "Manager"),
            "active object scan must keep DATA as the driven object, got: {:?}",
            relations
        );
        assert!(
            !relations.iter().any(|r| r.signal_name == "AWVALID"
                && matches!(r.relation, RelationKind::Drives)
                && r.actor_name == "Manager"),
            "guard signal AWVALID must not become a driven object, got: {:?}",
            relations
        );
    }

    #[test]
    fn coordinated_active_read_extracts_all_sampled_objects() {
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };

        let signals = ["ARCHUNKEN".to_string(), "RCHUNKV".to_string()]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let stmts = vec![ExtractedStatement {
            statement_id: "s8".to_string(),
            text: "The Manager samples ARCHUNKEN and RCHUNKV.".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];

        let relations = extract_actor_signal_relations(&stmts, &signals);

        assert!(
            relations.iter().any(|r| r.signal_name == "ARCHUNKEN"
                && matches!(r.relation, RelationKind::Reads)
                && r.actor_name == "Manager"),
            "coordinated active read must recover Manager reads ARCHUNKEN, got: {:?}",
            relations
        );
        assert!(
            relations.iter().any(|r| r.signal_name == "RCHUNKV"
                && matches!(r.relation, RelationKind::Reads)
                && r.actor_name == "Manager"),
            "coordinated active read must recover Manager reads RCHUNKV, got: {:?}",
            relations
        );
    }

    #[test]
    fn coordinated_active_read_object_scan_stops_before_guard_clause() {
        use super::{
            EvidenceModality, ExtractedStatement, RelationKind, StatementClass,
            extract_actor_signal_relations,
        };

        let signals = ["DATA".to_string(), "RVALID".to_string()]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let stmts = vec![ExtractedStatement {
            statement_id: "s9".to_string(),
            text: "The Manager samples DATA when RVALID is HIGH.".to_string(),
            class: StatementClass::SignalValueConstraint,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];

        let relations = extract_actor_signal_relations(&stmts, &signals);

        assert!(
            relations.iter().any(|r| r.signal_name == "DATA"
                && matches!(r.relation, RelationKind::Reads)
                && r.actor_name == "Manager"),
            "active read object scan must keep DATA as the sampled object, got: {:?}",
            relations
        );
        assert!(
            !relations.iter().any(|r| r.signal_name == "RVALID"
                && matches!(r.relation, RelationKind::Reads)
                && r.actor_name == "Manager"),
            "guard signal RVALID must not become a sampled object, got: {:?}",
            relations
        );
    }

    #[test]
    fn synthesize_directions_produces_signal_is_output_declaration() {
        use super::synthesize_directions_from_relations;
        use crate::ir::source::{
            ActorSignalRelation, AutomationConfidence, RelationKind, WidthHint,
        };

        let relations = vec![ActorSignalRelation {
            relation_id: "asr_0001".to_string(),
            actor_name: "slave".to_string(),
            signal_name: "PREADY".to_string(),
            relation: RelationKind::Drives,
            source_statement_ids: vec!["s1".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        }];
        let mut counter = 10usize;
        let already_declared: std::collections::HashSet<String> = std::collections::HashSet::new();
        let width_map: std::collections::HashMap<String, WidthHint> =
            std::collections::HashMap::new();
        let stmts = synthesize_directions_from_relations(
            &relations,
            &already_declared,
            &width_map,
            &mut counter,
        );
        assert_eq!(stmts.len(), 1);
        assert_eq!(stmts[0].text, "Signal PREADY is output.");
        assert_eq!(stmts[0].class, StatementClass::SourceFact);
    }

    #[test]
    fn kg_extraction_produces_graph_declarations_in_evidence_ir() -> Result<()> {
        // Integration test: a spec with APB-style prose should produce actor-signal
        // relations and synthesized Signal X is output. declarations in EvidenceIR.
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        // Include a table-synthesized Signal PREADY declaration AND a prose sentence
        // that references PREADY and another known signal HTRANS.
        fs::write(
            &source,
            concat!(
                "# Signals\n",
                "Signal PREADY is input width 1.\n",
                "Signal HTRANS is output width 2.\n",
                "\n",
                "# Protocol\n",
                "PREADY is driven by the slave to indicate transfer completion.\n",
                "The Manager drives HTRANS to specify the transfer type.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        // The KG should have found both relations.
        assert!(
            evidence_ir.actor_signal_relations.iter().any(|r| {
                use crate::ir::source::RelationKind;
                r.signal_name == "PREADY"
                    && matches!(r.relation, RelationKind::Drives)
                    && r.actor_name == "slave"
            }),
            "expected (slave, Drives, PREADY) in actor_signal_relations"
        );
        assert!(
            evidence_ir.actor_signal_relations.iter().any(|r| {
                use crate::ir::source::RelationKind;
                r.signal_name == "HTRANS"
                    && matches!(r.relation, RelationKind::Drives)
                    && r.actor_name == "Manager"
            }),
            "expected (Manager, Drives, HTRANS) in actor_signal_relations"
        );

        // No direction synthesis for PREADY because it is already declared in the spec
        // ("Signal PREADY is input width 1."). Table declarations are authoritative.
        // The KG relation records are the important output for already-declared signals.
        assert!(
            !evidence_ir
                .extracted_statements
                .iter()
                .any(|s| s.text == "Signal PREADY is output."),
            "KG synthesis must NOT override an existing table declaration for PREADY"
        );

        Ok(())
    }

    #[test]
    fn source_table_relations_skip_infrastructure_labels() -> Result<()> {
        use crate::ir::source::RelationKind;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Signals\n",
                "Signal PCLK is input width 1.\n",
                "Signal PRESETn is input width 1.\n",
                "Signal XREQ is output width 1.\n",
                "Signal XACK is input width 1.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_signal_desc".to_string(),
            asset_id: "asset_signal_desc".to_string(),
            page_id: None,
            caption_text: Some("AMBA-style signal descriptions".to_string()),
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
                    make_table_cell("PCLK", false),
                    make_table_cell("Clock", false),
                    make_table_cell("1", false),
                    make_table_cell("Clock input", false),
                ],
                vec![
                    make_table_cell("PRESETn", false),
                    make_table_cell("Reset", false),
                    make_table_cell("1", false),
                    make_table_cell("Reset input", false),
                ],
                vec![
                    make_table_cell("XREQ", false),
                    make_table_cell("Requester", false),
                    make_table_cell("1", false),
                    make_table_cell("Transfer request", false),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell("Subordinate", false),
                    make_table_cell("1", false),
                    make_table_cell("Transfer accept", false),
                ],
            ],
            row_count: 4,
            col_count: 4,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        assert_eq!(evidence_ir.actor_signal_relations.len(), 4);
        assert!(
            evidence_ir
                .actor_signal_relations
                .iter()
                .all(|relation| { !matches!(relation.actor_name.as_str(), "Clock" | "Reset") }),
            "infrastructure labels must not become actor-signal relations: {:?}",
            evidence_ir.actor_signal_relations
        );
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Requester"
                && relation.signal_name == "XREQ"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Subordinate"
                && relation.signal_name == "XREQ"
                && matches!(relation.relation, RelationKind::Reads)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Subordinate"
                && relation.signal_name == "XACK"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Requester"
                && relation.signal_name == "XACK"
                && matches!(relation.relation, RelationKind::Reads)
        }));
        assert!(!evidence_ir.table_signal_declaration_provenance.is_empty());

        Ok(())
    }

    #[test]
    fn destination_table_relations_map_to_reads() -> Result<()> {
        use crate::ir::source::RelationKind;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");

        fs::write(
            &source,
            concat!("# Signals\n", "Signal XRESP is input width 1.\n",),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_dest_desc".to_string(),
            asset_id: "asset_dest_desc".to_string(),
            page_id: None,
            caption_text: Some("Destination-oriented signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Destination", true),
                make_table_cell("Width", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("XRESP", false),
                make_table_cell("Requester", false),
                make_table_cell("1", false),
                make_table_cell("Returned to the requester", false),
            ]],
            row_count: 1,
            col_count: 4,
        });

        let relations = super::extract_relations_from_signal_tables(&source_ir);
        assert_eq!(relations.len(), 1);
        assert_eq!(relations[0].actor_name, "Requester");
        assert_eq!(relations[0].signal_name, "XRESP");
        assert!(
            matches!(relations[0].relation, RelationKind::Reads),
            "destination columns must produce Reads relations, got: {:?}",
            relations
        );

        Ok(())
    }

    #[test]
    fn external_source_rows_do_not_synthesize_infrastructure_outputs() -> Result<()> {
        use crate::ir::source::RelationKind;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("external_infrastructure_rows.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(&source, "# Global Signals\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_global_signal_desc".to_string(),
            asset_id: "asset_global_signal_desc".to_string(),
            page_id: None,
            caption_text: Some("Global signal descriptions".to_string()),
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
                    make_table_cell("ACLK", false),
                    make_table_cell("External", false),
                    make_table_cell("1", false),
                    make_table_cell("Global clock signal", false),
                ],
                vec![
                    make_table_cell("ARESETn", false),
                    make_table_cell("External", false),
                    make_table_cell("1", false),
                    make_table_cell("Active low reset signal", false),
                ],
            ],
            row_count: 2,
            col_count: 4,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(
            evidence_ir
                .actor_signal_relations
                .iter()
                .all(|relation| relation.actor_name != "External"),
            "`External` must not become a protocol actor relation: {:?}",
            evidence_ir.actor_signal_relations
        );
        assert!(
            !evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Signal ACLK is output width 1."),
            "external clock source rows must not synthesize output declarations"
        );
        assert!(
            !evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Signal ARESETN is output width 1."),
            "external reset source rows must not synthesize output declarations"
        );
        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Clock ACLK."),
            "clock semantics should still be recovered from the local table description"
        );
        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| { statement.text == "Reset ARESETN is asynchronous active low." })
        );
        assert!(
            evidence_ir.actor_signal_relations.is_empty()
                || evidence_ir.actor_signal_relations.iter().all(|relation| {
                    !matches!(
                        relation.relation,
                        RelationKind::Drives | RelationKind::Reads
                    )
                }),
            "no protocol actor relations should be synthesized from external infrastructure rows"
        );

        Ok(())
    }

    #[test]
    fn tie_off_source_rows_become_input_declarations_without_fake_actor() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("tie_off_interface_controls.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(&source, "# Appendix\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_tie_off_signal_desc".to_string(),
            asset_id: "asset_tie_off_signal_desc".to_string(),
            page_id: Some("page_0001".to_string()),
            caption_text: Some("Table B1.13: Interface control signals".to_string()),
            source_ref: Some("#/tables/0".to_string()),
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Name", true),
                make_table_cell("Width", true),
                make_table_cell("Source", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("BROADCASTATOMIC", false),
                    make_table_cell("1", false),
                    make_table_cell("Tie-off", false),
                    make_table_cell("Control input for Atomic transactions", false),
                ],
                vec![
                    make_table_cell("BROADCASTSHAREABLE", false),
                    make_table_cell("1", false),
                    make_table_cell("Tie-off", false),
                    make_table_cell("Control input for Shareable transactions", false),
                ],
            ],
            row_count: 2,
            col_count: 4,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(
            evidence_ir
                .actor_signal_relations
                .iter()
                .all(|relation| relation.actor_name != "Tie-off"),
            "`Tie-off` must not become a protocol actor relation: {:?}",
            evidence_ir.actor_signal_relations
        );
        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Signal BROADCASTATOMIC is input width 1."),
            "tie-off rows should synthesize input declarations"
        );
        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Signal BROADCASTSHAREABLE is input width 1."),
            "tie-off rows should synthesize input declarations"
        );
        assert!(
            !evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Signal BROADCASTATOMIC is output width 1."),
            "tie-off rows must not synthesize output declarations"
        );
        assert!(
            !evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Signal BROADCASTSHAREABLE is output width 1."),
            "tie-off rows must not synthesize output declarations"
        );
        assert!(!evidence_ir.table_signal_declaration_provenance.is_empty());
        assert!(
            evidence_ir
                .table_signal_declaration_provenance
                .iter()
                .any(|p| p.signal_name == "BROADCASTATOMIC")
        );
        assert!(
            evidence_ir
                .table_signal_declaration_provenance
                .iter()
                .any(|p| p.signal_name == "BROADCASTSHAREABLE")
        );

        Ok(())
    }

    #[test]
    fn two_column_signal_description_table_synthesizes_declarations() -> Result<()> {
        // SIGNAL-TABLE-COLUMNLESS-RECALL (approach A: prose-direction inference).
        // A two-column `Signal | Description` interface table — no Source/Width/Direction
        // columns — like CHI "Table B13.2: REQ channel interface signals" must capture
        // the signals whose driver is stated in the description prose:
        //   - REQFLITV  "The transmitter sets this signal HIGH …" → output (captured)
        //   - REQLCRDV  "The receiver sets this signal HIGH …"    → input  (captured)
        //   - REQFLITPEND (no driver verb)                        → honest residual
        //   - REQFLIT[(R-1):0] (bracket chars)                    → not a signal token
        // Before the fix this table synthesized ZERO declarations (direction-OR-width
        // gate); the bug was confirmed live by this test failing with `got []`.
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(&source, "# REQ channel interface signals\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_req_channel".to_string(),
            asset_id: "asset_req_channel".to_string(),
            page_id: None,
            caption_text: Some("Table B13.2: REQ channel interface signals".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("REQFLITPEND", false),
                    make_table_cell(
                        "Request Flit Pending. Early indication that a request flit could be transmitted in the following cycle.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("REQFLITV", false),
                    make_table_cell(
                        "Request Flit Valid. The transmitter sets this signal HIGH to indicate when the request flit is valid.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("REQFLIT[(R-1):0]", false),
                    make_table_cell("Request Flit. See the request flit format.", false),
                ],
                vec![
                    make_table_cell("REQLCRDV", false),
                    make_table_cell(
                        "Request L-Credit Valid. The receiver sets this signal HIGH to return a request channel L-Credit to a transmitter.",
                        false,
                    ),
                ],
            ],
            row_count: 5,
            col_count: 2,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        let declared: Vec<&str> = evidence_ir
            .table_signal_declaration_provenance
            .iter()
            .filter(|p| p.table_id == "table_req_channel")
            .map(|p| p.signal_name.as_str())
            .collect();
        // Driver stated in prose → captured (was 0 before the fix).
        assert!(
            declared.contains(&"REQFLITV"),
            "transmitter-driven signal must be declared (output); got {declared:?}"
        );
        assert!(
            declared.contains(&"REQLCRDV"),
            "receiver-driven signal must be declared (input); got {declared:?}"
        );
        // No driver verb → honest residual, not a fabricated direction.
        assert!(
            !declared.contains(&"REQFLITPEND"),
            "direction-less signal must stay an honest residual; got {declared:?}"
        );

        // The inferred directions must match the prose: transmitter → output,
        // receiver → input.
        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|s| s.text == "Signal REQFLITV is output."),
            "transmitter-driven REQFLITV must be output"
        );
        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|s| s.text == "Signal REQLCRDV is input."),
            "receiver-driven REQLCRDV must be input"
        );

        Ok(())
    }

    #[test]
    fn source_table_relations_infer_unique_complementary_reads() -> Result<()> {
        use crate::ir::source::RelationKind;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");

        fs::write(&source, "# Signals\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_source_desc".to_string(),
            asset_id: "asset_source_desc".to_string(),
            page_id: None,
            caption_text: Some("Source-oriented signal descriptions".to_string()),
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
                    make_table_cell("Transfer response", false),
                ],
            ],
            row_count: 2,
            col_count: 4,
        });

        let relations = super::extract_relations_from_signal_tables(&source_ir);
        assert_eq!(relations.len(), 4, "expected paired drive/read relations");
        assert!(relations.iter().any(|relation| {
            relation.actor_name == "Requester"
                && relation.signal_name == "XREQ"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(relations.iter().any(|relation| {
            relation.actor_name == "Completer"
                && relation.signal_name == "XREQ"
                && matches!(relation.relation, RelationKind::Reads)
        }));
        assert!(relations.iter().any(|relation| {
            relation.actor_name == "Completer"
                && relation.signal_name == "XACK"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(relations.iter().any(|relation| {
            relation.actor_name == "Requester"
                && relation.signal_name == "XACK"
                && matches!(relation.relation, RelationKind::Reads)
        }));

        Ok(())
    }

    #[test]
    fn source_table_relations_skip_complementary_reads_when_opposite_actor_is_ambiguous()
    -> Result<()> {
        use crate::ir::source::RelationKind;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");

        fs::write(&source, "# Signals\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_source_desc".to_string(),
            asset_id: "asset_source_desc".to_string(),
            page_id: None,
            caption_text: Some("Ambiguous source-oriented signal descriptions".to_string()),
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
                    make_table_cell("XACK0", false),
                    make_table_cell("Completer A", false),
                    make_table_cell("1", false),
                    make_table_cell("Transfer response", false),
                ],
                vec![
                    make_table_cell("XACK1", false),
                    make_table_cell("Completer B", false),
                    make_table_cell("1", false),
                    make_table_cell("Transfer response", false),
                ],
            ],
            row_count: 3,
            col_count: 4,
        });

        let relations = super::extract_relations_from_signal_tables(&source_ir);
        assert!(
            relations.iter().all(|relation| {
                !(relation.signal_name == "XREQ"
                    && matches!(relation.relation, RelationKind::Reads))
            }),
            "ambiguous opposite actors must block complementary read inference: {:?}",
            relations
        );

        Ok(())
    }

    #[test]
    fn check_signal_tables_inherit_relations_from_covered_signals() -> Result<()> {
        use crate::ir::source::RelationKind;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Signals\n",
                "Signal XREQ is output width 1.\n",
                "Signal XACK is input width 1.\n",
                "Signal XDATA is output width DATA_WIDTH.\n",
                "Signal XKEEP is output width DATA_WIDTH/8.\n",
                "Signal XREQCHK is output width 1.\n",
                "Signal XACKCHK is input width 1.\n",
                "Signal XDATACHK is output width DATA_WIDTH/8.\n",
                "Signal XKEEPCHK is output width DATA_WIDTH/8.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_signal_desc".to_string(),
            asset_id: "asset_signal_desc".to_string(),
            page_id: None,
            caption_text: Some("Source-oriented signal descriptions".to_string()),
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
                    make_table_cell("Transfer response", false),
                ],
                vec![
                    make_table_cell("XDATA", false),
                    make_table_cell("Requester", false),
                    make_table_cell("DATA_WIDTH", false),
                    make_table_cell("Transfer payload", false),
                ],
                vec![
                    make_table_cell("XKEEP", false),
                    make_table_cell("Requester", false),
                    make_table_cell("DATA_WIDTH/8", false),
                    make_table_cell("Byte qualifier", false),
                ],
            ],
            row_count: 4,
            col_count: 4,
        });
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_check_desc".to_string(),
            asset_id: "asset_check_desc".to_string(),
            page_id: None,
            caption_text: Some("Parity check signals".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Check Signal", true),
                make_table_cell("Signals Covered", true),
                make_table_cell("Width", true),
                make_table_cell("Granularity", true),
                make_table_cell("Check Enable", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("XREQCHK", false),
                    make_table_cell("XREQ 1", false),
                    make_table_cell("1", false),
                    make_table_cell("XREQ", false),
                    make_table_cell("", false),
                ],
                vec![
                    make_table_cell("XACKCHK", false),
                    make_table_cell("XACK 1", false),
                    make_table_cell("1", false),
                    make_table_cell("", false),
                    make_table_cell("XACK", false),
                ],
                vec![
                    make_table_cell("XDATACHK", false),
                    make_table_cell("XDATA DATA_WIDTH/8", false),
                    make_table_cell("1-8", false),
                    make_table_cell("XREQ", false),
                    make_table_cell("", false),
                ],
                vec![
                    make_table_cell("XKEEPCHK", false),
                    make_table_cell("DATA_WIDTH/8", false),
                    make_table_cell("1-8", false),
                    make_table_cell("XREQ", false),
                    make_table_cell("XKEEP", false),
                ],
            ],
            row_count: 4,
            col_count: 5,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Requester"
                && relation.signal_name == "XREQCHK"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Completer"
                && relation.signal_name == "XREQCHK"
                && matches!(relation.relation, RelationKind::Reads)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Completer"
                && relation.signal_name == "XACKCHK"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Requester"
                && relation.signal_name == "XACKCHK"
                && matches!(relation.relation, RelationKind::Reads)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Requester"
                && relation.signal_name == "XDATACHK"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Completer"
                && relation.signal_name == "XDATACHK"
                && matches!(relation.relation, RelationKind::Reads)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Requester"
                && relation.signal_name == "XKEEPCHK"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Completer"
                && relation.signal_name == "XKEEPCHK"
                && matches!(relation.relation, RelationKind::Reads)
        }));
        let check_signal_statements = evidence_ir
            .extracted_statements
            .iter()
            .filter(|statement| {
                statement.text.contains("XDATACHK") || statement.text.contains("XKEEPCHK")
            })
            .map(|statement| statement.text.clone())
            .collect::<Vec<_>>();
        assert!(
            evidence_ir.extracted_statements.iter().any(|statement| {
                statement
                    .text
                    .contains("Signal XDATACHK is output width DATA_WIDTH/8.")
            }),
            "expected XDATACHK directional width statement, saw: {:?}",
            check_signal_statements
        );
        assert!(
            evidence_ir.extracted_statements.iter().any(|statement| {
                statement
                    .text
                    .contains("Signal XKEEPCHK is output width DATA_WIDTH/8.")
            }),
            "expected XKEEPCHK directional width statement, saw: {:?}",
            check_signal_statements
        );
        assert!(!evidence_ir.table_signal_declaration_provenance.is_empty());
        assert!(
            evidence_ir
                .table_signal_declaration_provenance
                .iter()
                .any(|p| p.signal_name == "XREQ")
        );
        assert!(
            evidence_ir
                .table_signal_declaration_provenance
                .iter()
                .any(|p| p.signal_name == "XACK")
        );

        Ok(())
    }

    #[test]
    fn actor_taxonomy_priors_guide_source_column_direction_inference() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("amba_producer_consumer.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let prior_memory_path = write_actor_taxonomy_prior_memory(tempdir.path())?;

        fs::write(&source, "# Interface\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_signal_desc".to_string(),
            asset_id: "asset_signal_desc".to_string(),
            page_id: None,
            caption_text: Some("Producer and consumer signals".to_string()),
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
                    make_table_cell("Producer", false),
                    make_table_cell("1", false),
                    make_table_cell("Transfer request", false),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell("Consumer", false),
                    make_table_cell("1", false),
                    make_table_cell("Transfer accept", false),
                ],
            ],
            row_count: 2,
            col_count: 4,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build_with_prior_memory(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
            Some(&prior_memory_path),
        )?;

        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Signal XREQ is output width 1."),
            "expected requester-like prior to recover output direction for Producer source column"
        );
        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Signal XACK is input width 1."),
            "expected completer-like prior to recover input direction for Consumer source column"
        );
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Producer"
                && relation.signal_name == "XREQ"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Consumer"
                && relation.signal_name == "XREQ"
                && matches!(relation.relation, RelationKind::Reads)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Consumer"
                && relation.signal_name == "XACK"
                && matches!(relation.relation, RelationKind::Drives)
        }));
        assert!(evidence_ir.actor_signal_relations.iter().any(|relation| {
            relation.actor_name == "Producer"
                && relation.signal_name == "XACK"
                && matches!(relation.relation, RelationKind::Reads)
        }));

        Ok(())
    }

    #[test]
    fn actor_taxonomy_priors_guide_section_heading_direction_inference() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("amba_section_heading.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let prior_memory_path = write_actor_taxonomy_prior_memory(tempdir.path())?;

        fs::write(&source, "# Interface\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir
            .document_sections
            .push(crate::ir::source::ContentSectionRecord {
                section_id: "section_producer_signals".to_string(),
                title: "Producer signals".to_string(),
                heading_level: 1,
                page_id: Some("page_0001".to_string()),
                source_ref: None,
                reading_order: 1,
                section_kind: SectionKind::SignalDescription,
            });
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_section_signal_desc".to_string(),
            asset_id: "asset_section_signal_desc".to_string(),
            page_id: Some("page_0001".to_string()),
            caption_text: Some("Section-guided signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Width", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("XREQ", false),
                make_table_cell("1", false),
                make_table_cell("Transfer request", false),
            ]],
            row_count: 1,
            col_count: 3,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build_with_prior_memory(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
            Some(&prior_memory_path),
        )?;

        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Signal XREQ is output width 1."),
            "expected section-heading actor taxonomy prior to recover output direction"
        );
        assert!(
            evidence_ir.actor_signal_relations.iter().any(|relation| {
                relation.actor_name == "Producer"
                    && relation.signal_name == "XREQ"
                    && matches!(relation.relation, RelationKind::Drives)
            }),
            "expected section-heading actor taxonomy prior to recover a structural Drives relation"
        );

        Ok(())
    }

    #[test]
    fn semantic_phrase_priors_guide_local_semantic_hint_recovery() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("amba_semantic_prior.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let prior_memory_path = write_semantic_phrase_prior_memory(tempdir.path())?;

        fs::write(
            &source,
            concat!(
                "# Interface\n",
                "Signal XACK is input width 1.\n",
                "XACK can receive the transfer.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        let without_priors = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        assert!(
            without_priors.signal_semantic_hints.is_empty(),
            "non-hardcoded phrase should not resolve without prior guidance: {:?}",
            without_priors.signal_semantic_hints
        );

        let evidence_ir = EvidenceIr::build_with_prior_memory(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
            Some(&prior_memory_path),
        )?;
        assert!(
            evidence_ir.signal_semantic_hints.iter().any(|hint| {
                hint.signal_name == "XACK"
                    && matches!(
                        hint.source_kind,
                        SignalSemanticHintSourceKind::ProseStatement
                    )
                    && hint
                        .semantic_tags
                        .contains(&SignalSemanticTag::HandshakeReadyLike)
                    && hint.automation_confidence == AutomationConfidence::Low
                    && !hint.supporting_statement_ids.is_empty()
            }),
            "semantic phrase prior should recover a ready-like hint from local prose: {:?}",
            evidence_ir.signal_semantic_hints
        );
        assert_eq!(
            evidence_ir.prior_memory_path,
            Some(canonicalize_existing_path(&prior_memory_path)?)
        );

        evidence_ir.write_to_disk()?;
        let mut reloaded =
            EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;
        reloaded.refresh_signal_semantic_hints()?;
        assert!(
            reloaded.signal_semantic_hints.iter().any(|hint| {
                hint.signal_name == "XACK"
                    && hint
                        .semantic_tags
                        .contains(&SignalSemanticTag::HandshakeReadyLike)
            }),
            "persisted prior_memory_path should preserve prior-guided semantic hints across refreshes"
        );

        Ok(())
    }

    #[test]
    fn visual_motif_priors_classify_unknown_captioned_visual_assets() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("axi_visual_motif_prior.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let prior_memory_path = write_visual_motif_prior_memory(tempdir.path())?;

        fs::write(&source, "# Interface\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_signal_desc".to_string(),
            asset_id: "asset_signal_desc".to_string(),
            page_id: None,
            caption_text: Some("Interface signals".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Direction", true),
                make_table_cell("Width", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("XREQ", false),
                make_table_cell("Output", false),
                make_table_cell("1", false),
                make_table_cell("Transfer request", false),
            ]],
            row_count: 1,
            col_count: 4,
        });
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
            diagram_kind: DiagramKind::Unknown,
        });
        source_ir.write_to_disk()?;

        let without_priors = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        assert!(
            !without_priors.visual_evidence[0]
                .observations
                .iter()
                .any(|observation| matches!(
                    observation.kind,
                    VisualObservationKind::Classification
                )),
            "unknown diagram should not gain prior-guided classification without prior memory"
        );
        assert_eq!(
            without_priors.visual_evidence[0].role,
            super::VisualEvidenceRole::Ambiguous
        );
        assert_eq!(
            without_priors.visual_evidence[0].automation_confidence,
            AutomationConfidence::High
        );

        let evidence_ir = EvidenceIr::build_with_prior_memory(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
            Some(&prior_memory_path),
        )?;
        assert!(
            evidence_ir.visual_evidence[0]
                .observations
                .iter()
                .any(|observation| matches!(
                    observation.kind,
                    VisualObservationKind::Classification
                ) && observation.created_by == "specforge_prior_memory"
                    && observation.text == "diagram_kind=timing_diagram"
                    && observation.automation_confidence == AutomationConfidence::Medium),
            "visual motif prior should add an explicit Classification observation: {:?}",
            evidence_ir.visual_evidence[0].observations
        );
        assert_eq!(
            evidence_ir.visual_evidence[0].role,
            super::VisualEvidenceRole::Normative,
            "prior-guided timing classification should only upgrade the visual evidence role, not synthesize semantic facts"
        );
        assert_eq!(
            evidence_ir.visual_evidence[0].automation_confidence,
            AutomationConfidence::High,
            "captioned visual evidence should carry High automation confidence"
        );

        Ok(())
    }

    #[test]
    fn misclassified_field_table_does_not_synthesize_fake_signal_semantics() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("fields.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!("# Control\n", "Signal CONTROL is input width 4.\n",),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_control_fields".to_string(),
            asset_id: "asset_control_fields".to_string(),
            page_id: None,
            caption_text: Some("Control signal fields".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Bits", true),
                make_table_cell("Name", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("3", false),
                    make_table_cell("REQ", false),
                    make_table_cell("Request field indicates that a transfer is valid.", false),
                ],
                vec![
                    make_table_cell("2", false),
                    make_table_cell("ACK", false),
                    make_table_cell(
                        "Accept field indicates that the transfer can be accepted.",
                        false,
                    ),
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

        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .all(|statement| !statement.text.contains("Signal REQ")
                    && !statement.text.contains("Signal ACK")),
            "misclassified field tables must not synthesize fake top-level signal declarations: {:?}",
            evidence_ir.extracted_statements
        );
        assert!(
            evidence_ir
                .signal_semantic_hints
                .iter()
                .all(|hint| hint.signal_name != "REQ" && hint.signal_name != "ACK"),
            "misclassified field tables must not synthesize fake semantic hints: {:?}",
            evidence_ir.signal_semantic_hints
        );
        assert!(
            evidence_ir.actor_signal_relations.is_empty(),
            "misclassified field tables must not synthesize actor relations: {:?}",
            evidence_ir.actor_signal_relations
        );

        Ok(())
    }

    #[test]
    fn field_like_width_table_does_not_leak_message_fields_as_signals() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("message_fields.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Messages\n",
                "Signal ACLK is input width 1.\n",
                "Signal ACTIVATEACK is output width 1.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_message_fields".to_string(),
            asset_id: "asset_message_fields".to_string(),
            page_id: None,
            caption_text: Some("DVM message fields".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Name", true),
                make_table_cell("Width", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("PA", false),
                    make_table_cell("32-52", false),
                    make_table_cell("Physical Address", false),
                ],
                vec![
                    make_table_cell("IS", false),
                    make_table_cell("4", false),
                    make_table_cell(
                        "Invalidation Size encoding for GPT TLBI by PA operations.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("Completion", false),
                    make_table_cell("1", false),
                    make_table_cell(
                        "Asserted HIGH to indicate that a Completion message is required.",
                        false,
                    ),
                ],
            ],
            row_count: 3,
            col_count: 3,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .all(|statement| !statement.text.contains("Signal PA is")
                    && !statement.text.contains("Signal IS is")
                    && !statement.text.contains("Signal COMPLETION is")),
            "field-like width tables must not synthesize fake top-level signal declarations: {:?}",
            evidence_ir.extracted_statements
        );
        assert!(
            evidence_ir.signal_polarities.iter().all(|record| {
                record.signal_name != "PA"
                    && record.signal_name != "IS"
                    && record.signal_name != "COMPLETION"
            }),
            "field-like width tables must not synthesize fake polarity facts: {:?}",
            evidence_ir.signal_polarities
        );
        assert!(
            evidence_ir.signal_semantic_hints.iter().all(|hint| {
                hint.signal_name != "PA"
                    && hint.signal_name != "IS"
                    && hint.signal_name != "COMPLETION"
            }),
            "field-like width tables must not synthesize fake semantic hints: {:?}",
            evidence_ir.signal_semantic_hints
        );

        Ok(())
    }

    #[test]
    fn abstract_transport_signal_tables_do_not_become_top_level_interfaces() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("transport.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Transport\n",
                "When using credited transport, transfers use abstract Tx/Rx channel signaling.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_transport_signals".to_string(),
            asset_id: "asset_transport_signals".to_string(),
            page_id: None,
            caption_text: Some("Credited channel signals".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Name", true),
                make_table_cell("Width", true),
                make_table_cell("Source", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("VALID", false),
                    make_table_cell("1", false),
                    make_table_cell("Tx", false),
                    make_table_cell("One transfer from Tx to Rx.", false),
                ],
                vec![
                    make_table_cell("PENDING", false),
                    make_table_cell("1", false),
                    make_table_cell("Tx", false),
                    make_table_cell("Transfer might occur next cycle.", false),
                ],
                vec![
                    make_table_cell("CRDT", false),
                    make_table_cell("1", false),
                    make_table_cell("Rx", false),
                    make_table_cell("Credit returned by Rx.", false),
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

        assert!(
            evidence_ir.extracted_statements.iter().all(|statement| {
                !statement.text.contains("Signal VALID is")
                    && !statement.text.contains("Signal PENDING is")
                    && !statement.text.contains("Signal CRDT is")
            }),
            "abstract transport tables must not synthesize top-level signal declarations: {:?}",
            evidence_ir.extracted_statements
        );
        assert!(
            evidence_ir.actor_signal_relations.iter().all(|relation| {
                relation.signal_name != "VALID"
                    && relation.signal_name != "PENDING"
                    && relation.signal_name != "CRDT"
            }),
            "abstract transport tables must not synthesize actor-signal relations: {:?}",
            evidence_ir.actor_signal_relations
        );
        assert!(
            evidence_ir.signal_semantic_hints.iter().all(|hint| {
                hint.signal_name != "VALID"
                    && hint.signal_name != "PENDING"
                    && hint.signal_name != "CRDT"
            }),
            "abstract transport tables must not synthesize top-level semantic hints: {:?}",
            evidence_ir.signal_semantic_hints
        );

        Ok(())
    }

    #[test]
    fn anchored_encoding_scan_unlocks_dynamic_value_constraint_extraction() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            "# Protocol\nHTRANS must be SETUP when HREADY is HIGH.\n",
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_signal_desc".to_string(),
            asset_id: "asset_signal_desc".to_string(),
            page_id: None,
            caption_text: Some("Signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Source", true),
                make_table_cell("Width", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("HTRANS", false),
                make_table_cell("Manager", false),
                make_table_cell("2", false),
                make_table_cell("Transfer type signal", false),
            ]],
            row_count: 1,
            col_count: 4,
        });
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_htrans_encoding".to_string(),
            asset_id: "asset_htrans_encoding".to_string(),
            page_id: None,
            caption_text: Some("HTRANS encodings".to_string()),
            source_ref: None,
            table_kind: TableKind::Unknown,
            header_rows: vec![vec![
                make_table_cell("HTRANS[1:0]", true),
                make_table_cell("Transfer type", true),
            ]],
            body_rows: vec![
                vec![make_table_cell("00", false), make_table_cell("IDLE", false)],
                vec![
                    make_table_cell("01", false),
                    make_table_cell("SETUP", false),
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

        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text == "Enum HTRANS SETUP = 1."),
            "expected anchored scan to synthesize Enum HTRANS SETUP = 1."
        );
        assert!(
            evidence_ir.signal_constraints.iter().any(|constraint| {
                constraint.subject_signal == "HTRANS"
                    && constraint.target_value.as_deref() == Some("SETUP")
                    && matches!(
                        constraint.constraint_kind,
                        crate::ir::source::SignalConstraintKind::MustBeValue { ref value }
                            if value == "SETUP"
                    )
                    && constraint.condition_text.as_deref() == Some("HREADY is HIGH")
            }),
            "expected discovered enum value SETUP to unlock a dynamic signal constraint"
        );

        Ok(())
    }

    #[test]
    fn prose_polarity_refines_asserted_constraint_kind() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("reset.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Reset\n",
                "Signal RST_N is input width 1.\n",
                "RST_N is an active low reset signal.\n",
                "RST_N must be asserted during initialization.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(
            evidence_ir.signal_constraints.iter().any(|constraint| {
                constraint.subject_signal == "RST_N"
                    && matches!(
                        constraint.constraint_kind,
                        crate::ir::source::SignalConstraintKind::MustBeLow
                    )
            }),
            "expected active-low prose to refine asserted constraint into MustBeLow"
        );

        Ok(())
    }

    #[test]
    fn signal_polarity_detector_accepts_asserted_when_level_phrases() {
        assert_eq!(
            super::detect_signal_polarity("cs_n is asserted when low"),
            Some(super::SignalPolarity::ActiveLow)
        );
        assert_eq!(
            super::detect_signal_polarity("enable is high when asserted"),
            Some(super::SignalPolarity::ActiveHigh)
        );
        assert_eq!(
            super::detect_signal_polarity("cs_n is active low and enable is active high"),
            None
        );
    }

    #[test]
    fn prose_asserted_when_low_recovers_non_reset_control_polarity() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("chip_select_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

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

        let polarity = evidence_ir
            .signal_polarities
            .iter()
            .find(|record| record.signal_name == "CS_N")
            .expect("expected explicit asserted-when-LOW prose to recover CS_N polarity");
        assert_eq!(polarity.polarity, super::SignalPolarity::ActiveLow);
        assert_eq!(polarity.automation_confidence, AutomationConfidence::Medium);
        assert!(!polarity.supporting_statement_ids.is_empty());
        assert!(evidence_ir.signal_constraints.iter().any(|constraint| {
            constraint.subject_signal == "CS_N"
                && constraint.source_text == "CS_N must be asserted."
                && matches!(
                    constraint.constraint_kind,
                    crate::ir::source::SignalConstraintKind::MustBeLow
                )
        }));
        assert!(evidence_ir.signal_constraints.iter().any(|constraint| {
            constraint.subject_signal == "CS_N"
                && constraint.source_text == "CS_N must be deasserted."
                && matches!(
                    constraint.constraint_kind,
                    crate::ir::source::SignalConstraintKind::MustBeHigh
                )
        }));

        Ok(())
    }

    #[test]
    fn collective_active_low_prose_recovers_multiple_control_polarities() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("collective_control_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

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

        for signal_name in ["CS_N", "WE_N"] {
            let polarity = evidence_ir
                .signal_polarities
                .iter()
                .find(|record| record.signal_name == signal_name)
                .expect("expected collective active-low prose to recover both control signals");
            assert_eq!(polarity.polarity, super::SignalPolarity::ActiveLow);
            assert_eq!(polarity.automation_confidence, AutomationConfidence::Medium);
            assert!(!polarity.supporting_statement_ids.is_empty());
        }
        assert!(evidence_ir.signal_constraints.iter().any(|constraint| {
            constraint.subject_signal == "CS_N"
                && matches!(
                    constraint.constraint_kind,
                    crate::ir::source::SignalConstraintKind::MustBeLow
                )
        }));
        assert!(evidence_ir.signal_constraints.iter().any(|constraint| {
            constraint.subject_signal == "WE_N"
                && matches!(
                    constraint.constraint_kind,
                    crate::ir::source::SignalConstraintKind::MustBeHigh
                )
        }));

        Ok(())
    }

    #[test]
    fn mixed_polarity_prose_recovers_clause_local_control_polarities() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("mixed_control_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Control\n",
                "Signal CS_N is input width 1.\n",
                "\n",
                "Signal ENABLE is input width 1.\n",
                "\n",
                "CS_N is active LOW and ENABLE is active HIGH.\n",
                "\n",
                "CS_N must be asserted.\n",
                "\n",
                "ENABLE must be asserted.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        let cs_n_polarity = evidence_ir
            .signal_polarities
            .iter()
            .find(|record| record.signal_name == "CS_N")
            .expect("expected clause-local active-low polarity for CS_N");
        assert_eq!(cs_n_polarity.polarity, super::SignalPolarity::ActiveLow);
        assert_eq!(
            cs_n_polarity.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(!cs_n_polarity.supporting_statement_ids.is_empty());
        let enable_polarity = evidence_ir
            .signal_polarities
            .iter()
            .find(|record| record.signal_name == "ENABLE")
            .expect("expected clause-local active-high polarity for ENABLE");
        assert_eq!(enable_polarity.polarity, super::SignalPolarity::ActiveHigh);
        assert_eq!(
            enable_polarity.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(!enable_polarity.supporting_statement_ids.is_empty());
        assert!(evidence_ir.signal_constraints.iter().any(|constraint| {
            constraint.subject_signal == "CS_N"
                && matches!(
                    constraint.constraint_kind,
                    crate::ir::source::SignalConstraintKind::MustBeLow
                )
        }));
        assert!(evidence_ir.signal_constraints.iter().any(|constraint| {
            constraint.subject_signal == "ENABLE"
                && matches!(
                    constraint.constraint_kind,
                    crate::ir::source::SignalConstraintKind::MustBeHigh
                )
        }));

        Ok(())
    }

    #[test]
    fn detached_mixed_polarity_prose_does_not_guess_implicit_control_polarity() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("detached_mixed_control_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Control\n",
                "Signal CS_N is input width 1.\n",
                "\n",
                "CS_N is active LOW and active HIGH.\n",
                "\n",
                "CS_N must be asserted.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(
            evidence_ir.signal_polarities.is_empty(),
            "detached mixed-polarity prose must stay unresolved instead of inheriting an implicit signal subject"
        );
        assert!(evidence_ir.signal_constraints.iter().any(|constraint| {
            constraint.subject_signal == "CS_N"
                && matches!(
                    constraint.constraint_kind,
                    crate::ir::source::SignalConstraintKind::MustBeAsserted
                )
        }));

        Ok(())
    }

    #[test]
    fn signal_table_polarity_refines_asserted_constraint_kind() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("reset_table.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Reset\n",
                "Signal PRESETN is input width 1.\n",
                "PRESETN must be asserted during initialization.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_reset_desc".to_string(),
            asset_id: "asset_reset_desc".to_string(),
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
                make_table_cell("Active low reset input", false),
            ]],
            row_count: 1,
            col_count: 2,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(
            evidence_ir.signal_constraints.iter().any(|constraint| {
                constraint.subject_signal == "PRESETN"
                    && matches!(
                        constraint.constraint_kind,
                        crate::ir::source::SignalConstraintKind::MustBeLow
                    )
            }),
            "expected active-low signal table row to refine asserted constraint into MustBeLow"
        );

        Ok(())
    }

    #[test]
    fn conflicting_prose_and_table_polarity_keeps_constraint_polarity_neutral() -> Result<()> {
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

        assert!(
            evidence_ir.signal_constraints.iter().any(|constraint| {
                constraint.subject_signal == "PRESETN"
                    && matches!(
                        constraint.constraint_kind,
                        crate::ir::source::SignalConstraintKind::MustBeAsserted
                    )
            }),
            "expected conflicting prose/table polarity to keep asserted constraint polarity-neutral"
        );
        assert_eq!(evidence_ir.signal_polarity_conflicts.len(), 1);
        let conflict = &evidence_ir.signal_polarity_conflicts[0];
        assert_eq!(conflict.signal_name, "PRESETN");
        assert_eq!(conflict.observations.len(), 2);
        assert_eq!(conflict.automation_confidence, AutomationConfidence::Medium);
        assert!(conflict.observations.iter().any(|observation| matches!(
            observation.source_kind,
            super::SignalPolarityEvidenceSourceKind::ProseStatement
        ) && matches!(
            observation.polarity,
            super::SignalPolarity::ActiveLow
        )
            && !observation.supporting_statement_ids.is_empty()));
        assert!(conflict.observations.iter().any(|observation| matches!(
            observation.source_kind,
            super::SignalPolarityEvidenceSourceKind::SignalDescriptionTable
        ) && matches!(
            observation.polarity,
            super::SignalPolarity::ActiveHigh
        )
            && observation.supporting_statement_ids.is_empty()));

        Ok(())
    }

    #[test]
    fn signal_table_descriptions_produce_semantic_handshake_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("handshake_hints.md");
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

        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XREQ"
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeValidLike)
                && hint.automation_confidence == AutomationConfidence::Medium
                && hint.supporting_statement_ids.is_empty()
        }));
        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XACK"
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeReadyLike)
                && hint.automation_confidence == AutomationConfidence::Medium
                && hint.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn signal_table_descriptions_ignore_other_handshake_signal_mentions() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("multi_signal_handshake_table_hints.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Stream\n",
                "Signal TVALID is input width 1.\n",
                "Signal TREADY is input width 1.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_axi_stream_signals".to_string(),
            asset_id: "asset_axi_stream_signals".to_string(),
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
                    make_table_cell("TVALID", false),
                    make_table_cell(
                        "TVALID indicates the Transmitter is driving a valid transfer. A transfer takes place when both TVALID and TREADY are asserted.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("TREADY", false),
                    make_table_cell(
                        "TREADY indicates that the Receiver can accept a transfer.",
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

        let tvalid_hints: Vec<_> = evidence_ir
            .signal_semantic_hints
            .iter()
            .filter(|hint| hint.signal_name == "TVALID")
            .collect();
        assert!(!tvalid_hints.is_empty());
        assert!(tvalid_hints.iter().any(|hint| {
            hint.semantic_tags
                .contains(&super::SignalSemanticTag::HandshakeValidLike)
        }));
        assert!(!tvalid_hints.iter().any(|hint| {
            hint.semantic_tags
                .contains(&super::SignalSemanticTag::HandshakeReadyLike)
        }));

        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "TREADY"
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeReadyLike)
                && hint.automation_confidence == AutomationConfidence::Medium
                && hint.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn signal_declarations_do_not_create_semantic_hints_from_names_alone() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("signal_name_only_handshake_terms.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal AWVALID is input width 1.\n\n",
                "Signal AWREADY is output width 1.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(
            evidence_ir.signal_semantic_hints.is_empty(),
            "signal identifiers alone should not create semantic handshake hints without descriptive language"
        );

        Ok(())
    }

    #[test]
    fn multi_signal_prose_descriptions_produce_per_signal_semantic_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("multi_signal_prose_handshake_hints.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal XVALID is input width 1.\n\n",
                "Signal XREADY is output width 1.\n\n",
                "XVALID indicates that the request is pending and XREADY indicates that the subordinate can accept the transfer.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XVALID"
                && matches!(
                    hint.source_kind,
                    super::SignalSemanticHintSourceKind::ProseStatement
                )
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeValidLike)
                && hint.automation_confidence == AutomationConfidence::Low
                && !hint.supporting_statement_ids.is_empty()
        }));
        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XREADY"
                && matches!(
                    hint.source_kind,
                    super::SignalSemanticHintSourceKind::ProseStatement
                )
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeReadyLike)
                && hint.automation_confidence == AutomationConfidence::Low
                && !hint.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn acknowledged_event_prose_does_not_create_ready_like_hint() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("acknowledged_event_not_ready.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# DVM Channel\n",
                "Signal CRVALID is input width 1.\n\n",
                "CRVALID is asserted to indicate that the Manager has acknowledged the DVM message.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_crvalid_desc".to_string(),
            asset_id: "asset_crvalid_desc".to_string(),
            page_id: None,
            caption_text: Some("DVM response handshake".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("CRVALID", false),
                make_table_cell("DVMmessage response valid indicator.", false),
            ]],
            row_count: 1,
            col_count: 2,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        let crvalid_hints: Vec<_> = evidence_ir
            .signal_semantic_hints
            .iter()
            .filter(|hint| hint.signal_name == "CRVALID")
            .collect();
        assert!(!crvalid_hints.is_empty());
        assert!(crvalid_hints.iter().any(|hint| {
            hint.semantic_tags
                .contains(&super::SignalSemanticTag::HandshakeValidLike)
        }));
        assert!(!crvalid_hints.iter().any(|hint| {
            hint.semantic_tags
                .contains(&super::SignalSemanticTag::HandshakeReadyLike)
        }));

        Ok(())
    }

    #[test]
    fn dot_leader_contents_lines_do_not_create_semantic_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("contents_line_not_semantic.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal AWAKEUP is input width 1.\n\n",
                "| | A14.1 | Interface A14.1.1 | gating with Valid-Ready transport . . . . . . AWAKEUP rules and recommendations . . . . | 223 223 |\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(
            evidence_ir
                .signal_semantic_hints
                .iter()
                .all(|hint| hint.signal_name != "AWAKEUP"),
            "table-of-contents style dot-leader lines must not create semantic hints: {:?}",
            evidence_ir.signal_semantic_hints
        );

        Ok(())
    }

    #[test]
    fn alias_grounded_prose_descriptions_produce_semantic_handshake_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("alias_grounded_handshake_hints.md");
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
        assert_eq!(evidence_ir.signal_alias_map.len(), 2);
        assert_eq!(
            evidence_ir.signal_alias_map.get("request phase"),
            Some(&"XREQ".to_string())
        );
        assert_eq!(
            evidence_ir.signal_alias_map.get("accept phase"),
            Some(&"XACK".to_string())
        );
        evidence_ir.refresh_signal_semantic_hints()?;

        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XREQ"
                && matches!(
                    hint.source_kind,
                    super::SignalSemanticHintSourceKind::AliasGroundedProseStatement
                )
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeValidLike)
                && hint.automation_confidence == AutomationConfidence::Low
                && !hint.supporting_statement_ids.is_empty()
        }));
        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XACK"
                && matches!(
                    hint.source_kind,
                    super::SignalSemanticHintSourceKind::AliasGroundedProseStatement
                )
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeReadyLike)
                && hint.automation_confidence == AutomationConfidence::Low
                && !hint.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn explicit_signal_mentions_outrank_alias_grounding_for_same_statement() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("direct_signal_outranks_alias.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal XREQ is input width 1.\n\n",
                "The request phase XREQ indicates that address and control information are valid for transfer.\n",
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
        evidence_ir.refresh_signal_semantic_hints()?;

        let xreq_hints: Vec<_> = evidence_ir
            .signal_semantic_hints
            .iter()
            .filter(|hint| hint.signal_name == "XREQ")
            .collect();
        assert_eq!(xreq_hints.len(), 1);
        assert!(matches!(
            xreq_hints[0].source_kind,
            super::SignalSemanticHintSourceKind::ProseStatement
        ));
        assert!(
            xreq_hints[0]
                .semantic_tags
                .contains(&super::SignalSemanticTag::HandshakeValidLike)
        );

        Ok(())
    }

    #[test]
    fn visual_captions_produce_semantic_handshake_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("caption_grounded_handshake_hints.md");
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
            caption_text: Some("Figure 2: XACK ready timing.".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: None,
            diagram_kind: crate::ir::source::DiagramKind::TimingDiagram,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XREQ"
                && matches!(
                    hint.source_kind,
                    super::SignalSemanticHintSourceKind::VisualCaption
                )
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeValidLike)
                && !hint.supporting_visual_evidence_ids.is_empty()
                && hint.supporting_statement_ids.is_empty()
                && hint.automation_confidence == AutomationConfidence::Low
        }));
        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XACK"
                && matches!(
                    hint.source_kind,
                    super::SignalSemanticHintSourceKind::VisualCaption
                )
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeReadyLike)
                && !hint.supporting_visual_evidence_ids.is_empty()
                && hint.supporting_statement_ids.is_empty()
                && hint.automation_confidence == AutomationConfidence::Low
        }));

        Ok(())
    }

    #[test]
    fn multi_signal_visual_captions_produce_per_signal_semantic_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("caption_multi_signal_handshake_hints.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal XVALID is input width 1.\n\n",
                "Signal XREADY is input width 1.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "figure_xvalid_xready".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some(
                "Figure 1: XVALID indicates that the request is pending and XREADY indicates that the subordinate can accept the transfer."
                    .to_string(),
            ),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: None,
            diagram_kind: crate::ir::source::DiagramKind::TimingDiagram,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XVALID"
                && matches!(
                    hint.source_kind,
                    super::SignalSemanticHintSourceKind::VisualCaption
                )
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeValidLike)
                && hint.automation_confidence == AutomationConfidence::Low
                && hint.supporting_statement_ids.is_empty()
        }));
        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XREADY"
                && matches!(
                    hint.source_kind,
                    super::SignalSemanticHintSourceKind::VisualCaption
                )
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeReadyLike)
                && hint.automation_confidence == AutomationConfidence::Low
                && hint.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn vlm_timing_annotations_produce_semantic_handshake_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("vlm_handshake_hints.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            concat!("# Channel\n", "Signal XACK is input width 1.\n",),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "figure_vlm_xack".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 3: Transfer timing".to_string()),
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

        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XACK"
                && matches!(
                    hint.source_kind,
                    super::SignalSemanticHintSourceKind::VlmTimingDiagramAnnotation
                )
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeReadyLike)
                && !hint.supporting_visual_evidence_ids.is_empty()
                && hint.supporting_statement_ids.is_empty()
                && hint.automation_confidence == AutomationConfidence::Low
        }));

        Ok(())
    }

    #[test]
    fn conflicting_semantic_hints_are_surfaced_explicitly() -> Result<()> {
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

        assert_eq!(evidence_ir.signal_semantic_conflicts.len(), 1);
        let conflict = &evidence_ir.signal_semantic_conflicts[0];
        assert_eq!(conflict.signal_name, "XCTRL");
        assert_eq!(conflict.automation_confidence, AutomationConfidence::Medium);
        assert!(conflict.observations.iter().any(|observation| {
            observation
                .semantic_tags
                .contains(&super::SignalSemanticTag::HandshakeValidLike)
                && observation.supporting_statement_ids.is_empty()
        }));
        assert!(conflict.observations.iter().any(|observation| {
            observation
                .semantic_tags
                .contains(&super::SignalSemanticTag::HandshakeReadyLike)
                && !observation.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    // ── Form 2: signal alias learning ───────────────────────────────────

    #[test]
    fn apply_alias_reclassification_reclassifies_normative_statement_with_alias() -> Result<()> {
        use super::EvidenceModality;
        // Seed the alias map with "address bus" → "HADDR", then call
        // apply_alias_reclassification() and verify the NormativeStatement is
        // reclassified to SignalValueConstraint and a constraint record is created.
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(&source, "# Protocol\nSome content.\n")?;
        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        // Inject a NormativeStatement whose subject is a prose alias.
        let alias_sentence = "The address bus shall remain stable when HREADY is LOW";
        evidence_ir
            .extracted_statements
            .push(crate::ir::evidence::ExtractedStatement {
                statement_id: "stmt_alias_test".to_string(),
                text: alias_sentence.to_string(),
                class: StatementClass::NormativeStatement,
                modality: EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            });

        // Seed the alias map as if it had been learned by a previous nlp-enrich run.
        evidence_ir
            .signal_alias_map
            .insert("address bus".to_string(), "HADDR".to_string());

        let mut counter = 1usize;
        let (reclassified, new_records) = evidence_ir.apply_alias_reclassification(&mut counter);

        assert_eq!(reclassified, 1, "one statement should be reclassified");
        assert_eq!(
            new_records.len(),
            1,
            "one constraint record should be created"
        );
        assert_eq!(new_records[0].subject_signal, "HADDR");
        assert_eq!(new_records[0].constraint_id, "alias2_sigcon_0002");
        assert!(!new_records[0].negated);
        // The statement class must be updated in place.
        let updated_stmt = evidence_ir
            .extracted_statements
            .iter()
            .find(|s| s.text == alias_sentence)
            .unwrap();
        assert_eq!(
            updated_stmt.class,
            StatementClass::SignalValueConstraint,
            "statement must be reclassified from NormativeStatement to SignalValueConstraint"
        );

        Ok(())
    }

    #[test]
    fn apply_alias_reclassification_sets_negated_from_lowered_text() -> Result<()> {
        // negated is computed from lowered text: must be true when lowered
        // contains " not " or "cannot". Test with "cannot".
        use super::EvidenceModality;
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(&source, "# Protocol\nSome content.\n")?;
        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        let alias_sentence = "The address bus cannot be changed when HREADY is LOW";
        evidence_ir
            .extracted_statements
            .push(crate::ir::evidence::ExtractedStatement {
                statement_id: "stmt_negated_test".to_string(),
                text: alias_sentence.to_string(),
                class: StatementClass::NormativeStatement,
                modality: EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            });
        evidence_ir
            .signal_alias_map
            .insert("address bus".to_string(), "HADDR".to_string());

        let mut counter = 1usize;
        let (_reclassified, new_records) = evidence_ir.apply_alias_reclassification(&mut counter);

        assert_eq!(new_records.len(), 1);
        assert!(
            new_records[0].negated,
            "negated must be true when lowered text contains 'cannot'"
        );

        Ok(())
    }

    #[test]
    fn carry_forward_skips_a_pure_deterministic_existing_artifact() -> Result<()> {
        // Idempotency: re-building over an artifact that carries NO LLM-enriched (Nlp-tier)
        // facts must not carry its (now-stale) deterministic facts forward — the fresh build
        // supersedes it. (A V2 re-measurement accumulated stale constraints exactly this way:
        // 13 → 21 with duplicate ids.)
        use crate::ir::source::{
            AutomationConfidence, SignalConstraintKind, SignalConstraintRecord,
        };
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(&source, "# Protocol\nSome content.\n")?;
        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        // Build once, inject a stale deterministic constraint, persist (no Nlp provenance).
        let mut first = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        first.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "stale_sigcon".to_string(),
            subject_signal: "GHOST".to_string(),
            constraint_kind: SignalConstraintKind::MustBeStable,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "stale".to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        });
        first.write_to_disk()?;

        // Re-build over the existing artifact: the stale constraint must NOT survive.
        let second = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        assert!(
            !second
                .signal_constraints
                .iter()
                .any(|c| c.constraint_id == "stale_sigcon"),
            "a pure-deterministic existing artifact's stale constraint must not be carried forward"
        );
        Ok(())
    }

    #[test]
    fn apply_alias_reclassification_skips_already_covered_sentences() {
        // A sentence whose source_text already appears in signal_constraints must not
        // generate a duplicate record even if its class is still NormativeStatement.
        use super::EvidenceModality;
        use crate::ir::source::{
            AutomationConfidence, SignalConstraintKind, SignalConstraintRecord,
        };
        let tempdir = tempfile::tempdir().unwrap();
        let source = tempdir.path().join("s.md");
        std::fs::write(&source, "# P\nContent.\n").unwrap();
        let sib = tempdir.path().join("src_ir");
        let eib = tempdir.path().join("ev_ir");
        let source_ir = SourceIr::build(&source, &sib).unwrap();
        source_ir.write_to_disk().unwrap();
        let mut ev = EvidenceIr::build(&source_ir.artifact_layout.source_ir_path, &eib).unwrap();

        let covered_text = "The address bus shall remain stable";
        ev.extracted_statements
            .push(crate::ir::evidence::ExtractedStatement {
                statement_id: "stmt_covered".to_string(),
                text: covered_text.to_string(),
                class: StatementClass::NormativeStatement,
                modality: EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            });
        // Pre-populate with a constraint whose source_text matches.
        ev.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_existing".to_string(),
            subject_signal: "HADDR".to_string(),
            constraint_kind: SignalConstraintKind::MustBeStable,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: covered_text.to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        });
        ev.signal_alias_map
            .insert("address bus".to_string(), "HADDR".to_string());

        let mut counter = 1usize;
        let (reclassified, new_records) = ev.apply_alias_reclassification(&mut counter);
        assert_eq!(
            reclassified, 0,
            "already-covered sentence must not be reclassified"
        );
        assert!(new_records.is_empty());
    }

    #[test]
    fn dedup_loopback_records_removes_duplicate_constraints_and_rules() {
        use crate::ir::source::{
            AutomationConfidence, ConditionalRuleRecord, SignalConstraintKind,
            SignalConstraintRecord,
        };

        let tempdir = tempfile::tempdir().unwrap();
        let source = tempdir.path().join("s.md");
        std::fs::write(&source, "# P\nContent.\n").unwrap();
        let sib = tempdir.path().join("src_ir");
        let eib = tempdir.path().join("ev_ir");
        let source_ir = SourceIr::build(&source, &sib).unwrap();
        source_ir.write_to_disk().unwrap();
        let mut ev = EvidenceIr::build(&source_ir.artifact_layout.source_ir_path, &eib).unwrap();

        let sig = SignalConstraintRecord {
            constraint_id: "dup_sig_1".to_string(),
            subject_signal: "HADDR".to_string(),
            constraint_kind: SignalConstraintKind::MustBeStable,
            target_value: None,
            condition_text: Some("while HREADY is LOW".to_string()),
            negated: false,
            source_text: "HADDR must remain stable while HREADY is LOW.".to_string(),
            supporting_statement_ids: vec!["statement_0001".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        };
        ev.signal_constraints.push(sig.clone());
        ev.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "dup_sig_2".to_string(),
            ..sig
        });

        let rule = ConditionalRuleRecord {
            rule_id: "dup_rule_1".to_string(),
            antecedent_text: "AWVALID and AWREADY are asserted".to_string(),
            consequent_signal: Some("BVALID".to_string()),
            consequent_action: "must_be_asserted".to_string(),
            source_text: "When AWVALID and AWREADY are asserted, BVALID must be asserted."
                .to_string(),
            supporting_statement_ids: vec!["statement_0002".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        };
        ev.conditional_rules.push(rule.clone());
        ev.conditional_rules.push(ConditionalRuleRecord {
            rule_id: "dup_rule_2".to_string(),
            ..rule
        });

        assert!(ev.dedup_loopback_records());
        assert_eq!(ev.signal_constraints.len(), 1);
        assert_eq!(ev.conditional_rules.len(), 1);
    }

    // ── Layer A: section-aware boilerplate suppression ────────────────────

    #[test]
    fn is_boilerplate_section_title_matches_common_boilerplate_headings() {
        use super::is_boilerplate_section_title;

        // Typical boilerplate sections in chip specs.
        assert!(is_boilerplate_section_title("1 Introduction"));
        assert!(is_boilerplate_section_title("Introduction"));
        assert!(is_boilerplate_section_title("Revision History"));
        assert!(is_boilerplate_section_title("Copyright and Legal Notices"));
        assert!(is_boilerplate_section_title("Normative References"));
        assert!(is_boilerplate_section_title("Informative References"));
        assert!(is_boilerplate_section_title("Glossary"));
        assert!(is_boilerplate_section_title("Acronyms and Abbreviations"));
        assert!(is_boilerplate_section_title("Bibliography"));
        assert!(is_boilerplate_section_title("Terms and Definitions"));
        assert!(is_boilerplate_section_title("About this Document"));
        assert!(is_boilerplate_section_title("Scope"));

        // Behavioral / normative sections must NOT be suppressed.
        assert!(!is_boilerplate_section_title("Signal Description"));
        assert!(!is_boilerplate_section_title("Transfer Types"));
        assert!(!is_boilerplate_section_title("Protocol Operation"));
        assert!(!is_boilerplate_section_title("Bus Arbitration"));
        assert!(!is_boilerplate_section_title("Register Map"));
    }

    #[test]
    fn normative_sentence_in_introduction_section_is_suppressed_to_source_fact() -> Result<()> {
        // Layer A: a sentence with SHALL/MUST in an Introduction heading is a
        // legal compliance statement, not a hardware behavioral constraint.
        // It must be downgraded to SourceFact so it doesn't inflate NormativeStatement counts.
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        // The intro sentence has "shall" → would normally be NormativeStatement, but must
        // be suppressed to SourceFact by Layer A (boilerplate section).
        // The protocol sentence has "shall not" and no value-binding phrase → NormativeStatement.
        // (Note: sentences like "HREADY shall be asserted" are SignalValueConstraint, which is
        // more specific than NormativeStatement and is unaffected by Layer A.)
        fs::write(
            &source,
            concat!(
                "# Introduction\n",
                "Implementations shall comply with this version of the specification.\n",
                "\n",
                "# Protocol Rules\n",
                "Burst transfers shall not be interrupted by intervening requests.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        let intro_stmt = evidence_ir
            .extracted_statements
            .iter()
            .find(|s| s.text.contains("comply"))
            .expect("should find the intro normative sentence");
        let protocol_stmt = evidence_ir
            .extracted_statements
            .iter()
            .find(|s| s.text.contains("interrupted"))
            .expect("should find the protocol normative sentence");

        assert_eq!(
            intro_stmt.class,
            StatementClass::SourceFact,
            "Introduction normative sentence must be suppressed to SourceFact (Layer A)"
        );
        assert_eq!(
            protocol_stmt.class,
            StatementClass::NormativeStatement,
            "Protocol section normative sentence must remain NormativeStatement"
        );

        Ok(())
    }

    #[test]
    fn links_caption_and_figure_reference_into_visual_evidence() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("timing.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let asset_path = tempdir.path().join("assets").join("figure-0001.png");

        fs::create_dir_all(asset_path.parent().expect("asset parent should exist"))?;
        fs::write(&asset_path, b"png")?;
        fs::write(
            &source,
            "# Timing\nFigure 1: VALID/READY timing behavior.\n\n![Image](assets/figure-0001.png)\n\nThe handshake is shown in Figure 1.\n",
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "figure_0001".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0008".to_string()),
            image_path: Some(asset_path.clone()),
            caption_text: Some("Figure 1: VALID/READY timing behavior.".to_string()),
            caption_source_path: None,
            source_ref: Some("#/pictures/0".to_string()),
            placeholder_text: None,
            note: None,
            diagram_kind: crate::ir::source::DiagramKind::Unknown,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;

        assert_eq!(evidence_ir.visual_evidence.len(), 1);
        assert_eq!(evidence_ir.evidence_links.len(), 2);
        assert!(
            evidence_ir
                .evidence_links
                .iter()
                .any(|link| link.relation == EvidenceLinkKind::Describes)
        );
        assert!(
            evidence_ir
                .evidence_links
                .iter()
                .any(|link| link.relation == EvidenceLinkKind::Cites)
        );
        assert!(
            evidence_ir
                .evidence_links
                .iter()
                .all(|link| !link.from_evidence_span_id.is_empty())
        );
        assert!(
            evidence_ir
                .evidence_links
                .iter()
                .all(|link| !link.to_visual_evidence_id.is_empty())
        );
        assert_eq!(
            evidence_ir.visual_evidence[0]
                .figure_reference_text
                .as_deref(),
            Some("Figure 1")
        );
        assert!(
            evidence_ir.visual_evidence[0]
                .observations
                .iter()
                .any(|observation| {
                    observation.kind == VisualObservationKind::Caption
                        && observation.text == "Figure 1: VALID/READY timing behavior."
                        && observation.automation_confidence == AutomationConfidence::High
                        && !observation.supporting_span_ids.is_empty()
                })
        );
        assert!(
            evidence_ir.visual_evidence[0]
                .observations
                .iter()
                .any(|observation| {
                    observation.kind == VisualObservationKind::FigureReference
                        && observation.text == "Figure 1"
                        && observation.automation_confidence == AutomationConfidence::Medium
                        && !observation.supporting_span_ids.is_empty()
                })
        );
        assert!(evidence_ir.extracted_statements.iter().any(|statement| {
            !statement.related_visual_evidence_ids.is_empty()
                && !statement.evidence_span_ids.is_empty()
                && statement
                    .text
                    .contains("The handshake is shown in Figure 1.")
        }));
        assert!(
            evidence_ir
                .artifact_layout
                .evidence_ir_path
                .ends_with("generated/evidence_ir/timing/evidence_ir.json")
        );

        Ok(())
    }

    // --- is_image_line ---

    #[test]
    fn is_image_line_detects_image_markdown() {
        assert!(is_image_line("![alt text](path.png)"));
    }

    #[test]
    fn is_image_line_rejects_plain_text() {
        assert!(!is_image_line("plain text"));
    }

    #[test]
    fn is_image_line_rejects_empty() {
        assert!(!is_image_line(""));
    }

    // --- is_standalone_markdown_block ---

    #[test]
    fn is_standalone_markdown_block_detects_list_items() {
        assert!(is_standalone_markdown_block("- list item"));
        assert!(is_standalone_markdown_block("* star item"));
        assert!(is_standalone_markdown_block("+ plus item"));
    }

    #[test]
    fn is_standalone_markdown_block_detects_table_row() {
        assert!(is_standalone_markdown_block("| col1 | col2 |"));
    }

    #[test]
    fn is_standalone_markdown_block_detects_numbered_list() {
        assert!(is_standalone_markdown_block("1. first item"));
        assert!(is_standalone_markdown_block("42. answer"));
    }

    #[test]
    fn is_standalone_markdown_block_rejects_plain_text() {
        assert!(!is_standalone_markdown_block("plain text"));
    }

    // --- is_signal_name_char ---

    #[test]
    fn is_signal_name_char_accepts_alphanumeric_and_underscore() {
        assert!(is_signal_name_char('A'));
        assert!(is_signal_name_char('z'));
        assert!(is_signal_name_char('0'));
        assert!(is_signal_name_char('_'));
    }

    #[test]
    fn is_signal_name_char_rejects_special_chars() {
        assert!(!is_signal_name_char('-'));
        assert!(!is_signal_name_char('.'));
        assert!(!is_signal_name_char(' '));
        assert!(!is_signal_name_char('['));
    }

    // --- is_tie_off_actor_text ---

    #[test]
    fn is_tie_off_actor_text_detects_variants() {
        assert!(is_tie_off_actor_text("tie off"));
        assert!(is_tie_off_actor_text("tieoff"));
        assert!(is_tie_off_actor_text("TIE OFF"));
        assert!(is_tie_off_actor_text("TieOff"));
    }

    #[test]
    fn is_tie_off_actor_text_rejects_others() {
        assert!(!is_tie_off_actor_text("master"));
        assert!(!is_tie_off_actor_text(""));
    }

    // --- is_abstract_transport_signal_token ---

    #[test]
    fn is_abstract_transport_signal_token_detects_known_tokens() {
        assert!(is_abstract_transport_signal_token("VALID"));
        assert!(is_abstract_transport_signal_token("READY"));
        assert!(is_abstract_transport_signal_token("PENDING"));
        assert!(is_abstract_transport_signal_token("CRDT"));
    }

    #[test]
    fn is_abstract_transport_signal_token_rejects_unknown() {
        assert!(!is_abstract_transport_signal_token("CLOCK"));
        assert!(!is_abstract_transport_signal_token("data"));
        assert!(!is_abstract_transport_signal_token(""));
    }

    // --- is_abstract_transport_actor_term ---

    #[test]
    fn is_abstract_transport_actor_term_detects_known_terms() {
        assert!(is_abstract_transport_actor_term("tx"));
        assert!(is_abstract_transport_actor_term("rx"));
        assert!(is_abstract_transport_actor_term("transmitter"));
        assert!(is_abstract_transport_actor_term("receiver"));
    }

    #[test]
    fn is_abstract_transport_actor_term_rejects_unknown() {
        assert!(!is_abstract_transport_actor_term("master"));
        assert!(!is_abstract_transport_actor_term(""));
    }

    // --- diagram_kind_key ---

    #[test]
    fn diagram_kind_key_returns_correct_strings() {
        assert_eq!(
            diagram_kind_key(DiagramKind::TimingDiagram),
            "timing_diagram"
        );
        assert_eq!(
            diagram_kind_key(DiagramKind::StateMachineDiagram),
            "state_machine_diagram"
        );
        assert_eq!(diagram_kind_key(DiagramKind::BlockDiagram), "block_diagram");
        assert_eq!(
            diagram_kind_key(DiagramKind::RegisterBitfield),
            "register_bitfield"
        );
        assert_eq!(diagram_kind_key(DiagramKind::TruthTable), "truth_table");
        assert_eq!(diagram_kind_key(DiagramKind::FlowChart), "flow_chart");
        assert_eq!(diagram_kind_key(DiagramKind::Unknown), "unknown");
    }

    // --- parse_encoding_numeric_literal ---

    #[test]
    fn parse_encoding_numeric_literal_decimal() {
        assert_eq!(parse_encoding_numeric_literal("42"), Some(42));
    }

    #[test]
    fn parse_encoding_numeric_literal_binary_prefix() {
        assert_eq!(parse_encoding_numeric_literal("0b1010"), Some(10));
    }

    #[test]
    fn parse_encoding_numeric_literal_hex_prefix() {
        assert_eq!(parse_encoding_numeric_literal("0xFF"), Some(255));
    }

    #[test]
    fn parse_encoding_numeric_literal_verilog_binary() {
        assert_eq!(parse_encoding_numeric_literal("3'b101"), Some(5));
    }

    #[test]
    fn parse_encoding_numeric_literal_verilog_hex() {
        assert_eq!(parse_encoding_numeric_literal("8'hFF"), Some(255));
    }

    #[test]
    fn parse_encoding_numeric_literal_trims_brackets() {
        assert_eq!(parse_encoding_numeric_literal("[42]"), Some(42));
    }

    #[test]
    fn parse_encoding_numeric_literal_empty() {
        assert_eq!(parse_encoding_numeric_literal(""), None);
    }

    #[test]
    fn parse_encoding_numeric_literal_invalid() {
        assert_eq!(parse_encoding_numeric_literal("not_a_number"), None);
    }

    // --- contains_any ---

    #[test]
    fn contains_any_finds_match() {
        assert!(contains_any("hello world", &["hello", "foo"]));
    }

    #[test]
    fn contains_any_no_match() {
        assert!(!contains_any("hello world", &["foo", "bar"]));
    }

    #[test]
    fn contains_any_empty_candidates() {
        assert!(!contains_any("hello world", &[]));
    }

    // --- contains_reference_token ---

    #[test]
    fn contains_reference_token_matches_word_boundary() {
        assert!(contains_reference_token(
            "see Figure 1 for details",
            "Figure 1"
        ));
    }

    #[test]
    fn contains_reference_token_rejects_partial_word() {
        assert!(!contains_reference_token(
            "see Figure10 for details",
            "Figure 1"
        ));
    }

    #[test]
    fn contains_reference_token_at_start_of_text() {
        assert!(contains_reference_token("Figure 1 shows", "Figure 1"));
    }

    #[test]
    fn contains_reference_token_at_end_of_text() {
        assert!(contains_reference_token("see Figure 1", "Figure 1"));
    }

    // --- numbered_list_prefix ---

    #[test]
    fn numbered_list_prefix_detects_simple() {
        assert!(numbered_list_prefix("1. item"));
        assert!(numbered_list_prefix("42. answer"));
    }

    #[test]
    fn numbered_list_prefix_rejects_missing_dot() {
        assert!(!numbered_list_prefix("1 item"));
    }

    #[test]
    fn numbered_list_prefix_rejects_no_space_after_dot() {
        assert!(!numbered_list_prefix("1.item"));
    }

    #[test]
    fn numbered_list_prefix_rejects_empty() {
        assert!(!numbered_list_prefix(""));
    }

    // --- infer_signal_direction_from_description_prose ---

    #[test]
    fn description_prose_direction_maps_transmitter_to_output_receiver_to_input() {
        use super::infer_signal_direction_from_description_prose as infer;
        assert_eq!(
            infer(
                "Request Flit Valid. The transmitter sets this signal HIGH to indicate validity."
            ),
            Some("output"),
            "transmitter-driven → output"
        );
        assert_eq!(
            infer(
                "Request L-Credit Valid. The receiver sets this signal HIGH to return a credit to a transmitter."
            ),
            Some("input"),
            "receiver-driven → input (note: object mention of 'transmitter' must not flip it)"
        );
        // No driver verb stated → honest residual.
        assert_eq!(
            infer(
                "Request Flit Pending. Early indication that a request flit could be transmitted."
            ),
            None
        );
        // "driven by the <actor>" / "set by the <actor>" phrasings also resolve.
        assert_eq!(
            infer("This signal is driven by the receiver."),
            Some("input")
        );
        assert_eq!(
            infer("This signal is set by the transmitter."),
            Some("output")
        );
    }

    // --- looks_like_encoding_literal ---

    #[test]
    fn looks_like_encoding_literal_detects_binary() {
        assert!(looks_like_encoding_literal("0b1010"));
    }

    #[test]
    fn looks_like_encoding_literal_detects_hex() {
        assert!(looks_like_encoding_literal("0xFF"));
    }

    #[test]
    fn looks_like_encoding_literal_detects_verilog_binary() {
        assert!(looks_like_encoding_literal("3'b101"));
    }

    #[test]
    fn looks_like_encoding_literal_rejects_plain_text() {
        assert!(!looks_like_encoding_literal("hello"));
    }

    #[test]
    fn looks_like_encoding_literal_rejects_empty() {
        assert!(!looks_like_encoding_literal(""));
    }

    // --- looks_like_structural_contents_entry_for_semantic_hint ---

    #[test]
    fn looks_like_structural_contents_detects_toc() {
        assert!(looks_like_structural_contents_entry_for_semantic_hint(
            "Table of Contents"
        ));
    }

    #[test]
    fn looks_like_structural_contents_detects_dots() {
        assert!(looks_like_structural_contents_entry_for_semantic_hint(
            "1.0 Introduction . . . . . . 5"
        ));
    }

    #[test]
    fn looks_like_structural_contents_rejects_plain_text() {
        assert!(!looks_like_structural_contents_entry_for_semantic_hint(
            "Signal HADDR is input"
        ));
    }

    // --- is_hardware_signal_token ---

    #[test]
    fn is_hardware_signal_token_accepts_valid_signals() {
        assert!(is_hardware_signal_token("HADDR"));
        assert!(is_hardware_signal_token("HCLK"));
        assert!(is_hardware_signal_token("AWVALID"));
        assert!(is_hardware_signal_token("S_AXI_AWREADY"));
    }

    #[test]
    fn is_hardware_signal_token_rejects_lowercase() {
        assert!(!is_hardware_signal_token("haddr"));
    }

    #[test]
    fn is_hardware_signal_token_rejects_short() {
        assert!(!is_hardware_signal_token("A"));
    }

    #[test]
    fn is_hardware_signal_token_rejects_no_uppercase_letter() {
        assert!(!is_hardware_signal_token("123_456"));
    }

    #[test]
    fn is_hardware_signal_token_rejects_number_led_literals() {
        // WIRE-BASED-100.5j: binary/number literals lead with a digit (they pass the
        // uppercase-or-digit test via an embedded letter) — a value cell must not be a signal name.
        assert!(!is_hardware_signal_token("0B0"));
        assert!(!is_hardware_signal_token("0B1"));
        assert!(!is_hardware_signal_token("0X1F"));
        // A real signal that merely ends in a digit still passes (leads with a letter).
        assert!(is_hardware_signal_token("HSEL0"));
        assert!(is_hardware_signal_token("PSEL1"));
    }

    #[test]
    fn is_hardware_signal_token_rejects_empty() {
        assert!(!is_hardware_signal_token(""));
    }

    // --- is_signal_synthesis_non_signal ---

    #[test]
    fn is_signal_synthesis_non_signal_detects_roles() {
        assert!(is_signal_synthesis_non_signal("MANAGER"));
        assert!(is_signal_synthesis_non_signal("SUBORDINATE"));
        assert!(is_signal_synthesis_non_signal("INITIATOR"));
        assert!(is_signal_synthesis_non_signal("CLOCK"));
        assert!(is_signal_synthesis_non_signal("RESET"));
    }

    #[test]
    fn is_signal_synthesis_non_signal_rejects_common_english_words() {
        // WIRE-BASED-100.5k: common/description/logic words that pass `is_hardware_signal_token`
        // (all-uppercase) but are never signal names — leaked from scrambled AXI tables
        // ("Signal THE is width AWPROT, ARPROT").
        for w in [
            "THE",
            "WHEN",
            "AND",
            "HIGH",
            "LOW",
            "SECURE",
            "PHYSICAL",
            "INDICATES",
            "ASSERTED",
        ] {
            assert!(
                is_signal_synthesis_non_signal(w),
                "{w} must be rejected as a signal name"
            );
        }
    }

    #[test]
    fn is_signal_synthesis_non_signal_rejects_real_signals() {
        // Real signals must still pass (none of the added words collide with a real signal name).
        for s in ["HADDR", "AWVALID", "ASKSTOP", "BCOMP", "AWPROT", "RDATA"] {
            assert!(!is_signal_synthesis_non_signal(s), "{s} is a real signal");
        }
    }

    #[test]
    fn build_records_a_converged_convergence_report() {
        let tempdir = tempfile::tempdir().unwrap();
        let source = tempdir.path().join("conv.md");
        std::fs::write(
            &source,
            "# Spec\nSignal HADDR is input width 32.\nHADDR shall remain stable.\n",
        )
        .unwrap();
        let sib = tempdir.path().join("src_ir");
        let eib = tempdir.path().join("ev_ir");
        let source_ir = SourceIr::build(&source, &sib).unwrap();
        source_ir.write_to_disk().unwrap();
        let ev = EvidenceIr::build(&source_ir.artifact_layout.source_ir_path, &eib).unwrap();

        let report = ev
            .convergence_report
            .as_ref()
            .expect("build records a convergence report");
        // The monotone loop must stop on a fixpoint, not at the cap.
        assert!(report.converged, "simple spec must converge");
        assert!(report.passes_run >= 1);
        assert!(report.passes_run <= report.max_passes);
        // passes_run counts every executed pass, including the terminal one.
        assert_eq!(report.new_facts_per_pass.len(), report.passes_run);
        // The terminal pass (the one that broke the loop) discovered nothing new.
        assert_eq!(report.new_facts_per_pass.last().copied(), Some(0));
        // total is the sum of the per-pass counts.
        assert_eq!(
            report.total_new_facts,
            report.new_facts_per_pass.iter().sum::<usize>()
        );
    }

    #[test]
    fn signal_constraint_fact_key_normalizes_for_overlap() {
        use crate::ir::source::{
            AutomationConfidence, SignalConstraintKind, SignalConstraintRecord,
        };
        let mk = |id: &str, sig: &str, src: &str| SignalConstraintRecord {
            constraint_id: id.to_string(),
            subject_signal: sig.to_string(),
            constraint_kind: SignalConstraintKind::MustBeStable,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: src.to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        };
        // Same fact found by two extractors (different id/source/case) -> same key.
        let pattern = mk("sigcon_0001", "haddr", "the address bus shall be stable");
        let nlp = mk("nlp3_sigcon_0007", "HADDR", "HADDR must remain stable");
        assert_eq!(
            signal_constraint_fact_key(&pattern),
            signal_constraint_fact_key(&nlp),
            "the same constraint from different tiers must share a canonical key (overlap)"
        );
        // A different signal -> different key.
        let other = mk("sigcon_0002", "HWDATA", "HWDATA stable");
        assert_ne!(
            signal_constraint_fact_key(&pattern),
            signal_constraint_fact_key(&other)
        );
    }

    #[test]
    fn actor_signal_relation_fact_key_normalizes_for_overlap() {
        use crate::ir::source::{ActorSignalRelation, AutomationConfidence, RelationKind};
        let mk = |id: &str, actor: &str, sig: &str, rel: RelationKind| ActorSignalRelation {
            relation_id: id.to_string(),
            actor_name: actor.to_string(),
            signal_name: sig.to_string(),
            relation: rel,
            source_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        };
        // Same edge from two tiers (different id/case) -> same key.
        let pattern = mk("r14:001", "manager", "HTRANS", RelationKind::Drives);
        let llm = mk("r14:llm_007", "MANAGER", "htrans", RelationKind::Drives);
        assert_eq!(
            actor_signal_relation_fact_key(&pattern),
            actor_signal_relation_fact_key(&llm),
            "the same edge from different tiers must share a canonical key (overlap)"
        );
        // Different relation direction -> different key.
        let reads = mk("r14:002", "manager", "HTRANS", RelationKind::Reads);
        assert_ne!(
            actor_signal_relation_fact_key(&pattern),
            actor_signal_relation_fact_key(&reads)
        );
    }

    #[test]
    fn build_tags_pattern_fact_provenance_for_signal_constraints() {
        let tempdir = tempfile::tempdir().unwrap();
        let source = tempdir.path().join("prov.md");
        std::fs::write(
            &source,
            "# Spec\nSignal HADDR is input width 32.\nHADDR must remain stable until HREADY is HIGH.\n",
        )
        .unwrap();
        let sib = tempdir.path().join("src_ir");
        let eib = tempdir.path().join("ev_ir");
        let source_ir = SourceIr::build(&source, &sib).unwrap();
        source_ir.write_to_disk().unwrap();
        let ev = EvidenceIr::build(&source_ir.artifact_layout.source_ir_path, &eib).unwrap();

        // Every build-time fact-provenance entry is a Pattern-tier SignalConstraint,
        // and there is exactly one per produced signal constraint with a matching key.
        assert_eq!(ev.fact_provenance.len(), ev.signal_constraints.len());
        assert!(
            ev.fact_provenance
                .iter()
                .all(|p| p.producer == ExtractorTier::Pattern
                    && p.fact_kind == FactKind::SignalConstraint)
        );
        for c in &ev.signal_constraints {
            let key = signal_constraint_fact_key(c);
            assert!(
                ev.fact_provenance.iter().any(|p| p.canonical_key == key),
                "every pattern constraint must have a provenance entry"
            );
        }
    }
}

#[cfg(test)]
mod wire_based_100_5b {
    //! WIRE-BASED-100.5b — AHB constraint extraction is correct on current code:
    //! `must not change` carries no redundant `negated` (the kind encodes it), validity
    //! obligations resolve to `must_be_value VALID`, and a `The following signals … when
    //! <cond>` list-introducer yields NO constraint (the condition signal is not a subject).
    use super::*;

    fn run(text: &str) -> Vec<SignalConstraintRecord> {
        let stmts = vec![ExtractedStatement {
            statement_id: "s".into(),
            class: StatementClass::SignalValueConstraint,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];
        let mut counter = 0usize;
        extract_signal_constraints(&stmts, &mut counter)
    }

    #[test]
    fn must_not_change_carries_no_redundant_negated() {
        let recs = run(
            "- The HAUSER signal must not change between cycles when HREADY is LOW, unless HRESP signal is ERROR.",
        );
        let r: Vec<_> = recs
            .iter()
            .filter(|r| r.subject_signal == "HAUSER")
            .collect();
        assert_eq!(r.len(), 1, "one HAUSER constraint, got {recs:?}");
        assert!(matches!(
            r[0].constraint_kind,
            SignalConstraintKind::MustNotChange
        ));
        assert!(
            !r[0].negated,
            "MustNotChange already encodes the negation — negated must be false, not a double negative"
        );
    }

    #[test]
    fn validity_obligation_resolves_to_must_be_value_valid() {
        let recs =
            run("- The HWUSER signal must be valid during the data phase of a write transfer.");
        let r: Vec<_> = recs
            .iter()
            .filter(|r| r.subject_signal == "HWUSER")
            .collect();
        assert_eq!(r.len(), 1, "one HWUSER constraint, got {recs:?}");
        assert!(
            matches!(&r[0].constraint_kind, SignalConstraintKind::MustBeValue { value } if value == "VALID"),
            "got {:?}",
            r[0].constraint_kind
        );
        assert!(!r[0].negated);
    }

    #[test]
    fn list_introducer_condition_signal_is_not_a_subject() {
        // The condition signal (HTRANS / HREADY / HRESP) must never become the constraint
        // subject; the constrained signals are the following list items, not the condition.
        for text in [
            "The following signals must be valid when HTRANS is not IDLE:",
            "The following signals must be valid in the data phase of a write transaction when HREADY is HIGH and HRESP is LOW:",
        ] {
            let recs = run(text);
            assert!(
                recs.is_empty(),
                "list-introducer must yield no constraint, got {recs:?} for {text:?}"
            );
        }
    }
}

#[cfg(test)]
mod wire_based_100_5g {
    //! WIRE-BASED-100.5g — pronoun-subject anaphora: a bare "it"/"they" subject resolves to the
    //! clause's main actor, not a nearer noun (fixes AHB `(address, drives, HREADYOUT)` and
    //! `(response it, drives, HRESP)` → `Subordinate`).
    use super::*;
    fn run(text: &str) -> Vec<ActorSignalRelation> {
        let stmts = vec![ExtractedStatement {
            statement_id: "s".into(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }];
        let mut known = std::collections::HashSet::new();
        for s in ["HREADYOUT", "HRESP"] {
            known.insert(s.to_string());
        }
        extract_actor_signal_relations(&stmts, &known)
    }
    fn drives(rels: &[ActorSignalRelation], sig: &str) -> Vec<String> {
        rels.iter()
            .filter(|r| r.signal_name == sig && matches!(r.relation, RelationKind::Drives))
            .map(|r| r.actor_name.clone())
            .collect()
    }
    #[test]
    fn anaphora_it_resolves_to_main_actor_not_nearer_noun() {
        let rels = run(
            "After the Subordinate has sampled the address and control it can start to drive the appropriate HREADYOUT response.",
        );
        let actors = drives(&rels, "HREADYOUT");
        assert!(
            actors.contains(&"Subordinate".to_string()),
            "the 'it' subject must resolve to Subordinate, got {actors:?}"
        );
        assert!(
            !actors.iter().any(|a| a.eq_ignore_ascii_case("address")),
            "the noun 'address' must not be the actor, got {actors:?}"
        );
    }
    #[test]
    fn anaphora_it_must_drive_resolves_to_subordinate() {
        let rels = run(
            "When a Subordinate inserts a number of wait states prior to completing the response, it must drive HRESP to OKAY.",
        );
        let actors = drives(&rels, "HRESP");
        assert!(
            actors.contains(&"Subordinate".to_string()),
            "expected Subordinate, got {actors:?}"
        );
        assert!(
            !actors
                .iter()
                .any(|a| a.to_ascii_lowercase().contains("response")),
            "the garbage phrase 'response it' must not be the actor, got {actors:?}"
        );
    }
    #[test]
    fn non_pronoun_subject_is_unchanged() {
        // Guard: a normal noun subject still extracts as before (no regression).
        let rels = run("The Subordinate drives HRESP during the response phase.");
        assert!(drives(&rels, "HRESP").contains(&"Subordinate".to_string()));
    }
}

#[cfg(test)]
mod wire_based_100_5h {
    //! WIRE-BASED-100.5h — content-based name-column detection: a signal table whose body is
    //! rotated (Name column last, Width/Destination shifted) still yields declarations. This is
    //! AHB table_0009's shape (the sole source of HREADY).
    use super::*;
    use crate::ir::source::{StructuredTableCellRecord, StructuredTableRecord};

    fn cell(t: &str) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: t.to_string(),
            row_span: 1,
            col_span: 1,
            is_header: false,
        }
    }
    fn row(cells: &[&str]) -> Vec<StructuredTableCellRecord> {
        cells.iter().map(|t| cell(t)).collect()
    }

    #[test]
    fn rotated_signal_table_extracts_name_from_last_column() {
        // header says Name|Destination|Width|Description, but the body is rotated so the name
        // is in the LAST column (the AHB table_0009 pattern).
        let table = StructuredTableRecord {
            table_id: "table_0009".to_string(),
            asset_id: "asset_0009".to_string(),
            page_id: None,
            caption_text: None,
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![row(&["Name", "Destination", "Width", "Description"])],
            body_rows: vec![
                row(&[
                    "Manager",
                    "1",
                    "When HIGH, the HREADY signal indicates",
                    "HREADY",
                ]),
                row(&["Manager", "1", "Transfer response", "HRESP"]),
                row(&["Manager", "DATA_WIDTH", "Read data bus", "HRDATA"]),
            ],
            row_count: 3,
            col_count: 4,
        };
        let mut counter = 0usize;
        let mut prov = Vec::new();
        let stmts = synthesize_signal_declarations(
            &table,
            SectionKind::Unknown,
            "",
            &mut counter,
            None,
            &mut prov,
        );
        let names: Vec<&str> = stmts
            .iter()
            .filter_map(|s| s.text.strip_prefix("Signal "))
            .filter_map(|s| s.split_whitespace().next())
            .collect();
        assert!(
            names.contains(&"HREADY"),
            "rotated table must yield HREADY, got {names:?}"
        );
        assert!(names.contains(&"HRESP") && names.contains(&"HRDATA"));
        assert!(
            !names.iter().any(|n| n.eq_ignore_ascii_case("MANAGER")),
            "the Destination actor must not become a signal, got {names:?}"
        );
    }

    #[test]
    fn aligned_signal_table_is_unchanged() {
        // Guard: a normally-aligned table (name in col 0) is unaffected (offset 0).
        let table = StructuredTableRecord {
            table_id: "t".to_string(),
            asset_id: "a".to_string(),
            page_id: None,
            caption_text: None,
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![row(&["Signal", "Source", "Width", "Description"])],
            body_rows: vec![
                row(&["PCLK", "Clock", "1", "Clock signal"]),
                row(&["PADDR", "Requester", "ADDR_WIDTH", "Address bus"]),
            ],
            row_count: 2,
            col_count: 4,
        };
        let mut counter = 0usize;
        let mut prov = Vec::new();
        let stmts = synthesize_signal_declarations(
            &table,
            SectionKind::Unknown,
            "",
            &mut counter,
            None,
            &mut prov,
        );
        let names: Vec<&str> = stmts
            .iter()
            .filter_map(|s| s.text.strip_prefix("Signal "))
            .filter_map(|s| s.split_whitespace().next())
            .collect();
        assert!(
            names.contains(&"PCLK") && names.contains(&"PADDR"),
            "got {names:?}"
        );
    }
}

#[cfg(test)]
mod wire_based_100_5i {
    //! WIRE-BASED-100.5i — a constraint subject must be a DECLARED signal: property/config/doc-meta
    //! prose ("RME_Support must be False" → "RME") is not mined as a signal constraint, while a real
    //! declared-signal constraint is kept. Gated on a non-empty catalog (tiny no-declaration fixtures
    //! are unaffected).
    use super::*;

    fn stmt(id: &str, class: StatementClass, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn property_subject_dropped_declared_signal_kept() {
        let statements = vec![
            // The document's catalog: ASKSTOP is a declared signal; RME is NOT (it's a property).
            stmt(
                "d1",
                StatementClass::SourceFact,
                "Signal ASKSTOP is output width 1.",
            ),
            stmt(
                "c1",
                StatementClass::SignalValueConstraint,
                "ASKSTOP must be LOW during reset.",
            ),
            // dynamic path: "RME_Support must be False" → subject "RME", an undeclared property.
            stmt(
                "c2",
                StatementClass::NormativeStatement,
                "When GDI_Support is True, RME_Support must be False.",
            ),
        ];
        let mut counter = 0usize;
        let mut subjects: Vec<String> = extract_signal_constraints(&statements, &mut counter)
            .into_iter()
            .map(|r| r.subject_signal)
            .collect();
        let mut dyn_counter = 0usize;
        subjects.extend(
            extract_dynamic_signal_constraints(&statements, &mut dyn_counter, &HashSet::new())
                .into_iter()
                .map(|r| r.subject_signal),
        );
        assert!(
            subjects.contains(&"ASKSTOP".to_string()),
            "declared signal kept, got {subjects:?}"
        );
        assert!(
            !subjects.iter().any(|s| matches!(s.as_str(), "RME" | "GDI")),
            "undeclared property subjects must be dropped, got {subjects:?}"
        );
    }

    #[test]
    fn no_catalog_fixture_is_not_emptied() {
        // No "Signal X" declaration → catalog empty → filter skipped (constraint still extracted).
        let statements = vec![stmt(
            "c1",
            StatementClass::SignalValueConstraint,
            "HADDR must be stable.",
        )];
        let mut counter = 0usize;
        let recs = extract_signal_constraints(&statements, &mut counter);
        assert!(
            recs.iter().any(|r| r.subject_signal == "HADDR"),
            "with no catalog the filter must not drop the constraint, got {recs:?}"
        );
    }
}

#[cfg(test)]
mod swd_serial_extraction_2 {
    //! SWD-SERIAL-EXTRACTION.2 — capture interface signals declared in prose ("a clock pin, SWCLK").
    use super::*;

    fn stmt(text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: "s".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    fn declared(statements: &[ExtractedStatement]) -> Vec<String> {
        let mut c = 0usize;
        synthesize_signal_declarations_from_prose(statements, &mut c, true)
            .into_iter()
            .filter_map(|s| s.text.strip_prefix("Signal ").map(|t| t.to_string()))
            .filter_map(|t| t.split_whitespace().next().map(|w| w.to_string()))
            .collect()
    }

    #[test]
    fn captures_swclk_and_swdio_from_pin_appositive() {
        let stmts = vec![
            stmt("The SWD interface is synchronous, and requires a clock pin, SWCLK ."),
            stmt("The SWD interface uses a single bidirectional data pin, SWDIO ."),
        ];
        let names = declared(&stmts);
        assert!(names.contains(&"SWCLK".to_string()), "got {names:?}");
        assert!(names.contains(&"SWDIO".to_string()), "got {names:?}");
    }

    #[test]
    fn does_not_capture_cross_reference_after_pin() {
        // "… pin, see Figure …" must not declare a signal SEE.
        let names = declared(&[stmt(
            "Drive the line before tristating the pin, see Figure B4-3 .",
        )]);
        assert!(!names.iter().any(|n| n == "SEE"), "got {names:?}");
    }

    #[test]
    fn captures_signal_from_parenthetical_abbreviation() {
        // PDF-VARIANT-DIGESTION.3 — I2C-style prose: "a serial data line (SDA) and a serial clock (SCL)".
        let names = declared(&[stmt(
            "Two bus lines are required: a serial data line (SDA) and a serial clock (SCL) .",
        )]);
        assert!(names.contains(&"SDA".to_string()), "got {names:?}");
        assert!(names.contains(&"SCL".to_string()), "got {names:?}");
    }

    #[test]
    fn parenthetical_without_signal_descriptor_is_ignored() {
        // a bare parenthetical acronym with no signal descriptor nearby must NOT be captured.
        let names = declared(&[stmt("The protocol (I2C) supports multiple controllers .")]);
        assert!(!names.iter().any(|n| n == "I2C"), "got {names:?}");
    }

    #[test]
    fn parenthetical_head_must_be_a_wire_noun_i2c_precision() {
        // EXTRACTION-GAP-FIX.1 — the exact real I2C-bus prose (UM10204). The 6 genuine bus signals each
        // have a WIRE-NOUN head immediately before the abbreviation (line / clock / data) and must be
        // captured; the over-captures have a non-wire head (clock PULSE / Data CHANNEL / data RATE) and
        // must be dropped (precision 0.600 -> higher; recall held).
        let names = declared(&[
            stmt(
                "Only two bus lines are required: a serial data line (SDA) and a serial clock line (SCL) .",
            ),
            stmt(
                "The 2-wire push-pull driver consists of a UFm serial clock (USCL) and serial data (USDA) .",
            ),
            stmt(
                "During Hs-mode transfer, the high-speed data (SDAH) and high-speed serial clock (SCLH) lines .",
            ),
            stmt("An acknowledge clock pulse (ACK) and a Not Acknowledge clock pulse (NACK) ."),
            stmt("Display Data Channel (DDC) is an example interface ."),
            stmt("I3C supports a multi-drop bus that supports standard data rate (SDR) ."),
        ]);
        for keep in ["SDA", "SCL", "USCL", "USDA", "SDAH", "SCLH"] {
            assert!(
                names.contains(&keep.to_string()),
                "expected {keep} in {names:?}"
            );
        }
        for drop in ["ACK", "NACK", "DDC", "SDR"] {
            assert!(
                !names.iter().any(|n| n == drop),
                "non-wire-head over-capture {drop} must be dropped; got {names:?}"
            );
        }
    }

    #[test]
    fn ignores_pin_not_followed_by_signal_token() {
        let names = declared(&[stmt("The host parks the line before the turnaround pin.")]);
        assert!(names.is_empty(), "got {names:?}");
    }
}

#[cfg(test)]
mod pdf_variant_digestion_9_8 {
    //! PDF-VARIANT-DIGESTION.9.8 — definitional single-wire signal capture: a signal DEFINED in prose via a
    //! copula ("<NAME> is a/an signal …") or a glossary colon ("<NAME>: signal …"). SWP names S1/S2 only
    //! this way; the candidate must be an all-uppercase identifier token so lowercase subjects can't leak.
    use super::*;

    fn stmt(text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: "s".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    fn declared(statements: &[ExtractedStatement], enable_parenthetical: bool) -> Vec<String> {
        let mut c = 0usize;
        synthesize_signal_declarations_from_prose(statements, &mut c, enable_parenthetical)
            .into_iter()
            .filter_map(|s| s.text.strip_prefix("Signal ").map(|t| t.to_string()))
            .filter_map(|t| t.split_whitespace().next().map(|w| w.to_string()))
            .collect()
    }

    #[test]
    fn captures_swp_signals_from_copula_and_colon() {
        // The exact SWP (ETSI TS 102 613) prose: S1/S2 are defined only in prose, not a signal table.
        let names = declared(
            &[
                stmt(
                    "S1 is a signal in the voltage domain to transmit data from the CLF to the UICC on SWIO.",
                ),
                stmt(
                    "S2 is a signal in the current domain to transmit data from the UICC to the master.",
                ),
                stmt("S1: signal from the master to a slave"),
                stmt("S2: signal from the slave to the master"),
            ],
            true,
        );
        assert!(names.contains(&"S1".to_string()), "got {names:?}");
        assert!(names.contains(&"S2".to_string()), "got {names:?}");
    }

    #[test]
    fn rejects_lowercase_subject_definitions() {
        // A lowercase English subject must never become a signal — the identifier-shape guard, not a denylist.
        let names = declared(
            &[
                stmt("An interrupt is a signal that requests attention from the processor."),
                stmt("In this protocol it is a signal asserted by the controller."),
            ],
            true,
        );
        assert!(
            names.is_empty(),
            "lowercase subjects must not be captured; got {names:?}"
        );
    }

    #[test]
    fn colon_rejects_non_identifier_head() {
        // The glossary-colon head must be a single uppercase identifier: "master: entity which …" and
        // "Note: signal …" are not signal definitions.
        let names = declared(
            &[
                stmt("master: entity which provides the S1 signal"),
                stmt("Note: signal integrity must be maintained across the bus."),
            ],
            true,
        );
        assert!(names.is_empty(), "got {names:?}");
    }

    #[test]
    fn copula_ignores_non_signal_predicate() {
        // "<NAME> is transmitted/required" is NOT "<NAME> is a signal" — no capture.
        let names = declared(
            &[stmt(
                "The signal S1 is transmitted by a digital modulation in the voltage domain.",
            )],
            true,
        );
        assert!(!names.iter().any(|n| n == "S1"), "got {names:?}");
    }

    #[test]
    fn respects_synthesis_denylist() {
        // Role/logic words that pass the identifier shape are still rejected by the shared denylist.
        let names = declared(&[stmt("CLOCK is a signal used for synchronization.")], true);
        assert!(!names.iter().any(|n| n == "CLOCK"), "got {names:?}");
    }

    #[test]
    fn gated_off_for_table_rich_specs() {
        // The definitional form runs only as a sparse-catalog fallback, like the parenthetical form, so a
        // table-rich spec (which already has its signals from tables) is untouched.
        let names = declared(&[stmt("S1 is a signal in the voltage domain.")], false);
        assert!(names.is_empty(), "got {names:?}");
    }

    #[test]
    fn definitional_signal_names_unit() {
        assert_eq!(
            definitional_signal_names("S1 is a signal in the voltage domain."),
            vec!["S1".to_string()]
        );
        assert_eq!(
            definitional_signal_names("S2: signal from the slave to the master"),
            vec!["S2".to_string()]
        );
        assert!(definitional_signal_names("an interrupt is a signal").is_empty());
    }
}

#[cfg(test)]
mod extractor_architecture_9a {
    //! EXTRACTOR-ARCHITECTURE.9a — the serial-frame and SWD-operations surfaces run through the unified
    //! drivers: key-merge reproduces the "composition defers to bit-range names" policy and both surfaces
    //! record inspectable manifest entries.
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    /// Statements that fire BOTH strategies with one overlapping name (`ACK`): the bit-range form (serial
    /// doc marker + acknowledge phase + `ACK[2:0]`) and a composition list naming `ACK` and `CRC FIELD`.
    fn overlapping_statements() -> Vec<ExtractedStatement> {
        vec![
            stmt(
                "gate",
                "The SWD interface uses a single bidirectional data pin, SWDIO.",
            ),
            stmt(
                "ack",
                "The first three bits shifted out are ACK[2:0] in the acknowledge phase.",
            ),
            stmt(
                "comp",
                "The frame is composed of two bit fields: ACK, CRC FIELD.",
            ),
        ]
    }

    #[test]
    fn serial_frame_surface_composition_defers_to_bit_range_name() {
        let statements = overlapping_statements();
        let mut manifest = ExtractionManifest::default();
        let fields = serial_frame_field_surface(&statements, &mut manifest);

        let names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, vec!["ACK", "CRC FIELD"]);
        // The surviving ACK is the bit-range strategy's record (it carries the literal bit range), proving
        // first-wins precedence — the composition duplicate was dropped, not merged over it.
        assert_eq!(fields[0].bit_range, Some((2, 0)));
    }

    #[test]
    fn serial_frame_surface_records_per_strategy_manifest() {
        let statements = overlapping_statements();
        let mut manifest = ExtractionManifest::default();
        serial_frame_field_surface(&statements, &mut manifest);

        let surface = manifest
            .surfaces
            .iter()
            .find(|s| s.surface == "serial_frame_fields")
            .expect("serial_frame_fields surface recorded");
        assert_eq!(surface.eligible, 2);
        let by_name: std::collections::BTreeMap<&str, (usize, usize)> = surface
            .entries
            .iter()
            .map(|e| (e.name.as_str(), (e.produced, e.kept)))
            .collect();
        assert_eq!(by_name["serial_frame.bit_range"], (1, 1));
        // The composition strategy produced both names but kept only the non-overlapping one.
        assert_eq!(by_name["serial_frame.composition"], (2, 1));
    }

    #[test]
    fn serial_frame_surface_matches_legacy_two_step_merge() {
        // The exact legacy policy, run directly: bit-range fields first, then composition fields whose
        // name is not already present. The surface output must be identical.
        let statements = overlapping_statements();
        let mut legacy = extract_serial_frame_fields(&statements);
        let names: BTreeSet<String> = legacy.iter().map(|f| f.name.clone()).collect();
        for field in extract_composition_frame_fields(&statements) {
            if !names.contains(&field.name) {
                legacy.push(field);
            }
        }

        let mut manifest = ExtractionManifest::default();
        let surfaced = serial_frame_field_surface(&statements, &mut manifest);
        assert_eq!(surfaced, legacy);
    }

    #[test]
    fn swd_operation_surface_records_manifest_entry_even_when_empty() {
        // A non-serial document keeps the surface visible in the manifest (eligible, fired nothing) —
        // honest "ran and found nothing", distinct from "never ran".
        let statements = vec![stmt("p", "PWDATA is driven by the requester.")];
        let mut manifest = ExtractionManifest::default();
        let operations = swd_operation_surface(&statements, &mut manifest);
        assert!(operations.is_empty());

        let surface = manifest
            .surfaces
            .iter()
            .find(|s| s.surface == "swd_operations")
            .expect("swd_operations surface recorded");
        assert_eq!(surface.eligible, 1);
        assert_eq!(surface.entries.len(), 1);
        assert_eq!(surface.entries[0].name, "operations.prose");
        assert_eq!(surface.entries[0].produced, 0);
        assert_eq!(surface.entries[0].kept, 0);
    }
}

#[cfg(test)]
mod extractor_architecture_9b {
    //! EXTRACTOR-ARCHITECTURE.9b — the signal-polarity observation surface runs through the unified
    //! concat driver: strategy order (prose → tables) plus the unchanged arbitration post-pass reproduce
    //! the legacy `collect_signal_polarity_facts` exactly, and the surface records an inspectable
    //! manifest entry whose per-strategy counts match what each grammar produced.
    use super::*;
    use std::collections::HashSet;
    use std::fs;
    use tempfile::tempdir;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    fn cell(text: &str, is_header: bool) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        }
    }

    /// A minimal real `SourceIr` carrying one signal-description table with polarity-bearing
    /// descriptions: PRESETN declared active HIGH (conflicting with the active-low prose) and CS_N
    /// declared active low (agreeing with the prose).
    fn polarity_source_ir(dir: &std::path::Path) -> crate::error::Result<SourceIr> {
        let source = dir.join("polarity.md");
        fs::write(&source, "# Polarity\nSignal PRESETN is input width 1.\n")?;
        let mut source_ir = SourceIr::build(&source, &dir.join("generated").join("source_ir"))?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_polarity_desc".to_string(),
            asset_id: "asset_polarity_desc".to_string(),
            page_id: None,
            caption_text: Some("Signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![cell("Signal", true), cell("Description", true)]],
            body_rows: vec![
                vec![
                    cell("PRESETN", false),
                    cell("Active high reset input", false),
                ],
                vec![cell("CS_N", false), cell("Active low chip select", false)],
            ],
            row_count: 2,
            col_count: 2,
        });
        Ok(source_ir)
    }

    fn polarity_statements() -> Vec<ExtractedStatement> {
        vec![
            stmt("s01", "PRESETN is an active low reset signal."),
            stmt("s02", "CS_N is active LOW and ENABLE is active HIGH."),
        ]
    }

    fn known_signals() -> HashSet<String> {
        ["PRESETN", "CS_N", "ENABLE"]
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    #[test]
    fn signal_polarity_surface_matches_legacy_two_step() -> crate::error::Result<()> {
        let dir = tempdir()?;
        let source_ir = polarity_source_ir(dir.path())?;
        let statements = polarity_statements();
        let known = known_signals();

        // The legacy shape: run both strategies in collection order (prose, then tables), concatenate,
        // arbitrate. Both strategies must actually fire for this equivalence to mean anything.
        let prose_observations = extract_signal_polarity_from_prose(&statements, &known);
        let table_observations =
            extract_signal_polarity_from_signal_tables(&source_ir, &known, None);
        assert!(!prose_observations.is_empty(), "prose strategy must fire");
        assert!(!table_observations.is_empty(), "table strategy must fire");
        let mut legacy_observations = prose_observations.clone();
        legacy_observations.extend(table_observations.clone());
        let legacy = arbitrate_signal_polarity_observations(legacy_observations);

        let mut manifest = ExtractionManifest::default();
        let surfaced =
            signal_polarity_surface(&source_ir, &statements, &known, None, &mut manifest);
        assert_eq!(surfaced.resolved, legacy.resolved);
        assert_eq!(surfaced.resolved_records, legacy.resolved_records);
        assert_eq!(surfaced.conflicts, legacy.conflicts);

        // The semantic outcome itself: CS_N resolves active-low with prose AND table support (a second
        // same-polarity source strengthens, never duplicates), ENABLE resolves active-high from prose
        // alone, and PRESETN's prose/table disagreement stays an explicit conflict — never dropped.
        assert_eq!(
            surfaced.resolved.get("CS_N"),
            Some(&SignalPolarity::ActiveLow)
        );
        assert_eq!(
            surfaced.resolved.get("ENABLE"),
            Some(&SignalPolarity::ActiveHigh)
        );
        let cs_n = surfaced
            .resolved_records
            .iter()
            .find(|record| record.signal_name == "CS_N")
            .expect("CS_N resolved record");
        assert!(!cs_n.supporting_statement_ids.is_empty());
        assert!(!cs_n.supporting_table_ids.is_empty());
        assert_eq!(surfaced.conflicts.len(), 1);
        assert_eq!(surfaced.conflicts[0].signal_name, "PRESETN");

        // The manifest entry: concat driver, two strategies in registry order, kept == produced.
        let surface = manifest
            .surfaces
            .iter()
            .find(|s| s.surface == "signal_polarities")
            .expect("signal_polarities surface recorded");
        assert_eq!(surface.eligible, 2);
        assert_eq!(surface.entries.len(), 2);
        assert_eq!(surface.entries[0].name, "signal_polarity.prose");
        assert_eq!(surface.entries[0].produced, prose_observations.len());
        assert_eq!(surface.entries[0].kept, prose_observations.len());
        assert_eq!(surface.entries[1].name, "signal_polarity.tables");
        assert_eq!(surface.entries[1].produced, table_observations.len());
        assert_eq!(surface.entries[1].kept, table_observations.len());
        Ok(())
    }

    #[test]
    fn signal_polarity_surface_records_manifest_entry_even_when_empty() -> crate::error::Result<()>
    {
        // A document with no polarity-bearing prose or tables keeps the surface visible in the manifest
        // (eligible, fired nothing) — honest "ran and found nothing", distinct from "never ran".
        let dir = tempdir()?;
        let source = dir.path().join("plain.md");
        fs::write(&source, "# Plain\nSignal PWDATA is input width 32.\n")?;
        let source_ir = SourceIr::build(&source, &dir.path().join("generated").join("source_ir"))?;
        let statements = vec![stmt("s01", "PWDATA is driven by the requester.")];
        let known: HashSet<String> = ["PWDATA".to_string()].into_iter().collect();

        let mut manifest = ExtractionManifest::default();
        let surfaced =
            signal_polarity_surface(&source_ir, &statements, &known, None, &mut manifest);
        assert!(surfaced.resolved.is_empty());
        assert!(surfaced.resolved_records.is_empty());
        assert!(surfaced.conflicts.is_empty());

        let surface = manifest
            .surfaces
            .iter()
            .find(|s| s.surface == "signal_polarities")
            .expect("signal_polarities surface recorded");
        assert_eq!(surface.eligible, 2);
        assert_eq!(surface.entries.len(), 2);
        assert_eq!(surface.entries[0].produced, 0);
        assert_eq!(surface.entries[1].produced, 0);
        Ok(())
    }
}

#[cfg(test)]
mod extractor_architecture_9c {
    //! EXTRACTOR-ARCHITECTURE.9c — the actor-signal-relations observation surface runs through the
    //! unified concat driver: strategy order (prose → tables) plus the two ORDERED legacy post-passes
    //! (check-signal augmentation over the full pre-dedup list, THEN first-wins dedup) reproduce the
    //! legacy in-loop wiring exactly, and the surface records an inspectable manifest entry.
    use super::*;
    use std::collections::HashSet;
    use std::fs;
    use tempfile::tempdir;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    fn cell(text: &str, is_header: bool) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        }
    }

    fn relation(
        id: &str,
        actor: &str,
        signal: &str,
        kind: RelationKind,
        source_id: &str,
    ) -> ActorSignalRelation {
        ActorSignalRelation {
            relation_id: id.to_string(),
            actor_name: actor.to_string(),
            signal_name: signal.to_string(),
            relation: kind,
            source_statement_ids: vec![source_id.to_string()],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    /// A minimal real `SourceIr` carrying one check-signal table (the augment post-pass input shape):
    /// the check signal `PCHK` covers `PREQ`, so it must inherit PREQ's relations from the merged
    /// pre-dedup list.
    fn check_table_source_ir(dir: &std::path::Path) -> crate::error::Result<SourceIr> {
        let source = dir.join("relations.md");
        fs::write(&source, "# Relations\nSignal PREQ is input width 1.\n")?;
        let mut source_ir = SourceIr::build(&source, &dir.join("generated").join("source_ir"))?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_check_signals".to_string(),
            asset_id: "asset_check_signals".to_string(),
            page_id: None,
            caption_text: Some("Check signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                cell("Check Signal", true),
                cell("Signals covered", true),
            ]],
            body_rows: vec![vec![cell("PCHK", false), cell("PREQ", false)]],
            row_count: 1,
            col_count: 2,
        });
        Ok(source_ir)
    }

    #[test]
    fn actor_signal_relation_surface_matches_legacy_two_step() -> crate::error::Result<()> {
        let dir = tempdir()?;
        let source_ir = check_table_source_ir(dir.path())?;
        // Prose grounds "Requester drives PREQ"; the table strategy re-emits a precomputed duplicate of
        // that same relation (exercising the dedup post-pass) plus a distinct Reads relation.
        let statements = vec![stmt("s01", "The Requester drives PREQ.")];
        let known: HashSet<String> = ["PREQ".to_string()].into_iter().collect();
        let table_relations = vec![
            relation("tbl_0001", "Requester", "PREQ", RelationKind::Drives, "t01"),
            relation("tbl_0002", "Completer", "PREQ", RelationKind::Reads, "t01"),
        ];

        // The legacy shape: prose extract → cloned table extend → augment (full pre-dedup list) → dedup.
        let prose_relations = extract_actor_signal_relations(&statements, &known);
        assert!(!prose_relations.is_empty(), "prose strategy must fire");
        let mut legacy_merged = prose_relations.clone();
        legacy_merged.extend(table_relations.iter().cloned());
        let legacy_augmented =
            augment_check_signal_relations_from_tables(&source_ir, &legacy_merged, None);
        assert!(
            legacy_augmented.len() > legacy_merged.len(),
            "augment post-pass must inherit check-signal relations"
        );
        let legacy = dedup_actor_signal_relations(legacy_augmented);

        let mut manifest = ExtractionManifest::default();
        let surfaced = actor_signal_relation_surface(
            &source_ir,
            &statements,
            &known,
            &table_relations,
            None,
            &mut manifest,
        );
        assert_eq!(surfaced, legacy);

        // The semantic outcome itself: the prose Drives wins over the table duplicate (first-wins keeps
        // the prose record's ids), the distinct Reads survives, and PCHK inherits PREQ's relations.
        let preq_drives: Vec<&ActorSignalRelation> = surfaced
            .iter()
            .filter(|r| {
                r.signal_name == "PREQ"
                    && r.actor_name == "Requester"
                    && matches!(r.relation, RelationKind::Drives)
            })
            .collect();
        assert_eq!(preq_drives.len(), 1, "duplicate Drives deduped");
        assert_ne!(
            preq_drives[0].relation_id, "tbl_0001",
            "first-wins keeps the prose record, not the table duplicate"
        );
        assert!(
            surfaced
                .iter()
                .any(|r| r.signal_name == "PREQ" && matches!(r.relation, RelationKind::Reads)),
            "distinct table Reads relation survives"
        );
        assert!(
            surfaced
                .iter()
                .any(|r| r.signal_name == "PCHK" && r.relation_id.starts_with("chk_asr_")),
            "check signal inherits covered-signal relations via the augment post-pass"
        );

        // The manifest entry: concat driver, two strategies in registry order, kept == produced.
        let surface = manifest
            .surfaces
            .iter()
            .find(|s| s.surface == "actor_signal_relations")
            .expect("actor_signal_relations surface recorded");
        assert_eq!(surface.eligible, 2);
        assert_eq!(surface.entries.len(), 2);
        assert_eq!(surface.entries[0].name, "relations.prose");
        assert_eq!(surface.entries[0].produced, prose_relations.len());
        assert_eq!(surface.entries[0].kept, prose_relations.len());
        assert_eq!(surface.entries[1].name, "relations.tables");
        assert_eq!(surface.entries[1].produced, table_relations.len());
        assert_eq!(surface.entries[1].kept, table_relations.len());
        Ok(())
    }

    #[test]
    fn actor_signal_relation_surface_records_manifest_entry_even_when_empty()
    -> crate::error::Result<()> {
        // A document with no relation-bearing prose, no table relations, and no check-signal tables
        // keeps the surface visible in the manifest (eligible, fired nothing).
        let dir = tempdir()?;
        let source = dir.path().join("plain.md");
        fs::write(&source, "# Plain\nSignal PWDATA is input width 32.\n")?;
        let source_ir = SourceIr::build(&source, &dir.path().join("generated").join("source_ir"))?;
        let statements = vec![stmt("s01", "PWDATA carries the write data payload.")];
        let known: HashSet<String> = ["PWDATA".to_string()].into_iter().collect();

        let mut manifest = ExtractionManifest::default();
        let surfaced = actor_signal_relation_surface(
            &source_ir,
            &statements,
            &known,
            &[],
            None,
            &mut manifest,
        );
        assert!(surfaced.is_empty());

        let surface = manifest
            .surfaces
            .iter()
            .find(|s| s.surface == "actor_signal_relations")
            .expect("actor_signal_relations surface recorded");
        assert_eq!(surface.eligible, 2);
        assert_eq!(surface.entries.len(), 2);
        assert_eq!(surface.entries[0].produced, 0);
        assert_eq!(surface.entries[1].produced, 0);
        Ok(())
    }
}

#[cfg(test)]
mod pdf_variant_digestion_9_3b {
    //! PDF-VARIANT-DIGESTION.9.3b — recover a frame STRUCTURE from a prose composition list (CAN's
    //! "composed of seven bit fields: …") + per-field prose widths, honestly (no width fabrication).
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    fn can_statements() -> Vec<ExtractedStatement> {
        // The exact CAN (Bosch CAN 2.0) prose shape: the composition sentence ends at the colon and the
        // ordered field list is the following statement; widths are stated unevenly in later prose.
        vec![
            stmt(
                "s01",
                "A DATA FRAME is composed of seven different bit fields:",
            ),
            stmt(
                "s02",
                "START OF FRAME, ARBITRATION FIELD, CONTROL FIELD, DATA FIELD, CRC FIELD, ACK FIELD, END OF FRAME. The DATA FIELD can be of length zero.",
            ),
            stmt("s03", "The CONTROL FIELD consists of six bits."),
            stmt(
                "s04",
                "The ACK FIELD is two bits long and contains the ACK SLOT and the ACK DELIMITER.",
            ),
            // A sub-field count that must NOT become the ARBITRATION FIELD's width (honesty guardrail).
            stmt(
                "s05",
                "In Standard Format the ARBITRATION FIELD consists of the 11 bit IDENTIFIER and the RTR-BIT.",
            ),
        ]
    }

    #[test]
    fn recovers_can_frame_structure_in_order() {
        let fields = extract_composition_frame_fields(&can_statements());
        let names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "START OF FRAME",
                "ARBITRATION FIELD",
                "CONTROL FIELD",
                "DATA FIELD",
                "CRC FIELD",
                "ACK FIELD",
                "END OF FRAME",
            ],
            "frame sequence"
        );
        // Order is the composition sequence.
        assert_eq!(fields[0].order, Some(0));
        assert_eq!(fields[6].order, Some(6));
    }

    #[test]
    fn records_only_directly_stated_widths() {
        let fields = extract_composition_frame_fields(&can_statements());
        let w = |name: &str| fields.iter().find(|f| f.name == name).unwrap().bit_width;
        assert_eq!(w("CONTROL FIELD"), Some(6), "consists of six bits");
        assert_eq!(w("ACK FIELD"), Some(2), "is two bits long");
        // Honesty guardrail: "the 11 bit IDENTIFIER" must NOT become the ARBITRATION FIELD's width.
        assert_eq!(w("ARBITRATION FIELD"), None);
        // Unstated / anaphoric / variable widths stay honest residuals.
        assert_eq!(w("START OF FRAME"), None);
        assert_eq!(w("DATA FIELD"), None);
    }

    #[test]
    fn does_not_fire_without_a_composition_list() {
        let fields = extract_composition_frame_fields(&[
            stmt("a", "The Requester drives PADDR during the setup phase."),
            stmt("b", "PREADY is two bits wide."),
        ]);
        assert!(fields.is_empty(), "got {fields:?}");
    }

    #[test]
    fn helper_units() {
        assert!(is_frame_field_name("CONTROL FIELD"));
        assert!(is_frame_field_name("START OF FRAME"));
        assert!(!is_frame_field_name("and"));
        assert!(!is_frame_field_name("A"));
        assert!(!is_frame_field_name("the field"));
        assert_eq!(parse_count_word("six"), Some(6));
        assert_eq!(parse_count_word("single"), Some(1));
        assert_eq!(parse_count_word("11"), Some(11));
        assert_eq!(parse_count_word("recessive"), None);
    }
}

#[cfg(test)]
mod swd_serial_extraction_3 {
    //! SWD-SERIAL-EXTRACTION.3 — typed serial-frame fields from NAME[hi:lo], gated to serial docs +
    //! frame phases (so parallel buses and unrelated bit-fields produce nothing).
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn parses_bit_ranges_and_widths() {
        let f = parse_bit_range_fields("ACK[2:0] and WDATA[0:31] and A[2:3]");
        assert_eq!(
            f,
            vec![
                ("ACK".to_string(), 2, 0),
                ("WDATA".to_string(), 0, 31),
                ("A".to_string(), 2, 3),
            ]
        );
    }

    #[test]
    fn extracts_swd_frame_fields_with_widths() {
        let stmts = vec![
            stmt(
                "d",
                "The SWD interface uses a single bidirectional data pin, SWDIO.",
            ),
            stmt(
                "a",
                "The first three bits of data that are shifted out are ACK[2:0].",
            ),
            stmt(
                "w",
                "The parity check is made over the 32 data bits WDATA[0:31].",
            ),
            stmt(
                "r",
                "The parity check is made over the 32 data bits RDATA[0:31].",
            ),
        ];
        let fields = extract_serial_frame_fields(&stmts);
        let by: std::collections::BTreeMap<_, _> = fields
            .iter()
            .map(|f| (f.name.as_str(), f.bit_width))
            .collect();
        assert_eq!(by.get("ACK"), Some(&Some(3)), "ACK is 3 bits");
        assert_eq!(by.get("WDATA"), Some(&Some(32)), "WDATA is 32 bits");
        assert_eq!(by.get("RDATA"), Some(&Some(32)), "RDATA is 32 bits");
    }

    #[test]
    fn non_serial_document_yields_nothing() {
        // A parallel-bus statement (no serial markers) → no frame fields, even with a bit-range.
        let stmts = vec![stmt(
            "h",
            "HMASTER[3:0] indicates the master number during the data phase.",
        )];
        assert!(extract_serial_frame_fields(&stmts).is_empty());
    }

    #[test]
    fn serial_doc_skips_non_frame_bit_fields() {
        // A serial doc, but the bit-field statement is not in a frame phase → dropped.
        let stmts = vec![
            stmt("m", "The SWD interface uses the serial wire SWDIO pin."),
            stmt(
                "x",
                "The cache attribute field AxCACHE[3:0] is bridged through.",
            ),
        ];
        let fields = extract_serial_frame_fields(&stmts);
        assert!(
            !fields.iter().any(|f| f.name == "AxCACHE"),
            "got {fields:?}"
        );
    }
}

#[cfg(test)]
mod pdf_variant_digestion_9_3a_fsm {
    //! PDF-VARIANT-DIGESTION.9.3a — agnostic quoted-mode FSM extraction (CAN-style error-state FSMs).
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn recovers_can_error_states_from_quoted_node_modes() {
        // CAN-shaped prose: each state is a single-quoted operational mode of a node/unit/station.
        let stmts = vec![
            stmt(
                "a",
                "An 'error active' unit can normally take part in bus communication.",
            ),
            stmt(
                "b",
                "An 'error active' station detecting an error condition signals this.",
            ),
            stmt(
                "c",
                "An 'error passive' unit must not send an ACTIVE ERROR FLAG.",
            ),
            stmt(
                "d",
                "A node is 'error passive' when the TRANSMIT ERROR COUNT equals or exceeds 128. An error condition follows.",
            ),
            stmt(
                "e",
                "A 'bus off' unit is not allowed to have any influence on the bus.",
            ),
            stmt(
                "f",
                "A node is 'bus off' when the TRANSMIT ERROR COUNT is greater than or equal to 256.",
            ),
        ];
        let states = extract_quoted_mode_states(&stmts);
        let names: Vec<&str> = states.iter().map(|s| s.state_name.as_str()).collect();
        assert!(names.contains(&"error active"), "got {names:?}");
        assert!(names.contains(&"error passive"), "got {names:?}");
        assert!(names.contains(&"bus off"), "got {names:?}");
        assert_eq!(
            states.len(),
            3,
            "exactly the three node modes; got {names:?}"
        );
        // The trailing `when …` clause is captured as the transition guard / action.
        let passive = states
            .iter()
            .find(|s| s.state_name == "error passive")
            .unwrap();
        assert_eq!(
            passive.action.as_deref(),
            Some("when the TRANSMIT ERROR COUNT equals or exceeds 128"),
        );
    }

    #[test]
    fn rejects_quoted_bit_values_and_bus_conditions() {
        // 'dominant'/'recessive' are bit VALUES (subject = bit) and 'bus idle' is a bus condition
        // (subject = bus) — none is a node mode, so none becomes a state, even though all are quoted.
        let stmts = vec![
            stmt(
                "a",
                "When a RECEIVER detects a 'dominant' bit the count increases.",
            ),
            stmt(
                "b",
                "The stuff bit should have been 'recessive' but was monitored as 'dominant'.",
            ),
            stmt(
                "c",
                "An 'error passive' node may need the bus to be 'bus idle' for three bit times.",
            ),
            stmt("d", "The bit is 'recessive' during the idle phase."),
        ];
        let names: Vec<String> = extract_quoted_mode_states(&stmts)
            .into_iter()
            .map(|s| s.state_name)
            .collect();
        assert!(!names.iter().any(|n| n == "dominant"), "got {names:?}");
        assert!(!names.iter().any(|n| n == "recessive"), "got {names:?}");
        assert!(!names.iter().any(|n| n == "bus idle"), "got {names:?}");
    }

    #[test]
    fn single_mode_is_not_a_state_machine() {
        // One recurring mode is not an FSM (need ≥2 distinct states).
        let stmts = vec![
            stmt("a", "An 'error active' unit takes part in communication."),
            stmt("b", "An 'error active' node sends an ACTIVE ERROR FLAG."),
        ];
        assert!(extract_quoted_mode_states(&stmts).is_empty());
    }

    #[test]
    fn non_recurring_modes_are_dropped() {
        // Two distinct modes that each appear only once → no recurrence → not promoted.
        let stmts = vec![
            stmt("a", "An 'error active' unit takes part."),
            stmt(
                "b",
                "An 'error passive' unit must not send an ACTIVE ERROR FLAG.",
            ),
        ];
        assert!(extract_quoted_mode_states(&stmts).is_empty());
    }

    #[test]
    fn parallel_bus_prose_yields_no_modes() {
        // APB/AHB-style prose quotes nothing as a node mode → 0 states (no corpus false positives).
        let stmts = vec![
            stmt(
                "a",
                "PSEL selects the completer and PENABLE indicates the access phase.",
            ),
            stmt("b", "The manager drives HADDR during the address phase."),
        ];
        assert!(extract_quoted_mode_states(&stmts).is_empty());
    }
}

#[cfg(test)]
mod pdf_variant_digestion_9_7_fsm {
    //! PDF-VARIANT-DIGESTION.9.7 — transition-bound single-word (ALL-CAPS) FSM extraction (SWP-style
    //! `<NAME> state` machines), additive to the SWD-hyphen and `.9.3a` quoted-mode paths.
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn recovers_swp_style_allcaps_states_via_transition_binding() {
        // SWP-shaped prose: each state is an ALL-CAPS word a node ENTERS / is IN / moves TO.
        let stmts = vec![
            stmt("a", "The CLF shall put SWP into the ACTIVATED state."),
            stmt(
                "b",
                "On RF-field appearance the interface moves from the DEACTIVATED state to the ACTIVATED state.",
            ),
            stmt(
                "c",
                "The terminal shall set SWP to the DEACTIVATED state as defined in clause 8.3.",
            ),
            stmt(
                "d",
                "While in the SUSPENDED state the UICC keeps its context.",
            ),
            stmt(
                "e",
                "The interface enters the SUSPENDED state after an inactivity timeout.",
            ),
        ];
        let states = extract_transition_bound_states(&stmts);
        let names: Vec<&str> = states.iter().map(|s| s.state_name.as_str()).collect();
        assert!(names.contains(&"ACTIVATED"), "got {names:?}");
        assert!(names.contains(&"DEACTIVATED"), "got {names:?}");
        assert!(names.contains(&"SUSPENDED"), "got {names:?}");
        // Ids use a distinct prefix so they never collide with the other three FSM paths.
        assert!(
            states
                .iter()
                .all(|s| s.state_id.starts_with("named_state_")),
            "got {:?}",
            states.iter().map(|s| &s.state_id).collect::<Vec<_>>()
        );
    }

    #[test]
    fn after_guard_rejects_machine_and_diagram_names() {
        // "<X> state machine" / "<X> state diagram" names the MACHINE, not a state — the candidate before
        // "state" must NOT be promoted even though it is ALL-CAPS and there are two distinct such words.
        let stmts = vec![
            stmt("a", "The JTAG TAP state machine must be reset first."),
            stmt(
                "b",
                "Enter the TAP state machine through the reset sequence.",
            ),
            stmt("c", "Refer to the DMA state diagram in figure 4."),
            stmt("d", "The DMA state diagram shows every transition."),
        ];
        assert!(
            extract_transition_bound_states(&stmts).is_empty(),
            "machine/diagram names must not become states"
        );
    }

    #[test]
    fn requires_a_transition_binding() {
        // ALL-CAPS words before "state" with NO transition/locative trigger are "the <X> state"
        // descriptions, not entered states → nothing promoted.
        let stmts = vec![
            stmt("a", "The CONFIG state value is read from a register."),
            stmt("b", "The CONFIG state determines behaviour."),
            stmt("c", "A DEBUG state flag is also provided."),
            stmt("d", "The DEBUG state flag is sticky."),
        ];
        assert!(extract_transition_bound_states(&stmts).is_empty());
    }

    #[test]
    fn single_state_is_not_a_machine() {
        // One recurring transition-bound state is not an FSM (need ≥2 distinct states).
        let stmts = vec![
            stmt("a", "The device enters the HALT state on error."),
            stmt("b", "It remains in the HALT state until reset."),
        ];
        assert!(extract_transition_bound_states(&stmts).is_empty());
    }

    #[test]
    fn non_recurring_states_are_dropped() {
        // Two distinct states that each appear only once → no recurrence → not promoted.
        let stmts = vec![
            stmt("a", "The link moves into the ACTIVE state."),
            stmt("b", "The link moves into the IDLE state."),
        ];
        assert!(extract_transition_bound_states(&stmts).is_empty());
    }

    #[test]
    fn rejects_lowercase_logic_levels_and_pseudo_values() {
        // Lowercase words are descriptions not named states; HIGH/LOW are logic levels; UNKNOWN /
        // UNPREDICTABLE are universal architectural pseudo-values (the SWD/ADI noise) — all denylisted.
        let stmts = vec![
            stmt(
                "a",
                "The signal moves to the high state then to the low state.",
            ),
            stmt("b", "It returns to the high state and then the low state."),
            stmt("c", "The register enters the UNKNOWN state on reset."),
            stmt("d", "Behaviour in the UNKNOWN state is UNPREDICTABLE."),
            stmt("e", "The pin moves into the idle state.."),
        ];
        assert!(
            extract_transition_bound_states(&stmts).is_empty(),
            "logic levels / lowercase / pseudo-values must not become states"
        );
    }

    #[test]
    fn parallel_bus_real_operating_states_are_honestly_captured() {
        // APB DOES have an IDLE→SETUP→ACCESS operating FSM; capturing its named states from genuine
        // transition prose is correct extraction (no scored-metric change), not a false positive.
        let stmts = vec![
            stmt(
                "a",
                "When a transfer is required, the interface moves into the SETUP state.",
            ),
            stmt(
                "b",
                "The interface only remains in the SETUP state for one clock.",
            ),
            stmt("c", "PENABLE is asserted in the ACCESS state."),
            stmt(
                "d",
                "Exit from the ACCESS state is controlled by PREADY; it can stay in the ACCESS state.",
            ),
        ];
        let names: Vec<String> = extract_transition_bound_states(&stmts)
            .into_iter()
            .map(|s| s.state_name)
            .collect();
        assert!(names.contains(&"SETUP".to_string()), "got {names:?}");
        assert!(names.contains(&"ACCESS".to_string()), "got {names:?}");
    }

    #[test]
    fn is_bare_state_name_classifies_correctly() {
        assert!(is_bare_state_name("ACTIVATED"));
        assert!(is_bare_state_name("CL0")); // letters + digit, all-caps
        assert!(is_bare_state_name("BUS-OFF")); // hyphen allowed
        assert!(!is_bare_state_name("Activated")); // mixed case
        assert!(!is_bare_state_name("access")); // lowercase
        assert!(!is_bare_state_name("HIGH")); // logic level (denylist)
        assert!(!is_bare_state_name("UNKNOWN")); // architectural pseudo-value
        assert!(!is_bare_state_name("A")); // too short
        assert!(!is_bare_state_name("123")); // no letter
    }
}

#[cfg(test)]
mod evidence_determinism {
    //! EVIDENCE-DETERMINISM.2 — the EvidenceIR build must be reproducible. Rust's `HashSet` is per-instance
    //! randomly seeded, so building the same logical inputs twice yields different iteration orders;
    //! asserting identical extractor output across two fresh builds is a real non-determinism regression test.
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn actor_signal_relations_are_deterministic_across_hashset_orderings() {
        // One statement names TWO signals, so the relation order + `asr_NNNN` ids depend on the order
        // signals are processed — which, before the fix, was the non-deterministic `known_signals` HashSet
        // iteration order. Build the set + extract twice (fresh HashSets, independently seeded) and require
        // byte-identical output (ids, order, attribution), not just set-equality.
        let stmts = vec![stmt(
            "s1",
            "PSTRB is driven by the Manager and PWUSER is driven by the Manager.",
        )];
        let run = || {
            let signals: std::collections::HashSet<String> = [
                "PSTRB".to_string(),
                "PWUSER".to_string(),
                "PWAKE".to_string(),
            ]
            .into_iter()
            .collect();
            extract_actor_signal_relations(&stmts, &signals)
        };
        let a = run();
        let b = run();
        let proj = |rs: &[ActorSignalRelation]| {
            rs.iter()
                .map(|r| {
                    (
                        r.relation_id.clone(),
                        r.actor_name.clone(),
                        r.signal_name.clone(),
                        format!("{:?}", r.relation),
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            proj(&a),
            proj(&b),
            "relation ids + order must be deterministic across HashSet seedings"
        );
        // sanity: the statement really did yield the two relations that exercise the ordering
        assert!(
            a.iter().any(|r| r.signal_name == "PSTRB"),
            "got {:?}",
            proj(&a)
        );
        assert!(
            a.iter().any(|r| r.signal_name == "PWUSER"),
            "got {:?}",
            proj(&a)
        );
    }
}

#[cfg(test)]
mod extractor_architecture_3_fsm_surface {
    //! EXTRACTOR-ARCHITECTURE.3 — the FSM cluster, migrated onto the unified `Extractor`/`run_surface`
    //! framework, unions its four grammars and dedups across them by uppercased state name (the legacy
    //! behavior, now in one driver call). Corpus byte-identicality (SWP/SWD/CAN) is verified out-of-band;
    //! this locks the cross-grammar union + dedup wiring hermetically.
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn surface_unions_quoted_and_transition_grammars_and_dedups_by_name() {
        // Quoted-mode grammar contributes `reset` + `active` (a node IS '<mode>'); transition-bound grammar
        // contributes `RESET` + `HALT` (enters the <NAME> state). `RESET` collapses onto the earlier-in-order
        // quoted `reset` (uppercased key), so the union is {reset, active, HALT} — proving both grammars feed
        // ONE surface and the cross-grammar dedup matches the legacy first-wins-by-name behavior.
        let stmts = vec![
            stmt("a", "A node is 'reset' when the controller clears it."),
            stmt("b", "A node is 'reset' until reinitialised."),
            stmt("c", "A node is 'active' when participating."),
            stmt("d", "A node is 'active' during normal operation."),
            stmt("e", "The interface enters the RESET state on error."),
            stmt("f", "It remains in the RESET state until cleared."),
            stmt("g", "The interface enters the HALT state on fault."),
            stmt("h", "It stays in the HALT state until reset."),
        ];
        let names: Vec<String> = protocol_state_surface(
            &stmts,
            &mut crate::ir::extractor::ExtractionManifest::default(),
        )
        .into_iter()
        .map(|s| s.state_name)
        .collect();
        assert!(names.contains(&"reset".to_string()), "got {names:?}");
        assert!(names.contains(&"active".to_string()), "got {names:?}");
        assert!(names.contains(&"HALT".to_string()), "got {names:?}");
        // `RESET` (transition) must NOT appear as a second record — it deduped onto quoted `reset`.
        assert!(!names.contains(&"RESET".to_string()), "got {names:?}");
        assert_eq!(
            names.len(),
            3,
            "union minus the one cross-grammar duplicate; got {names:?}"
        );
    }

    #[test]
    fn non_fsm_prose_yields_no_states_through_the_surface() {
        // A parallel-bus snippet trips none of the four grammars → empty surface (no corpus false positives).
        let stmts = vec![
            stmt(
                "a",
                "PSEL selects the completer and PENABLE indicates the access phase.",
            ),
            stmt("b", "The manager drives HADDR during the address phase."),
        ];
        assert!(
            protocol_state_surface(
                &stmts,
                &mut crate::ir::extractor::ExtractionManifest::default()
            )
            .is_empty()
        );
    }
}

#[cfg(test)]
mod swd_serial_extraction_3b {
    //! SWD-SERIAL-EXTRACTION.3b — named request bits, ACK response values, frame ordering.
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn parses_named_bits_skipping_ranges() {
        let bits = parse_named_bit_list(
            "The parity check is made over the four bits APnDP, RnW and A[2:3]:",
        );
        assert_eq!(bits, vec!["APnDP".to_string(), "RnW".to_string()]);
    }

    #[test]
    fn ack_response_values_are_clean() {
        let stmts = vec![
            stmt("a", "0b001 WAIT WAIT response to a DPACC or APACC access."),
            stmt(
                "b",
                "0b010 OK or FAULT response to a DPACC or APACC access.",
            ),
            stmt("c", "The DP response to a CTI request is separate."),
        ];
        let vals = extract_ack_response_values(&stmts);
        assert_eq!(
            vals,
            vec!["FAULT".to_string(), "OK".to_string(), "WAIT".to_string()]
        );
        assert!(
            !vals.iter().any(|v| v == "DP" || v == "CTI"),
            "no noise: {vals:?}"
        );
    }

    #[test]
    fn frame_fields_are_ordered_by_phase() {
        let stmts = vec![
            stmt(
                "d",
                "The SWD interface uses a single bidirectional data pin, SWDIO.",
            ),
            stmt(
                "w",
                "The parity check is made over the 32 data bits WDATA[0:31].",
            ),
            stmt(
                "q",
                "The four bits APnDP, RnW are part of the packet request.",
            ),
            stmt(
                "a",
                "The first three bits of data shifted out are ACK[2:0].",
            ),
        ];
        let fields = extract_serial_frame_fields(&stmts);
        let req = fields
            .iter()
            .find(|f| f.name == "APnDP")
            .and_then(|f| f.order)
            .unwrap();
        let ack = fields
            .iter()
            .find(|f| f.name == "ACK")
            .and_then(|f| f.order)
            .unwrap();
        let data = fields
            .iter()
            .find(|f| f.name == "WDATA")
            .and_then(|f| f.order)
            .unwrap();
        assert!(
            req < ack && ack < data,
            "request<ack<data: req={req} ack={ack} data={data}"
        );
    }
}

#[cfg(test)]
mod swd_serial_extraction_4 {
    //! SWD-SERIAL-EXTRACTION.4 — protocol FSM state extraction (the JTAG TAP / SWD line state machine).
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn recognizes_state_names() {
        assert!(looks_like_state_name("Shift-DR"));
        assert!(looks_like_state_name("Run-Test/Idle"));
        assert!(looks_like_state_name("Test-Logic-Reset"));
        assert!(!looks_like_state_name("reset"));
        assert!(!looks_like_state_name("this"));
        assert!(!looks_like_state_name("low-level")); // lowercase parts → not a state
    }

    #[test]
    fn extracts_state_and_action() {
        let got = find_states_with_actions(
            "In the Shift-DR state, data is transferred from DBGTDI to DBGTDO.",
        );
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].0, "Shift-DR");
        assert_eq!(
            got[0].1.as_deref(),
            Some("data is transferred from DBGTDI to DBGTDO")
        );
    }

    #[test]
    fn extracts_tap_states_with_machine_name() {
        let stmts = vec![
            stmt(
                "m",
                "The debug port includes a Debug TAP State Machine (DBGTAPSM).",
            ),
            stmt(
                "s",
                "While the DBGTAPSM is in the Shift-IR state, the IR scan chain advances.",
            ),
            stmt(
                "u",
                "When the DBGTAPSM goes through the Update-DR state, the value is transferred.",
            ),
        ];
        let states = extract_protocol_states(&stmts);
        let names: Vec<&str> = states.iter().map(|s| s.state_name.as_str()).collect();
        assert!(
            names.contains(&"Shift-IR") && names.contains(&"Update-DR"),
            "got {names:?}"
        );
        assert!(
            states
                .iter()
                .all(|s| s.machine_name.as_deref() == Some("DBGTAPSM"))
        );
    }

    #[test]
    fn non_state_machine_doc_yields_nothing() {
        let stmts = vec![stmt(
            "x",
            "The Manager drives HTRANS during the data phase.",
        )];
        assert!(extract_protocol_states(&stmts).is_empty());
    }

    #[test]
    fn extracts_actors_from_prose_definitions() {
        // PDF-VARIANT-DIGESTION.3b — I2C-style agent definitions.
        let stmts = vec![
            stmt(
                "c",
                "A controller is the device which initiates a data transfer on the bus and generates the clock.",
            ),
            stmt(
                "t",
                "At that time, any device addressed is considered a target.",
            ),
        ];
        let actors = extract_protocol_actors(&stmts);
        let names: Vec<String> = actors.iter().map(|a| a.name.to_ascii_lowercase()).collect();
        assert!(names.contains(&"controller".to_string()), "got {names:?}");
        assert!(names.contains(&"target".to_string()), "got {names:?}");
        // the defining capability clause is captured
        assert!(
            actors
                .iter()
                .any(|a| a.name.eq_ignore_ascii_case("controller")
                    && a.definition
                        .as_deref()
                        .map(|d| d.contains("initiates"))
                        .unwrap_or(false)),
            "controller definition not captured: {actors:?}"
        );
    }

    #[test]
    fn actor_extraction_ignores_function_words() {
        // "It is the device which …" must not yield an actor named "it".
        let actors = extract_protocol_actors(&[stmt("x", "It is the device which is connected.")]);
        assert!(
            actors.iter().all(|a| !a.name.eq_ignore_ascii_case("it")),
            "got {actors:?}"
        );
    }

    #[test]
    fn extracts_actors_from_generalized_agent_class_nouns() {
        // PDF-VARIANT-DIGESTION.8 — the same agent-definition grammar, now over a curated set of
        // generic agent-class nouns beyond the literal "device" (no chip names, ADR 0006).
        let stmts = vec![
            stmt(
                "s",
                "The SMMU is a component that translates addresses on behalf of a device.",
            ),
            stmt(
                "b",
                "A bus bridge is the module which forwards transactions between domains.",
            ),
            stmt(
                "m",
                "The Manager is an agent that initiates read and write transactions.",
            ),
        ];
        let actors = extract_protocol_actors(&stmts);
        let names: Vec<String> = actors.iter().map(|a| a.name.to_ascii_lowercase()).collect();
        assert!(names.contains(&"smmu".to_string()), "got {names:?}");
        assert!(names.contains(&"bridge".to_string()), "got {names:?}");
        assert!(names.contains(&"manager".to_string()), "got {names:?}");
        // the generalized form still captures the defining capability clause
        assert!(
            actors.iter().any(|a| a.name.eq_ignore_ascii_case("smmu")
                && a.definition
                    .as_deref()
                    .map(|d| d.contains("translates"))
                    .unwrap_or(false)),
            "smmu definition not captured: {actors:?}"
        );
    }

    #[test]
    fn rejects_non_agent_class_definitions() {
        // A non-agent class noun ("signal", "value", "register") must NOT mint a false actor,
        // even with the same "is a <X> that <capability>" shape — the allowlist is the gate.
        let stmts = vec![
            stmt("a", "PADDR is a signal that carries the transfer address."),
            stmt("b", "The data width is a value that must be configured."),
            stmt("c", "CTRL is a register that holds the control bits."),
        ];
        let actors = extract_protocol_actors(&stmts);
        assert!(
            actors.is_empty(),
            "non-agent-class definitions must yield no actors: {actors:?}"
        );
    }

    #[test]
    fn rejects_part_of_mention_without_defining_clause() {
        // "is a component of …" is a part-of mention, not an agent definition: with no
        // defining "that"/"which" clause it must not be captured.
        let actors = extract_protocol_actors(&[stmt(
            "x",
            "The address decoder is a component of the interconnect fabric.",
        )]);
        assert!(
            actors.is_empty(),
            "a part-of mention must not become an actor: {actors:?}"
        );
    }

    #[test]
    fn cross_reference_parenthetical_does_not_pollute_actor_name() {
        // PDF-VARIANT-DIGESTION.8 — a "(refer to section …)" cross-reference before
        // "is a <class> that" must NOT mint "section" as an actor (the real NVMe case); the
        // real agent ("controller") is recovered, and "section" is denylisted as a
        // document-structural noun (defense in depth).
        let actors = extract_protocol_actors(&[stmt(
            "n",
            "An I/O controller (refer to section 3.1.2.1) is a controller that supports user-data commands.",
        )]);
        let names: Vec<String> = actors.iter().map(|a| a.name.to_ascii_lowercase()).collect();
        assert!(!names.contains(&"section".to_string()), "got {names:?}");
        assert!(names.contains(&"controller".to_string()), "got {names:?}");
    }

    #[test]
    fn prepositional_object_is_not_mistaken_for_the_subject() {
        // PDF-VARIANT-DIGESTION.8 — when a PREPOSITION ("for") sits in the subject, the last noun
        // before "is" ("signals") is a prepositional-phrase object, not the agent. The real AHB
        // case: "A typical use case for multiple HSELx signals is a peripheral that …".
        let actors = extract_protocol_actors(&[stmt(
            "h",
            "A typical use case for multiple HSELx signals is a peripheral that has its registers at different locations.",
        )]);
        assert!(
            actors
                .iter()
                .all(|a| !a.name.eq_ignore_ascii_case("signals")),
            "a prepositional-phrase object must not become an actor: {actors:?}"
        );
    }

    #[test]
    fn anaphora_across_a_sentence_boundary_does_not_misname() {
        // PDF-VARIANT-DIGESTION.8 — "… host system. It is the entity that …" must not reach back
        // across the period and name the antecedent "system" (the real USB4 case); the NAME search
        // is confined to the current sentence, and "It" is too short to be an agent → no actor.
        let actors = extract_protocol_actors(&[stmt(
            "u",
            "A Connection Manager is part of a USB4 host system. It is the entity that discovers connected devices.",
        )]);
        assert!(
            actors
                .iter()
                .all(|a| !a.name.eq_ignore_ascii_case("system")),
            "an anaphor's antecedent across a sentence boundary must not become an actor: {actors:?}"
        );
    }
}

#[cfg(test)]
mod swd_serial_extraction_4c {
    //! SWD-SERIAL-EXTRACTION.4c — per-phase SWDIO direction derived from the spec's directional prose.
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn source_actor_and_direction() {
        assert_eq!(
            swdio_source_actor("from the host to the target").as_deref(),
            Some("host")
        );
        assert_eq!(
            swdio_source_actor("from the target to the host").as_deref(),
            Some("target")
        );
        assert_eq!(
            swdio_source_actor("Target to host, following a read request (RDATA).").as_deref(),
            Some("target")
        );
        assert_eq!(
            swdio_direction_from_actor("host"),
            Some(SwdioDirection::HostDrives)
        );
        assert_eq!(
            swdio_direction_from_actor("target"),
            Some(SwdioDirection::TargetDrives)
        );
    }

    #[test]
    fn word_boundary_match() {
        assert!(text_has_word("over the 32 data bits WDATA[0:31]", "WDATA"));
        assert!(!text_has_word("MYWDATAX", "WDATA"));
    }

    #[test]
    fn derives_per_phase_swdio_direction() {
        let stmts = vec![
            stmt(
                "d",
                "The SWD interface uses a single bidirectional data pin, SWDIO.",
            ),
            stmt(
                "r",
                "An eight-bit write packet request, from the host to the target. The four bits APnDP, RnW are part of the packet request.",
            ),
            stmt(
                "a",
                "A three-bit OK acknowledge response, from the target to the host. The bits shifted out are ACK[2:0].",
            ),
            stmt(
                "w",
                "A 33-bit WDATA[0:31] data transfer phase, from the host to the target.",
            ),
            stmt(
                "rd",
                "A 33-bit RDATA[0:31] data transfer phase, where data is transferred from the target to the host.",
            ),
        ];
        let fields = extract_serial_frame_fields(&stmts);
        let dir = |n: &str| {
            fields
                .iter()
                .find(|f| f.name == n)
                .and_then(|f| f.swdio_direction)
        };
        assert_eq!(
            dir("APnDP"),
            Some(SwdioDirection::HostDrives),
            "request driven by host"
        );
        assert_eq!(
            dir("ACK"),
            Some(SwdioDirection::TargetDrives),
            "ack driven by target"
        );
        assert_eq!(
            dir("WDATA"),
            Some(SwdioDirection::HostDrives),
            "write data driven by host"
        );
        assert_eq!(
            dir("RDATA"),
            Some(SwdioDirection::TargetDrives),
            "read data driven by target"
        );
    }
}

#[cfg(test)]
mod swd_serial_extraction_4b {
    //! SWD-SERIAL-EXTRACTION.4b — single-bit control fields (Start/Stop/Parity/Park) complete the frame.
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn parses_named_control_bits_only() {
        assert_eq!(
            parse_control_bit_fields("A single start bit, with value 0b1 ."),
            vec!["Start".to_string()]
        );
        assert_eq!(
            parse_control_bit_fields(
                "A single stop bit. In the synchronous SWD protocol, this bit is always 0b0 ."
            ),
            vec!["Stop".to_string()]
        );
        assert_eq!(
            parse_control_bit_fields("A single parity bit for the preceding packet."),
            vec!["Parity".to_string()]
        );
        assert_eq!(
            parse_control_bit_fields("The Park bit is not 0b1 ."),
            vec!["Park".to_string()]
        );
        // Register-heavy noise must NOT match (the broad "<Word> bit" form is rejected).
        assert!(
            parse_control_bit_fields("To clear the WDATAERR bit to 0b0 , write 0b1.").is_empty()
        );
        assert!(
            parse_control_bit_fields("Each bit of the register is independently writable.")
                .is_empty()
        );
        assert!(
            parse_control_bit_fields("A single bit, indicating whether the access is a read.")
                .is_empty()
        );
    }

    #[test]
    fn control_bits_join_the_request_frame() {
        let stmts = vec![
            stmt(
                "d",
                "The SWD interface uses a single bidirectional data pin, SWDIO.",
            ),
            stmt(
                "r",
                "An eight-bit write packet request, from the host to the target. APnDP, RnW are request bits.",
            ),
            stmt("s", "Start: A single start bit, with value 0b1 ."),
            stmt(
                "p",
                "Park: A single bit. The Park bit is not 0b1 in a protocol error.",
            ),
        ];
        let fields = extract_serial_frame_fields(&stmts);
        let names: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
        assert!(
            names.contains(&"Start") && names.contains(&"Park"),
            "got {names:?}"
        );
        // they are request-phase, host-driven (the host drives the packet request onto SWDIO)
        let start = fields.iter().find(|f| f.name == "Start").unwrap();
        assert_eq!(start.phase, Some(SerialFramePhase::Request));
        assert_eq!(start.swdio_direction, Some(SwdioDirection::HostDrives));
    }
}

#[cfg(test)]
mod swd_serial_extraction_4b_ops {
    //! SWD-SERIAL-EXTRACTION.4b — SWD packet operations: response branching + turnaround model.
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn derives_response_branching_and_turnaround() {
        let stmts = vec![
            stmt(
                "ctx",
                "The SWD interface uses a single bidirectional data pin, SWDIO. A packet request is sent.",
            ),
            stmt(
                "wtrn",
                "For a write request, there is a turnaround period between the acknowledge phase and the WDATA data transfer phase.",
            ),
            stmt(
                "rtrn",
                "For a read request, there is no turnaround period between the acknowledge phase and the data transfer phase.",
            ),
            stmt(
                "w",
                "Therefore, a successful write operation consists of three phases:",
            ),
            stmt(
                "r",
                "Therefore, a successful read operation consists of three phases:",
            ),
            stmt(
                "wa",
                "A WAIT response to a read or write packet request consists of two phases:",
            ),
            stmt(
                "f",
                "A FAULT response to a read or write packet request consists of two phases:",
            ),
        ];
        let ops = extract_swd_operations(&stmts);
        let find = |resp: &str, acc: Option<&str>| {
            ops.iter()
                .find(|o| o.response == resp && o.access.as_deref() == acc)
                .cloned()
        };
        let wr = find("OK", Some("write")).expect("OK write");
        assert!(
            wr.phase_count == 3 && wr.has_data_phase && wr.turnaround_before_data == Some(true)
        );
        let rd = find("OK", Some("read")).expect("OK read");
        assert!(
            rd.phase_count == 3 && rd.has_data_phase && rd.turnaround_before_data == Some(false)
        );
        let wait = find("WAIT", None).expect("WAIT");
        assert!(wait.phase_count == 2 && !wait.has_data_phase);
        assert!(find("FAULT", None).is_some());
    }

    #[test]
    fn non_serial_doc_has_no_operations() {
        let stmts = vec![stmt(
            "x",
            "A write operation consists of three phases on the AHB bus.",
        )];
        assert!(extract_swd_operations(&stmts).is_empty());
    }
}

#[cfg(test)]
mod swd_serial_extraction_4d {
    //! SWD-SERIAL-EXTRACTION.4d — the SWD LINE state machine (reset/protocol-error/lockout/dormant).
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn extracts_line_states_with_verb_and_context() {
        let stmts = vec![
            stmt(
                "ctx",
                "The SWD interface uses a single bidirectional data pin, SWDIO.",
            ),
            stmt(
                "pe",
                "On detecting a protocol error, the SW-DP target enters the protocol error state.",
            ),
            stmt(
                "lo",
                "If the target detects more errors, it enters the lockout state.",
            ),
            stmt(
                "rs",
                "When the SWD interface detects a line reset, it must enter the reset state.",
            ),
            stmt("dm", "The host places the target into the dormant state."),
        ];
        let states = extract_swd_line_states(&stmts);
        let names: Vec<&str> = states.iter().map(|s| s.state_name.as_str()).collect();
        for want in ["Protocol error", "Lockout", "Reset", "Dormant"] {
            assert!(names.contains(&want), "missing {want}: got {names:?}");
        }
        assert!(
            states
                .iter()
                .all(|s| s.machine_name.as_deref() == Some("SWD line state machine"))
        );
    }

    #[test]
    fn processor_debug_state_is_not_a_line_state() {
        // No SWD context → the processor "Debug state" must not become a line state.
        let stmts = vec![
            stmt("c", "A packet request is sent over SWDIO."),
            stmt(
                "d",
                "Facilities allow an external system to force the processor to enter Debug state.",
            ),
        ];
        let names: Vec<String> = extract_swd_line_states(&stmts)
            .into_iter()
            .map(|s| s.state_name)
            .collect();
        assert!(!names.iter().any(|n| n == "Debug"), "got {names:?}");
    }
}

#[cfg(test)]
mod swd_serial_extraction_4d_tidy {
    //! SWD-SERIAL-EXTRACTION.4d tidy — `to the <adj> operating state` captured; line-reset deduped;
    //! logic-level / verb garbage rejected.
    use super::*;

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn operating_captured_dups_and_garbage_rejected() {
        let stmts = vec![
            stmt("c", "A packet request is sent over the SWD line interface."),
            stmt(
                "op",
                "The debugger transitions the SWD target to the required operating state.",
            ),
            stmt(
                "lr",
                "The SWD interface enters line reset state on a line reset.",
            ),
            stmt("rs", "The SWD target must enter the reset state."),
            stmt(
                "hi",
                "The pull-up resistor returns the SWD line to the HIGH state.",
            ),
            stmt(
                "mt",
                "This SWD pull-up can be relied on to maintain the state of the wire.",
            ),
        ];
        let names: Vec<String> = extract_swd_line_states(&stmts)
            .into_iter()
            .map(|s| s.state_name)
            .collect();
        assert!(
            names.contains(&"Operating".to_string()),
            "operating captured: {names:?}"
        );
        assert!(
            names.contains(&"Reset".to_string()),
            "reset present: {names:?}"
        );
        // line-reset collapses into Reset (no separate "Line reset")
        assert!(
            !names.iter().any(|n| n == "Line reset"),
            "line-reset deduped: {names:?}"
        );
        // logic level + verb are not states
        assert!(
            !names.iter().any(|n| n == "High" || n == "Maintain the"),
            "garbage rejected: {names:?}"
        );
    }
}
