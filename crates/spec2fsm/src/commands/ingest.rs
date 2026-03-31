use std::fs;
use std::path::Path;

use crate::cli::IngestArgs;
use crate::error::{AppError, Result};
use crate::source::SourceKind;

pub fn run(args: IngestArgs) -> Result<()> {
    if !args.source.exists() {
        return Err(AppError::MissingPath(args.source));
    }

    if !args.dry_run {
        return Err(AppError::FeatureNotYetImplemented(
            "non-dry-run ingest; rerun with --dry-run for the current bootstrap command",
        ));
    }

    let canonical = fs::canonicalize(&args.source)?;
    let source_kind = SourceKind::detect(&args.source);
    let stable_stem = stable_stem(&args.source);

    println!("command: ingest");
    println!("mode: dry-run");
    println!("source: {}", args.source.display());
    println!("canonical: {}", canonical.display());
    println!("source_kind: {}", source_kind.as_str());
    println!("stable_artifact_stem: {stable_stem}");
    println!("planned_steps:");
    println!("- register source manifest");

    if source_kind.requires_markdown_conversion() {
        println!("- convert source to markdown");
        println!("- preserve converter metadata and auxiliary assets");
        println!("- promote a stable markdown path for downstream stages");
    } else {
        println!("- classify source as already-normalized or non-pdf input");
        println!("- skip conversion if an explicit markdown source is already provided");
    }

    println!("- record evidence and section-map preparation inputs");
    println!("- prepare typed ingest result for later extraction stages");

    Ok(())
}

fn stable_stem(path: &Path) -> String {
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::stable_stem;

    #[test]
    fn stable_stem_uses_file_stem_for_files() {
        assert_eq!(stable_stem(Path::new("foo/bar/spec.pdf")), "spec");
    }
}
