use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::source::{
    AutomationConfidence, NormalizationStatus, SourceIr, VisualAsset, VisualAssetKind, document_key,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StatementClass {
    SourceFact,
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
}

impl EvidenceIr {
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
        assert_eq!(
            evidence_ir.extracted_statements[0].class,
            StatementClass::SourceFact
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
