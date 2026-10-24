//! `PRODUCTION-GENERICITY`'s `DEPENDENCIES` component, native since `RUSTDOC-AND-XTASK.15.a`: the product package
//! direction holds (core never depends on conformance or on the application; conformance depends on core; the
//! application on both), the enforcement graph tool and the product never depend on each other, the xtask tooling
//! crate uses only the crates the workspace already locks and nothing depends on it, and the core root and IR
//! registry name no conformance or oracle module. The workspace's members are exactly the five declared.
//!
//! `scripts/check_production_genericity_dependencies.pl` is a shim that keeps its `Getopt::Long` front end and the
//! root resolution, then passes `--root DIR` and `--self-test`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const LABEL: &str = "production-genericity-dependencies";
const MEMBERS: [&str; 5] = [
    "crates/specforge",
    "crates/specforge-core",
    "crates/specforge-conformance",
    "tools/production-genericity-graph",
    "xtask",
];
const PRODUCT: [&str; 3] = ["specforge", "specforge-core", "specforge-conformance"];
/// The crates the tooling crate may use, already locked for the product (`RUSTDOC-AND-XTASK.4`, `.13.b`).
const XTASK_ALLOWED: [&str; 2] = ["serde_json", "sha2"];
const ORACLE_SOURCES: [&str; 6] = [
    "eval.rs",
    "ir/completeness.rs",
    "ir/source_to_intent_eval.rs",
    "ir/source_to_intent_replay.rs",
    "ir/trajectory.rs",
    "test_support/trajectory_snapshot.rs",
];
const ORACLE_MODULES: [&str; 4] = [
    "completeness",
    "source_to_intent_eval",
    "source_to_intent_replay",
    "trajectory",
];

/// What Perl's `die` printed and the status it exited with: the failed call's `errno`, or 255.
struct Death {
    message: String,
    code: i32,
}

fn slurp(path: &Path) -> Result<String, Death> {
    fs::read(path)
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .map_err(|error| {
            let text = error.to_string();
            Death {
                message: format!(
                    "cannot read {}: {}",
                    path.display(),
                    text.split(" (os error").next().unwrap_or(&text)
                ),
                code: error.raw_os_error().unwrap_or(255),
            }
        })
}

/// `--root DIR` (resolved by the shim) and `--self-test`.
pub fn run(repository: &Path, arguments: &[String]) -> i32 {
    let mut root = repository.to_path_buf();
    let mut self_test = false;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => match arguments.next() {
                Some(directory) => root = PathBuf::from(directory),
                None => return usage(),
            },
            "--self-test" => self_test = true,
            _ => return usage(),
        }
    }
    let outcome = if self_test {
        run_self_tests(&root).map(|()| {
            println!("{LABEL} self-test: 11/11 pass");
            0
        })
    } else {
        inspect_boundary(&root).map(|problems| {
            if problems.is_empty() {
                println!(
                    "{LABEL}: core -> conformance absent; conformance -> core and application -> {{core, conformance}} \
                     present; enforcement graph tool disconnected from all product packages; xtask uses only \
                     already-locked crates and nothing depends on it"
                );
                return 0;
            }
            for problem in &problems {
                eprintln!("{LABEL}: {problem}");
            }
            1
        })
    };
    match outcome {
        Ok(code) => code,
        Err(death) => {
            eprintln!("{}", death.message);
            death.code
        }
    }
}

fn usage() -> i32 {
    eprintln!(
        "Usage: cargo xtask check production-genericity-dependencies [--root DIR] [--self-test]"
    );
    2
}

fn is_space(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n' | '\r' | '\u{b}' | '\u{c}')
}

/// The dependency names of a manifest's `[dependencies]` table, as
/// `/^\[dependencies\]\s*\n(.*?)(?=^\[|\z)/ms` and then `/^\s*([A-Za-z0-9_-]+)\s*=/` per line read them.
fn dependency_names(text: &str) -> Vec<String> {
    let header = "[dependencies]";
    let Some(start) = line_starts(text).find(|start| text[*start..].starts_with(header)) else {
        return Vec::new();
    };
    // `\s*\n`: the body starts after the last newline in the white space that follows the header.
    let after = start + header.len();
    let run_end = after
        + text[after..]
            .find(|character: char| !is_space(character))
            .unwrap_or(text.len() - after);
    let Some(newline) = text[after..run_end].rfind('\n') else {
        return Vec::new();
    };
    let body_start = after + newline + 1;
    let body_end = line_starts(text)
        .filter(|start| *start >= body_start)
        .find(|start| text[*start..].starts_with('['))
        .unwrap_or(text.len());
    text[body_start..body_end]
        .split('\n')
        .filter_map(|line| {
            let rest = line.trim_start_matches(is_space);
            let length = rest
                .find(|character: char| {
                    !(character.is_ascii_alphanumeric() || character == '_' || character == '-')
                })
                .unwrap_or(rest.len());
            let after_name = rest[length..].trim_start_matches(is_space);
            (length > 0 && after_name.starts_with('=')).then(|| rest[..length].to_string())
        })
        .collect()
}

/// Every offset at which a line begins (`^` under `/m`).
fn line_starts(text: &str) -> impl Iterator<Item = usize> + '_ {
    std::iter::once(0).chain(text.match_indices('\n').map(|(at, _)| at + 1))
}

/// `/^members\s*=\s*\[(.*?)\]/ms`, then every `"([^"]+)"` inside.
fn workspace_members(text: &str) -> Vec<String> {
    for start in line_starts(text) {
        let Some(rest) = text[start..].strip_prefix("members") else {
            continue;
        };
        let rest = rest.trim_start_matches(is_space);
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let rest = rest.trim_start_matches(is_space);
        let Some(rest) = rest.strip_prefix('[') else {
            continue;
        };
        let Some(close) = rest.find(']') else {
            continue;
        };
        return quoted_strings(&rest[..close]);
    }
    Vec::new()
}

/// Every match of `/"([^"]+)"/g`: an empty `""` is no match, and its second quote may open the next one.
fn quoted_strings(body: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut at = 0;
    while let Some(open) = body[at..].find('"').map(|offset| at + offset) {
        let Some(close) = body[open + 1..].find('"').map(|offset| open + 1 + offset) else {
            break;
        };
        if close > open + 1 {
            found.push(body[open + 1..close].to_string());
            at = close + 1;
        } else {
            at = open + 1;
        }
    }
    found
}

fn exact_set_problems(label: &str, actual: &[String], expected: &[&str]) -> Vec<String> {
    let actual_set: BTreeSet<&str> = actual.iter().map(String::as_str).collect();
    let expected_set: BTreeSet<&str> = expected.iter().copied().collect();
    let mut problems = Vec::new();
    for missing in expected_set.difference(&actual_set) {
        problems.push(format!("{label} is missing '{missing}'"));
    }
    for unexpected in actual_set.difference(&expected_set) {
        problems.push(format!("{label} has unexpected member '{unexpected}'"));
    }
    if actual.len() != actual_set.len() {
        problems.push(format!("{label} contains duplicate entries"));
    }
    problems
}

/// `/^\s*pub\s+mod\s+(?:completeness|…)\s*;/m`.
fn declares_oracle_module(text: &str) -> bool {
    line_starts(text).any(|start| {
        let rest = text[start..].trim_start_matches(is_space);
        let Some(rest) = rest.strip_prefix("pub") else {
            return false;
        };
        let spaced = rest.trim_start_matches(is_space);
        if spaced.len() == rest.len() {
            return false;
        }
        let Some(rest) = spaced.strip_prefix("mod") else {
            return false;
        };
        let spaced = rest.trim_start_matches(is_space);
        if spaced.len() == rest.len() {
            return false;
        }
        ORACLE_MODULES.iter().any(|module| {
            spaced
                .strip_prefix(module)
                .is_some_and(|tail| tail.trim_start_matches(is_space).starts_with(';'))
        })
    })
}

fn names_conformance(text: &str) -> bool {
    text.contains("specforge-conformance") || text.contains("specforge_conformance")
}

fn inspect_boundary(root: &Path) -> Result<Vec<String>, Death> {
    let mut problems = Vec::new();
    let members = workspace_members(&slurp(&root.join("Cargo.toml"))?);
    problems.extend(exact_set_problems("workspace", &members, &MEMBERS));
    let mut texts = std::collections::BTreeMap::new();
    let mut dependencies = std::collections::BTreeMap::new();
    for package in PRODUCT {
        let text = slurp(&root.join("crates").join(package).join("Cargo.toml"))?;
        dependencies.insert(
            package,
            dependency_names(&text)
                .into_iter()
                .collect::<BTreeSet<String>>(),
        );
        texts.insert(package, text);
    }
    let graph = slurp(&root.join("tools/production-genericity-graph/Cargo.toml"))?;
    dependencies.insert(
        "specforge-production-graph",
        dependency_names(&graph).into_iter().collect(),
    );
    let depends = |package: &str, on: &str| {
        dependencies
            .get(package)
            .is_some_and(|names| names.contains(on))
    };
    if depends("specforge-core", "specforge-conformance") {
        problems.push("specforge-core must not depend on specforge-conformance".to_string());
    }
    if names_conformance(&texts["specforge-core"]) {
        problems.push("specforge-core manifest names or aliases specforge-conformance".to_string());
    }
    if depends("specforge-core", "specforge") {
        problems.push("specforge-core must not depend on the specforge application".to_string());
    }
    if !depends("specforge-conformance", "specforge-core") {
        problems.push("specforge-conformance must depend on specforge-core".to_string());
    }
    if depends("specforge-conformance", "specforge") {
        problems
            .push("specforge-conformance must not depend on the specforge application".to_string());
    }
    if !depends("specforge", "specforge-core") {
        problems.push("specforge application must depend on specforge-core".to_string());
    }
    if !depends("specforge", "specforge-conformance") {
        problems.push("specforge application must depend on specforge-conformance".to_string());
    }
    for product in PRODUCT {
        if depends(product, "specforge-production-graph") {
            problems.push(format!(
                "{product} must not depend on the enforcement graph tool"
            ));
        }
        if depends("specforge-production-graph", product) {
            problems.push(format!(
                "the enforcement graph tool must not depend on {product}"
            ));
        }
    }
    let xtask = dependency_names(&slurp(&root.join("xtask/Cargo.toml"))?);
    let refused: Vec<&str> = xtask
        .iter()
        .map(String::as_str)
        .filter(|name| !XTASK_ALLOWED.contains(name))
        .collect();
    if !refused.is_empty() {
        problems.push(format!(
            "the xtask tooling crate may depend only on the already-locked serde_json or sha2, but names: {}",
            refused.join(" ")
        ));
    }
    for package in PRODUCT
        .iter()
        .copied()
        .chain(["specforge-production-graph"])
    {
        if depends(package, "xtask") {
            problems.push(format!(
                "{package} must not depend on the xtask tooling crate"
            ));
        }
    }
    let core_root = slurp(&root.join("crates/specforge-core/src/lib.rs"))?;
    if names_conformance(&core_root) {
        problems.push("core package root references the conformance package".to_string());
    }
    if ORACLE_SOURCES
        .iter()
        .any(|source| core_root.contains(source))
    {
        problems.push(
            "core package root directly includes a conformance/oracle source module".to_string(),
        );
    }
    let registry = slurp(&root.join("crates/specforge/src/ir/mod.rs"))?;
    if declares_oracle_module(&registry) {
        problems.push("core IR registry declares a conformance/oracle module".to_string());
    }
    Ok(problems)
}

// The self-test.

fn write_text(path: &Path, text: &str) -> Result<(), Death> {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, text).map_err(|error| Death {
        message: format!("cannot write {}: {error}", path.display()),
        code: error.raw_os_error().unwrap_or(255),
    })
}

fn write_fixture(fixture: &Path) -> Result<(), Death> {
    write_text(
        &fixture.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/specforge\", \"crates/specforge-core\", \"crates/specforge-conformance\", \
         \"tools/production-genericity-graph\", \"xtask\"]\n",
    )?;
    write_text(
        &fixture.join("xtask/Cargo.toml"),
        "[package]\nname = \"xtask\"\n\n[dependencies]\n",
    )?;
    write_text(
        &fixture.join("crates/specforge/Cargo.toml"),
        "[package]\nname = \"specforge\"\n[dependencies]\nspecforge-core = { path = \"../specforge-core\" }\n\
         specforge-conformance = { path = \"../specforge-conformance\" }\n",
    )?;
    write_text(
        &fixture.join("crates/specforge-core/Cargo.toml"),
        "[package]\nname = \"specforge-core\"\n[dependencies]\nserde = \"1\"\n",
    )?;
    write_text(
        &fixture.join("crates/specforge-conformance/Cargo.toml"),
        "[package]\nname = \"specforge-conformance\"\n[dependencies]\nspecforge-core = { path = \"../specforge-core\" }\n",
    )?;
    write_text(
        &fixture.join("tools/production-genericity-graph/Cargo.toml"),
        "[package]\nname = \"specforge-production-graph\"\n",
    )?;
    write_text(
        &fixture.join("crates/specforge-core/src/lib.rs"),
        "#[path = \"../../specforge/src/ir/mod.rs\"]\npub mod ir;\n",
    )?;
    write_text(
        &fixture.join("crates/specforge/src/ir/mod.rs"),
        "pub mod source;\n",
    )
}

/// The script's eleven mutations of a clean fixture written under `generated/`, each refused for its own reason;
/// the fixture is removed whatever the verdict.
fn run_self_tests(root: &Path) -> Result<(), Death> {
    let fixture = root.join(format!(
        "generated/production-genericity-dependency-tests.{}",
        std::process::id()
    ));
    if fixture.exists() {
        let _ = fs::remove_dir_all(&fixture);
    }
    let failure = self_test_cases(&fixture).err();
    if fixture.exists() {
        let _ = fs::remove_dir_all(&fixture);
    }
    if fixture.exists() {
        return Err(Death {
            message: format!("self-test residue remains at {}", fixture.display()),
            code: 255,
        });
    }
    failure.map_or(Ok(()), Err)
}

fn self_test_cases(fixture: &Path) -> Result<(), Death> {
    let die = |message: String| Death { message, code: 255 };
    let refused = |problems: &[String], fragment: &str| {
        problems.iter().any(|problem| problem.contains(fragment))
    };
    write_fixture(fixture)?;
    let clean = inspect_boundary(fixture)?;
    if !clean.is_empty() {
        return Err(die(format!(
            "self-test clean fixture failed: {}",
            clean.join(" ")
        )));
    }
    let core = fixture.join("crates/specforge-core/Cargo.toml");
    let conformance = fixture.join("crates/specforge-conformance/Cargo.toml");
    let registry = fixture.join("crates/specforge/src/ir/mod.rs");
    let graph = fixture.join("tools/production-genericity-graph/Cargo.toml");
    let xtask = fixture.join("xtask/Cargo.toml");
    let append = |path: &Path, text: &str| -> Result<(), Death> {
        write_text(path, &format!("{}{text}", slurp(path)?))
    };
    let cases: [(&Path, &str, &str, &str); 9] = [
        (
            &core,
            "specforge-conformance = { path = \"../specforge-conformance\" }\n",
            "must not depend on specforge-conformance",
            "self-test did not reject the reverse package dependency",
        ),
        (
            &core,
            "oracle = { package = \"specforge-conformance\", path = \"../specforge-conformance\" }\n",
            "manifest names or aliases specforge-conformance",
            "self-test did not reject an aliased reverse package dependency",
        ),
        (
            &conformance,
            "specforge = { path = \"../specforge\" }\n",
            "must not depend on the specforge application",
            "self-test did not reject the conformance-to-application dependency",
        ),
        (
            &registry,
            "pub mod trajectory;\n",
            "core IR registry declares",
            "self-test did not reject a conformance module in the core registry",
        ),
        (
            &core,
            "specforge-production-graph = { path = \"../../../tools/production-genericity-graph\" }\n",
            "must not depend on the enforcement graph tool",
            "self-test did not reject a product dependency on the enforcement tool",
        ),
        (
            &graph,
            "[dependencies]\nspecforge-core = { path = \"../../crates/specforge-core\" }\n",
            "enforcement graph tool must not depend on specforge-core",
            "self-test did not reject an enforcement-tool dependency on a product crate",
        ),
        (
            &xtask,
            "specforge-core = { path = \"../crates/specforge-core\" }\n",
            "xtask tooling crate may depend only on the already-locked serde_json or sha2, but names: specforge-core",
            "self-test did not reject a product dependency of the xtask tooling crate",
        ),
        (
            &xtask,
            "regex = \"1\"\n",
            " but names: regex",
            "self-test did not reject a new crate in the xtask tooling crate",
        ),
        (
            &core,
            "xtask = { path = \"../../xtask\" }\n",
            "specforge-core must not depend on the xtask tooling crate",
            "self-test did not reject a product dependency on the xtask tooling crate",
        ),
    ];
    for (index, (path, addition, fragment, failure)) in cases.iter().enumerate() {
        append(path, addition)?;
        let problems = inspect_boundary(fixture)?;
        let xtask_case = fragment.starts_with(" but names: regex");
        let caught = if xtask_case {
            problems.iter().any(|problem| {
                problem.contains("xtask tooling crate may depend only on ")
                    && problem.contains(fragment)
            })
        } else {
            refused(&problems, fragment)
        };
        if !caught {
            return Err(die((*failure).to_string()));
        }
        write_fixture(fixture)?;
        // The script admits an already-locked crate between the new-crate case and the reverse xtask case.
        if index == 7 {
            append(&xtask, "serde_json = { workspace = true }\n")?;
            let admitted = inspect_boundary(fixture)?;
            if !admitted.is_empty() {
                return Err(die(format!(
                    "self-test rejected an already-locked crate in the xtask tooling crate: {}",
                    admitted.join(" ")
                )));
            }
            write_fixture(fixture)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_real_workspace_holds_and_the_self_test_refuses_every_mutation() {
        let root = crate::repository_root();
        assert_eq!(inspect_boundary(&root).ok(), Some(Vec::new()));
        assert!(run_self_tests(&root).is_ok());
    }

    #[test]
    fn manifests_are_read_as_the_scripts_patterns_read_them() {
        assert_eq!(
            dependency_names(
                "[package]\n[dependencies]   \n\n  a = 1\nb-c={x}\n# c = 1\n[dev-dependencies]\nd = 1\n"
            ),
            vec!["a".to_string(), "b-c".to_string()]
        );
        assert_eq!(dependency_names("[dependencies]"), Vec::<String>::new());
        assert_eq!(
            workspace_members("[workspace]\nmembers = [\n  \"a\",\n  \"b\" ]\n"),
            vec!["a".to_string(), "b".to_string()]
        );
        assert_eq!(
            quoted_strings(r#""a", "", "b""#),
            vec!["a".to_string(), ", ".to_string()]
        );
        assert!(declares_oracle_module(
            "pub mod source;\n  pub  mod trajectory ;\n"
        ));
        assert!(!declares_oracle_module(
            "pub mod trajectory_view;\n// pub mod trajectory;\n"
        ));
    }
}
