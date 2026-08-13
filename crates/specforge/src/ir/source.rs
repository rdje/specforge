mod docling_backend;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::adapters::AdapterTarget;
use crate::ir::derivation::{
    AlphaObligation, ClaimAddress, DerivationError, DerivationResult, PremiseKind, PremiseRef,
    PromotionKernelBuilder, ProofConfidence, ProofLedger, RuleCompatibility, RuleDescriptor,
    RuleId, RuleRegistration, RuleRegistry, RuleVerificationContext, Sha256Digest,
    SymbolCapabilityClass, VerifiedProofLedger, production_semantic_implementation_digest,
};
use crate::persisted_path::{
    PersistedPathOrigin, infer_existing_origin, normalize_for_storage, resolve_existing,
    resolve_reference, resolve_repository_output,
};

const SOURCE_IR_SCHEMA_VERSION: u32 = 3;
const SOURCE_PROOF_CONTEXT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct SourceProofContext {
    schema_version: u32,
    field_premises: BTreeMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    grounded_proposals: Vec<SourceGroundedProposal>,
    /// Synthetic extractor fixtures are deliberately a different authority class. This field and
    /// its verifier branch do not exist in production builds, whose deny-unknown-fields decoder
    /// therefore rejects a fixture artifact.
    #[cfg(any(test, feature = "test-support"))]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    test_fixture: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum SourceGroundedProposal {
    VisualObservation {
        proposal_id: String,
        asset_id: String,
        diagram_kind: DiagramKind,
        exact_response: String,
    },
    TableGridRepair {
        proposal_id: String,
        table_id: String,
        exact_response: String,
    },
    TableClassification {
        proposal_id: String,
        table_id: String,
        exact_response: String,
    },
}

impl SourceGroundedProposal {
    fn proposal_id(&self) -> &str {
        match self {
            Self::VisualObservation { proposal_id, .. }
            | Self::TableGridRepair { proposal_id, .. }
            | Self::TableClassification { proposal_id, .. } => proposal_id,
        }
    }

    fn target_index(
        &self,
        visual_assets: &[VisualAsset],
        tables: &[StructuredTableRecord],
    ) -> DerivationResult<usize> {
        match self {
            Self::VisualObservation { asset_id, .. } => visual_assets
                .iter()
                .position(|asset| asset.asset_id == *asset_id)
                .ok_or_else(|| {
                    DerivationError::new(format!(
                        "grounded visual proposal targets absent asset '{asset_id}'"
                    ))
                }),
            Self::TableGridRepair { table_id, .. } | Self::TableClassification { table_id, .. } => {
                tables
                    .iter()
                    .position(|table| table.table_id == *table_id)
                    .ok_or_else(|| {
                        DerivationError::new(format!(
                            "grounded table proposal targets absent table '{table_id}'"
                        ))
                    })
            }
        }
    }
}

fn source_grounded_proposal_surface(proposal: &SourceGroundedProposal) -> &'static str {
    match proposal {
        SourceGroundedProposal::VisualObservation { .. } => "visual_assets",
        SourceGroundedProposal::TableGridRepair { .. }
        | SourceGroundedProposal::TableClassification { .. } => "structured_tables",
    }
}

#[derive(Debug)]
struct SourceClaimInput {
    address: ClaimAddress,
    rule_id: RuleId,
    premise_key: String,
    confidence: ProofConfidence,
    conclusion: serde_json::Value,
}

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
    /// Purpose of this table as proven by generic structural roles at ingest time.
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

/// Classification of a structured table's purpose, proven from generic structural roles at ingest
/// time. Downstream stages (EvidenceIR, SemanticIR) use this to apply table-type-specific
/// extraction.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TableKind {
    /// Signal name plus direction or width columns.
    SignalDescription,
    /// Value or encoding columns plus a name or description column.
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

/// Column authority for the current scalar timing-constraint schema.
///
/// Docling can place data rows in `header_rows` when their row-label cell is marked as a
/// row header.  Only the leading rows whose non-empty cells are all actual header cells may
/// therefore name columns.  Header words keep `_` as an identifier character, so `instruction`
/// is not `ns` and `OPTIMAL_TRIM_UNIT_SIZE` is not a standalone `unit`.  A layout is accepted only
/// when each scalar MIN/TYP/MAX role maps to at most one column and the table also carries timing
/// context (a parameter/symbol header, an explicit unit marker, or timing/unit words in its
/// caption).  Variant tables with several MIN/MAX columns remain residual because
/// `TimingConstraintRecord` cannot preserve the variant dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TimingTableColumns {
    pub name: usize,
    pub min: Option<usize>,
    pub typ: Option<usize>,
    pub max: Option<usize>,
    pub unit: Option<usize>,
    pub description: Option<usize>,
}

fn timing_header_words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .filter(|word| !word.is_empty())
}

fn timing_text_has_word(text: &str, expected: &[&str]) -> bool {
    timing_header_words(text).any(|word| {
        expected
            .iter()
            .any(|candidate| word.eq_ignore_ascii_case(candidate))
    })
}

/// Recover a table-wide timing unit only from an explicit caption declaration such as
/// `all values in ns`. The grammar requires the complete cue and a closed timing-unit token, so a
/// caption that merely mentions a unit or says values are "in nominal order" cannot manufacture a
/// unit for every row. The source spelling is preserved.
pub(crate) fn timing_caption_unit(table: &StructuredTableRecord) -> Option<String> {
    let words = timing_header_words(table.caption_text.as_deref()?).collect::<Vec<_>>();
    for (index, window) in words.windows(4).enumerate() {
        if !window[0].eq_ignore_ascii_case("all")
            || !matches!(window[1].to_ascii_lowercase().as_str(), "value" | "values")
            || !window[2].eq_ignore_ascii_case("in")
        {
            continue;
        }
        let unit = window[3];
        if ["s", "ms", "us", "ns", "ps", "fs", "cycle", "cycles", "ui"]
            .iter()
            .any(|candidate| unit.eq_ignore_ascii_case(candidate))
        {
            return Some(unit.to_string());
        }
        if unit.eq_ignore_ascii_case("clock")
            && words
                .get(index + 4)
                .is_some_and(|word| word.eq_ignore_ascii_case("cycles"))
        {
            return Some(format!("{unit} {}", words[index + 4]));
        }
    }
    None
}

fn timing_header_cell_is_identity(text: &str, include_name: bool) -> bool {
    let normalized = text.trim();
    normalized.eq_ignore_ascii_case("parameter")
        || normalized.eq_ignore_ascii_case("parameters")
        || normalized.eq_ignore_ascii_case("symbol")
        || normalized.eq_ignore_ascii_case("symbols")
        || (include_name && normalized.eq_ignore_ascii_case("name"))
}

fn unique_timing_column(
    columns: &[Vec<&str>],
    vocabulary: &[&str],
) -> std::result::Result<Option<usize>, ()> {
    let matches = columns
        .iter()
        .enumerate()
        .filter_map(|(index, texts)| {
            texts
                .iter()
                .any(|text| timing_text_has_word(text, vocabulary))
                .then_some(index)
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Ok(None),
        [index] => Ok(Some(*index)),
        _ => Err(()),
    }
}

fn timing_structural_header_columns(table: &StructuredTableRecord) -> Option<Vec<Vec<&str>>> {
    let structural_header_rows = table
        .header_rows
        .iter()
        .take_while(|row| {
            let non_empty = row
                .iter()
                .filter(|cell| !cell.text.trim().is_empty())
                .collect::<Vec<_>>();
            !non_empty.is_empty() && non_empty.iter().all(|cell| cell.is_header)
        })
        .collect::<Vec<_>>();
    let column_count = structural_header_rows.iter().map(|row| row.len()).max()?;
    let mut columns = vec![Vec::new(); column_count];
    for row in structural_header_rows {
        for (index, cell) in row.iter().enumerate() {
            let text = cell.text.trim();
            if !text.is_empty() {
                columns[index].push(text);
            }
        }
    }
    Some(columns)
}

fn timing_columns_have_context(table: &StructuredTableRecord, columns: &[Vec<&str>]) -> bool {
    let has_parameter_or_symbol = columns.iter().any(|texts| {
        texts
            .iter()
            .any(|text| timing_header_cell_is_identity(text, false))
    });
    let has_unit = columns.iter().any(|texts| {
        texts.iter().any(|text| {
            timing_text_has_word(text, &["unit", "units", "ns", "ps", "cycles", "period"])
        })
    });
    let caption_has_timing_context = table.caption_text.as_deref().is_some_and(|caption| {
        timing_text_has_word(
            caption,
            &["timing", "timings", "ns", "ps", "cycles", "period"],
        )
    });
    has_parameter_or_symbol || has_unit || caption_has_timing_context
}

/// Whether structural column headers establish that this is a timing/limits table at all.
/// This category authority is intentionally broader than `timing_table_columns`: a table with
/// several variant-specific MIN/MAX pairs is genuinely timing-bearing but cannot be collapsed into
/// the current scalar record without losing its variant dimension. It stays classified as timing
/// and becomes an explicit unexplained-table residual instead of falling through to another table
/// extractor.
pub fn timing_table_has_structural_authority(table: &StructuredTableRecord) -> bool {
    let Some(columns) = timing_structural_header_columns(table) else {
        return false;
    };
    let has_value_role = columns.iter().flatten().any(|text| {
        timing_text_has_word(
            text,
            &[
                "min", "minimum", "typ", "typical", "nominal", "max", "maximum",
            ],
        )
    });
    has_value_role && timing_columns_have_context(table, &columns)
}

pub(crate) fn timing_table_columns(table: &StructuredTableRecord) -> Option<TimingTableColumns> {
    let columns = timing_structural_header_columns(table)?;

    let min = unique_timing_column(&columns, &["min", "minimum"]).ok()?;
    let typ = unique_timing_column(&columns, &["typ", "typical", "nominal"]).ok()?;
    let max = unique_timing_column(&columns, &["max", "maximum"]).ok()?;
    let value_columns = [min, typ, max].into_iter().flatten().collect::<Vec<_>>();
    if value_columns.is_empty()
        || value_columns
            .iter()
            .enumerate()
            .any(|(index, column)| value_columns[..index].contains(column))
    {
        return None;
    }

    let name = columns
        .iter()
        .position(|texts| {
            texts
                .iter()
                .any(|text| timing_header_cell_is_identity(text, true))
        })
        .unwrap_or(0);
    let unit_columns = columns
        .iter()
        .enumerate()
        .filter_map(|(index, texts)| {
            texts
                .iter()
                .any(|text| {
                    timing_text_has_word(text, &["unit", "units", "ns", "ps", "cycles", "period"])
                })
                .then_some(index)
        })
        .collect::<Vec<_>>();
    if !timing_columns_have_context(table, &columns) {
        return None;
    }

    let unit = unit_columns
        .into_iter()
        .find(|column| !value_columns.contains(column));
    let description = columns.iter().position(|texts| {
        texts
            .iter()
            .any(|text| timing_text_has_word(text, &["description"]))
    });
    Some(TimingTableColumns {
        name,
        min,
        typ,
        max,
        unit,
        description,
    })
}

fn normalize_timing_table_kinds(tables: &mut [StructuredTableRecord]) {
    for table in tables {
        if matches!(table.table_kind, TableKind::TimingParameter)
            && !timing_table_has_structural_authority(table)
        {
            table.table_kind = TableKind::Unknown;
        }
    }
}

/// Retained schema-1 artifacts were classified by corpus-calibrated caption/header shortcuts.
/// Their source geometry and text remain valid, but their semantic labels do not satisfy the
/// current genericity contract. Fail closed instead of silently carrying that authority across
/// the boundary; re-ingest reconstructs labels with the current document-independent grammar.
fn neutralize_legacy_source_classifications(source_ir: &mut SourceIr) {
    let previous_schema = source_ir.schema_version;
    for asset in &mut source_ir.visual_assets {
        asset.diagram_kind = DiagramKind::Unknown;
    }
    for table in &mut source_ir.structured_tables {
        table.table_kind = TableKind::Unknown;
    }
    for section in &mut source_ir.document_sections {
        section.section_kind = SectionKind::Unknown;
    }
    source_ir.normalization_plan.notes.push(format!(
        "loaded legacy SourceIR schema {previous_schema} for inspection only; semantic source classifications were neutralized because only schema {SOURCE_IR_SCHEMA_VERSION} plus a verified proof ledger carries canonical authority; re-ingest to reconstruct typed classifications"
    ));
}

/// Section kind classified from an explicit, document-independent heading grammar at ingest time.
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
    /// No registered heading-form classification is available.
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
    /// Section kind proven by the document-independent heading grammar.
    pub section_kind: SectionKind,
}

/// Width specification for a hardware signal port or pin.
///
/// Bit widths in RTL design are either:
/// - **Numeric**: a fixed compile-time constant (always positive, typically a power of 2
///   or an even multiple: 1, 2, 3, 4, 8, 16, 32, 64, 128, 256, …)
/// - **Parametric**: a user-configurable RTL parameter that the integrator sets at
///   instantiation time.
///   A parametric width is NOT unknown — it is a fully specified design intent whose
///   concrete value is supplied by whoever instantiates the IP.
///
/// **Serialization**: a numeric variant becomes a JSON number; a parametric variant becomes the
/// source-defined JSON string. This is backward-compatible with the legacy `width_hint: u32` field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum WidthHint {
    /// A fixed numeric bit width.
    Numeric(u32),
    /// A user-configurable RTL parameter expression derived from the input.
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
/// Actor identity is behavioral: every input-defined participant name is accepted as-is and the
/// graph normalizes it through current-document evidence, not a hardcoded vocabulary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorSignalRelation {
    pub relation_id: String,
    /// Actor name as it appears in the current input.
    pub actor_name: String,
    /// Signal name as it appears in the current input.
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
    /// `SIGNAL must be VALUE` where VALUE is an input-defined state or encoding.
    MustBeValue { value: String },
}

impl SignalConstraintKind {
    /// Stable generic label for diagnostics, evaluation, and serialized reports.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MustBeHigh => "must_be_high",
            Self::MustBeLow => "must_be_low",
            Self::MustBeAsserted => "must_be_asserted",
            Self::MustBeDeasserted => "must_be_deasserted",
            Self::MustNotChange => "must_not_change",
            Self::MustBeStable => "must_be_stable",
            Self::MustHoldData => "must_hold_data",
            Self::MustBeValue { .. } => "must_be_value",
        }
    }
}

/// A structured signal constraint extracted from a `SignalValueConstraint` sentence.
/// This is the Level 2 NLP output — not just a classified sentence but a typed record.
///
/// A source-derived signal, action, and optional condition become one typed record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalConstraintRecord {
    pub constraint_id: String,
    /// The hardware signal that is being constrained.
    pub subject_signal: String,
    /// What the signal must do or be.
    pub constraint_kind: SignalConstraintKind,
    /// The input-defined target value or state, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<String>,
    /// The condition clause, if present.
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
/// The antecedent, consequent subject, and action are carried from the current input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConditionalRuleRecord {
    pub rule_id: String,
    /// The condition that triggers the rule.
    pub antecedent_text: String,
    /// The signal that is the subject of the consequent, if identifiable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consequent_signal: Option<String>,
    /// The action the consequent describes.
    pub consequent_action: String,
    /// The original sentence this record was extracted from.
    pub source_text: String,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// One register extracted from a register map table in the current input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterRecord {
    pub register_id: String,
    pub register_name: String,
    /// Register-level access policy when the source map declares one. This is distinct from
    /// per-field access in [`RegisterFieldRecord`]; both use a free string so source notation and
    /// footnote markers remain lossless.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    /// Byte offset from the block base address, retaining the input notation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_address: Option<String>,
    /// Register width in bits when known (from a width column/caption, or the maximum field bit
    /// extent).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_bits: Option<u32>,
    pub fields: Vec<RegisterFieldRecord>,
    /// Structured-table identities that directly support this register or its fields.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_table_ids: Vec<String>,
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
    /// Field width in bits. A field is normally `(offset = bits_low, width)`; for a `[high:low]`
    /// range this is `high - low + 1`, and a single-bit field has width 1. Offset-plus-width forms
    /// therefore remain representable rather than only bit ranges.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_width: Option<u32>,
    /// Access type retained as a free source-defined string so unfamiliar notation remains
    /// representable without a built-in vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Enumerated value encodings of this field (value → meaning). Empty when the field has no
    /// enumeration.
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

/// Quantity domain that can prove a scalar table row is outside executable digital intent.
///
/// This is deliberately a closed, evidence-derived vocabulary. A producer may add a domain only
/// when the source unit proves it without relying on a parameter, document, vendor, or table name.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum NonApplicableTimingQuantityDomain {
    /// A logarithmic amplitude/power ratio such as dB, dB RMS, or dBc/Hz.
    Decibel,
}

/// First canonical-promotion boundary at which a captured timing/limits observation is rejected.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum TimingIntentBoundary {
    SourceToEvidenceIr,
}

/// Whether a captured scalar timing/limits row may cross the executable-digital boundary.
///
/// Legacy records omit this field and therefore retain their historical canonical disposition.
/// New non-applicable records carry the causal reason, first boundary, and replay instruction so
/// the physical source fact remains actionable instead of disappearing from the IR chain.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum TimingIntentDisposition {
    #[default]
    Canonical,
    NonApplicable {
        quantity_domain: NonApplicableTimingQuantityDomain,
        reason: String,
        first_failing_stage: TimingIntentBoundary,
        replay: String,
    },
}

impl TimingIntentDisposition {
    pub fn is_canonical(&self) -> bool {
        matches!(self, Self::Canonical)
    }
}

/// One scalar constraint or physical limit captured from a timing/limits table.
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
    /// Direct structured-table authority for this constraint, distinct from prose statement
    /// support. Additive/defaulted so retained artifacts written before the carrier still load.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_table_ids: Vec<String>,
    /// Explicit applicability to executable digital intent. Canonical is omitted for stable
    /// backward-compatible JSON; a non-applicable disposition keeps the physical fact, reason,
    /// boundary, and replay path visible without promoting it as executable timing.
    #[serde(default, skip_serializing_if = "TimingIntentDisposition::is_canonical")]
    pub intent_disposition: TimingIntentDisposition,
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
    /// Verifier-owned exact unclassified/captured premises. This is metadata, not a semantic
    /// claim family and therefore is intentionally private.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    proof_context: Option<SourceProofContext>,
    /// Current ruleset-bound proof chain. Deserialization never grants authority without
    /// executable rule verification against `proof_context` and the exact public fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    proof_ledger: Option<ProofLedger>,
}

/// Conformance-only SourceIR overlay. It deliberately has no canonical SourceIR writer and cannot
/// be consumed by [`SourceIr::load_from_path`] or any production downstream builder.
#[cfg(any(test, feature = "test-support", feature = "conformance-support"))]
#[derive(Debug, Clone)]
pub struct NonCanonicalSourceOverlay {
    source_ir: SourceIr,
}

#[cfg(any(test, feature = "test-support", feature = "conformance-support"))]
impl NonCanonicalSourceOverlay {
    #[doc(hidden)]
    pub fn from_fixture(mut source_ir: SourceIr) -> Result<Self> {
        // The fixture may have patched any public field after a valid build. Strip canonical
        // authority instead of manufacturing a privileged proof for those edits.
        source_ir.proof_context = None;
        source_ir.proof_ledger = None;
        Ok(Self { source_ir })
    }

    #[doc(hidden)]
    pub(crate) fn source_ir(&self) -> &SourceIr {
        &self.source_ir
    }

    #[doc(hidden)]
    pub fn validation_report(&self) -> Result<ValidationReportRecord> {
        let persisted = self.source_ir.persisted_clone()?;
        source_validation_report_from_fields(
            &persisted
                .public_field_values()
                .map_err(source_derivation_error)?,
        )
        .map_err(source_derivation_error)
    }
}

const SOURCE_RULE_FIELDS: &[(&str, &str)] = &[
    ("schema_version", "source.envelope"),
    ("stage", "source.envelope"),
    ("source", "source.envelope"),
    ("artifact_layout", "source.envelope"),
    ("document_identity", "source.envelope"),
    ("normalization_plan", "source.envelope"),
    ("downstream_stages", "source.envelope"),
    ("adapter_targets", "source.envelope"),
    ("planned_actions", "source.envelope"),
    ("automation_confidence", "source.envelope"),
    ("page_artifacts", "source.capture"),
    ("content_elements", "source.capture"),
    ("document_profile", "source.capture"),
    ("placeholder_bindings", "source.capture"),
    ("visual_assets", "source.classification"),
    ("structured_tables", "source.classification"),
    ("document_sections", "source.classification"),
    ("residual_decisions", "source.residual"),
    ("validation_reports", "source.validation"),
];

fn source_derivation_error(error: impl std::fmt::Display) -> AppError {
    AppError::InvalidStageArtifact(format!("SourceIR proof verification failed: {error}"))
}

fn source_rule_registry() -> DerivationResult<RuleRegistry> {
    let implementation_sha256 = production_semantic_implementation_digest(IrStage::SourceIr)?;
    let registrations = SOURCE_RULE_FIELDS
        .iter()
        .map(|(field, family)| {
            let (capability, alpha, premises) = match *family {
                "source.envelope" => (
                    SymbolCapabilityClass::ExactIdentityOnly,
                    AlphaObligation::IdentityGraphInvariant,
                    vec![PremiseKind::SourceSpan, PremiseKind::UniversalAxiom],
                ),
                "source.validation" => (
                    SymbolCapabilityClass::ExactIdentityOnly,
                    AlphaObligation::IdentityGraphInvariant,
                    vec![PremiseKind::UpstreamClaim],
                ),
                "source.capture" | "source.classification" => (
                    SymbolCapabilityClass::GrammarIntroduces,
                    AlphaObligation::IntroducedSymbolsPreserveOrigins,
                    [
                        PremiseKind::SourceSpan,
                        PremiseKind::TableCell,
                        PremiseKind::VisualRegion,
                    ]
                    .into_iter()
                    .chain(
                        (*family == "source.classification")
                            .then_some(PremiseKind::GroundedModelProposal),
                    )
                    .collect(),
                ),
                "source.residual" => (
                    SymbolCapabilityClass::Residual,
                    AlphaObligation::ResidualTopologyInvariant,
                    vec![PremiseKind::SourceSpan, PremiseKind::UniversalAxiom],
                ),
                _ => unreachable!("closed SourceIR rule family"),
            };
            let rule_id =
                RuleId::try_from(format!("{family}.{field}.v1")).map_err(DerivationError::new)?;
            let descriptor = RuleDescriptor::new(
                rule_id,
                1,
                "crate::ir::source",
                implementation_sha256.clone(),
                premises,
                IrStage::SourceIr,
                *field,
                capability,
                alpha,
                RuleCompatibility::CurrentOnly,
            )?;
            Ok(RuleRegistration::new(descriptor, {
                #[cfg(any(test, feature = "test-support"))]
                {
                    verify_source_test_rule_relation
                }
                #[cfg(not(any(test, feature = "test-support")))]
                {
                    verify_source_rule_relation
                }
            }))
        })
        .collect::<DerivationResult<Vec<_>>>()?;
    RuleRegistry::new(registrations, [])
}

#[cfg(test)]
pub(super) fn rule_registry_for_qualification() -> DerivationResult<RuleRegistry> {
    source_rule_registry()
}

fn classifier_label(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .split(|character: char| !character.is_ascii_alphanumeric() && character != '/')
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn classifier_has_phrase(value: &str, phrases: &[&str]) -> bool {
    let normalized = format!(" {} ", classifier_label(value));
    phrases
        .iter()
        .any(|phrase| normalized.contains(&format!(" {} ", classifier_label(phrase))))
}

fn classified_diagram_kind(asset: &VisualAsset) -> DiagramKind {
    if asset.asset_kind == VisualAssetKind::TableRegion {
        return DiagramKind::Unknown;
    }
    let caption = asset.caption_text.as_deref().unwrap_or_default();
    if classifier_has_phrase(
        caption,
        &[
            "timing diagram",
            "timing waveform",
            "waveform diagram",
            "waveform plot",
        ],
    ) {
        DiagramKind::TimingDiagram
    } else if classifier_has_phrase(
        caption,
        &[
            "state machine",
            "state diagram",
            "state-transition diagram",
            "state transition diagram",
            "finite-state machine",
            "finite state machine",
        ],
    ) {
        DiagramKind::StateMachineDiagram
    } else if classifier_has_phrase(
        caption,
        &[
            "block diagram",
            "architecture diagram",
            "system diagram",
            "component diagram",
            "topology diagram",
            "interconnection diagram",
        ],
    ) {
        DiagramKind::BlockDiagram
    } else if classifier_has_phrase(
        caption,
        &[
            "register bit-field diagram",
            "register bit field diagram",
            "register bit-field layout",
            "register bit field layout",
        ],
    ) {
        DiagramKind::RegisterBitfield
    } else if classifier_has_phrase(caption, &["truth table"]) {
        DiagramKind::TruthTable
    } else if classifier_has_phrase(caption, &["flow chart", "flowchart", "flow diagram"]) {
        DiagramKind::FlowChart
    } else {
        DiagramKind::Unknown
    }
}

fn classified_section_kind(title: &str) -> SectionKind {
    let normalized = classifier_label(title);
    if classifier_has_phrase(
        title,
        &[
            "licence",
            "license",
            "copyright",
            "proprietary",
            "trademark",
            "disclaimer",
            "change history",
            "revision history",
            "release note",
            "release notes",
            "release information",
            "acknowledgement",
            "acknowledgements",
            "acknowledgment",
            "acknowledgments",
            "preface",
            "foreword",
            "feedback",
            "about this",
        ],
    ) {
        SectionKind::Boilerplate
    } else if classifier_has_phrase(title, &["table of contents"]) || normalized == "contents" {
        SectionKind::TableOfContents
    } else if classifier_has_phrase(
        title,
        &[
            "glossary",
            "abbreviation",
            "abbreviations",
            "acronym",
            "acronyms",
            "definition",
            "definitions",
        ],
    ) {
        SectionKind::Glossary
    } else if normalized == "appendix"
        || normalized.starts_with("appendix ")
        || normalized == "annex"
        || normalized.starts_with("annex ")
    {
        SectionKind::Appendix
    } else if classifier_has_phrase(
        title,
        &[
            "signal", "signals", "port", "ports", "pin", "pins", "pinout", "i/o",
        ],
    ) {
        SectionKind::SignalDescription
    } else if classifier_has_phrase(
        title,
        &["register", "registers", "memory map", "address map"],
    ) {
        SectionKind::RegisterDescription
    } else if classifier_has_phrase(
        title,
        &[
            "timing",
            "waveform",
            "waveforms",
            "clock",
            "clocks",
            "latency",
            "throughput",
        ],
    ) {
        SectionKind::Timing
    } else {
        SectionKind::Normative
    }
}

fn header_has_role(headers: &[&str], roles: &[&str]) -> bool {
    let roles = roles
        .iter()
        .map(|role| classifier_label(role))
        .collect::<BTreeSet<_>>();
    headers
        .iter()
        .any(|header| roles.contains(&classifier_label(header)))
}

fn table_cell_is_bit_range(value: &str) -> bool {
    let trimmed = value.trim().trim_start_matches('[').trim_end_matches(']');
    trimmed.split_once(':').is_some_and(|(high, low)| {
        !high.trim().is_empty()
            && !low.trim().is_empty()
            && high.trim().bytes().all(|byte| byte.is_ascii_digit())
            && low.trim().bytes().all(|byte| byte.is_ascii_digit())
    })
}

fn classified_table_kind(table: &StructuredTableRecord) -> TableKind {
    if table.header_rows.is_empty() && table.body_rows.is_empty() {
        return TableKind::Unknown;
    }
    let caption = table.caption_text.as_deref().unwrap_or_default();
    let headers = table
        .header_rows
        .iter()
        .flatten()
        .map(|cell| cell.text.as_str())
        .collect::<Vec<_>>();
    let caption_words = classifier_label(caption)
        .split_whitespace()
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    if classifier_has_phrase(caption, &["table"])
        && ["signal", "signals", "port", "ports", "pin", "pins"]
            .iter()
            .any(|word| caption_words.contains(*word))
    {
        return TableKind::SignalDescription;
    }
    let has_signal_name = header_has_role(
        &headers,
        &[
            "name",
            "signal",
            "signal name",
            "port",
            "port name",
            "pin",
            "pin name",
        ],
    );
    let has_width = header_has_role(&headers, &["width", "bit width", "bits", "size"]);
    let has_direction = header_has_role(&headers, &["direction", "dir"]);
    let has_explicit_signal = header_has_role(
        &headers,
        &[
            "signal",
            "signal name",
            "port",
            "port name",
            "pin",
            "pin name",
        ],
    );
    if has_signal_name && (has_direction || (has_explicit_signal && has_width)) {
        return TableKind::SignalDescription;
    }
    let has_value = header_has_role(
        &headers,
        &[
            "value",
            "encoded value",
            "encoding",
            "code",
            "binary",
            "hex",
            "bit pattern",
        ],
    );
    let has_meaning = header_has_role(
        &headers,
        &["name", "meaning", "description", "definition", "semantics"],
    );
    if (has_value && has_meaning) || classifier_has_phrase(caption, &["encoding", "encodings"]) {
        return TableKind::Encoding;
    }
    let has_register_name = header_has_role(
        &headers,
        &[
            "name",
            "register",
            "register name",
            "field",
            "field name",
            "symbol",
        ],
    );
    let has_address = header_has_role(
        &headers,
        &[
            "offset",
            "address",
            "addr",
            "base",
            "base address",
            "register offset",
            "byte offset",
        ],
    );
    let has_access_column = header_has_role(
        &headers,
        &[
            "access",
            "access type",
            "r/w",
            "read/write",
            "read write",
            "read",
            "read access",
            "write",
            "write access",
            "permission",
            "permissions",
        ],
    );
    let structural_cells = headers
        .iter()
        .copied()
        .chain(
            table
                .body_rows
                .iter()
                .take(16)
                .flatten()
                .map(|cell| cell.text.as_str()),
        )
        .collect::<Vec<_>>();
    let access_tokens = [
        "ro", "rw", "wo", "rc", "rs", "w1c", "w1s", "w0c", "rw1c", "r/w",
    ];
    let has_access_value = structural_cells
        .iter()
        .any(|cell| access_tokens.contains(&cell.trim().to_ascii_lowercase().as_str()));
    let is_toc = structural_cells.iter().any(|cell| cell.contains("...."));
    if !is_toc
        && has_register_name
        && (has_access_column || has_access_value)
        && (has_address
            || structural_cells
                .iter()
                .any(|cell| table_cell_is_bit_range(cell)))
    {
        return TableKind::RegisterMap;
    }
    if timing_table_has_structural_authority(table) {
        return TableKind::TimingParameter;
    }
    let has_feature = header_has_role(&headers, &["feature", "property", "capability", "option"]);
    let has_support = header_has_role(
        &headers,
        &["support", "requirement", "status", "mandatory optional"],
    );
    let support_tokens = [
        "mandatory",
        "optional",
        "prohibited",
        "required",
        "supported",
    ];
    let has_support_value = table
        .body_rows
        .iter()
        .take(16)
        .flatten()
        .any(|cell| support_tokens.contains(&classifier_label(&cell.text).as_str()));
    if has_feature && (has_support || has_support_value) {
        TableKind::FeatureMatrix
    } else {
        TableKind::Unknown
    }
}

fn parse_vlm_kind(value: &str) -> Option<TableKind> {
    match value.trim() {
        "signal_description" => Some(TableKind::SignalDescription),
        "encoding" => Some(TableKind::Encoding),
        "timing_parameter" => Some(TableKind::TimingParameter),
        "feature_matrix" => Some(TableKind::FeatureMatrix),
        "register_map" => Some(TableKind::RegisterMap),
        _ => None,
    }
}

fn parse_vlm_table_kind(response: &str) -> Option<TableKind> {
    let start = response.find('{')?;
    let end = response.rfind('}')?;
    let value: serde_json::Value = serde_json::from_str(&response[start..=end]).ok()?;
    parse_vlm_kind(value.get("kind")?.as_str()?)
}

fn parse_vlm_grid(response: &str) -> Option<(String, Vec<String>, Vec<Vec<String>>)> {
    let start = response.find('{')?;
    let end = response.rfind('}')?;
    let value: serde_json::Value = serde_json::from_str(&response[start..=end]).ok()?;
    let kind = value
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("other")
        .to_string();
    let cell_text = |cell: &serde_json::Value| match cell {
        serde_json::Value::String(text) => text.trim().to_string(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    };
    let columns = value
        .get("columns")?
        .as_array()?
        .iter()
        .map(cell_text)
        .collect::<Vec<_>>();
    if columns.len() < 2 {
        return None;
    }
    let rows = value
        .get("rows")
        .and_then(serde_json::Value::as_array)
        .map(|rows| {
            rows.iter()
                .filter_map(serde_json::Value::as_array)
                .map(|row| row.iter().map(cell_text).collect())
                .collect()
        })
        .unwrap_or_default();
    Some((kind, columns, rows))
}

fn vlm_kind_structurally_consistent(table: &StructuredTableRecord, kind: TableKind) -> bool {
    let headers = table
        .header_rows
        .first()
        .or_else(|| table.body_rows.first())
        .map(|row| {
            row.iter()
                .map(|cell| classifier_label(&cell.text))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let has_role = |roles: &[&str]| {
        headers
            .iter()
            .any(|header| roles.iter().any(|role| header == &classifier_label(role)))
    };
    match kind {
        TableKind::SignalDescription => {
            has_role(&["signal", "name", "pin", "port"])
                && has_role(&["width", "bits", "direction", "source", "destination"])
                && !(has_role(&["field"]) && has_role(&["access", "reset"]))
        }
        TableKind::RegisterMap => {
            has_role(&["offset", "address", "bits", "field"])
                && has_role(&["access", "reset", "type", "attribute", "bits"])
        }
        TableKind::Encoding => has_role(&["value", "encoding", "code", "binary", "hex"]),
        TableKind::TimingParameter => timing_table_has_structural_authority(table),
        TableKind::FeatureMatrix => has_role(&[
            "feature",
            "property",
            "capability",
            "mandatory",
            "optional",
            "support",
        ]),
        _ => false,
    }
}

fn table_is_degenerate(table: &StructuredTableRecord) -> bool {
    let maximum_body_cells = table.body_rows.iter().map(Vec::len).max().unwrap_or(0);
    table.col_count <= 1 || maximum_body_cells <= 1
}

fn validate_visual_response(kind: DiagramKind, response: &str) -> DerivationResult<()> {
    let start = response.find('{').ok_or_else(|| {
        DerivationError::new("visual model response does not contain a JSON object")
    })?;
    let end = response.rfind('}').ok_or_else(|| {
        DerivationError::new("visual model response does not contain a complete JSON object")
    })?;
    let value: serde_json::Value = serde_json::from_str(&response[start..=end])
        .map_err(|error| DerivationError::new(format!("invalid visual model JSON: {error}")))?;
    let required_arrays: &[&str] = match kind {
        DiagramKind::TimingDiagram => &["signals", "annotations"],
        DiagramKind::StateMachineDiagram => &["states", "transitions"],
        _ => {
            return Err(DerivationError::new(
                "only timing/state-machine observations have a registered SourceIR model grammar",
            ));
        }
    };
    if required_arrays
        .iter()
        .all(|field| value.get(*field).is_some_and(serde_json::Value::is_array))
    {
        Ok(())
    } else {
        Err(DerivationError::new(
            "visual model response lacks the registered array schema",
        ))
    }
}

fn apply_source_grounded_proposal(
    proposal: &SourceGroundedProposal,
    visual_assets: &mut [VisualAsset],
    tables: &mut [StructuredTableRecord],
) -> DerivationResult<bool> {
    match proposal {
        SourceGroundedProposal::VisualObservation {
            asset_id,
            diagram_kind,
            exact_response,
            ..
        } => {
            let asset = visual_assets
                .iter_mut()
                .find(|asset| asset.asset_id == *asset_id)
                .ok_or_else(|| {
                    DerivationError::new(format!("visual asset '{asset_id}' is absent"))
                })?;
            if asset.diagram_kind != *diagram_kind {
                return Err(DerivationError::new(
                    "visual proposal kind does not match the registered caption classification",
                ));
            }
            validate_visual_response(*diagram_kind, exact_response)?;
            let prefix = match diagram_kind {
                DiagramKind::TimingDiagram => "vlm_timing_diagram_extraction: ",
                DiagramKind::StateMachineDiagram => "vlm_state_machine_extraction: ",
                _ => unreachable!("validated visual proposal kind"),
            };
            asset.note = Some(format!("{prefix}{exact_response}"));
            Ok(true)
        }
        SourceGroundedProposal::TableGridRepair {
            table_id,
            exact_response,
            ..
        } => {
            let table = tables
                .iter_mut()
                .find(|table| table.table_id == *table_id)
                .ok_or_else(|| {
                    DerivationError::new(format!("structured table '{table_id}' is absent"))
                })?;
            if table.table_kind != TableKind::Unknown || !table_is_degenerate(table) {
                return Err(DerivationError::new(
                    "table-grid repair requires an unknown degenerate captured table",
                ));
            }
            let (kind_label, columns, rows) = parse_vlm_grid(exact_response).ok_or_else(|| {
                DerivationError::new("table-grid response does not satisfy the registered schema")
            })?;
            table.header_rows = vec![
                columns
                    .iter()
                    .map(|text| StructuredTableCellRecord {
                        text: text.clone(),
                        row_span: 1,
                        col_span: 1,
                        is_header: true,
                    })
                    .collect(),
            ];
            table.body_rows = rows
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|text| StructuredTableCellRecord {
                            text: text.clone(),
                            row_span: 1,
                            col_span: 1,
                            is_header: false,
                        })
                        .collect()
                })
                .collect();
            table.col_count = columns.len() as u32;
            table.row_count = (rows.len() + 1) as u32;
            if let Some(kind) = parse_vlm_kind(&kind_label)
                .filter(|kind| vlm_kind_structurally_consistent(table, *kind))
            {
                table.table_kind = kind;
            }
            Ok(true)
        }
        SourceGroundedProposal::TableClassification {
            table_id,
            exact_response,
            ..
        } => {
            let table = tables
                .iter_mut()
                .find(|table| table.table_id == *table_id)
                .ok_or_else(|| {
                    DerivationError::new(format!("structured table '{table_id}' is absent"))
                })?;
            if table.table_kind != TableKind::Unknown {
                return Err(DerivationError::new(
                    "table classification may only refine an unknown captured table",
                ));
            }
            let Some(kind) = parse_vlm_table_kind(exact_response) else {
                return Ok(false);
            };
            if !vlm_kind_structurally_consistent(table, kind) {
                return Ok(false);
            }
            table.table_kind = kind;
            Ok(true)
        }
    }
}

fn classify_source_captures(
    visual_assets: &mut [VisualAsset],
    tables: &mut [StructuredTableRecord],
    sections: &mut [ContentSectionRecord],
) {
    for asset in visual_assets {
        asset.diagram_kind = classified_diagram_kind(asset);
    }
    for table in tables {
        table.table_kind = classified_table_kind(table);
    }
    for section in sections {
        section.section_kind = classified_section_kind(&section.title);
    }
}

fn source_validation_fingerprint(
    fields: &BTreeMap<String, serde_json::Value>,
) -> DerivationResult<String> {
    let mut semantic_fields = fields.clone();
    semantic_fields.remove("validation_reports");
    Ok(Sha256Digest::of_serializable(&semantic_fields)?
        .as_str()
        .to_string())
}

fn source_validation_report_from_fields(
    fields: &BTreeMap<String, serde_json::Value>,
) -> DerivationResult<ValidationReportRecord> {
    let page_artifacts: Vec<PageArtifact> =
        serde_json::from_value(fields["page_artifacts"].clone())
            .map_err(|error| DerivationError::new(format!("invalid validation pages: {error}")))?;
    let visual_assets: Vec<VisualAsset> = serde_json::from_value(fields["visual_assets"].clone())
        .map_err(|error| {
        DerivationError::new(format!("invalid validation visuals: {error}"))
    })?;
    let structured_tables: Vec<StructuredTableRecord> =
        serde_json::from_value(fields["structured_tables"].clone())
            .map_err(|error| DerivationError::new(format!("invalid validation tables: {error}")))?;
    let content_elements: Vec<ContentElementRecord> =
        serde_json::from_value(fields["content_elements"].clone()).map_err(|error| {
            DerivationError::new(format!("invalid validation content: {error}"))
        })?;
    let document_sections: Vec<ContentSectionRecord> =
        serde_json::from_value(fields["document_sections"].clone()).map_err(|error| {
            DerivationError::new(format!("invalid validation sections: {error}"))
        })?;
    let residual_decisions: Vec<ResidualDecisionPacket> =
        serde_json::from_value(fields["residual_decisions"].clone()).map_err(|error| {
            DerivationError::new(format!("invalid validation residuals: {error}"))
        })?;
    let document_identity: DocumentIdentity =
        serde_json::from_value(fields["document_identity"].clone()).map_err(|error| {
            DerivationError::new(format!("invalid validation identity: {error}"))
        })?;

    let timing_count = visual_assets
        .iter()
        .filter(|asset| asset.diagram_kind == DiagramKind::TimingDiagram)
        .count();
    let state_count = visual_assets
        .iter()
        .filter(|asset| asset.diagram_kind == DiagramKind::StateMachineDiagram)
        .count();
    let block_count = visual_assets
        .iter()
        .filter(|asset| asset.diagram_kind == DiagramKind::BlockDiagram)
        .count();
    let unknown_count = visual_assets
        .iter()
        .filter(|asset| asset.diagram_kind == DiagramKind::Unknown)
        .count();
    let classified = timing_count + state_count + block_count;
    let diagram_coverage = if visual_assets.is_empty() {
        0.0
    } else {
        classified as f64 / visual_assets.len() as f64 * 100.0
    };
    let vlm_enriched = visual_assets
        .iter()
        .filter(|asset| {
            asset.note.as_deref().is_some_and(|note| {
                note.starts_with("vlm_timing_diagram_extraction:")
                    || note.starts_with("vlm_state_machine_extraction:")
            })
        })
        .count();
    let figures_ready_for_vlm = timing_count + state_count;
    let missing_vlm_ids = visual_assets
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
        .collect::<Vec<_>>();
    let mut findings = Vec::new();
    if figures_ready_for_vlm > 0 && vlm_enriched == 0 {
        findings.push(ValidationFindingRecord {
            finding_id: "source_vlm_enrichment_missing".to_string(),
            severity: ValidationFindingSeverity::Warning,
            category: "visual_enrichment".to_string(),
            summary: format!(
                "{figures_ready_for_vlm} classified timing/state diagrams are still missing VLM enrichment"
            ),
            related_ids: missing_vlm_ids.clone(),
        });
        if !missing_vlm_ids.is_empty() {
            findings.push(ValidationFindingRecord {
                finding_id: "source_vlm_enrichment_missing_surface_rescan_guidance".to_string(),
                severity: ValidationFindingSeverity::Info,
                category: "rescan_guidance".to_string(),
                summary: "SourceIR still carries timing/state diagram asset ids without VLM enrichment; this should trigger targeted local visual enrichment plus bounded replay to see whether those same asset ids gain extracted observations".to_string(),
                related_ids: missing_vlm_ids,
            });
        }
    }
    if unknown_count > 0 {
        findings.push(ValidationFindingRecord {
            finding_id: "source_unknown_diagrams_remaining".to_string(),
            severity: ValidationFindingSeverity::Info,
            category: "diagram_classification".to_string(),
            summary: format!("{unknown_count} visual assets remain unclassified"),
            related_ids: visual_assets
                .iter()
                .filter(|asset| asset.diagram_kind == DiagramKind::Unknown)
                .map(|asset| asset.asset_id.clone())
                .take(6)
                .collect(),
        });
    }
    if !residual_decisions.is_empty() {
        findings.push(ValidationFindingRecord {
            finding_id: "source_residual_decisions_present".to_string(),
            severity: ValidationFindingSeverity::Warning,
            category: "residual_decisions".to_string(),
            summary: format!(
                "SourceIR still carries {} residual decision packet(s)",
                residual_decisions.len()
            ),
            related_ids: residual_decisions
                .iter()
                .map(|packet| packet.packet_id.clone())
                .collect(),
        });
    }
    let artifact_fingerprint = source_validation_fingerprint(fields)?;
    let metric = |name: &str, value: String| ValidationMetricRecord {
        name: name.to_string(),
        value,
    };
    Ok(ValidationReportRecord {
        report_id: format!("validation_source_ir_{artifact_fingerprint}"),
        validated_stage: IrStage::SourceIr,
        artifact_fingerprint,
        summary: format!(
            "SourceIR validation for {} with {} finding(s)",
            document_identity.display_name,
            findings.len()
        ),
        overall_score: None,
        grade: None,
        metrics: vec![
            metric("pages", page_artifacts.len().to_string()),
            metric("visual_assets", visual_assets.len().to_string()),
            metric("structured_tables", structured_tables.len().to_string()),
            metric("content_elements", content_elements.len().to_string()),
            metric("document_sections", document_sections.len().to_string()),
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
            metric("residual_decisions", residual_decisions.len().to_string()),
        ],
        findings,
    })
}

#[cfg(any(test, feature = "test-support"))]
fn verify_source_test_rule_relation(context: RuleVerificationContext<'_>) -> DerivationResult<()> {
    let premise = context
        .premise_bytes(0)?
        .ok_or_else(|| DerivationError::new("SourceIR rule requires exact captured bytes"))?;
    if context.proof().premises().iter().any(|premise| {
        matches!(
            premise,
            PremiseRef::SourceSpan { span_id, .. }
                if span_id.starts_with("test-fixture-source-field:")
        )
    }) && premise == context.conclusion_json()
    {
        return Ok(());
    }
    verify_source_rule_relation(context)
}

fn verify_source_rule_relation(context: RuleVerificationContext<'_>) -> DerivationResult<()> {
    if context.proof().address().surface() == "validation_reports" {
        if context.proof().address().field_path().is_some() {
            let root = context
                .premise_bytes(0)?
                .ok_or_else(|| DerivationError::new("validation record lacks its root proof"))?;
            let reports: Vec<ValidationReportRecord> =
                serde_json::from_slice(root).map_err(|error| {
                    DerivationError::new(format!("invalid validation root: {error}"))
                })?;
            let index = context
                .proof()
                .address()
                .field_path()
                .and_then(|path| path.strip_prefix('['))
                .and_then(|path| path.strip_suffix(']'))
                .and_then(|path| path.parse::<usize>().ok())
                .ok_or_else(|| DerivationError::new("invalid validation record field path"))?;
            let expected = reports.get(index).ok_or_else(|| {
                DerivationError::new("validation record index exceeds its proved root")
            })?;
            let expected =
                serde_json::to_vec(&serde_json::to_value(expected).map_err(|error| {
                    DerivationError::new(format!("cannot serialize validation record: {error}"))
                })?)
                .map_err(|error| {
                    DerivationError::new(format!("cannot encode validation record: {error}"))
                })?;
            return if expected == context.conclusion_json() {
                Ok(())
            } else {
                Err(DerivationError::new(
                    "validation record is not the selected member of its proved root",
                ))
            };
        }
        if context.conclusion_json() == b"[]" {
            return Ok(());
        }
        let mut fields = BTreeMap::new();
        for (index, premise) in context.proof().premises().iter().enumerate() {
            let PremiseRef::UpstreamClaim { address, .. } = premise else {
                return Err(DerivationError::new(
                    "validation root accepts only verified SourceIR field roots",
                ));
            };
            if address.stage() != IrStage::SourceIr
                || address.stable_record_key() != "root"
                || address.surface() == "validation_reports"
            {
                return Err(DerivationError::new(
                    "validation root premise is not a non-validation SourceIR field root",
                ));
            }
            let bytes = context.premise_bytes(index)?.ok_or_else(|| {
                DerivationError::new("validation upstream premise bytes are absent")
            })?;
            let value = serde_json::from_slice(bytes).map_err(|error| {
                DerivationError::new(format!("invalid validation upstream value: {error}"))
            })?;
            fields.insert(address.surface().to_string(), value);
        }
        if fields.len() + 1 != SOURCE_RULE_FIELDS.len() {
            return Err(DerivationError::new(
                "validation root does not cover every non-validation SourceIR field root",
            ));
        }
        fields.insert("validation_reports".to_string(), serde_json::json!([]));
        let expected = vec![source_validation_report_from_fields(&fields)?];
        let expected = serde_json::to_vec(&serde_json::to_value(expected).map_err(|error| {
            DerivationError::new(format!(
                "cannot serialize expected validation report: {error}"
            ))
        })?)
        .map_err(|error| {
            DerivationError::new(format!("cannot encode expected validation report: {error}"))
        })?;
        return if expected == context.conclusion_json() {
            Ok(())
        } else {
            Err(DerivationError::new(
                "validation report is not the registered deterministic SourceIR evaluation",
            ))
        };
    }

    let premise = context
        .premise_bytes(0)?
        .ok_or_else(|| DerivationError::new("SourceIR rule requires exact captured bytes"))?;
    let per_record = context.proof().address().field_path().is_some();
    match context.proof().address().surface() {
        "visual_assets" | "structured_tables" | "document_sections" => {
            let mut visuals = Vec::new();
            let mut tables = Vec::new();
            let mut sections = Vec::new();
            match context.proof().address().surface() {
                "visual_assets" => {
                    visuals = if per_record {
                        vec![serde_json::from_slice(premise).map_err(|error| {
                            DerivationError::new(format!("invalid visual premise: {error}"))
                        })?]
                    } else {
                        serde_json::from_slice(premise).map_err(|error| {
                            DerivationError::new(format!("invalid visual premise: {error}"))
                        })?
                    };
                }
                "structured_tables" => {
                    tables = if per_record {
                        vec![serde_json::from_slice(premise).map_err(|error| {
                            DerivationError::new(format!("invalid table premise: {error}"))
                        })?]
                    } else {
                        serde_json::from_slice(premise).map_err(|error| {
                            DerivationError::new(format!("invalid table premise: {error}"))
                        })?
                    };
                }
                "document_sections" => {
                    sections = if per_record {
                        vec![serde_json::from_slice(premise).map_err(|error| {
                            DerivationError::new(format!("invalid section premise: {error}"))
                        })?]
                    } else {
                        serde_json::from_slice(premise).map_err(|error| {
                            DerivationError::new(format!("invalid section premise: {error}"))
                        })?
                    };
                }
                _ => unreachable!("closed classification surface"),
            }
            classify_source_captures(&mut visuals, &mut tables, &mut sections);
            for index in 1..context.proof().premises().len() {
                if context.proof().premises()[index].kind() != PremiseKind::GroundedModelProposal {
                    continue;
                }
                let proposal_bytes = context.premise_bytes(index)?.ok_or_else(|| {
                    DerivationError::new("grounded SourceIR proposal bytes are absent")
                })?;
                let proposal: SourceGroundedProposal = serde_json::from_slice(proposal_bytes)
                    .map_err(|error| {
                        DerivationError::new(format!("invalid grounded SourceIR proposal: {error}"))
                    })?;
                if source_grounded_proposal_surface(&proposal)
                    != context.proof().address().surface()
                {
                    return Err(DerivationError::new(
                        "grounded SourceIR proposal targets the wrong classification surface",
                    ));
                }
                if !apply_source_grounded_proposal(&proposal, &mut visuals, &mut tables)? {
                    return Err(DerivationError::new(
                        "grounded SourceIR proposal did not produce a canonical refinement",
                    ));
                }
            }
            let replayed_value = match context.proof().address().surface() {
                "visual_assets" if per_record => serde_json::to_value(&visuals[0]),
                "visual_assets" => serde_json::to_value(&visuals),
                "structured_tables" if per_record => serde_json::to_value(&tables[0]),
                "structured_tables" => serde_json::to_value(&tables),
                "document_sections" if per_record => serde_json::to_value(&sections[0]),
                "document_sections" => serde_json::to_value(&sections),
                _ => unreachable!("closed classification surface"),
            }
            .map_err(|error| {
                DerivationError::new(format!("cannot serialize replayed SourceIR claim: {error}"))
            })?;
            let replayed = serde_json::to_vec(&replayed_value).map_err(|error| {
                DerivationError::new(format!("cannot encode replayed SourceIR claim: {error}"))
            })?;
            if replayed == context.conclusion_json() {
                Ok(())
            } else {
                Err(DerivationError::new(format!(
                    "SourceIR {}:{} classification is not the registered capture/proposal replay",
                    context.proof().address().surface(),
                    context.proof().address().stable_record_key()
                )))
            }
        }
        _ if premise == context.conclusion_json() => Ok(()),
        other => Err(DerivationError::new(format!(
            "SourceIR field '{other}' is not an exact capture copy",
        ))),
    }
}

impl SourceIr {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        Self::load_with_verified_proof(path).map(|(source_ir, _)| source_ir)
    }

    /// Load canonical SourceIR and retain the non-deserializable verification witness required by
    /// the next IR stage. The witness describes persisted conclusions; runtime-only absolute path
    /// expansion never enters the proof chain.
    pub(crate) fn load_with_verified_proof(path: &Path) -> Result<(Self, VerifiedProofLedger)> {
        let path = resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
        let source_ir = serde_json::from_str::<Self>(&fs::read_to_string(&path)?)?;
        if source_ir.schema_version > SOURCE_IR_SCHEMA_VERSION {
            return Err(AppError::InvalidStageArtifact(format!(
                "SourceIR schema {} at {} is newer than supported schema {SOURCE_IR_SCHEMA_VERSION}",
                source_ir.schema_version,
                path.display()
            )));
        }
        if source_ir.schema_version < SOURCE_IR_SCHEMA_VERSION {
            return Err(AppError::InvalidStageArtifact(format!(
                "legacy proofless SourceIR schema {} at {} is inspection-only and must be rebuilt before canonical use",
                source_ir.schema_version,
                path.display()
            )));
        }
        let verified = source_ir.verified_canonical_proof()?;
        Ok((source_ir.runtime_clone()?, verified))
    }

    /// Parse an old SourceIR for diagnostics or an explicit rebuild command without granting it
    /// canonical authority. Downstream builders must use [`Self::load_from_path`] instead.
    pub fn load_for_inspection(path: &Path) -> Result<Self> {
        let path = resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
        let mut source_ir = serde_json::from_str::<Self>(&fs::read_to_string(&path)?)?;
        if source_ir.schema_version > SOURCE_IR_SCHEMA_VERSION {
            return Err(AppError::InvalidStageArtifact(format!(
                "SourceIR schema {} at {} is newer than supported schema {SOURCE_IR_SCHEMA_VERSION}",
                source_ir.schema_version,
                path.display()
            )));
        }
        if source_ir.schema_version < SOURCE_IR_SCHEMA_VERSION {
            neutralize_legacy_source_classifications(&mut source_ir);
        } else {
            source_ir.verify_canonical_proof()?;
        }
        source_ir.runtime_clone()
    }

    /// Rebuild one legacy SourceIR from its retained capture bundle without re-running a PDF
    /// model. This maintenance capability is absent from normal builds and must be explicitly
    /// enabled for an audited schema migration.
    #[cfg(feature = "source-proof-migration")]
    #[doc(hidden)]
    pub fn rebuild_legacy_from_retained_capture(path: &Path) -> Result<Self> {
        Self::rebuild_from_retained_capture(path)
    }

    /// Refresh either a legacy artifact or a current-schema artifact whose proof implementation
    /// digest is stale. Current artifacts retain their exact, already-captured proof context; this
    /// path re-executes that context with the current registry and never recaptures source or model
    /// output. The capability is feature-gated to audited corpus reconciliation builds.
    #[cfg(feature = "source-proof-migration")]
    #[doc(hidden)]
    pub fn rebuild_from_retained_capture(path: &Path) -> Result<Self> {
        let path = resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
        let raw = fs::read_to_string(&path)?;
        let retained = serde_json::from_str::<Self>(&raw)?;
        if retained.schema_version > SOURCE_IR_SCHEMA_VERSION {
            return Err(AppError::InvalidStageArtifact(format!(
                "SourceIR at {} is schema {}; retained-capture migration supports schemas through {SOURCE_IR_SCHEMA_VERSION}",
                path.display(),
                retained.schema_version
            )));
        }
        if !matches!(retained.stage, IrStage::SourceIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} is not SourceIR",
                path.display()
            )));
        }
        retained.verify_retained_capture_backing(&path)?;

        if retained.schema_version == SOURCE_IR_SCHEMA_VERSION {
            let context = retained.proof_context.clone().ok_or_else(|| {
                AppError::InvalidStageArtifact(format!(
                    "current SourceIR at {} has no retained proof context to refresh",
                    path.display()
                ))
            })?;
            let mut rebuilt = retained;
            rebuilt.proof_context = None;
            rebuilt.proof_ledger = None;
            let persisted = rebuilt.persisted_clone()?;
            rebuilt.refresh_proof_from_context(persisted, context)?;
            return Ok(rebuilt);
        }

        let mut rebuilt = retained;
        neutralize_legacy_source_classifications(&mut rebuilt);
        classify_source_captures(
            &mut rebuilt.visual_assets,
            &mut rebuilt.structured_tables,
            &mut rebuilt.document_sections,
        );
        rebuilt.schema_version = SOURCE_IR_SCHEMA_VERSION;
        // A legacy report used a legacy fingerprint/evaluator. Current validation is recomputed
        // only after the rebuilt artifact has passed canonical proof verification.
        rebuilt.validation_reports.clear();
        rebuilt.proof_context = None;
        rebuilt.proof_ledger = None;
        rebuilt = rebuilt.runtime_clone()?;
        rebuilt.refresh_canonical_proof()?;
        Ok(rebuilt)
    }

    #[cfg(feature = "source-proof-migration")]
    fn verify_retained_capture_backing(&self, artifact_path: &Path) -> Result<()> {
        let recorded_artifact = resolve_existing(
            &self.artifact_layout.source_ir_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        if recorded_artifact != artifact_path {
            return Err(AppError::InvalidStageArtifact(format!(
                "legacy SourceIR path mismatch: opened {} but artifact records {}",
                artifact_path.display(),
                recorded_artifact.display()
            )));
        }

        let source_path =
            resolve_existing(&self.source.canonical_path, self.source_path_origin()?)?;
        if let Some(recorded_size) = self.source.size_bytes {
            let actual_size = fs::metadata(&source_path)?.len();
            if actual_size != recorded_size {
                return Err(AppError::InvalidStageArtifact(format!(
                    "legacy SourceIR source-size mismatch at {}: recorded {recorded_size}, found {actual_size}",
                    source_path.display()
                )));
            }
        }

        if !matches!(self.source.source_kind, SourceKind::Pdf)
            || !matches!(self.normalization_plan.status, NormalizationStatus::Ready)
        {
            return Ok(());
        }

        let promoted_markdown = self
            .normalization_plan
            .promoted_markdown_path
            .as_deref()
            .ok_or_else(|| {
                AppError::InvalidStageArtifact(
                    "retained PDF capture lacks promoted markdown path".to_string(),
                )
            })?;
        resolve_existing(promoted_markdown, self.promoted_markdown_origin()?)?;
        let metadata_path = self
            .normalization_plan
            .metadata_output_path
            .as_deref()
            .ok_or_else(|| {
                AppError::InvalidStageArtifact(
                    "retained PDF capture lacks backend metadata path".to_string(),
                )
            })?;
        resolve_existing(metadata_path, PersistedPathOrigin::RepositoryOwned)?;

        let layout = self.artifact_layout.runtime_layout()?;
        resolve_existing(
            &layout.backend_raw_output_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        let page_manifest_path = resolve_existing(
            &layout.page_artifact_manifest_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        let visual_manifest_path = resolve_existing(
            &layout.visual_asset_manifest_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        let page_manifest: Vec<PageArtifact> =
            serde_json::from_str(&fs::read_to_string(page_manifest_path)?)?;
        let visual_manifest: Vec<VisualAsset> =
            serde_json::from_str(&fs::read_to_string(visual_manifest_path)?)?;
        if page_manifest != self.page_artifacts {
            return Err(AppError::InvalidStageArtifact(
                "legacy SourceIR page capture differs from its retained manifest".to_string(),
            ));
        }
        if visual_manifest != self.visual_assets {
            return Err(AppError::InvalidStageArtifact(
                "legacy SourceIR visual capture differs from its retained manifest".to_string(),
            ));
        }

        for page in &self.page_artifacts {
            if let Some(path) = page.page_image_path.as_deref() {
                resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
            }
            if let Some(path) = page.layout_metadata_path.as_deref() {
                resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
            }
        }
        for visual in &self.visual_assets {
            if let Some(path) = visual.image_path.as_deref() {
                resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
            }
            if let Some(path) = visual.caption_source_path.as_deref() {
                resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
            }
        }
        Ok(())
    }

    pub fn build(source: &Path, artifact_base_root: &Path) -> Result<Self> {
        let source_path_origin = if source.is_relative() {
            PersistedPathOrigin::RepositoryOwned
        } else {
            infer_existing_origin(source)?
        };
        let canonical = resolve_existing(source, source_path_origin)?;
        let metadata = fs::metadata(&canonical)?;
        let path_kind = SourcePathKind::detect(&metadata);
        let source_kind = SourceKind::detect(&canonical);
        let stable_artifact_stem = stable_stem(&canonical);
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
            path_origin: Some(source_path_origin),
            path_kind,
            source_kind,
            stable_artifact_stem,
            size_bytes: metadata.is_file().then_some(metadata.len()),
        };

        let document_identity = DocumentIdentity {
            document_key: document_key.clone(),
            display_name: display_name(&canonical),
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

        let mut source_ir = Self {
            schema_version: SOURCE_IR_SCHEMA_VERSION,
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
            proof_context: None,
            proof_ledger: None,
        }
        .runtime_clone()?;
        source_ir.refresh_canonical_proof()?;
        Ok(source_ir)
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        let persisted = self.persisted_clone()?;
        persisted.verify_canonical_proof()?;
        Ok(serde_json::to_string_pretty(&persisted)?)
    }

    pub(crate) fn source_path_origin(&self) -> Result<PersistedPathOrigin> {
        self.source.resolved_origin()
    }

    pub(crate) fn promoted_markdown_origin(&self) -> Result<PersistedPathOrigin> {
        if matches!(self.source.source_kind, SourceKind::Markdown) {
            self.source_path_origin()
        } else {
            Ok(PersistedPathOrigin::RepositoryOwned)
        }
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

        let source_path_origin = self.source_path_origin()?;
        let source_path = resolve_existing(&self.source.canonical_path, source_path_origin)?;
        let promoted_markdown_path = resolve_repository_output(&promoted_markdown_path)?;
        let metadata_output_path = resolve_repository_output(&metadata_output_path)?;
        let runtime_artifact_layout = self.artifact_layout.runtime_layout()?;

        let backend_summary = docling_backend::materialize_pdf(
            &source_path,
            &promoted_markdown_path,
            &metadata_output_path,
            &runtime_artifact_layout,
            &self.document_identity.document_key,
            source_path_origin,
        )?;

        self.page_artifacts = backend_summary.page_artifacts;
        self.visual_assets = backend_summary.visual_assets;
        self.structured_tables = backend_summary.structured_tables;
        normalize_timing_table_kinds(&mut self.structured_tables);
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

        *self = self.runtime_clone()?;
        self.refresh_canonical_proof()?;

        Ok(())
    }

    pub fn write_to_disk(&self) -> Result<()> {
        let persisted = self.persisted_clone()?;
        persisted.verify_canonical_proof()?;
        let runtime_layout = persisted.artifact_layout.runtime_layout()?;
        fs::create_dir_all(&runtime_layout.artifact_root)?;
        if matches!(persisted.source.source_kind, SourceKind::Pdf)
            && matches!(
                persisted.normalization_plan.status,
                NormalizationStatus::Ready
            )
        {
            fs::create_dir_all(&runtime_layout.normalized_root)?;
            fs::write(
                &runtime_layout.page_artifact_manifest_path,
                serde_json::to_string_pretty(&persisted.page_artifacts)?,
            )?;
            fs::write(
                &runtime_layout.visual_asset_manifest_path,
                serde_json::to_string_pretty(&persisted.visual_assets)?,
            )?;
        }
        fs::write(
            &runtime_layout.source_ir_path,
            serde_json::to_string_pretty(&persisted)?,
        )?;
        Ok(())
    }

    /// Apply one exact VLM response through the registered visual-observation grammar. The
    /// response is retained in the proof context and replayed on every canonical reload.
    pub fn apply_visual_observation(
        &mut self,
        asset_id: &str,
        diagram_kind: DiagramKind,
        exact_response: String,
    ) -> Result<()> {
        self.apply_grounded_source_proposal(SourceGroundedProposal::VisualObservation {
            proposal_id: self.next_grounded_proposal_id("visual", &exact_response),
            asset_id: asset_id.to_string(),
            diagram_kind,
            exact_response,
        })?
        .then_some(())
        .ok_or_else(|| {
            AppError::InvalidStageArtifact(
                "visual observation did not produce a registered refinement".to_string(),
            )
        })
    }

    /// Apply one exact VLM grid transcription through the registered repair grammar.
    pub fn apply_table_grid_repair(
        &mut self,
        table_id: &str,
        exact_response: String,
    ) -> Result<bool> {
        self.apply_grounded_source_proposal(SourceGroundedProposal::TableGridRepair {
            proposal_id: self.next_grounded_proposal_id("table-grid", &exact_response),
            table_id: table_id.to_string(),
            exact_response,
        })
    }

    /// Apply one exact VLM kind proposal only when the registered structural gate agrees.
    pub fn apply_table_classification(
        &mut self,
        table_id: &str,
        exact_response: String,
    ) -> Result<bool> {
        self.apply_grounded_source_proposal(SourceGroundedProposal::TableClassification {
            proposal_id: self.next_grounded_proposal_id("table-kind", &exact_response),
            table_id: table_id.to_string(),
            exact_response,
        })
    }

    fn next_grounded_proposal_id(&self, kind: &str, response: &str) -> String {
        let ordinal = self
            .proof_context
            .as_ref()
            .map_or(0, |context| context.grounded_proposals.len());
        let digest = Sha256Digest::of_bytes(response.as_bytes());
        format!("source-{kind}-{ordinal:08}-{}", &digest.as_str()[..16])
    }

    fn apply_grounded_source_proposal(&mut self, proposal: SourceGroundedProposal) -> Result<bool> {
        self.persisted_clone()?.verify_canonical_proof()?;
        let mut candidate = self.clone();
        let applied = apply_source_grounded_proposal(
            &proposal,
            &mut candidate.visual_assets,
            &mut candidate.structured_tables,
        )
        .map_err(source_derivation_error)?;
        if !applied {
            return Ok(false);
        }
        let mut context = candidate.proof_context.clone().ok_or_else(|| {
            AppError::InvalidStageArtifact(
                "verified SourceIR lost its proof context during mutation".to_string(),
            )
        })?;
        #[cfg(any(test, feature = "test-support"))]
        if context.test_fixture {
            return Err(AppError::InvalidStageArtifact(
                "synthetic fixture authority cannot enter the production enrichment path"
                    .to_string(),
            ));
        }
        context.grounded_proposals.push(proposal);
        let mut persisted = candidate.persisted_clone()?;
        persisted.proof_context = None;
        persisted.proof_ledger = None;
        candidate.refresh_proof_from_context(persisted, context)?;
        *self = candidate;
        Ok(true)
    }

    /// Deterministic current SourceIR evaluation. The report is recomputed by the registered
    /// validation rule on canonical reload; callers cannot supply alternative metrics/findings.
    pub fn validation_report(&self) -> Result<ValidationReportRecord> {
        let persisted = self.persisted_clone()?;
        persisted.verify_canonical_proof()?;
        source_validation_report_from_fields(
            &persisted
                .public_field_values()
                .map_err(source_derivation_error)?,
        )
        .map_err(source_derivation_error)
    }

    /// Replace the optional validation backannotation with the exact registered evaluation.
    pub fn apply_validation_report(&mut self, report: ValidationReportRecord) -> Result<()> {
        let expected = self.validation_report()?;
        if report != expected {
            return Err(AppError::InvalidStageArtifact(
                "SourceIR validation backannotation differs from the registered evaluation"
                    .to_string(),
            ));
        }
        let mut candidate = self.clone();
        candidate.validation_reports = vec![report];
        let mut context = candidate.proof_context.clone().ok_or_else(|| {
            AppError::InvalidStageArtifact(
                "verified SourceIR lost its proof context during validation".to_string(),
            )
        })?;
        #[cfg(any(test, feature = "test-support"))]
        let test_fixture = context.test_fixture;
        context.field_premises.retain(|key, _| {
            key != "validation_reports" && !key.starts_with("validation_reports[")
        });
        let validation_value = serde_json::to_value(&candidate.validation_reports)?;
        context
            .field_premises
            .insert("validation_reports".to_string(), validation_value.clone());
        if let Some(reports) = validation_value.as_array() {
            for (index, report) in reports.iter().enumerate() {
                context
                    .field_premises
                    .insert(format!("validation_reports[{index}]"), report.clone());
            }
        }
        let mut persisted = candidate.persisted_clone()?;
        persisted.proof_context = None;
        persisted.proof_ledger = None;
        #[cfg(any(test, feature = "test-support"))]
        if test_fixture {
            context = persisted
                .test_fixture_proof_context()
                .map_err(source_derivation_error)?;
        }
        candidate.refresh_proof_from_context(persisted, context)?;
        *self = candidate;
        Ok(())
    }

    /// Explicit test-only fixture seam. Production code cannot use this to bless arbitrary field
    /// mutation; test modules use it only after assembling synthetic, classifier-valid captures.
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn write_test_fixture_to_disk(&self) -> Result<()> {
        let mut fixture = self.clone();
        let mut persisted = fixture.persisted_clone()?;
        persisted.proof_context = None;
        persisted.proof_ledger = None;
        let context = persisted
            .test_fixture_proof_context()
            .map_err(source_derivation_error)?;
        fixture.refresh_proof_from_context(persisted, context)?;
        fixture.write_to_disk()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn is_test_fixture(&self) -> bool {
        self.proof_context
            .as_ref()
            .is_some_and(|context| context.test_fixture)
    }

    /// Current proof ledger after successful construction or canonical reload.
    pub fn proof_ledger(&self) -> Option<&ProofLedger> {
        self.proof_ledger.as_ref()
    }

    fn public_field_values(&self) -> DerivationResult<BTreeMap<String, serde_json::Value>> {
        let serialized = serde_json::to_value(self).map_err(|error| {
            DerivationError::new(format!("cannot serialize SourceIR fields: {error}"))
        })?;
        let object = serialized
            .as_object()
            .ok_or_else(|| DerivationError::new("serialized SourceIR must be a JSON object"))?;
        SOURCE_RULE_FIELDS
            .iter()
            .map(|(field, _)| {
                object
                    .get(*field)
                    .cloned()
                    .or_else(|| (*field == "document_profile").then_some(serde_json::Value::Null))
                    .map(|value| ((*field).to_string(), value))
                    .ok_or_else(|| {
                        DerivationError::new(format!(
                            "SourceIR public field '{field}' is absent from serialization"
                        ))
                    })
            })
            .collect()
    }

    fn expected_proof_context(&self) -> DerivationResult<SourceProofContext> {
        let mut neutral = self.clone();
        neutral.proof_context = None;
        neutral.proof_ledger = None;
        for asset in &mut neutral.visual_assets {
            asset.diagram_kind = DiagramKind::Unknown;
        }
        for table in &mut neutral.structured_tables {
            table.table_kind = TableKind::Unknown;
        }
        for section in &mut neutral.document_sections {
            section.section_kind = SectionKind::Unknown;
        }
        let mut field_premises = neutral.public_field_values()?;
        for (field, _) in SOURCE_RULE_FIELDS {
            if let Some(items) = field_premises
                .get(*field)
                .and_then(serde_json::Value::as_array)
                .cloned()
            {
                for (index, item) in items.into_iter().enumerate() {
                    field_premises.insert(format!("{field}[{index}]"), item);
                }
            }
        }
        Ok(SourceProofContext {
            schema_version: SOURCE_PROOF_CONTEXT_SCHEMA_VERSION,
            field_premises,
            grounded_proposals: Vec::new(),
            #[cfg(any(test, feature = "test-support"))]
            test_fixture: false,
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    fn test_fixture_proof_context(&self) -> DerivationResult<SourceProofContext> {
        let mut field_premises = self.public_field_values()?;
        for (field, _) in SOURCE_RULE_FIELDS {
            if let Some(items) = field_premises
                .get(*field)
                .and_then(serde_json::Value::as_array)
                .cloned()
            {
                for (index, item) in items.into_iter().enumerate() {
                    field_premises.insert(format!("{field}[{index}]"), item);
                }
            }
        }
        Ok(SourceProofContext {
            schema_version: SOURCE_PROOF_CONTEXT_SCHEMA_VERSION,
            field_premises,
            grounded_proposals: Vec::new(),
            test_fixture: true,
        })
    }

    fn claim_inputs(&self) -> DerivationResult<Vec<SourceClaimInput>> {
        let fields = self.public_field_values()?;
        let mut claims = Vec::new();
        for (field, family) in SOURCE_RULE_FIELDS {
            let value = fields.get(*field).cloned().ok_or_else(|| {
                DerivationError::new(format!("SourceIR claim field '{field}' is absent"))
            })?;
            let rule_id =
                RuleId::try_from(format!("{family}.{field}.v1")).map_err(DerivationError::new)?;
            let confidence = match *family {
                "source.capture" => ProofConfidence::Captured,
                "source.residual" => ProofConfidence::Residual,
                _ => ProofConfidence::Deterministic,
            };
            claims.push(SourceClaimInput {
                address: ClaimAddress::new(IrStage::SourceIr, *field, "root", None)?,
                rule_id: rule_id.clone(),
                premise_key: (*field).to_string(),
                confidence,
                conclusion: value.clone(),
            });
            if let Some(items) = value.as_array() {
                for (index, item) in items.iter().enumerate() {
                    claims.push(SourceClaimInput {
                        address: ClaimAddress::new(
                            IrStage::SourceIr,
                            *field,
                            format!("record-{index:08}"),
                            Some(format!("[{index}]")),
                        )?,
                        rule_id: rule_id.clone(),
                        premise_key: format!("{field}[{index}]"),
                        confidence,
                        conclusion: item.clone(),
                    });
                }
            }
        }
        Ok(claims)
    }

    fn validate_proof_context(&self, context: &SourceProofContext) -> DerivationResult<()> {
        #[cfg(any(test, feature = "test-support"))]
        if context.test_fixture {
            return if context == &self.test_fixture_proof_context()? {
                Ok(())
            } else {
                Err(DerivationError::new(
                    "test fixture proof context is not an exact field projection",
                ))
            };
        }

        let current = self.public_field_values()?;
        for (field, _) in SOURCE_RULE_FIELDS {
            let captured = context.field_premises.get(*field).ok_or_else(|| {
                DerivationError::new(format!("SourceIR proof context lacks field '{field}'"))
            })?;
            if !matches!(
                *field,
                "visual_assets" | "structured_tables" | "document_sections"
            ) && current.get(*field) != Some(captured)
            {
                return Err(DerivationError::new(format!(
                    "SourceIR field '{field}' differs from its exact captured premise"
                )));
            }
            if let Some(items) = captured.as_array() {
                for (index, item) in items.iter().enumerate() {
                    if context.field_premises.get(&format!("{field}[{index}]")) != Some(item) {
                        return Err(DerivationError::new(format!(
                            "SourceIR per-record premise '{field}[{index}]' differs from its root capture"
                        )));
                    }
                }
            }
        }

        let mut visuals: Vec<VisualAsset> = serde_json::from_value(
            context
                .field_premises
                .get("visual_assets")
                .cloned()
                .ok_or_else(|| DerivationError::new("visual capture root is absent"))?,
        )
        .map_err(|error| DerivationError::new(format!("invalid visual capture root: {error}")))?;
        let mut tables: Vec<StructuredTableRecord> = serde_json::from_value(
            context
                .field_premises
                .get("structured_tables")
                .cloned()
                .ok_or_else(|| DerivationError::new("table capture root is absent"))?,
        )
        .map_err(|error| DerivationError::new(format!("invalid table capture root: {error}")))?;
        let mut sections: Vec<ContentSectionRecord> = serde_json::from_value(
            context
                .field_premises
                .get("document_sections")
                .cloned()
                .ok_or_else(|| DerivationError::new("section capture root is absent"))?,
        )
        .map_err(|error| DerivationError::new(format!("invalid section capture root: {error}")))?;
        classify_source_captures(&mut visuals, &mut tables, &mut sections);
        let mut proposal_ids = BTreeSet::new();
        for proposal in &context.grounded_proposals {
            if !proposal_ids.insert(proposal.proposal_id()) {
                return Err(DerivationError::new(format!(
                    "duplicate grounded SourceIR proposal id '{}'",
                    proposal.proposal_id()
                )));
            }
            if !apply_source_grounded_proposal(proposal, &mut visuals, &mut tables)? {
                return Err(DerivationError::new(format!(
                    "grounded SourceIR proposal '{}' does not authorize a refinement",
                    proposal.proposal_id()
                )));
            }
        }
        if visuals != self.visual_assets {
            return Err(DerivationError::new(
                "SourceIR visual assets differ from registered capture/proposal replay",
            ));
        }
        if tables != self.structured_tables {
            return Err(DerivationError::new(
                "SourceIR structured tables differ from registered capture/proposal replay",
            ));
        }
        if sections != self.document_sections {
            return Err(DerivationError::new(
                "SourceIR document sections differ from registered capture/proposal replay",
            ));
        }
        Ok(())
    }

    fn proof_kernel(
        &self,
        context: &SourceProofContext,
    ) -> DerivationResult<(
        crate::ir::derivation::PromotionKernel,
        BTreeMap<String, Vec<PremiseRef>>,
    )> {
        if context.schema_version != SOURCE_PROOF_CONTEXT_SCHEMA_VERSION {
            return Err(DerivationError::new(format!(
                "unsupported SourceIR proof-context schema {}",
                context.schema_version
            )));
        }
        self.validate_proof_context(context)?;
        let capture_digest = Sha256Digest::of_serializable(context)?;
        let mut builder = PromotionKernelBuilder::new(capture_digest, source_rule_registry()?)?;
        let mut premises = BTreeMap::new();
        {
            let mut capture = builder.capture();
            for (key, value) in &context.field_premises {
                let bytes = serde_json::to_vec(value).map_err(|error| {
                    DerivationError::new(format!("cannot serialize SourceIR premise: {error}"))
                })?;
                #[cfg(any(test, feature = "test-support"))]
                let prefix = if context.test_fixture {
                    "test-fixture-source-field:"
                } else {
                    "source-field:"
                };
                #[cfg(not(any(test, feature = "test-support")))]
                let prefix = "source-field:";
                let mut captured = vec![capture.source_span(format!("{prefix}{key}"), &bytes)?];
                if let Some(index) = key
                    .strip_prefix("visual_assets[")
                    .and_then(|index| index.strip_suffix(']'))
                    .and_then(|index| index.parse::<usize>().ok())
                {
                    let asset: VisualAsset =
                        serde_json::from_value(value.clone()).map_err(|error| {
                            DerivationError::new(format!(
                                "invalid visual capture premise '{key}': {error}"
                            ))
                        })?;
                    captured.push(capture.visual_region(
                        format!("visual-asset:{index}:{}", asset.asset_id),
                        &bytes,
                    )?);
                } else if let Some(index) = key
                    .strip_prefix("page_artifacts[")
                    .and_then(|index| index.strip_suffix(']'))
                    .and_then(|index| index.parse::<usize>().ok())
                {
                    let page: PageArtifact =
                        serde_json::from_value(value.clone()).map_err(|error| {
                            DerivationError::new(format!(
                                "invalid page capture premise '{key}': {error}"
                            ))
                        })?;
                    captured.push(capture.visual_region(
                        format!("page-artifact:{index}:{}", page.page_id),
                        &bytes,
                    )?);
                } else if key.starts_with("structured_tables[") {
                    let table: StructuredTableRecord = serde_json::from_value(value.clone())
                        .map_err(|error| {
                            DerivationError::new(format!(
                                "invalid table capture premise '{key}': {error}"
                            ))
                        })?;
                    for (row_index, row) in
                        table.header_rows.iter().chain(&table.body_rows).enumerate()
                    {
                        let row_index = u32::try_from(row_index).map_err(|_| {
                            DerivationError::new(format!(
                                "table '{}' exceeds the supported row count",
                                table.table_id
                            ))
                        })?;
                        for (column_index, cell) in row.iter().enumerate() {
                            let column_index = u32::try_from(column_index).map_err(|_| {
                                DerivationError::new(format!(
                                    "table '{}' exceeds the supported column count",
                                    table.table_id
                                ))
                            })?;
                            let cell_bytes = serde_json::to_vec(cell).map_err(|error| {
                                DerivationError::new(format!(
                                    "cannot serialize table-cell premise: {error}"
                                ))
                            })?;
                            captured.push(capture.table_cell(
                                table.table_id.clone(),
                                row_index,
                                column_index,
                                &cell_bytes,
                            )?);
                        }
                    }
                }
                premises.insert(key.clone(), captured);
            }
            let base_visuals: Vec<VisualAsset> = serde_json::from_value(
                context.field_premises["visual_assets"].clone(),
            )
            .map_err(|error| {
                DerivationError::new(format!("invalid visual proposal grounding: {error}"))
            })?;
            let base_tables: Vec<StructuredTableRecord> =
                serde_json::from_value(context.field_premises["structured_tables"].clone())
                    .map_err(|error| {
                        DerivationError::new(format!("invalid table proposal grounding: {error}"))
                    })?;
            for proposal in &context.grounded_proposals {
                let target_index = proposal.target_index(&base_visuals, &base_tables)?;
                let record_key = format!(
                    "{}[{target_index}]",
                    source_grounded_proposal_surface(proposal)
                );
                let target_premises = premises.get(&record_key).ok_or_else(|| {
                    DerivationError::new(format!(
                        "grounded proposal '{}' lacks target capture",
                        proposal.proposal_id()
                    ))
                })?;
                let typed_kind = match proposal {
                    SourceGroundedProposal::VisualObservation { .. } => PremiseKind::VisualRegion,
                    SourceGroundedProposal::TableGridRepair { .. }
                    | SourceGroundedProposal::TableClassification { .. } => PremiseKind::TableCell,
                };
                let direct_grounding = target_premises
                    .iter()
                    .find(|premise| premise.kind() == typed_kind)
                    .or_else(|| target_premises.first())
                    .cloned()
                    .ok_or_else(|| {
                        DerivationError::new(format!(
                            "grounded proposal '{}' has an empty target capture",
                            proposal.proposal_id()
                        ))
                    })?;
                let payload = serde_json::to_vec(proposal).map_err(|error| {
                    DerivationError::new(format!("cannot serialize grounded proposal: {error}"))
                })?;
                let model_premise = capture.grounded_model_proposal(
                    proposal.proposal_id(),
                    &payload,
                    vec![direct_grounding],
                )?;
                premises
                    .get_mut(source_grounded_proposal_surface(proposal))
                    .expect("classification root premise")
                    .push(model_premise.clone());
                premises
                    .get_mut(&record_key)
                    .expect("classification record premise")
                    .push(model_premise);
            }
        }
        Ok((builder.seal(), premises))
    }

    fn refresh_canonical_proof(&mut self) -> Result<()> {
        let mut persisted = self.persisted_clone()?;
        persisted.proof_context = None;
        persisted.proof_ledger = None;
        let context = persisted
            .expected_proof_context()
            .map_err(source_derivation_error)?;
        self.refresh_proof_from_context(persisted, context)
    }

    fn refresh_proof_from_context(
        &mut self,
        persisted: Self,
        context: SourceProofContext,
    ) -> Result<()> {
        let claims = persisted.claim_inputs().map_err(source_derivation_error)?;
        let (mut kernel, premises) = persisted
            .proof_kernel(&context)
            .map_err(source_derivation_error)?;
        let mut root_claims: BTreeMap<String, PremiseRef> = BTreeMap::new();
        for claim in claims {
            let claim_premises = if claim.address.surface() == "validation_reports" {
                if claim.address.field_path().is_some() {
                    vec![
                        root_claims
                            .get("validation_reports")
                            .cloned()
                            .ok_or_else(|| {
                                source_derivation_error("validation record precedes its root proof")
                            })?,
                    ]
                } else {
                    root_claims
                        .iter()
                        .filter(|(surface, _)| surface.as_str() != "validation_reports")
                        .map(|(_, premise)| premise.clone())
                        .collect()
                }
            } else {
                premises.get(&claim.premise_key).cloned().ok_or_else(|| {
                    source_derivation_error(format!(
                        "missing SourceIR proof premise '{}'",
                        claim.premise_key
                    ))
                })?
            };
            let proposal = kernel.grammar_capability().propose(
                claim.address,
                claim.rule_id,
                claim_premises.clone(),
                Vec::new(),
                if claim_premises
                    .iter()
                    .any(|premise| premise.kind() == PremiseKind::GroundedModelProposal)
                {
                    ProofConfidence::GroundedModel
                } else {
                    claim.confidence
                },
                claim.conclusion,
            );
            let proved = kernel.promote(proposal).map_err(source_derivation_error)?;
            if proved.proof().address().field_path().is_none() {
                root_claims.insert(
                    proved.proof().address().surface().to_string(),
                    PremiseRef::UpstreamClaim {
                        address: proved.proof().address().clone(),
                        conclusion_sha256: proved.proof().conclusion_sha256().clone(),
                    },
                );
            }
        }
        let ledger = kernel
            .finish()
            .map_err(source_derivation_error)?
            .into_ledger();
        self.proof_context = Some(context);
        self.proof_ledger = Some(ledger);
        Ok(())
    }

    fn verify_canonical_proof(&self) -> Result<()> {
        self.verified_canonical_proof().map(|_| ())
    }

    fn verified_canonical_proof(&self) -> Result<VerifiedProofLedger> {
        if self.schema_version != SOURCE_IR_SCHEMA_VERSION {
            return Err(AppError::InvalidStageArtifact(format!(
                "SourceIR schema {} cannot receive current canonical authority",
                self.schema_version
            )));
        }
        let context = self.proof_context.as_ref().ok_or_else(|| {
            AppError::InvalidStageArtifact(
                "current SourceIR is proofless; rebuild it from captured source evidence"
                    .to_string(),
            )
        })?;
        let ledger = self.proof_ledger.clone().ok_or_else(|| {
            AppError::InvalidStageArtifact(
                "current SourceIR is missing its proof ledger; rebuild is required".to_string(),
            )
        })?;
        let claims = self.claim_inputs().map_err(source_derivation_error)?;
        let conclusions = claims
            .into_iter()
            .map(|claim| {
                serde_json::to_vec(&claim.conclusion)
                    .map(|bytes| (claim.address, bytes))
                    .map_err(|error| {
                        source_derivation_error(format!(
                            "cannot serialize SourceIR conclusion: {error}"
                        ))
                    })
            })
            .collect::<Result<BTreeMap<_, _>>>()?;
        let (kernel, _) = self
            .proof_kernel(context)
            .map_err(source_derivation_error)?;
        kernel
            .verify_persisted(ledger, &conclusions)
            .map_err(source_derivation_error)
    }

    fn persisted_clone(&self) -> Result<Self> {
        let mut persisted = self.clone();
        let source_origin = persisted.source.resolved_origin()?;
        persisted.source.path_origin = Some(source_origin);
        persisted.source.canonical_path =
            normalize_for_storage(&persisted.source.canonical_path, source_origin)?;
        persisted.source.requested_path = if source_origin == PersistedPathOrigin::ExternalInput
            && persisted.source.requested_path.is_relative()
        {
            // Pre-contract artifacts could retain a caller-relative spelling for an external
            // input even though only the canonical absolute path remains meaningful after a
            // process or repository move.
            persisted.source.canonical_path.clone()
        } else {
            normalize_for_storage(&persisted.source.requested_path, source_origin)?
        };
        persisted.artifact_layout.normalize_for_storage()?;

        let promoted_origin = if matches!(persisted.source.source_kind, SourceKind::Markdown) {
            source_origin
        } else {
            PersistedPathOrigin::RepositoryOwned
        };
        normalize_optional_path(
            &mut persisted.normalization_plan.promoted_markdown_path,
            promoted_origin,
        )?;
        normalize_optional_path(
            &mut persisted.normalization_plan.auxiliary_asset_dir,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        normalize_optional_path(
            &mut persisted.normalization_plan.metadata_output_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;

        for page in &mut persisted.page_artifacts {
            normalize_optional_path(
                &mut page.page_image_path,
                PersistedPathOrigin::RepositoryOwned,
            )?;
            normalize_optional_path(
                &mut page.layout_metadata_path,
                PersistedPathOrigin::RepositoryOwned,
            )?;
        }
        for asset in &mut persisted.visual_assets {
            normalize_optional_path(&mut asset.image_path, PersistedPathOrigin::RepositoryOwned)?;
            normalize_optional_path(
                &mut asset.caption_source_path,
                PersistedPathOrigin::RepositoryOwned,
            )?;
        }
        for binding in &mut persisted.placeholder_bindings {
            binding.normalized_source_path = normalize_for_storage(
                &binding.normalized_source_path,
                PersistedPathOrigin::RepositoryOwned,
            )?;
        }
        Ok(persisted)
    }

    fn runtime_clone(&self) -> Result<Self> {
        let mut runtime = self.clone();
        let source_origin = runtime.source.resolved_origin()?;
        runtime.source.path_origin = Some(source_origin);
        runtime.source.canonical_path =
            resolve_reference(&runtime.source.canonical_path, source_origin)?;
        runtime.source.requested_path = if source_origin == PersistedPathOrigin::ExternalInput
            && runtime.source.requested_path.is_relative()
        {
            runtime.source.canonical_path.clone()
        } else {
            resolve_reference(&runtime.source.requested_path, source_origin)?
        };
        runtime.artifact_layout = runtime.artifact_layout.runtime_layout()?;

        if let Some(path) = &mut runtime.normalization_plan.promoted_markdown_path {
            *path = if matches!(runtime.source.source_kind, SourceKind::Markdown) {
                resolve_reference(path, source_origin)?
            } else {
                resolve_repository_output(path)?
            };
        }
        resolve_optional_repository_output(&mut runtime.normalization_plan.auxiliary_asset_dir)?;
        resolve_optional_repository_output(&mut runtime.normalization_plan.metadata_output_path)?;
        for page in &mut runtime.page_artifacts {
            resolve_optional_repository_output(&mut page.page_image_path)?;
            resolve_optional_repository_output(&mut page.layout_metadata_path)?;
        }
        for asset in &mut runtime.visual_assets {
            resolve_optional_repository_output(&mut asset.image_path)?;
            resolve_optional_repository_output(&mut asset.caption_source_path)?;
        }
        for binding in &mut runtime.placeholder_bindings {
            binding.normalized_source_path =
                resolve_repository_output(&binding.normalized_source_path)?;
        }
        Ok(runtime)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceRegistration {
    pub requested_path: PathBuf,
    pub canonical_path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_origin: Option<PersistedPathOrigin>,
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

impl SourceRegistration {
    fn resolved_origin(&self) -> Result<PersistedPathOrigin> {
        if let Some(origin) = self.path_origin {
            return Ok(origin);
        }
        if self.canonical_path.is_relative() {
            return Ok(PersistedPathOrigin::RepositoryOwned);
        }
        if self.canonical_path.exists() {
            return infer_existing_origin(&self.canonical_path);
        }
        resolve_existing(&self.canonical_path, PersistedPathOrigin::RepositoryOwned)?;
        Ok(PersistedPathOrigin::RepositoryOwned)
    }
}

impl SourceArtifactLayout {
    fn normalize_for_storage(&mut self) -> Result<()> {
        self.artifact_root =
            normalize_for_storage(&self.artifact_root, PersistedPathOrigin::RepositoryOwned)?;
        self.source_ir_path =
            normalize_for_storage(&self.source_ir_path, PersistedPathOrigin::RepositoryOwned)?;
        self.normalized_root =
            normalize_for_storage(&self.normalized_root, PersistedPathOrigin::RepositoryOwned)?;
        self.page_image_root =
            normalize_for_storage(&self.page_image_root, PersistedPathOrigin::RepositoryOwned)?;
        self.page_artifact_manifest_path = normalize_for_storage(
            &self.page_artifact_manifest_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        self.visual_asset_root = normalize_for_storage(
            &self.visual_asset_root,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        self.visual_asset_manifest_path = normalize_for_storage(
            &self.visual_asset_manifest_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        self.backend_raw_output_path = normalize_for_storage(
            &self.backend_raw_output_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        Ok(())
    }

    fn runtime_layout(&self) -> Result<Self> {
        Ok(Self {
            artifact_root: resolve_repository_output(&self.artifact_root)?,
            source_ir_path: resolve_repository_output(&self.source_ir_path)?,
            normalized_root: resolve_repository_output(&self.normalized_root)?,
            page_image_root: resolve_repository_output(&self.page_image_root)?,
            page_artifact_manifest_path: resolve_repository_output(
                &self.page_artifact_manifest_path,
            )?,
            visual_asset_root: resolve_repository_output(&self.visual_asset_root)?,
            visual_asset_manifest_path: resolve_repository_output(
                &self.visual_asset_manifest_path,
            )?,
            backend_raw_output_path: resolve_repository_output(&self.backend_raw_output_path)?,
        })
    }
}

fn normalize_optional_path(path: &mut Option<PathBuf>, origin: PersistedPathOrigin) -> Result<()> {
    if let Some(value) = path {
        *value = normalize_for_storage(value, origin)?;
    }
    Ok(())
}

fn resolve_optional_repository_output(path: &mut Option<PathBuf>) -> Result<()> {
    if let Some(value) = path {
        *value = resolve_repository_output(value)?;
    }
    Ok(())
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
/// Set only when the caption explicitly names a generic visual form (zero VLM deps).
/// Used to route VLM enrichment calls at the `specforge enrich` step.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiagramKind {
    /// Waveform on a horizontal time axis.
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
    /// Semantic diagram kind proven by an explicit generic caption form at ingest time.
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
    use std::collections::{BTreeMap, BTreeSet};
    use std::env;
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use crate::error::{AppError, Result};
    use crate::ir::IrStage;
    use crate::ir::source::AdapterTarget;
    use crate::test_support::env_var_lock;

    use super::{
        AutomationConfidence, DiagramKind, NormalizationBackend, SectionKind, SourceIr, SourceKind,
        StructuredTableCellRecord, StructuredTableRecord, TableKind, VisualAsset, VisualAssetKind,
        document_key, normalize_timing_table_kinds, stable_stem, timing_caption_unit,
        timing_table_columns, timing_table_has_structural_authority,
    };

    fn table_cell(text: &str, is_header: bool) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        }
    }

    fn timing_test_table(
        kind: TableKind,
        caption: Option<&str>,
        header_rows: Vec<Vec<StructuredTableCellRecord>>,
    ) -> StructuredTableRecord {
        let row_count = header_rows.len() as u32;
        let col_count = header_rows.iter().map(|row| row.len()).max().unwrap_or(0) as u32;
        StructuredTableRecord {
            table_id: "table_test".to_string(),
            asset_id: "asset_test".to_string(),
            page_id: None,
            caption_text: caption.map(str::to_string),
            source_ref: None,
            table_kind: kind,
            header_rows,
            body_rows: Vec::new(),
            row_count,
            col_count,
        }
    }

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
    fn timing_table_authority_uses_structural_identifier_safe_headers() {
        let mut instruction_table = timing_test_table(
            TableKind::TimingParameter,
            None,
            vec![
                vec![
                    table_cell("Instruction group", true),
                    table_cell("AArch64 instructions", true),
                    table_cell("Exec latency", true),
                    table_cell("Execution throughput", true),
                ],
                vec![
                    table_cell("Signed minimum", true),
                    table_cell("SMIN", false),
                    table_cell("4", false),
                    table_cell("2", false),
                ],
            ],
        );
        assert!(
            timing_table_columns(&instruction_table).is_none(),
            "data-row text and substrings inside instruction headers grant no timing authority"
        );

        let identifier_table = timing_test_table(
            TableKind::TimingParameter,
            None,
            vec![vec![
                table_cell("OPTIMAL_TRIM_UNIT_SIZE", true),
                table_cell("Maximum value", true),
            ]],
        );
        assert!(
            timing_table_columns(&identifier_table).is_none(),
            "underscored identifiers do not contribute a standalone unit token"
        );

        let mut trapped_timing = timing_test_table(
            TableKind::Unknown,
            Some("Receiver timing (all values in ns)"),
            vec![
                vec![
                    table_cell("", false),
                    table_cell("MIN", true),
                    table_cell("TYP", true),
                    table_cell("MAX", true),
                ],
                vec![
                    table_cell("clock period", true),
                    table_cell("360", false),
                    table_cell("400", false),
                    table_cell("440", false),
                ],
            ],
        );
        let columns = timing_table_columns(&trapped_timing)
            .expect("the genuine first-row min/typ/max schema is authoritative");
        assert_eq!(
            (columns.name, columns.min, columns.typ, columns.max),
            (0, Some(1), Some(2), Some(3))
        );
        assert_eq!(timing_caption_unit(&trapped_timing).as_deref(), Some("ns"));

        let mut misleading_caption = trapped_timing.clone();
        misleading_caption.caption_text = Some("All values in nominal order; ns example".into());
        assert_eq!(timing_caption_unit(&misleading_caption), None);
        misleading_caption.caption_text = Some("Timing values can be measured in ns".into());
        assert_eq!(timing_caption_unit(&misleading_caption), None);
        misleading_caption.caption_text = Some("All values in clock cycles".into());
        assert_eq!(
            timing_caption_unit(&misleading_caption).as_deref(),
            Some("clock cycles")
        );

        let mut variant_timing = timing_test_table(
            TableKind::TimingParameter,
            Some("AC timing limits"),
            vec![
                vec![
                    table_cell("Symbol", true),
                    table_cell("Parameters", true),
                    table_cell("Mode A", true),
                    table_cell("Mode A", true),
                    table_cell("Mode B", true),
                    table_cell("Mode B", true),
                    table_cell("Units", true),
                ],
                vec![
                    table_cell("Symbol", true),
                    table_cell("Parameters", true),
                    table_cell("Min", true),
                    table_cell("Max", true),
                    table_cell("Min", true),
                    table_cell("Max", true),
                    table_cell("Units", true),
                ],
            ],
        );
        assert!(timing_table_has_structural_authority(&variant_timing));
        assert!(
            timing_table_columns(&variant_timing).is_none(),
            "variant limits remain timing-bearing but cannot be collapsed into one scalar record"
        );

        normalize_timing_table_kinds(std::slice::from_mut(&mut instruction_table));
        normalize_timing_table_kinds(std::slice::from_mut(&mut trapped_timing));
        normalize_timing_table_kinds(std::slice::from_mut(&mut variant_timing));
        assert_eq!(instruction_table.table_kind, TableKind::Unknown);
        assert_eq!(
            trapped_timing.table_kind,
            TableKind::Unknown,
            "structural shape alone must not promote an unclassified table"
        );
        assert_eq!(variant_timing.table_kind, TableKind::TimingParameter);
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
    fn source_ir_proof_covers_every_root_and_nonempty_collection_record() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("proof.md");
        fs::write(&source, "# proof\n")?;
        let source_ir = SourceIr::build(&source, &tempdir.path().join("artifacts"))?;
        let expected_records = source_ir
            .public_field_values()
            .expect("serialize SourceIR fields")
            .values()
            .filter_map(serde_json::Value::as_array)
            .map(Vec::len)
            .sum::<usize>();
        let ledger = source_ir
            .proof_ledger()
            .expect("current SourceIR proof ledger");
        assert_eq!(
            ledger.claims().len(),
            super::SOURCE_RULE_FIELDS.len() + expected_records
        );
        assert!(ledger.claims().iter().any(|claim| {
            claim.address().surface() == "planned_actions"
                && claim.address().field_path() == Some("[0]")
        }));
        Ok(())
    }

    #[test]
    fn source_rule_registry_covers_all_five_families_with_typed_premises() {
        use crate::ir::derivation::PremiseKind;

        let registry = super::source_rule_registry().expect("SourceIR rule registry");
        let by_id = registry
            .descriptors()
            .map(|descriptor| {
                (
                    descriptor.rule_id().as_str().to_string(),
                    descriptor.premise_kinds().clone(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        assert_eq!(by_id.len(), super::SOURCE_RULE_FIELDS.len());
        assert_eq!(
            by_id["source.envelope.source.v1"],
            BTreeSet::from([PremiseKind::SourceSpan, PremiseKind::UniversalAxiom])
        );
        assert_eq!(
            by_id["source.capture.content_elements.v1"],
            BTreeSet::from([
                PremiseKind::SourceSpan,
                PremiseKind::TableCell,
                PremiseKind::VisualRegion,
            ])
        );
        assert_eq!(
            by_id["source.classification.structured_tables.v1"],
            BTreeSet::from([
                PremiseKind::SourceSpan,
                PremiseKind::TableCell,
                PremiseKind::VisualRegion,
                PremiseKind::GroundedModelProposal,
            ])
        );
        assert_eq!(
            by_id["source.residual.residual_decisions.v1"],
            BTreeSet::from([PremiseKind::SourceSpan, PremiseKind::UniversalAxiom])
        );
        assert_eq!(
            by_id["source.validation.validation_reports.v1"],
            BTreeSet::from([PremiseKind::UpstreamClaim])
        );
    }

    #[test]
    fn source_classification_records_carry_their_typed_capture_premises() -> Result<()> {
        use crate::ir::derivation::PremiseKind;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("typed-captures.md");
        fs::write(&source, "# typed captures\n")?;
        let mut source_ir = SourceIr::build(&source, &tempdir.path().join("artifacts"))?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "asset-typed".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: None,
            image_path: None,
            caption_text: Some("Timing diagram".to_string()),
            caption_source_path: None,
            source_ref: Some("line-1".to_string()),
            placeholder_text: None,
            note: None,
            diagram_kind: DiagramKind::TimingDiagram,
        });
        source_ir.structured_tables.push(timing_test_table(
            TableKind::TimingParameter,
            Some("Timing parameters"),
            vec![vec![table_cell("Symbol", true), table_cell("Max", true)]],
        ));
        source_ir.refresh_canonical_proof()?;

        let ledger = source_ir.proof_ledger().expect("current SourceIR proof");
        let visual = ledger
            .claims()
            .iter()
            .find(|claim| {
                claim.address().surface() == "visual_assets"
                    && claim.address().field_path() == Some("[0]")
            })
            .expect("visual record proof");
        assert!(
            visual
                .premises()
                .iter()
                .any(|premise| premise.kind() == PremiseKind::VisualRegion)
        );
        let table = ledger
            .claims()
            .iter()
            .find(|claim| {
                claim.address().surface() == "structured_tables"
                    && claim.address().field_path() == Some("[0]")
            })
            .expect("table record proof");
        assert!(
            table
                .premises()
                .iter()
                .any(|premise| premise.kind() == PremiseKind::TableCell)
        );
        Ok(())
    }

    #[test]
    fn grounded_visual_observation_replays_on_reload_and_tampering_fails() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("visual.md");
        let artifact_base = tempdir.path().join("artifacts");
        fs::write(&source, "# visual\n")?;
        let mut source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "asset-0".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: None,
            image_path: None,
            caption_text: Some("Timing diagram".to_string()),
            caption_source_path: None,
            source_ref: Some("line-1".to_string()),
            placeholder_text: None,
            note: None,
            diagram_kind: DiagramKind::TimingDiagram,
        });
        source_ir.refresh_canonical_proof()?;
        source_ir.apply_visual_observation(
            "asset-0",
            DiagramKind::TimingDiagram,
            r#"{"signals":[],"annotations":[]}"#.to_string(),
        )?;
        let captured_visuals: Vec<VisualAsset> = serde_json::from_value(
            source_ir
                .proof_context
                .as_ref()
                .expect("proof context")
                .field_premises["visual_assets"]
                .clone(),
        )?;
        assert_eq!(captured_visuals[0].diagram_kind, DiagramKind::Unknown);
        assert_eq!(
            source_ir.visual_assets[0].diagram_kind,
            DiagramKind::TimingDiagram
        );
        source_ir.write_to_disk()?;
        let artifact = source_ir.artifact_layout.source_ir_path.clone();
        let reloaded = SourceIr::load_from_path(&artifact)?;
        assert!(
            reloaded.visual_assets[0]
                .note
                .as_deref()
                .is_some_and(|note| note.contains("\"signals\":[]"))
        );

        let mut json = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&artifact)?)?;
        json["visual_assets"][0]["note"] = serde_json::json!(
            "vlm_timing_diagram_extraction: {\"signals\":[{\"name\":\"forged\"}],\"annotations\":[]}"
        );
        fs::write(&artifact, serde_json::to_string_pretty(&json)?)?;
        let error = SourceIr::load_from_path(&artifact)
            .expect_err("edited conclusion must stale its registered proof");
        assert!(error.to_string().contains("proof verification failed"));
        Ok(())
    }

    #[test]
    fn hash_consistent_field_edit_cannot_self_attest_without_satisfying_rule() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("self-attestation.md");
        let artifact_base = tempdir.path().join("artifacts");
        fs::write(&source, "# self attestation\n")?;
        let source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.write_to_disk()?;
        let artifact = source_ir.artifact_layout.source_ir_path.clone();

        let mut json = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&artifact)?)?;
        json["automation_confidence"] = serde_json::json!("low");
        let forged_digest = super::Sha256Digest::of_bytes(
            &serde_json::to_vec(&json["automation_confidence"]).expect("encode forged conclusion"),
        );
        let root_claim = json["proof_ledger"]["claims"]
            .as_array_mut()
            .expect("proof claims")
            .iter_mut()
            .find(|claim| {
                claim["address"]["surface"] == "automation_confidence"
                    && claim["address"]["field_path"].is_null()
            })
            .expect("automation root claim");
        root_claim["conclusion_sha256"] = serde_json::json!(forged_digest.as_str());
        fs::write(&artifact, serde_json::to_string_pretty(&json)?)?;

        let error = SourceIr::load_from_path(&artifact)
            .expect_err("a recomputed conclusion hash cannot replace executable verification");
        assert!(
            error
                .to_string()
                .contains("differs from its exact captured premise"),
            "unexpected verification failure: {error}"
        );
        Ok(())
    }

    #[test]
    fn current_schema_proofless_source_is_rejected_and_deterministic_validation_round_trips()
    -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("validation.md");
        let artifact_base = tempdir.path().join("artifacts");
        fs::write(&source, "# validation\n")?;
        let mut source_ir = SourceIr::build(&source, &artifact_base)?;
        let report = source_ir.validation_report()?;
        source_ir.apply_validation_report(report.clone())?;
        source_ir.write_to_disk()?;
        let artifact = source_ir.artifact_layout.source_ir_path.clone();
        let reloaded = SourceIr::load_from_path(&artifact)?;
        assert_eq!(reloaded.validation_reports, vec![report]);

        let mut json = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&artifact)?)?;
        json.as_object_mut()
            .expect("SourceIR object")
            .remove("proof_context");
        json.as_object_mut()
            .expect("SourceIR object")
            .remove("proof_ledger");
        fs::write(&artifact, serde_json::to_string_pretty(&json)?)?;
        let error = SourceIr::load_from_path(&artifact)
            .expect_err("current proofless SourceIR must not feed canonical consumers");
        assert!(error.to_string().contains("proofless"));
        Ok(())
    }

    #[cfg(feature = "source-proof-migration")]
    #[test]
    fn retained_capture_migration_rebuilds_legacy_only_after_backing_verification() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("legacy.md");
        let artifact_base = tempdir.path().join("artifacts");
        fs::write(&source, "# legacy\n")?;
        let source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.write_to_disk()?;
        let artifact = source_ir.artifact_layout.source_ir_path.clone();

        let mut legacy =
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&artifact)?)?;
        legacy["schema_version"] = serde_json::json!(1);
        legacy
            .as_object_mut()
            .expect("legacy object")
            .remove("proof_context");
        legacy
            .as_object_mut()
            .expect("legacy object")
            .remove("proof_ledger");
        fs::write(&artifact, serde_json::to_string_pretty(&legacy)?)?;

        let rebuilt = SourceIr::rebuild_legacy_from_retained_capture(&artifact)?;
        assert_eq!(rebuilt.schema_version, super::SOURCE_IR_SCHEMA_VERSION);
        rebuilt.write_to_disk()?;
        SourceIr::load_from_path(&artifact)?;

        legacy["source"]["size_bytes"] = serde_json::json!(999_999);
        fs::write(&artifact, serde_json::to_string_pretty(&legacy)?)?;
        let error = SourceIr::rebuild_legacy_from_retained_capture(&artifact)
            .expect_err("changed source backing must reject migration");
        assert!(error.to_string().contains("source-size mismatch"));
        Ok(())
    }

    #[cfg(feature = "source-proof-migration")]
    #[test]
    fn retained_capture_migration_refreshes_a_stale_current_ruleset_without_field_drift()
    -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("current.md");
        let artifact_base = tempdir.path().join("artifacts");
        fs::write(&source, "# current\nSignal ALPHA is input width 1.\n")?;
        let source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.write_to_disk()?;
        let artifact = source_ir.artifact_layout.source_ir_path.clone();

        let mut stale = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&artifact)?)?;
        let expected_fields = {
            let mut fields = stale.clone();
            fields
                .as_object_mut()
                .expect("SourceIR object")
                .remove("proof_context");
            fields
                .as_object_mut()
                .expect("SourceIR object")
                .remove("proof_ledger");
            fields
        };
        stale["proof_ledger"]["ruleset_sha256"] = serde_json::Value::String("0".repeat(64));
        fs::write(&artifact, serde_json::to_string_pretty(&stale)?)?;
        assert!(SourceIr::load_from_path(&artifact).is_err());

        let rebuilt = SourceIr::rebuild_from_retained_capture(&artifact)?;
        let mut rebuilt_fields = serde_json::to_value(&rebuilt)?;
        rebuilt_fields
            .as_object_mut()
            .expect("SourceIR object")
            .remove("proof_context");
        rebuilt_fields
            .as_object_mut()
            .expect("SourceIR object")
            .remove("proof_ledger");
        assert_eq!(rebuilt_fields, expected_fields);
        rebuilt.write_to_disk()?;
        SourceIr::load_from_path(&artifact)?;
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
        assert!(source_ir_json.contains("\"path_origin\": \"repository_owned\""));
        assert!(
            !source_ir_json.contains(
                crate::project_data::repository_root()?
                    .to_string_lossy()
                    .as_ref()
            ),
            "repository-owned SourceIR paths must serialize without the runtime root"
        );

        Ok(())
    }

    #[test]
    fn source_ir_load_rebases_unlabeled_legacy_repository_source() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("legacy.md");
        let artifact_base = tempdir.path().join("generated/source_ir");
        fs::write(&source, "# legacy\n")?;

        let source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.write_to_disk()?;
        let source_ir_path = artifact_base.join("legacy/source_ir.json");
        let mut json =
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&source_ir_path)?)?;
        let repository = crate::project_data::repository_root()?;
        let relative_source = source_ir
            .source
            .canonical_path
            .strip_prefix(&repository)
            .expect("test source resides below the repository")
            .to_path_buf();
        let retired_source = Path::new("/retired/specforge").join(&relative_source);
        assert!(retired_source.starts_with("/retired/specforge"));
        json["source"]["requested_path"] =
            serde_json::Value::String(retired_source.to_string_lossy().into_owned());
        json["source"]["canonical_path"] =
            serde_json::Value::String(retired_source.to_string_lossy().into_owned());
        json["source"]
            .as_object_mut()
            .expect("source object")
            .remove("path_origin");
        fs::write(&source_ir_path, serde_json::to_string_pretty(&json)?)?;

        json["schema_version"] = serde_json::json!(2);
        json.as_object_mut()
            .expect("SourceIR object")
            .remove("proof_context");
        json.as_object_mut()
            .expect("SourceIR object")
            .remove("proof_ledger");
        fs::write(&source_ir_path, serde_json::to_string_pretty(&json)?)?;

        let reloaded = SourceIr::load_for_inspection(&source_ir.artifact_layout.source_ir_path)?;
        assert_eq!(reloaded.source.canonical_path, source.canonicalize()?);
        assert_eq!(
            reloaded.source.path_origin,
            Some(crate::persisted_path::PersistedPathOrigin::RepositoryOwned)
        );
        Ok(())
    }

    #[test]
    fn source_ir_load_keeps_intentionally_reclaimed_repository_provenance() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("reclaimed.md");
        let artifact_base = tempdir.path().join("generated/source_ir");
        fs::write(&source, "# Reclaimed provenance\n")?;
        let source_path = source.canonicalize()?;

        let source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.write_to_disk()?;
        fs::remove_file(&source)?;

        let reloaded = SourceIr::load_from_path(&source_ir.artifact_layout.source_ir_path)?;
        assert_eq!(reloaded.source.canonical_path, source_path);
        assert!(!reloaded.source.canonical_path.exists());
        assert!(
            !reloaded.to_pretty_json()?.contains(
                crate::project_data::repository_root()?
                    .to_string_lossy()
                    .as_ref()
            )
        );
        Ok(())
    }

    #[test]
    fn source_ir_load_neutralizes_legacy_classifier_authority() -> Result<()> {
        let tempdir = crate::project_data::tempdir()?;
        let source = tempdir.path().join("legacy-classifier.md");
        let artifact_base = tempdir.path().join("generated/source_ir");
        fs::write(&source, "# Legacy classifier\n")?;

        let source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.write_to_disk()?;
        let source_ir_path = artifact_base.join("legacy_classifier/source_ir.json");
        let mut json =
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&source_ir_path)?)?;
        json["schema_version"] = serde_json::json!(1);
        json["visual_assets"] = serde_json::json!([{
            "asset_id": "picture_0001",
            "asset_kind": "figure",
            "page_id": null,
            "image_path": null,
            "caption_text": "Operation transaction",
            "caption_source_path": null,
            "source_ref": null,
            "placeholder_text": null,
            "note": null,
            "diagram_kind": "timing_diagram"
        }]);
        json["structured_tables"] = serde_json::json!([{
            "table_id": "table_0001",
            "asset_id": "table_0001",
            "page_id": null,
            "caption_text": "Operation fields",
            "source_ref": null,
            "table_kind": "encoding",
            "header_rows": [],
            "body_rows": [],
            "row_count": 0,
            "col_count": 0
        }]);
        json["document_sections"] = serde_json::json!([{
            "section_id": "section_0001",
            "title": "Participant interface",
            "heading_level": 1,
            "page_id": null,
            "source_ref": null,
            "reading_order": 1,
            "section_kind": "signal_description"
        }]);
        fs::write(&source_ir_path, serde_json::to_string_pretty(&json)?)?;

        let canonical_error = SourceIr::load_from_path(&source_ir_path)
            .expect_err("legacy proofless SourceIR must not gain canonical authority");
        assert!(canonical_error.to_string().contains("inspection-only"));
        let reloaded = SourceIr::load_for_inspection(&source_ir_path)?;
        assert_eq!(reloaded.schema_version, 1);
        assert_eq!(reloaded.visual_assets[0].diagram_kind, DiagramKind::Unknown);
        assert_eq!(reloaded.structured_tables[0].table_kind, TableKind::Unknown);
        assert_eq!(
            reloaded.document_sections[0].section_kind,
            SectionKind::Unknown
        );
        assert!(
            reloaded
                .normalization_plan
                .notes
                .iter()
                .any(|note| note.contains("semantic source classifications were neutralized"))
        );
        Ok(())
    }

    #[test]
    fn source_ir_load_rejects_a_future_schema() -> Result<()> {
        let tempdir = crate::project_data::tempdir()?;
        let source = tempdir.path().join("future-schema.md");
        let artifact_base = tempdir.path().join("generated/source_ir");
        fs::write(&source, "# Future schema\n")?;

        let source_ir = SourceIr::build(&source, &artifact_base)?;
        source_ir.write_to_disk()?;
        let mut json = serde_json::from_str::<serde_json::Value>(&fs::read_to_string(
            &source_ir.artifact_layout.source_ir_path,
        )?)?;
        json["schema_version"] = serde_json::json!(super::SOURCE_IR_SCHEMA_VERSION + 1);
        fs::write(
            &source_ir.artifact_layout.source_ir_path,
            serde_json::to_string_pretty(&json)?,
        )?;

        let error = SourceIr::load_from_path(&source_ir.artifact_layout.source_ir_path)
            .expect_err("future SourceIR must be rejected");
        assert!(error.to_string().contains("newer than supported schema"));
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
printf '{"backend":"docling_stub","batch_threshold_pages":"%s","batch_pages":"%s"}\n' \
  "$SPECFORGE_INGEST_BATCH_THRESHOLD" "$SPECFORGE_INGEST_BATCH_PAGES" > "$metadata_output"
printf 'stub-page' > "$page_image_root/page-0001.png"
printf '{"page_number":1,"rendered_image":{"path":"%s"}}\n' "$page_image_root/page-0001.png" > "$page_image_root/page-0001.json"
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
        // Likewise disable the disk pre-flight so a CI host that happens to be low on disk cannot
        // false-abort the stub-helper ingest (MEMORY-BOUNDED-INGEST.4b).
        let _disk_guard =
            EnvVarGuard::set_path("SPECFORGE_INGEST_MIN_FREE_DISK_MB", Path::new("off"));
        // Pin adaptive batch sizing off so the child's batch-pages env is a fixed value regardless of
        // the CI host's total RAM (MEMORY-BOUNDED-INGEST.4c); the stub helper ignores it either way.
        let _batch_guard =
            EnvVarGuard::set_path("SPECFORGE_INGEST_ADAPTIVE_BATCH", Path::new("off"));
        let _batch_threshold =
            EnvVarGuard::set_path("SPECFORGE_INGEST_BATCH_THRESHOLD", Path::new("123"));
        let _batch_pages = EnvVarGuard::set_path("SPECFORGE_INGEST_BATCH_PAGES", Path::new("17"));
        let mut source_ir = SourceIr::build(&source, &artifact_base)?;

        source_ir.materialize()?;
        source_ir.write_to_disk()?;

        assert_eq!(
            source_ir.normalization_plan.status,
            super::NormalizationStatus::Ready
        );
        assert_eq!(source_ir.page_artifacts.len(), 1);
        assert_eq!(source_ir.visual_assets.len(), 1);
        let backend_metadata: serde_json::Value = serde_json::from_slice(&fs::read(
            source_ir
                .normalization_plan
                .metadata_output_path
                .as_ref()
                .expect("PDF metadata path"),
        )?)?;
        assert_eq!(backend_metadata["batch_threshold_pages"], "123");
        assert_eq!(backend_metadata["batch_pages"], "17");
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
        let page_sidecar = fs::read_to_string(
            artifact_base
                .join("bus_spec")
                .join("normalized")
                .join("pages")
                .join("page-0001.json"),
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
        assert!(page_sidecar.contains("normalized/pages/page-0001.png"));
        assert!(!page_sidecar.contains("normalized.staging"));
        assert!(visual_manifest.contains("\"source_ref\": \"#/pictures/0\""));
        assert!(promoted_markdown.contains("![Image](assets/picture-0001.png)"));
        let repository = crate::project_data::repository_root()?;
        for persisted_artifact in [
            &source_ir_json,
            &page_manifest,
            &page_sidecar,
            &visual_manifest,
        ] {
            assert!(
                !persisted_artifact.contains(repository.to_string_lossy().as_ref()),
                "repository-owned normalization and visual paths must serialize without the runtime root"
            );
        }
        let reloaded = SourceIr::load_from_path(&source_ir.artifact_layout.source_ir_path)?;
        assert!(
            reloaded.page_artifacts[0]
                .page_image_path
                .as_ref()
                .is_some_and(|path| path.is_absolute())
        );
        assert!(
            reloaded.visual_assets[0]
                .image_path
                .as_ref()
                .is_some_and(|path| path.is_absolute())
        );

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
printf '{"page_number":1,"rendered_image":{"path":"%s"}}\n' "$page_image_root/page-0001.png" > "$page_image_root/page-0001.json"
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
        // Likewise disable the disk pre-flight so a CI host that happens to be low on disk cannot
        // false-abort the stub-helper ingest (MEMORY-BOUNDED-INGEST.4b).
        let _disk_guard =
            EnvVarGuard::set_path("SPECFORGE_INGEST_MIN_FREE_DISK_MB", Path::new("off"));
        // Pin adaptive batch sizing off so the child's batch-pages env is a fixed value regardless of
        // the CI host's total RAM (MEMORY-BOUNDED-INGEST.4c); the stub helper ignores it either way.
        let _batch_guard =
            EnvVarGuard::set_path("SPECFORGE_INGEST_ADAPTIVE_BATCH", Path::new("off"));
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

        #[cfg(unix)]
        {
            fs::write(
                &helper,
                r##"#!/bin/sh
kill -9 $$
"##,
            )?;
            let mut signal_source_ir = SourceIr::build(&source, &artifact_base)?;
            let signal_error = signal_source_ir
                .materialize()
                .expect_err("signal termination should fail with a typed diagnostic");
            match &signal_error {
                AppError::IngestTerminatedBySignal { signal, .. } => assert_eq!(*signal, 9),
                other => panic!("unexpected signal error: {other:?}"),
            }
            assert!(
                signal_error
                    .to_string()
                    .contains("does not prove an out-of-memory event")
            );
            assert!(stale_root.join("bus_spec.md").exists());
            assert!(
                !artifact_base
                    .join("bus_spec")
                    .join("normalized.staging")
                    .exists()
            );
        }

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
        // Likewise disable the disk pre-flight so a CI host that happens to be low on disk cannot
        // false-abort the stub-helper ingest (MEMORY-BOUNDED-INGEST.4b).
        let _disk_guard =
            EnvVarGuard::set_path("SPECFORGE_INGEST_MIN_FREE_DISK_MB", Path::new("off"));
        // Pin adaptive batch sizing off so the child's batch-pages env is a fixed value regardless of
        // the CI host's total RAM (MEMORY-BOUNDED-INGEST.4c); the stub helper ignores it either way.
        let _batch_guard =
            EnvVarGuard::set_path("SPECFORGE_INGEST_ADAPTIVE_BATCH", Path::new("off"));
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
