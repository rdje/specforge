use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::CorpusKbArgs;
use crate::error::{AppError, Result};
use crate::ir::source::ValidationReportRecord;

const MANAGED_START: &str = "<!-- corpus_kb_validation_findings:start -->";
const MANAGED_END: &str = "<!-- corpus_kb_validation_findings:end -->";

#[derive(Debug)]
struct ValidationFindingProjection {
    report_path: PathBuf,
    display_path: String,
    document_key: String,
    report: ValidationReportRecord,
}

pub fn run(args: CorpusKbArgs) -> Result<()> {
    let page_path = refresh_validation_findings_page(&args.repo_root, &args.validation_reports)?;

    println!("command: corpus-kb");
    println!("refreshed_page: {}", page_path.display());
    println!("validation_reports: {}", args.validation_reports.len());

    Ok(())
}

fn refresh_validation_findings_page(
    repo_root: &Path,
    validation_report_paths: &[PathBuf],
) -> Result<PathBuf> {
    let entries = load_validation_report_projections(repo_root, validation_report_paths)?;
    let page_path = repo_root
        .join("corpus_kb")
        .join("failures")
        .join("validation-findings.md");
    if let Some(parent) = page_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let existing = fs::read_to_string(&page_path).unwrap_or_else(|_| default_validation_page());
    let managed_block = render_validation_findings_block(&entries);
    let updated = replace_managed_block(&existing, &managed_block)?;
    fs::write(&page_path, updated)?;

    Ok(page_path)
}

fn load_validation_report_projections(
    repo_root: &Path,
    validation_report_paths: &[PathBuf],
) -> Result<Vec<ValidationFindingProjection>> {
    let mut entries = Vec::new();
    for report_path in validation_report_paths {
        if !report_path.exists() {
            return Err(AppError::MissingPath(report_path.clone()));
        }
        let raw = fs::read_to_string(report_path)?;
        let report = serde_json::from_str::<ValidationReportRecord>(&raw)?;
        entries.push(ValidationFindingProjection {
            report_path: report_path.clone(),
            display_path: display_path(repo_root, report_path),
            document_key: document_key_from_report_path(report_path),
            report,
        });
    }
    entries.sort_by(|left, right| {
        left.document_key
            .cmp(&right.document_key)
            .then(left.report_path.cmp(&right.report_path))
    });
    Ok(entries)
}

fn render_validation_findings_block(entries: &[ValidationFindingProjection]) -> String {
    let mut output = String::new();
    output.push_str(MANAGED_START);
    output.push('\n');
    output.push_str("<!-- This block is refreshed by `specforge corpus-kb`. -->\n\n");

    if entries.is_empty() {
        output.push_str("- No validation reports were projected.\n\n");
    }

    for entry in entries {
        output.push_str("### ");
        output.push_str(&entry.document_key);
        output.push('\n');
        output.push_str("- report_path: `");
        output.push_str(&entry.display_path);
        output.push_str("`\n");
        output.push_str("- stage: `");
        output.push_str(entry.report.validated_stage.as_str());
        output.push_str("`\n");
        output.push_str("- artifact_fingerprint: `");
        output.push_str(&entry.report.artifact_fingerprint);
        output.push_str("`\n");
        if let Some(score) = entry.report.overall_score {
            output.push_str("- score: `");
            output.push_str(&score.to_string());
            if let Some(grade) = &entry.report.grade {
                output.push_str("/100 ");
                output.push_str(grade);
            }
            output.push_str("`\n");
        }
        output.push_str("- summary: ");
        output.push_str(&escape_markdown_line(&entry.report.summary));
        output.push('\n');
        output.push_str("- findings:\n");
        if entry.report.findings.is_empty() {
            output.push_str("  - none\n");
        } else {
            for finding in &entry.report.findings {
                output.push_str("  - [");
                output.push_str(finding.severity.as_str());
                output.push(':');
                output.push_str(&finding.category);
                output.push_str("] `");
                output.push_str(&finding.finding_id);
                output.push_str("` ");
                output.push_str(&escape_markdown_line(&finding.summary));
                output.push('\n');
            }
        }
        output.push('\n');
    }

    output.push_str(MANAGED_END);
    output.push('\n');
    output
}

fn replace_managed_block(existing: &str, managed_block: &str) -> Result<String> {
    let Some(start) = existing.find(MANAGED_START) else {
        let mut updated = existing.trim_end().to_string();
        updated.push_str("\n\n## Managed Validation Projection\n\n");
        updated.push_str(managed_block.trim_end());
        updated.push('\n');
        return Ok(updated);
    };
    let Some(relative_end) = existing[start..].find(MANAGED_END) else {
        return Err(AppError::InvalidStageArtifact(format!(
            "corpus knowledge page has `{MANAGED_START}` but no `{MANAGED_END}`"
        )));
    };
    let end = start + relative_end + MANAGED_END.len();
    let mut updated = String::new();
    updated.push_str(existing[..start].trim_end());
    updated.push_str("\n\n");
    updated.push_str(managed_block.trim_end());
    updated.push('\n');
    updated.push_str(existing[end..].trim_start_matches('\n'));
    Ok(updated)
}

fn default_validation_page() -> String {
    format!(
        "# Validation Finding Patterns\n\n\
This page is the first auto-refreshable page family in the `R15g` corpus knowledge base.\n\
It records validation-finding patterns from reviewable validation reports without promoting them into canonical document truth.\n\n\
## Human Synthesis\n\n\
Use this section for curated notes that explain recurring validation patterns across documents.\n\
Keep provenance explicit, and do not treat this page as an approval artifact.\n\n\
## Managed Validation Projection\n\n\
{MANAGED_START}\n{MANAGED_END}\n"
    )
}

fn document_key_from_report_path(report_path: &Path) -> String {
    report_path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        .unwrap_or("unknown_document")
        .to_string()
}

fn display_path(repo_root: &Path, path: &Path) -> String {
    let canonical_root = fs::canonicalize(repo_root).ok();
    let canonical_path = fs::canonicalize(path).ok();
    if let (Some(root), Some(path)) = (canonical_root, canonical_path)
        && let Ok(relative) = path.strip_prefix(root)
    {
        return relative.display().to_string();
    }
    path.display().to_string()
}

fn escape_markdown_line(value: &str) -> String {
    value.replace('\n', " ").replace('|', "\\|")
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::ir::IrStage;
    use crate::ir::source::{ValidationFindingRecord, ValidationFindingSeverity};

    #[test]
    fn corpus_kb_refresh_preserves_human_synthesis_and_replaces_managed_block() -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = tempdir.path();
        let report_dir = repo_root
            .join("generated")
            .join("intent_ir")
            .join("toy_protocol");
        fs::create_dir_all(&report_dir)?;
        let report_path = report_dir.join("validation_report.json");
        let report = ValidationReportRecord {
            report_id: "validation_intent_ir_test".to_string(),
            validated_stage: IrStage::IntentIr,
            artifact_fingerprint: "abc123".to_string(),
            summary: "IntentIR validation for toy protocol with 1 finding".to_string(),
            overall_score: Some(85),
            grade: Some("GOOD".to_string()),
            metrics: Vec::new(),
            findings: vec![ValidationFindingRecord {
                finding_id: "intent_quality_below_excellent_threshold".to_string(),
                severity: ValidationFindingSeverity::Warning,
                category: "quality_score".to_string(),
                summary: "IntentIR quality score is 85/100 (GOOD)".to_string(),
                related_ids: Vec::new(),
            }],
        };
        fs::write(&report_path, serde_json::to_string_pretty(&report)?)?;

        let page_path = repo_root
            .join("corpus_kb")
            .join("failures")
            .join("validation-findings.md");
        fs::create_dir_all(page_path.parent().expect("page has parent"))?;
        fs::write(
            &page_path,
            format!(
                "# Validation Finding Patterns\n\n\
## Human Synthesis\n\n\
Keep this curated note.\n\n\
## Managed Validation Projection\n\n\
{MANAGED_START}\nold generated content\n{MANAGED_END}\n"
            ),
        )?;

        refresh_validation_findings_page(repo_root, &[report_path])?;
        let refreshed = fs::read_to_string(page_path)?;

        assert!(refreshed.contains("Keep this curated note."));
        assert!(!refreshed.contains("old generated content"));
        assert!(refreshed.contains("### toy_protocol"));
        assert!(refreshed.contains("[warning:quality_score]"));
        assert!(refreshed.contains("`85/100 GOOD`"));

        Ok(())
    }
}
