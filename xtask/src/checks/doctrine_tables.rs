//! The `DOCTRINE-TABLES` doctrine (`RUSTDOC-AND-XTASK.18`), native from its first commit: the doctrines the driver
//! registers (`scripts/check_doctrines.sh`, its `DOCTRINES` entries) are exactly the doctrines documented in
//! `docs/DOCTRINE_INSTANCE.md` §10, each with the tier and check the driver gives it, and exactly the doctrines in the
//! book's enforcement table (`docs/book/src/reference/doctrine-enforcement.md`), each with the kind §10 gives it.
//!
//! §10 is "the complete doctrine registry" (`DOCTRINE_ENFORCEMENT.md` §10), and nothing held it complete: by
//! `RUSTDOC-AND-XTASK.13.c` it lacked one of the 20 registered doctrines and the book's table lacked five, with every
//! gate green.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const DRIVER: &str = "scripts/check_doctrines.sh";
const INSTANCE: &str = "docs/DOCTRINE_INSTANCE.md";
const INSTANCE_SECTION: &str = "## 10.";
const BOOK: &str = "docs/book/src/reference/doctrine-enforcement.md";
const BOOK_SECTION: &str = "## What is enforced today";

/// One doctrine the driver registers: its tier and the check it runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registered {
    /// `gate` or `ci`.
    pub tier: String,
    /// The repository-relative script the driver runs.
    pub script: String,
}

/// One row of a doctrine table: the cells after the backticked id, trimmed, backticks dropped.
pub type Row = Vec<String>;

/// Run the check from `root`, or with `--self-test` prove that each refusal fails closed.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    match arguments {
        [] => {}
        [mode] if mode == "--self-test" => return self_test(),
        _ => {
            eprintln!("Usage: cargo xtask check doctrine-tables [--self-test]");
            return 2;
        }
    }
    let read = |path: &str| {
        fs::read_to_string(root.join(path)).map_err(|error| format!("cannot read {path}: {error}"))
    };
    let texts = (read(DRIVER), read(INSTANCE), read(BOOK));
    let (Ok(driver), Ok(instance), Ok(book)) = texts.clone() else {
        for error in [texts.0.err(), texts.1.err(), texts.2.err()]
            .into_iter()
            .flatten()
        {
            eprintln!("doctrine-tables: {error}");
        }
        return 1;
    };
    let problems = problems(&driver, &instance, &book);
    if problems.is_empty() {
        println!(
            "doctrine-tables: the {} doctrines {DRIVER} registers each have one row in {INSTANCE} §10, with their tier and check, and one in the book's enforcement table, with the same kind.",
            registered(&driver).map_or(0, |registry| registry.len())
        );
        return 0;
    }
    for problem in &problems {
        eprintln!("doctrine-tables: {problem}");
    }
    eprintln!(
        "doctrine-tables: FAILED with {} problem(s).",
        problems.len()
    );
    1
}

/// Every way the three documents disagree.
pub fn problems(driver: &str, instance: &str, book: &str) -> Vec<String> {
    let registry = match registered(driver) {
        Ok(registry) => registry,
        Err(problem) => return vec![problem],
    };
    let mut problems = Vec::new();
    let instance_rows = rows(instance, INSTANCE_SECTION, INSTANCE, &mut problems);
    let book_rows = rows(book, BOOK_SECTION, BOOK, &mut problems);
    for (id, entry) in &registry {
        match instance_rows.get(id) {
            None => problems.push(format!(
                "{id} is registered in {DRIVER} but has no row in {INSTANCE} §10"
            )),
            Some(row) => {
                if row.get(1) != Some(&entry.tier) {
                    problems.push(format!(
                        "{INSTANCE} §10 gives {id} the tier '{}', the driver '{}'",
                        row.get(1).map_or("", String::as_str),
                        entry.tier
                    ));
                }
                if row.get(2) != Some(&entry.script) {
                    problems.push(format!(
                        "{INSTANCE} §10 gives {id} the check '{}', the driver '{}'",
                        row.get(2).map_or("", String::as_str),
                        entry.script
                    ));
                }
                match book_rows.get(id) {
                    Some(book_row) if book_row.first() != row.first() => problems.push(format!(
                        "the book's table gives {id} the kind '{}', {INSTANCE} §10 '{}'",
                        book_row.first().map_or("", String::as_str),
                        row.first().map_or("", String::as_str)
                    )),
                    _ => {}
                }
            }
        }
        if !book_rows.contains_key(id) {
            problems.push(format!("{id} is registered in {DRIVER} but has no row in the book's enforcement table ({BOOK})"));
        }
    }
    for id in instance_rows
        .keys()
        .filter(|id| !registry.contains_key(*id))
    {
        problems.push(format!(
            "{INSTANCE} §10 has a row for {id}, which {DRIVER} does not register"
        ));
    }
    for id in book_rows.keys().filter(|id| !registry.contains_key(*id)) {
        problems.push(format!(
            "the book's enforcement table has a row for {id}, which {DRIVER} does not register"
        ));
    }
    problems
}

/// The driver's `DOCTRINES` entries, `"ID|tier|proves|script"` each, by id.
pub fn registered(driver: &str) -> Result<BTreeMap<String, Registered>, String> {
    let start = driver
        .find("\nDOCTRINES=(\n")
        .ok_or_else(|| format!("{DRIVER} declares no DOCTRINES array"))?;
    let body = &driver[start + "\nDOCTRINES=(\n".len()..];
    let body = &body[..body
        .find("\n)")
        .ok_or_else(|| format!("{DRIVER}'s DOCTRINES array is not closed"))?];
    let mut registry = BTreeMap::new();
    for line in body.lines() {
        let line = line.trim();
        let Some(entry) = line
            .strip_prefix('"')
            .and_then(|line| line.strip_suffix('"'))
        else {
            continue;
        };
        let fields: Vec<&str> = entry.split('|').collect();
        let [id, tier, _proves, script] = fields[..] else {
            return Err(format!(
                "{DRIVER} has a DOCTRINES entry that is not ID|tier|proves|script: {entry}"
            ));
        };
        let entry = Registered {
            tier: tier.to_string(),
            script: script.to_string(),
        };
        if registry.insert(id.to_string(), entry).is_some() {
            return Err(format!("{DRIVER} registers {id} twice"));
        }
    }
    Ok(registry)
}

/// The rows of the first table under `section`, each by its backticked id; a repeated id is a problem.
fn rows(
    text: &str,
    section: &str,
    path: &str,
    problems: &mut Vec<String>,
) -> BTreeMap<String, Row> {
    let mut rows = BTreeMap::new();
    let Some(start) = text.split('\n').position(|line| line.starts_with(section)) else {
        problems.push(format!("{path} has no section '{section}'"));
        return rows;
    };
    for line in text
        .split('\n')
        .skip(start + 1)
        .take_while(|line| !line.starts_with("## "))
    {
        let Some(rest) = line.strip_prefix("| `") else {
            continue;
        };
        let Some((id, rest)) = rest.split_once('`') else {
            continue;
        };
        if !id.chars().all(|character| {
            character.is_ascii_uppercase() || character.is_ascii_digit() || character == '-'
        }) {
            continue;
        }
        let cells: Row = rest
            .trim_start_matches(" |")
            .split(" | ")
            .map(|cell| cell.trim().trim_matches('`').to_string())
            .collect();
        if rows.insert(id.to_string(), cells).is_some() {
            problems.push(format!("{path} has two rows for {id}"));
        }
    }
    rows
}

/// Each refusal on a fixture: the three documents agreeing pass, and each way of disagreeing is named.
fn self_test() -> i32 {
    let driver = "x\nDOCTRINES=(\n  \"ALPHA|gate|proves a|scripts/a.sh\"\n  \"BETA|ci|proves b|scripts/b.pl\"\n)\n";
    let instance = "## 10. The registry\n\n| `ALPHA` | structural | gate | `scripts/a.sh` | a |\n| `BETA` | oracle | ci | `scripts/b.pl` | b |\n\n## 11. Next\n| `GAMMA` | x | gate | `scripts/g.sh` | outside the section |\n";
    let book =
        "## What is enforced today\n\n| `ALPHA` | structural | a |\n| `BETA` | oracle | b |\n";
    let cases: [(&str, String, String, String, &str); 8] = [
        (
            "agreeing tables",
            driver.into(),
            instance.into(),
            book.into(),
            "",
        ),
        (
            "a doctrine missing from §10",
            driver.into(),
            instance.replace("| `BETA` | oracle | ci | `scripts/b.pl` | b |\n", ""),
            book.into(),
            "BETA is registered in scripts/check_doctrines.sh but has no row in docs/DOCTRINE_INSTANCE.md §10",
        ),
        (
            "a doctrine missing from the book",
            driver.into(),
            instance.into(),
            book.replace("| `BETA` | oracle | b |\n", ""),
            "BETA is registered in scripts/check_doctrines.sh but has no row in the book's enforcement table",
        ),
        (
            "an unregistered row",
            driver.replace("  \"BETA|ci|proves b|scripts/b.pl\"\n", ""),
            instance.into(),
            book.into(),
            "docs/DOCTRINE_INSTANCE.md §10 has a row for BETA, which scripts/check_doctrines.sh does not register",
        ),
        (
            "a tier that disagrees",
            driver.replace("BETA|ci", "BETA|gate"),
            instance.into(),
            book.into(),
            "gives BETA the tier 'ci', the driver 'gate'",
        ),
        (
            "a check that disagrees",
            driver.replace("scripts/b.pl", "scripts/b2.pl"),
            instance.into(),
            book.into(),
            "gives BETA the check 'scripts/b.pl', the driver 'scripts/b2.pl'",
        ),
        (
            "a kind that disagrees",
            driver.into(),
            instance.into(),
            book.replace("| `BETA` | oracle |", "| `BETA` | structural |"),
            "the book's table gives BETA the kind 'structural'",
        ),
        (
            "a repeated row",
            driver.into(),
            instance.into(),
            format!("{book}| `ALPHA` | structural | again |\n"),
            "has two rows for ALPHA",
        ),
    ];
    for (name, driver, instance, book, wanted) in &cases {
        let found = problems(driver, instance, book);
        let holds = if wanted.is_empty() {
            found.is_empty()
        } else {
            found.iter().any(|problem| problem.contains(wanted))
        };
        if !holds {
            eprintln!("doctrine-tables self-test: '{name}' expected {wanted:?}, found {found:?}");
            return 1;
        }
    }
    println!(
        "doctrine-tables self-test: {} agreement and refusal cases hold.",
        cases.len()
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_self_test_cases_hold() {
        assert_eq!(self_test(), 0);
    }

    #[test]
    fn the_repository_tables_agree_with_the_driver() {
        let root = crate::repository_root();
        let read = |path: &str| fs::read_to_string(root.join(path)).expect(path);
        let driver = read(DRIVER);
        assert_eq!(
            problems(&driver, &read(INSTANCE), &read(BOOK)),
            Vec::<String>::new()
        );
        assert!(
            registered(&driver)
                .expect("registry")
                .contains_key("DOCTRINE-TABLES")
        );
    }

    #[test]
    fn a_malformed_registry_entry_is_refused() {
        assert!(registered("DOCTRINES=(\n  \"A|gate|x\"\n)\n").is_err());
        let driver = "\nDOCTRINES=(\n  \"A|gate|x|s.sh\"\n  \"A|gate|x|s.sh\"\n)\n";
        assert_eq!(
            registered(driver).unwrap_err(),
            "scripts/check_doctrines.sh registers A twice"
        );
    }
}
