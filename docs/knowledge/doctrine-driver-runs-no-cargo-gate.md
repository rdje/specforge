---
id: doctrine-driver-runs-no-cargo-gate
title: The doctrine enforcer runs no cargo gate, so a red clippy reads as an all-PASS tree
answers:
  - "does scripts/check_doctrines.sh run cargo clippy"
  - "does scripts/check_doctrines.sh run cargo test or cargo fmt"
  - "why did check_doctrines.sh pass while the build did not lint"
  - "what does the pre-commit hook NOT check"
  - "where are the Rust toolchain gates registered"
  - "can a task leaf tick NO REGRESSION with a clippy claim that never re-derived"
  - "which command does CI use for clippy"
  - "why is a green doctrine report not evidence that the branch builds cleanly"
date: 2026-09-11
status: current
tags: [doctrine-enforcement, ci, gates, claim-verification, signal-declaration-row-drop]
evidence: scripts/check_doctrines.sh (the DOCTRINES registry — fourteen entries, no cargo invocation; the header names fmt/clippy/test as DETERMINISTIC-ORACLE doctrines living elsewhere); scripts/run_ci.sh:30; docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.1a)
reverify: "grep -c cargo scripts/check_doctrines.sh finds only header prose; run `bash scripts/check_doctrines.sh` and `cargo clippy --offline --all-targets -- -D warnings` over the same tree and compare exit codes"
---

`scripts/check_doctrines.sh` is the general doctrine enforcer, it is what the pre-commit hook runs,
and its report ends with `doctrines: … commit/merge blocked`. It is easy to read that report as "this
tree is clean". It is not: **the registry contains no cargo invocation at all.** `cargo fmt`,
`cargo clippy` and `cargo test` are named only in the script's own header, as DETERMINISTIC-ORACLE
doctrines that live in `scripts/run_ci.sh` — `run_ci.sh:30` runs
`cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings`. The tiering is deliberate: a
pre-commit hook has to stay fast, and CI is the un-bypassable backstop (`DOCTRINE_ENFORCEMENT.md`
§4.7 tiers `CHAIN-CURRENCY` the same way, which is why it prints `DEFER`). But `clippy` does not even
print `DEFER`, because it is not a registered doctrine — so nothing in the report says it was skipped.

Measured instance. `SIGNAL-DECLARATION-ROW-DROP.1` (`48def695`) wrote
`extraction_manifest.declaration_row_accounting = …` immediately after
`ExtractionManifest::default()` — `clippy::field_reassign_with_default`, denied under `-D warnings`.
Its acceptance checklist ticked NO REGRESSION citing clippy clean. Over that exact tree,
`bash scripts/check_doctrines.sh` reports every doctrine PASS while
`cargo clippy --offline --all-targets -- -D warnings` fails to compile two targets. The leaf, the
hook, and the enforcer all agreed on a branch CI would have rejected. Repaired by `.1a`.

The general shape is the one `CLAIM_VERIFICATION.md` §2 tabulates: **a ticked acceptance box catches a
forgotten step and still permits the step being done wrong.** Between a clippy claim in a task leaf
and the next `run_ci.sh` run there is no mechanical check of that claim whatsoever — the doctrine
report is silent about it rather than negative about it, which is the more dangerous of the two. So a
leaf that cites a toolchain oracle must actually run it in the same session, and a reviewer must not
read PASS rows as coverage of anything outside the fourteen registered doctrines
(`[[corpus-canonical-currency-and-ownership]]` is the same lesson about a different ratio: read a
gate's cohort rule before treating its output as coverage).
