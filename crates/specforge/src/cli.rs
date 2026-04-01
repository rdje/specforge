use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

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
