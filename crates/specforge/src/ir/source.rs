mod docling_backend;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::adapters::AdapterTarget;

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NormalizationStrategy {
    ExistingMarkdown,
    ExtractStructuredPdfArtifacts,
    InterpretDirectory,
    ResolveUnknownSource,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NormalizationBackend {
    DirectMarkdown,
    Docling,
    Marker,
    MistralOcr,
    Undecided,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SourceIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub source: SourceRegistration,
    pub artifact_layout: SourceArtifactLayout,
    pub document_identity: DocumentIdentity,
    pub normalization_plan: NormalizationPlan,
    pub page_artifacts: Vec<PageArtifact>,
    pub visual_assets: Vec<VisualAsset>,
    pub placeholder_bindings: Vec<PlaceholderBinding>,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
    pub downstream_stages: Vec<IrStage>,
    pub adapter_targets: Vec<AdapterTarget>,
    pub planned_actions: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

impl SourceIr {
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
            placeholder_bindings: Vec::new(),
            residual_decisions,
            downstream_stages: vec![IrStage::EvidenceIr, IrStage::SemanticIr, IrStage::IntentIr],
            adapter_targets: vec![
                AdapterTarget::Fsm,
                AdapterTarget::SystemVerilog,
                AdapterTarget::Verilog,
                AdapterTarget::Vhdl,
            ],
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
        self.placeholder_bindings = backend_summary.placeholder_bindings;
        self.normalization_plan.status = NormalizationStatus::Ready;
        self.planned_actions = materialized_source_actions(&self.residual_decisions);
        self.normalization_plan.notes.push(format!(
            "docling materialized promoted markdown, backend raw JSON, {} page artifacts, {} picture assets, and {} table assets",
            backend_summary.metadata.page_count,
            backend_summary.metadata.picture_count,
            backend_summary.metadata.table_count
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SourceRegistration {
    pub requested_path: PathBuf,
    pub canonical_path: PathBuf,
    pub path_kind: SourcePathKind,
    pub source_kind: SourceKind,
    pub stable_artifact_stem: String,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DocumentIdentity {
    pub document_key: String,
    pub display_name: String,
    pub origin_kind: SourceKind,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
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
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaceholderBinding {
    pub placeholder_text: String,
    pub asset_id: String,
    pub normalized_source_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ResidualDecisionPacket {
    pub packet_id: String,
    pub question: String,
    pub why_unresolved: String,
    pub automation_confidence: AutomationConfidence,
    pub candidate_interpretations: Vec<CandidateInterpretation>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
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
    use std::sync::Mutex;

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::IrStage;

    use super::{NormalizationBackend, SourceIr, SourceKind, document_key, stable_stem};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvVarGuard {
        key: &'static str,
        original: Option<std::ffi::OsString>,
    }

    impl EnvVarGuard {
        fn set_path(key: &'static str, value: &Path) -> Self {
            let original = env::var_os(key);
            // SAFETY: tests serialize environment mutation with ENV_LOCK.
            unsafe { env::set_var(key, value) };
            Self { key, original }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            match &self.original {
                Some(value) => {
                    // SAFETY: tests serialize environment mutation with ENV_LOCK.
                    unsafe { env::set_var(self.key, value) };
                }
                None => {
                    // SAFETY: tests serialize environment mutation with ENV_LOCK.
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

        Ok(())
    }

    #[test]
    fn pdf_source_ir_materialization_uses_backend_helper_and_writes_manifests() -> Result<()> {
        let _env_lock = ENV_LOCK.lock().expect("environment mutex poisoned");
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
        assert_eq!(
            source_ir.planned_actions,
            vec![
                "build_evidence_ir".to_string(),
                "build_semantic_ir".to_string(),
                "build_intent_ir".to_string(),
                "plan_adapter_lowering".to_string()
            ]
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
}
