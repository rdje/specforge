//! The `PROJECT-DATA-LOCALITY` doctrine, native since `RUSTDOC-AND-XTASK.14.c`: every project-owned temporary,
//! cache, dependency and artifact seam resolves from the repository root, on the repository's filesystem. It
//! activates the roots as `scripts/project_data_env.sh` does (`project_data::activate`), requires the contract text
//! each seam carries (`.cargo/config.toml`'s forced variables, every shell entry point's activation, the native
//! commands, the hooks, the product's temporary and subprocess seams), holds the two Python environments to their
//! own directories, refuses disposable residue, runs the persisted-artifact path contract's self-test and check
//! in-process (`persisted_artifact_paths`, native since `RUSTDOC-AND-XTASK.14.g`), and runs
//! `scripts/test_project_data_locality.sh`, the shell activation's own cases, which stays shell with the library it
//! tests, with the activated environment.
//!
//! `scripts/check_project_data_locality.sh`, the registered script, is a shim over it that keeps its argument
//! refusal and the shell activation.

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::project_data::{self, Environment};

const LABEL: &str = "project-data-locality";

/// The variables `.cargo/config.toml` must force to repository-relative values.
const CARGO_VARIABLES: [&str; 10] = [
    "SPECFORGE_REPO_ROOT",
    "TMPDIR",
    "TMP",
    "TEMP",
    "XDG_CACHE_HOME",
    "HF_HOME",
    "HUGGINGFACE_HUB_CACHE",
    "PIP_CACHE_DIR",
    "TORCH_HOME",
    "MPLCONFIGDIR",
];

/// The seams outside `scripts/` and the text each must carry. A script ported to `cargo xtask` keeps its seam: the
/// native command activates the same roots before it spawns anything (`RUSTDOC-AND-XTASK.5`).
const REQUIRED: &[(&str, &str)] = &[
    ("xtask/src/docs.rs", "project_data::activate(root)"),
    (
        "xtask/src/project_data.rs",
        "is off the repository filesystem",
    ),
    (
        ".githooks/pre-commit",
        "source \"$ROOT/scripts/project_data_env.sh\"",
    ),
    (
        ".githooks/pre-commit",
        "specforge_activate_project_data \"$ROOT\"",
    ),
    (
        ".claude/hooks/post-compact.sh",
        "source \"$PROJECT_DIR/scripts/project_data_env.sh\"",
    ),
    (
        ".claude/hooks/post-compact.sh",
        "specforge_activate_project_data \"$PROJECT_DIR\"",
    ),
    (".claude/hooks/post-compact.sh", "SPECFORGE_FSMGEN_MAIN_DIR"),
    ("crates/specforge/src/lib.rs", "project_data::prepare()?;"),
    (
        "crates/specforge/src/project_data.rs",
        ".tempdir_in(roots.temporary)?",
    ),
    (
        "crates/specforge/src/project_data.rs",
        "ensure_same_filesystem",
    ),
];

/// The product's commands that take their temporary directory from the project-data roots.
const TEMPDIR_SEAMS: [&str; 6] = [
    "crates/specforge/src/commands/doctor.rs",
    "crates/specforge/src/commands/enrich.rs",
    "crates/specforge/src/commands/llm_text.rs",
    "crates/specforge/src/commands/eval_extraction.rs",
    "crates/specforge/src/commands/kg_bench.rs",
    "crates/specforge/src/ir/source/docling_backend.rs",
];

/// The product's seams that configure a subprocess with the project-data environment.
const CONFIGURE_SEAMS: [&str; 6] = [
    "crates/specforge/src/commands/doctor.rs",
    "crates/specforge/src/commands/enrich.rs",
    "crates/specforge/src/commands/llm_text.rs",
    "crates/specforge/src/commands/recover_register_bits.rs",
    "crates/specforge/src/ir/mod.rs",
    "crates/specforge/src/ir/source/docling_backend.rs",
];

const LOCKS: [&str; 2] = [
    "requirements/docling-macos-arm64.lock.txt",
    "requirements/eval-macos-arm64.lock.txt",
];
const ENVIRONMENTS: [&str; 2] = [".venv-docling", ".venv-eval"];

/// The gate's verdict so far: each note is printed when found, as the script printed it.
struct Gate<'a> {
    root: &'a Path,
    failed: bool,
}

impl Gate<'_> {
    fn note(&mut self, message: &str) {
        eprintln!("{LABEL}: {message}");
        self.failed = true;
    }

    /// `grep -Fq -- TEXT FILE`: the file must contain the text; an unreadable file fails as well.
    fn require_text(&mut self, file: &str, text: &str) {
        let found =
            fs::read(self.root.join(file)).is_ok_and(|bytes| contains(&bytes, text.as_bytes()));
        if !found {
            self.note(&format!("{file} is missing required contract text: {text}"));
        }
    }
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    needle.is_empty()
        || haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

/// The gate: every check runs and reports, then one verdict. `--root DIR` checks another tree, as a copy of the
/// script inside it would.
pub fn run(repository: &Path, arguments: &[String]) -> i32 {
    let root = match arguments {
        [] => repository,
        [flag, root] if flag == "--root" => Path::new(root),
        _ => {
            eprintln!(
                "Usage: cargo xtask check project-data-locality [--root PROJECT_ROOT]   (the gate takes no other \
                 arguments)"
            );
            return 2;
        }
    };
    let mut gate = Gate {
        root,
        failed: false,
    };
    let environment = match project_data::activate(root) {
        Ok(environment) => Some(environment),
        Err(error) => {
            eprintln!("{error}");
            gate.note("canonical project-data roots failed validation");
            None
        }
    };
    if !root.join(".project-data/tmp/.gitkeep").is_file() {
        gate.note(".project-data/tmp must exist in a fresh checkout");
    }
    for variable in CARGO_VARIABLES {
        gate.require_text(".cargo/config.toml", &format!("{variable} = {{ value ="));
        gate.require_text(".cargo/config.toml", "relative = true, force = true");
    }
    for script in shell_entry_points(root) {
        if script == "scripts/project_data_env.sh" {
            continue;
        }
        gate.require_text(&script, "scripts/project_data_env.sh\"");
        gate.require_text(&script, "specforge_activate_project_data");
    }
    for (file, text) in REQUIRED {
        gate.require_text(file, text);
    }
    for seam in TEMPDIR_SEAMS {
        gate.require_text(seam, "crate::project_data::tempdir()?");
    }
    for seam in CONFIGURE_SEAMS {
        gate.require_text(seam, "crate::project_data::configure_command");
    }
    for lock in LOCKS {
        // `[[ -s FILE ]]`: present and not empty, following a symbolic link.
        if !fs::metadata(root.join(lock)).is_ok_and(|metadata| metadata.len() > 0) {
            gate.note(&format!(
                "missing non-empty Python environment authority: {lock}"
            ));
        }
    }
    for name in ENVIRONMENTS {
        check_environment(&mut gate, name, environment.as_ref());
    }
    let residue = residue(root);
    if !residue.is_empty() {
        gate.note(&format!(
            "disposable FSM/log residue remains in .project-data/tmp: {}",
            residue.join("\n")
        ));
    }
    if fs::read(root.join(".claude/hooks/post-compact.sh"))
        .is_ok_and(|bytes| contains(&bytes, b"/Users/"))
    {
        gate.note(".claude/hooks/post-compact.sh retains an absolute user-home path");
    }
    // The persisted-artifact path contract, native since `RUSTDOC-AND-XTASK.14.g`, in-process: its self-test, then its
    // check, each reporting a failure on standard error as the tool does.
    let (self_test_holds, check_holds) =
        crate::checks::persisted_artifact_paths::locality_verdicts(root);
    if !self_test_holds {
        gate.note("persisted-artifact path gate self-test failed");
    }
    if !check_holds {
        gate.note(
            "canonical artifacts or their producer seams violate the persisted-path contract",
        );
    }
    let shell_cases = root.join("scripts/test_project_data_locality.sh");
    if !succeeds(&shell_cases, &[], root, environment.as_ref()) {
        gate.note("focused project-data locality cases failed");
    }
    if gate.failed {
        eprintln!("{LABEL}: FAIL");
        1
    } else {
        eprintln!("{LABEL}: PASS — repository-derived roots and producer seams are enforced");
        0
    }
}

/// `find scripts -maxdepth 1 -type f -name '*.sh'`, sorted: every shell entry point, hidden ones included, never a
/// symbolic link.
fn shell_entry_points(root: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(root.join("scripts")) else {
        return Vec::new();
    };
    let mut scripts: Vec<String> = entries
        .flatten()
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".sh"))
        .map(|name| format!("scripts/{name}"))
        .collect();
    scripts.sort();
    scripts
}

/// A Python environment, when present, resolves to its own directory and names no other repository copy.
fn check_environment(gate: &mut Gate, name: &str, environment: Option<&Environment>) {
    let directory = gate.root.join(name);
    if !directory.is_dir() {
        return;
    }
    let mut command = Command::new(directory.join("bin/python"));
    command
        .args(["-c", "import sys; print(sys.prefix)"])
        .current_dir(gate.root)
        .stderr(Stdio::null());
    export(&mut command, environment);
    let prefix = match command.output() {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_string(),
        _ => {
            gate.note(&format!("{name}/bin/python is not executable"));
            return;
        }
    };
    let expected = format!("{}/{name}", gate.root.display());
    if prefix != expected {
        gate.note(&format!("{name} resolves outside the repository: {prefix}"));
    }
    if !stale_lines(&directory, gate.root).is_empty() {
        gate.note(&format!(
            "{name} retains a launcher or activation path into another repository copy"
        ));
    }
}

/// The lines of an environment's launchers (every regular file under `bin/`) and its `pyvenv.cfg` that name a
/// `…/specforge/.venv…` path outside this repository. The script searched them with `rg` and dropped every output
/// line containing the root, but each `rg` line begins with its file's absolute path under the root, so it dropped
/// them all and the check never fired; this compares the line's own text.
fn stale_lines(directory: &Path, root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    collect_regular_files(&directory.join("bin"), &mut files);
    files.push(directory.join("pyvenv.cfg"));
    let root = root.to_string_lossy().into_owned();
    let mut stale = Vec::new();
    for file in files {
        let Ok(bytes) = fs::read(&file) else {
            continue;
        };
        for line in bytes.split(|byte| *byte == b'\n') {
            if names_a_repository_environment(line) && !contains(line, root.as_bytes()) {
                stale.push(format!(
                    "{}: {}",
                    file.display(),
                    String::from_utf8_lossy(line)
                ));
            }
        }
    }
    stale
}

fn collect_regular_files(directory: &Path, files: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        match entry.file_type() {
            Ok(kind) if kind.is_dir() => collect_regular_files(&entry.path(), files),
            Ok(kind) if kind.is_file() => files.push(entry.path()),
            _ => {}
        }
    }
}

/// Whether a line matches `/[^ ]*/specforge/\.venv`: an occurrence of `/specforge/.venv` with a `/` somewhere
/// before it and no space between.
fn names_a_repository_environment(line: &[u8]) -> bool {
    let needle = b"/specforge/.venv";
    (0..line.len().saturating_sub(needle.len() - 1)).any(|start| {
        line[start..].starts_with(needle)
            && line[..start]
                .iter()
                .rev()
                .take_while(|byte| **byte != b' ')
                .any(|byte| *byte == b'/')
    })
}

/// `find .project-data/tmp -mindepth 1 -maxdepth 1 -type f ( -name '*.fsm' -o -name '*.log' )`, sorted, as absolute
/// paths.
fn residue(root: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(root.join(".project-data/tmp")) else {
        return Vec::new();
    };
    let mut residue: Vec<String> = entries
        .flatten()
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .filter(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.ends_with(".fsm") || name.ends_with(".log")
        })
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect();
    residue.sort();
    residue
}

fn export(command: &mut Command, environment: Option<&Environment>) {
    for (name, value) in environment.into_iter().flatten() {
        command.env(name, value);
    }
}

/// Run a composed tool with the activated environment, its output discarded and its errors shown, as the script
/// ran it (`TOOL ARGS >/dev/null`).
fn succeeds(
    program: &Path,
    arguments: &[&str],
    root: &Path,
    environment: Option<&Environment>,
) -> bool {
    let mut command = Command::new(program);
    command
        .args(arguments)
        .current_dir(root)
        .stdout(Stdio::null());
    export(&mut command, environment);
    command.status().is_ok_and(|status| status.success())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_launcher_into_another_repository_copy_is_stale_and_one_into_this_repository_is_not() {
        let root = crate::repository_root();
        let directory = root.join(".project-data/tmp/xtask-project-data-locality-venv");
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir_all(directory.join("bin/nested")).expect("the fixture's directories");
        let here = format!("{}/.venv-docling/bin/python", root.display());
        fs::write(
            directory.join("bin/tool"),
            format!("#!{here}\nexec {here} \"$@\"\n"),
        )
        .expect("a current launcher");
        fs::write(directory.join("pyvenv.cfg"), "home = /opt/homebrew/bin\n").expect("a config");
        let current = stale_lines(&directory, &root);
        fs::write(
            directory.join("bin/nested/old"),
            "exec /Users/someone/old/specforge/.venv-docling/bin/python \"$@\"\n",
        )
        .expect("a stale launcher");
        let stale = stale_lines(&directory, &root);
        fs::remove_dir_all(&directory).expect("cleanup");
        assert!(current.is_empty(), "{current:?}");
        assert_eq!(stale.len(), 1, "{stale:?}");
        assert!(names_a_repository_environment(b"x //specforge/.venv"));
        assert!(!names_a_repository_environment(b"/specforge/.venv-docling"));
        assert!(!names_a_repository_environment(b"/a b/specforge/.venv"));
        assert!(names_a_repository_environment(b"/a\tb/specforge/.venv"));
    }

    #[test]
    fn the_seams_are_where_the_contract_says() {
        let root = crate::repository_root();
        let mut gate = Gate {
            root: &root,
            failed: false,
        };
        for (file, text) in REQUIRED {
            gate.require_text(file, text);
        }
        for script in shell_entry_points(&root) {
            if script != "scripts/project_data_env.sh" {
                gate.require_text(&script, "specforge_activate_project_data");
            }
        }
        assert!(!gate.failed);
        assert!(
            shell_entry_points(&root)
                .contains(&"scripts/check_project_data_locality.sh".to_string())
        );
    }

    /// Every file the gate reads, copied into a fixture tree so one input at a time can be broken.
    fn copy_tree(root: &Path, tree: &Path) {
        let mut files: Vec<String> = [
            "Cargo.toml",
            "README.md",
            ".project-data/tmp/.gitkeep",
            ".cargo/config.toml",
            ".githooks/pre-commit",
            ".claude/hooks/post-compact.sh",
            "crates/specforge/src/persisted_path.rs",
            "crates/specforge/src/ir/source.rs",
            "crates/specforge/src/ir/evidence.rs",
            "crates/specforge/src/ir/figure_region.rs",
            "crates/specforge/src/ir/semantic.rs",
            "crates/specforge/src/ir/intent.rs",
            "crates/specforge/src/ir/adapters.rs",
            "crates/specforge/src/ir/prior_memory.rs",
            "crates/specforge/src/commands/learn_priors.rs",
            "crates/specforge/src/commands/project_validation.rs",
        ]
        .iter()
        .map(|file| (*file).to_string())
        .collect();
        files.extend(REQUIRED.iter().map(|(file, _)| (*file).to_string()));
        files.extend(
            TEMPDIR_SEAMS
                .iter()
                .chain(&CONFIGURE_SEAMS)
                .map(|file| (*file).to_string()),
        );
        files.extend(LOCKS.iter().map(|file| (*file).to_string()));
        files.extend(shell_entry_points(root));
        for file in files {
            let target = tree.join(&file);
            fs::create_dir_all(target.parent().expect("a parent"))
                .expect("the fixture's directories");
            fs::copy(root.join(&file), &target).expect("a copied input");
        }
    }

    #[test]
    fn each_broken_input_fails_the_gate_on_a_copied_tree() {
        let root = crate::repository_root();
        let base = root.join(".project-data/tmp/xtask-project-data-locality-gate");
        let _ = fs::remove_dir_all(&base);
        fn write(path: &Path, text: &str) {
            fs::create_dir_all(path.parent().expect("a parent")).expect("directories");
            fs::write(path, text).expect("a fixture file");
        }
        fn executable(path: &Path, text: &str) {
            use std::os::unix::fs::PermissionsExt;
            write(path, text);
            fs::set_permissions(path, fs::Permissions::from_mode(0o755)).expect("executable");
        }
        type Damage = Box<dyn Fn(&Path)>;
        let breaks: Vec<(&str, Damage, i32)> = vec![
            ("clean", Box::new(|_: &Path| {}), 0),
            (
                "a log left in the temporary root",
                Box::new(|tree: &Path| write(&tree.join(".project-data/tmp/run.log"), "x")),
                1,
            ),
            (
                "a hidden entry point without the activation",
                Box::new(|tree: &Path| {
                    executable(&tree.join("scripts/.hidden.sh"), "#!/usr/bin/env bash\n")
                }),
                1,
            ),
            (
                "a launcher into another repository copy",
                Box::new(|tree: &Path| {
                    let prefix = tree.join(".venv-eval");
                    executable(
                        &prefix.join("bin/python"),
                        &format!("#!/bin/sh\nprintf '%s\\n' '{}'\n", prefix.display()),
                    );
                    write(&prefix.join("pyvenv.cfg"), "home = /opt/homebrew/bin\n");
                    executable(
                        &prefix.join("bin/tool"),
                        "#!/Users/someone/specforge/.venv-eval/bin/python\n",
                    );
                }),
                1,
            ),
            (
                "a persisted-path producer seam removed",
                Box::new(|tree: &Path| {
                    let source = tree.join("crates/specforge/src/ir/source.rs");
                    let text = fs::read_to_string(&source).expect("the copied source");
                    write(
                        &source,
                        &text.replace("persisted.verify_canonical_proof()?;", "verify()?;"),
                    );
                }),
                1,
            ),
        ];
        let mut verdicts = Vec::new();
        for (index, (name, damage, expected)) in breaks.iter().enumerate() {
            let tree = base.join(index.to_string());
            copy_tree(&root, &tree);
            damage(&tree);
            let arguments = ["--root".to_string(), tree.to_string_lossy().into_owned()];
            verdicts.push((*name, run(&root, &arguments), *expected));
        }
        fs::remove_dir_all(&base).expect("cleanup");
        for (name, actual, expected) in verdicts {
            assert_eq!(actual, expected, "{name}");
        }
    }
}
