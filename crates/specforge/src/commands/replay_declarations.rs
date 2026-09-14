//! `replay-declarations` — SIGNAL-DECLARATION-ROW-DROP.4e: what does the CURRENT SemanticIR
//! declaration reader do with the `Signal <name> …` statements a persisted EvidenceIR carries?
//!
//! The sibling of [`crate::commands::replay_constraints`], and it exists for the same measured
//! reason. A census over `generated/` reports what SpecForge PUBLISHED; for 51 of the 78 persisted
//! documents that is an older binary's output, and `specforge semantic` refuses their EvidenceIR
//! outright as legacy/proofless. `SIGNAL-DECLARATION-ROW-DROP.4b` measured the consequence: of the
//! 75 signals declared in EvidenceIR and absent from the persisted SemanticIR catalog, **74 live in
//! documents the current chain cannot reproduce**, so the only two honest ways to see them are
//! re-ingest and this replay.
//!
//! The declaration surface is a pure function of the artifact's own `extracted_statements` — no
//! `SourceIr`, no proof context — so it replays offline for every document. It runs the REAL reader
//! (`read_explicit_signal_declaration`), never a re-implementation, which would answer a question
//! about itself (`CLAIM_VERIFICATION.md` §2).
//!
//! `unrecovered` is recomputed from THIS replay's own read declarations and never joined against the
//! persisted SemanticIR catalog: that catalog is exactly the stale output this instrument exists to
//! stop standing in for the current reader.
//!
//! Read-only: no provider, no network, no write. The artifact is loaded for inspection and never
//! written back, so its proof context is untouched (unlike `validate`, which appends a mutation).

use std::path::{Path, PathBuf};

use crate::cli::ReplayDeclarationsArgs;
use crate::error::{AppError, Result};
use crate::ir::evidence::EvidenceIr;
use crate::ir::semantic::{DeclarationReplayReport, replay_persisted_signal_declarations};

pub fn run(args: ReplayDeclarationsArgs) -> Result<()> {
    match (args.evidence_ir, args.evidence_root) {
        (Some(_), Some(_)) => Err(AppError::InvalidStageArtifact(
            "replay-declarations takes either one EvidenceIR path or --evidence-root, not both"
                .to_string(),
        )),
        (Some(path), None) => run_one(&path, args.json),
        (None, Some(root)) => run_corpus(&root, args.json),
        (None, None) => Err(AppError::InvalidStageArtifact(
            "replay-declarations needs an EvidenceIR path or --evidence-root".to_string(),
        )),
    }
}

/// `load_for_inspection`, not `load_from_path`: the frozen stratum this command exists to measure is
/// exactly the legacy/proofless artifacts the canonical loader refuses, and replaying a statement
/// set requires no proof — nothing here is written back or promoted.
fn load_report(path: &Path) -> Result<DeclarationReplayReport> {
    let ir = EvidenceIr::load_for_inspection(path)?;
    Ok(replay_persisted_signal_declarations(
        &ir.extracted_statements,
    ))
}

fn run_one(path: &Path, json: bool) -> Result<()> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }
    let report = load_report(path)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }
    print_report(&path.display().to_string(), &report);
    Ok(())
}

/// Replay every document under an evidence root. An artifact the loader refuses is an honest, NAMED
/// skip with its reason, never a silent omission: a corpus figure that quietly drops the documents
/// it could not read is exactly the shape of number this instrument exists to retire.
fn run_corpus(root: &Path, json: bool) -> Result<()> {
    let mut documents: Vec<PathBuf> = std::fs::read_dir(root)?
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path().join("evidence_ir.json"))
        .filter(|path| path.is_file())
        .collect();
    documents.sort();

    let mut opened_total = 0usize;
    let mut read_total = 0usize;
    let mut refused_total = 0usize;
    let mut unrecovered_total = 0usize;
    let mut by_reason: std::collections::BTreeMap<&'static str, usize> =
        std::collections::BTreeMap::new();
    let mut skipped: Vec<(String, String)> = Vec::new();
    let mut rows: Vec<(String, DeclarationReplayReport)> = Vec::new();

    for path in &documents {
        let key = path
            .parent()
            .and_then(Path::file_name)
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        match load_report(path) {
            Ok(report) => {
                opened_total += report.opened_total;
                read_total += report.read_total;
                refused_total += report.refusals.len();
                unrecovered_total += report.unrecovered_names.len();
                for (reason, count) in &report.refusals_by_reason {
                    *by_reason.entry(reason).or_insert(0) += count;
                }
                if !report.refusals.is_empty() {
                    rows.push((key, report));
                }
            }
            Err(error) => skipped.push((key, error.to_string())),
        }
    }

    if json {
        let payload = serde_json::json!({
            "documents_scanned": documents.len(),
            "documents_with_refusals": rows.len(),
            "opened_total": opened_total,
            "read_total": read_total,
            "refused_total": refused_total,
            "unrecovered_total": unrecovered_total,
            "refusals_by_reason": by_reason,
            "skipped": skipped
                .iter()
                .map(|(key, reason)| serde_json::json!({"document": key, "reason": reason}))
                .collect::<Vec<_>>(),
            "documents": rows
                .iter()
                .map(|(key, report)| serde_json::json!({"document": key, "report": report}))
                .collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    println!("command: replay-declarations");
    println!("evidence_root: {}", root.display());
    println!("documents_scanned: {}", documents.len());
    println!("documents_skipped: {}", skipped.len());
    println!("opened_total: {opened_total} (sentences that opened as `Signal <name> …`)");
    println!("read_total: {read_total}");
    println!("refused_total: {refused_total}");
    println!(
        "unrecovered_total: {unrecovered_total} (refused identities no read declaration in the \
same document mints)"
    );
    for (reason, count) in &by_reason {
        println!("  {reason}: {count}");
    }
    for (key, report) in &rows {
        println!(
            "document: {key} opened={} read={} refused={} unrecovered={}",
            report.opened_total,
            report.read_total,
            report.refusals.len(),
            report.unrecovered_names.len()
        );
        for refusal in &report.refusals {
            let name = refusal.signal_name.as_deref().unwrap_or("<no identifier>");
            let recovered = refusal
                .signal_name
                .as_ref()
                .is_some_and(|signal| report.read_names.contains(signal));
            println!(
                "  {}: {name}{} <- {}",
                refusal.reason,
                if recovered { " (read elsewhere)" } else { "" },
                excerpt(&refusal.declaration_text)
            );
        }
    }
    for (key, reason) in &skipped {
        println!("skipped: {key} — {reason}");
    }
    Ok(())
}

/// A bounded provenance excerpt: enough to find the sentence, short enough that a corpus report
/// stays readable. Character-bounded, not byte-bounded, so a multi-byte source never splits.
fn excerpt(text: &str) -> String {
    const LIMIT: usize = 140;
    let trimmed = text.trim();
    if trimmed.chars().count() <= LIMIT {
        return trimmed.to_string();
    }
    let head: String = trimmed.chars().take(LIMIT).collect();
    format!("{head}…")
}

fn print_report(input: &str, report: &DeclarationReplayReport) {
    println!("command: replay-declarations");
    println!("input: {input}");
    println!(
        "opened: {} (sentences that opened as `Signal <name> …`)",
        report.opened_total
    );
    println!("read: {}", report.read_total);
    println!("distinct_read_names: {}", report.read_names.len());
    println!("refused: {}", report.refusals.len());
    println!(
        "unrecovered: {} (refused identities no read declaration in this document mints; computed \
from THIS replay, never from the persisted SemanticIR catalog)",
        report.unrecovered_names.len()
    );
    for (reason, count) in &report.refusals_by_reason {
        println!("  {reason}: {count}");
    }
    for refusal in &report.refusals {
        let name = refusal.signal_name.as_deref().unwrap_or("<no identifier>");
        let recovered = refusal
            .signal_name
            .as_ref()
            .is_some_and(|signal| report.read_names.contains(signal));
        println!(
            "refused: {} {name}{} [{}] <- {}",
            refusal.reason,
            if recovered { " (read elsewhere)" } else { "" },
            refusal.statement_id,
            excerpt(&refusal.declaration_text)
        );
    }
}
