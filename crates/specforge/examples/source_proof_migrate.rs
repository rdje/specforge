use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use specforge::ir::source::SourceIr;

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
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
            let manifest_path = PathBuf::from(arguments.next().ok_or_else(|| {
                invalid_input("--retained-manifest requires a repository-relative JSON path")
            })?);
            if manifest_path.is_absolute() {
                return Err(invalid_input(format!(
                    "migration manifest must be repository-root-relative: {}",
                    manifest_path.display()
                ))
                .into());
            }
            let manifest: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(&manifest_path)?)?;
            let retained = manifest["retained"].as_array().ok_or_else(|| {
                invalid_input(format!(
                    "migration manifest {} lacks a retained array",
                    manifest_path.display()
                ))
            })?;
            for key in retained {
                let key = key.as_str().ok_or_else(|| {
                    invalid_input("migration manifest retained entries must be strings")
                })?;
                paths.push(
                    PathBuf::from("generated")
                        .join("source_ir")
                        .join(key)
                        .join("source_ir.json"),
                );
            }
            continue;
        }
        let path = PathBuf::from(argument);
        if path.is_absolute() {
            return Err(invalid_input(format!(
                "migration paths must be repository-root-relative: {}",
                path.display()
            ))
            .into());
        }
        paths.push(path);
    }
    if paths.is_empty() {
        return Err(invalid_input(
            "usage: cargo run -p specforge --features source-proof-migration \
             --example source_proof_migrate -- [--write] \
             [--retained-manifest <manifest.json>] <source_ir.json>...",
        )
        .into());
    }

    let stdout = io::stdout();
    let mut output = stdout.lock();
    for path in paths {
        let rebuilt = SourceIr::rebuild_from_retained_capture(&path)?;
        if write {
            rebuilt.write_to_disk()?;
        }
        writeln!(
            output,
            "{}\t{}\t{}\t{}",
            if write { "wrote" } else { "verified" },
            path.display(),
            rebuilt.document_identity.document_key,
            rebuilt
                .proof_ledger()
                .map(|ledger| ledger.claims().len())
                .unwrap_or(0)
        )?;
    }
    Ok(())
}
