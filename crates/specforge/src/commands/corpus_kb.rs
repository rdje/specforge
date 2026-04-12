use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::CorpusKbArgs;
use crate::commands::kg_bench::{self, KgBenchFixtureOutcome};
use crate::error::{AppError, Result};
use crate::ir::source::ValidationReportRecord;

const VALIDATION_MANAGED_START: &str = "<!-- corpus_kb_validation_findings:start -->";
const VALIDATION_MANAGED_END: &str = "<!-- corpus_kb_validation_findings:end -->";
const KG_FIXTURES_MANAGED_START: &str = "<!-- corpus_kb_kg_fixtures:start -->";
const KG_FIXTURES_MANAGED_END: &str = "<!-- corpus_kb_kg_fixtures:end -->";

#[derive(Debug)]
struct ValidationFindingProjection {
    report_path: PathBuf,
    display_path: String,
    document_key: String,
    report: ValidationReportRecord,
}

#[derive(Debug)]
struct KgFixtureProjection {
    outcome: KgBenchFixtureOutcome,
    display_path: String,
}

#[derive(Debug)]
struct KgFixturesRefresh {
    page_path: PathBuf,
    fixture_count: usize,
    failed_count: usize,
}

#[derive(Debug)]
struct KgFixtureFamilySummary {
    label: String,
    fixture_count: usize,
    passed_count: usize,
    failed_count: usize,
    failed_fixture_names: Vec<String>,
}

pub fn run(args: CorpusKbArgs) -> Result<()> {
    if args.validation_reports.is_empty() && args.kg_fixtures_root.is_none() {
        return Err(AppError::InvalidStageArtifact(
            "corpus-kb requires at least one validation report or --kg-fixtures-root".to_string(),
        ));
    }
    if !args.kg_fixture.is_empty() && args.kg_fixtures_root.is_none() {
        return Err(AppError::InvalidStageArtifact(
            "--kg-fixture requires --kg-fixtures-root".to_string(),
        ));
    }

    println!("command: corpus-kb");

    if !args.validation_reports.is_empty() {
        let page_path =
            refresh_validation_findings_page(&args.repo_root, &args.validation_reports)?;
        println!("refreshed_page: {}", page_path.display());
        println!("validation_reports: {}", args.validation_reports.len());
    }

    if let Some(fixtures_root) = args.kg_fixtures_root.as_deref() {
        let refresh = refresh_kg_fixtures_page(&args.repo_root, fixtures_root, &args.kg_fixture)?;
        println!("refreshed_page: {}", refresh.page_path.display());
        println!("kg_fixtures: {}", refresh.fixture_count);
        println!("kg_fixtures_failed: {}", refresh.failed_count);
        if refresh.failed_count > 0 {
            return Err(AppError::InvalidStageArtifact(format!(
                "corpus KB benchmark projection captured {} failing KG fixture(s)",
                refresh.failed_count
            )));
        }
    }

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
    let updated = replace_managed_block(
        &existing,
        &managed_block,
        VALIDATION_MANAGED_START,
        VALIDATION_MANAGED_END,
        "## Managed Validation Projection",
    )?;
    fs::write(&page_path, updated)?;

    Ok(page_path)
}

fn refresh_kg_fixtures_page(
    repo_root: &Path,
    fixtures_root: &Path,
    requested_fixtures: &[PathBuf],
) -> Result<KgFixturesRefresh> {
    let (_, outcomes) = kg_bench::collect_fixture_outcomes(fixtures_root, requested_fixtures)?;
    let failed_count = outcomes
        .iter()
        .filter(|outcome| !outcome.failures.is_empty())
        .count();
    let entries = outcomes
        .into_iter()
        .map(|outcome| {
            let display_path = display_path(repo_root, &outcome.fixture_path);
            KgFixtureProjection {
                outcome,
                display_path,
            }
        })
        .collect::<Vec<_>>();
    let page_path = repo_root
        .join("corpus_kb")
        .join("benchmarks")
        .join("kg-fixtures.md");
    if let Some(parent) = page_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let existing = fs::read_to_string(&page_path).unwrap_or_else(|_| default_kg_fixtures_page());
    let managed_block = render_kg_fixtures_block(&entries);
    let updated = replace_managed_block(
        &existing,
        &managed_block,
        KG_FIXTURES_MANAGED_START,
        KG_FIXTURES_MANAGED_END,
        "## Managed KG Benchmark Projection",
    )?;
    fs::write(&page_path, updated)?;

    Ok(KgFixturesRefresh {
        page_path,
        fixture_count: entries.len(),
        failed_count,
    })
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
    output.push_str(VALIDATION_MANAGED_START);
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

    output.push_str(VALIDATION_MANAGED_END);
    output.push('\n');
    output
}

fn render_kg_fixtures_block(entries: &[KgFixtureProjection]) -> String {
    let mut output = String::new();
    output.push_str(KG_FIXTURES_MANAGED_START);
    output.push('\n');
    output.push_str("<!-- This block is refreshed by `specforge corpus-kb`. -->\n\n");

    let passed = entries
        .iter()
        .filter(|entry| entry.outcome.failures.is_empty())
        .count();
    let failed = entries.len().saturating_sub(passed);
    output.push_str("- fixtures_total: `");
    output.push_str(&entries.len().to_string());
    output.push_str("`\n");
    output.push_str("- fixtures_passed: `");
    output.push_str(&passed.to_string());
    output.push_str("`\n");
    output.push_str("- fixtures_failed: `");
    output.push_str(&failed.to_string());
    output.push_str("`\n\n");

    let family_summaries = kg_fixture_family_summaries(entries);
    if !family_summaries.is_empty() {
        output.push_str("### Fixture Family Summary\n");
        output.push_str(
            "Fixtures can appear in more than one family because protocol semantics, modality, and expected behavior are orthogonal.\n\n",
        );
        output.push_str("| family | fixtures | passed | failed |\n");
        output.push_str("| --- | ---: | ---: | ---: |\n");
        for summary in &family_summaries {
            output.push_str("| ");
            output.push_str(&escape_markdown_line(&summary.label));
            output.push_str(" | `");
            output.push_str(&summary.fixture_count.to_string());
            output.push_str("` | `");
            output.push_str(&summary.passed_count.to_string());
            output.push_str("` | `");
            output.push_str(&summary.failed_count.to_string());
            output.push_str("` |\n");
        }
        output.push('\n');

        let failed_summaries = family_summaries
            .iter()
            .filter(|summary| !summary.failed_fixture_names.is_empty())
            .collect::<Vec<_>>();
        if !failed_summaries.is_empty() {
            output.push_str("Failed fixture family members:\n");
            for summary in failed_summaries {
                output.push_str("- ");
                output.push_str(&escape_markdown_line(&summary.label));
                output.push_str(": `");
                output.push_str(&summary.failed_fixture_names.join("`, `"));
                output.push_str("`\n");
            }
            output.push('\n');
        }
    }

    if entries.is_empty() {
        output.push_str("- No KG fixtures were projected.\n\n");
    }

    for entry in entries {
        output.push_str("### ");
        output.push_str(&entry.outcome.name);
        output.push('\n');
        output.push_str("- fixture_path: `");
        output.push_str(&entry.display_path);
        output.push_str("`\n");
        output.push_str("- status: `");
        output.push_str(if entry.outcome.failures.is_empty() {
            "pass"
        } else {
            "fail"
        });
        output.push_str("`\n");
        output.push_str("- failures:\n");
        if entry.outcome.failures.is_empty() {
            output.push_str("  - none\n");
        } else {
            for failure in &entry.outcome.failures {
                output.push_str("  - ");
                output.push_str(&escape_markdown_line(failure));
                output.push('\n');
            }
        }
        output.push('\n');
    }

    output.push_str(KG_FIXTURES_MANAGED_END);
    output.push('\n');
    output
}

fn kg_fixture_family_summaries(entries: &[KgFixtureProjection]) -> Vec<KgFixtureFamilySummary> {
    let mut summaries = BTreeMap::<String, KgFixtureFamilySummary>::new();
    for entry in entries {
        for label in kg_fixture_family_labels(&entry.outcome.name) {
            let summary =
                summaries
                    .entry(label.to_string())
                    .or_insert_with(|| KgFixtureFamilySummary {
                        label: label.to_string(),
                        fixture_count: 0,
                        passed_count: 0,
                        failed_count: 0,
                        failed_fixture_names: Vec::new(),
                    });
            summary.fixture_count += 1;
            if entry.outcome.failures.is_empty() {
                summary.passed_count += 1;
            } else {
                summary.failed_count += 1;
                summary
                    .failed_fixture_names
                    .push(entry.outcome.name.clone());
            }
        }
    }
    summaries.into_values().collect()
}

fn kg_fixture_family_labels(name: &str) -> BTreeSet<&'static str> {
    let mut labels = BTreeSet::new();
    let normalized = name.to_ascii_lowercase();

    if normalized.contains("actor")
        || normalized.contains("producer")
        || normalized.contains("source_column")
        || normalized.contains("destination_column")
        || normalized.contains("direction")
        || normalized.contains("connectivity")
        || normalized.contains("ports")
    {
        labels.insert("actor connectivity");
    }
    if normalized.starts_with("amba_")
        || normalized.starts_with("apb_")
        || normalized.starts_with("ahb_")
        || normalized.starts_with("axi_")
        || normalized.contains("source_column")
        || normalized.contains("destination_column")
    {
        labels.insert("protocol-family AMBA/APB/AHB/AXI");
    }
    if normalized.contains("semantic")
        || normalized.contains("handshake")
        || normalized.contains("alias_dependent")
        || normalized.contains("name_only")
        || normalized.contains("modality_reliability")
    {
        labels.insert("semantic role arbitration");
    }
    if normalized.contains("timing")
        || normalized.contains("temporal")
        || normalized.contains("cycle")
        || normalized.contains("wait_state")
    {
        labels.insert("temporal semantics");
    }
    if normalized.contains("polarity")
        || normalized.contains("active_low")
        || normalized.contains("non_reset_control")
    {
        labels.insert("polarity semantics");
    }
    if normalized.contains("visual")
        || normalized.contains("vlm")
        || normalized.contains("cross_modality")
    {
        labels.insert("multimodal visual grounding");
    }
    if normalized.contains("vlm_timing") {
        labels.insert("VLM timing diagrams");
    }
    if normalized.contains("vlm_state_machine") {
        labels.insert("VLM state machines");
    }
    if normalized.contains("_prior_guided")
        || normalized.contains("negative_knowledge")
        || normalized.contains("modality_reliability")
        || normalized.contains("without_prior")
    {
        labels.insert("typed prior memory");
    }
    if normalized.contains("negative_knowledge") {
        labels.insert("negative knowledge");
    }
    if normalized.contains("table")
        || normalized.contains("source_column")
        || normalized.contains("destination_column")
    {
        labels.insert("table extraction and hygiene");
    }
    if normalized.contains("_negative")
        || normalized.contains("conflict")
        || normalized.contains("misclassification")
        || normalized.contains("noise")
        || normalized.contains("bogus")
        || normalized.contains("without_prior")
        || normalized.contains("caution")
        || normalized.contains("residual")
    {
        labels.insert("truthfulness negatives and cautions");
    }
    if normalized.contains("residual")
        || normalized.contains("caveat")
        || normalized.contains("alias_dependent")
    {
        labels.insert("residuals and caveats");
    }

    if labels.is_empty() {
        labels.insert("uncategorized");
    }
    labels
}

fn replace_managed_block(
    existing: &str,
    managed_block: &str,
    managed_start: &str,
    managed_end: &str,
    missing_heading: &str,
) -> Result<String> {
    let Some(start) = existing.find(managed_start) else {
        let mut updated = existing.trim_end().to_string();
        updated.push_str("\n\n");
        updated.push_str(missing_heading);
        updated.push_str("\n\n");
        updated.push_str(managed_block.trim_end());
        updated.push('\n');
        return Ok(updated);
    };
    let Some(relative_end) = existing[start..].find(managed_end) else {
        return Err(AppError::InvalidStageArtifact(format!(
            "corpus knowledge page has `{managed_start}` but no `{managed_end}`"
        )));
    };
    let end = start + relative_end + managed_end.len();
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
{VALIDATION_MANAGED_START}\n{VALIDATION_MANAGED_END}\n"
    )
}

fn default_kg_fixtures_page() -> String {
    format!(
        "# KG Benchmark Fixture Results\n\n\
This page is the benchmark-result projection family in the `R15g` corpus knowledge base.\n\
It records KG fixture outcomes from the tracked truthfulness benchmark suite without promoting benchmark behavior into canonical document truth.\n\n\
## Human Synthesis\n\n\
Use this section for curated notes about recurring benchmark patterns, fixture families, and candidate follow-up work.\n\
Keep provenance explicit, and do not treat this page as an approval artifact.\n\n\
## Managed KG Benchmark Projection\n\n\
{KG_FIXTURES_MANAGED_START}\n{KG_FIXTURES_MANAGED_END}\n"
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
{VALIDATION_MANAGED_START}\nold generated content\n{VALIDATION_MANAGED_END}\n"
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

    #[test]
    fn corpus_kb_refreshes_kg_fixture_results_without_replacing_human_synthesis() -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = tempdir.path();
        let fixtures_root = repo_root.join("fixtures");
        let fixture_dir = fixtures_root.join("semantic_prior_guided_phrase_gold");
        fs::create_dir_all(&fixture_dir)?;
        fs::write(
            fixture_dir.join("source.md"),
            "# Toy Protocol\n\nSignal XREQ is output width 1.\n",
        )?;
        fs::write(
            fixture_dir.join("fixture.json"),
            r#"{
  "name": "semantic_prior_guided_phrase_gold",
  "source": "source.md",
  "expectations": {}
}"#,
        )?;

        let page_path = repo_root
            .join("corpus_kb")
            .join("benchmarks")
            .join("kg-fixtures.md");
        fs::create_dir_all(page_path.parent().expect("page has parent"))?;
        fs::write(
            &page_path,
            format!(
                "# KG Benchmark Fixture Results\n\n\
## Human Synthesis\n\n\
Keep this benchmark note.\n\n\
## Managed KG Benchmark Projection\n\n\
{KG_FIXTURES_MANAGED_START}\nold generated benchmark content\n{KG_FIXTURES_MANAGED_END}\n"
            ),
        )?;

        let refresh = refresh_kg_fixtures_page(repo_root, &fixtures_root, &[])?;
        let refreshed = fs::read_to_string(page_path)?;

        assert_eq!(refresh.fixture_count, 1);
        assert_eq!(refresh.failed_count, 0);
        assert!(refreshed.contains("Keep this benchmark note."));
        assert!(!refreshed.contains("old generated benchmark content"));
        assert!(refreshed.contains("### Fixture Family Summary"));
        assert!(refreshed.contains("| semantic role arbitration | `1` | `1` | `0` |"));
        assert!(refreshed.contains("| typed prior memory | `1` | `1` | `0` |"));
        assert!(refreshed.contains("### semantic_prior_guided_phrase_gold"));
        assert!(refreshed.contains("- status: `pass`"));

        Ok(())
    }

    #[test]
    fn kg_fixture_family_labels_are_deterministic_and_review_facing() {
        let labels =
            kg_fixture_family_labels("visual_motif_prior_guided_diagram_classification_gold");

        assert!(labels.contains("multimodal visual grounding"));
        assert!(labels.contains("typed prior memory"));
        assert!(!labels.contains("uncategorized"));

        let labels = kg_fixture_family_labels("toy_fixture");
        assert_eq!(labels.len(), 1);
        assert!(labels.contains("uncategorized"));
    }

    #[test]
    fn corpus_kb_rejects_empty_refresh_inputs() {
        let tempdir = tempdir().expect("tempdir");
        let error = run(CorpusKbArgs {
            validation_reports: Vec::new(),
            repo_root: tempdir.path().to_path_buf(),
            kg_fixtures_root: None,
            kg_fixture: Vec::new(),
        })
        .expect_err("empty refresh inputs should fail");

        assert!(matches!(error, AppError::InvalidStageArtifact(_)));
    }

    #[test]
    fn corpus_kb_rejects_kg_fixture_selectors_without_fixture_root() {
        let tempdir = tempdir().expect("tempdir");
        let error = run(CorpusKbArgs {
            validation_reports: vec![tempdir.path().join("validation_report.json")],
            repo_root: tempdir.path().to_path_buf(),
            kg_fixtures_root: None,
            kg_fixture: vec![PathBuf::from("toy_fixture")],
        })
        .expect_err("kg fixture selectors require a fixture root");

        assert!(matches!(error, AppError::InvalidStageArtifact(_)));
    }
}
