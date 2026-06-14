mod docling_backend;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::adapters::AdapterTarget;

pub use docling_backend::{
    DEFAULT_DOCLING_BOOTSTRAP_SCRIPT, DEFAULT_DOCLING_VENV_DIR, DOCLING_PYTHON_ENV,
    DoclingRuntimeCandidate, DoclingRuntimeCandidateStatus, DoclingRuntimeDiagnosis,
    DoclingRuntimeSource,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Pdf,
    Markdown,
    Directory,
    Unknown,
}

impl SourceKind {
    pub fn detect(path: &Path) -> Self {
        if path.is_dir() {
            return Self::Directory;
        }

        match normalized_extension(path).as_deref() {
            Some("pdf") => Self::Pdf,
            Some("md") | Some("markdown") => Self::Markdown,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Markdown => "markdown",
            Self::Directory => "directory",
            Self::Unknown => "unknown",
        }
    }
}

pub fn inspect_docling_runtime() -> Result<DoclingRuntimeDiagnosis> {
    docling_backend::inspect_docling_runtime()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourcePathKind {
    File,
    Directory,
    Other,
}

impl SourcePathKind {
    pub fn detect(metadata: &fs::Metadata) -> Self {
        if metadata.is_file() {
            Self::File
        } else if metadata.is_dir() {
            Self::Directory
        } else {
            Self::Other
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Directory => "directory",
            Self::Other => "other",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NormalizationStrategy {
    ExistingMarkdown,
    ExtractStructuredPdfArtifacts,
    InterpretDirectory,
    ResolveUnknownSource,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NormalizationBackend {
    DirectMarkdown,
    Docling,
    Marker,
    MistralOcr,
    Undecided,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NormalizationStatus {
    Ready,
    PlannedConversion,
    DecisionRequired,
}

impl NormalizationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::PlannedConversion => "planned_conversion",
            Self::DecisionRequired => "decision_required",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AutomationConfidence {
    High,
    Medium,
    Low,
}

impl AutomationConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Structured cell-level representation of one table extracted from a PDF.
/// The cells are Docling's native extraction; header rows are those where
/// Docling marks cells as `column_header` or `row_header`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuredTableRecord {
    pub table_id: String,
    /// Cross-references `VisualAsset.asset_id` for the corresponding table image.
    pub asset_id: String,
    pub page_id: Option<String>,
    pub caption_text: Option<String>,
    pub source_ref: Option<String>,
    /// Purpose of this table as inferred from its header cells at ingest time.
    #[serde(default)]
    pub table_kind: TableKind,
    /// Rows where at least one cell is marked as a header by Docling.
    pub header_rows: Vec<Vec<StructuredTableCellRecord>>,
    /// Non-header data rows.
    pub body_rows: Vec<Vec<StructuredTableCellRecord>>,
    pub row_count: u32,
    pub col_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuredTableCellRecord {
    pub text: String,
    pub row_span: u32,
    pub col_span: u32,
    pub is_header: bool,
}

/// Type label for a text content element as classified by Docling.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentElementKind {
    Title,
    Abstract,
    SectionHeader,
    BodyText,
    ListItem,
    Code,
    Caption,
    Footnote,
    Formula,
    Unknown,
}

/// One typed text element from the document, in reading order.
/// Page headers, footers, and empty elements are excluded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentElementRecord {
    pub element_id: String,
    pub kind: ContentElementKind,
    pub text: String,
    /// Heading level 1–6 for `SectionHeader` elements; `None` for all other kinds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_level: Option<u8>,
    pub page_id: Option<String>,
    pub source_ref: Option<String>,
    /// Position of this element in Docling's reading-order traversal.
    pub reading_order: u32,
}

/// Classification of a structured table's purpose, inferred from its header cells at ingest time.
/// Downstream stages (EvidenceIR, SemanticIR) use this to apply table-type-specific extraction.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TableKind {
    /// Signal name + direction/width columns (AHB manager/subordinate signal tables).
    SignalDescription,
    /// Value/encoding columns + name/description column (HTRANS, HBURST, HRESP encodings).
    Encoding,
    /// Offset/address + name + access type + reset value columns.
    RegisterMap,
    /// Parameter + min/typ/max + unit columns.
    TimingParameter,
    /// Feature/property + mandatory/optional/prohibited columns.
    FeatureMatrix,
    #[default]
    Unknown,
}

/// Section kind as heuristically classified from the heading title at ingest time.
/// Downstream stages can use this to avoid re-discovering section roles.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SectionKind {
    /// Legal text, revision history, proprietary notice, etc.
    Boilerplate,
    /// Normative protocol / specification content.
    Normative,
    /// Section describing signals, ports, or I/O.
    SignalDescription,
    /// Section describing registers, memory maps, or CSRs.
    RegisterDescription,
    /// Section containing timing diagrams or constraints.
    Timing,
    /// Glossary, abbreviations, definitions.
    Glossary,
    /// Table of contents.
    TableOfContents,
    /// Appendix or annex.
    Appendix,
    Unknown,
}

/// One section heading from the document, in reading order.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentSectionRecord {
    pub section_id: String,
    pub title: String,
    /// Heading depth 1–6 (h1 = 1, h2 = 2, …).
    pub heading_level: u8,
    pub page_id: Option<String>,
    pub source_ref: Option<String>,
    pub reading_order: u32,
    /// Heuristic section kind inferred from the heading title.
    pub section_kind: SectionKind,
}

/// Width specification for a hardware signal port or pin.
///
/// Bit widths in RTL design are either:
/// - **Numeric**: a fixed compile-time constant (always positive, typically a power of 2
///   or an even multiple: 1, 2, 3, 4, 8, 16, 32, 64, 128, 256, …)
/// - **Parametric**: a user-configurable RTL parameter that the integrator sets at
///   instantiation time (e.g. `ADDR_WIDTH = 32`, `DATA_WIDTH = 64`).
///   A parametric width is NOT unknown — it is a fully specified design intent whose
///   concrete value is supplied by whoever instantiates the IP.
///
/// **Serialization**: `WidthHint::Numeric(32)` → JSON `32`; `WidthHint::Parametric("ADDR_WIDTH")`
/// → JSON `"ADDR_WIDTH"`.  This is backward-compatible with the legacy `width_hint: u32` field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum WidthHint {
    /// A fixed numeric bit width (e.g. 1, 2, 32, 64).
    Numeric(u32),
    /// A user-configurable RTL parameter expression (e.g. "ADDR_WIDTH", "DATA_WIDTH/8").
    /// The actual value is bound at instantiation time by the integrator.
    Parametric(String),
}

impl WidthHint {
    /// Returns the numeric value if this is a fixed-width hint, otherwise `None`.
    pub fn as_numeric(&self) -> Option<u32> {
        match self {
            Self::Numeric(bits) => Some(*bits),
            Self::Parametric(_) => None,
        }
    }

    /// Returns `true` if this is a user-configurable parametric width.
    pub fn is_parametric(&self) -> bool {
        matches!(self, Self::Parametric(_))
    }
}

/// The kind of relation between an actor and a signal in the knowledge graph.
/// Extracted from prose verb phrases (Tier 2 pattern matching or Tier 3 LLM).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    /// The actor DRIVES / ASSERTS / OUTPUTS this signal — signal is an OUTPUT of this actor.
    /// Extracted from: "is driven by", "is asserted by", "drives", "asserts", etc.
    Drives,
    /// The actor READS / SAMPLES / MONITORS this signal — signal is an INPUT to this actor.
    /// Extracted from: "is read by", "is sampled by", "reads", "samples", etc.
    Reads,
}

/// One actor–signal relation extracted from a prose sentence.
/// Forms a node in the knowledge graph:
///   `(actor_name) —[Drives|Reads]—> (signal_name)`
///
/// Direction is derived: if `(A, Drives, S)`, then `S` is `output_of(A)` and
/// `input_of(others)`.
/// Actor identity is behavioral: names like “Manager”, “Requester”, “master”, “slave”
/// are all accepted as-is — the graph normalises them through co-occurrence,
/// not through a hardcoded vocabulary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorSignalRelation {
    pub relation_id: String,
    /// Actor name as it appears in the spec (e.g. "Manager", "slave", "Requester").
    pub actor_name: String,
    /// Uppercase signal name (e.g. "PREADY", "HTRANS", "AWADDR").
    pub signal_name: String,
    /// Whether the actor drives or reads the signal.
    pub relation: RelationKind,
    pub source_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// The kind of constraint that a `SignalConstraintRecord` captures.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum SignalConstraintKind {
    /// `SIGNAL must be HIGH` / `shall be asserted`.
    MustBeHigh,
    /// `SIGNAL must be LOW` / `shall be deasserted`.
    MustBeLow,
    /// `SIGNAL must be asserted` (polarity-neutral).
    MustBeAsserted,
    /// `SIGNAL must be deasserted` (polarity-neutral).
    MustBeDeasserted,
    /// `SIGNAL must not change` / `shall remain stable`.
    MustNotChange,
    /// `SIGNAL must be stable` throughout a phase.
    MustBeStable,
    /// `SIGNAL must hold data` / be held.
    MustHoldData,
    /// `SIGNAL must be VALUE` where VALUE is a specific protocol state (IDLE, NONSEQ, OKAY, etc.).
    MustBeValue { value: String },
}

/// A structured signal constraint extracted from a `SignalValueConstraint` sentence.
/// This is the Level 2 NLP output — not just a classified sentence but a typed record.
///
/// Example: `"HAUSER must not change between cycles when HREADY is LOW"` →
/// `{ subject: "HAUSER", kind: MustNotChange, condition: "when HREADY is LOW", negated: false }`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalConstraintRecord {
    pub constraint_id: String,
    /// The hardware signal that is being constrained.
    pub subject_signal: String,
    /// What the signal must do or be.
    pub constraint_kind: SignalConstraintKind,
    /// The specific target value/state, if applicable (e.g. "IDLE", "NONSEQ").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<String>,
    /// The condition clause, if present (e.g. "when HREADY is LOW").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_text: Option<String>,
    /// Whether the constraint was negated (`must not`, `shall not`).
    pub negated: bool,
    /// The original sentence this record was extracted from.
    pub source_text: String,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// A structured conditional rule extracted from a `ConditionalRule` sentence.
/// Captures `when ANTECEDENT, SIGNAL shall/must ACTION`.
///
/// Example: `"When HREADY is LOW, the Manager must not change HTRANS"` →
/// `{ antecedent: "HREADY is LOW", consequent_signal: "HTRANS", consequent_action: "must not change" }`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConditionalRuleRecord {
    pub rule_id: String,
    /// The condition that triggers the rule ("when HREADY is LOW").
    pub antecedent_text: String,
    /// The signal that is the subject of the consequent, if identifiable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consequent_signal: Option<String>,
    /// The action the consequent describes ("must not change", "shall be IDLE", etc.).
    pub consequent_action: String,
    /// The original sentence this record was extracted from.
    pub source_text: String,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// One register extracted from a register map table in the chip spec.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterRecord {
    pub register_id: String,
    pub register_name: String,
    /// Byte offset from the block base address (hexadecimal string, e.g. "0x04").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_address: Option<String>,
    /// Register width in bits when known (from a width column/caption, or the maximum field bit
    /// extent). Flexible-register-model field (PDF-VARIANT-DIGESTION.2c).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bits: Option<u32>,
    pub fields: Vec<RegisterFieldRecord>,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// One bit-field within a `RegisterRecord`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterFieldRecord {
    pub field_name: String,
    /// Most-significant bit position of the field within the register.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits_high: Option<u32>,
    /// Least-significant bit position — i.e. the field's OFFSET from the register's bit 0 (LSb).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bits_low: Option<u32>,
    /// Field WIDTH in bits. A field is normally `(offset = bits_low, width)`; for a `[high:low]` range
    /// this is `high - low + 1`, and a single-bit field has width 1. Flexible-register-model field
    /// (PDF-VARIANT-DIGESTION.2c) so offset+width forms are representable, not only bit ranges.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_width: Option<u32>,
    /// Access type: RO, WO, RW, RC, RS, W1C, WARL, RAZ/WI, … — kept as a FREE string so any vendor's
    /// notation is representable (flexible-register-model; PDF-VARIANT-DIGESTION.2c).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Enumerated value encodings of this field (value → meaning), e.g. `0b00 → Idle`. Empty when the
    /// field has no enumeration. Flexible-register-model field (PDF-VARIANT-DIGESTION.2c).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enumerated_values: Vec<RegisterFieldEnumRecord>,
}

/// One enumerated value of a [`RegisterFieldRecord`] — a value literal and its meaning.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterFieldEnumRecord {
    /// The value literal as written: `0b00`, `0x1`, `3`, `2'b01`, …
    pub value: String,
    /// What the value means.
    pub meaning: String,
}

/// One timing constraint extracted from a timing parameter table.
/// Parameter names typically follow the tXX convention (tSU, tHD, tCKH, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimingConstraintRecord {
    pub constraint_id: String,
    pub parameter_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// High-level statistics and document title extracted at ingest time.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentProfile {
    pub title: Option<String>,
    pub page_count: u32,
    pub table_count: u32,
    pub figure_count: u32,
    pub content_element_count: u32,
    pub section_count: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationFindingSeverity {
    Info,
    Warning,
    Error,
}

impl ValidationFindingSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationMetricRecord {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationFindingRecord {
    pub finding_id: String,
    pub severity: ValidationFindingSeverity,
    pub category: String,
    pub summary: String,
    #[serde(default)]
    pub related_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationReportRecord {
    pub report_id: String,
    pub validated_stage: IrStage,
    pub artifact_fingerprint: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_score: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<String>,
    #[serde(default)]
    pub metrics: Vec<ValidationMetricRecord>,
    #[serde(default)]
    pub findings: Vec<ValidationFindingRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub source: SourceRegistration,
    pub artifact_layout: SourceArtifactLayout,
    pub document_identity: DocumentIdentity,
    pub normalization_plan: NormalizationPlan,
    pub page_artifacts: Vec<PageArtifact>,
    pub visual_assets: Vec<VisualAsset>,
    /// Cell-level structured data for every table Docling extracted from the PDF.
    /// Empty for Markdown inputs.
    #[serde(default)]
    pub structured_tables: Vec<StructuredTableRecord>,
    /// Every non-page-header/footer text element in reading order, with type label.
    /// Empty for Markdown inputs.
    #[serde(default)]
    pub content_elements: Vec<ContentElementRecord>,
    /// All section headings in reading order with heading level and section kind.
    /// Empty for Markdown inputs.
    #[serde(default)]
    pub document_sections: Vec<ContentSectionRecord>,
    /// High-level document profile: title, counts.
    /// `None` for Markdown inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_profile: Option<DocumentProfile>,
    pub placeholder_bindings: Vec<PlaceholderBinding>,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
    #[serde(default)]
    pub validation_reports: Vec<ValidationReportRecord>,
    pub downstream_stages: Vec<IrStage>,
    pub adapter_targets: Vec<AdapterTarget>,
    pub planned_actions: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

impl SourceIr {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(AppError::MissingPath(path.to_path_buf()));
        }

        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }
    pub fn build(source: &Path, artifact_base_root: &Path) -> Result<Self> {
        if !source.exists() {
            return Err(AppError::MissingPath(source.to_path_buf()));
        }

        let metadata = fs::metadata(source)?;
        let canonical = fs::canonicalize(source)?;
        let path_kind = SourcePathKind::detect(&metadata);
        let source_kind = SourceKind::detect(source);
        let stable_artifact_stem = stable_stem(source);
        let document_key = document_key(&stable_artifact_stem);
        let artifact_root = artifact_base_root.join(&document_key);
        let normalized_root = artifact_root.join("normalized");
        let source_ir_path = artifact_root.join("source_ir.json");
        let page_image_root = normalized_root.join("pages");
        let visual_asset_root = normalized_root.join("assets");
        let page_artifact_manifest_path = normalized_root.join("page_artifacts.json");
        let visual_asset_manifest_path = normalized_root.join("visual_assets.json");
        let backend_raw_output_path = normalized_root.join(format!("{document_key}.backend.json"));

        let source_registration = SourceRegistration {
            requested_path: source.to_path_buf(),
            canonical_path: canonical.clone(),
            path_kind,
            source_kind,
            stable_artifact_stem,
            size_bytes: metadata.is_file().then_some(metadata.len()),
        };

        let document_identity = DocumentIdentity {
            document_key: document_key.clone(),
            display_name: display_name(source),
            origin_kind: source_kind,
        };

        let artifact_layout = SourceArtifactLayout {
            artifact_root,
            source_ir_path,
            normalized_root: normalized_root.clone(),
            page_image_root,
            page_artifact_manifest_path,
            visual_asset_root: visual_asset_root.clone(),
            visual_asset_manifest_path,
            backend_raw_output_path,
        };

        let mut residual_decisions = Vec::new();
        let normalization_plan = match source_kind {
            SourceKind::Markdown => NormalizationPlan {
                strategy: NormalizationStrategy::ExistingMarkdown,
                backend: NormalizationBackend::DirectMarkdown,
                status: NormalizationStatus::Ready,
                promoted_markdown_path: Some(canonical),
                auxiliary_asset_dir: None,
                metadata_output_path: None,
                notes: vec![
                    "source is already markdown and can flow directly into EvidenceIR construction"
                        .to_string(),
                    "markdown remains a convenient normalized view, but later stages may still resolve figure/image references into typed visual evidence records"
                        .to_string(),
                ],
            },
            SourceKind::Pdf => NormalizationPlan {
                strategy: NormalizationStrategy::ExtractStructuredPdfArtifacts,
                backend: NormalizationBackend::Docling,
                status: NormalizationStatus::PlannedConversion,
                promoted_markdown_path: Some(normalized_root.join(format!("{document_key}.md"))),
                auxiliary_asset_dir: Some(visual_asset_root),
                metadata_output_path: Some(
                    normalized_root.join(format!("{document_key}.meta.json")),
                ),
                notes: vec![
                    "pdf normalization remains a SourceIR-stage structured document-conversion activity before EvidenceIR construction"
                        .to_string(),
                    "the default local-first backend target is a Docling-style structured parser because it preserves reading order, pictures, captions, tables, and page geometry"
                        .to_string(),
                    "markdown is treated as a lossy convenience view over richer structured page and visual artifacts, not as the sole system of record"
                        .to_string(),
                ],
            },
            SourceKind::Directory => {
                residual_decisions.push(directory_source_packet());

                NormalizationPlan {
                    strategy: NormalizationStrategy::InterpretDirectory,
                    backend: NormalizationBackend::Undecided,
                    status: NormalizationStatus::DecisionRequired,
                    promoted_markdown_path: None,
                    auxiliary_asset_dir: None,
                    metadata_output_path: None,
                    notes: vec![
                        "directory ingest remains ambiguous until the tool can distinguish a converted bundle from a multi-document corpus"
                            .to_string(),
                    ],
                }
            }
            SourceKind::Unknown => {
                residual_decisions.push(unknown_source_packet());

                NormalizationPlan {
                    strategy: NormalizationStrategy::ResolveUnknownSource,
                    backend: NormalizationBackend::Undecided,
                    status: NormalizationStatus::DecisionRequired,
                    promoted_markdown_path: None,
                    auxiliary_asset_dir: None,
                    metadata_output_path: None,
                    notes: vec![
                        "unknown source kinds require either an external normalization step or a new built-in SourceIR strategy"
                            .to_string(),
                    ],
                }
            }
        };

        let automation_confidence = automation_confidence(source_kind, &residual_decisions);
        let planned_actions = planned_actions(source_kind, &residual_decisions);

        Ok(Self {
            schema_version: 1,
            stage: IrStage::SourceIr,
            source: source_registration,
            artifact_layout,
            document_identity,
            normalization_plan,
            page_artifacts: Vec::new(),
            visual_assets: Vec::new(),
            structured_tables: Vec::new(),
            content_elements: Vec::new(),
            document_sections: Vec::new(),
            document_profile: None,
            placeholder_bindings: Vec::new(),
            residual_decisions,
            validation_reports: Vec::new(),
            downstream_stages: vec![IrStage::EvidenceIr, IrStage::SemanticIr, IrStage::IntentIr],
            adapter_targets: vec![AdapterTarget::Isf],
            planned_actions,
            automation_confidence,
        })
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    pub fn materialize(&mut self) -> Result<()> {
        if !matches!(self.source.source_kind, SourceKind::Pdf) {
            return Ok(());
        }

        if matches!(self.normalization_plan.status, NormalizationStatus::Ready) {
            return Ok(());
        }

        let promoted_markdown_path = self
            .normalization_plan
            .promoted_markdown_path
            .clone()
            .ok_or_else(|| {
                AppError::InvalidBackendOutput(
                    "pdf normalization plan is missing a promoted markdown path".to_string(),
                )
            })?;
        let metadata_output_path = self
            .normalization_plan
            .metadata_output_path
            .clone()
            .ok_or_else(|| {
                AppError::InvalidBackendOutput(
                    "pdf normalization plan is missing a metadata output path".to_string(),
                )
            })?;

        let backend_summary = docling_backend::materialize_pdf(
            &self.source.canonical_path,
            &promoted_markdown_path,
            &metadata_output_path,
            &self.artifact_layout,
            &self.document_identity.document_key,
        )?;

        self.page_artifacts = backend_summary.page_artifacts;
        self.visual_assets = backend_summary.visual_assets;
        self.structured_tables = backend_summary.structured_tables;
        self.content_elements = backend_summary.content_elements;
        self.document_sections = backend_summary.document_sections;
        self.document_profile = backend_summary.document_profile;
        self.placeholder_bindings = backend_summary.placeholder_bindings;
        self.normalization_plan.status = NormalizationStatus::Ready;
        self.planned_actions = materialized_source_actions(&self.residual_decisions);
        self.normalization_plan.notes.push(format!(
            "docling materialized promoted markdown, backend raw JSON, {} page artifacts, {} picture assets, and {} table assets ({} structured)",
            backend_summary.metadata.page_count,
            backend_summary.metadata.picture_count,
            backend_summary.metadata.table_count,
            self.structured_tables.len()
        ));
        if let Some(version) = backend_summary.backend_version {
            self.normalization_plan
                .notes
                .push(format!("docling backend version: {version}"));
        }

        Ok(())
    }

    pub fn write_to_disk(&self) -> Result<()> {
        fs::create_dir_all(&self.artifact_layout.artifact_root)?;
        if matches!(self.source.source_kind, SourceKind::Pdf)
            && matches!(self.normalization_plan.status, NormalizationStatus::Ready)
        {
            fs::create_dir_all(&self.artifact_layout.normalized_root)?;
            fs::write(
                &self.artifact_layout.page_artifact_manifest_path,
                serde_json::to_string_pretty(&self.page_artifacts)?,
            )?;
            fs::write(
                &self.artifact_layout.visual_asset_manifest_path,
                serde_json::to_string_pretty(&self.visual_assets)?,
            )?;
        }
        fs::write(&self.artifact_layout.source_ir_path, self.to_pretty_json()?)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceRegistration {
    pub requested_path: PathBuf,
    pub canonical_path: PathBuf,
    pub path_kind: SourcePathKind,
    pub source_kind: SourceKind,
    pub stable_artifact_stem: String,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceArtifactLayout {
    pub artifact_root: PathBuf,
    pub source_ir_path: PathBuf,
    pub normalized_root: PathBuf,
    pub page_image_root: PathBuf,
    pub page_artifact_manifest_path: PathBuf,
    pub visual_asset_root: PathBuf,
    pub visual_asset_manifest_path: PathBuf,
    pub backend_raw_output_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentIdentity {
    pub document_key: String,
    pub display_name: String,
    pub origin_kind: SourceKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NormalizationPlan {
    pub strategy: NormalizationStrategy,
    pub backend: NormalizationBackend,
    pub status: NormalizationStatus,
    pub promoted_markdown_path: Option<PathBuf>,
    pub auxiliary_asset_dir: Option<PathBuf>,
    pub metadata_output_path: Option<PathBuf>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageArtifact {
    pub page_id: String,
    pub page_number: u32,
    pub page_image_path: Option<PathBuf>,
    pub layout_metadata_path: Option<PathBuf>,
    pub width_px: Option<u32>,
    pub height_px: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VisualAssetKind {
    Figure,
    Diagram,
    Chart,
    TableRegion,
    FormulaRegion,
    Screenshot,
    Unknown,
}

/// Semantic classification of a visual asset's diagram type.
/// Set from caption text heuristics at ingest time (zero VLM deps).
/// Used to route VLM enrichment calls at the `specforge enrich` step.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiagramKind {
    /// Waveform on horizontal time axis — the most normative content in chip specs.
    TimingDiagram,
    /// Boxes and arrows with guard labels (state/transition diagrams).
    StateMachineDiagram,
    /// Component rectangles with connections (system/block diagrams).
    BlockDiagram,
    /// Horizontal bit-field layout (register diagrams).
    RegisterBitfield,
    /// Tabular with binary inputs/outputs.
    TruthTable,
    /// Flow chart with diamond decision nodes.
    FlowChart,
    /// Cannot be determined from caption text alone; requires VLM.
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VisualAsset {
    pub asset_id: String,
    pub asset_kind: VisualAssetKind,
    pub page_id: Option<String>,
    pub image_path: Option<PathBuf>,
    pub caption_text: Option<String>,
    pub caption_source_path: Option<PathBuf>,
    pub source_ref: Option<String>,
    pub placeholder_text: Option<String>,
    pub note: Option<String>,
    /// Semantic diagram kind inferred from caption text at ingest time.
    /// Set to `Unknown` when classification cannot be determined from caption alone.
    #[serde(default)]
    pub diagram_kind: DiagramKind,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaceholderBinding {
    pub placeholder_text: String,
    pub asset_id: String,
    pub normalized_source_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidualDecisionPacket {
    pub packet_id: String,
    pub question: String,
    pub why_unresolved: String,
    pub automation_confidence: AutomationConfidence,
    pub candidate_interpretations: Vec<CandidateInterpretation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CandidateInterpretation {
    pub interpretation_id: String,
    pub description: String,
    pub downstream_impact: String,
}

fn automation_confidence(
    source_kind: SourceKind,
    residual_decisions: &[ResidualDecisionPacket],
) -> AutomationConfidence {
    if !residual_decisions.is_empty() {
        return AutomationConfidence::Low;
    }

    match source_kind {
        SourceKind::Markdown | SourceKind::Pdf => AutomationConfidence::High,
        SourceKind::Directory | SourceKind::Unknown => AutomationConfidence::Low,
    }
}

fn planned_actions(
    source_kind: SourceKind,
    residual_decisions: &[ResidualDecisionPacket],
) -> Vec<String> {
    let mut actions = Vec::new();

    if !residual_decisions.is_empty() {
        actions.push("resolve_source_ir_residual_decisions".to_string());
        return actions;
    }

    if matches!(source_kind, SourceKind::Pdf) {
        actions.push("run_structured_pdf_normalization".to_string());
        actions.push("extract_page_and_visual_artifacts".to_string());
        actions.push("materialize_promoted_source_artifacts".to_string());
        actions.push("run_multimodal_visual_enrichment".to_string());
    }

    actions.push("build_evidence_ir".to_string());
    actions.push("build_semantic_ir".to_string());
    actions.push("build_intent_ir".to_string());
    actions.push("plan_adapter_lowering".to_string());

    actions
}

fn materialized_source_actions(residual_decisions: &[ResidualDecisionPacket]) -> Vec<String> {
    if !residual_decisions.is_empty() {
        return vec!["resolve_source_ir_residual_decisions".to_string()];
    }

    vec![
        "build_evidence_ir".to_string(),
        "build_semantic_ir".to_string(),
        "build_intent_ir".to_string(),
        "plan_adapter_lowering".to_string(),
    ]
}

fn directory_source_packet() -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: "directory_source_ir_interpretation".to_string(),
        question: "How should this directory source be interpreted at the SourceIR stage?"
            .to_string(),
        why_unresolved: "A directory may represent a converter-produced bundle, a normalized documentation package, or a multi-document corpus; the current SourceIR slice cannot disambiguate that safely yet.".to_string(),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "converted_bundle".to_string(),
                description: "Treat the directory as a single converted-document bundle with one dominant markdown entrypoint plus metadata/assets.".to_string(),
                downstream_impact: "SourceIR should search for a primary markdown file and preserve sibling figures/metadata under one normalized source record.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "multi_document_corpus".to_string(),
                description: "Treat the directory as a corpus containing multiple independently ingestible specifications.".to_string(),
                downstream_impact: "SourceIR should either emit multiple source artifacts or require explicit document selection before EvidenceIR construction.".to_string(),
            },
        ],
    }
}

fn unknown_source_packet() -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: "unknown_source_ir_kind".to_string(),
        question: "What SourceIR normalization path should be used for this source kind?"
            .to_string(),
        why_unresolved: "The current SourceIR implementation only has built-in normalization strategies for PDF, Markdown, and directory inputs.".to_string(),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "external_pre_normalization".to_string(),
                description: "Normalize the source externally into Markdown before re-running ingest.".to_string(),
                downstream_impact: "The pipeline can continue with the existing markdown path once the source becomes a `.md` or `.markdown` file.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "extend_source_ir_support".to_string(),
                description: "Add a new built-in SourceIR normalization strategy for this source kind.".to_string(),
                downstream_impact: "The ingest layer must gain source-specific normalization logic before fully automated intent capture can proceed.".to_string(),
            },
        ],
    }
}

fn display_name(path: &Path) -> String {
    if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
        return name.to_string();
    }

    path.display().to_string()
}

pub fn stable_stem(path: &Path) -> String {
    if path.is_dir() {
        return path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("source")
            .to_string();
    }

    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("source")
        .to_string()
}

pub fn document_key(input: &str) -> String {
    let mut key = String::new();
    let mut last_was_separator = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            key.push(ch.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator && !key.is_empty() {
            key.push('_');
            last_was_separator = true;
        }
    }

    let key = key.trim_matches('_').to_string();

    if key.is_empty() {
        "source".to_string()
    } else {
        key
    }
}

fn normalized_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::IrStage;
    use crate::ir::source::AdapterTarget;
    use crate::test_support::env_var_lock;

    use super::{
        AutomationConfidence, NormalizationBackend, SourceIr, SourceKind, document_key, stable_stem,
    };

    struct EnvVarGuard {
        key: &'static str,
        original: Option<std::ffi::OsString>,
    }

    impl EnvVarGuard {
        fn set_path(key: &'static str, value: &Path) -> Self {
            let original = env::var_os(key);
            // SAFETY: tests serialize environment mutation with env_var_lock().
            unsafe { env::set_var(key, value) };
            Self { key, original }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            match &self.original {
                Some(value) => {
                    // SAFETY: tests serialize environment mutation with env_var_lock().
                    unsafe { env::set_var(self.key, value) };
                }
                None => {
                    // SAFETY: tests serialize environment mutation with env_var_lock().
                    unsafe { env::remove_var(self.key) };
                }
            }
        }
    }

    #[test]
    fn detects_pdf_extension_case_insensitively() {
        assert_eq!(SourceKind::detect(Path::new("spec.PDF")), SourceKind::Pdf);
    }

    #[test]
    fn detects_markdown_extensions() {
        assert_eq!(
            SourceKind::detect(Path::new("spec.md")),
            SourceKind::Markdown
        );
        assert_eq!(
            SourceKind::detect(Path::new("spec.markdown")),
            SourceKind::Markdown
        );
    }

    #[test]
    fn reports_unknown_for_other_extensions() {
        assert_eq!(
            SourceKind::detect(Path::new("spec.txt")),
            SourceKind::Unknown
        );
    }

    #[test]
    fn stable_stem_uses_file_stem_for_files() {
        assert_eq!(stable_stem(Path::new("foo/bar/spec.pdf")), "spec");
    }

    #[test]
    fn document_key_normalizes_non_identifier_characters() {
        assert_eq!(document_key("Spec Rev.A"), "spec_rev_a");
        assert_eq!(document_key("$$$"), "source");
    }

    #[test]
    fn markdown_source_ir_reuses_existing_markdown() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("README.md");
        fs::write(&source, "# hello\n")?;

        let source_ir = SourceIr::build(&source, &tempdir.path().join("artifacts"))?;

        assert_eq!(source_ir.stage, IrStage::SourceIr);
        assert_eq!(source_ir.source.source_kind, SourceKind::Markdown);
        assert_eq!(
            source_ir.normalization_plan.backend,
            NormalizationBackend::DirectMarkdown
        );
        assert_eq!(
            source_ir.normalization_plan.promoted_markdown_path,
            Some(fs::canonicalize(&source)?)
        );
        assert_eq!(source_ir.downstream_stages[0], IrStage::EvidenceIr);
        assert!(source_ir.residual_decisions.is_empty());
        assert!(source_ir.visual_assets.is_empty());
        assert!(source_ir.placeholder_bindings.is_empty());
        assert_eq!(source_ir.automation_confidence, AutomationConfidence::High);
        assert!(!source_ir.adapter_targets.is_empty());
        assert!(source_ir.adapter_targets.contains(&AdapterTarget::Isf));
        assert_eq!(
            source_ir.document_identity.origin_kind,
            SourceKind::Markdown
        );

        Ok(())
    }

    #[test]
    fn pdf_source_ir_plans_conversion_outputs() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("bus_spec.PDF");
        fs::write(&source, b"%PDF-1.0")?;

        let source_ir = SourceIr::build(&source, &tempdir.path().join("artifacts"))?;

        assert_eq!(source_ir.source.source_kind, SourceKind::Pdf);
        assert_eq!(
            source_ir.normalization_plan.backend,
            NormalizationBackend::Docling
        );
        assert_eq!(
            source_ir
                .normalization_plan
                .promoted_markdown_path
                .as_ref()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str()),
            Some("bus_spec.md")
        );
        assert!(
            source_ir
                .planned_actions
                .contains(&"run_structured_pdf_normalization".to_string())
        );
        assert_eq!(
            source_ir
                .artifact_layout
                .page_artifact_manifest_path
                .file_name()
                .and_then(|name| name.to_str()),
            Some("page_artifacts.json")
        );
        assert_eq!(
            source_ir
                .artifact_layout
                .visual_asset_manifest_path
                .file_name()
                .and_then(|name| name.to_str()),
            Some("visual_assets.json")
        );
        assert!(source_ir.residual_decisions.is_empty());
        assert_eq!(source_ir.automation_confidence, AutomationConfidence::High);
        assert!(!source_ir.adapter_targets.is_empty());
        assert_eq!(source_ir.document_identity.origin_kind, SourceKind::Pdf);

        Ok(())
    }

    #[test]
    fn directory_source_ir_emits_residual_decision_packet() -> Result<()> {
        let tempdir = tempdir()?;
        let source_dir = tempdir.path().join("converted_bundle");
        fs::create_dir_all(&source_dir)?;

        let source_ir = SourceIr::build(&source_dir, &tempdir.path().join("artifacts"))?;

        assert_eq!(source_ir.source.source_kind, SourceKind::Directory);
        assert_eq!(source_ir.residual_decisions.len(), 1);
        assert_eq!(
            source_ir.residual_decisions[0].packet_id,
            "directory_source_ir_interpretation"
        );
        assert_eq!(
            source_ir.planned_actions,
            vec!["resolve_source_ir_residual_decisions".to_string()]
        );
        assert_eq!(source_ir.automation_confidence, AutomationConfidence::Low);
        assert_eq!(
            source_ir.residual_decisions[0].automation_confidence,
            AutomationConfidence::Low
        );

        Ok(())
    }

    #[test]
    fn write_to_disk_materializes_source_ir_json() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("README.md");
        let artifact_base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# hello\n")?;

        let source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.write_to_disk()?;

        let source_ir_json =
            fs::read_to_string(artifact_base.join("readme").join("source_ir.json"))?;
        assert!(source_ir_json.contains("\"document_key\": \"readme\""));
        assert!(source_ir_json.contains("\"source_kind\": \"markdown\""));
        assert!(source_ir_json.contains("\"stage\": \"source_ir\""));
        assert!(source_ir_json.contains("\"backend\": \"direct_markdown\""));
        assert!(source_ir_json.contains("\"automation_confidence\": \"high\""));

        Ok(())
    }

    #[test]
    fn pdf_source_ir_materialization_uses_backend_helper_and_writes_manifests() -> Result<()> {
        let _env_lock = env_var_lock();
        let tempdir = tempdir()?;
        let source = tempdir.path().join("bus_spec.pdf");
        let artifact_base = tempdir.path().join("generated").join("source_ir");
        let helper = tempdir.path().join("docling_stub.sh");

        fs::write(&source, b"%PDF-1.0")?;
        fs::write(
            &helper,
            r##"#!/bin/sh
while [ "$#" -gt 0 ]; do
  case "$1" in
    --markdown) markdown="$2"; shift 2 ;;
    --page-image-root) page_image_root="$2"; shift 2 ;;
    --visual-asset-root) visual_asset_root="$2"; shift 2 ;;
    --backend-raw-output) backend_raw_output="$2"; shift 2 ;;
    --metadata-output) metadata_output="$2"; shift 2 ;;
    --summary-output) summary_output="$2"; shift 2 ;;
    *) shift ;;
  esac
done

mkdir -p "$(dirname "$markdown")" "$page_image_root" "$visual_asset_root"
printf '# normalized\n\n![Image](assets/picture-0001.png)\n' > "$markdown"
printf '{}' > "$backend_raw_output"
printf '{"backend":"docling_stub"}\n' > "$metadata_output"
printf 'stub-page' > "$page_image_root/page-0001.png"
printf '{"page_number":1}\n' > "$page_image_root/page-0001.json"
printf 'stub-asset' > "$visual_asset_root/picture-0001.png"
cat > "$summary_output" <<EOF
{
  "backend_version": "stub-1.0",
  "page_artifacts": [
    {
      "page_id": "page_0001",
      "page_number": 1,
      "page_image_path": "$page_image_root/page-0001.png",
      "layout_metadata_path": "$page_image_root/page-0001.json",
      "width_px": 800,
      "height_px": 600
    }
  ],
  "visual_assets": [
    {
      "asset_id": "picture_0001",
      "asset_kind": "figure",
      "page_id": "page_0001",
      "image_path": "$visual_asset_root/picture-0001.png",
      "caption_text": "Stub figure",
      "caption_source_path": "$backend_raw_output",
      "source_ref": "#/pictures/0",
      "placeholder_text": null,
      "note": null
    }
  ],
  "placeholder_bindings": [],
  "metadata": {
    "page_count": 1,
    "picture_count": 1,
    "table_count": 0
  }
}
EOF
"##,
        )?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            fs::set_permissions(&helper, fs::Permissions::from_mode(0o755))?;
        }

        let _env_guard = EnvVarGuard::set_path("SPECFORGE_DOCLING_HELPER", &helper);
        // Disable the built-in RAM guard so a hot CI host (>85% used) cannot false-abort the
        // stub-helper ingest; behavior is then identical to the pre-guard blocking path
        // (MEMORY-BOUNDED-INGEST.4a).
        let _ram_guard =
            EnvVarGuard::set_path("SPECFORGE_INGEST_RAM_ABORT_PERCENT", Path::new("off"));
        let mut source_ir = SourceIr::build(&source, &artifact_base)?;

        source_ir.materialize()?;
        source_ir.write_to_disk()?;

        assert_eq!(
            source_ir.normalization_plan.status,
            super::NormalizationStatus::Ready
        );
        assert_eq!(source_ir.page_artifacts.len(), 1);
        assert_eq!(source_ir.visual_assets.len(), 1);
        assert_eq!(
            source_ir.visual_assets[0].source_ref.as_deref(),
            Some("#/pictures/0")
        );
        assert!(
            source_ir.page_artifacts[0]
                .page_image_path
                .as_ref()
                .is_some_and(|path| path
                    .ends_with("generated/source_ir/bus_spec/normalized/pages/page-0001.png"))
        );
        assert_eq!(
            source_ir.planned_actions,
            vec![
                "build_evidence_ir".to_string(),
                "build_semantic_ir".to_string(),
                "build_intent_ir".to_string(),
                "plan_adapter_lowering".to_string()
            ]
        );
        assert_eq!(source_ir.automation_confidence, AutomationConfidence::High);
        assert!(source_ir.source.size_bytes.is_some());
        assert!(source_ir.source.size_bytes.unwrap() > 0);
        assert!(!source_ir.normalization_plan.notes.is_empty());
        assert!(
            source_ir
                .normalization_plan
                .notes
                .iter()
                .any(|note| note.contains("docling materialized"))
        );
        assert_eq!(source_ir.page_artifacts[0].width_px, Some(800));
        assert_eq!(source_ir.page_artifacts[0].height_px, Some(600));
        assert_eq!(
            source_ir.visual_assets[0].diagram_kind,
            crate::ir::source::DiagramKind::Unknown
        );

        let source_ir_json =
            fs::read_to_string(artifact_base.join("bus_spec").join("source_ir.json"))?;
        let page_manifest = fs::read_to_string(
            artifact_base
                .join("bus_spec")
                .join("normalized")
                .join("page_artifacts.json"),
        )?;
        let visual_manifest = fs::read_to_string(
            artifact_base
                .join("bus_spec")
                .join("normalized")
                .join("visual_assets.json"),
        )?;
        let promoted_markdown = fs::read_to_string(
            artifact_base
                .join("bus_spec")
                .join("normalized")
                .join("bus_spec.md"),
        )?;

        assert!(source_ir_json.contains("\"status\": \"ready\""));
        assert!(page_manifest.contains("\"page_id\": \"page_0001\""));
        assert!(visual_manifest.contains("\"source_ref\": \"#/pictures/0\""));
        assert!(promoted_markdown.contains("![Image](assets/picture-0001.png)"));

        Ok(())
    }

    #[test]
    fn pdf_source_ir_materialization_replaces_stale_normalized_artifacts() -> Result<()> {
        let _env_lock = env_var_lock();
        let tempdir = tempdir()?;
        let source = tempdir.path().join("bus_spec.pdf");
        let artifact_base = tempdir.path().join("generated").join("source_ir");
        let helper = tempdir.path().join("docling_stub.sh");
        let stale_root = artifact_base.join("bus_spec").join("normalized");

        fs::write(&source, b"%PDF-1.0")?;
        fs::create_dir_all(&stale_root)?;
        fs::write(stale_root.join("stale.txt"), b"old")?;
        fs::write(
            &helper,
            r##"#!/bin/sh
while [ "$#" -gt 0 ]; do
  case "$1" in
    --markdown) markdown="$2"; shift 2 ;;
    --page-image-root) page_image_root="$2"; shift 2 ;;
    --visual-asset-root) visual_asset_root="$2"; shift 2 ;;
    --backend-raw-output) backend_raw_output="$2"; shift 2 ;;
    --metadata-output) metadata_output="$2"; shift 2 ;;
    --summary-output) summary_output="$2"; shift 2 ;;
    *) shift ;;
  esac
done

mkdir -p "$(dirname "$markdown")" "$page_image_root" "$visual_asset_root"
printf '# normalized\n' > "$markdown"
printf '{}' > "$backend_raw_output"
printf '{"backend":"docling_stub"}\n' > "$metadata_output"
printf 'stub-page' > "$page_image_root/page-0001.png"
printf '{"page_number":1}\n' > "$page_image_root/page-0001.json"
printf 'stub-asset' > "$visual_asset_root/picture-0001.png"
cat > "$summary_output" <<EOF
{
  "backend_version": "stub-1.0",
  "page_artifacts": [
    {
      "page_id": "page_0001",
      "page_number": 1,
      "page_image_path": "$page_image_root/page-0001.png",
      "layout_metadata_path": "$page_image_root/page-0001.json",
      "width_px": 800,
      "height_px": 600
    }
  ],
  "visual_assets": [
    {
      "asset_id": "picture_0001",
      "asset_kind": "figure",
      "page_id": "page_0001",
      "image_path": "$visual_asset_root/picture-0001.png",
      "caption_text": "Stub figure",
      "caption_source_path": "$backend_raw_output",
      "source_ref": "#/pictures/0",
      "placeholder_text": null,
      "note": null
    }
  ],
  "placeholder_bindings": [],
  "metadata": {
    "page_count": 1,
    "picture_count": 1,
    "table_count": 0
  }
}
EOF
"##,
        )?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            fs::set_permissions(&helper, fs::Permissions::from_mode(0o755))?;
        }

        let _env_guard = EnvVarGuard::set_path("SPECFORGE_DOCLING_HELPER", &helper);
        // Disable the built-in RAM guard so a hot CI host (>85% used) cannot false-abort the
        // stub-helper ingest; behavior is then identical to the pre-guard blocking path
        // (MEMORY-BOUNDED-INGEST.4a).
        let _ram_guard =
            EnvVarGuard::set_path("SPECFORGE_INGEST_RAM_ABORT_PERCENT", Path::new("off"));
        let mut source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.materialize()?;
        source_ir.write_to_disk()?;

        assert!(!stale_root.join("stale.txt").exists());
        assert!(stale_root.join("bus_spec.md").exists());
        assert!(
            !artifact_base
                .join("bus_spec")
                .join("normalized.staging")
                .exists()
        );
        assert_eq!(source_ir.automation_confidence, AutomationConfidence::High);

        Ok(())
    }

    #[test]
    fn pdf_source_ir_failed_materialization_keeps_existing_normalized_artifacts() -> Result<()> {
        let _env_lock = env_var_lock();
        let tempdir = tempdir()?;
        let source = tempdir.path().join("bus_spec.pdf");
        let artifact_base = tempdir.path().join("generated").join("source_ir");
        let helper = tempdir.path().join("docling_stub.sh");
        let normalized_root = artifact_base.join("bus_spec").join("normalized");

        fs::write(&source, b"%PDF-1.0")?;
        fs::create_dir_all(&normalized_root)?;
        fs::write(normalized_root.join("keep.txt"), b"last-good-run")?;
        fs::write(
            &helper,
            r##"#!/bin/sh
exit 7
"##,
        )?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            fs::set_permissions(&helper, fs::Permissions::from_mode(0o755))?;
        }

        let _env_guard = EnvVarGuard::set_path("SPECFORGE_DOCLING_HELPER", &helper);
        // Disable the built-in RAM guard so a hot CI host (>85% used) cannot false-abort the
        // stub-helper ingest; behavior is then identical to the pre-guard blocking path
        // (MEMORY-BOUNDED-INGEST.4a).
        let _ram_guard =
            EnvVarGuard::set_path("SPECFORGE_INGEST_RAM_ABORT_PERCENT", Path::new("off"));
        let mut source_ir = SourceIr::build(&source, &artifact_base)?;
        let error = source_ir
            .materialize()
            .expect_err("materialization should fail");

        assert!(error.to_string().contains("exit code: 7"));
        assert!(normalized_root.join("keep.txt").exists());
        assert!(
            !artifact_base
                .join("bus_spec")
                .join("normalized.staging")
                .exists()
        );
        assert_eq!(source_ir.automation_confidence, AutomationConfidence::High);

        Ok(())
    }
}
