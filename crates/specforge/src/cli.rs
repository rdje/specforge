use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::ir::adapters::AdapterTarget;

#[derive(Debug, Parser)]
#[command(
    name = "specforge",
    version,
    about = "Stage specifications into SourceIR, EvidenceIR, SemanticIR, IntentIR, and downstream adapters"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Inspect a source path and report the detected source kind
    Inspect(InspectArgs),
    /// Build or materialize a SourceIR artifact for a source
    Ingest(IngestArgs),
    /// Build or materialize an EvidenceIR artifact from a SourceIR JSON file
    Evidence(EvidenceArgs),
    /// Build or materialize a SemanticIR artifact from an EvidenceIR JSON file
    Semantic(SemanticArgs),
    /// Build or materialize an IntentIR artifact from a SemanticIR JSON file
    Intent(IntentArgs),
    /// Build or materialize a target adapter artifact from an IntentIR JSON file
    Adapt(AdaptArgs),
    /// Enrich a SourceIR artifact with VLM-derived visual observations (timing diagrams, state machines)
    Enrich(EnrichArgs),
    /// Validate an IR artifact and report extraction coverage metrics
    Validate(ValidateArgs),
}

#[derive(Debug, Args)]
pub struct InspectArgs {
    /// Path to inspect
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct IngestArgs {
    /// Source file or directory to ingest
    pub source: PathBuf,
    /// Do not write SourceIR artifacts; print the computed SourceIR JSON instead
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Args)]
pub struct EvidenceArgs {
    /// Path to a SourceIR JSON artifact
    pub source_ir: PathBuf,
    /// Do not write EvidenceIR artifacts; print the computed EvidenceIR JSON instead
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Args)]
pub struct SemanticArgs {
    /// Path to an EvidenceIR JSON artifact
    pub evidence_ir: PathBuf,
    /// Do not write SemanticIR artifacts; print the computed SemanticIR JSON instead
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Args)]
pub struct IntentArgs {
    /// Path to a SemanticIR JSON artifact
    pub semantic_ir: PathBuf,
    /// Do not write IntentIR artifacts; print the computed IntentIR JSON instead
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AdapterTargetArg {
    #[value(name = "fsm")]
    Fsm,
    #[value(name = "systemverilog")]
    SystemVerilog,
    #[value(name = "verilog")]
    Verilog,
    #[value(name = "vhdl")]
    Vhdl,
}

impl From<AdapterTargetArg> for AdapterTarget {
    fn from(value: AdapterTargetArg) -> Self {
        match value {
            AdapterTargetArg::Fsm => AdapterTarget::Fsm,
            AdapterTargetArg::SystemVerilog => AdapterTarget::SystemVerilog,
            AdapterTargetArg::Verilog => AdapterTarget::Verilog,
            AdapterTargetArg::Vhdl => AdapterTarget::Vhdl,
        }
    }
}

#[derive(Debug, Args)]
pub struct ValidateArgs {
    /// Path to any IR artifact (source_ir.json, evidence_ir.json, semantic_ir.json, intent_ir.json)
    pub artifact: std::path::PathBuf,
}

/// VLM provider selection for the `enrich` command.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum VlmProviderArg {
    /// Local Ollama server at http://localhost:11434. Use `llava:13b` or `ibm/granite-docling:258m`.
    Ollama,
    /// OpenAI cloud API. Requires `OPENAI_API_KEY` environment variable. Uses `gpt-4o`.
    OpenAi,
    /// LM Studio local server at http://localhost:1234. Load a vision model in LM Studio.
    LmStudio,
    /// Skip VLM enrichment (dry-run / no-op). Useful for testing the classification step.
    Skip,
}

#[derive(Debug, Args)]
pub struct EnrichArgs {
    /// Path to a SourceIR JSON artifact to enrich
    pub source_ir: PathBuf,
    /// VLM provider to use for visual content enrichment
    #[arg(long, value_enum, default_value = "skip")]
    pub vlm_provider: VlmProviderArg,
    /// Model name to use (overrides the provider default).
    /// Defaults: ollama=llava:13b, openai=gpt-4o, lmstudio=(loaded model)
    #[arg(long)]
    pub vlm_model: Option<String>,
    /// Only classify diagram kinds without calling the VLM (zero-cost step)
    #[arg(long)]
    pub classify_only: bool,
    /// Do not write enriched SourceIR; print the enrichment summary instead
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Args)]
pub struct AdaptArgs {
    /// Path to an IntentIR JSON artifact
    pub intent_ir: PathBuf,
    /// Adapter target to lower toward
    #[arg(long, value_enum)]
    pub target: AdapterTargetArg,
    /// Do not write adapter artifacts; print the computed adapter JSON instead
    #[arg(long)]
    pub dry_run: bool,
}
