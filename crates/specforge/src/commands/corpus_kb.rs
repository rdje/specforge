use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::cli::CorpusKbArgs;
use crate::commands::kg_bench::{self, KgBenchFixtureOutcome};
use crate::error::{AppError, Result};
use crate::ir::source::ValidationReportRecord;

const VALIDATION_MANAGED_START: &str = "<!-- corpus_kb_validation_findings:start -->";
const VALIDATION_MANAGED_END: &str = "<!-- corpus_kb_validation_findings:end -->";
const KG_FIXTURES_MANAGED_START: &str = "<!-- corpus_kb_kg_fixtures:start -->";
const KG_FIXTURES_MANAGED_END: &str = "<!-- corpus_kb_kg_fixtures:end -->";
const KG_FIXTURE_FAMILY_MANAGED_START: &str = "<!-- corpus_kb_kg_fixture_family:start -->";
const KG_FIXTURE_FAMILY_MANAGED_END: &str = "<!-- corpus_kb_kg_fixture_family:end -->";
const PRIOR_CANDIDATES_MANAGED_START: &str = "<!-- corpus_kb_prior_candidates:start -->";
const PRIOR_CANDIDATES_MANAGED_END: &str = "<!-- corpus_kb_prior_candidates:end -->";

const PATTERN_FAMILY_LABELS: &[&str] = &[
    "actor connectivity",
    "semantic role arbitration",
    "negative knowledge",
    "truthfulness negatives and cautions",
    "residuals and caveats",
];
const PRIOR_MEMORY_FAMILY_LABELS: &[&str] = &["typed prior memory"];
const TABLE_FAMILY_LABELS: &[&str] = &["table extraction and hygiene"];
const VISUAL_FAMILY_LABELS: &[&str] = &[
    "multimodal visual grounding",
    "VLM state machines",
    "VLM timing diagrams",
];
const STATE_MACHINE_FAMILY_LABELS: &[&str] = &["VLM state machines"];
const TIMING_FAMILY_LABELS: &[&str] = &["temporal semantics", "VLM timing diagrams"];
const INFRA_FAMILY_LABELS: &[&str] = &["infrastructure semantics", "polarity semantics"];
const AMBA_PROTOCOL_FAMILY_LABELS: &[&str] = &["protocol-family AMBA/APB/AHB/AXI"];

const PRIOR_CANDIDATE_SPECS: &[PriorCandidateSpec] = &[
    PriorCandidateSpec {
        marker: "actor_taxonomy_prior",
        candidate_kind: "actor_taxonomy_prior",
        target_schema: "CorpusMemory.actor_taxonomy_priors",
        required_gates: "typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; local-grounding consumer",
        schema_gate: "present_schema_v5",
        fixture_gate: "paired_gold_without_prior_fixture",
        harvest_gate: "learn_priors_actor_taxonomy_harvester_present",
        consumer_gate: "evidence_actor_taxonomy_local_grounding_consumer_present",
    },
    PriorCandidateSpec {
        marker: "semantic_modality_reliability_prior",
        candidate_kind: "semantic_modality_reliability_prior",
        target_schema: "CorpusMemory.semantic_modality_reliability_priors",
        required_gates: "typed CorpusMemory schema; paired KG-bench conflict coverage; validated IntentIR harvest input; local-grounded arbitration consumer",
        schema_gate: "present_schema_v5",
        fixture_gate: "paired_conflict_gold_without_prior_fixture",
        harvest_gate: "learn_priors_modality_reliability_harvester_present",
        consumer_gate: "semantic_arbitration_reliability_consumer_present",
    },
    PriorCandidateSpec {
        marker: "semantic_prior",
        candidate_kind: "semantic_phrase_prior",
        target_schema: "CorpusMemory.semantic_phrase_priors",
        required_gates: "typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; local-grounding semantic consumer",
        schema_gate: "present_schema_v5",
        fixture_gate: "paired_text_and_visual_gold_without_prior_fixtures",
        harvest_gate: "learn_priors_semantic_phrase_harvester_present",
        consumer_gate: "evidence_semantic_phrase_local_grounding_consumer_present",
    },
    PriorCandidateSpec {
        marker: "temporal_prior",
        candidate_kind: "temporal_phrase_prior",
        target_schema: "CorpusMemory.temporal_phrase_priors",
        required_gates: "typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; local-grounding temporal consumer",
        schema_gate: "present_schema_v5",
        fixture_gate: "paired_gold_without_prior_fixture",
        harvest_gate: "learn_priors_temporal_phrase_harvester_present",
        consumer_gate: "semantic_temporal_phrase_cycle_window_consumer_present",
    },
    PriorCandidateSpec {
        marker: "table_shape_prior",
        candidate_kind: "table_shape_prior",
        target_schema: "CorpusMemory.table_shape_priors",
        required_gates: "typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated SourceIR/IntentIR harvest chain; local table-kind consumer",
        schema_gate: "present_schema_v5",
        fixture_gate: "paired_signal_and_timing_table_gold_without_prior_fixtures",
        harvest_gate: "learn_priors_source_ir_table_shape_harvester_present",
        consumer_gate: "evidence_table_kind_local_shape_consumer_present",
    },
    PriorCandidateSpec {
        marker: "visual_motif_prior",
        candidate_kind: "visual_motif_prior",
        target_schema: "CorpusMemory.visual_motif_priors",
        required_gates: "typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; VLM/multimodal corroboration gate",
        schema_gate: "present_schema_v5",
        fixture_gate: "paired_visual_motif_gold_without_prior_fixture",
        harvest_gate: "learn_priors_source_ir_visual_motif_harvester_present",
        consumer_gate: "evidence_visual_caption_motif_consumer_with_corroboration_gate_present",
    },
    PriorCandidateSpec {
        marker: "negative_knowledge_prior",
        candidate_kind: "negative_knowledge_prior",
        target_schema: "CorpusMemory.negative_knowledge_priors",
        required_gates: "typed CorpusMemory schema; caution-only validation consumer; paired KG-bench conflict/residual coverage; rescan guidance review gate",
        schema_gate: "present_schema_v5",
        fixture_gate: "caution_fixtures_across_conflict_residual_families_present",
        harvest_gate: "learn_priors_conflict_residual_negative_knowledge_harvester_present",
        consumer_gate: "validation_caution_and_rescan_guidance_consumer_present",
    },
];

const KG_FIXTURE_FAMILY_PAGE_SPECS: &[KgFixtureFamilyPageSpec] = &[
    KgFixtureFamilyPageSpec {
        relative_path: "patterns/kg-fixtures.md",
        title: "Semantic And Truthfulness Fixture Patterns",
        description: "This page records semantic arbitration, actor/connectivity, residual, caveat, negative-knowledge, and truthfulness-caution KG fixture coverage from the tracked truthfulness benchmark suite.",
        human_prompt: "Use this section for curated notes about semantic arbitration, graph/connectivity evidence, residual/caveat behavior, and false-positive control patterns.",
        labels: PATTERN_FAMILY_LABELS,
    },
    KgFixtureFamilyPageSpec {
        relative_path: "prior_memory/kg-fixtures.md",
        title: "Typed Prior-Memory Fixture Patterns",
        description: "This page records typed prior-memory KG fixture coverage from the tracked truthfulness benchmark suite.",
        human_prompt: "Use this section for curated notes about prior-guided gold/negative pairs, caution-only negative knowledge, local-grounding boundaries, and future CorpusMemory benchmark gaps.",
        labels: PRIOR_MEMORY_FAMILY_LABELS,
    },
    KgFixtureFamilyPageSpec {
        relative_path: "tables/kg-fixtures.md",
        title: "Table Extraction Fixture Patterns",
        description: "This page records table-related KG fixture coverage from the tracked truthfulness benchmark suite.",
        human_prompt: "Use this section for curated notes about table-shape recovery, table-misclassification risks, and future table-prior candidates.",
        labels: TABLE_FAMILY_LABELS,
    },
    KgFixtureFamilyPageSpec {
        relative_path: "visuals/kg-fixtures.md",
        title: "Visual Evidence Fixture Patterns",
        description: "This page records visual and VLM-related KG fixture coverage from the tracked truthfulness benchmark suite.",
        human_prompt: "Use this section for curated notes about visual grounding, VLM timing/state-machine extraction, and multimodal conflict patterns.",
        labels: VISUAL_FAMILY_LABELS,
    },
    KgFixtureFamilyPageSpec {
        relative_path: "state_machines/kg-fixtures.md",
        title: "State-Machine Fixture Patterns",
        description: "This page records VLM state-machine KG fixture coverage from the tracked truthfulness benchmark suite.",
        human_prompt: "Use this section for curated notes about state labels, transition endpoint grounding, duplicate initial markers, and initial-cardinality validation behavior.",
        labels: STATE_MACHINE_FAMILY_LABELS,
    },
    KgFixtureFamilyPageSpec {
        relative_path: "timing/kg-fixtures.md",
        title: "Timing Motif Fixture Patterns",
        description: "This page records temporal and timing-motif KG fixture coverage from the tracked truthfulness benchmark suite.",
        human_prompt: "Use this section for curated notes about cycle windows, handshake completion, timing diagrams, and temporal conflict patterns.",
        labels: TIMING_FAMILY_LABELS,
    },
    KgFixtureFamilyPageSpec {
        relative_path: "infra/kg-fixtures.md",
        title: "Infrastructure Semantics Fixture Patterns",
        description: "This page records infrastructure-adjacent KG fixture coverage from the tracked truthfulness benchmark suite.",
        human_prompt: "Use this section for curated notes about clock/reset handling, active-level polarity, and infrastructure/control boundaries.",
        labels: INFRA_FAMILY_LABELS,
    },
    KgFixtureFamilyPageSpec {
        relative_path: "protocols/amba-kg-fixtures.md",
        title: "AMBA Family Fixture Patterns",
        description: "This page records AMBA/APB/AHB/AXI-style KG fixture coverage from the tracked truthfulness benchmark suite.",
        human_prompt: "Use this section for curated notes about AMBA-family evidence idioms, protocol vocabulary, and future protocol-family benchmark gaps.",
        labels: AMBA_PROTOCOL_FAMILY_LABELS,
    },
];

#[derive(Debug)]
struct ValidationFindingProjection {
    report_path: PathBuf,
    display_path: String,
    document_key: String,
    report: ValidationReportRecord,
}

#[derive(Debug, PartialEq, Eq)]
struct ReviewedValidationProjection {
    document_key: String,
    artifact_path: String,
    stage: String,
    artifact_fingerprint: String,
    score: String,
    summary: String,
    findings: Vec<String>,
}

#[derive(Debug)]
struct KgFixtureProjection {
    outcome: KgBenchFixtureOutcome,
    display_path: String,
}

#[derive(Debug)]
struct KgFixturesRefresh {
    page_paths: Vec<PathBuf>,
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

struct KgFixtureFamilyPageSpec {
    relative_path: &'static str,
    title: &'static str,
    description: &'static str,
    human_prompt: &'static str,
    labels: &'static [&'static str],
}

struct PriorCandidateSpec {
    marker: &'static str,
    candidate_kind: &'static str,
    target_schema: &'static str,
    required_gates: &'static str,
    schema_gate: &'static str,
    fixture_gate: &'static str,
    harvest_gate: &'static str,
    consumer_gate: &'static str,
}

#[derive(Debug)]
struct PriorCandidateProjection {
    candidate_kind: &'static str,
    target_schema: &'static str,
    required_gates: &'static str,
    schema_gate: &'static str,
    fixture_gate: &'static str,
    harvest_gate: &'static str,
    consumer_gate: &'static str,
    supporting_fixtures: BTreeSet<String>,
    positive_fixtures: BTreeSet<String>,
    guard_fixtures: BTreeSet<String>,
}

#[derive(Debug, Serialize)]
struct PriorCandidateManifest {
    schema_version: u32,
    source: &'static str,
    review_scope: &'static str,
    promotion_status: &'static str,
    canonical_mutation_allowed: bool,
    corpus_memory_mutation_allowed: bool,
    candidates: Vec<PriorCandidateManifestEntry>,
}

#[derive(Debug, Serialize)]
struct PriorCandidateManifestEntry {
    candidate_kind: &'static str,
    target_schema: &'static str,
    readiness: &'static str,
    supporting_fixture_count: usize,
    positive_fixture_count: usize,
    guard_fixture_count: usize,
    supporting_fixtures: Vec<String>,
    positive_fixtures: Vec<String>,
    guard_fixtures: Vec<String>,
    required_gates: &'static str,
    gates: PriorCandidateGateManifest,
    promotion_boundary: &'static str,
}

#[derive(Debug, Serialize)]
struct PriorCandidateGateManifest {
    schema_gate: &'static str,
    fixture_gate: &'static str,
    harvest_gate: &'static str,
    consumer_gate: &'static str,
}

pub fn run(args: CorpusKbArgs) -> Result<()> {
    if args.validation_reports.is_empty()
        && args.validation_snapshot.is_none()
        && args.kg_fixtures_root.is_none()
    {
        return Err(AppError::InvalidStageArtifact(
            "corpus-kb requires at least one validation report, --validation-snapshot, or --kg-fixtures-root"
                .to_string(),
        ));
    }
    if !args.validation_reports.is_empty() && args.validation_snapshot.is_some() {
        return Err(AppError::InvalidStageArtifact(
            "validation reports and --validation-snapshot are mutually exclusive".to_string(),
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

    if let Some(snapshot_path) = args.validation_snapshot.as_deref() {
        let page_path = refresh_reviewed_validation_findings_page(&args.repo_root, snapshot_path)?;
        println!("refreshed_page: {}", page_path.display());
        println!("validation_snapshot: {}", snapshot_path.display());
    }

    if let Some(fixtures_root) = args.kg_fixtures_root.as_deref() {
        let refresh = refresh_kg_fixtures_page(&args.repo_root, fixtures_root, &args.kg_fixture)?;
        for page_path in &refresh.page_paths {
            println!("refreshed_page: {}", page_path.display());
        }
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

fn refresh_reviewed_validation_findings_page(
    repo_root: &Path,
    snapshot_path: &Path,
) -> Result<PathBuf> {
    let snapshot = fs::read_to_string(snapshot_path)?;
    let entries = parse_reviewed_validation_snapshot(&snapshot)?;
    let page_path = repo_root
        .join("corpus_kb")
        .join("failures")
        .join("validation-findings.md");
    if let Some(parent) = page_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let existing = fs::read_to_string(&page_path).unwrap_or_else(|_| default_validation_page());
    let managed_block = render_reviewed_validation_findings_block(&entries);
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
    let mut page_paths = vec![page_path];
    page_paths.extend(refresh_kg_fixture_family_pages(repo_root, &entries)?);
    page_paths.extend(refresh_prior_candidate_projection(repo_root, &entries)?);

    Ok(KgFixturesRefresh {
        page_paths,
        fixture_count: entries.len(),
        failed_count,
    })
}

fn refresh_kg_fixture_family_pages(
    repo_root: &Path,
    entries: &[KgFixtureProjection],
) -> Result<Vec<PathBuf>> {
    let mut page_paths = Vec::new();
    for spec in KG_FIXTURE_FAMILY_PAGE_SPECS {
        let page_path = repo_root.join("corpus_kb").join(spec.relative_path);
        if let Some(parent) = page_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let family_entries = entries
            .iter()
            .filter(|entry| kg_fixture_matches_any_family(&entry.outcome.name, spec.labels))
            .collect::<Vec<_>>();
        let existing =
            fs::read_to_string(&page_path).unwrap_or_else(|_| default_kg_fixture_family_page(spec));
        let managed_block = render_kg_fixture_family_block(spec, &family_entries);
        let updated = replace_managed_block(
            &existing,
            &managed_block,
            KG_FIXTURE_FAMILY_MANAGED_START,
            KG_FIXTURE_FAMILY_MANAGED_END,
            "## Managed Fixture Projection",
        )?;
        fs::write(&page_path, updated)?;
        page_paths.push(page_path);
    }
    Ok(page_paths)
}

fn refresh_prior_candidate_projection(
    repo_root: &Path,
    entries: &[KgFixtureProjection],
) -> Result<Vec<PathBuf>> {
    let page_path = repo_root
        .join("corpus_kb")
        .join("prior_candidates")
        .join("kg-fixture-candidates.md");
    if let Some(parent) = page_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let existing =
        fs::read_to_string(&page_path).unwrap_or_else(|_| default_prior_candidate_page());
    let candidates = prior_candidate_projections(entries);
    let managed_block = render_prior_candidate_block(&candidates);
    let updated = replace_managed_block(
        &existing,
        &managed_block,
        PRIOR_CANDIDATES_MANAGED_START,
        PRIOR_CANDIDATES_MANAGED_END,
        "## Managed Prior Candidate Projection",
    )?;
    fs::write(&page_path, updated)?;

    let manifest_path = repo_root
        .join("corpus_kb")
        .join("prior_candidates")
        .join("kg-fixture-candidates.json");
    let manifest = prior_candidate_manifest(&candidates);
    fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
    Ok(vec![page_path, manifest_path])
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

fn parse_reviewed_validation_snapshot(snapshot: &str) -> Result<Vec<ReviewedValidationProjection>> {
    let (_, projected) = snapshot
        .split_once("## Projected Artifacts\n")
        .ok_or_else(|| {
            AppError::InvalidStageArtifact(
                "reviewed validation snapshot has no `## Projected Artifacts` section".to_string(),
            )
        })?;
    let projected = projected.strip_prefix("### ").ok_or_else(|| {
        AppError::InvalidStageArtifact(
            "reviewed validation snapshot has no projected artifact records".to_string(),
        )
    })?;

    let mut entries = Vec::new();
    let mut document_keys = BTreeSet::new();
    for raw_chunk in projected.split("\n### ") {
        let chunk = format!("### {raw_chunk}");
        let heading = chunk.lines().next().unwrap_or_default();
        let stage = heading
            .strip_suffix(')')
            .and_then(|value| value.rsplit_once(" (").map(|(_, stage)| stage))
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                AppError::InvalidStageArtifact(format!(
                    "reviewed validation snapshot has malformed artifact heading `{heading}`"
                ))
            })?
            .to_string();
        let document_key = reviewed_snapshot_backtick_field(&chunk, "document_key")?;
        if !document_keys.insert(document_key.clone()) {
            return Err(AppError::InvalidStageArtifact(format!(
                "reviewed validation snapshot repeats document_key `{document_key}`"
            )));
        }
        let artifact_path = reviewed_snapshot_backtick_field(&chunk, "artifact_path")?;
        let artifact_fingerprint =
            reviewed_snapshot_backtick_field(&chunk, "artifact_fingerprint")?;
        let score = reviewed_snapshot_backtick_field(&chunk, "score")?;
        let summary = reviewed_snapshot_plain_field(&chunk, "summary")?;
        let (_, findings_text) = chunk.split_once("- findings:\n").ok_or_else(|| {
            AppError::InvalidStageArtifact(format!(
                "reviewed validation snapshot artifact `{document_key}` has no findings list"
            ))
        })?;
        let findings = findings_text
            .lines()
            .take_while(|line| line.starts_with("  - "))
            .map(|line| line[4..].to_string())
            .collect::<Vec<_>>();
        if findings.is_empty() {
            return Err(AppError::InvalidStageArtifact(format!(
                "reviewed validation snapshot artifact `{document_key}` has an empty findings list"
            )));
        }

        entries.push(ReviewedValidationProjection {
            document_key,
            artifact_path,
            stage,
            artifact_fingerprint,
            score,
            summary,
            findings,
        });
    }
    if entries.is_empty() {
        return Err(AppError::InvalidStageArtifact(
            "reviewed validation snapshot projected no artifacts".to_string(),
        ));
    }
    Ok(entries)
}

fn reviewed_snapshot_backtick_field(chunk: &str, field: &str) -> Result<String> {
    let prefix = format!("- {field}: `");
    let line = chunk
        .lines()
        .find(|line| line.starts_with(&prefix))
        .ok_or_else(|| {
            AppError::InvalidStageArtifact(format!(
                "reviewed validation snapshot artifact has no `{field}` field"
            ))
        })?;
    let value = line
        .strip_prefix(&prefix)
        .and_then(|value| value.strip_suffix('`'))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AppError::InvalidStageArtifact(format!(
                "reviewed validation snapshot artifact has malformed `{field}` field"
            ))
        })?;
    Ok(value.to_string())
}

fn reviewed_snapshot_plain_field(chunk: &str, field: &str) -> Result<String> {
    let prefix = format!("- {field}: ");
    let value = chunk
        .lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AppError::InvalidStageArtifact(format!(
                "reviewed validation snapshot artifact has no `{field}` field"
            ))
        })?;
    Ok(value.to_string())
}

fn render_reviewed_validation_findings_block(entries: &[ReviewedValidationProjection]) -> String {
    let mut output = String::new();
    output.push_str(VALIDATION_MANAGED_START);
    output.push('\n');
    output.push_str(
        "<!-- This reviewed block is refreshed from `VALIDATION_SNAPSHOT.md` by `specforge corpus-kb --validation-snapshot`. -->\n\n",
    );

    for entry in entries {
        output.push_str("### ");
        output.push_str(&entry.document_key);
        output.push('\n');
        output.push_str("- artifact_path: `");
        output.push_str(&entry.artifact_path);
        output.push_str("`\n");
        output.push_str("- stage: `");
        output.push_str(&entry.stage);
        output.push_str("`\n");
        output.push_str("- artifact_fingerprint: `");
        output.push_str(&entry.artifact_fingerprint);
        output.push_str("`\n");
        output.push_str("- score: `");
        output.push_str(&entry.score);
        output.push_str("`\n");
        output.push_str("- summary: ");
        output.push_str(&escape_markdown_line(&entry.summary));
        output.push_str("\n- findings:\n");
        for finding in &entry.findings {
            output.push_str("  - ");
            output.push_str(&escape_markdown_line(finding));
            output.push('\n');
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
                output.push_str(":\n");
                for fixture in &summary.failed_fixture_names {
                    output.push_str("  - `");
                    output.push_str(&escape_markdown_line(fixture));
                    output.push_str("`\n");
                }
            }
            output.push('\n');
        }
    }

    if entries.is_empty() {
        output.push_str("- No KG fixtures were projected.\n\n");
    } else {
        output.push_str("### Fixture Results\n");
        output.push_str("| fixture | status | path |\n");
        output.push_str("| --- | --- | --- |\n");
        for entry in entries {
            output.push_str("| `");
            output.push_str(&escape_markdown_line(&entry.outcome.name));
            output.push_str("` | `");
            output.push_str(if entry.outcome.failures.is_empty() {
                "pass"
            } else {
                "fail"
            });
            output.push_str("` | `");
            output.push_str(&escape_markdown_line(&entry.display_path));
            output.push_str("` |\n");
        }
        output.push('\n');

        let failing_entries = entries
            .iter()
            .filter(|entry| !entry.outcome.failures.is_empty())
            .collect::<Vec<_>>();
        if !failing_entries.is_empty() {
            output.push_str("Failed fixture details:\n");
            for entry in failing_entries {
                output.push_str("- `");
                output.push_str(&escape_markdown_line(&entry.outcome.name));
                output.push_str("`:\n");
                for failure in &entry.outcome.failures {
                    output.push_str("  - ");
                    output.push_str(&escape_markdown_line(failure));
                    output.push('\n');
                }
            }
            output.push('\n');
        }
    }

    output.push_str(KG_FIXTURES_MANAGED_END);
    output.push('\n');
    output
}

fn render_kg_fixture_family_block(
    spec: &KgFixtureFamilyPageSpec,
    entries: &[&KgFixtureProjection],
) -> String {
    let mut output = String::new();
    output.push_str(KG_FIXTURE_FAMILY_MANAGED_START);
    output.push('\n');
    output.push_str("<!-- This block is refreshed by `specforge corpus-kb`. -->\n\n");
    output.push_str("- source: `kg-bench fixtures`\n");
    output.push_str("- selected_family_labels: `");
    output.push_str(&spec.labels.join("`, `"));
    output.push_str("`\n");
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

    if entries.is_empty() {
        output.push_str("- No KG fixtures currently match this family page.\n\n");
    } else {
        output.push_str("| fixture | status | matched families | path |\n");
        output.push_str("| --- | --- | --- | --- |\n");
        for entry in entries {
            let family_labels = kg_fixture_family_labels(&entry.outcome.name)
                .into_iter()
                .filter(|label| spec.labels.contains(label))
                .collect::<Vec<_>>();
            output.push_str("| `");
            output.push_str(&entry.outcome.name);
            output.push_str("` | `");
            output.push_str(if entry.outcome.failures.is_empty() {
                "pass"
            } else {
                "fail"
            });
            output.push_str("` | `");
            output.push_str(&family_labels.join("`, `"));
            output.push_str("` | `");
            output.push_str(&entry.display_path);
            output.push_str("` |\n");
        }
        output.push('\n');
    }

    let failing_entries = entries
        .iter()
        .filter(|entry| !entry.outcome.failures.is_empty())
        .collect::<Vec<_>>();
    if !failing_entries.is_empty() {
        output.push_str("Failed fixture details:\n");
        for entry in failing_entries {
            output.push_str("- `");
            output.push_str(&entry.outcome.name);
            output.push_str("`:\n");
            for failure in &entry.outcome.failures {
                output.push_str("  - ");
                output.push_str(&escape_markdown_line(failure));
                output.push('\n');
            }
        }
        output.push('\n');
    }

    output.push_str(KG_FIXTURE_FAMILY_MANAGED_END);
    output.push('\n');
    output
}

fn render_prior_candidate_block(candidates: &[PriorCandidateProjection]) -> String {
    let mut output = String::new();
    output.push_str(PRIOR_CANDIDATES_MANAGED_START);
    output.push('\n');
    output.push_str("<!-- This block is refreshed by `specforge corpus-kb`. -->\n\n");
    output.push_str("- source: `kg-bench fixtures`\n");
    output.push_str("- review_scope: `family_surface_not_individual_prior`\n");
    output.push_str("- promotion_status: `candidate_not_promoted_review_required`\n");
    output.push_str("- canonical_mutation_allowed: `false`\n");
    output.push_str("- corpus_memory_mutation_allowed: `false`\n\n");

    if candidates.is_empty() {
        output
            .push_str("- No prior candidates were projected from the current KG fixture run.\n\n");
    } else {
        output.push_str(
            "| candidate_kind | target_schema | supporting | positive | guard | required_gates |\n",
        );
        output.push_str("| --- | --- | ---: | ---: | ---: | --- |\n");
        for candidate in candidates {
            output.push_str("| `");
            output.push_str(candidate.candidate_kind);
            output.push_str("` | `");
            output.push_str(candidate.target_schema);
            output.push_str("` | `");
            output.push_str(&candidate.supporting_fixtures.len().to_string());
            output.push_str("` | `");
            output.push_str(&candidate.positive_fixtures.len().to_string());
            output.push_str("` | `");
            output.push_str(&candidate.guard_fixtures.len().to_string());
            output.push_str("` | ");
            output.push_str(&escape_markdown_line(candidate.required_gates));
            output.push_str(" |\n");
        }
        output.push('\n');
        output.push_str("### Fixture Evidence\n");
        output.push_str("Each evidence fixture is listed on its own line so growth remains reviewable and bounded.\n\n");
        for candidate in candidates {
            output.push_str("#### `");
            output.push_str(candidate.candidate_kind);
            output.push_str("`\n");
            push_fixture_evidence(&mut output, "positive_gates", &candidate.positive_fixtures);
            push_fixture_evidence(&mut output, "guard_gates", &candidate.guard_fixtures);
            output.push('\n');
        }
        output.push_str("### Readiness Summary\n");
        output.push_str("Readiness is fixture-surface readiness only. It is not promotion approval and does not allow `CorpusMemory` or canonical IR mutation.\n\n");
        output.push_str(
            "| candidate_kind | readiness | supporting | positive | guard | promotion_boundary |\n",
        );
        output.push_str("| --- | --- | ---: | ---: | ---: | --- |\n");
        for candidate in candidates {
            output.push_str("| `");
            output.push_str(candidate.candidate_kind);
            output.push_str("` | `");
            output.push_str(prior_candidate_readiness(candidate));
            output.push_str("` | `");
            output.push_str(&candidate.supporting_fixtures.len().to_string());
            output.push_str("` | `");
            output.push_str(&candidate.positive_fixtures.len().to_string());
            output.push_str("` | `");
            output.push_str(&candidate.guard_fixtures.len().to_string());
            output.push_str("` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |\n");
        }
        output.push('\n');
        output.push_str("### Promotion Gate Review Matrix\n");
        output.push_str("These gates describe the family-level implementation surface already visible to review. They are not approval records and do not grant mutation authority.\n\n");
        output.push_str("| candidate_kind | schema_gate | fixture_gate | harvest_gate | consumer_gate | promotion_boundary |\n");
        output.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for candidate in candidates {
            output.push_str("| `");
            output.push_str(candidate.candidate_kind);
            output.push_str("` | `");
            output.push_str(candidate.schema_gate);
            output.push_str("` | `");
            output.push_str(candidate.fixture_gate);
            output.push_str("` | `");
            output.push_str(candidate.harvest_gate);
            output.push_str("` | `");
            output.push_str(candidate.consumer_gate);
            output.push_str("` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |\n");
        }
        output.push('\n');
    }

    output.push_str(PRIOR_CANDIDATES_MANAGED_END);
    output.push('\n');
    output
}

fn prior_candidate_manifest(candidates: &[PriorCandidateProjection]) -> PriorCandidateManifest {
    PriorCandidateManifest {
        schema_version: 1,
        source: "kg-bench fixtures",
        review_scope: "family_surface_not_individual_prior",
        promotion_status: "candidate_not_promoted_review_required",
        canonical_mutation_allowed: false,
        corpus_memory_mutation_allowed: false,
        candidates: candidates
            .iter()
            .map(|candidate| PriorCandidateManifestEntry {
                candidate_kind: candidate.candidate_kind,
                target_schema: candidate.target_schema,
                readiness: prior_candidate_readiness(candidate),
                supporting_fixture_count: candidate.supporting_fixtures.len(),
                positive_fixture_count: candidate.positive_fixtures.len(),
                guard_fixture_count: candidate.guard_fixtures.len(),
                supporting_fixtures: candidate.supporting_fixtures.iter().cloned().collect(),
                positive_fixtures: candidate.positive_fixtures.iter().cloned().collect(),
                guard_fixtures: candidate.guard_fixtures.iter().cloned().collect(),
                required_gates: candidate.required_gates,
                gates: PriorCandidateGateManifest {
                    schema_gate: candidate.schema_gate,
                    fixture_gate: candidate.fixture_gate,
                    harvest_gate: candidate.harvest_gate,
                    consumer_gate: candidate.consumer_gate,
                },
                promotion_boundary: "review_only_no_corpus_memory_or_canonical_ir_mutation",
            })
            .collect(),
    }
}

fn prior_candidate_readiness(candidate: &PriorCandidateProjection) -> &'static str {
    if candidate.candidate_kind == "negative_knowledge_prior"
        && !candidate.guard_fixtures.is_empty()
    {
        return "caution_surface_review_ready";
    }
    match (
        candidate.positive_fixtures.is_empty(),
        candidate.guard_fixtures.is_empty(),
    ) {
        (false, false) => "fixture_paired_review_ready",
        (false, true) => "needs_guard_fixture",
        (true, false) => "needs_positive_fixture",
        (true, true) => "needs_fixture_coverage",
    }
}

fn prior_candidate_projections(entries: &[KgFixtureProjection]) -> Vec<PriorCandidateProjection> {
    let mut candidates = BTreeMap::<&'static str, PriorCandidateProjection>::new();
    for entry in entries {
        let normalized_name = entry.outcome.name.to_ascii_lowercase();
        for spec in PRIOR_CANDIDATE_SPECS {
            if !normalized_name.contains(spec.marker) {
                continue;
            }
            let candidate =
                candidates
                    .entry(spec.candidate_kind)
                    .or_insert_with(|| PriorCandidateProjection {
                        candidate_kind: spec.candidate_kind,
                        target_schema: spec.target_schema,
                        required_gates: spec.required_gates,
                        schema_gate: spec.schema_gate,
                        fixture_gate: spec.fixture_gate,
                        harvest_gate: spec.harvest_gate,
                        consumer_gate: spec.consumer_gate,
                        supporting_fixtures: BTreeSet::new(),
                        positive_fixtures: BTreeSet::new(),
                        guard_fixtures: BTreeSet::new(),
                    });
            candidate
                .supporting_fixtures
                .insert(entry.outcome.name.clone());
            if normalized_name.contains("_gold") {
                candidate
                    .positive_fixtures
                    .insert(entry.outcome.name.clone());
            }
            if normalized_name.contains("_negative")
                || normalized_name.contains("without_prior")
                || normalized_name.contains("caution")
            {
                candidate.guard_fixtures.insert(entry.outcome.name.clone());
            }
        }
    }
    candidates.into_values().collect()
}

fn push_fixture_evidence(output: &mut String, label: &str, fixtures: &BTreeSet<String>) {
    output.push_str("- ");
    output.push_str(label);
    output.push_str(":\n");
    if fixtures.is_empty() {
        output.push_str("  - none\n");
    } else {
        for fixture in fixtures {
            output.push_str("  - `");
            output.push_str(&escape_markdown_line(fixture));
            output.push_str("`\n");
        }
    }
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

fn kg_fixture_matches_any_family(name: &str, labels: &[&str]) -> bool {
    let fixture_labels = kg_fixture_family_labels(name);
    labels.iter().any(|label| fixture_labels.contains(label))
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
        || normalized.contains("stability")
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
    if normalized.contains("active_low")
        || normalized.contains("clock")
        || (normalized.contains("reset") && !normalized.contains("non_reset"))
    {
        labels.insert("infrastructure semantics");
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

fn default_kg_fixture_family_page(spec: &KgFixtureFamilyPageSpec) -> String {
    format!(
        "# {title}\n\n\
{description}\n\
It is derived from tracked KG fixture outcomes and remains reviewable synthesis, not canonical document truth.\n\n\
## Human Synthesis\n\n\
{human_prompt}\n\
Keep provenance explicit, and do not treat this page as an approval artifact.\n\n\
## Managed Fixture Projection\n\n\
{managed_start}\n{managed_end}\n",
        title = spec.title,
        description = spec.description,
        human_prompt = spec.human_prompt,
        managed_start = KG_FIXTURE_FAMILY_MANAGED_START,
        managed_end = KG_FIXTURE_FAMILY_MANAGED_END,
    )
}

fn default_prior_candidate_page() -> String {
    format!(
        "# KG Fixture Prior Candidates\n\n\
This page records review-only prior candidates derived from tracked KG fixture patterns.\n\
It is an explicit candidate surface, not a `CorpusMemory` artifact and not a promotion approval.\n\n\
## Human Synthesis\n\n\
Use this section for curated notes about which candidate prior families should become typed harvesters or consumers next.\n\
Every machine-usable promotion must still pass through an explicit schema, KG-bench coverage, validation, and local-grounding review.\n\n\
## Managed Prior Candidate Projection\n\n\
{PRIOR_CANDIDATES_MANAGED_START}\n{PRIOR_CANDIDATES_MANAGED_END}\n"
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
        let fixture_dir = fixtures_root.join("table_shape_prior_guided_signal_table_gold");
        fs::create_dir_all(&fixture_dir)?;
        fs::write(
            fixture_dir.join("source.md"),
            "# Toy Protocol\n\nSignal XREQ is output width 1.\n",
        )?;
        fs::write(
            fixture_dir.join("fixture.json"),
            r#"{
  "name": "table_shape_prior_guided_signal_table_gold",
  "source": "source.md",
  "expectations": {}
}"#,
        )?;
        let table_guard_fixture_dir =
            fixtures_root.join("table_shape_prior_guided_signal_table_without_prior_negative");
        fs::create_dir_all(&table_guard_fixture_dir)?;
        fs::write(
            table_guard_fixture_dir.join("source.md"),
            "# Toy Protocol\n\nSignal XREQ is output width 1.\n",
        )?;
        fs::write(
            table_guard_fixture_dir.join("fixture.json"),
            r#"{
  "name": "table_shape_prior_guided_signal_table_without_prior_negative",
  "source": "source.md",
  "expectations": {}
}"#,
        )?;
        let semantic_fixture_dir = fixtures_root.join("name_only_semantic_noise_negative");
        fs::create_dir_all(&semantic_fixture_dir)?;
        fs::write(
            semantic_fixture_dir.join("source.md"),
            "# Toy Protocol\n\nThe VALID signal is listed by name only.\n",
        )?;
        fs::write(
            semantic_fixture_dir.join("fixture.json"),
            r#"{
  "name": "name_only_semantic_noise_negative",
  "source": "source.md",
  "expectations": {}
}"#,
        )?;
        let state_machine_fixture_dir =
            fixtures_root.join("vlm_state_machine_duplicate_initial_gold");
        fs::create_dir_all(&state_machine_fixture_dir)?;
        fs::write(
            state_machine_fixture_dir.join("source.md"),
            "# Toy Protocol\n\nThe IDLE state transitions to BUSY.\n",
        )?;
        fs::write(
            state_machine_fixture_dir.join("fixture.json"),
            r#"{
  "name": "vlm_state_machine_duplicate_initial_gold",
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

        assert_eq!(refresh.fixture_count, 4);
        assert_eq!(refresh.failed_count, 0);
        assert!(refreshed.contains("Keep this benchmark note."));
        assert!(!refreshed.contains("old generated benchmark content"));
        assert!(refreshed.contains("### Fixture Family Summary"));
        assert!(refreshed.contains("| table extraction and hygiene | `2` | `2` | `0` |"));
        assert!(refreshed.contains("| VLM state machines | `1` | `1` | `0` |"));
        assert!(refreshed.contains("| multimodal visual grounding | `1` | `1` | `0` |"));
        assert!(refreshed.contains("| typed prior memory | `2` | `2` | `0` |"));
        assert!(refreshed.contains("| semantic role arbitration | `1` | `1` | `0` |"));
        assert!(refreshed.contains("| truthfulness negatives and cautions | `2` | `2` | `0` |"));
        assert!(refreshed.contains("### Fixture Results"));
        assert!(refreshed.contains("| fixture | status | path |"));
        assert!(refreshed.contains("| `table_shape_prior_guided_signal_table_gold` | `pass` | `"));
        assert!(refreshed.contains(
            "| `table_shape_prior_guided_signal_table_without_prior_negative` | `pass` | `"
        ));
        assert!(refreshed.contains("| `name_only_semantic_noise_negative` | `pass` | `"));
        assert!(refreshed.contains("| `vlm_state_machine_duplicate_initial_gold` | `pass` | `"));

        let pattern_family_page = repo_root
            .join("corpus_kb")
            .join("patterns")
            .join("kg-fixtures.md");
        let pattern_family_refreshed = fs::read_to_string(pattern_family_page)?;
        assert!(pattern_family_refreshed.contains("# Semantic And Truthfulness Fixture Patterns"));
        assert!(
            pattern_family_refreshed.contains("| `name_only_semantic_noise_negative` | `pass` |")
        );
        assert!(pattern_family_refreshed.contains("`semantic role arbitration`"));
        assert!(pattern_family_refreshed.contains("`truthfulness negatives and cautions`"));

        let prior_memory_family_page = repo_root
            .join("corpus_kb")
            .join("prior_memory")
            .join("kg-fixtures.md");
        let prior_memory_family_refreshed = fs::read_to_string(prior_memory_family_page)?;
        assert!(prior_memory_family_refreshed.contains("# Typed Prior-Memory Fixture Patterns"));
        assert!(
            prior_memory_family_refreshed
                .contains("| `table_shape_prior_guided_signal_table_gold` | `pass` |")
        );
        assert!(prior_memory_family_refreshed.contains(
            "| `table_shape_prior_guided_signal_table_without_prior_negative` | `pass` |"
        ));
        assert!(prior_memory_family_refreshed.contains("`typed prior memory`"));

        let table_family_page = repo_root
            .join("corpus_kb")
            .join("tables")
            .join("kg-fixtures.md");
        let table_family_refreshed = fs::read_to_string(table_family_page)?;
        assert!(table_family_refreshed.contains("# Table Extraction Fixture Patterns"));
        assert!(
            table_family_refreshed
                .contains("| `table_shape_prior_guided_signal_table_gold` | `pass` |")
        );
        assert!(table_family_refreshed.contains(
            "| `table_shape_prior_guided_signal_table_without_prior_negative` | `pass` |"
        ));
        assert!(table_family_refreshed.contains("`table extraction and hygiene`"));

        let state_machine_family_page = repo_root
            .join("corpus_kb")
            .join("state_machines")
            .join("kg-fixtures.md");
        let state_machine_family_refreshed = fs::read_to_string(state_machine_family_page)?;
        assert!(state_machine_family_refreshed.contains("# State-Machine Fixture Patterns"));
        assert!(
            state_machine_family_refreshed
                .contains("| `vlm_state_machine_duplicate_initial_gold` | `pass` |")
        );
        assert!(state_machine_family_refreshed.contains("`VLM state machines`"));

        let prior_candidate_page = repo_root
            .join("corpus_kb")
            .join("prior_candidates")
            .join("kg-fixture-candidates.md");
        let prior_candidate_refreshed = fs::read_to_string(prior_candidate_page)?;
        assert!(prior_candidate_refreshed.contains("# KG Fixture Prior Candidates"));
        assert!(prior_candidate_refreshed.contains("`table_shape_prior`"));
        assert!(prior_candidate_refreshed.contains("`CorpusMemory.table_shape_priors`"));
        assert!(prior_candidate_refreshed.contains("candidate_not_promoted_review_required"));
        assert!(prior_candidate_refreshed.contains("`table_shape_prior_guided_signal_table_gold`"));
        assert!(prior_candidate_refreshed.contains("### Fixture Evidence"));
        assert!(
            prior_candidate_refreshed
                .contains("- positive_gates:\n  - `table_shape_prior_guided_signal_table_gold`")
        );
        assert!(prior_candidate_refreshed.contains("### Promotion Gate Review Matrix"));
        assert!(prior_candidate_refreshed.contains("### Readiness Summary"));
        assert!(prior_candidate_refreshed.contains("`fixture_paired_review_ready`"));
        assert!(prior_candidate_refreshed.contains("family_surface_not_individual_prior"));
        assert!(
            prior_candidate_refreshed
                .contains("`learn_priors_source_ir_table_shape_harvester_present`")
        );
        assert!(
            prior_candidate_refreshed
                .contains("`review_only_no_corpus_memory_or_canonical_ir_mutation`")
        );
        let prior_candidate_manifest = fs::read_to_string(
            repo_root
                .join("corpus_kb")
                .join("prior_candidates")
                .join("kg-fixture-candidates.json"),
        )?;
        let prior_candidate_manifest: serde_json::Value =
            serde_json::from_str(&prior_candidate_manifest)?;
        assert_eq!(prior_candidate_manifest["schema_version"], 1);
        assert_eq!(
            prior_candidate_manifest["promotion_status"],
            "candidate_not_promoted_review_required"
        );
        assert_eq!(
            prior_candidate_manifest["candidates"][0]["readiness"],
            "fixture_paired_review_ready"
        );
        assert_eq!(
            prior_candidate_manifest["candidates"][0]["promotion_boundary"],
            "review_only_no_corpus_memory_or_canonical_ir_mutation"
        );

        Ok(())
    }

    #[test]
    fn kg_fixture_family_labels_are_deterministic_and_review_facing() {
        let labels =
            kg_fixture_family_labels("visual_motif_prior_guided_diagram_classification_gold");

        assert!(labels.contains("multimodal visual grounding"));
        assert!(labels.contains("typed prior memory"));
        assert!(!labels.contains("uncategorized"));

        let labels = kg_fixture_family_labels("vlm_timing_active_low_assertion_equivalence_gold");
        assert!(labels.contains("infrastructure semantics"));
        assert!(labels.contains("polarity semantics"));
        assert!(labels.contains("VLM timing diagrams"));

        let labels = kg_fixture_family_labels("axi_sideband_stability_gold");
        assert!(labels.contains("protocol-family AMBA/APB/AHB/AXI"));
        assert!(labels.contains("temporal semantics"));

        let labels = kg_fixture_family_labels("toy_fixture");
        assert_eq!(labels.len(), 1);
        assert!(labels.contains("uncategorized"));
    }

    #[test]
    fn kg_fixture_matches_any_family_matches_correctly() {
        assert!(kg_fixture_matches_any_family(
            "amba_axi_gold",
            &["protocol-family AMBA/APB/AHB/AXI"]
        ));
        assert!(!kg_fixture_matches_any_family(
            "amba_axi_gold",
            &["actor connectivity"]
        ));
    }

    #[test]
    fn kg_fixture_matches_any_family_returns_false_when_no_match() {
        assert!(!kg_fixture_matches_any_family(
            "toy_fixture",
            &["actor connectivity"]
        ));
    }

    #[test]
    fn default_validation_page_is_non_empty() {
        let page = default_validation_page();
        assert!(!page.is_empty());
        assert!(page.contains("Human Synthesis"));
        assert!(page.contains(VALIDATION_MANAGED_START));
    }

    #[test]
    fn default_kg_fixtures_page_is_non_empty() {
        let page = default_kg_fixtures_page();
        assert!(!page.is_empty());
        assert!(page.contains("Human Synthesis"));
        assert!(page.contains(KG_FIXTURES_MANAGED_START));
    }

    #[test]
    fn display_path_returns_non_empty_string() -> Result<()> {
        let tempdir = tempdir()?;
        let sub_path = tempdir.path().join("subdir").join("file.txt");
        let result = display_path(tempdir.path(), &sub_path);
        assert!(!result.is_empty());
        Ok(())
    }

    #[test]
    fn display_path_falls_back_to_display_on_error() {
        let non_existent = std::path::Path::new("/nonexistent/root");
        let path = std::path::Path::new("/some/path");
        let result = display_path(non_existent, path);
        assert!(!result.is_empty());
        assert_eq!(result, path.display().to_string());
    }

    #[test]
    fn corpus_kb_rejects_kg_fixture_selectors_and_reports_invalid_combination() {
        let tempdir = tempdir().expect("tempdir");
        let report_path = tempdir.path().join("nonexistent_report.json");
        let error = run(CorpusKbArgs {
            validation_reports: vec![report_path],
            validation_snapshot: None,
            repo_root: tempdir.path().to_path_buf(),
            kg_fixtures_root: None,
            kg_fixture: vec![PathBuf::from("toy_fixture")],
        })
        .expect_err("kg fixture selectors require a fixture root");
        assert!(matches!(error, AppError::InvalidStageArtifact(_)));
    }

    #[test]
    fn corpus_kb_rejects_empty_refresh_inputs() {
        let tempdir = tempdir().expect("tempdir");
        let error = run(CorpusKbArgs {
            validation_reports: Vec::new(),
            validation_snapshot: None,
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
            validation_snapshot: None,
            repo_root: tempdir.path().to_path_buf(),
            kg_fixtures_root: None,
            kg_fixture: vec![PathBuf::from("toy_fixture")],
        })
        .expect_err("kg fixture selectors require a fixture root");

        assert!(matches!(error, AppError::InvalidStageArtifact(_)));
    }

    #[test]
    fn reviewed_validation_snapshot_parser_and_renderer_preserve_reviewed_findings() -> Result<()> {
        let snapshot = "# Snapshot\n\n## Projected Artifacts\n### Protocol.pdf (intent_ir)\n- document_key: `protocol`\n- artifact_path: `generated/intent_ir/protocol/intent_ir.json`\n- artifact_fingerprint: `0123456789abcdef`\n- score: `71/100 GOOD`\n- summary: IntentIR review with 2 finding(s)\n- findings:\n  - [warning:quality_score] score finding\n  - [info:knowledge_graph] graph finding\n";

        let entries = parse_reviewed_validation_snapshot(snapshot)?;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].document_key, "protocol");
        assert_eq!(entries[0].stage, "intent_ir");
        assert_eq!(entries[0].score, "71/100 GOOD");
        assert_eq!(entries[0].findings.len(), 2);

        let rendered = render_reviewed_validation_findings_block(&entries);
        assert!(rendered.contains("refreshed from `VALIDATION_SNAPSHOT.md`"));
        assert!(
            rendered.contains("- artifact_path: `generated/intent_ir/protocol/intent_ir.json`")
        );
        assert!(rendered.contains("  - [info:knowledge_graph] graph finding"));
        assert!(!rendered.contains("report_path"));
        Ok(())
    }

    #[test]
    fn reviewed_validation_snapshot_parser_rejects_missing_findings() {
        let snapshot = "## Projected Artifacts\n### Protocol.pdf (intent_ir)\n- document_key: `protocol`\n- artifact_path: `generated/intent_ir/protocol/intent_ir.json`\n- artifact_fingerprint: `0123456789abcdef`\n- score: `71/100 GOOD`\n- summary: review\n";
        let error = parse_reviewed_validation_snapshot(snapshot)
            .expect_err("missing findings must fail closed");
        assert!(matches!(error, AppError::InvalidStageArtifact(_)));
    }

    #[test]
    fn corpus_kb_rejects_mixed_validation_authorities() {
        let tempdir = tempdir().expect("tempdir");
        let error = run(CorpusKbArgs {
            validation_reports: vec![tempdir.path().join("validation_report.json")],
            validation_snapshot: Some(tempdir.path().join("VALIDATION_SNAPSHOT.md")),
            repo_root: tempdir.path().to_path_buf(),
            kg_fixtures_root: None,
            kg_fixture: Vec::new(),
        })
        .expect_err("reviewed snapshot and ambient reports must not be mixed");
        assert!(matches!(error, AppError::InvalidStageArtifact(_)));
    }

    // --- prior_candidate_readiness ---

    fn make_projection(
        candidate_kind: &'static str,
        positive_fixtures: &[&str],
        guard_fixtures: &[&str],
    ) -> PriorCandidateProjection {
        PriorCandidateProjection {
            candidate_kind,
            target_schema: "test_schema",
            required_gates: "none",
            schema_gate: "none",
            fixture_gate: "none",
            harvest_gate: "none",
            consumer_gate: "none",
            supporting_fixtures: BTreeSet::new(),
            positive_fixtures: positive_fixtures.iter().map(|s| s.to_string()).collect(),
            guard_fixtures: guard_fixtures.iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn prior_candidate_readiness_caution_surface_for_negative_with_guards() {
        let c = make_projection("negative_knowledge_prior", &[], &["guard1"]);
        assert_eq!(
            prior_candidate_readiness(&c),
            "caution_surface_review_ready"
        );
    }

    #[test]
    fn prior_candidate_readiness_fixture_paired() {
        let c = make_projection("some_kind", &["pos1"], &["guard1"]);
        assert_eq!(prior_candidate_readiness(&c), "fixture_paired_review_ready");
    }

    #[test]
    fn prior_candidate_readiness_needs_guard_fixture() {
        let c = make_projection("some_kind", &["pos1"], &[]);
        assert_eq!(prior_candidate_readiness(&c), "needs_guard_fixture");
    }

    #[test]
    fn prior_candidate_readiness_needs_positive_fixture() {
        let c = make_projection("some_kind", &[], &["guard1"]);
        assert_eq!(prior_candidate_readiness(&c), "needs_positive_fixture");
    }

    #[test]
    fn prior_candidate_readiness_needs_fixture_coverage() {
        let c = make_projection("some_kind", &[], &[]);
        assert_eq!(prior_candidate_readiness(&c), "needs_fixture_coverage");
    }

    #[test]
    fn prior_candidate_readiness_negative_without_guards_is_not_caution() {
        // negative_knowledge_prior with empty guard_fixtures → falls through to match
        let c = make_projection("negative_knowledge_prior", &["pos1"], &[]);
        assert_eq!(prior_candidate_readiness(&c), "needs_guard_fixture");
    }

    // --- push_fixture_evidence ---

    #[test]
    fn push_fixture_evidence_empty() {
        let mut rendered = String::new();
        push_fixture_evidence(&mut rendered, "positive_gates", &BTreeSet::new());
        assert_eq!(rendered, "- positive_gates:\n  - none\n");
    }

    #[test]
    fn render_prior_candidates_keeps_large_fixture_sets_line_bounded() {
        let fixtures = (0..64)
            .map(|index| format!("fixture_{index:02}_with_a_deliberately_reviewable_name"))
            .collect::<BTreeSet<_>>();
        let candidate = PriorCandidateProjection {
            candidate_kind: "test_prior",
            target_schema: "CorpusMemory.test_priors",
            required_gates: "typed schema; paired coverage; local consumer",
            schema_gate: "schema_gate",
            fixture_gate: "fixture_gate",
            harvest_gate: "harvest_gate",
            consumer_gate: "consumer_gate",
            supporting_fixtures: fixtures.clone(),
            positive_fixtures: fixtures.clone(),
            guard_fixtures: fixtures,
        };

        let rendered = render_prior_candidate_block(&[candidate]);
        assert!(
            rendered.contains("| `test_prior` | `CorpusMemory.test_priors` | `64` | `64` | `64` |")
        );
        assert!(rendered.contains("  - `fixture_00_with_a_deliberately_reviewable_name`"));
        assert!(rendered.contains("  - `fixture_63_with_a_deliberately_reviewable_name`"));
        assert!(rendered.lines().all(|line| line.len() < 512));
    }

    #[test]
    fn render_kg_fixtures_compacts_rows_but_preserves_failure_details() {
        let entry = KgFixtureProjection {
            outcome: KgBenchFixtureOutcome {
                name: "broken_fixture".to_string(),
                fixture_path: PathBuf::from("fixtures/broken_fixture/fixture.json"),
                failures: vec!["expected pass | observed fail".to_string()],
            },
            display_path: "fixtures/broken_fixture/fixture.json".to_string(),
        };

        let rendered = render_kg_fixtures_block(&[entry]);
        assert!(
            rendered
                .contains("| `broken_fixture` | `fail` | `fixtures/broken_fixture/fixture.json` |")
        );
        assert!(rendered.contains("Failed fixture details:\n- `broken_fixture`:"));
        assert!(
            rendered
                .contains("Failed fixture family members:\n- uncategorized:\n  - `broken_fixture`")
        );
        assert!(rendered.contains("  - expected pass \\| observed fail"));
        assert!(!rendered.contains("- failures:\n"));
    }

    // --- escape_markdown_line ---

    #[test]
    fn escape_markdown_line_replaces_newlines() {
        assert_eq!(escape_markdown_line("line1\nline2"), "line1 line2");
    }

    #[test]
    fn escape_markdown_line_escapes_pipes() {
        assert_eq!(escape_markdown_line("a|b"), "a\\|b");
    }

    #[test]
    fn escape_markdown_line_no_special_chars() {
        assert_eq!(escape_markdown_line("hello world"), "hello world");
    }

    // --- document_key_from_report_path ---

    #[test]
    fn document_key_from_report_path_extracts_parent_dir_name() {
        let p = std::path::Path::new("generated/intent_ir/my_document/validation_report.json");
        assert_eq!(document_key_from_report_path(p), "my_document");
    }

    #[test]
    fn document_key_from_report_path_falls_back_for_root_file() {
        let p = std::path::Path::new("validation_report.json");
        assert_eq!(document_key_from_report_path(p), "unknown_document");
    }

    // --- replace_managed_block ---

    #[test]
    fn replace_managed_block_inserts_when_start_not_found() -> Result<()> {
        let existing = "# Title\n\nSome human text.\n";
        let managed = "| Col1 | Col2 |\n|------|------|\n| a | b |\n";
        let result = replace_managed_block(
            existing,
            managed,
            "<!-- MANAGED_START -->",
            "<!-- MANAGED_END -->",
            "## Managed Section\n",
        )?;
        assert!(result.contains("<!-- MANAGED_START -->") || result.contains("## Managed Section"));
        assert!(result.contains("| Col1 | Col2 |"));
        Ok(())
    }

    #[test]
    fn replace_managed_block_replaces_existing_managed_section() -> Result<()> {
        let existing = "# Title\n\nHuman text.\n\n<!-- MANAGED_START -->\nold content\n<!-- MANAGED_END -->\n\nMore human text.\n";
        let managed = "new content\n";
        let result = replace_managed_block(
            existing,
            managed,
            "<!-- MANAGED_START -->",
            "<!-- MANAGED_END -->",
            "## Managed Section\n",
        )?;
        assert!(result.contains("new content"));
        assert!(!result.contains("old content"));
        assert!(result.contains("More human text."));
        assert!(result.contains("Human text."));
        Ok(())
    }

    #[test]
    fn replace_managed_block_errors_when_end_not_found() {
        let existing = "<!-- MANAGED_START -->\ncontent\nno end marker";
        let result = replace_managed_block(
            existing,
            "new",
            "<!-- MANAGED_START -->",
            "<!-- MANAGED_END -->",
            "## Managed\n",
        );
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("has `<!-- MANAGED_START -->` but no `<!-- MANAGED_END -->`")
        );
    }
}
