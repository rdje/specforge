//! Recapture the table premise of persisted SourceIR artifacts from their retained converter
//! documents, re-running only the ingest helper's table-grid extraction (no Docling model), then
//! refresh each proof. Without `--write` it reports what would change and writes nothing.

use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use specforge::ir::source::SourceIr;

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

fn repository_relative(path: PathBuf, what: &str) -> io::Result<PathBuf> {
    if path.is_absolute() {
        return Err(invalid_input(format!(
            "{what} must be repository-root-relative: {}",
            path.display()
        )));
    }
    Ok(path)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut write = false;
    let mut paths = Vec::new();
    let mut arguments = env::args_os().skip(1);
    while let Some(argument) = arguments.next() {
        if argument == "--write" {
            if write {
                return Err(invalid_input("--write may be supplied only once").into());
            }
            write = true;
            continue;
        }
        if argument == "--retained-manifest" {
            let manifest_path = repository_relative(
                PathBuf::from(arguments.next().ok_or_else(|| {
                    invalid_input("--retained-manifest requires a repository-relative JSON path")
                })?),
                "the retained manifest",
            )?;
            let manifest: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(&manifest_path)?)?;
            let retained = manifest["retained"].as_array().ok_or_else(|| {
                invalid_input(format!(
                    "retained manifest {} lacks a retained array",
                    manifest_path.display()
                ))
            })?;
            for key in retained {
                let key = key
                    .as_str()
                    .ok_or_else(|| invalid_input("retained manifest entries must be strings"))?;
                paths.push(
                    PathBuf::from("generated")
                        .join("source_ir")
                        .join(key)
                        .join("source_ir.json"),
                );
            }
            continue;
        }
        paths.push(repository_relative(
            PathBuf::from(argument),
            "a SourceIR path",
        )?);
    }
    if paths.is_empty() {
        return Err(invalid_input(
            "usage: cargo run -p specforge --features source-proof-migration \
             --example source_table_recapture -- [--write] \
             [--retained-manifest <manifest.json>] <source_ir.json>...",
        )
        .into());
    }

    let stdout = io::stdout();
    let mut output = stdout.lock();
    for path in paths {
        let (rebuilt, changed) = SourceIr::recapture_tables_from_retained_converter(&path)?;
        let status = match (changed.is_empty(), write) {
            (true, _) => "unchanged",
            (false, true) => {
                rebuilt.write_to_disk()?;
                "wrote"
            }
            (false, false) => "would-change",
        };
        writeln!(
            output,
            "{status}\t{}\t{}\t{}",
            path.display(),
            rebuilt.document_identity.document_key,
            changed.join(",")
        )?;
    }
    Ok(())
}
