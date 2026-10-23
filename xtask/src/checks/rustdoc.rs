//! The `RUSTDOC` doctrine (`RUSTDOC-AND-XTASK.2`), native since `.8`: the workspace documents with no rustdoc
//! warning under `-D warnings`, after a self-test that requires a scratch crate with an unresolved intra-doc link
//! to fail, so the check's own negative is re-proven on every run.

use std::fs;
use std::path::Path;
use std::process::Output;

use crate::project_data::{self, Environment};

/// The rustdoc flags: the doctrine's `-D warnings`, then any the caller set.
pub fn flags() -> String {
    match std::env::var("RUSTDOCFLAGS") {
        Ok(extra) if !extra.is_empty() => format!("-D warnings {extra}"),
        _ => "-D warnings".to_string(),
    }
}

/// Run the check from `root`; returns the exit code `scripts/check_rustdoc.sh` gave.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    if !arguments.is_empty() {
        eprintln!("Usage: cargo xtask check rustdoc   (this check takes no arguments)");
        return 2;
    }
    let environment = match project_data::activate(root) {
        Ok(environment) => environment,
        Err(message) => {
            eprintln!("{message}");
            return 1;
        }
    };
    let tmp = project_data::path(&environment, "TMPDIR");
    let work = tmp.join(format!("rustdoc-selftest.{}", std::process::id()));
    let code = check(root, &environment, &work);
    let _ = fs::remove_dir_all(&work);
    code
}

fn check(root: &Path, environment: &Environment, work: &Path) -> i32 {
    let flags = flags();
    if let Err(error) = write_selftest_crate(work) {
        eprintln!("rustdoc: FAIL — cannot write the self-test crate: {error}");
        return 1;
    }
    let manifest = work.join("Cargo.toml");
    let target = work.join("target");
    let selftest = project_data::command("cargo", root, environment)
        .args([
            "doc",
            "--offline",
            "--quiet",
            "--no-deps",
            "--manifest-path",
        ])
        .arg(&manifest)
        .env("RUSTDOCFLAGS", &flags)
        .env("CARGO_TARGET_DIR", &target)
        .output();
    let selftest = match selftest {
        Ok(output) => output,
        Err(error) => {
            eprintln!("rustdoc: FAIL — cannot run cargo doc: {error}");
            return 1;
        }
    };
    if selftest.status.success() {
        eprintln!(
            "rustdoc: FAIL — the self-test crate with an unresolved link documented cleanly, so the gate cannot see one"
        );
        return 1;
    }
    if !combined(&selftest).contains("unresolved link") {
        eprintln!("rustdoc: FAIL — the self-test crate failed for another reason:");
        eprint!("{}", combined(&selftest));
        return 1;
    }
    let workspace = project_data::command("cargo", root, environment)
        .args([
            "doc",
            "--offline",
            "--locked",
            "--quiet",
            "--workspace",
            "--no-deps",
        ])
        .env("RUSTDOCFLAGS", &flags)
        .output();
    match workspace {
        Ok(output) if output.status.success() => {
            println!(
                "rustdoc: PASS — the self-test crate's unresolved link failed as it must, and the workspace documents cleanly"
            );
            0
        }
        Ok(output) => {
            eprint!("{}", combined(&output));
            eprintln!(
                "rustdoc: FAIL — cargo doc --workspace --no-deps reports a rustdoc warning (see above)"
            );
            1
        }
        Err(error) => {
            eprintln!("rustdoc: FAIL — cannot run cargo doc: {error}");
            1
        }
    }
}

/// The scratch crate: one public function whose doc comment links to an item that does not exist.
pub fn write_selftest_crate(work: &Path) -> std::io::Result<()> {
    fs::create_dir_all(work.join("src"))?;
    fs::write(
        work.join("Cargo.toml"),
        "[package]\nname = \"rustdoc_selftest\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )?;
    fs::write(
        work.join("src/lib.rs"),
        "/// Links to [`missing_item`], which does not exist.\npub fn documented() {}\n",
    )
}

/// A process's stdout then stderr, as `> log 2>&1` collects them when the process writes only one.
fn combined(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_selftest_crate_links_to_nothing() {
        let work = crate::repository_root().join(".project-data/tmp/xtask-rustdoc-selftest-test");
        let _ = fs::remove_dir_all(&work);
        write_selftest_crate(&work).expect("write");
        let lib = fs::read_to_string(work.join("src/lib.rs")).expect("lib");
        assert!(lib.contains("[`missing_item`]"));
        let manifest = fs::read_to_string(work.join("Cargo.toml")).expect("manifest");
        assert!(
            manifest.contains("[workspace]"),
            "its own workspace, outside SpecForge's"
        );
        fs::remove_dir_all(&work).expect("cleanup");
    }

    #[test]
    fn the_doctrine_flag_comes_first() {
        assert!(flags().starts_with("-D warnings"));
    }
}
