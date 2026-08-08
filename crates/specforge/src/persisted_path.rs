use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};

const LEGACY_REPOSITORY_ROOTS: &[&str] = &[
    "generated",
    "corpus",
    ".project-data",
    ".cache",
    "target",
    ".venv-docling",
    ".venv-eval",
];

/// Declares whether a persisted path is owned by this repository or is an
/// explicitly authorized input outside it.
///
/// The declaration is deliberately supplied by the typed artifact field's
/// producer or consumer rather than inferred from an absolute path. That keeps
/// an unrelated external path from masquerading as a moved repository path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PersistedPathOrigin {
    RepositoryOwned,
    ExternalInput,
}

/// Converts a runtime path to its persisted representation.
///
/// Repository-owned paths are always returned relative to the current
/// repository root. Explicit external inputs remain absolute unless their
/// canonical target is inside the repository. Repository outputs do not need
/// to exist yet, but their nearest existing ancestor must remain inside the
/// repository.
pub(crate) fn encode(path: &Path, origin: PersistedPathOrigin) -> Result<PathBuf> {
    encode_at(path, origin, &crate::project_data::repository_root()?)
}

/// Resolves an existing persisted path for runtime I/O.
///
/// Relative paths are anchored at the current repository root. Legacy absolute
/// repository paths may rebase only through a recognized project-data root and
/// only when exactly one existing target is found. Explicit external inputs
/// never use that compatibility path.
pub(crate) fn resolve_existing(path: &Path, origin: PersistedPathOrigin) -> Result<PathBuf> {
    resolve_existing_at(path, origin, &crate::project_data::repository_root()?)
}

fn encode_at(path: &Path, origin: PersistedPathOrigin, repository: &Path) -> Result<PathBuf> {
    let repository = canonical_repository(repository)?;
    match origin {
        PersistedPathOrigin::RepositoryOwned => encode_repository_path(path, &repository),
        PersistedPathOrigin::ExternalInput => encode_external_path(path, &repository),
    }
}

fn resolve_existing_at(
    path: &Path,
    origin: PersistedPathOrigin,
    repository: &Path,
) -> Result<PathBuf> {
    let repository = canonical_repository(repository)?;
    match origin {
        PersistedPathOrigin::RepositoryOwned => resolve_repository_path(path, &repository, true),
        PersistedPathOrigin::ExternalInput => resolve_external_path(path, &repository),
    }
}

fn canonical_repository(repository: &Path) -> Result<PathBuf> {
    let canonical = canonicalize_required(repository)?;
    if !canonical.is_dir() {
        return Err(invalid_path(format!(
            "repository path is not a directory: {}",
            repository.display()
        )));
    }
    Ok(canonical)
}

fn encode_repository_path(path: &Path, repository: &Path) -> Result<PathBuf> {
    reject_lexical_escape(path)?;

    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        repository.join(clean_relative(path)?)
    };

    if absolute.exists() {
        let canonical = canonicalize_required(&absolute)?;
        return relative_below_repository(&canonical, repository);
    }

    let relative = absolute.strip_prefix(repository).map_err(|_| {
        invalid_path(format!(
            "repository-owned path is outside the current repository: {}",
            path.display()
        ))
    })?;
    let relative = clean_relative(relative)?;
    ensure_existing_ancestor_is_local(&repository.join(&relative), repository)?;
    Ok(relative)
}

fn encode_external_path(path: &Path, repository: &Path) -> Result<PathBuf> {
    if path.is_relative() {
        return encode_repository_path(path, repository);
    }

    reject_lexical_escape(path)?;
    let canonical = canonicalize_required(path)?;
    if canonical.starts_with(repository) {
        relative_below_repository(&canonical, repository)
    } else {
        Ok(canonical)
    }
}

fn resolve_repository_path(
    path: &Path,
    repository: &Path,
    allow_legacy_rebase: bool,
) -> Result<PathBuf> {
    if path.is_relative() {
        return resolve_current_relative(path, repository);
    }

    reject_lexical_escape(path)?;
    if let Ok(relative) = path.strip_prefix(repository) {
        return resolve_current_relative(relative, repository);
    }

    if !allow_legacy_rebase {
        return Err(invalid_path(format!(
            "absolute repository path is outside the current repository: {}",
            path.display()
        )));
    }

    resolve_legacy_repository_path(path, repository)
}

fn resolve_external_path(path: &Path, repository: &Path) -> Result<PathBuf> {
    if path.is_relative() {
        return resolve_current_relative(path, repository);
    }

    reject_lexical_escape(path)?;
    canonicalize_required(path)
}

fn resolve_current_relative(path: &Path, repository: &Path) -> Result<PathBuf> {
    let relative = clean_relative(path)?;
    let canonical = canonicalize_required(&repository.join(relative))?;
    if !canonical.starts_with(repository) {
        return Err(invalid_path(format!(
            "persisted repository path escapes through a symlink: {}",
            path.display()
        )));
    }
    Ok(canonical)
}

fn resolve_legacy_repository_path(path: &Path, repository: &Path) -> Result<PathBuf> {
    let normal_components = path
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut targets = BTreeSet::new();

    for (index, component) in normal_components.iter().enumerate() {
        if !is_legacy_repository_root(component) {
            continue;
        }
        let relative = normal_components[index..].iter().collect::<PathBuf>();
        let candidate = repository.join(&relative);
        if !candidate.exists() {
            continue;
        }
        let canonical = canonicalize_required(&candidate)?;
        if !canonical.starts_with(repository) {
            return Err(invalid_path(format!(
                "legacy repository path resolves through an escaping symlink: {}",
                relative.display()
            )));
        }
        targets.insert(canonical);
    }

    match targets.len() {
        0 => Err(AppError::MissingPath(path.to_path_buf())),
        1 => Ok(targets.pop_first().expect("one legacy path target")),
        count => Err(invalid_path(format!(
            "legacy repository path has {count} valid rebasing targets: {}",
            path.display()
        ))),
    }
}

fn is_legacy_repository_root(component: &OsStr) -> bool {
    LEGACY_REPOSITORY_ROOTS
        .iter()
        .any(|root| component == OsStr::new(root))
}

fn relative_below_repository(path: &Path, repository: &Path) -> Result<PathBuf> {
    let relative = path.strip_prefix(repository).map_err(|_| {
        invalid_path(format!(
            "repository-owned path escapes the current repository: {}",
            path.display()
        ))
    })?;
    clean_relative(relative)
}

fn clean_relative(path: &Path) -> Result<PathBuf> {
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(value) => clean.push(value),
            Component::CurDir => {
                return Err(invalid_path(format!(
                    "persisted path contains a current-directory component: {}",
                    path.display()
                )));
            }
            Component::ParentDir => {
                return Err(invalid_path(format!(
                    "persisted path contains parent traversal: {}",
                    path.display()
                )));
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(invalid_path(format!(
                    "persisted repository path is not relative: {}",
                    path.display()
                )));
            }
        }
    }
    if clean.as_os_str().is_empty() {
        return Err(invalid_path("persisted path is empty".to_string()));
    }
    Ok(clean)
}

fn reject_lexical_escape(path: &Path) -> Result<()> {
    for component in path.components() {
        if matches!(component, Component::ParentDir | Component::CurDir) {
            return Err(invalid_path(format!(
                "persisted path contains a traversal component: {}",
                path.display()
            )));
        }
    }
    Ok(())
}

fn ensure_existing_ancestor_is_local(path: &Path, repository: &Path) -> Result<()> {
    let mut ancestor = path;
    while !ancestor.exists() {
        ancestor = ancestor.parent().ok_or_else(|| {
            invalid_path(format!(
                "repository-owned path has no existing ancestor: {}",
                path.display()
            ))
        })?;
    }
    let canonical = canonicalize_required(ancestor)?;
    if !canonical.starts_with(repository) {
        return Err(invalid_path(format!(
            "repository-owned path escapes through an ancestor symlink: {}",
            path.display()
        )));
    }
    Ok(())
}

fn canonicalize_required(path: &Path) -> Result<PathBuf> {
    match fs::canonicalize(path) {
        Ok(canonical) => Ok(canonical),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            Err(AppError::MissingPath(path.to_path_buf()))
        }
        Err(error) => Err(error.into()),
    }
}

fn invalid_path(message: String) -> AppError {
    AppError::InvalidStageArtifact(format!("persisted path contract violation: {message}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roots() -> (tempfile::TempDir, PathBuf, PathBuf) {
        let workspace = crate::project_data::tempdir().expect("repository-local test directory");
        let repository = workspace.path().join("current");
        let external = workspace.path().join("external");
        fs::create_dir_all(&repository).expect("current repository fixture");
        fs::create_dir_all(&external).expect("external fixture");
        (workspace, repository, external)
    }

    fn write_file(path: &Path) {
        fs::create_dir_all(path.parent().expect("fixture parent")).expect("fixture directory");
        fs::write(path, b"fixture").expect("fixture file");
    }

    #[test]
    fn repository_paths_encode_relative_and_resolve_from_the_current_root() {
        let (_workspace, repository, _external) = test_roots();
        let artifact = repository.join("generated/source_ir/spec/source_ir.json");
        write_file(&artifact);

        let encoded = encode_at(&artifact, PersistedPathOrigin::RepositoryOwned, &repository)
            .expect("encode repository path");
        assert_eq!(
            encoded,
            Path::new("generated/source_ir/spec/source_ir.json")
        );
        assert_eq!(
            resolve_existing_at(&encoded, PersistedPathOrigin::RepositoryOwned, &repository)
                .expect("resolve repository path"),
            artifact.canonicalize().expect("canonical artifact")
        );
    }

    #[test]
    fn repository_output_can_encode_before_its_leaf_exists() {
        let (_workspace, repository, _external) = test_roots();
        let output = repository.join("generated/intent_ir/spec/intent_ir.json");

        assert_eq!(
            encode_at(&output, PersistedPathOrigin::RepositoryOwned, &repository)
                .expect("encode future output"),
            Path::new("generated/intent_ir/spec/intent_ir.json")
        );
    }

    #[test]
    fn current_absolute_repository_path_remains_loadable() {
        let (_workspace, repository, _external) = test_roots();
        let artifact = repository.join("generated/semantic_ir/spec/semantic_ir.json");
        write_file(&artifact);

        assert_eq!(
            resolve_existing_at(&artifact, PersistedPathOrigin::RepositoryOwned, &repository)
                .expect("resolve current absolute path"),
            artifact.canonicalize().expect("canonical artifact")
        );
    }

    #[test]
    fn legacy_repository_path_rebases_to_one_existing_current_target() {
        let (workspace, repository, _external) = test_roots();
        let current = repository.join("generated/evidence_ir/spec/evidence_ir.json");
        write_file(&current);
        let retired = workspace
            .path()
            .join("retired/specforge/generated/evidence_ir/spec/evidence_ir.json");

        assert_eq!(
            resolve_existing_at(&retired, PersistedPathOrigin::RepositoryOwned, &repository)
                .expect("rebase legacy path"),
            current.canonicalize().expect("canonical current artifact")
        );
    }

    #[test]
    fn external_input_never_uses_legacy_rebasing() {
        let (workspace, repository, _external) = test_roots();
        write_file(&repository.join("corpus/vendor/spec.pdf"));
        let missing_external = workspace.path().join("retired/corpus/vendor/spec.pdf");

        assert!(matches!(
            resolve_existing_at(
                &missing_external,
                PersistedPathOrigin::ExternalInput,
                &repository
            ),
            Err(AppError::MissingPath(path)) if path == missing_external
        ));
    }

    #[test]
    fn explicit_external_input_stays_absolute() {
        let (_workspace, repository, external) = test_roots();
        let source = external.join("authorized/spec.pdf");
        write_file(&source);

        let encoded = encode_at(&source, PersistedPathOrigin::ExternalInput, &repository)
            .expect("encode external input");
        assert!(encoded.is_absolute());
        assert_eq!(encoded, source.canonicalize().expect("canonical source"));
        assert_eq!(
            resolve_existing_at(&encoded, PersistedPathOrigin::ExternalInput, &repository)
                .expect("resolve external input"),
            encoded
        );
    }

    #[test]
    fn input_inside_the_repository_encodes_relative_even_when_declared_external() {
        let (_workspace, repository, _external) = test_roots();
        let source = repository.join("corpus/vendor/spec.pdf");
        write_file(&source);

        assert_eq!(
            encode_at(&source, PersistedPathOrigin::ExternalInput, &repository)
                .expect("encode repository-local input"),
            Path::new("corpus/vendor/spec.pdf")
        );
    }

    #[test]
    fn origin_labels_have_stable_schema_values() {
        assert_eq!(
            serde_json::to_string(&PersistedPathOrigin::RepositoryOwned)
                .expect("serialize repository origin"),
            "\"repository_owned\""
        );
        assert_eq!(
            serde_json::from_str::<PersistedPathOrigin>("\"external_input\"")
                .expect("deserialize external origin"),
            PersistedPathOrigin::ExternalInput
        );
    }

    #[test]
    fn repository_owned_path_outside_the_root_is_refused() {
        let (_workspace, repository, external) = test_roots();
        let source = external.join("not-project-data.json");
        write_file(&source);

        assert!(matches!(
            encode_at(&source, PersistedPathOrigin::RepositoryOwned, &repository),
            Err(AppError::InvalidStageArtifact(_))
        ));
    }

    #[test]
    fn parent_traversal_is_refused() {
        let (_workspace, repository, _external) = test_roots();

        assert!(matches!(
            resolve_existing_at(
                Path::new("generated/../Cargo.toml"),
                PersistedPathOrigin::RepositoryOwned,
                &repository
            ),
            Err(AppError::InvalidStageArtifact(_))
        ));
    }

    #[test]
    fn ambiguous_legacy_suffix_is_refused() {
        let (workspace, repository, _external) = test_roots();
        write_file(&repository.join("generated/corpus/spec.pdf"));
        write_file(&repository.join("corpus/spec.pdf"));
        let retired = workspace.path().join("retired/generated/corpus/spec.pdf");

        assert!(matches!(
            resolve_existing_at(
                &retired,
                PersistedPathOrigin::RepositoryOwned,
                &repository
            ),
            Err(AppError::InvalidStageArtifact(message)) if message.contains("2 valid rebasing targets")
        ));
    }

    #[test]
    fn missing_legacy_target_is_reported_without_guessing() {
        let (workspace, repository, _external) = test_roots();
        let retired = workspace
            .path()
            .join("retired/generated/source_ir/missing/source_ir.json");

        assert!(matches!(
            resolve_existing_at(
                &retired,
                PersistedPathOrigin::RepositoryOwned,
                &repository
            ),
            Err(AppError::MissingPath(path)) if path == retired
        ));
    }

    #[cfg(unix)]
    #[test]
    fn symlink_escape_is_refused_for_storage_and_resolution() {
        use std::os::unix::fs::symlink;

        let (_workspace, repository, external) = test_roots();
        let outside = external.join("secret.json");
        write_file(&outside);
        fs::create_dir_all(repository.join("generated")).expect("generated fixture root");
        symlink(&external, repository.join("generated/escape")).expect("escape symlink");
        let escaped = repository.join("generated/escape/secret.json");

        assert!(matches!(
            encode_at(&escaped, PersistedPathOrigin::RepositoryOwned, &repository),
            Err(AppError::InvalidStageArtifact(_))
        ));
        assert!(matches!(
            resolve_existing_at(
                Path::new("generated/escape/secret.json"),
                PersistedPathOrigin::RepositoryOwned,
                &repository
            ),
            Err(AppError::InvalidStageArtifact(_))
        ));
    }
}
