use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "spec2fsm",
    version,
    about = "Staged protocol-spec extraction CLI bootstrap for SpecForge"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Inspect a source path and report the detected source kind
    Inspect(InspectArgs),
    /// Plan a source ingest run; currently dry-run only
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
    /// Do not modify anything; print the planned ingest actions instead
    #[arg(long)]
    pub dry_run: bool,
}
