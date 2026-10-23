//! The `RUSTDOC` doctrine (`RUSTDOC-AND-XTASK.2`), native since `.8`, with its coverage ratchet since `.7`: the
//! workspace documents with no rustdoc warning other than `missing_docs`, and each package has exactly as many
//! undocumented public items as its ceiling in [`CEILINGS`], which only goes down. A self-test first requires a
//! scratch crate's unresolved intra-doc link to be reported and its one undocumented item to be counted, so both of
//! the check's negatives are re-proven on every run.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Output;

use crate::project_data::{self, Environment};

/// The coverage ratchet: a header, then one `package<TAB>ceiling` row per workspace package; a package it does not
/// list has a ceiling of 0.
pub const CEILINGS: &str = "doctrine/rustdoc/coverage_ceilings.tsv";
const CEILINGS_HEADER: &str = "package\tceiling";
/// The scratch crate's package name, under which its undocumented item is counted.
const SELFTEST_PACKAGE: &str = "rustdoc_selftest";

/// The rustdoc flags: `missing_docs` on as a warning, so it is counted rather than refused, then any the caller
/// set. Every other warning is refused by [`Diagnostics::other`], as `-D warnings` refused it before `.7`.
pub fn flags() -> String {
    match std::env::var("RUSTDOCFLAGS") {
        Ok(extra) if !extra.is_empty() => format!("-W missing_docs {extra}"),
        _ => "-W missing_docs".to_string(),
    }
}

/// What a `cargo doc --message-format=json` run reported.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Diagnostics {
    /// The `missing_docs` warnings, counted per package.
    pub missing_docs: BTreeMap<String, usize>,
    /// Every other diagnostic, as rustdoc rendered it.
    pub other: Vec<String>,
}

/// Read cargo's JSON messages: each compiler message is either a `missing_docs` warning, counted against its
/// package, or another diagnostic, kept.
pub fn read_diagnostics(stdout: &str) -> Diagnostics {
    let mut diagnostics = Diagnostics::default();
    for line in stdout.lines() {
        if !line.contains("\"reason\":\"compiler-message\"") {
            continue;
        }
        if line.contains("\"code\":{\"code\":\"missing_docs\"") {
            let package = json_string(line, "package_id")
                .map(|id| package_name(&id))
                .unwrap_or_default();
            *diagnostics.missing_docs.entry(package).or_insert(0) += 1;
        } else {
            diagnostics
                .other
                .push(json_string(line, "rendered").unwrap_or_else(|| line.to_string()));
        }
    }
    diagnostics
}

/// A package's name from cargo's package id: `path+file:///…/name#version`, or `…/directory#name@version` when
/// the two differ.
pub fn package_name(package_id: &str) -> String {
    let (location, version) = package_id.rsplit_once('#').unwrap_or((package_id, ""));
    match version.split_once('@') {
        Some((name, _)) => name.to_string(),
        None => location.rsplit('/').next().unwrap_or(location).to_string(),
    }
}

/// The decoded value of the first `"key":"…"` string on a line of compact JSON.
fn json_string(line: &str, key: &str) -> Option<String> {
    let start = line.find(&format!("\"{key}\":\""))? + key.len() + 4;
    let mut value = String::new();
    let mut characters = line[start..].chars();
    while let Some(character) = characters.next() {
        match character {
            '"' => return Some(value),
            '\\' => match characters.next()? {
                'n' => value.push('\n'),
                't' => value.push('\t'),
                'r' => value.push('\r'),
                'b' => value.push('\u{8}'),
                'f' => value.push('\u{c}'),
                'u' => {
                    let mut code = hex4(&mut characters)?;
                    if (0xd800..0xdc00).contains(&code) {
                        if characters.next()? != '\\' || characters.next()? != 'u' {
                            return None;
                        }
                        code =
                            0x10000 + ((code - 0xd800) << 10) + (hex4(&mut characters)? - 0xdc00);
                    }
                    value.push(char::from_u32(code)?);
                }
                escaped => value.push(escaped),
            },
            _ => value.push(character),
        }
    }
    None
}

fn hex4(characters: &mut std::str::Chars) -> Option<u32> {
    let digits: String = characters.take(4).collect();
    u32::from_str_radix(&digits, 16).ok()
}

/// The ceilings `text` records, or why it cannot be read.
pub fn read_ceilings(text: &str) -> Result<BTreeMap<String, usize>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(CEILINGS_HEADER) {
        return Err(format!(
            "{CEILINGS} does not start with its header 'package<TAB>ceiling'"
        ));
    }
    let mut ceilings = BTreeMap::new();
    for (index, line) in lines.enumerate() {
        let row = index + 2;
        let Some((package, ceiling)) = line.split_once('\t') else {
            return Err(format!("{CEILINGS} row {row} is not 'package<TAB>ceiling'"));
        };
        let Ok(ceiling) = ceiling.parse::<usize>() else {
            return Err(format!("{CEILINGS} row {row} has no whole-number ceiling"));
        };
        if package.is_empty() || ceilings.insert(package.to_string(), ceiling).is_some() {
            return Err(format!("{CEILINGS} row {row} repeats or omits its package"));
        }
    }
    Ok(ceilings)
}

/// Each package whose count differs from its ceiling: above it, an item went undocumented; below it, the ceiling
/// must come down to the count, so the slack cannot be spent later.
pub fn ratchet_problems(
    counts: &BTreeMap<String, usize>,
    ceilings: &BTreeMap<String, usize>,
) -> Vec<String> {
    let mut packages: Vec<&String> = counts.keys().chain(ceilings.keys()).collect();
    packages.sort();
    packages.dedup();
    let mut problems = Vec::new();
    for package in packages {
        let count = counts.get(package).copied().unwrap_or(0);
        let ceiling = ceilings.get(package).copied().unwrap_or(0);
        if count > ceiling {
            problems.push(format!(
                "{package} has {count} undocumented public items, above its ceiling of {ceiling} in {CEILINGS}: \
                 document the new ones (the ceiling only goes down)"
            ));
        } else if count < ceiling {
            problems.push(format!(
                "{package} has {count} undocumented public items, below its ceiling of {ceiling}: lower it to \
                 {count} in {CEILINGS}"
            ));
        }
    }
    problems
}

/// Run the check from `root`; returns 0 when the workspace documents cleanly within its ceilings.
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
            "--message-format=json",
            "--manifest-path",
        ])
        .arg(&manifest)
        .env("RUSTDOCFLAGS", &flags)
        .env("CARGO_TARGET_DIR", &target)
        .output();
    let selftest = match selftest {
        Ok(output) if output.status.success() => read_diagnostics(&stdout(&output)),
        Ok(output) => {
            eprintln!("rustdoc: FAIL — the self-test crate did not document:");
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
            return 1;
        }
        Err(error) => {
            eprintln!("rustdoc: FAIL — cannot run cargo doc: {error}");
            return 1;
        }
    };
    if !selftest
        .other
        .iter()
        .any(|diagnostic| diagnostic.contains("unresolved link"))
    {
        eprintln!(
            "rustdoc: FAIL — the self-test crate's unresolved link was not reported, so the gate cannot see one"
        );
        return 1;
    }
    let counted = selftest.missing_docs.get(SELFTEST_PACKAGE).copied();
    if counted != Some(1) {
        eprintln!(
            "rustdoc: FAIL — the self-test crate's one undocumented item was counted {counted:?} times, so the gate cannot count coverage"
        );
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
            "--message-format=json",
        ])
        .env("RUSTDOCFLAGS", &flags)
        .output();
    let output = match workspace {
        Ok(output) => output,
        Err(error) => {
            eprintln!("rustdoc: FAIL — cannot run cargo doc: {error}");
            return 1;
        }
    };
    let diagnostics = read_diagnostics(&stdout(&output));
    for diagnostic in &diagnostics.other {
        eprint!("{diagnostic}");
    }
    if !output.status.success() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        eprintln!("rustdoc: FAIL — cargo doc --workspace --no-deps did not complete (see above)");
        return 1;
    }
    if !diagnostics.other.is_empty() {
        eprintln!(
            "rustdoc: FAIL — cargo doc --workspace --no-deps reports a rustdoc warning (see above)"
        );
        return 1;
    }
    let ceilings = match fs::read_to_string(root.join(CEILINGS))
        .map_err(|error| format!("cannot read {CEILINGS}: {error}"))
        .and_then(|text| read_ceilings(&text))
    {
        Ok(ceilings) => ceilings,
        Err(message) => {
            eprintln!("rustdoc: FAIL — {message}");
            return 1;
        }
    };
    let problems = ratchet_problems(&diagnostics.missing_docs, &ceilings);
    if !problems.is_empty() {
        for problem in problems {
            eprintln!("rustdoc: FAIL — {problem}");
        }
        return 1;
    }
    let counts: Vec<String> = ceilings
        .iter()
        .map(|(package, ceiling)| format!("{package} {ceiling}"))
        .collect();
    println!(
        "rustdoc: PASS — the self-test crate's unresolved link was reported and its undocumented item counted; the workspace documents with no other warning, and its undocumented public items match their ceilings ({})",
        counts.join(", ")
    );
    0
}

/// The scratch crate: a documented function whose doc comment links to an item that does not exist, and one
/// undocumented function.
pub fn write_selftest_crate(work: &Path) -> std::io::Result<()> {
    fs::create_dir_all(work.join("src"))?;
    fs::write(
        work.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{SELFTEST_PACKAGE}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[workspace]\n"
        ),
    )?;
    fs::write(
        work.join("src/lib.rs"),
        "//! The rustdoc gate's self-test crate.\n\n/// Links to [`missing_item`], which does not exist.\npub fn documented() {}\n\npub fn undocumented() {}\n",
    )
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_selftest_crate_links_to_nothing_and_leaves_one_item_undocumented() {
        let work = crate::repository_root().join(".project-data/tmp/xtask-rustdoc-selftest-test");
        let _ = fs::remove_dir_all(&work);
        write_selftest_crate(&work).expect("write");
        let lib = fs::read_to_string(work.join("src/lib.rs")).expect("lib");
        assert!(lib.contains("[`missing_item`]"));
        assert!(lib.contains("\n\npub fn undocumented() {}\n"));
        let manifest = fs::read_to_string(work.join("Cargo.toml")).expect("manifest");
        assert!(
            manifest.contains("[workspace]"),
            "its own workspace, outside SpecForge's"
        );
        fs::remove_dir_all(&work).expect("cleanup");
    }

    #[test]
    fn the_coverage_lint_comes_first() {
        assert!(flags().starts_with("-W missing_docs"));
    }

    #[test]
    fn missing_docs_are_counted_per_package_and_every_other_diagnostic_kept() {
        let missing = r#"{"reason":"compiler-message","package_id":"path+file:///w/crates/specforge-core#0.1.0","message":{"rendered":"warning: missing documentation for a function\n","children":[],"code":{"code":"missing_docs","explanation":null},"level":"warning"}}"#;
        let renamed = r#"{"reason":"compiler-message","package_id":"path+file:///w/tools/graph#specforge-production-graph@0.1.0","message":{"rendered":"warning: missing documentation for a struct\n","code":{"code":"missing_docs","explanation":null},"level":"warning"}}"#;
        let link = r#"{"reason":"compiler-message","package_id":"path+file:///w/xtask#0.0.0","message":{"rendered":"warning: unresolved link to `x`\n  --> \"a.rs\"\n","code":{"code":"rustdoc::broken_intra_doc_links","explanation":null},"level":"warning"}}"#;
        let artifact =
            r#"{"reason":"compiler-artifact","package_id":"path+file:///w/xtask#0.0.0"}"#;
        let stdout = [missing, missing, renamed, link, artifact].join("\n");
        let diagnostics = read_diagnostics(&stdout);
        assert_eq!(
            diagnostics.missing_docs,
            BTreeMap::from([
                ("specforge-core".to_string(), 2),
                ("specforge-production-graph".to_string(), 1),
            ])
        );
        assert_eq!(
            diagnostics.other,
            ["warning: unresolved link to `x`\n  --> \"a.rs\"\n"]
        );
    }

    #[test]
    fn a_json_string_decodes_its_escapes() {
        let line = r#"{"rendered":"tab\there é 😀 \\ \/ end"}"#;
        assert_eq!(
            json_string(line, "rendered").as_deref(),
            Some("tab\there é 😀 \\ / end")
        );
        assert_eq!(json_string(line, "absent"), None);
    }

    #[test]
    fn the_ratchet_refuses_a_count_above_or_below_its_ceiling() {
        let ceilings =
            read_ceilings("package\tceiling\nspecforge\t39\nxtask\t0\n").expect("ceilings");
        let counts = |pairs: &[(&str, usize)]| -> BTreeMap<String, usize> {
            pairs
                .iter()
                .map(|(package, count)| (package.to_string(), *count))
                .collect()
        };
        assert!(ratchet_problems(&counts(&[("specforge", 39)]), &ceilings).is_empty());
        let above = ratchet_problems(&counts(&[("specforge", 40)]), &ceilings);
        assert!(above.len() == 1 && above[0].contains("above its ceiling of 39"));
        let below = ratchet_problems(&counts(&[("specforge", 38)]), &ceilings);
        assert!(below.len() == 1 && below[0].contains("lower it to 38"));
        let unlisted = ratchet_problems(&counts(&[("specforge", 39), ("new-crate", 1)]), &ceilings);
        assert!(
            unlisted.len() == 1
                && unlisted[0].starts_with("new-crate has 1")
                && unlisted[0].contains("ceiling of 0")
        );
    }

    #[test]
    fn a_malformed_ceiling_file_is_refused() {
        assert!(read_ceilings("crate\tceiling\n").is_err());
        assert!(read_ceilings("package\tceiling\nspecforge 39\n").is_err());
        assert!(read_ceilings("package\tceiling\nspecforge\tmany\n").is_err());
        assert!(read_ceilings("package\tceiling\nspecforge\t39\nspecforge\t40\n").is_err());
    }

    #[test]
    fn the_recorded_ceilings_read_cleanly() {
        let text = fs::read_to_string(crate::repository_root().join(CEILINGS)).expect("ceilings");
        let ceilings = read_ceilings(&text).expect("well-formed");
        assert!(ceilings.contains_key("specforge-core") && ceilings.contains_key("xtask"));
    }
}
