//! The `SECTION-ANCHORS` doctrine (`LIVE-DOCUMENT-PRESSURE-HEADROOM.4f`), native since `RUSTDOC-AND-XTASK.13.a`:
//! every qualified cross-document section reference, `` `path.md` §`section` ``, resolves to a heading in the file
//! it names whose text, with backticks dropped and whitespace collapsed, contains the cited section. A bare
//! `` §`section` `` names no file and is resolved from its prose, which is not mechanically decidable, so it is
//! counted and never guessed at (`DOCTRINE_ENFORCEMENT.md` §9). The check reads the tree and writes nothing.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

const USAGE: &str =
    "Usage: scripts/check_section_anchors.pl [--check|--report|--self-test] [--root DIR]";
const SECTION: char = '\u{a7}';

/// Run the check from `root`, or from `--root DIR`; `--self-test` proves the comparison accepts a cited heading
/// and refuses an absent section. `--check` and `--report` both run the check.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    let mut root = root.to_path_buf();
    let mut self_test = false;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" | "--report" => self_test = false,
            "--self-test" => self_test = true,
            "--root" => {
                let Some(directory) = arguments.next() else {
                    eprintln!("section-anchors: --root requires a path");
                    return 2;
                };
                match fs::canonicalize(directory) {
                    Ok(directory) => root = directory,
                    Err(_) => {
                        eprintln!("section-anchors: --root does not resolve");
                        return 2;
                    }
                }
            }
            _ => {
                eprintln!("{USAGE}");
                return 2;
            }
        }
    }
    if self_test {
        return self_test_verdict();
    }
    let scan = scan(&root, &tracked_markdown(&root));
    if !scan.violations.is_empty() {
        for violation in &scan.violations {
            eprintln!("section-anchors: {violation}");
        }
        eprintln!(
            "section-anchors: FAILED with {} unresolved section reference(s).",
            scan.violations.len()
        );
        return 1;
    }
    println!(
        "section-anchors: {} qualified section reference(s) resolve to a real heading ({} bare {SECTION} reference(s) are context-relative and not mechanically checkable).",
        scan.resolved, scan.bare
    );
    0
}

/// What a scan of the tracked Markdown found.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Scan {
    /// Qualified references whose section a heading of the named file contains.
    pub resolved: usize,
    /// Every `§` followed by a backticked section and not itself inside backticks, qualified or not.
    pub bare: usize,
    /// One line per reference that does not resolve.
    pub violations: Vec<String>,
}

/// Scan `files` (repository-relative) under `root`, resolving each qualified reference against its target's
/// headings, read once per target.
pub fn scan(root: &Path, files: &[String]) -> Scan {
    let mut scan = Scan::default();
    let mut headings_of: HashMap<String, Option<Vec<String>>> = HashMap::new();
    for file in files {
        let Some(text) = read_text(&root.join(file)) else {
            continue;
        };
        scan.bare += bare_references(&text);
        for (target, section) in qualified_references(&text) {
            let section = normalize(&section);
            let headings = headings_of
                .entry(target.clone())
                .or_insert_with(|| read_text(&root.join(&target)).map(|text| headings(&text)));
            match headings {
                None => scan.violations.push(format!(
                    "{file} cites '{target}' {SECTION}'{section}' but that file is not readable"
                )),
                Some(headings) if headings.iter().any(|heading| heading.contains(&section)) => {
                    scan.resolved += 1
                }
                Some(_) => scan.violations.push(format!(
                    "{file} cites '{target}' {SECTION}'{section}' but no heading in that file contains it (a moved section must keep a redirect heading at the cited target)"
                )),
            }
        }
    }
    scan
}

/// The tracked Markdown files under `root` (`git ls-files --cached -- '*.md'`), sorted.
pub fn tracked_markdown(root: &Path) -> Vec<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-z", "--cached", "--", "*.md"])
        .stderr(std::process::Stdio::inherit())
        .output();
    let mut files: Vec<String> = match output {
        Ok(output) => output
            .stdout
            .split(|byte| *byte == 0)
            .filter(|path| !path.is_empty())
            .map(|path| String::from_utf8_lossy(path).into_owned())
            .collect(),
        Err(_) => Vec::new(),
    };
    files.sort();
    files
}

/// A file's text, invalid UTF-8 replaced; a directory reads as empty, as Perl's `open` reads it; `None` when it
/// cannot be read.
fn read_text(path: &Path) -> Option<String> {
    if path.is_dir() {
        return Some(String::new());
    }
    fs::read(path)
        .ok()
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
}

/// A heading or a citation in one comparable form: no backticks, whitespace runs collapsed to one space, trimmed.
pub fn normalize(text: &str) -> String {
    text.replace('`', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// The normalized text of each ATX heading: one to six `#`, whitespace, then the rest of the line.
pub fn headings(text: &str) -> Vec<String> {
    text.split('\n')
        .filter_map(|line| {
            let level = line
                .chars()
                .take_while(|character| *character == '#')
                .count();
            let rest = &line[level..];
            ((1..=6).contains(&level) && rest.starts_with(char::is_whitespace))
                .then(|| normalize(rest))
        })
        .collect()
}

fn is_path_character(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '_' | '.' | '/' | '-')
}

/// Each `` `target.md` §`section` `` in `text` (whitespace allowed before the `§`), left to right without overlap;
/// the target is a run of word characters, `.`, `/` and `-` that ends in `.md`.
pub fn qualified_references(text: &str) -> Vec<(String, String)> {
    let characters: Vec<char> = text.chars().collect();
    let mut references = Vec::new();
    let mut index = 0;
    while index < characters.len() {
        match qualified_at(&characters, index) {
            Some((target, section, end)) => {
                references.push((target, section));
                index = end;
            }
            None => index += 1,
        }
    }
    references
}

/// The reference starting at `start`, with the index just past its closing backtick.
fn qualified_at(characters: &[char], start: usize) -> Option<(String, String, usize)> {
    if characters[start] != '`' {
        return None;
    }
    let mut index = start + 1;
    while index < characters.len() && is_path_character(characters[index]) {
        index += 1;
    }
    let target: String = characters[start + 1..index].iter().collect();
    if index >= characters.len()
        || characters[index] != '`'
        || target.chars().count() < 4
        || !target.ends_with(".md")
    {
        return None;
    }
    index += 1;
    while index < characters.len() && characters[index].is_whitespace() {
        index += 1;
    }
    if characters.get(index) != Some(&SECTION) || characters.get(index + 1) != Some(&'`') {
        return None;
    }
    let (section, end) = backticked_from(characters, index + 1)?;
    Some((target, section, end))
}

/// The non-empty text between the backtick at `open` and the next one, and the index past the closing backtick.
fn backticked_from(characters: &[char], open: usize) -> Option<(String, usize)> {
    let close = open
        + 1
        + characters[open + 1..]
            .iter()
            .position(|character| *character == '`')?;
    (close > open + 1).then(|| (characters[open + 1..close].iter().collect(), close + 1))
}

/// The `` §`section` `` occurrences whose `§` does not follow a backtick, left to right without overlap.
pub fn bare_references(text: &str) -> usize {
    let characters: Vec<char> = text.chars().collect();
    let mut count = 0;
    let mut index = 0;
    while index < characters.len() {
        let follows_backtick = index > 0 && characters[index - 1] == '`';
        if characters[index] == SECTION
            && !follows_backtick
            && characters.get(index + 1) == Some(&'`')
            && let Some((_, end)) = backticked_from(&characters, index + 1)
        {
            count += 1;
            index = end;
            continue;
        }
        index += 1;
    }
    count
}

/// The script's RED control, held in memory so the check stays read-only: a cited heading resolves and an absent
/// section does not.
fn self_test_verdict() -> i32 {
    let headings = [normalize(
        "## `.5.ii` measurement (`2026-06-24`) — the gate is PER-MEMBER",
    )];
    let good = headings
        .iter()
        .filter(|heading| heading.contains(&normalize(".5.ii measurement")))
        .count();
    let bad = headings
        .iter()
        .filter(|heading| heading.contains(&normalize(".5.ii LANDED")))
        .count();
    if good != 1 || bad != 0 {
        eprintln!("section-anchors: SELF-TEST FAILED (good={good} bad={bad})");
        return 1;
    }
    println!(
        "section-anchors: self-test passed (backtick/whitespace-normalized containment accepts the cited heading and refuses a section that is absent)."
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_heading_and_a_citation_normalize_alike() {
        assert_eq!(
            normalize("  `.5.ii`   measurement\t(x) "),
            ".5.ii measurement (x)"
        );
        assert_eq!(
            headings("# One\n####### seven\n##no space\n###### `Six`  deep\r\ntext"),
            ["One", "Six deep"]
        );
    }

    #[test]
    fn qualified_references_are_found_left_to_right() {
        let text = "See `docs/a.md` §`Part one` and `b.md`§`x`; not `c.mdx` §`y`, `.md` §`z` or `d.md` §``.\n\
                    `a`e.md` \u{a0}§`moved` too";
        assert_eq!(
            qualified_references(text),
            [
                ("docs/a.md".to_string(), "Part one".to_string()),
                ("b.md".to_string(), "x".to_string()),
                ("e.md".to_string(), "moved".to_string()),
            ]
        );
    }

    #[test]
    fn a_bare_reference_is_any_section_sign_not_after_a_backtick() {
        assert_eq!(
            bare_references("§`a` and `x.md` §`b`, `§`c`, §`` and §`unclosed"),
            2
        );
    }

    #[test]
    fn each_reference_resolves_or_names_its_failure() {
        let root = crate::repository_root().join(".project-data/tmp/xtask-section-anchors-scan");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("docs")).expect("fixture");
        fs::write(
            root.join("docs/target.md"),
            "# Title\n## `.5.ii` measurement (2026)\n",
        )
        .expect("target");
        fs::write(
            root.join("docs/source.md"),
            "`docs/target.md` §`.5.ii measurement` `docs/target.md` §`LANDED` `docs/gone.md` §`x` §`bare`\n",
        )
        .expect("source");
        let found = scan(
            &root,
            &["docs/source.md".to_string(), "docs/missing.md".to_string()],
        );
        fs::remove_dir_all(&root).expect("cleanup");
        assert_eq!(found.resolved, 1);
        assert_eq!(found.bare, 4);
        assert_eq!(
            found.violations,
            [
                "docs/source.md cites 'docs/target.md' §'LANDED' but no heading in that file contains it (a moved section must keep a redirect heading at the cited target)",
                "docs/source.md cites 'docs/gone.md' §'x' but that file is not readable",
            ]
        );
    }

    #[test]
    fn the_tracked_markdown_is_listed_sorted() {
        let files = tracked_markdown(&crate::repository_root());
        assert!(files.iter().any(|file| file == "README.md"));
        assert!(files.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn the_self_test_accepts_the_cited_heading_and_refuses_an_absent_section() {
        assert_eq!(self_test_verdict(), 0);
    }

    #[test]
    fn an_argument_refusal_exits_2() {
        let root = crate::repository_root();
        assert_eq!(run(&root, &["--bogus".to_string()]), 2);
        assert_eq!(run(&root, &["--root".to_string()]), 2);
        assert_eq!(
            run(&root, &["--root".to_string(), "/no/such/root".to_string()]),
            2
        );
    }
}
