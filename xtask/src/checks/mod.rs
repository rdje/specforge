//! `cargo xtask check <doctrine>`: the doctrine gates ported to Rust (`RUSTDOC-AND-XTASK.6`). Each replaces the
//! script the doctrine driver registers, which stays as a shim over it.

pub mod contract_encoding;
pub mod readme_policy;
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
];

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
