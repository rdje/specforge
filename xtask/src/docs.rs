//! `cargo xtask docs`: build the mdBook and the rustdoc API reference beside it (`RUSTDOC-AND-XTASK.5`), the
//! native port of `scripts/run_docs_ci.sh`, which is now a shim over this command.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::exit_code;
use crate::project_data::{self, Environment};

/// Where the docs build publishes the API reference, relative to the repository root.
pub const API_DIR: &str = "generated/mdbook/specforge/api";

/// The workspace crates whose index the API reference must hold.
pub const CRATES: &[&str] = &[
    "specforge_core",
    "specforge",
    "specforge_conformance",
    "specforge_production_graph",
];

/// Run the docs build from `root`; returns the exit code the script would.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    let _ = arguments;
    let environment = match project_data::activate(root) {
        Ok(environment) => environment,
        Err(message) => {
            eprintln!("{message}");
            return 1;
        }
    };
    let path = env::var_os("PATH").unwrap_or_default();
    if find_in_path("mdbook", &path).is_none() {
        println!("[specforge-docs] mdbook is required but not installed");
        return 1;
    }
    println!("[specforge-docs] testing mdBook examples");
    let code = step(root, &environment, "mdbook", &["test", "docs/book"], None);
    if code != 0 {
        return code;
    }
    println!("[specforge-docs] building mdBook");
    let code = step(root, &environment, "mdbook", &["build", "docs/book"], None);
    if code != 0 {
        return code;
    }
    // The RUSTDOC doctrine's own flag, ahead of any the caller set.
    println!("[specforge-docs] building the rustdoc API reference");
    let flags = match env::var("RUSTDOCFLAGS") {
        Ok(extra) if !extra.is_empty() => format!("-D warnings {extra}"),
        _ => "-D warnings".to_string(),
    };
    let code = step(
        root,
        &environment,
        "cargo",
        &["doc", "--offline", "--locked", "--workspace", "--no-deps"],
        Some(("RUSTDOCFLAGS", flags)),
    );
    if code != 0 {
        return code;
    }
    let target = env::var_os("CARGO_TARGET_DIR").map_or_else(|| root.join("target"), PathBuf::from);
    let api = root.join(API_DIR);
    let copied = remove_if_present(&api).and_then(|()| copy_tree(&target.join("doc"), &api));
    if let Err(error) = copied {
        eprintln!("[specforge-docs] cannot write {API_DIR}: {error}");
        return 1;
    }
    for name in CRATES {
        if !api.join(name).join("index.html").is_file() {
            println!("[specforge-docs] the API reference lacks {name}/index.html");
            return 1;
        }
    }
    println!("[specforge-docs] API reference written to {API_DIR}");
    0
}

/// Run one program from `root` with the project-data environment and an optional extra variable; its output
/// passes through, and its exit code is returned.
fn step(
    root: &Path,
    environment: &Environment,
    program: &str,
    arguments: &[&str],
    extra: Option<(&str, String)>,
) -> i32 {
    let mut command = Command::new(program);
    command.args(arguments).current_dir(root);
    for (name, value) in environment {
        command.env(name, value);
    }
    if let Some((name, value)) = extra {
        command.env(name, value);
    }
    match command.status() {
        Ok(status) => exit_code(status),
        Err(error) => {
            eprintln!("[specforge-docs] cannot run {program}: {error}");
            127
        }
    }
}

/// The first executable file named `name` on a `PATH`-style list, as `command -v` finds it.
pub fn find_in_path(name: &str, path: &std::ffi::OsStr) -> Option<PathBuf> {
    env::split_paths(path)
        .map(|directory| directory.join(name))
        .find(|candidate| is_executable(candidate))
}

fn is_executable(path: &Path) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        metadata.is_file()
    }
}

fn remove_if_present(path: &Path) -> io::Result<()> {
    match fs::remove_dir_all(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

/// Copy the directory `from` to `to`, as `cp -R from to` does when `to` does not exist.
pub fn copy_tree(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let source = entry.path();
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&source, &target)?;
        } else {
            fs::copy(&source, &target)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_program_is_found_only_where_it_is_executable() {
        let scratch = crate::repository_root().join(".project-data/tmp/xtask-path-test");
        let _ = fs::remove_dir_all(&scratch);
        let bin = scratch.join("bin");
        fs::create_dir_all(&bin).expect("scratch");
        let tool = bin.join("tool");
        fs::write(&tool, "#!/bin/sh\n").expect("write");
        let path = env::join_paths([scratch.join("empty"), bin.clone()]).expect("path");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(find_in_path("tool", &path), None, "not executable yet");
            fs::set_permissions(&tool, fs::Permissions::from_mode(0o755)).expect("chmod");
        }
        assert_eq!(find_in_path("tool", &path), Some(tool));
        assert_eq!(find_in_path("absent", &path), None);
        fs::remove_dir_all(&scratch).expect("cleanup");
    }

    #[test]
    fn a_tree_is_copied_whole() {
        let scratch = crate::repository_root().join(".project-data/tmp/xtask-copy-test");
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(scratch.join("from/nested")).expect("scratch");
        fs::write(scratch.join("from/a.html"), "a").expect("write");
        fs::write(scratch.join("from/nested/b.js"), "b").expect("write");
        copy_tree(&scratch.join("from"), &scratch.join("to")).expect("copy");
        assert_eq!(
            fs::read_to_string(scratch.join("to/a.html")).expect("a"),
            "a"
        );
        assert_eq!(
            fs::read_to_string(scratch.join("to/nested/b.js")).expect("b"),
            "b"
        );
        fs::remove_dir_all(&scratch).expect("cleanup");
    }
}
