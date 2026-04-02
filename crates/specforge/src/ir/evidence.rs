use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::source::{
    AutomationConfidence, NormalizationStatus, SectionKind, SourceIr, TableKind, VisualAsset,
    VisualAssetKind, document_key,
};
use crate::ir::source::{
    ConditionalRuleRecord, RegisterFieldRecord, RegisterRecord, SignalConstraintKind,
    SignalConstraintRecord, TimingConstraintRecord,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StatementClass {
    SourceFact,
    /// Sentence explicitly constraining a hardware signal to a specific logic value or
    /// protocol state. These are the most precise and directly actionable constraints.
    /// Examples: "HTRANS must be IDLE when HREADY is LOW",
    ///           "HWRITE shall remain HIGH throughout the burst",
    ///           "HREADYOUT must be asserted when transfer is accepted".
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
}

impl EvidenceIr {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(AppError::MissingPath(path.to_path_buf()));
        }

        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }
    pub fn build(source_ir_path: &Path, artifact_base_root: &Path) -> Result<Self> {
        let source_ir_path = canonicalize_existing_path(source_ir_path)?;
        let source_ir = SourceIr::load_from_path(&source_ir_path)?;

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

        let mut visual_evidence = build_visual_evidence_items(&source_ir.visual_assets);
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

        let mut span_counter = 1usize;
        let mut link_counter = 1usize;
        let mut statement_counter = 1usize;

        for block in &parsed_markdown.blocks {
            let span_id = format!("span_{span_counter:04}");
            span_counter += 1;

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

            if let Some(section_index) = section_index {
                if let Some(source_page) = source_page {
                    section_pages[section_index].push(source_page);
                }
            }

            let related_visual_evidence_ids: Vec<String> = linked_asset_ids
                .iter()
                .filter_map(|asset_id| asset_id_to_visual_evidence_id.get(asset_id).cloned())
                .collect();
            extracted_statements.push(ExtractedStatement {
                statement_id: format!("statement_{statement_counter:04}"),
                class: classify_statement(&block.text),
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
        // This is the correct layer for this extraction: SourceIR captured the cell grids;
        // EvidenceIR produces the typed evidence; SemanticIR lifts without re-parsing.
        let synthesized = synthesize_declarations_from_tables(&source_ir, &mut statement_counter);
        extracted_statements.extend(synthesized);

        // Synthesize typed register and timing records from structured tables.
        let register_records = synthesize_register_records(&source_ir);
        let timing_constraints = synthesize_timing_constraints(&source_ir);

        // Level 2 NLP: extract structured records from already-classified normative sentences.
        // These operate on classified statement text, not raw text, so precision is high.
        let mut constraint_counter = 1usize;
        let signal_constraints =
            extract_signal_constraints(&extracted_statements, &mut constraint_counter);
        let conditional_rules =
            extract_conditional_rules(&extracted_statements, &mut constraint_counter);

        Ok(Self {
            schema_version: 1,
            stage: IrStage::EvidenceIr,
            source_ir_path,
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
        })
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn write_to_disk(&self) -> Result<()> {
        fs::create_dir_all(&self.artifact_layout.artifact_root)?;
        fs::write(
            &self.artifact_layout.evidence_ir_path,
            self.to_pretty_json()?,
        )?;
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

fn build_visual_evidence_items(visual_assets: &[VisualAsset]) -> Vec<VisualEvidenceItem> {
    visual_assets
        .iter()
        .enumerate()
        .map(|(index, asset)| VisualEvidenceItem {
            evidence_id: format!("visual_{:04}", index + 1),
            asset_id: asset.asset_id.clone(),
            asset_kind: asset.asset_kind,
            role: infer_visual_role(asset.asset_kind, asset.caption_text.as_deref()),
            source_path: asset
                .image_path
                .clone()
                .or_else(|| asset.caption_source_path.clone()),
            source_page: asset.page_id.as_deref().and_then(page_number_from_page_id),
            caption_text: asset.caption_text.clone(),
            figure_reference_text: None,
            observations: Vec::new(),
            automation_confidence: if asset.caption_text.is_some() {
                AutomationConfidence::High
            } else {
                AutomationConfidence::Medium
            },
        })
        .collect()
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
) -> VisualEvidenceRole {
    let lowered_caption = caption_text.map(|text| text.to_ascii_lowercase());

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

/// Synthesizes formal typed declarations from the structured table data captured in `SourceIR`.
///
/// This is the correct architectural layer for table-to-declaration conversion:
/// `SourceIR` holds the raw cell grids; `EvidenceIR` produces typed statements;
/// `SemanticIR` lifts from those statements using its existing parsers.
///
/// Currently handles two table kinds:
/// - `SignalDescription` → `Signal X is output/input [width N].` declarations
/// - `Encoding` → `Enum <name> <member> = <value>.` declarations
fn synthesize_declarations_from_tables(
    source_ir: &SourceIr,
    statement_counter: &mut usize,
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

        match table.table_kind {
            TableKind::SignalDescription => {
                statements.extend(synthesize_signal_declarations(
                    table,
                    section_kind,
                    &section_title,
                    statement_counter,
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

/// Infer signal direction from a section kind + title for signal description tables.
fn infer_signal_direction_from_section(kind: SectionKind, title: &str) -> Option<&'static str> {
    // Explicit section kind takes priority.
    match kind {
        SectionKind::SignalDescription => {}
        SectionKind::Boilerplate | SectionKind::Glossary | SectionKind::TableOfContents => {
            return None;
        }
        _ => {}
    }
    let lowered = title.to_ascii_lowercase();
    if lowered.contains("manager") || lowered.contains("initiator") || lowered.contains("master") {
        return Some("output");
    }
    if lowered.contains("subordinate")
        || lowered.contains("slave")
        || lowered.contains("responder")
        || lowered.contains("multiplexor")
    {
        return Some("input");
    }
    if lowered.contains("global")
        || lowered.contains("system")
        || lowered.contains("clock")
        || lowered.contains("decoder")
    {
        return Some("input");
    }
    None
}

/// Returns true if the token looks like a hardware signal name:
/// all-uppercase with optional digits and underscores, at least 2 chars.
fn is_hardware_signal_token(token: &str) -> bool {
    token.len() >= 2
        && token
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
        && token.chars().any(|c| c.is_ascii_uppercase())
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

    for statement in statements {
        if !matches!(statement.class, StatementClass::SignalValueConstraint) {
            continue;
        }
        let text = &statement.text;
        let lowered = text.to_ascii_lowercase();

        // Strip the condition clause so signal names in "when X" / "during X" / "unless X"
        // are not mistaken for subjects of the constraint.
        let subject_part = text_before_condition_marker(text);

        // Collect ALL valid signal tokens from the subject part, creating one record each.
        // Fall back to scanning the full text if no signals found in the subject part.
        let mut subject_signals = collect_subject_signal_tokens(subject_part);
        if subject_signals.is_empty() {
            subject_signals = collect_subject_signal_tokens(text);
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

        // Extract condition clause: text after "when", "while", "during", "unless".
        let condition_text = extract_condition_clause(text);

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
fn text_before_condition_marker(text: &str) -> &str {
    let lowered_bytes = text.to_ascii_lowercase();
    for marker in &[
        " when ",
        " while ",
        " during ",
        " unless ",
        " provided ",
        " after ",
        " before ",
    ] {
        if let Some(pos) = lowered_bytes.find(marker) {
            return &text[..pos];
        }
    }
    text
}

/// Collect all uppercase hardware signal tokens from a text fragment.
/// Excludes logic-level values (HIGH/LOW), protocol state names (NONSEQ/SEQ/...),
/// protocol family names (AHB/AXI/...), and document structure words.
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
                && !matches!(
                    *tok,
                    // Logic levels and protocol state values are never signal subjects.
                    "HIGH" | "LOW" | "IDLE" | "BUSY" | "NONSEQ" | "SEQ" | "OKAY" | "ERROR"
                        | "VALID" | "INVALID" | "NONE" | "ALL" | "ANY" | "BOTH"
                        | "SINGLE" | "INCR" | "WRAP" | "OKAY" | "RETRY" | "SPLIT"
                        | "BYTE" | "HALF" | "WORD"
                        // Protocol family and company names
                        | "AMBA" | "AHB" | "AHB5" | "APB" | "AXI" | "CHI" | "ARM" | "AMD"
                        | "RISC" | "IP" | "SoC"
                        // Document structure terms
                        | "NOTE" | "TABLE" | "FIGURE" | "CHAPTER" | "SECTION" | "REF"
                        // Role/component terms that appear uppercase in signal tables
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
                    && !matches!(
                        *tok,
                        "HIGH" | "LOW" | "IDLE" | "BUSY" | "NONSEQ" | "SEQ" | "OKAY" | "ERROR"
                    )
            })
            .next()
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
fn extract_protocol_state_value(lowered: &str) -> Option<String> {
    for state in &[
        // HTRANS encoding values
        "idle",
        "busy",
        "nonseq",
        "nonsequential",
        "seq",
        "sequential",
        // HRESP values
        "okay",
        "error",
        "retry",
        "split",
        // HBURST values
        "single",
        "incr",
        "incr4",
        "incr8",
        "incr16",
        "wrap4",
        "wrap8",
        "wrap16",
        // HSIZE values
        "byte",
        "halfword",
        "word",
        // Generic
        "valid",
        "invalid",
        "exclusive",
    ] {
        if contains_any(
            lowered,
            &[
                &format!("must be {state}"),
                &format!("shall be {state}"),
                &format!("must remain {state}"),
                &format!("shall remain {state}"),
            ],
        ) {
            return Some(state.to_ascii_uppercase());
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
    for phrase in &[
        "must not change",
        "shall not change",
        "must remain",
        "shall remain",
        "must be idle",
        "shall be idle",
        "must be nonseq",
        "shall be nonseq",
        "must be seq",
        "shall be seq",
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
            return phrase.to_string();
        }
    }
    "(see source_text)".to_string()
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

    // Step 1: Check for a value-binding phrase.
    // These phrases all indicate a signal is constrained to a specific logic level,
    // stable state, or protocol encoding value.
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
            // Protocol states (HTRANS, HBURST, HRESP, HSIZE encoding values)
            "must be idle",
            "shall be idle",
            "must be nonseq",
            "shall be nonseq",
            "must be seq",
            "shall be seq",
            "must be busy",
            "shall be busy",
            "must be okay",
            "shall be okay",
            "must be error",
            "shall be error",
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
fn is_signal_synthesis_non_signal(token: &str) -> bool {
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
    )
}

fn synthesize_signal_declarations(
    table: &crate::ir::source::StructuredTableRecord,
    section_kind: SectionKind,
    section_title: &str,
    statement_counter: &mut usize,
) -> Vec<ExtractedStatement> {
    let mut statements = Vec::new();
    if table.body_rows.is_empty() || table.col_count < 2 {
        return statements;
    }

    // Find width column index from header rows.
    let header_texts: Vec<String> = table
        .header_rows
        .first()
        .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
        .unwrap_or_default();
    let width_col = header_texts
        .iter()
        .position(|h| h.contains("width") || h.contains("bits") || h.contains("size"));
    let dir_col = header_texts
        .iter()
        .position(|h| h.contains("direction") || h.contains("source") || h.contains("destination"));

    let default_dir = infer_signal_direction_from_section(section_kind, section_title);

    for row in &table.body_rows {
        let Some(name_cell) = row.first() else {
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

        // Determine direction from an explicit column or fall back to section context.
        let direction = dir_col
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
            .or(default_dir);

        // Parse numeric width; skip parametric widths like ADDR_WIDTH.
        let width: Option<u32> = width_col.and_then(|col| {
            row.get(col)
                .and_then(|cell| cell.text.trim().parse::<u32>().ok())
                .filter(|&w| w > 0 && w <= 1024)
        });

        let text = match (direction, width) {
            (Some(dir), Some(w)) => format!("Signal {token} is {dir} width {w}."),
            (Some(dir), None) => format!("Signal {token} is {dir}."),
            _ => continue,
        };

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

fn synthesize_encoding_declarations(
    table: &crate::ir::source::StructuredTableRecord,
    section_title: &str,
    statement_counter: &mut usize,
) -> Vec<ExtractedStatement> {
    let mut statements = Vec::new();
    if table.body_rows.is_empty() {
        return statements;
    }

    // Derive enum name from caption or section title.
    // e.g. "HTRANS encoding" → "HTRANS", "Table 5-3 HBURST values" → "HBURST"
    let enum_name_source = table.caption_text.as_deref().unwrap_or(section_title);
    // Extract the first all-uppercase token that looks like a signal name.
    let enum_name: Option<String> = enum_name_source
        .split_whitespace()
        .find(|tok| is_hardware_signal_token(&tok.to_ascii_uppercase()))
        .map(|tok| tok.to_ascii_uppercase());
    let Some(enum_name) = enum_name else {
        return statements;
    };

    // Find header column indices.
    let header_texts: Vec<String> = table
        .header_rows
        .first()
        .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
        .unwrap_or_default();
    // Name/meaning column: the column that names each encoding value.
    let name_col = header_texts
        .iter()
        .position(|h| {
            h.contains("name")
                || h.contains("meaning")
                || h.contains("transfer")
                || h.contains("type")
                || h.contains("description")
        })
        .unwrap_or(0);
    // Value column: binary/hex encoding value.
    let value_col = header_texts
        .iter()
        .position(|h| {
            h.contains("value")
                || h.contains("encoding")
                || h.contains("code")
                || h.contains("binary")
                || h.contains("hex")
        })
        .unwrap_or(1);

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
            .and_then(|cell| {
                let t = cell.text.trim();
                // Try direct integer, then strip binary prefix like 2'b00 or 0b00.
                t.parse::<u32>().ok().or_else(|| {
                    let stripped = t
                        .trim_start_matches(|c: char| c.is_ascii_digit())
                        .trim_start_matches("'b")
                        .trim_start_matches("'h");
                    u32::from_str_radix(stripped, 2)
                        .ok()
                        .or_else(|| u32::from_str_radix(stripped, 16).ok())
                })
            })
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

/// Synthesize `RegisterRecord` entries from `register_map` tables captured in `SourceIR`.
/// Each table row becomes either a register-level record or, if the table has bit-field
/// columns, a field within the preceding register.
fn synthesize_register_records(source_ir: &SourceIr) -> Vec<RegisterRecord> {
    let mut records: Vec<RegisterRecord> = Vec::new();
    let register_tables: Vec<_> = source_ir
        .structured_tables
        .iter()
        .filter(|t| matches!(t.table_kind, TableKind::RegisterMap))
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

        let name_col = header
            .iter()
            .position(|h| h.contains("name") || h.contains("register") || h.contains("field"))
            .unwrap_or(0);
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
                let field = RegisterFieldRecord {
                    field_name: name.clone(),
                    bits_high,
                    bits_low,
                    access_type: access,
                    reset_value: reset,
                    description: desc,
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
                fields: Vec::new(),
                supporting_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    records
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

/// Synthesize `TimingConstraintRecord` entries from `timing_parameter` tables in `SourceIR`.
fn synthesize_timing_constraints(source_ir: &SourceIr) -> Vec<TimingConstraintRecord> {
    let mut records: Vec<TimingConstraintRecord> = Vec::new();
    let timing_tables: Vec<_> = source_ir
        .structured_tables
        .iter()
        .filter(|t| matches!(t.table_kind, TableKind::TimingParameter))
        .collect();

    for table in timing_tables {
        if table.body_rows.is_empty() {
            continue;
        }

        let header: Vec<String> = table
            .header_rows
            .first()
            .map(|r| r.iter().map(|c| c.text.to_ascii_lowercase()).collect())
            .unwrap_or_default();

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
        for (row_idx, row) in table.body_rows.iter().enumerate() {
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
    visual_evidence: &mut Vec<VisualEvidenceItem>,
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

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::source::{SourceIr, VisualAsset, VisualAssetKind};

    use super::{EvidenceIr, EvidenceLinkKind, StatementClass, VisualObservationKind};

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
            }),
            "expected TimingDiagramExtraction observation from VLM note"
        );
        // The figure's role should be upgraded to Normative.
        assert_eq!(
            evidence_ir.visual_evidence[0].role,
            super::VisualEvidenceRole::Normative
        );

        Ok(())
    }

    /// Unit tests for Level 1 and Level 2 NLP classification and extraction.
    /// These lock in the expanded vocabulary so regressions are caught immediately.
    mod nlp_classification {
        use super::super::{
            StatementClass, classify_statement, collect_subject_signal_tokens,
            is_signal_value_constraint, text_before_condition_marker,
        };
        use crate::ir::source::SignalConstraintKind;

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
        fn collect_subject_signal_tokens_excludes_logic_level_values() {
            // HIGH, LOW, IDLE etc. must never be treated as subject signals.
            let signals = collect_subject_signal_tokens("HTRANS must be IDLE");
            assert!(signals.contains(&"HTRANS".to_string()));
            assert!(
                !signals.contains(&"IDLE".to_string()),
                "IDLE is a value, not a signal"
            );
            assert!(
                !signals.contains(&"HIGH".to_string()),
                "HIGH is a logic level, not a signal"
            );
        }
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
                })
        );
        assert!(
            evidence_ir.visual_evidence[0]
                .observations
                .iter()
                .any(|observation| {
                    observation.kind == VisualObservationKind::FigureReference
                        && observation.text == "Figure 1"
                })
        );
        assert!(evidence_ir.extracted_statements.iter().any(|statement| {
            !statement.related_visual_evidence_ids.is_empty()
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
}
