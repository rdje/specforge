//! `cargo xtask check production-genericity`: the `PRODUCTION-GENERICITY` doctrine, the single unconditional
//! composition point for ADR 0006 and ADR 0038, native since `RUSTDOC-AND-XTASK.15.e`. It runs every component and
//! reports each, never stopping at the first failure: the package direction and tool disconnection, the module and
//! claim-family inventories, the rule joins, the compiled graph's information flow, and the frozen behavioral
//! contract; `--self-test` adds the CI qualification oracles, each component's mutations, the graph fixture and the
//! inventory-bound runtime alpha obligations of every rule.
//!
//! The four components written as xtask checks run in-process, their two streams captured as one in the order they
//! were written, as the script's `$(… 2>&1)` captured them; the graph crate and the two qualification tests run as
//! the script ran them, through `cargo` with the project-data environment. A component that passes prints what it
//! wrote to standard output, one that fails to standard error, and the report and summary go to standard error.
//!
//! `scripts/check_production_genericity.sh` is a shim that keeps its argument check and the project-data activation,
//! and passes the name it was invoked by (`--program`), which the gate's refusals quote.

use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

use super::{
    Captured, Output, behavioral_genericity_contract, production_genericity_dependencies,
    production_genericity_inventory, production_genericity_rules,
};
use crate::project_data;

const LABEL: &str = "production-genericity";

/// The components quantified over the persisted corpus (`COMMIT-GATE-SINGLE-RUN.15`): with no corpus they are
/// reported as not discharged, never run against nothing. The other components need no corpus, so the gate does
/// not skip as a whole.
const CORPUS_DEPENDENT_COMPONENTS: [&str; 2] =
    ["BEHAVIORAL-CONTRACT", "BEHAVIORAL-CONTRACT-MUTATIONS"];

/// How a component runs.
enum Runner {
    /// An xtask check, in-process, with these arguments.
    Check(
        fn(&Path, &[String], &mut dyn Output) -> i32,
        &'static [&'static str],
    ),
    /// `cargo` with these arguments; `{root}` is the repository root.
    Cargo(&'static [&'static str]),
}

/// One component: its id, what it proves, and how it runs.
struct Component {
    id: &'static str,
    proves: &'static str,
    runner: Runner,
}

const BASELINE: [Component; 5] = [
    Component {
        id: "DEPENDENCIES",
        proves: "the product package direction and enforcement-tool disconnection hold",
        runner: Runner::Check(production_genericity_dependencies::run_to, &[]),
    },
    Component {
        id: "INVENTORY",
        proves: "the production module and claim-family inventories are exact and complete",
        runner: Runner::Check(production_genericity_inventory::run_to, &[]),
    },
    Component {
        id: "RULES",
        proves: "every canonical field, producer/mutator, seam, and conformance bypass is registered",
        runner: Runner::Check(production_genericity_rules::run_to, &[]),
    },
    Component {
        id: "INFORMATION-FLOW",
        proves: "the compiled graph enforces raw/identity noninterference and proof-only promotion",
        // `scripts/check_production_genericity_flow.sh`'s command; it stays the graph's standalone entry.
        runner: Runner::Cargo(&[
            "run",
            "--quiet",
            "--locked",
            "--offline",
            "-p",
            "specforge-production-graph",
            "--",
            "--root",
            "{root}",
            "--check-census",
        ]),
    },
    Component {
        id: "BEHAVIORAL-CONTRACT",
        proves: "the behavioral population, input planes, relations, held-out split, and evidence taxonomy are \
                 frozen",
        runner: Runner::Check(behavioral_genericity_contract::run_to, &[]),
    },
];

const QUALIFICATION: [Component; 6] = [
    Component {
        id: "DEPENDENCY-MUTATIONS",
        proves: "forbidden product/tool dependency directions fail closed",
        runner: Runner::Check(production_genericity_dependencies::run_to, &["--self-test"]),
    },
    Component {
        id: "INVENTORY-MUTATIONS",
        proves: "unclassified modules and schema-specialized artifact fields fail closed",
        runner: Runner::Check(production_genericity_inventory::run_to, &["--self-test"]),
    },
    Component {
        id: "RULE-MUTATIONS",
        proves: "missing rules, entrypoints, alpha declarations, and safe bypass contracts fail closed",
        runner: Runner::Check(production_genericity_rules::run_to, &["--self-test"]),
    },
    Component {
        id: "FLOW-MUTATIONS",
        proves: "identity, raw literal/substring/regex, laundering, unregistered mutation, and proofless promotion \
                 fail closed while legal sinks pass",
        runner: Runner::Cargo(&[
            "test",
            "--quiet",
            "--locked",
            "--offline",
            "-p",
            "specforge-production-graph",
            "--lib",
            "tests::information_flow_fixture_fails_closed_on_boundary_breaches",
            "--",
            "--exact",
        ]),
    },
    Component {
        id: "BEHAVIORAL-CONTRACT-MUTATIONS",
        proves: "population omission, unsafe authority, hash drift, leakage, vacuity, and partial comparison \
                 declarations fail closed",
        runner: Runner::Check(behavioral_genericity_contract::run_to, &["--self-test"]),
    },
    Component {
        id: "ALPHA-OBLIGATIONS",
        proves: "all 170 runtime rules exactly match inventory and execute their structural alpha obligations",
        runner: Runner::Cargo(&[
            "test",
            "--quiet",
            "--locked",
            "--offline",
            "-p",
            "specforge-core",
            "--lib",
            "ir::production_genericity_qualification_tests::\
             every_registered_rule_satisfies_its_inventory_bound_structural_alpha_obligation",
            "--",
            "--exact",
        ]),
    },
];

/// `[--program NAME] [--self-test]`: the shim passes the name it was invoked by.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    let mut program = "scripts/check_production_genericity.sh".to_string();
    let mut qualify = false;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--program" => match arguments.next() {
                Some(name) => program.clone_from(name),
                None => return usage(&program),
            },
            "--self-test" if !qualify => qualify = true,
            _ => return usage(&program),
        }
    }
    let environment = match project_data::activate(root) {
        Ok(environment) => environment,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let corpus_absent = corpus_absent(root);
    if corpus_absent
        && std::env::var_os("SPECFORGE_CORPUS_ABSENT").is_none_or(|value| value.is_empty())
    {
        eprintln!(
            "{program}: refused — generated/source_ir is empty or absent, so {} components of this gate",
            CORPUS_DEPENDENT_COMPONENTS.len()
        );
        eprintln!(
            "{program}: have no subject in this tree and would fail on the missing artifacts."
        );
        eprintln!(
            "{program}: build the corpus, or declare the environment corpus-free with SPECFORGE_CORPUS_ABSENT=1"
        );
        eprintln!("{program}: to have them reported as NOT discharged instead.");
        return 2;
    }
    let mut components: Vec<&Component> = BASELINE.iter().collect();
    if qualify {
        components.extend(QUALIFICATION.iter());
    }
    let mut report: Vec<String> = Vec::new();
    let mut failed = false;
    for component in &components {
        if corpus_absent && CORPUS_DEPENDENT_COMPONENTS.contains(&component.id) {
            report.push(format!(
                "SKIP  {} — corpus absent and declared; NOTHING was measured",
                component.id
            ));
            println!(
                "{LABEL}: SKIP: no corpus at generated/source_ir — {} does not govern this tree. Nothing was \
                 measured; nothing is claimed.",
                component.id
            );
            continue;
        }
        let (passed, output) = match &component.runner {
            Runner::Check(check, arguments) => {
                let arguments: Vec<String> = arguments
                    .iter()
                    .map(|argument| (*argument).to_string())
                    .collect();
                let mut captured = Captured::default();
                let code = check(root, &arguments, &mut captured);
                (code == 0, captured.text.into_bytes())
            }
            Runner::Cargo(arguments) => cargo(root, &environment, arguments),
        };
        // `$(…)` drops every trailing newline, and `printf '%s\n'` adds one back.
        let trimmed = trim_trailing_newlines(&output);
        if passed {
            report.push(format!("PASS  {} — {}", component.id, component.proves));
            if !trimmed.is_empty() {
                write_line(&mut std::io::stdout(), trimmed);
            }
        } else {
            report.push(format!("FAIL  {} — {}", component.id, component.proves));
            if !trimmed.is_empty() {
                write_line(&mut std::io::stderr(), trimmed);
            }
            failed = true;
        }
    }
    // A dangling corpus-dependent id would silently stop guarding a component; `--self-test` reaches them all.
    if qualify {
        for dependent in CORPUS_DEPENDENT_COMPONENTS {
            if !components.iter().any(|component| component.id == dependent) {
                eprintln!(
                    "{program}: CORPUS_DEPENDENT_COMPONENTS names no component of this gate: {dependent}"
                );
                return 2;
            }
        }
    }
    eprintln!("\n============= PRODUCTION GENERICITY REPORT =============");
    for line in &report {
        eprintln!("  {line}");
    }
    eprintln!("========================================================");
    if failed {
        eprintln!("{LABEL}: one or more requested components FAILED.");
        return 1;
    }
    for line in summary(&report, qualify) {
        eprintln!("{line}");
    }
    0
}

/// The passing run's closing lines. A skipped component is not a passing one, and the count must not absorb it
/// (`COMMIT-GATE-SINGLE-RUN.15`): it is named as not governed instead.
fn summary(report: &[String], qualify: bool) -> Vec<String> {
    let ungoverned: Vec<&str> = report
        .iter()
        .filter_map(|line| {
            line.strip_prefix("SKIP  ")
                .and_then(|rest| rest.split(' ').next())
        })
        .collect();
    let executed = report.len() - ungoverned.len();
    let scope = if qualify {
        "baseline and qualification"
    } else {
        "baseline contract"
    };
    let mut lines = vec![format!(
        "{LABEL}: {executed} of {} {scope} components executed and PASS.",
        report.len()
    )];
    if !ungoverned.is_empty() {
        lines.push(format!(
            "{LABEL}: NOT GOVERNED here, so NOT enforced by this run: {}.",
            ungoverned.join(", ")
        ));
    }
    lines
}

fn usage(program: &str) -> i32 {
    eprintln!("usage: {program} [--self-test]");
    2
}

/// `[ ! -d generated/source_ir ] || [ -z "$(ls -A generated/source_ir)" ]`.
fn corpus_absent(root: &Path) -> bool {
    std::fs::read_dir(root.join("generated/source_ir"))
        .map_or(true, |mut entries| entries.next().is_none())
}

/// `cargo` with `arguments` from the repository root, with the project-data environment, its standard output and
/// error one pipe as `2>&1` makes them: whether it succeeded, and what it wrote.
fn cargo(
    root: &Path,
    environment: &project_data::Environment,
    arguments: &[&str],
) -> (bool, Vec<u8>) {
    let root_text = root.to_string_lossy();
    let mut command: Command = project_data::command("cargo", root, environment);
    command.args(
        arguments
            .iter()
            .map(|argument| argument.replace("{root}", &root_text)),
    );
    let outcome = (|| -> std::io::Result<(bool, Vec<u8>)> {
        let (mut reader, writer) = std::io::pipe()?;
        command.stdout(writer.try_clone()?).stderr(writer);
        let mut child = command.spawn()?;
        // The command holds the pipe's write ends; until they close, reading would never end.
        drop(command);
        let mut output = Vec::new();
        reader.read_to_end(&mut output)?;
        Ok((child.wait()?.success(), output))
    })();
    outcome.unwrap_or_else(|error| (false, format!("cargo: {error}\n").into_bytes()))
}

fn trim_trailing_newlines(bytes: &[u8]) -> &[u8] {
    let end = bytes
        .iter()
        .rposition(|byte| *byte != b'\n')
        .map_or(0, |index| index + 1);
    &bytes[..end]
}

fn write_line(stream: &mut dyn Write, bytes: &[u8]) {
    let _ = stream.write_all(bytes);
    let _ = stream.write_all(b"\n");
    let _ = stream.flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captured_output_reads_as_command_substitution_reads_it() {
        assert_eq!(trim_trailing_newlines(b"a\nb\n\n\n"), b"a\nb");
        assert_eq!(trim_trailing_newlines(b"\n\n"), b"");
        assert_eq!(trim_trailing_newlines(b"a"), b"a");
    }

    #[test]
    fn a_cargo_component_gets_both_streams_in_one_pipe() {
        let root = crate::repository_root();
        let environment = project_data::activate(&root).expect("the project-data roots");
        let (passed, output) = cargo(&root, &environment, &["--version"]);
        assert!(passed);
        assert!(String::from_utf8_lossy(&output).starts_with("cargo "));
        let (passed, output) = cargo(&root, &environment, &["no-such-cargo-command"]);
        assert!(!passed);
        assert!(String::from_utf8_lossy(&output).contains("no-such-cargo-command"));
    }

    #[test]
    fn arguments_are_refused_as_the_script_refused_them() {
        let root = crate::repository_root();
        let arguments = |words: &[&str]| {
            words
                .iter()
                .map(|word| (*word).to_string())
                .collect::<Vec<_>>()
        };
        assert_eq!(run(&root, &arguments(&["--bogus"])), 2);
        assert_eq!(run(&root, &arguments(&["--self-test", "--self-test"])), 2);
        assert_eq!(run(&root, &arguments(&["--program"])), 2);
    }

    #[test]
    fn a_skipped_component_is_named_and_never_counted() {
        let report = [
            "PASS  DEPENDENCIES — x".to_string(),
            "SKIP  BEHAVIORAL-CONTRACT — corpus absent and declared; NOTHING was measured"
                .to_string(),
        ];
        assert_eq!(
            summary(&report, false),
            vec![
                "production-genericity: 1 of 2 baseline contract components executed and PASS.".to_string(),
                "production-genericity: NOT GOVERNED here, so NOT enforced by this run: BEHAVIORAL-CONTRACT."
                    .to_string(),
            ]
        );
        assert_eq!(
            summary(&report[..1], true),
            vec!["production-genericity: 1 of 1 baseline and qualification components executed and PASS.".to_string()]
        );
    }

    #[test]
    fn the_corpus_is_absent_without_an_entry_under_generated_source_ir() {
        let scratch =
            crate::repository_root().join(".project-data/tmp/xtask-production-genericity");
        let _ = std::fs::remove_dir_all(&scratch);
        assert!(corpus_absent(&scratch));
        std::fs::create_dir_all(scratch.join("generated/source_ir")).expect("the fixture");
        assert!(corpus_absent(&scratch));
        std::fs::write(scratch.join("generated/source_ir/x"), b"").expect("an entry");
        assert!(!corpus_absent(&scratch));
        std::fs::remove_dir_all(&scratch).expect("the fixture removed");
        assert!(!corpus_absent(&crate::repository_root()));
    }

    #[test]
    fn every_corpus_dependent_component_is_a_component() {
        for dependent in CORPUS_DEPENDENT_COMPONENTS {
            assert!(
                BASELINE
                    .iter()
                    .chain(QUALIFICATION.iter())
                    .any(|component| component.id == dependent)
            );
        }
    }
}
