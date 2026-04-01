use std::path::PathBuf;

use serde::Serialize;

use crate::ir::IrStage;
use crate::ir::source::{AutomationConfidence, VisualAssetKind};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StatementClass {
    SourceFact,
    DerivedRule,
    LocalDesignDecision,
    ExplicitAbstraction,
    Unknown,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceModality {
    Text,
    Visual,
    Mixed,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VisualEvidenceRole {
    Normative,
    Explanatory,
    Illustrative,
    Ambiguous,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
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
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLinkKind {
    Cites,
    Describes,
    DerivedFrom,
    Supports,
    Refines,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EvidenceIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub source_ir_path: PathBuf,
    pub section_anchors: Vec<SectionAnchor>,
    pub evidence_spans: Vec<EvidenceSpan>,
    pub visual_evidence: Vec<VisualEvidenceItem>,
    pub evidence_links: Vec<EvidenceLink>,
    pub extracted_statements: Vec<ExtractedStatement>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SectionAnchor {
    pub section_id: String,
    pub title: String,
    pub source_path: PathBuf,
    pub page_start: Option<u32>,
    pub page_end: Option<u32>,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct VisualObservation {
    pub observation_id: String,
    pub kind: VisualObservationKind,
    pub created_by: String,
    pub text: String,
    pub supporting_span_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EvidenceLink {
    pub link_id: String,
    pub from_evidence_span_id: String,
    pub to_visual_evidence_id: String,
    pub relation: EvidenceLinkKind,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ExtractedStatement {
    pub statement_id: String,
    pub class: StatementClass,
    pub modality: EvidenceModality,
    pub text: String,
    pub evidence_span_ids: Vec<String>,
    pub related_visual_evidence_ids: Vec<String>,
}
