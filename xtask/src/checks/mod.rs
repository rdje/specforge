//! `cargo xtask check <doctrine>`: the doctrine gates ported to Rust (`RUSTDOC-AND-XTASK.6`). Each replaces the
//! script the doctrine driver registers, which stays as a shim over it.

pub mod behavioral_genericity_contract;
pub mod chain_currency;
pub mod constraint_part_span;
pub mod contract_encoding;
pub mod corpus_frontier;
pub mod doctrine_tables;
pub mod ownership_citations;
pub mod persisted_artifact_paths;
pub mod production_genericity;
pub mod production_genericity_dependencies;
pub mod production_genericity_inventory;
pub mod production_genericity_rules;
pub mod project_data_locality;
pub mod proof_seal_currency;
pub mod published_assertions;
pub mod readme_policy;
pub mod residual_actionability;
pub mod rustdoc;
pub mod section_anchors;
pub mod task_node_retention;

use std::path::Path;

/// One ported doctrine check: its name, the script it replaced, and its entry point.
#[derive(Debug, Clone, Copy)]
pub struct Check {
    /// The doctrine's name in lower case, as `cargo xtask check <name>` takes it.
    pub name: &'static str,
    /// The repository-relative script the doctrine driver registers, now a shim over this check.
    pub replaces: &'static str,
    /// The check: the repository root and the arguments in, the exit code out.
    pub run: fn(&Path, &[String]) -> i32,
}

/// Every ported check, in the order `.6` ports them.
pub const CHECKS: &[Check] = &[
    Check {
        name: "rustdoc",
        replaces: "scripts/check_rustdoc.sh",
        run: rustdoc::run,
    },
    Check {
        name: "readme-policy",
        replaces: "scripts/check_readme_policy.sh",
        run: readme_policy::run,
    },
    Check {
        name: "section-anchors",
        replaces: "scripts/check_section_anchors.pl",
        run: section_anchors::run,
    },
    Check {
        name: "task-node-retention",
        replaces: "scripts/check_task_node_retention.py",
        run: task_node_retention::run,
    },
    Check {
        name: "contract-encoding",
        replaces: "scripts/check_contract_encoding.py",
        run: contract_encoding::run,
    },
    Check {
        name: "ownership-citations",
        replaces: "scripts/check_ownership_citations.pl",
        run: ownership_citations::run,
    },
    Check {
        name: "doctrine-tables",
        replaces: "scripts/check_doctrine_tables.sh",
        run: doctrine_tables::run,
    },
    Check {
        name: "published-assertions",
        replaces: "scripts/check_published_assertions.pl",
        run: published_assertions::run,
    },
    Check {
        name: "residual-actionability",
        replaces: "scripts/validate_residual_actionability_contract.py",
        run: residual_actionability::run,
    },
    Check {
        name: "corpus-frontier",
        replaces: "scripts/check_corpus_frontier.sh",
        run: corpus_frontier::run,
    },
    Check {
        name: "constraint-part-span",
        replaces: "scripts/check_constraint_part_span.sh",
        run: constraint_part_span::run,
    },
    Check {
        name: "project-data-locality",
        replaces: "scripts/check_project_data_locality.sh",
        run: project_data_locality::run,
    },
    Check {
        name: "chain-currency",
        replaces: "scripts/check_chain_currency.sh",
        run: chain_currency::run,
    },
    Check {
        name: "proof-seal-currency",
        replaces: "scripts/check_proof_seal_currency.sh",
        run: proof_seal_currency::run,
    },
    Check {
        name: "proof-seal-total",
        replaces: "scripts/check_proof_seal_total.sh",
        run: proof_seal_currency::run_total,
    },
    Check {
        name: "persisted-artifact-paths",
        replaces: "scripts/check_persisted_artifact_paths.pl",
        run: persisted_artifact_paths::run,
    },
    Check {
        name: "production-genericity-dependencies",
        replaces: "scripts/check_production_genericity_dependencies.pl",
        run: production_genericity_dependencies::run,
    },
    Check {
        name: "production-genericity-inventory",
        replaces: "scripts/check_production_genericity_inventory.pl",
        run: production_genericity_inventory::run,
    },
    Check {
        name: "production-genericity-rules",
        replaces: "scripts/check_production_genericity_rules.pl",
        run: production_genericity_rules::run,
    },
    Check {
        name: "behavioral-genericity-contract",
        replaces: "scripts/check_behavioral_genericity_contract.py",
        run: behavioral_genericity_contract::run,
    },
    Check {
        name: "production-genericity",
        replaces: "scripts/check_production_genericity.sh",
        run: production_genericity::run,
    },
];

/// Where a check writes (`RUSTDOC-AND-XTASK.15.e`): the terminal's two streams, or one buffer holding both in the
/// order they were written, as a shell's `$(… 2>&1)` captured a component.
pub trait Output {
    /// Text for standard output.
    fn out(&mut self, text: &str);
    /// Text for standard error.
    fn err(&mut self, text: &str);
}

/// The terminal: standard output and standard error.
pub struct Terminal;

impl Output for Terminal {
    fn out(&mut self, text: &str) {
        print!("{text}");
    }

    fn err(&mut self, text: &str) {
        eprint!("{text}");
    }
}

/// Both streams in one buffer, in the order they were written.
#[derive(Default)]
pub struct Captured {
    /// What the check wrote, to either stream.
    pub text: String,
}

impl Output for Captured {
    fn out(&mut self, text: &str) {
        self.text.push_str(text);
    }

    fn err(&mut self, text: &str) {
        self.text.push_str(text);
    }
}

/// Run the check named by the first argument with the rest; a missing or unknown name prints the checks and
/// exits 2.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    let Some(name) = arguments.first() else {
        eprint!("{}", usage());
        return 2;
    };
    match CHECKS.iter().find(|check| check.name == name) {
        Some(check) => (check.run)(root, &arguments[1..]),
        None => {
            eprintln!("xtask: unknown check '{name}'");
            eprint!("{}", usage());
            2
        }
    }
}

/// The checks `cargo xtask check` knows.
pub fn usage() -> String {
    let mut text = String::from("Usage: cargo xtask check <doctrine>\n\nChecks:\n");
    for check in CHECKS {
        text.push_str(&format!(
            "  {}  (replaces {})\n",
            check.name, check.replaces
        ));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_check_is_listed_and_a_missing_or_unknown_one_exits_2() {
        let help = usage();
        for check in CHECKS {
            assert!(help.contains(check.name) && help.contains(check.replaces));
        }
        let root = crate::repository_root();
        assert_eq!(run(&root, &[]), 2);
        assert_eq!(run(&root, &["no-such-check".to_string()]), 2);
    }
}
