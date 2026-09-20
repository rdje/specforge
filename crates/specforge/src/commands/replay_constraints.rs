//! `replay-constraints` — EXTRACTION-QUALITY-GAUGE.3k.6: does the CURRENT producer still mint the
//! deterministic constraint records a persisted EvidenceIR carries?
//!
//! The persisted corpus is not one code generation. Only 24 of the 78 documents keep a normalized
//! bundle, so the rest cannot have their evidence stage re-run and their artifacts are frozen at
//! whatever generation wrote them. A census over `generated/` therefore measures what SpecForge
//! PUBLISHED, which is a different number from what today's extractor does — and sizing an extractor
//! change on the first number is how `EXTRACTION-QUALITY-GAUGE.3k.1` came to ship a gate whose four
//! measured instances the current code already refused for an unrelated reason.
//!
//! This command closes that gap without rebuilding anything. The deterministic constraint surface is
//! a function of the artifact's own `extracted_statements`, not of the source document, so it can be
//! replayed offline for every document including the 54 that cannot be rebuilt. It runs the REAL
//! producer — never a re-implementation, which would answer a question about itself
//! (`CLAIM_VERIFICATION.md` §2).
//!
//! Read-only: no provider, no network, no write. The artifact is loaded and not written back, so the
//! proof context is untouched (unlike `validate`, which appends a back-annotation mutation).

use std::path::{Path, PathBuf};

use crate::cli::ReplayConstraintsArgs;
use crate::error::{AppError, Result};
use crate::ir::evidence::{
    ConstraintReplayReport, EvidenceIr, replay_persisted_signal_constraints,
};
use crate::ir::source::SourceIr;

pub fn run(args: ReplayConstraintsArgs) -> Result<()> {
    match (args.evidence_ir, args.evidence_root) {
        (Some(_), Some(_)) => Err(AppError::InvalidStageArtifact(
            "replay-constraints takes either one EvidenceIR path or --evidence-root, not both"
                .to_string(),
        )),
        (Some(path), None) => run_one(&path, args.json),
        (None, Some(root)) => run_corpus(&root, args.json),
        (None, None) => Err(AppError::InvalidStageArtifact(
            "replay-constraints needs an EvidenceIR path or --evidence-root".to_string(),
        )),
    }
}

fn load_report(
    path: &Path,
) -> Result<(ConstraintReplayReport, Option<SourceClassificationOrigin>)> {
    // `load_for_inspection`, not `load_from_path`: the frozen stratum this command exists to measure
    // is exactly the legacy/proofless artifacts the canonical loader refuses, and replaying a
    // statement set requires no proof — nothing here is written back or promoted.
    let ir = EvidenceIr::load_for_inspection(path)?;
    // EXTRACTION-QUALITY-GAUGE.3k.2g — the table-row producer needs the document's own SourceIr, and
    // the artifact already names it, repository-root-relative.
    let sibling = sibling_source_ir(&ir);
    let source_ir = sibling.as_ref().map(|(source_ir, _)| source_ir);
    Ok((
        replay_persisted_signal_constraints(
            &ir.extracted_statements,
            &ir.signal_constraints,
            &ir.signal_polarities,
            source_ir,
        ),
        sibling.as_ref().map(|(_, origin)| *origin),
    ))
}

/// The `SourceIr` the artifact itself names, with the typed table classifications the row producer
/// reads — carried by the artifact when it is current, re-derived when it is legacy.
///
/// `load_for_inspection` accepts a legacy artifact but **neutralizes every source classification to
/// `Unknown`**, so the row producer — which selects tables by `TableKind::SignalDescription` — saw
/// none and minted nothing. This used to answer `None` for that case rather than report an empty
/// result as if the document stated no row obligation, which is the silent zero this instrument
/// exists to retire. `LEGACY-SOURCE-RECLASSIFICATION.0` measured that the label recomputes exactly
/// from the artifact's own preserved structure, so the honest answer is no longer "unjudged" but
/// "judged from re-derived labels", and the report says which.
fn sibling_source_ir(ir: &EvidenceIr) -> Option<(SourceIr, SourceClassificationOrigin)> {
    let path = ir.source_ir_path.as_path();
    if !path.is_file() {
        return None;
    }
    let mut source_ir = SourceIr::load_for_inspection(path).ok()?;
    if source_ir.carries_canonical_source_classifications() {
        return Some((source_ir, SourceClassificationOrigin::Carried));
    }
    // `LEGACY-SOURCE-RECLASSIFICATION.1` — the write lives HERE, in the command, and that placement
    // is the leaf's whole answer. `.0` prototyped it as a core method and the compiled
    // information-flow graph refused: *"raw_evidence reaches semantic control outside its
    // registered region"*. Core now returns the classifier's verdict as a VALUE and this diagnostic
    // decides what to do with it — the shape every existing `diagnostics_only` region already has,
    // each pointing at a command rather than at a core mutator.
    //
    // Nothing here is written back, promoted, or granted authority: the copy is local to this
    // replay, and `carries_canonical_source_classifications` still answers `false` for the artifact.
    for table in &mut source_ir.structured_tables {
        table.table_kind = crate::ir::source::current_table_classification(table);
    }
    Some((source_ir, SourceClassificationOrigin::Rederived))
}

/// Where the table classifications the row replay used came from. Kept in the report because a
/// re-derived label is a CURRENT-CLASSIFIER fact about preserved structure, not a fact the artifact
/// records — `CORPUS-CHAIN-CURRENCY.11`'s rule that a legacy artifact is evidence about itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceClassificationOrigin {
    /// The artifact carries them and is current-schema.
    Carried,
    /// Re-derived by the current classifier from the artifact's own preserved table structure.
    Rederived,
}

fn run_one(path: &Path, json: bool) -> Result<()> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }
    let (report, origin) = load_report(path)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }
    print_report(&path.display().to_string(), &report);
    if origin == Some(SourceClassificationOrigin::Rederived) {
        println!(
            "row_stratum_classifications: RE-DERIVED from the artifact's own preserved table \
structure (legacy SourceIR; no canonical authority — LEGACY-SOURCE-RECLASSIFICATION.1)"
        );
    }
    Ok(())
}

/// Replay every document under an evidence root. An artifact the loader refuses is an honest, NAMED
/// skip with its reason, never a silent omission: a corpus figure that quietly drops the documents it
/// could not read is exactly the shape of number this instrument exists to retire.
fn run_corpus(root: &Path, json: bool) -> Result<()> {
    let mut documents: Vec<PathBuf> = std::fs::read_dir(root)?
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path().join("evidence_ir.json"))
        .filter(|path| path.is_file())
        .collect();
    documents.sort();

    let mut persisted_total = 0usize;
    let mut reproduced_total = 0usize;
    let mut granted_total = 0usize;
    let mut skipped: Vec<(String, String)> = Vec::new();
    let mut rows: Vec<(String, usize, usize)> = Vec::new();
    let mut row_judged = 0usize;
    let mut row_judged_rederived = 0usize;
    let mut row_unjudged = 0usize;
    // `LEGACY-SOURCE-RECLASSIFICATION.1` — the number the re-derivation actually buys. Flipping a
    // legacy document from unjudged to judged judges an EMPTY SET: measured, 0 of the 51 carry any
    // persisted `row_sigcon_*` record, so there is nothing there to reproduce or fail to reproduce.
    // What IS new is that the current row producer can be RUN on them at all, and what it mints is
    // a recall signal about documents that were unreadable. Publishing the stratum count without
    // this would be the inverse of the silent zero this instrument exists to retire.
    let mut rederived_replayed_records = 0usize;

    for path in &documents {
        let key = path
            .parent()
            .and_then(Path::file_name)
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        match load_report(path) {
            Ok((report, origin)) => {
                let reproduced = report.verdicts.iter().filter(|v| v.reproduced).count();
                persisted_total += report.persisted_total;
                reproduced_total += reproduced;
                granted_total += report.granted_declarations.len();
                if report.row_stratum_judged {
                    row_judged += 1;
                    if origin == Some(SourceClassificationOrigin::Rederived) {
                        row_judged_rederived += 1;
                        rederived_replayed_records += report.replayed_total;
                    }
                } else {
                    row_unjudged += 1;
                }
                if report.persisted_total > 0 {
                    rows.push((key, reproduced, report.persisted_total));
                }
            }
            Err(error) => skipped.push((key, error.to_string())),
        }
    }

    if json {
        let payload = serde_json::json!({
            "documents_scanned": documents.len(),
            "documents_with_records": rows.len(),
            "persisted_deterministic_records": persisted_total,
            "reproduced": reproduced_total,
            "not_reproduced": persisted_total - reproduced_total,
            "granted_declarations": granted_total,
            "row_stratum_judged_documents": row_judged,
            "row_stratum_judged_from_rederived_labels": row_judged_rederived,
            "rederived_stratum_replayed_records": rederived_replayed_records,
            "row_stratum_unjudged_documents": row_unjudged,
            "skipped": skipped
                .iter()
                .map(|(key, reason)| serde_json::json!({"document": key, "reason": reason}))
                .collect::<Vec<_>>(),
            "documents": rows
                .iter()
                .map(|(key, reproduced, persisted)| {
                    serde_json::json!({
                        "document": key,
                        "reproduced": reproduced,
                        "persisted": persisted,
                    })
                })
                .collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    println!("command: replay-constraints");
    println!("evidence_root: {}", root.display());
    println!("documents_scanned: {}", documents.len());
    println!("documents_skipped: {}", skipped.len());
    println!("persisted_deterministic_records: {persisted_total}");
    println!("reproduced: {reproduced_total}");
    println!("not_reproduced: {}", persisted_total - reproduced_total);
    println!("granted_declarations: {granted_total}");
    println!(
        "row_stratum_judged_documents: {row_judged} (the row producer could be replayed — its \
sibling SourceIR was readable and its table classifications were carried or re-derivable)"
    );
    println!(
        "row_stratum_judged_from_rederived_labels: {row_judged_rederived} (legacy SourceIR whose \
labels this diagnostic re-derived from the artifact's own preserved structure — a \
current-classifier fact carrying NO canonical authority). Judging these judges an EMPTY SET: none \
carries a persisted row_sigcon_* record, so read the next line, not this one"
    );
    println!(
        "rederived_stratum_replayed_records: {rederived_replayed_records} (records the CURRENT \
producer mints across those documents; nothing persisted to compare them against, so a recall \
signal and NOT a reproduction verdict)"
    );
    println!(
        "row_stratum_unjudged_documents: {row_unjudged} (no readable sibling SourceIR at all, so \
the row producer could not be replayed)"
    );
    for (key, reproduced, persisted) in &rows {
        if reproduced != persisted {
            println!("document: {key} {reproduced}/{persisted}");
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

fn print_report(input: &str, report: &ConstraintReplayReport) {
    let reproduced = report
        .verdicts
        .iter()
        .filter(|verdict| verdict.reproduced)
        .count();
    let missing = report.persisted_total - reproduced;

    println!("command: replay-constraints");
    println!("input: {input}");
    println!(
        "persisted_deterministic_records: {}",
        report.persisted_total
    );
    println!("replayed_records: {}", report.replayed_total);
    println!("reproduced: {reproduced}");
    println!("not_reproduced: {missing}");
    println!(
        "granted_declarations: {} (published subjects the artifact no longer declares, granted a \
declaration so their records could still be judged)",
        report.granted_declarations.len()
    );
    println!(
        "unpersisted_replay_records: {} (NOT a drift measure — the build applies convergence \
stages this replay does not, and this replay runs a widened catalog)",
        report.unpersisted_replay_records.len()
    );
    println!(
        "row_stratum_judged: {} ({})",
        report.row_stratum_judged,
        if report.row_stratum_judged {
            "row_sigcon_* replayed from the document's own SourceIr; prior guidance is not applied"
        } else {
            "no current-schema SourceIR — row_sigcon_* records are NOT judged here"
        }
    );

    for verdict in report.verdicts.iter().filter(|v| !v.reproduced) {
        let gates = if verdict.refused_by.is_empty() {
            "no positional gate refuses this subject — the kind, condition or negation moved"
                .to_string()
        } else {
            verdict.refused_by.join(", ")
        };
        println!(
            "not_reproduced: {} {} {}{} cond={:?} — {}",
            verdict.constraint_id,
            verdict.subject_signal,
            verdict.constraint_kind,
            if verdict.negated { " negated" } else { "" },
            verdict.condition_text.as_deref().unwrap_or(""),
            gates
        );
    }
    for name in &report.granted_declarations {
        println!("granted_declaration: {name}");
    }
    for verdict in &report.unpersisted_replay_records {
        println!(
            "unpersisted_replay_record: {} {} {}{} cond={:?} src={:?}",
            verdict.constraint_id,
            verdict.subject_signal,
            verdict.constraint_kind,
            if verdict.negated { " negated" } else { "" },
            verdict.condition_text.as_deref().unwrap_or(""),
            excerpt(&verdict.source_text)
        );
    }
}
