//! `cargo xtask` — the repository's tooling entry point (`RUSTDOC-AND-XTASK.4`).
//!
//! A command either runs the script that implements it, passing its arguments through and returning its
//! exit code, or is native Rust. Commands move to native Rust one at a time (`RUSTDOC-AND-XTASK.5`, `.6`);
//! once a port reproduces its script, the script becomes a shim over `cargo xtask`, so every command a
//! record cites keeps working.

pub mod checks;
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
        /// The repository-relative script the port replaced, now its shim.
        replaces: &'static str,
        /// The native entry point: the repository root and the arguments in, the exit code out.
        run: fn(&Path, &[String]) -> i32,
    },
    /// The ported doctrine checks, each replacing the script its doctrine registers (`checks::CHECKS`).
    Checks,
}

/// One `cargo xtask` command: its name, how it runs, and what it does.
#[derive(Debug, Clone, Copy)]
pub struct Task {
    /// The command's name, as `cargo xtask <name>` takes it.
    pub name: &'static str,
    /// How the command runs.
    pub runner: Runner,
    /// The one line `cargo xtask help` prints for it.
    pub summary: &'static str,
}

impl Task {
    /// The script that implements the command, or that its native port replaced; `None` for `check`, whose
    /// checks each name their own.
    pub fn script(&self) -> Option<&'static str> {
        match self.runner {
            Runner::Script(script)
            | Runner::Native {
                replaces: script, ..
            } => Some(script),
            Runner::Checks => None,
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
    Task {
        name: "check",
        runner: Runner::Checks,
        summary: "run one ported doctrine check (cargo xtask check <doctrine>)",
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
        Runner::Checks => Ok(checks::run(&root, arguments)),
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
        let scripts = TASKS
            .iter()
            .filter_map(|task| task.script().map(|script| (task.name, script)))
            .chain(
                checks::CHECKS
                    .iter()
                    .map(|check| (check.name, check.replaces)),
            );
        for (name, script) in scripts {
            assert!(
                root.join(script).is_file(),
                "{name} names {script}, which does not exist"
            );
        }
    }

    #[test]
    fn a_ported_scripts_shim_runs_its_native_command() {
        let root = repository_root();
        let shims = TASKS
            .iter()
            .filter_map(|task| match task.runner {
                Runner::Native { replaces, .. } => Some((replaces, task.name.to_string())),
                _ => None,
            })
            .chain(
                checks::CHECKS
                    .iter()
                    .map(|check| (check.replaces, format!("check {}", check.name))),
            );
        for (replaces, command) in shims {
            // A shell shim names the command in its `exec` line, a Perl one in the `exec` list it builds.
            let shim = std::fs::read_to_string(root.join(replaces)).expect("the shim exists");
            assert!(
                shim.contains(&format!("exec cargo xtask {command}"))
                    || shim.contains(&format!("qw(cargo xtask {command})")),
                "{replaces} is not a shim over cargo xtask {command}"
            );
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
