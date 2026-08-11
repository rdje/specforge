//! Isolated, repository-local replay of the four source-to-IntentIR stages.
//!
//! `SPEC-TO-INTENT-ALIGNMENT.6a` needs current-binary evidence without overwriting the
//! canonical `generated/` artifacts or the frozen `.4c` baseline. This module accepts only
//! repository-relative inputs and a fresh output root below `.project-data/tmp/`, then runs
//! SourceIR → EvidenceIR → SemanticIR → IntentIR with no provider calls.

use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::evidence::EvidenceIr;
use super::intent::IntentIr;
use super::semantic::SemanticIr;
use super::source::{SourceIr, TableKind};
use crate::error::{AppError, Result};

const REPLAY_ROOT: &str = ".project-data/tmp";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceToIntentReplayRequest {
    pub source: PathBuf,
    pub output_root: PathBuf,
    pub prior_memory: Option<PathBuf>,
    pub observed_table_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayStageArtifact {
    pub path: String,
    pub total_timing_constraints: usize,
    pub observed_timing_constraints: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceToIntentReplayReport {
    pub schema_version: u32,
    pub source: String,
    pub output_root: String,
    pub prior_memory: Option<String>,
    pub document_key: String,
    pub observed_table_id: Option<String>,
    pub observed_source_table_kind: Option<TableKind>,
    pub source_ir_path: String,
    pub evidence_ir: ReplayStageArtifact,
    pub semantic_ir: ReplayStageArtifact,
    pub intent_ir: ReplayStageArtifact,
}

/// Run all four deterministic stages below a fresh repository-owned scratch root.
pub fn replay_source_to_intent(
    request: &SourceToIntentReplayRequest,
) -> Result<SourceToIntentReplayReport> {
    validate_request(request)?;
    let repository = crate::project_data::repository_root()?;
    validate_existing_repository_file(&repository, &request.source, "source")?;
    if let Some(prior_memory) = &request.prior_memory {
        validate_existing_repository_file(&repository, prior_memory, "prior-memory")?;
    }
    let output_root = repository.join(&request.output_root);
    if output_root.exists() {
        return Err(AppError::InvalidStageArtifact(format!(
            "source-to-IntentIR replay output root already exists: {}",
            request.output_root.display()
        )));
    }
    fs::create_dir_all(&output_root)?;

    let source_root = output_root.join("source_ir");
    let evidence_root = output_root.join("evidence_ir");
    let semantic_root = output_root.join("semantic_ir");
    let intent_root = output_root.join("intent_ir");

    let mut source_ir = SourceIr::build(&request.source, &source_root)?;
    source_ir.materialize()?;
    source_ir.write_to_disk()?;
    let source_ir_path = source_ir.artifact_layout.source_ir_path.clone();
    let source_ir = SourceIr::load_from_path(&source_ir_path)?;

    let evidence_ir = EvidenceIr::build_with_prior_memory(
        &source_ir_path,
        &evidence_root,
        request.prior_memory.as_deref(),
    )?;
    evidence_ir.write_to_disk()?;
    let evidence_ir_path = evidence_ir.artifact_layout.evidence_ir_path.clone();

    let semantic_ir = SemanticIr::build(&evidence_ir_path, &semantic_root)?;
    semantic_ir.write_to_disk()?;
    let semantic_ir_path = semantic_ir.artifact_layout.semantic_ir_path.clone();

    let intent_ir = IntentIr::build(&semantic_ir_path, &intent_root)?;
    intent_ir.write_to_disk()?;
    let intent_ir_path = intent_ir.artifact_layout.intent_ir_path.clone();

    let observed_table_id = request.observed_table_id.as_deref();
    let observed_prefix = observed_table_id.map(|table_id| {
        format!(
            "timing_{}_",
            table_id
                .chars()
                .map(|character| {
                    if character.is_ascii_alphanumeric() {
                        character.to_ascii_lowercase()
                    } else {
                        '_'
                    }
                })
                .collect::<String>()
                .trim_matches('_')
        )
    });
    let observed_count = |ids: &[String]| {
        observed_prefix.as_deref().map_or(0, |prefix| {
            ids.iter().filter(|id| id.starts_with(prefix)).count()
        })
    };

    Ok(SourceToIntentReplayReport {
        schema_version: 1,
        source: request.source.to_string_lossy().into_owned(),
        output_root: request.output_root.to_string_lossy().into_owned(),
        prior_memory: request
            .prior_memory
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned()),
        document_key: source_ir.document_identity.document_key.clone(),
        observed_table_id: request.observed_table_id.clone(),
        observed_source_table_kind: observed_table_id.and_then(|table_id| {
            source_ir
                .structured_tables
                .iter()
                .find(|table| table.table_id == table_id)
                .map(|table| table.table_kind)
        }),
        source_ir_path: repository_relative(&repository, &source_ir_path)?,
        evidence_ir: stage_artifact(
            &repository,
            &evidence_ir_path,
            evidence_ir.timing_constraints.len(),
            observed_count(
                &evidence_ir
                    .timing_constraints
                    .iter()
                    .map(|constraint| constraint.constraint_id.clone())
                    .collect::<Vec<_>>(),
            ),
        )?,
        semantic_ir: stage_artifact(
            &repository,
            &semantic_ir_path,
            semantic_ir.timing_constraints.len(),
            observed_count(
                &semantic_ir
                    .timing_constraints
                    .iter()
                    .map(|constraint| constraint.constraint_id.clone())
                    .collect::<Vec<_>>(),
            ),
        )?,
        intent_ir: stage_artifact(
            &repository,
            &intent_ir_path,
            intent_ir.timing_constraints.len(),
            observed_count(
                &intent_ir
                    .timing_constraints
                    .iter()
                    .map(|constraint| constraint.constraint_id.clone())
                    .collect::<Vec<_>>(),
            ),
        )?,
    })
}

fn validate_request(request: &SourceToIntentReplayRequest) -> Result<()> {
    validate_relative_path(&request.source, "source")?;
    validate_relative_path(&request.output_root, "output root")?;
    if let Some(prior_memory) = &request.prior_memory {
        validate_relative_path(prior_memory, "prior-memory")?;
    }
    if !request.output_root.starts_with(REPLAY_ROOT)
        || request.output_root == Path::new(REPLAY_ROOT)
    {
        return Err(AppError::InvalidStageArtifact(format!(
            "replay output root must be a child of {REPLAY_ROOT}: {}",
            request.output_root.display()
        )));
    }
    if request
        .observed_table_id
        .as_ref()
        .is_some_and(|table_id| table_id.trim().is_empty())
    {
        return Err(AppError::InvalidStageArtifact(
            "observed table id must not be empty".to_string(),
        ));
    }
    Ok(())
}

fn validate_relative_path(path: &Path, label: &str) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(AppError::InvalidStageArtifact(format!(
            "replay {label} must be repository-relative: {}",
            path.display()
        )));
    }
    Ok(())
}

fn validate_existing_repository_file(repository: &Path, path: &Path, label: &str) -> Result<()> {
    let absolute = repository.join(path).canonicalize().map_err(|error| {
        AppError::InvalidStageArtifact(format!(
            "replay {label} cannot be resolved inside the repository: {} ({error})",
            path.display()
        ))
    })?;
    if !absolute.starts_with(repository) || !absolute.is_file() {
        return Err(AppError::InvalidStageArtifact(format!(
            "replay {label} must resolve to a repository-contained file: {}",
            path.display()
        )));
    }
    Ok(())
}

fn stage_artifact(
    repository: &Path,
    path: &Path,
    total_timing_constraints: usize,
    observed_timing_constraints: usize,
) -> Result<ReplayStageArtifact> {
    Ok(ReplayStageArtifact {
        path: repository_relative(repository, path)?,
        total_timing_constraints,
        observed_timing_constraints,
    })
}

fn repository_relative(repository: &Path, path: &Path) -> Result<String> {
    let absolute = repository.join(path).canonicalize()?;
    let relative = absolute.strip_prefix(repository).map_err(|_| {
        AppError::InvalidStageArtifact(format!(
            "replay artifact escaped the repository: {}",
            path.display()
        ))
    })?;
    Ok(relative.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(output_root: &str) -> SourceToIntentReplayRequest {
        SourceToIntentReplayRequest {
            source: PathBuf::from(".cache/local-references/example.pdf"),
            output_root: PathBuf::from(output_root),
            prior_memory: Some(PathBuf::from("generated/prior_memory/corpus_memory.json")),
            observed_table_id: Some("table_0004".to_string()),
        }
    }

    #[test]
    fn replay_paths_must_be_relative_and_below_project_tmp() {
        assert!(validate_request(&request(".project-data/tmp/replay-a")).is_ok());
        assert!(validate_request(&request("generated/replay-a")).is_err());
        assert!(validate_request(&request(".project-data/tmp")).is_err());

        let mut absolute = request(".project-data/tmp/replay-a");
        absolute.source = PathBuf::from("/external/spec.pdf");
        assert!(validate_request(&absolute).is_err());

        let mut traversal = request(".project-data/tmp/replay-a");
        traversal.prior_memory = Some(PathBuf::from("../prior.json"));
        assert!(validate_request(&traversal).is_err());
    }

    #[test]
    fn replay_rejects_a_symlink_escape_before_creating_output() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let escape = Path::new(".cache/local-references");
        if repository.join(escape).is_symlink() {
            assert!(
                validate_existing_repository_file(&repository, escape, "source").is_err(),
                "a repository symlink to external source authority must be copied locally before replay"
            );
        }
        Ok(())
    }
}
