use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{AppError, Result};

pub(crate) const TEMP_ROOT: &str = ".project-data/tmp";
pub(crate) const CACHE_ROOT: &str = ".cache";

#[derive(Debug, Clone)]
pub(crate) struct ProjectDataRoots {
    pub(crate) repository: PathBuf,
    pub(crate) temporary: PathBuf,
    pub(crate) xdg_cache: PathBuf,
    pub(crate) huggingface: PathBuf,
    pub(crate) pip: PathBuf,
    pub(crate) torch: PathBuf,
    pub(crate) matplotlib: PathBuf,
}

pub(crate) fn prepare() -> Result<ProjectDataRoots> {
    prepare_at(&repository_root()?)
}

pub(crate) fn tempdir() -> Result<tempfile::TempDir> {
    let roots = prepare()?;
    Ok(tempfile::Builder::new()
        .prefix("specforge-")
        .tempdir_in(roots.temporary)?)
}

pub(crate) fn configure_command(command: &mut Command) -> Result<()> {
    let roots = prepare()?;
    command
        .env("SPECFORGE_REPO_ROOT", &roots.repository)
        .env("TMPDIR", &roots.temporary)
        .env("TMP", &roots.temporary)
        .env("TEMP", &roots.temporary)
        .env("XDG_CACHE_HOME", &roots.xdg_cache)
        .env("HF_HOME", &roots.huggingface)
        .env("HUGGINGFACE_HUB_CACHE", roots.huggingface.join("hub"))
        .env("PIP_CACHE_DIR", &roots.pip)
        .env("TORCH_HOME", &roots.torch)
        .env("MPLCONFIGDIR", &roots.matplotlib);
    Ok(())
}

pub(crate) fn repository_root() -> Result<PathBuf> {
    if let Some(explicit) = env::var_os("SPECFORGE_REPO_ROOT") {
        return validate_repository_root(Path::new(&explicit));
    }

    let current = env::current_dir()?;
    if let Some(root) = discover_repository_root(&current) {
        return validate_repository_root(&root);
    }

    if let Ok(executable) = env::current_exe()
        && let Some(root) = discover_repository_root(&executable)
    {
        return validate_repository_root(&root);
    }

    Err(AppError::InvalidStageArtifact(
        "cannot locate the SpecForge repository root; run inside the repository or set \
         SPECFORGE_REPO_ROOT to the repository root"
            .to_string(),
    ))
}

fn discover_repository_root(start: &Path) -> Option<PathBuf> {
    let start = if start.is_file() {
        start.parent()?
    } else {
        start
    };
    start
        .ancestors()
        .find(|candidate| repository_markers_exist(candidate))
        .map(Path::to_path_buf)
}

fn validate_repository_root(candidate: &Path) -> Result<PathBuf> {
    let candidate = candidate.canonicalize()?;
    if !repository_markers_exist(&candidate) {
        return Err(AppError::InvalidStageArtifact(format!(
            "SPECFORGE_REPO_ROOT does not identify a SpecForge repository: {}",
            candidate.display()
        )));
    }
    Ok(candidate)
}

fn repository_markers_exist(candidate: &Path) -> bool {
    candidate.join("Cargo.toml").is_file()
        && candidate.join("crates/specforge/Cargo.toml").is_file()
        && candidate.join("scripts/check_doctrines.sh").is_file()
}

fn prepare_at(repository: &Path) -> Result<ProjectDataRoots> {
    let roots = ProjectDataRoots {
        repository: repository.to_path_buf(),
        temporary: repository.join(TEMP_ROOT),
        xdg_cache: repository.join(CACHE_ROOT).join("xdg"),
        huggingface: repository.join(CACHE_ROOT).join("huggingface"),
        pip: repository.join(CACHE_ROOT).join("pip"),
        torch: repository.join(CACHE_ROOT).join("torch"),
        matplotlib: repository.join(CACHE_ROOT).join("matplotlib"),
    };

    for path in [
        &roots.temporary,
        &roots.xdg_cache,
        &roots.huggingface,
        &roots.pip,
        &roots.torch,
        &roots.matplotlib,
    ] {
        std::fs::create_dir_all(path)?;
        let canonical = path.canonicalize()?;
        if !canonical.starts_with(&roots.repository) {
            return Err(AppError::InvalidStageArtifact(format!(
                "project-data path {} escapes the repository",
                path.display()
            )));
        }
        ensure_same_filesystem(&roots.repository, path)?;
    }

    Ok(roots)
}

#[cfg(unix)]
fn ensure_same_filesystem(repository: &Path, project_data: &Path) -> Result<()> {
    use std::os::unix::fs::MetadataExt;

    let repository_device = std::fs::metadata(repository)?.dev();
    let data_device = std::fs::metadata(project_data)?.dev();
    if repository_device != data_device {
        return Err(AppError::InvalidStageArtifact(format!(
            "project-data path {} is not on the repository filesystem",
            project_data.display()
        )));
    }
    Ok(())
}

#[cfg(not(unix))]
fn ensure_same_filesystem(_repository: &Path, _project_data: &Path) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_the_repository_from_a_nested_path() {
        let root = repository_root().expect("repository root");
        assert_eq!(
            discover_repository_root(&root.join("crates/specforge/src")),
            Some(root)
        );
    }

    #[test]
    fn creates_temporary_directories_below_the_repository() {
        let root = repository_root().expect("repository root");
        let temporary = tempdir().expect("project-local tempdir");
        assert!(temporary.path().starts_with(root.join(TEMP_ROOT)));
        ensure_same_filesystem(&root, temporary.path()).expect("same filesystem");
    }

    #[test]
    fn gives_children_only_repository_local_project_data_roots() {
        let roots = prepare().expect("project-data roots");
        let mut command = Command::new("true");
        configure_command(&mut command).expect("command environment");
        let environment = command
            .get_envs()
            .map(|(key, value)| (key.to_owned(), value.map(ToOwned::to_owned)))
            .collect::<std::collections::BTreeMap<_, _>>();

        for key in [
            "TMPDIR",
            "TMP",
            "TEMP",
            "XDG_CACHE_HOME",
            "HF_HOME",
            "HUGGINGFACE_HUB_CACHE",
            "PIP_CACHE_DIR",
            "TORCH_HOME",
            "MPLCONFIGDIR",
        ] {
            let value = environment
                .get(std::ffi::OsStr::new(key))
                .and_then(|value| value.as_ref())
                .expect("configured project-data environment");
            assert!(Path::new(value).starts_with(&roots.repository), "{key}");
        }
    }
}
