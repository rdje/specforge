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
    /// Inspect local runtime readiness for the default local-first PDF-to-IntentIR pipeline
    Doctor(DoctorArgs),
    /// Iterate the staged pipeline until the materialized knowledge snapshot stops growing
    Converge(ConvergeArgs),
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
    /// Validate artifacts and project their latest reports into tracked live docs
    ProjectValidation(ProjectValidationArgs),
    /// Inspect or execute a schema-v2 validation rescan plan
    RescanPlan(RescanPlanArgs),
    /// Run tracked KG-quality fixtures against the staged pipeline
    KgBench(KgBenchArgs),
    /// Build a local cross-document prior store from validated IntentIR artifacts
    LearnPriors(LearnPriorsArgs),
    /// Refresh tracked corpus knowledge-base pages from reviewable evidence
    CorpusKb(CorpusKbArgs),
    /// Reclaim local generated artifacts that can be rebuilt later
    Clean(CleanArgs),
    /// Enrich an EvidenceIR artifact with LLM-extracted NLP Level 3 constraints
    NlpEnrich(NlpEnrichArgs),
    /// Extract typed ActorContracts from prose via the constrained-verified extractor (Qwen)
    ExtractContracts(ExtractContractsArgs),
}

#[derive(Debug, Args)]
pub struct InspectArgs {
    /// Path to inspect
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct DoctorArgs {
    /// Return a non-zero exit code when a required runtime is missing
    #[arg(long)]
    pub strict: bool,
}

#[derive(Debug, Args)]
pub struct ConvergeArgs {
    /// Source specification file to iterate on
    pub source: PathBuf,
    /// Adapter target to materialize each pass
    #[arg(long, value_enum, default_value = "isf")]
    pub target: AdapterTargetArg,
    /// Safety cap on whole-pipeline convergence passes
    #[arg(long, default_value = "8")]
    pub max_iterations: usize,
    /// VLM provider to use for figure enrichment during each pass.
    /// Defaults to `ollama`; pass `--vlm-provider skip` to opt out.
    #[arg(long, value_enum, default_value = "ollama")]
    pub vlm_provider: VlmProviderArg,
    /// Model name override for figure enrichment
    #[arg(long)]
    pub vlm_model: Option<String>,
    /// LLM provider to use for Level 3 NLP backannotation during each pass.
    /// Defaults to `ollama`; pass `--nlp-provider skip` to opt out.
    #[arg(long, value_enum, default_value = "ollama")]
    pub nlp_provider: VlmProviderArg,
    /// Model name override for NLP Level 3 backannotation
    #[arg(long)]
    pub nlp_model: Option<String>,
    /// Maximum number of sentences to send to NLP Level 3 per pass (0 = all)
    #[arg(long, default_value = "0")]
    pub nlp_max_sentences: usize,
    /// Advisory local prior-memory store to consult during extraction when present
    #[arg(long, default_value = "generated/prior_memory/corpus_memory.json")]
    pub prior_memory: PathBuf,
    /// Optional schema-v2 validation rescan plan to inspect after convergence stabilizes
    #[arg(long)]
    pub rescan_plan: Option<PathBuf>,
    /// Execute whitelisted recommendations from --rescan-plan after convergence stabilizes
    #[arg(long)]
    pub execute_rescan_plan: bool,
    /// Maximum pending rescan-plan recommendation(s) to process; 0 means all
    #[arg(long, default_value = "0")]
    pub rescan_plan_limit: usize,
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
    /// Advisory local prior-memory store to consult during extraction when present
    #[arg(long, default_value = "generated/prior_memory/corpus_memory.json")]
    pub prior_memory: PathBuf,
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
    #[value(name = "isf")]
    Isf,
}

impl From<AdapterTargetArg> for AdapterTarget {
    fn from(value: AdapterTargetArg) -> Self {
        match value {
            AdapterTargetArg::Isf => AdapterTarget::Isf,
        }
    }
}

#[derive(Debug, Args)]
pub struct NlpEnrichArgs {
    /// Path to an EvidenceIR JSON artifact to enrich with LLM-extracted NLP constraints
    pub evidence_ir: std::path::PathBuf,
    /// LLM provider (same providers as specforge enrich)
    #[arg(long, value_enum, default_value = "skip")]
    pub vlm_provider: VlmProviderArg,
    /// Model name override (default: qwen2.5vl:7b for ollama/lmstudio, gpt-4o for openai)
    #[arg(long)]
    pub vlm_model: Option<String>,
    /// Show what would be enriched without making LLM calls
    #[arg(long)]
    pub dry_run: bool,
    /// Maximum number of NormativeStatement sentences to send to LLM per pass (0 = all)
    #[arg(long, default_value = "0")]
    pub max_sentences: usize,
    /// Comma-separated declared signal names to ground the LLM prompt context.
    /// If omitted, signals are auto-extracted from synthesized Signal declarations
    /// in the EvidenceIR. Pass an empty string to disable grounding entirely.
    #[arg(long)]
    pub grounding_signals: Option<String>,
}

#[derive(Debug, Args)]
pub struct ExtractContractsArgs {
    /// Path to an EvidenceIR JSON artifact to enrich with extracted ActorContracts
    pub evidence_ir: std::path::PathBuf,
    /// LLM provider (default ollama; same providers as enrich/nlp-enrich)
    #[arg(long, value_enum, default_value = "ollama")]
    pub provider: VlmProviderArg,
    /// Model name override (default qwen2.5vl:7b for ollama/lmstudio, gpt-4o for openai)
    #[arg(long)]
    pub model: Option<String>,
    /// Show candidate statements without making LLM calls
    #[arg(long)]
    pub dry_run: bool,
    /// Maximum NormativeStatement sentences to send to the LLM (0 = all)
    #[arg(long, default_value = "0")]
    pub max_statements: usize,
}

#[derive(Debug, Args)]
pub struct ValidateArgs {
    /// Path to any IR artifact (source_ir.json, evidence_ir.json, semantic_ir.json, intent_ir.json)
    pub artifact: std::path::PathBuf,
}

#[derive(Debug, Args)]
pub struct ProjectValidationArgs {
    /// One or more IR artifacts whose latest validation should be projected into the live docs
    #[arg(required = true)]
    pub artifacts: Vec<std::path::PathBuf>,
    /// Repository root containing the tracked live docs to update
    #[arg(long, default_value = ".")]
    pub repo_root: std::path::PathBuf,
    /// Local VLM provider policy for generated visual-motif rescan enrichment hints
    #[arg(long, value_enum, default_value = "auto-local")]
    pub rescan_vlm_provider: RescanVlmProviderArg,
    /// Optional local VLM model override to bake into generated visual rescan hints
    #[arg(long)]
    pub rescan_vlm_model: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum RescanVlmProviderArg {
    /// Prefer ready local Ollama, then ready local LM Studio, falling back to Ollama.
    AutoLocal,
    /// Always emit local Ollama enrichment hints.
    Ollama,
    /// Always emit local LM Studio enrichment hints.
    LmStudio,
    /// Emit a no-VLM enrichment hint for explicitly offline rescan planning.
    Skip,
}

#[derive(Debug, Args)]
pub struct RescanPlanArgs {
    /// Local schema-v2 validation rescan plan to inspect or execute
    #[arg(long, default_value = "generated/validation/rescan_plan.json")]
    pub plan: std::path::PathBuf,
    /// Execute whitelisted command hints instead of printing a dry-run plan
    #[arg(long)]
    pub execute: bool,
    /// Maximum pending recommendation(s) to process; 0 means all
    #[arg(long, default_value = "0")]
    pub limit: usize,
    /// Optional document_key filter for multi-document rescan plans
    #[arg(long)]
    pub document_key: Option<String>,
    /// Advisory local prior-memory store to use when executing EvidenceIR rebuild hints
    #[arg(long, default_value = "generated/prior_memory/corpus_memory.json")]
    pub prior_memory: std::path::PathBuf,
}

#[derive(Debug, Args)]
pub struct KgBenchArgs {
    /// Directory containing tracked KG-quality fixture directories
    #[arg(long, default_value = "crates/specforge/test_data/kg_quality")]
    pub fixtures_root: std::path::PathBuf,
    /// Optional fixture paths or fixture directories to run relative to fixtures_root
    pub fixtures: Vec<std::path::PathBuf>,
}

#[derive(Debug, Args)]
pub struct LearnPriorsArgs {
    /// One or more validated IntentIR artifacts to learn priors from
    #[arg(required = true)]
    pub artifacts: Vec<std::path::PathBuf>,
    /// Local output path for the learned prior store
    #[arg(long, default_value = "generated/prior_memory/corpus_memory.json")]
    pub output: std::path::PathBuf,
    /// Do not write the prior store; print the computed JSON instead
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Args)]
pub struct CorpusKbArgs {
    /// Validation report JSON files to project into the tracked corpus knowledge base
    pub validation_reports: Vec<PathBuf>,
    /// Repository root containing corpus_kb/
    #[arg(long, default_value = ".")]
    pub repo_root: PathBuf,
    /// Optional KG-quality fixture root to run and project into the corpus knowledge base
    #[arg(long)]
    pub kg_fixtures_root: Option<PathBuf>,
    /// Optional KG-quality fixture paths or directories relative to --kg-fixtures-root
    #[arg(long)]
    pub kg_fixture: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CleanScopeArg {
    /// Delete only heavyweight `generated/source_ir/<document_key>/normalized` bundles.
    SourceNormalized,
    /// Delete full per-document generated directories across SourceIR/EvidenceIR/SemanticIR/IntentIR/adapters.
    Document,
    /// Delete the entire generated artifact root in one sweep.
    AllGenerated,
}

#[derive(Debug, Args)]
pub struct CleanArgs {
    /// Generated artifact root to scan
    #[arg(long, default_value = "generated")]
    pub generated_root: PathBuf,
    /// Cleanup scope to target
    #[arg(long, value_enum, default_value = "source-normalized")]
    pub scope: CleanScopeArg,
    /// Optional document key filter
    #[arg(long)]
    pub document_key: Option<String>,
    /// Actually delete the discovered artifacts
    #[arg(long)]
    pub execute: bool,
}

/// VLM provider selection for the `enrich` command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum VlmProviderArg {
    /// Local Ollama server at http://localhost:11434. Common model: `qwen2.5vl:7b`.
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

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{CleanScopeArg, Cli, Commands, RescanVlmProviderArg, VlmProviderArg};

    #[test]
    fn converge_defaults_to_ollama_for_vlm_and_nlp() {
        let cli = Cli::parse_from(["specforge", "converge", "spec.pdf"]);
        let Commands::Converge(args) = cli.command else {
            panic!("expected converge command");
        };

        assert!(matches!(args.vlm_provider, VlmProviderArg::Ollama));
        assert!(matches!(args.nlp_provider, VlmProviderArg::Ollama));
        assert_eq!(
            args.prior_memory,
            PathBuf::from("generated/prior_memory/corpus_memory.json")
        );
        assert!(args.rescan_plan.is_none());
        assert!(!args.execute_rescan_plan);
        assert_eq!(args.rescan_plan_limit, 0);
    }

    use std::path::PathBuf;

    #[test]
    fn evidence_defaults_to_local_prior_memory() {
        let cli = Cli::parse_from([
            "specforge",
            "evidence",
            "generated/source_ir/doc/source_ir.json",
        ]);
        let Commands::Evidence(args) = cli.command else {
            panic!("expected evidence command");
        };

        assert_eq!(
            args.prior_memory,
            PathBuf::from("generated/prior_memory/corpus_memory.json")
        );
    }

    #[test]
    fn rescan_plan_defaults_to_dry_run_local_plan() {
        let cli = Cli::parse_from(["specforge", "rescan-plan"]);
        let Commands::RescanPlan(args) = cli.command else {
            panic!("expected rescan-plan command");
        };

        assert_eq!(
            args.plan,
            PathBuf::from("generated/validation/rescan_plan.json")
        );
        assert_eq!(
            args.prior_memory,
            PathBuf::from("generated/prior_memory/corpus_memory.json")
        );
        assert!(!args.execute);
        assert_eq!(args.limit, 0);
        assert!(args.document_key.is_none());
    }

    #[test]
    fn project_validation_defaults_to_auto_local_rescan_vlm_provider() {
        let cli = Cli::parse_from([
            "specforge",
            "project-validation",
            "generated/evidence_ir/doc/evidence_ir.json",
        ]);
        let Commands::ProjectValidation(args) = cli.command else {
            panic!("expected project-validation command");
        };

        assert!(matches!(
            args.rescan_vlm_provider,
            RescanVlmProviderArg::AutoLocal
        ));
        assert!(args.rescan_vlm_model.is_none());
    }

    #[test]
    fn corpus_kb_defaults_to_repo_root_current_directory() {
        let cli = Cli::parse_from([
            "specforge",
            "corpus-kb",
            "generated/intent_ir/doc/validation_report.json",
        ]);
        let Commands::CorpusKb(args) = cli.command else {
            panic!("expected corpus-kb command");
        };

        assert_eq!(args.repo_root, PathBuf::from("."));
        assert_eq!(args.kg_fixtures_root, None);
        assert!(args.kg_fixture.is_empty());
        assert_eq!(
            args.validation_reports,
            vec![PathBuf::from(
                "generated/intent_ir/doc/validation_report.json"
            )]
        );
    }

    #[test]
    fn corpus_kb_accepts_kg_fixture_refresh_without_validation_reports() {
        let cli = Cli::parse_from([
            "specforge",
            "corpus-kb",
            "--kg-fixtures-root",
            "crates/specforge/test_data/kg_quality",
            "--kg-fixture",
            "actor_ports_gold",
        ]);
        let Commands::CorpusKb(args) = cli.command else {
            panic!("expected corpus-kb command");
        };

        assert!(args.validation_reports.is_empty());
        assert_eq!(
            args.kg_fixtures_root,
            Some(PathBuf::from("crates/specforge/test_data/kg_quality"))
        );
        assert_eq!(args.kg_fixture, vec![PathBuf::from("actor_ports_gold")]);
    }

    #[test]
    fn doctor_defaults_to_non_strict() {
        let cli = Cli::parse_from(["specforge", "doctor"]);
        let Commands::Doctor(args) = cli.command else {
            panic!("expected doctor command");
        };

        assert!(!args.strict);
    }

    #[test]
    fn clean_defaults_to_source_normalized_dry_run() {
        let cli = Cli::parse_from(["specforge", "clean"]);
        let Commands::Clean(args) = cli.command else {
            panic!("expected clean command");
        };

        assert_eq!(args.generated_root, PathBuf::from("generated"));
        assert!(matches!(args.scope, CleanScopeArg::SourceNormalized));
        assert_eq!(args.document_key, None);
        assert!(!args.execute);
    }

    #[test]
    fn clean_accepts_all_generated_scope() {
        let cli = Cli::parse_from(["specforge", "clean", "--scope", "all-generated"]);
        let Commands::Clean(args) = cli.command else {
            panic!("expected clean command");
        };

        assert!(matches!(args.scope, CleanScopeArg::AllGenerated));
        assert_eq!(args.document_key, None);
    }
}
