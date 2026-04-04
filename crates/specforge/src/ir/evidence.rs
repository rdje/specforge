use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::source::{
    ActorSignalRelation, ConditionalRuleRecord, RegisterFieldRecord, RegisterRecord, RelationKind,
    SignalConstraintKind, SignalConstraintRecord, TimingConstraintRecord, ValidationReportRecord,
    WidthHint,
};
use crate::ir::source::{
    AutomationConfidence, NormalizationStatus, SectionKind, SourceIr, TableKind, VisualAsset,
    VisualAssetKind, document_key,
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
        let synthesized = synthesize_declarations_from_tables(&source_ir, &mut statement_counter);

        // Extract system contract (clock + reset) from signal-description prose in tables.
        let contract_stmts =
            synthesize_system_contract_from_table_descriptions(&source_ir, &mut statement_counter);

        // Synthesize typed register and timing records from structured tables.
        let register_records = synthesize_register_records(&source_ir);
        let timing_constraints = synthesize_timing_constraints(&source_ir);

        // Replace the previous one-shot extraction with a monotone convergent loop:
        // discovered signals unlock anchored encoding tables, which unlock new value atoms,
        // which unlock additional prose-derived constraints.
        let (
            extracted_statements,
            signal_constraints,
            conditional_rules,
            signal_polarity_conflicts,
            actor_signal_relations,
        ) = converge_evidence_extractions(
            &source_ir,
            extracted_statements,
            synthesized,
            contract_stmts,
            &mut statement_counter,
        );

        let mut evidence_ir = Self {
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
            signal_polarity_conflicts,
            signal_semantic_hints: Vec::new(),
            signal_semantic_conflicts: Vec::new(),
            actor_signal_relations,
            signal_alias_map: BTreeMap::new(),
            validation_reports: Vec::new(),
        };
        evidence_ir.carry_forward_existing_knowledge()?;
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
        let (signal_semantic_hints, signal_semantic_conflicts) = synthesize_signal_semantic_hints(
            &source_ir,
            &self.extracted_statements,
            &self.signal_alias_map,
            &self.visual_evidence,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

// ─────────────────────────────────────────────────────────────────────────────
// Tier 2 Knowledge Graph: actor–signal relation extraction
// ─────────────────────────────────────────────────────────────────────────────

/// Collect all hardware signal names that have been formally declared via
/// synthesized `Signal X is input/output` statements.  These come from signal
/// description tables (High confidence) and are the known universe of signals
/// we should look for in prose.
fn collect_known_signal_names(
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

/// Collect hardware signal names from ALL signal-description table rows (first column),
/// regardless of whether direction could be determined.  This covers specs like APB and AXI
/// where the Source/Direction column uses non-standard values ("Requester", "Completer")
/// or is absent entirely.
fn collect_signal_names_from_tables(source_ir: &SourceIr) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    for table in &source_ir.structured_tables {
        if !matches!(table.table_kind, TableKind::SignalDescription) {
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

/// Extract actor–signal relation triples directly from signal-description table structure.
///
/// The Source/Driver column of a signal-description table encodes the same information
/// as a prose sentence: `PADDR | Requester | ...` means `(Requester, Drives, PADDR)`.
/// This is part of the knowledge graph — the table is part of the document, and the
/// Source column directly records which actor drives each signal.
///
/// Unlike prose extraction, no verb-pattern matching is needed here: the table cell
/// value IS the actor name, and the table structure implies the Drives relation.
/// Actor names are stored as-is ("Requester", "Completer", "Clock", etc.) without
/// vocabulary normalisation.
fn extract_relations_from_signal_tables(source_ir: &SourceIr) -> Vec<ActorSignalRelation> {
    let mut records = Vec::new();
    let mut counter = 1usize;

    for table in &source_ir.structured_tables {
        if !matches!(table.table_kind, TableKind::SignalDescription) {
            continue;
        }

        // Find the column index for a Source/Driver/Direction column.
        let header_texts: Vec<String> = table
            .header_rows
            .first()
            .map(|row| row.iter().map(|c| c.text.to_ascii_lowercase()).collect())
            .unwrap_or_default();
        let source_col = header_texts.iter().position(|h| {
            h.contains("source")
                || h.contains("driver")
                || h.contains("direction")
                || h.contains("destination")
        });

        let Some(src_col_idx) = source_col else {
            continue; // no Source column in this table (e.g. AXI Name|Width|Default|Description)
        };

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

            // Actor name from Source column
            let Some(source_cell) = row.get(src_col_idx) else {
                continue;
            };
            let actor = source_cell.text.trim().to_string();
            if actor.is_empty() {
                continue;
            }

            // Determine relation: the Source column says who drives the signal.
            // If the cell says "input" or "output" explicitly, use that.
            // Otherwise the Source column value is the driving actor.
            let actor_lower = actor.to_ascii_lowercase();
            let relation = if actor_lower == "input" {
                // Unusual: Source column says direction directly
                RelationKind::Reads // input = the actor READS this (but we don’t know who)
            } else {
                RelationKind::Drives // any other value = the named actor drives this signal
            };

            records.push(ActorSignalRelation {
                relation_id: format!("tbl_asr_{counter:04}"),
                actor_name: actor,
                signal_name: signal_token,
                relation,
                source_statement_ids: vec![table.table_id.clone()],
                automation_confidence: AutomationConfidence::Medium,
            });
            counter += 1;
        }
    }

    records
}

/// Collect hardware signal widths from signal-description table Width columns.
/// Returns a map of signal_name → WidthHint.
/// Used to enrich KG-synthesized direction declarations with width information
/// (e.g. "Signal PADDR is output width ADDR_WIDTH.") even when direction
/// must come from prose rather than the table.
fn collect_signal_widths_from_tables(
    source_ir: &SourceIr,
) -> std::collections::HashMap<String, WidthHint> {
    let mut widths = std::collections::HashMap::new();
    for table in &source_ir.structured_tables {
        if !matches!(table.table_kind, TableKind::SignalDescription) {
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
            if let Some(width_cell) = row.get(w_col) {
                let t = width_cell.text.trim();
                if t.is_empty() || t == "-" || t == "N/A" {
                    continue;
                }
                if let Ok(n) = t.parse::<u32>() {
                    if n > 0 {
                        widths.insert(signal, WidthHint::Numeric(n));
                    }
                } else if t.chars().any(|c| c.is_ascii_alphabetic()) {
                    widths.insert(signal, WidthHint::Parametric(t.to_string()));
                }
            }
        }
    }
    widths
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

    // Passive patterns: "{signal} is {verb} by|from {actor}"
    const PASSIVE_DRIVES_VERBS: &[&str] = &[
        "driven",
        "asserted",
        "provided",
        "returned",
        "sent",
        "sourced",
        "generated",
        "issued",
        "set",
        "produced",
        "supplied",
        "output",
        "outputted",
        "activated",
        "presented",
        "placed",
        "applied",
    ];
    const PASSIVE_READS_VERBS: &[&str] = &[
        "read",
        "sampled",
        "monitored",
        "accepted",
        "received",
        "captured",
        "observed",
        "detected",
        "checked",
        "latched",
    ];
    // Active patterns: "{actor} {verb} {signal}"  (verb immediately before signal)
    const ACTIVE_DRIVES_VERBS: &[&str] = &[
        "drives",
        "asserts",
        "provides",
        "returns",
        "sources",
        "generates",
        "issues",
        "sets",
        "produces",
        "supplies",
        "outputs",
        "sends",
        "activates",
        "presents",
        "applies",
        "places",
        "drive",
    ];
    const ACTIVE_READS_VERBS: &[&str] = &[
        "reads", "samples", "monitors", "accepts", "receives", "captures", "observes", "detects",
        "checks", "latches", "read", "sample", "monitor", "accept", "receive",
    ];

    let mut records = Vec::new();
    let mut counter = 1usize;
    let mut seen: std::collections::HashSet<(String, String, u8)> =
        std::collections::HashSet::new();

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

        for signal in known_signals {
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
                        if actor_start <= text.len() {
                            if let Some(actor) = extract_actor_phrase(&text[actor_start..]) {
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

            // ── Active drives: "{actor} {verb} {signal}" ────────────────────
            for verb in ACTIVE_DRIVES_VERBS {
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
        "which", "where", "as", "is", "are", "has", "have", "will", "shall",
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
    Some(actor)
}

/// Extract the actor name from the text BEFORE an active verb phrase like
/// `"drives HTRANS"`.  Returns the last 1–2 meaningful words of the subject,
/// stripped of trailing articles and punctuation.
///
/// Examples:
///   `"The Manager"` → `Some("Manager")`
///   `"The Completer device"` → `Some("Completer device")`
///   `"AMBA AHB The Manager"` → `Some("Manager")`
fn extract_subject_phrase(text: &str) -> Option<String> {
    const SKIP_WORDS: &[&str] = &[
        "the", "a", "an", "this", "that", "and", "or", "when", "if", ".", ",", ";", "(", ")", ":",
    ];
    let words: Vec<&str> = text.split_whitespace().collect();
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
    Some(actor)
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
        let mut ordered_signals: Vec<&String> = known_signals.iter().collect();
        ordered_signals.sort_by_key(|signal| std::cmp::Reverse(signal.len()));

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
) -> bool {
    if matches!(table.table_kind, TableKind::Encoding) {
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
            table.table_kind,
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
        if !table_looks_like_encoding(table, &anchor_signal) {
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
        let polarity = detect_signal_polarity(&lowered);
        let Some(polarity) = polarity else {
            continue;
        };

        let mentioned_signals: Vec<String> = ordered_signals
            .iter()
            .filter_map(|signal| {
                let signal_lower = signal.to_ascii_lowercase();
                contains_reference_token(&lowered, &signal_lower).then_some((*signal).clone())
            })
            .collect();
        if mentioned_signals.len() != 1 {
            continue;
        }

        observations.push(SignalPolarityObservationCandidate {
            signal_name: mentioned_signals[0].clone(),
            polarity,
            source_kind: SignalPolarityEvidenceSourceKind::ProseStatement,
            supporting_statement_ids: vec![statement.statement_id.clone()],
            supporting_table_ids: Vec::new(),
        });
    }

    observations
}

fn extract_signal_polarity_from_signal_tables(
    source_ir: &SourceIr,
    known_signals: &HashSet<String>,
) -> Vec<SignalPolarityObservationCandidate> {
    let mut observations = Vec::new();

    for table in &source_ir.structured_tables {
        if !matches!(table.table_kind, TableKind::SignalDescription) {
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

fn collect_signal_polarity_facts(
    source_ir: &SourceIr,
    statements: &[ExtractedStatement],
    known_signals: &HashSet<String>,
) -> SignalPolarityFactCollection {
    let mut observations_by_signal =
        BTreeMap::<String, Vec<SignalPolarityObservationRecord>>::new();

    for observation in extract_signal_polarity_from_prose(statements, known_signals) {
        record_signal_polarity_observation(&mut observations_by_signal, observation);
    }
    for observation in extract_signal_polarity_from_signal_tables(source_ir, known_signals) {
        record_signal_polarity_observation(&mut observations_by_signal, observation);
    }

    let mut resolved = HashMap::new();
    let mut conflicts = Vec::new();
    let mut conflict_counter = 1usize;

    for (signal_name, observations) in observations_by_signal {
        let polarities = observations
            .iter()
            .map(|observation| observation.polarity)
            .collect::<BTreeSet<_>>();
        if polarities.len() == 1 {
            if let Some(polarity) = polarities.iter().next().copied() {
                resolved.insert(signal_name, polarity);
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
    if text_lower.contains("active low")
        || text_lower.contains("active-low")
        || text_lower.contains("asserted low")
        || text_lower.contains("low asserted")
    {
        Some(SignalPolarity::ActiveLow)
    } else if text_lower.contains("active high")
        || text_lower.contains("active-high")
        || text_lower.contains("asserted high")
        || text_lower.contains("high asserted")
    {
        Some(SignalPolarity::ActiveHigh)
    } else {
        None
    }
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

fn extract_dynamic_signal_constraints(
    statements: &[ExtractedStatement],
    counter: &mut usize,
    discovered_values: &HashSet<String>,
) -> Vec<SignalConstraintRecord> {
    if discovered_values.is_empty() {
        return Vec::new();
    }

    let mut records = Vec::new();
    for statement in statements {
        if matches!(statement.class, StatementClass::SignalValueConstraint) {
            continue;
        }

        let lowered = statement.text.to_ascii_lowercase();
        let Some(value) = extract_discovered_state_value_from_text(&lowered, discovered_values)
        else {
            continue;
        };

        let subject_part = text_before_condition_marker(&statement.text);
        let mut subject_signals =
            collect_subject_signal_tokens_with_discovered_values(subject_part, discovered_values);
        if subject_signals.is_empty() {
            subject_signals = collect_subject_signal_tokens_with_discovered_values(
                &statement.text,
                discovered_values,
            );
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
                constraint_kind: SignalConstraintKind::MustBeValue {
                    value: value.clone(),
                },
                target_value: Some(value.clone()),
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
) -> Vec<ExtractedStatement> {
    let mut statements = Vec::new();
    let mut clock_found = false;
    let mut reset_found = false;

    'outer: for table in &source_ir.structured_tables {
        if !matches!(table.table_kind, TableKind::SignalDescription) {
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
                if cell_lower.starts_with("clock")
                    || cell_lower.contains("clock signal")
                    || cell_lower.contains("bus clock")
                    || cell_lower.contains("is a clock")
                    || cell_lower.contains("times all bus transfers")
                    || cell_lower.contains("timed against the rising edge")
                    || cell_lower.contains("sampled on the rising edge of")
                    || cell_lower.contains("related to the rising edge")
                    || cell_lower.contains("all signals are sampled")
                    || cell_lower.contains("all signal timings")
                {
                    if row_clock_desc
                        .as_ref()
                        .map(|d: &String| d.len())
                        .unwrap_or(0)
                        < cell_lower.len()
                    {
                        row_clock_desc = Some(cell_lower.clone());
                    }
                }

                // Reset description — prefer longer / more informative text.
                if cell_lower.starts_with("reset")
                    || cell_lower.contains("reset signal")
                    || cell_lower.contains("is the reset")
                    || cell_lower.contains("is a reset")
                    || cell_lower.contains("bus reset")
                    || (cell_lower.contains("is an active") && cell_lower.contains("reset"))
                {
                    if row_reset_desc
                        .as_ref()
                        .map(|d: &String| d.len())
                        .unwrap_or(0)
                        < cell_lower.len()
                    {
                        row_reset_desc = Some(cell_lower.clone());
                    }
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
            if !reset_found {
                if let Some(desc) = row_reset_desc {
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
                    let kind = if desc.contains("synchronous") {
                        "synchronous"
                    } else if desc.contains("asynchronous") || desc.contains("async") {
                        "asynchronous"
                    } else if polarity == "active low" {
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
            }

            if clock_found && reset_found {
                break 'outer;
            }
        }
    }

    statements
}

fn synthesize_signal_semantic_hints(
    source_ir: &SourceIr,
    statements: &[ExtractedStatement],
    signal_alias_map: &BTreeMap<String, String>,
    visual_evidence: &[VisualEvidenceItem],
) -> (
    Vec<SignalSemanticHintRecord>,
    Vec<SignalSemanticConflictRecord>,
) {
    let mut hints = synthesize_signal_semantic_hints_from_tables(source_ir);
    let known_signals = collect_known_signal_names_for_semantic_hints(source_ir, statements);
    let mut seen = hints
        .iter()
        .map(signal_semantic_hint_key)
        .collect::<BTreeSet<_>>();
    for hint in
        synthesize_signal_semantic_hints_from_prose(statements, signal_alias_map, &known_signals)
    {
        if seen.insert(signal_semantic_hint_key(&hint)) {
            hints.push(hint);
        }
    }
    for hint in synthesize_signal_semantic_hints_from_visual_evidence(
        visual_evidence,
        signal_alias_map,
        &known_signals,
    ) {
        if seen.insert(signal_semantic_hint_key(&hint)) {
            hints.push(hint);
        }
    }
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
) -> HashSet<String> {
    let mut known_signals = collect_known_signal_names(statements);
    known_signals.extend(collect_signal_names_from_tables(source_ir));
    known_signals
}

fn synthesize_signal_semantic_hints_from_tables(
    source_ir: &SourceIr,
) -> Vec<SignalSemanticHintRecord> {
    let mut hints = Vec::new();
    let mut seen = BTreeSet::<String>::new();

    for table in &source_ir.structured_tables {
        if !matches!(table.table_kind, TableKind::SignalDescription) {
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

            let semantic_tags = infer_signal_semantic_tags_from_description(description);
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
) -> Vec<SignalSemanticHintRecord> {
    let mut hints = Vec::new();
    let mut seen = BTreeSet::<String>::new();

    for statement in statements {
        if !matches!(statement.class, StatementClass::SourceFact) {
            continue;
        }

        let semantic_tags = infer_signal_semantic_tags_from_description(&statement.text);
        if semantic_tags.is_empty() {
            continue;
        }

        let Some((signal_name, alias_grounded)) = resolve_signal_semantic_target_from_text(
            &statement.text,
            known_signals,
            signal_alias_map,
        ) else {
            continue;
        };
        let source_kind = if alias_grounded {
            SignalSemanticHintSourceKind::AliasGroundedProseStatement
        } else {
            SignalSemanticHintSourceKind::ProseStatement
        };

        let key = format!(
            "{}:{}:{}:{}",
            signal_name,
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
            signal_name,
            semantic_tags,
            source_kind,
            source_text: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
            supporting_table_ids: Vec::new(),
            supporting_visual_evidence_ids: Vec::new(),
            automation_confidence: AutomationConfidence::Low,
        });
    }

    hints
}

fn synthesize_signal_semantic_hints_from_visual_evidence(
    visual_evidence: &[VisualEvidenceItem],
    signal_alias_map: &BTreeMap<String, String>,
    known_signals: &HashSet<String>,
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
                signal_alias_map,
                AutomationConfidence::Low,
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
                    signal_alias_map,
                    AutomationConfidence::Low,
                );
            }
        }
    }

    hints
}

fn push_visual_signal_semantic_hint(
    hints: &mut Vec<SignalSemanticHintRecord>,
    seen: &mut BTreeSet<String>,
    source_text: &str,
    source_kind: SignalSemanticHintSourceKind,
    visual_evidence_id: &str,
    known_signals: &HashSet<String>,
    signal_alias_map: &BTreeMap<String, String>,
    automation_confidence: AutomationConfidence,
) {
    let semantic_tags = infer_signal_semantic_tags_from_description(source_text);
    if semantic_tags.is_empty() {
        return;
    }

    let Some((signal_name, _alias_grounded)) =
        resolve_signal_semantic_target_from_text(source_text, known_signals, signal_alias_map)
    else {
        return;
    };

    let key = format!(
        "{}:{}:{}:{}",
        signal_name,
        visual_evidence_id,
        source_kind.as_str(),
        semantic_tags
            .iter()
            .map(|tag| tag.as_str())
            .collect::<Vec<_>>()
            .join(",")
    );
    if !seen.insert(key) {
        return;
    }

    hints.push(SignalSemanticHintRecord {
        signal_name,
        semantic_tags,
        source_kind,
        source_text: source_text.to_string(),
        supporting_statement_ids: Vec::new(),
        supporting_table_ids: Vec::new(),
        supporting_visual_evidence_ids: vec![visual_evidence_id.to_string()],
        automation_confidence,
    });
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

fn resolve_signal_semantic_target_from_text(
    text: &str,
    known_signals: &HashSet<String>,
    signal_alias_map: &BTreeMap<String, String>,
) -> Option<(String, bool)> {
    let lowered = text.to_ascii_lowercase();
    let mut direct_matches = BTreeSet::new();
    for signal_name in known_signals {
        if contains_reference_token(&lowered, &signal_name.to_ascii_lowercase()) {
            direct_matches.insert(signal_name.clone());
        }
    }
    if direct_matches.is_empty() {
        direct_matches.extend(collect_hardware_signal_tokens_anywhere(text));
    }

    let mut alias_matches = BTreeSet::new();
    for (alias_phrase, signal_name) in signal_alias_map {
        if lowered.contains(alias_phrase.as_str()) {
            alias_matches.insert(signal_name.clone());
        }
    }

    let mut resolved = direct_matches.clone();
    resolved.extend(alias_matches.iter().cloned());
    if resolved.len() != 1 {
        return None;
    }

    let signal_name = resolved.into_iter().next()?;
    Some((signal_name, !alias_matches.is_empty()))
}

fn collect_hardware_signal_tokens_anywhere(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !(c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
        .filter(|token| is_hardware_signal_token(token) && !is_signal_synthesis_non_signal(token))
        .map(ToString::to_string)
        .collect()
}

fn infer_signal_semantic_tags_from_description(description: &str) -> Vec<SignalSemanticTag> {
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
            "acknowledge",
            "acknowledges",
            "acknowledged",
            "complete the transfer",
            "transfer can complete",
        ],
    ) {
        tags.insert(SignalSemanticTag::HandshakeReadyLike);
    }

    tags.into_iter().collect()
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
    // Driving actors (Source side) → output from that actor's perspective.
    // Covers AMBA 3/4 (Master/Slave), AMBA 5 (Manager/Subordinate), APB 5 (Requester/Completer).
    if lowered.contains("manager")
        || lowered.contains("initiator")
        || lowered.contains("master")
        || lowered.contains("requester")
    {
        return Some("output");
    }
    if lowered.contains("subordinate")
        || lowered.contains("slave")
        || lowered.contains("responder")
        || lowered.contains("multiplexor")
        || lowered.contains("completer")
        || lowered.contains("target")
    {
        return Some("input");
    }
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
                        | "SINGLE" | "INCR" | "WRAP" | "RETRY" | "SPLIT"
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

    let default_dir = infer_signal_direction_from_section(section_kind, section_title);

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
                // Source column: the cell names the DRIVING actor.
                //   Requester / Initiator / Master         → output (signal driven from this actor)
                //   Completer / Subordinate / Slave / Target → input  (signal driven by the other side)
                //   Clock / Reset / System-bus / Global     → input  (infrastructure)
                source_col.and_then(|col| row.get(col)).and_then(|cell| {
                    let t = cell.text.to_ascii_lowercase();
                    if t.contains("output")
                        || t.contains("requester")
                        || t.contains("initiator")
                        || t.contains("master")
                    {
                        Some("output")
                    } else if t.contains("input")
                        || t.contains("completer")
                        || t.contains("subordinate")
                        || t.contains("slave")
                        || t.contains("responder")
                        || t.contains("target")
                    {
                        Some("input")
                    } else if t.contains("clock")
                        || t.contains("reset")
                        || t.contains("system bus")
                        || t.contains("global")
                    {
                        Some("input") // Infrastructure signals distributed as inputs
                    } else {
                        None
                    }
                })
            })
            .or_else(|| {
                // Destination column: the cell names the RECEIVING actor (inverted semantics).
                //   Signal flows TO Subordinate/Completer/Slave/Target → output from driver
                //   Signal flows TO Manager/Requester/Initiator/Master  → input  to driver
                dest_col.and_then(|col| row.get(col)).and_then(|cell| {
                    let t = cell.text.to_ascii_lowercase();
                    if t.contains("subordinate")
                        || t.contains("completer")
                        || t.contains("slave")
                        || t.contains("target")
                        || t.contains("responder")
                    {
                        Some("output") // flows TO the subordinate side
                    } else if t.contains("manager")
                        || t.contains("requester")
                        || t.contains("initiator")
                        || t.contains("master")
                    {
                        Some("input") // flows TO the manager side
                    } else {
                        None
                    }
                })
            })
            .or(default_dir);

        // Extract width: numeric (e.g. 32) or parametric (e.g. ADDR_WIDTH, DATA_WIDTH/8).
        // Both are valid RTL port widths; parametric means the integrator sets the value.
        let width: Option<WidthHint> = width_col.and_then(|col| {
            row.get(col).and_then(|cell| {
                let t = cell.text.trim();
                // Skip empty or placeholder cells
                if t.is_empty() || t == "-" || t == "N/A" || t == "n/a" {
                    return None;
                }
                // Try numeric first (positive; no artificial upper bound — bus widths can be large)
                if let Ok(n) = t.parse::<u32>() {
                    return (n > 0).then_some(WidthHint::Numeric(n));
                }
                // Non-numeric but contains alphabetic chars → parametric expression
                if t.chars().any(|c| c.is_ascii_alphabetic()) {
                    return Some(WidthHint::Parametric(t.to_string()));
                }
                None
            })
        });

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

fn converge_evidence_extractions(
    source_ir: &SourceIr,
    base_extracted_statements: Vec<ExtractedStatement>,
    seed_synthesized_statements: Vec<ExtractedStatement>,
    contract_statements: Vec<ExtractedStatement>,
    statement_counter: &mut usize,
) -> (
    Vec<ExtractedStatement>,
    Vec<SignalConstraintRecord>,
    Vec<ConditionalRuleRecord>,
    Vec<SignalPolarityConflictRecord>,
    Vec<ActorSignalRelation>,
) {
    let signal_names_from_tables = collect_signal_names_from_tables(source_ir);
    let signal_widths_from_tables = collect_signal_widths_from_tables(source_ir);
    let table_relations = extract_relations_from_signal_tables(source_ir);
    let mut dynamic_synthesized_statements = Vec::new();
    let mut final_extracted_statements = Vec::new();
    let mut final_signal_constraints = Vec::new();
    let mut final_conditional_rules = Vec::new();
    let mut final_signal_polarity_conflicts = Vec::new();
    let mut final_actor_signal_relations = Vec::new();
    let max_passes = source_ir.structured_tables.len().max(1) + 4;

    for _pass in 0..max_passes {
        let mut extracted_statements = base_extracted_statements.clone();
        extracted_statements.extend(seed_synthesized_statements.iter().cloned());
        extracted_statements.extend(contract_statements.iter().cloned());
        extracted_statements.extend(dynamic_synthesized_statements.iter().cloned());

        let mut known_signals = signal_names_from_tables.clone();
        known_signals.extend(collect_known_signal_names(&extracted_statements));
        let discovered_values = collect_discovered_enum_values(&[extracted_statements.as_slice()]);
        let signal_polarity =
            collect_signal_polarity_facts(source_ir, &extracted_statements, &known_signals);

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

        let mut actor_signal_relations =
            extract_actor_signal_relations(&extracted_statements, &known_signals);
        actor_signal_relations.extend(table_relations.iter().cloned());
        let actor_signal_relations = dedup_actor_signal_relations(actor_signal_relations);

        let already_declared = collect_known_signal_names(&extracted_statements);
        let mut candidate_statements =
            scan_encoding_tables_by_signal_anchor(source_ir, &known_signals, statement_counter);
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
        final_signal_polarity_conflicts = signal_polarity.conflicts;
        final_actor_signal_relations = actor_signal_relations;

        if new_dynamic_statements.is_empty() {
            break;
        }
        dynamic_synthesized_statements.extend(new_dynamic_statements);
    }

    (
        final_extracted_statements,
        final_signal_constraints,
        final_conditional_rules,
        final_signal_polarity_conflicts,
        final_actor_signal_relations,
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

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::source::{
        SourceIr, StructuredTableCellRecord, StructuredTableRecord, TableKind, VisualAsset,
        VisualAssetKind,
    };

    use super::{EvidenceIr, EvidenceLinkKind, StatementClass, VisualObservationKind};

    fn make_table_cell(text: &str, is_header: bool) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        }
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
        assert!(conflict.observations.iter().any(|observation| matches!(
            observation.source_kind,
            super::SignalPolarityEvidenceSourceKind::ProseStatement
        ) && matches!(
            observation.polarity,
            super::SignalPolarity::ActiveLow
        )));
        assert!(conflict.observations.iter().any(|observation| matches!(
            observation.source_kind,
            super::SignalPolarityEvidenceSourceKind::SignalDescriptionTable
        ) && matches!(
            observation.polarity,
            super::SignalPolarity::ActiveHigh
        )));

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
        }));
        assert!(evidence_ir.signal_semantic_hints.iter().any(|hint| {
            hint.signal_name == "XACK"
                && hint
                    .semantic_tags
                    .contains(&super::SignalSemanticTag::HandshakeReadyLike)
        }));

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
        }));

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
        assert!(conflict.observations.iter().any(|observation| {
            observation
                .semantic_tags
                .contains(&super::SignalSemanticTag::HandshakeValidLike)
        }));
        assert!(conflict.observations.iter().any(|observation| {
            observation
                .semantic_tags
                .contains(&super::SignalSemanticTag::HandshakeReadyLike)
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
