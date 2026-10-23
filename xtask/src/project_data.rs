//! The repository's project-data roots, as `scripts/project_data_env.sh` activates them
//! (`RUSTDOC-AND-XTASK.5`): temporary and cache directories under the repository, on its filesystem, exported to
//! every child process.

use std::fs;
use std::path::{Path, PathBuf};

/// The environment a command's children get: each variable and its repository-derived value.
pub type Environment = Vec<(&'static str, PathBuf)>;

/// The variables `specforge_activate_project_data` exports, relative to the repository root.
fn variables(root: &Path) -> Environment {
    let tmp = root.join(".project-data/tmp");
    let hf = root.join(".cache/huggingface");
    vec![
        ("SPECFORGE_REPO_ROOT", root.to_path_buf()),
        ("TMPDIR", tmp.clone()),
        ("TMP", tmp.clone()),
        ("TEMP", tmp),
        ("XDG_CACHE_HOME", root.join(".cache/xdg")),
        ("HF_HOME", hf.clone()),
        ("HUGGINGFACE_HUB_CACHE", hf.join("hub")),
        ("PIP_CACHE_DIR", root.join(".cache/pip")),
        ("TORCH_HOME", root.join(".cache/torch")),
        ("MPLCONFIGDIR", root.join(".cache/matplotlib")),
    ]
}

/// The directories the activation creates, in its order.
const CREATED: &[&str] = &[
    "TMPDIR",
    "XDG_CACHE_HOME",
    "HUGGINGFACE_HUB_CACHE",
    "PIP_CACHE_DIR",
    "TORCH_HOME",
    "MPLCONFIGDIR",
];

/// Activate the project-data roots under `root`: refuse a directory that is no SpecForge repository, create the
/// directories, and refuse one that escapes the repository or sits on another filesystem. Returns the environment
/// to export, or the error message the shell activation prints.
pub fn activate(root: &Path) -> Result<Environment, String> {
    let root = fs::canonicalize(root)
        .map_err(|error| format!("error: cannot resolve {}: {error}", root.display()))?;
    if !root.join("Cargo.toml").is_file() || !root.join("README.md").is_file() {
        return Err(format!(
            "error: project-data root is not a SpecForge repository: {}",
            root.display()
        ));
    }
    let environment = variables(&root);
    let path_of = |name: &str| -> &PathBuf {
        &environment
            .iter()
            .find(|(variable, _)| *variable == name)
            .expect("a created directory is an exported variable")
            .1
    };
    for name in CREATED {
        let path = path_of(name);
        fs::create_dir_all(path)
            .map_err(|error| format!("error: cannot create {}: {error}", path.display()))?;
    }
    let root_device = device(&root)?;
    for name in CREATED {
        let path = path_of(name);
        let canonical = fs::canonicalize(path)
            .map_err(|error| format!("error: cannot resolve {}: {error}", path.display()))?;
        if !canonical.starts_with(&root) || canonical == root {
            return Err(format!(
                "error: project-data path escapes the repository: {}",
                path.display()
            ));
        }
        if device(&canonical)? != root_device {
            return Err(format!(
                "error: project-data path is off the repository filesystem: {}",
                path.display()
            ));
        }
    }
    Ok(environment)
}

/// The filesystem a path is on.
fn device(path: &Path) -> Result<u64, String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("error: cannot stat {}: {error}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Ok(metadata.dev())
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_repository_activates_and_a_non_repository_is_refused() {
        let root = crate::repository_root();
        let environment = activate(&root).expect("the repository activates");
        let canonical = fs::canonicalize(&root).expect("root resolves");
        let tmp = &environment
            .iter()
            .find(|(name, _)| *name == "TMPDIR")
            .expect("TMPDIR")
            .1;
        assert_eq!(tmp, &canonical.join(".project-data/tmp"));
        assert!(
            environment
                .iter()
                .all(|(_, path)| path.starts_with(&canonical))
        );
        let not_a_repository = canonical.join("xtask/src");
        assert!(
            activate(&not_a_repository)
                .is_err_and(|message| message.contains("is not a SpecForge repository"))
        );
    }
}
