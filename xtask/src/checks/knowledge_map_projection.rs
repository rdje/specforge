//! Repository-local integration tests for the portable Knowledge Map projection-set bundle, native since
//! `RUSTDOC-AND-XTASK.15.f`. The generator and checker remain external portable-kit commands; this module
//! reproduces the eight integration checks and the output of `scripts/test_knowledge_map_projection.sh`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::project_data;

const LABEL: &str = "knowledge-map-projection-tests";
const SUCCESS: &str = "knowledge-map-projection-tests: 8/8 generation, drift, collision, cleanup, staging, bound, name, and residue checks pass.\n";

#[derive(Debug)]
struct Failure {
    message: Option<String>,
    code: i32,
}

impl Failure {
    fn custom(message: &str) -> Self {
        Self {
            message: Some(format!("{LABEL}: {message}\n")),
            code: 1,
        }
    }

    fn raw(message: String) -> Self {
        Self {
            message: Some(message),
            code: 1,
        }
    }

    fn code(code: i32) -> Self {
        Self {
            message: None,
            code,
        }
    }
}

struct ProcessResult {
    code: i32,
    stdout: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stdout {
    Discard,
    Forward,
    Capture,
}

fn execute(
    command: &mut Command,
    stdout: Stdout,
    discard_stderr: bool,
    output: &mut dyn super::Output,
) -> Result<ProcessResult, Failure> {
    let process = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| Failure::custom(&error.to_string()))?;
    let out = String::from_utf8_lossy(&process.stdout);
    if stdout == Stdout::Forward {
        output.out(&out);
    }
    if !discard_stderr {
        output.err(&String::from_utf8_lossy(&process.stderr));
    }
    Ok(ProcessResult {
        code: crate::exit_code(process.status),
        stdout: out.into_owned(),
    })
}

fn succeed(process: ProcessResult) -> Result<String, Failure> {
    if process.code == 0 {
        Ok(process.stdout)
    } else {
        Err(Failure::code(process.code))
    }
}

fn expect_failure(
    command: &mut Command,
    display: &str,
    output: &mut dyn super::Output,
) -> Result<(), Failure> {
    let process = execute(command, Stdout::Discard, true, output)?;
    if process.code == 0 {
        Err(Failure::custom(&format!("expected failure: {display}")))
    } else {
        Ok(())
    }
}

struct Fixture {
    path: PathBuf,
}

impl Fixture {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    fn join(&self, relative: &str) -> PathBuf {
        self.path.join(relative)
    }

    fn write(&self, relative: &str, content: &str) -> Result<(), Failure> {
        fs::write(self.join(relative), content).map_err(|error| Failure::custom(&error.to_string()))
    }

    fn write_config(&self, shard_lines: u32, projection_lines: u32) -> Result<(), Failure> {
        self.write(
            ".knowledge_map.conf",
            &format!(
                ": \"${{KM_TARGET_SHARD_LINES:={shard_lines}}}\"\n: \"${{KM_MAX_PROJECTION_LINES:={projection_lines}}}\"\n"
            ),
        )
    }

    fn write_beta(&self, extra: Option<&str>) -> Result<(), Failure> {
        let mut text = String::from(
            "---\nid: beta\ntitle: Beta fact\nanswers:\n  - \"what is beta\"\n  - \"where is beta\"\n",
        );
        if let Some(extra) = extra {
            text.push_str(&format!("  - \"{extra}\"\n"));
        }
        text.push_str("date: 2026-08-08\nevidence: docs/knowledge/beta.md\n---\n\nBeta.\n");
        self.write("docs/knowledge/beta.md", &text)
    }

    fn generator(
        &self,
        arguments: &[&str],
        output: &mut dyn super::Output,
    ) -> Result<ProcessResult, Failure> {
        let mut command = Command::new("bash");
        command
            .arg("knowledge-map/scripts/gen_knowledge_map.sh")
            .args(arguments)
            .current_dir(&self.path);
        execute(&mut command, Stdout::Discard, false, output)
    }

    fn checker(&self, output: &mut dyn super::Output) -> Result<ProcessResult, Failure> {
        let mut command = Command::new("bash");
        command
            .arg("knowledge-map/scripts/check_knowledge_map.sh")
            .current_dir(&self.path);
        execute(&mut command, Stdout::Discard, false, output)
    }

    fn expect_generator_failure(&self, output: &mut dyn super::Output) -> Result<(), Failure> {
        let command_text = format!(
            "cd '{}' && bash knowledge-map/scripts/gen_knowledge_map.sh",
            self.path.display()
        );
        let mut command = Command::new("bash");
        command.arg("-c").arg(&command_text);
        expect_failure(&mut command, &format!("bash -c {command_text}"), output)
    }

    fn expect_checker_failure(&self, output: &mut dyn super::Output) -> Result<(), Failure> {
        let command_text = format!(
            "cd '{}' && bash knowledge-map/scripts/check_knowledge_map.sh",
            self.path.display()
        );
        let mut command = Command::new("bash");
        command.arg("-c").arg(&command_text);
        expect_failure(&mut command, &format!("bash -c {command_text}"), output)
    }

    fn git(
        &self,
        arguments: &[&str],
        output: &mut dyn super::Output,
    ) -> Result<ProcessResult, Failure> {
        let mut command = Command::new("git");
        command.arg("-C").arg(&self.path).args(arguments);
        execute(&mut command, Stdout::Forward, false, output)
    }

    fn cksum(&self, output: &mut dyn super::Output) -> Result<String, Failure> {
        let mut command = Command::new("cksum");
        command.arg(self.join("KNOWLEDGE_MAP.md"));
        let process = execute(&mut command, Stdout::Capture, false, output)?;
        succeed(process)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn shard_count(fixture: &Path) -> Result<usize, Failure> {
    let directory = fixture.join("docs/knowledge-map");
    let entries = fs::read_dir(directory).map_err(|error| Failure::custom(&error.to_string()))?;
    let mut count = 0;
    for entry in entries {
        let entry = entry.map_err(|error| Failure::custom(&error.to_string()))?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry
            .file_type()
            .map_err(|error| Failure::custom(&error.to_string()))?
            .is_file()
            && name.starts_with("questions-")
            && name.ends_with(".md")
        {
            count += 1;
        }
    }
    Ok(count)
}

fn require_three_shards(fixture: &Path) -> Result<(), Failure> {
    if shard_count(fixture)? == 3 {
        Ok(())
    } else {
        Err(Failure::custom(
            "bounded rollover did not create three shards",
        ))
    }
}

fn require_bounded_landing(fixture: &Path) -> Result<(), Failure> {
    let raw = fs::read(fixture.join("KNOWLEDGE_MAP.md"))
        .map_err(|error| Failure::custom(&error.to_string()))?;
    if raw.iter().filter(|byte| **byte == b'\n').count() <= 96 {
        Ok(())
    } else {
        Err(Failure::custom("landing is not bounded"))
    }
}

fn require_unchanged(before: &str, after: &str, message: &str) -> Result<(), Failure> {
    if before == after {
        Ok(())
    } else {
        Err(Failure::custom(message))
    }
}

fn require_deleted_shards_absent(fixture: &Path) -> Result<(), Failure> {
    if !fixture
        .join("docs/knowledge-map/questions-0002.md")
        .exists()
        && !fixture
            .join("docs/knowledge-map/questions-0003.md")
            .exists()
    {
        Ok(())
    } else {
        Err(Failure::custom(
            "obsolete generated shards were not removed",
        ))
    }
}

fn require_deleted_shard_staged(paths: &str) -> Result<(), Failure> {
    if paths
        .lines()
        .any(|line| line == "docs/knowledge-map/questions-0002.md")
    {
        Ok(())
    } else {
        Err(Failure::custom(
            "tracked shard deletion was omitted from the staging path set",
        ))
    }
}

fn require_no_residue(fixture: &Path) -> Result<(), Failure> {
    let entries = fs::read_dir(fixture.join("docs/knowledge-map"))
        .map_err(|error| Failure::custom(&error.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|error| Failure::custom(&error.to_string()))?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if entry
            .file_type()
            .map_err(|error| Failure::custom(&error.to_string()))?
            .is_file()
            && name.starts_with('.')
            && name.ends_with(".tmp")
        {
            return Err(Failure::custom("generator temporary residue remains"));
        }
    }
    let entries = fs::read_dir(fixture.join("generated"))
        .map_err(|error| Failure::custom(&error.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|error| Failure::custom(&error.to_string()))?;
        if entry
            .file_name()
            .to_string_lossy()
            .starts_with(".knowledge-map-check.")
        {
            return Err(Failure::custom("checker temporary residue remains"));
        }
    }
    Ok(())
}

fn run_integration(root: &Path, output: &mut dyn super::Output) -> Result<(), Failure> {
    let environment =
        project_data::activate(root).map_err(|message| Failure::raw(format!("{message}\n")))?;
    let root = project_data::path(&environment, "SPECFORGE_REPO_ROOT");
    let fixture = Fixture::new(root.join(format!(
        "generated/knowledge-map-projection-tests.{}",
        std::process::id()
    )));
    for directory in ["knowledge-map/scripts", "docs/knowledge", "generated"] {
        fs::create_dir_all(fixture.join(directory))
            .map_err(|error| Failure::custom(&error.to_string()))?;
    }
    for name in [
        "gen_knowledge_map.sh",
        "check_knowledge_map.sh",
        "knowledge_map.conf",
    ] {
        fs::copy(
            root.join(format!("knowledge-map/scripts/{name}")),
            fixture.join(&format!("knowledge-map/scripts/{name}")),
        )
        .map_err(|error| Failure::custom(&error.to_string()))?;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for name in ["gen_knowledge_map.sh", "check_knowledge_map.sh"] {
            let path = fixture.join(&format!("knowledge-map/scripts/{name}"));
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
                .map_err(|error| Failure::custom(&error.to_string()))?;
        }
    }
    succeed(fixture.git(&["init", "-q"], output)?)?;

    fixture.write(
        "docs/knowledge/alpha.md",
        "---\nid: alpha\ntitle: Alpha fact\nanswers:\n  - \"what is alpha\"\n  - \"where is alpha\"\n  - \"how does alpha work\"\ndate: 2026-08-08\nevidence: docs/knowledge/alpha.md\n---\n\nAlpha.\n",
    )?;
    fixture.write_beta(None)?;
    fixture.write_config(8, 128)?;
    succeed(fixture.generator(&[], output)?)?;
    require_three_shards(&fixture.path)?;
    require_bounded_landing(&fixture.path)?;
    succeed(fixture.checker(output)?)?;

    fs::OpenOptions::new()
        .append(true)
        .open(fixture.join("docs/knowledge-map/questions-0001.md"))
        .and_then(|mut file| std::io::Write::write_all(&mut file, b"\nDRIFT\n"))
        .map_err(|error| Failure::custom(&error.to_string()))?;
    fixture.expect_checker_failure(output)?;
    succeed(fixture.generator(&[], output)?)?;

    let root_before = fixture.cksum(output)?;
    fixture.write_beta(Some("what is alpha"))?;
    fixture.expect_generator_failure(output)?;
    let root_after = fixture.cksum(output)?;
    require_unchanged(
        &root_before,
        &root_after,
        "collision failure replaced a committed output",
    )?;
    fixture.write_beta(None)?;

    succeed(fixture.git(&["add", "KNOWLEDGE_MAP.md", "docs/knowledge-map"], output)?)?;
    fixture.write_config(32, 128)?;
    succeed(fixture.generator(&[], output)?)?;
    require_deleted_shards_absent(&fixture.path)?;
    let paths = succeed(fixture.generator(&["--print-output-paths"], output)?)?;
    require_deleted_shard_staged(&paths)?;

    let root_before = fixture.cksum(output)?;
    fixture.write_config(32, 5)?;
    fixture.expect_generator_failure(output)?;
    let root_after = fixture.cksum(output)?;
    require_unchanged(
        &root_before,
        &root_after,
        "aggregate-bound failure replaced a committed output",
    )?;
    fixture.write_config(32, 128)?;

    fixture.write("docs/knowledge-map/questions-invalid.md", "# stale\n")?;
    fixture.expect_checker_failure(output)?;
    fs::remove_file(fixture.join("docs/knowledge-map/questions-invalid.md"))
        .map_err(|error| Failure::custom(&error.to_string()))?;
    succeed(fixture.checker(output)?)?;
    require_no_residue(&fixture.path)?;
    output.out(SUCCESS);
    Ok(())
}

/// Run the integration suite, ignoring arguments as the shell suite did.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing both streams to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, _arguments: &[String], output: &mut dyn super::Output) -> i32 {
    match run_integration(root, output) {
        Ok(()) => 0,
        Err(failure) => {
            if let Some(message) = failure.message {
                output.err(&message);
            }
            failure.code
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::Captured;

    #[test]
    fn the_real_suite_passes_and_leaves_no_fixture() {
        let root = crate::repository_root();
        let fixture = root.join(format!(
            "generated/knowledge-map-projection-tests.{}",
            std::process::id()
        ));
        assert!(!fixture.exists(), "stale fixture before the run");
        let mut output = Captured::default();
        assert_eq!(run_to(&root, &[], &mut output), 0, "{}", output.text);
        assert!(output.text.ends_with(SUCCESS), "{}", output.text);
        assert!(
            output
                .text
                .contains("knowledge-map: wrote KNOWLEDGE_MAP.md + 3 bounded shards")
        );
        assert!(!fixture.exists(), "the run did not clean its own fixture");
    }

    #[test]
    fn every_integration_assertion_fails_closed() {
        let root = crate::repository_root();
        let fixture = Fixture::new(root.join(format!(
            "generated/knowledge-map-projection-tests-unit.{}",
            std::process::id()
        )));
        fs::create_dir_all(fixture.join("docs/knowledge-map")).expect("shard directory");
        fs::create_dir_all(fixture.join("generated")).expect("generated directory");
        for name in ["questions-0001.md", "questions-0002.md"] {
            fixture
                .write(&format!("docs/knowledge-map/{name}"), "# shard\n")
                .expect("shard");
        }
        assert_eq!(
            require_three_shards(&fixture.path)
                .unwrap_err()
                .message
                .as_deref(),
            Some("knowledge-map-projection-tests: bounded rollover did not create three shards\n")
        );
        fixture
            .write("KNOWLEDGE_MAP.md", &"line\n".repeat(97))
            .expect("landing");
        assert_eq!(
            require_bounded_landing(&fixture.path)
                .unwrap_err()
                .message
                .as_deref(),
            Some("knowledge-map-projection-tests: landing is not bounded\n")
        );
        assert!(require_unchanged("same", "same", "changed").is_ok());
        assert_eq!(
            require_unchanged("before", "after", "changed")
                .unwrap_err()
                .message
                .as_deref(),
            Some("knowledge-map-projection-tests: changed\n")
        );
        fixture
            .write("docs/knowledge-map/questions-0002.md", "# stale\n")
            .expect("stale shard");
        assert_eq!(
            require_deleted_shards_absent(&fixture.path)
                .unwrap_err()
                .message
                .as_deref(),
            Some("knowledge-map-projection-tests: obsolete generated shards were not removed\n")
        );
        assert!(require_deleted_shard_staged("docs/knowledge-map/questions-0002.md\n").is_ok());
        assert_eq!(
            require_deleted_shard_staged("docs/knowledge-map/questions-0001.md\n")
                .unwrap_err()
                .message
                .as_deref(),
            Some(
                "knowledge-map-projection-tests: tracked shard deletion was omitted from the staging path set\n"
            )
        );
        fixture
            .write("docs/knowledge-map/.rows.tmp", "temporary")
            .expect("generator residue");
        assert_eq!(
            require_no_residue(&fixture.path)
                .unwrap_err()
                .message
                .as_deref(),
            Some("knowledge-map-projection-tests: generator temporary residue remains\n")
        );
        fs::remove_file(fixture.join("docs/knowledge-map/.rows.tmp"))
            .expect("remove generator residue");
        fixture
            .write("generated/.knowledge-map-check.1", "temporary")
            .expect("checker residue");
        assert_eq!(
            require_no_residue(&fixture.path)
                .unwrap_err()
                .message
                .as_deref(),
            Some("knowledge-map-projection-tests: checker temporary residue remains\n")
        );
        fs::remove_file(fixture.join("generated/.knowledge-map-check.1"))
            .expect("remove checker residue");
        assert!(require_no_residue(&fixture.path).is_ok());

        let mut output = Captured::default();
        expect_failure(
            Command::new("bash").args(["-c", "exit 7"]),
            "bash -c exit 7",
            &mut output,
        )
        .expect("a nonzero child is the expected result");
        let error = expect_failure(
            Command::new("bash").args(["-c", "exit 0"]),
            "bash -c exit 0",
            &mut output,
        )
        .unwrap_err();
        assert_eq!(
            error.message.as_deref(),
            Some("knowledge-map-projection-tests: expected failure: bash -c exit 0\n")
        );
    }
}
