//! `cargo xtask` — the repository's tooling entry point (`RUSTDOC-AND-XTASK.4`).
//!
//! A command either runs the script that implements it, passing its arguments through and returning its
//! exit code, or is native Rust. Commands move to native Rust one at a time (`RUSTDOC-AND-XTASK.5`, `.6`);
//! once a port reproduces its script, the script becomes a shim over `cargo xtask`, so every command a
//! record cites keeps working.

pub mod docs;
pub mod project_data;

use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

/// How a command runs.
#[derive(Debug, Clone, Copy)]
pub enum Runner {
    /// The script at this repository-relative path.
    Script(&'static str),
    /// Native Rust, given the repository root and the arguments; its return value is the exit code. The script
    /// it replaced is now a shim over it.
    Native {
        replaces: &'static str,
        run: fn(&Path, &[String]) -> i32,
    },
}

/// One `cargo xtask` command: its name, how it runs, and what it does.
#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub name: &'static str,
    pub runner: Runner,
    pub summary: &'static str,
}

impl Task {
    /// The script that implements the command, or that its native port replaced.
    pub fn script(&self) -> &'static str {
        match self.runner {
            Runner::Script(script)
            | Runner::Native {
                replaces: script, ..
            } => script,
        }
    }
}

/// Every command, in the order `cargo xtask help` lists them.
pub const TASKS: &[Task] = &[
    Task {
        name: "doctrines",
        runner: Runner::Script("scripts/check_doctrines.sh"),
        summary: "run the doctrine gates (pass --fast, --only ID, --all, --list)",
    },
    Task {
        name: "ci",
        runner: Runner::Script("scripts/run_ci.sh"),
        summary: "run the full local CI",
    },
    Task {
        name: "docs",
        runner: Runner::Native {
            replaces: "scripts/run_docs_ci.sh",
            run: docs::run,
        },
        summary: "build the mdBook and the rustdoc API reference beside it",
    },
];

/// The command named `name`, if there is one.
pub fn find(name: &str) -> Option<&'static Task> {
    TASKS.iter().find(|task| task.name == name)
}

/// The workspace root: the directory that holds this crate.
pub fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the xtask crate sits in the workspace root")
        .to_path_buf()
}

/// The help text: every command with its summary.
pub fn usage() -> String {
    let width = TASKS.iter().map(|task| task.name.len()).max().unwrap_or(0);
    let mut text = String::from("Usage: cargo xtask <command> [arguments...]\n\nCommands:\n");
    for task in TASKS {
        text.push_str(&format!("  {:width$}  {}\n", task.name, task.summary));
    }
    text.push_str(&format!("  {:width$}  {}\n", "help", "print this help"));
    text
}

/// Run `task` from the repository root with `arguments`, and return its exit code. A script is named by its
/// repository-relative path, as the documented `bash scripts/...` form names it, so whatever it prints about
/// itself is byte-identical.
pub fn run(task: &Task, arguments: &[String]) -> std::io::Result<i32> {
    let root = repository_root();
    match task.runner {
        Runner::Script(script) => {
            let status = Command::new("bash")
                .arg(script)
                .args(arguments)
                .current_dir(&root)
                .status()?;
            Ok(exit_code(status))
        }
        Runner::Native { run, .. } => Ok(run(&root, arguments)),
    }
}

/// A process's exit code as a shell reports it: its own code, or 128 plus the signal that ended it.
pub fn exit_code(status: ExitStatus) -> i32 {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return 128 + signal;
        }
    }
    status.code().unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_command_names_an_existing_script() {
        let root = repository_root();
        for task in TASKS {
            assert!(
                root.join(task.script()).is_file(),
                "{} names {}, which does not exist",
                task.name,
                task.script()
            );
        }
    }

    #[test]
    fn a_ported_scripts_shim_runs_its_native_command() {
        let root = repository_root();
        for task in TASKS {
            if let Runner::Native { replaces, .. } = task.runner {
                let shim = std::fs::read_to_string(root.join(replaces)).expect("the shim exists");
                assert!(
                    shim.contains(&format!("exec cargo xtask {}", task.name)),
                    "{replaces} is not a shim over cargo xtask {}",
                    task.name
                );
            }
        }
    }

    #[test]
    fn commands_are_unique_and_listed_in_the_help() {
        let help = usage();
        for (index, task) in TASKS.iter().enumerate() {
            assert!(
                TASKS[index + 1..]
                    .iter()
                    .all(|other| other.name != task.name),
                "{} is declared twice",
                task.name
            );
            assert!(help.contains(task.name) && help.contains(task.summary));
            assert_eq!(find(task.name).map(|found| found.name), Some(task.name));
        }
        assert!(find("no-such-command").is_none());
    }

    #[test]
    fn a_script_exit_code_passes_through() {
        let status = Command::new("bash")
            .args(["-c", "exit 3"])
            .status()
            .expect("bash runs");
        assert_eq!(exit_code(status), 3);
        let killed = Command::new("bash")
            .args(["-c", "kill -TERM $$"])
            .status()
            .expect("bash runs");
        assert_eq!(exit_code(killed), 128 + 15);
    }
}
