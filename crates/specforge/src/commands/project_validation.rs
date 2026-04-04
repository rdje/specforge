use std::cmp::Reverse;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::cli::{ProjectValidationArgs, ValidateArgs};
use crate::commands::validate;
use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::EvidenceIr;
use crate::ir::intent::IntentIr;
use crate::ir::semantic::SemanticIr;
use crate::ir::source::{
    SourceIr, ValidationFindingRecord, ValidationFindingSeverity, ValidationReportRecord,
};

const VALIDATION_SNAPSHOT_DOC: &str = "VALIDATION_SNAPSHOT.md";
const LIVE_STATUS_DOC: &str = "LIVE_ACHIEVEMENT_STATUS.md";
const VALIDATION_PROJECTION_START: &str = "<!-- validation_projection:start -->";
const VALIDATION_PROJECTION_END: &str = "<!-- validation_projection:end -->";

#[derive(Debug, Clone)]
struct ProjectedArtifactSnapshot {
    document_key: String,
    display_name: String,
    stage: IrStage,
    artifact_path: PathBuf,
    report: ValidationReportRecord,
}

pub fn run(args: ProjectValidationArgs) -> Result<()> {
    let repo_root = canonicalize_existing_path(&args.repo_root)?;
    let live_status_path = repo_root.join(LIVE_STATUS_DOC);
    if !live_status_path.exists() {
        return Err(AppError::MissingPath(live_status_path));
    }

    let mut snapshots = Vec::new();
    for artifact in &args.artifacts {
        snapshots.push(project_artifact(artifact, &repo_root)?);
    }
    snapshots.sort_by(|left, right| {
        left.document_key
            .cmp(&right.document_key)
            .then_with(|| left.stage.as_str().cmp(right.stage.as_str()))
    });

    let snapshot_doc_path = repo_root.join(VALIDATION_SNAPSHOT_DOC);
    fs::write(
        &snapshot_doc_path,
        render_validation_snapshot_doc(&snapshots, &repo_root),
    )?;
    upsert_live_status_projection(&live_status_path, &snapshots, &repo_root)?;

    println!("command: project-validation");
    println!("repo_root: {}", repo_root.display());
    println!("projected_artifacts: {}", snapshots.len());
    println!("validation_snapshot_path: {}", snapshot_doc_path.display());
    println!("live_status_path: {}", live_status_path.display());

    Ok(())
}

fn project_artifact(artifact: &Path, repo_root: &Path) -> Result<ProjectedArtifactSnapshot> {
    let artifact_path = normalize_input_path(artifact, repo_root);
    validate::run(ValidateArgs {
        artifact: artifact_path.clone(),
    })?;

    #[derive(Deserialize)]
    struct StageProbe {
        stage: IrStage,
    }

    let raw = fs::read_to_string(&artifact_path)?;
    let probe: StageProbe = serde_json::from_str(&raw).map_err(|error| {
        AppError::InvalidStageArtifact(format!(
            "cannot determine stage from {}: {error}",
            artifact_path.display()
        ))
    })?;

    match probe.stage {
        IrStage::SourceIr => {
            let ir = SourceIr::load_from_path(&artifact_path)?;
            projected_snapshot(
                ir.document_identity.document_key,
                ir.document_identity.display_name,
                probe.stage,
                artifact_path,
                ir.validation_reports,
            )
        }
        IrStage::EvidenceIr => {
            let ir = EvidenceIr::load_from_path(&artifact_path)?;
            projected_snapshot(
                ir.document_identity.document_key,
                ir.document_identity.display_name,
                probe.stage,
                artifact_path,
                ir.validation_reports,
            )
        }
        IrStage::SemanticIr => {
            let ir = SemanticIr::load_from_path(&artifact_path)?;
            projected_snapshot(
                ir.document_identity.document_key,
                ir.document_identity.display_name,
                probe.stage,
                artifact_path,
                ir.validation_reports,
            )
        }
        IrStage::IntentIr => {
            let ir = IntentIr::load_from_path(&artifact_path)?;
            projected_snapshot(
                ir.document_identity.document_key,
                ir.document_identity.display_name,
                probe.stage,
                artifact_path,
                ir.validation_reports,
            )
        }
    }
}

fn projected_snapshot(
    document_key: String,
    display_name: String,
    stage: IrStage,
    artifact_path: PathBuf,
    validation_reports: Vec<ValidationReportRecord>,
) -> Result<ProjectedArtifactSnapshot> {
    let Some(report) = validation_reports.into_iter().next() else {
        return Err(AppError::InvalidStageArtifact(format!(
            "artifact at {} does not carry a persisted validation report after validation",
            artifact_path.display()
        )));
    };

    Ok(ProjectedArtifactSnapshot {
        document_key,
        display_name,
        stage,
        artifact_path,
        report,
    })
}

fn render_validation_snapshot_doc(
    snapshots: &[ProjectedArtifactSnapshot],
    repo_root: &Path,
) -> String {
    let mut lines = vec![
        "# VALIDATION_SNAPSHOT".to_string(),
        "This file is auto-refreshed by `specforge project-validation <artifact>...`.".to_string(),
        "It summarizes the latest persisted validation reports projected from IR artifacts into the tracked live-doc surface.".to_string(),
        String::new(),
        "## Snapshot Summary".to_string(),
        format!("- Artifacts projected: {}", snapshots.len()),
        format!(
            "- Highest severity observed: {}",
            highest_severity_label(snapshots)
        ),
    ];

    if snapshots.is_empty() {
        lines.push("- Score-bearing artifacts: none".to_string());
    } else {
        lines.push("- Score-bearing artifacts:".to_string());
        for snapshot in snapshots {
            lines.push(format!(
                "  - `{}` (`{}`): `{}`",
                snapshot.display_name,
                snapshot.stage.as_str(),
                score_summary(&snapshot.report)
            ));
        }
    }

    lines.push(String::new());
    lines.push("## Projected Artifacts".to_string());

    for snapshot in snapshots {
        lines.push(format!(
            "### {} ({})",
            snapshot.display_name,
            snapshot.stage.as_str()
        ));
        lines.push(format!("- document_key: `{}`", snapshot.document_key));
        lines.push(format!(
            "- artifact_path: `{}`",
            repo_relative_display(&snapshot.artifact_path, repo_root)
        ));
        lines.push(format!(
            "- artifact_fingerprint: `{}`",
            snapshot.report.artifact_fingerprint
        ));
        lines.push(format!("- score: `{}`", score_summary(&snapshot.report)));
        lines.push(format!("- summary: {}", snapshot.report.summary));
        lines.push("- findings:".to_string());
        if snapshot.report.findings.is_empty() {
            lines.push("  - none".to_string());
        } else {
            for finding in sorted_findings(&snapshot.report.findings) {
                lines.push(format!(
                    "  - [{}:{}] {}",
                    finding.severity.as_str(),
                    finding.category,
                    finding.summary
                ));
            }
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

fn render_live_status_projection(
    snapshots: &[ProjectedArtifactSnapshot],
    repo_root: &Path,
) -> String {
    let mut lines = vec!["- Latest projected validation snapshot:".to_string()];
    for snapshot in snapshots {
        lines.push(format!(
            "  - `{}` (`{}`): `{}` from `{}`",
            snapshot.display_name,
            snapshot.stage.as_str(),
            score_summary(&snapshot.report),
            repo_relative_display(&snapshot.artifact_path, repo_root)
        ));
    }
    lines.push("- Highest-signal projected findings:".to_string());
    for snapshot in snapshots {
        let finding_summary = primary_finding_summary(&snapshot.report);
        lines.push(format!(
            "  - `{}`: {}",
            snapshot.display_name, finding_summary
        ));
    }
    lines.join("\n")
}

fn upsert_live_status_projection(
    live_status_path: &Path,
    snapshots: &[ProjectedArtifactSnapshot],
    repo_root: &Path,
) -> Result<()> {
    let current = fs::read_to_string(live_status_path)?;
    let managed_block = format!(
        "{VALIDATION_PROJECTION_START}\n{}\n{VALIDATION_PROJECTION_END}",
        render_live_status_projection(snapshots, repo_root)
    );
    let updated = replace_or_append_managed_section(
        &current,
        VALIDATION_PROJECTION_START,
        VALIDATION_PROJECTION_END,
        &managed_block,
    )?;
    fs::write(live_status_path, updated)?;
    Ok(())
}

fn replace_or_append_managed_section(
    input: &str,
    start_marker: &str,
    end_marker: &str,
    managed_block: &str,
) -> Result<String> {
    match (input.find(start_marker), input.find(end_marker)) {
        (Some(start), Some(end)) if end > start => {
            let before = &input[..start];
            let after = &input[end + end_marker.len()..];
            Ok(format!(
                "{}{}{}",
                before.trim_end(),
                if before.trim_end().is_empty() {
                    ""
                } else {
                    "\n\n"
                },
                managed_block.to_string() + after
            ))
        }
        (None, None) => {
            let mut output = input.trim_end().to_string();
            if !output.is_empty() {
                output.push_str("\n\n");
            }
            output.push_str("## Validation Projection\n");
            output.push_str(managed_block);
            output.push('\n');
            Ok(output)
        }
        _ => Err(AppError::InvalidBackendOutput(
            "malformed validation projection markers in LIVE_ACHIEVEMENT_STATUS.md".to_string(),
        )),
    }
}

fn sorted_findings(findings: &[ValidationFindingRecord]) -> Vec<&ValidationFindingRecord> {
    let mut sorted: Vec<_> = findings.iter().collect();
    sorted.sort_by_key(|finding| {
        (
            Reverse(severity_rank(finding.severity)),
            finding.category.clone(),
            finding.finding_id.clone(),
        )
    });
    sorted
}

fn primary_finding_summary(report: &ValidationReportRecord) -> String {
    sorted_findings(&report.findings)
        .into_iter()
        .next()
        .map(|finding| {
            format!(
                "[{}:{}] {}",
                finding.severity.as_str(),
                finding.category,
                finding.summary
            )
        })
        .unwrap_or_else(|| "none".to_string())
}

fn score_summary(report: &ValidationReportRecord) -> String {
    match (report.overall_score, report.grade.as_deref()) {
        (Some(score), Some(grade)) => format!("{score}/100 {grade}"),
        (Some(score), None) => format!("{score}/100"),
        (None, Some(grade)) => grade.to_string(),
        (None, None) => report.summary.clone(),
    }
}

fn highest_severity_label(snapshots: &[ProjectedArtifactSnapshot]) -> &'static str {
    let rank = snapshots
        .iter()
        .flat_map(|snapshot| snapshot.report.findings.iter())
        .map(|finding| severity_rank(finding.severity))
        .max()
        .unwrap_or(0);
    match rank {
        3 => "error",
        2 => "warning",
        1 => "info",
        _ => "none",
    }
}

fn severity_rank(severity: ValidationFindingSeverity) -> u8 {
    match severity {
        ValidationFindingSeverity::Info => 1,
        ValidationFindingSeverity::Warning => 2,
        ValidationFindingSeverity::Error => 3,
    }
}

fn normalize_input_path(path: &Path, repo_root: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        repo_root.join(path)
    }
}

fn repo_relative_display(path: &Path, repo_root: &Path) -> String {
    let absolute = normalize_input_path(path, repo_root);
    absolute
        .strip_prefix(repo_root)
        .unwrap_or(absolute.as_path())
        .display()
        .to_string()
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;
    use crate::ir::evidence::EvidenceIr;
    use crate::ir::intent::IntentIr;
    use crate::ir::semantic::SemanticIr;
    use crate::ir::source::SourceIr;

    #[test]
    fn project_validation_writes_snapshot_doc_and_updates_live_status() -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = tempdir.path();
        fs::write(
            repo_root.join(LIVE_STATUS_DOC),
            "# LIVE_ACHIEVEMENT_STATUS\n## Current snapshot\n- placeholder: Done\n",
        )?;

        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        let intent_artifact_base = repo_root.join("generated").join("intent_ir");
        fs::write(
            &source,
            "# Spec\nSignal DATA_IN is input width 8.\n\nSignal DATA_OUT is output width 8.\n\nBlock pass: DATA_OUT = DATA_IN.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        intent_ir.write_to_disk()?;

        run(ProjectValidationArgs {
            artifacts: vec![intent_ir.artifact_layout.intent_ir_path.clone()],
            repo_root: repo_root.to_path_buf(),
        })?;

        let snapshot_doc = fs::read_to_string(repo_root.join(VALIDATION_SNAPSHOT_DOC))?;
        assert!(snapshot_doc.contains("spec.md"));
        assert!(snapshot_doc.contains("intent_ir"));
        assert!(snapshot_doc.contains("35/100 NEEDS IMPROVEMENT"));

        let live_status = fs::read_to_string(repo_root.join(LIVE_STATUS_DOC))?;
        assert!(live_status.contains("## Validation Projection"));
        assert!(live_status.contains(VALIDATION_PROJECTION_START));
        assert!(live_status.contains("35/100 NEEDS IMPROVEMENT"));

        let reloaded = IntentIr::load_from_path(&intent_ir.artifact_layout.intent_ir_path)?;
        assert_eq!(reloaded.validation_reports.len(), 1);

        Ok(())
    }
}
