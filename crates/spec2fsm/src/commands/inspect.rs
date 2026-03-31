use std::fs;

use crate::cli::InspectArgs;
use crate::error::{AppError, Result};
use crate::source::SourceKind;

pub fn run(args: InspectArgs) -> Result<()> {
    if !args.path.exists() {
        return Err(AppError::MissingPath(args.path));
    }

    let metadata = fs::metadata(&args.path)?;
    let canonical = fs::canonicalize(&args.path)?;
    let path_kind = if metadata.is_file() {
        "file"
    } else if metadata.is_dir() {
        "directory"
    } else {
        "other"
    };
    let extension = args
        .path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("none");
    let source_kind = SourceKind::detect(&args.path);

    println!("command: inspect");
    println!("input: {}", args.path.display());
    println!("canonical: {}", canonical.display());
    println!("exists: yes");
    println!("path_kind: {path_kind}");
    println!("detected_source_kind: {}", source_kind.as_str());
    println!("extension: {extension}");

    if metadata.is_file() {
        println!("size_bytes: {}", metadata.len());
    }

    Ok(())
}
