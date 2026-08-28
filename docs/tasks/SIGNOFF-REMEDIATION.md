# SIGNOFF-REMEDIATION: Restore signoff quality at HEAD

## Metadata

- Tree ID: `SIGNOFF-REMEDIATION`
- Status: `done` (`.1`/`.2` `2026-05-17`; `.3` `2026-08-28`)
- Roadmap lane: `R0` (CI / quality-gate; cross-cutting signoff doctrine)
- Created: `2026-05-17`
- Last updated: `2026-08-28`
- Owner: repo-local workflow

## Goal

Restore the project's non-negotiable signoff bar at `main`. HEAD (`490e6aed`)
fails `cargo fmt --all --check` and fails `cargo clippy -- -D warnings` with 25
errors, so the canonical CI entrypoint `scripts/run_ci.sh` would reject `main`.
This tree makes `scripts/run_ci.sh` green again with idiomatic fixes (no blanket
`#[allow]` suppression) and zero production behavior change.

## Non-Goals

- Do not change runtime/production behavior. Fixes are idiomatic refactors only.
- Do not suppress lints with blanket `#[allow]` attributes.
- Do not address the untracked-ISF documentation/task-tree gap here — that is
  separate follow-up work after signoff is restored.
- Do not widen scope to unrelated refactors.

## Acceptance Criteria

- All 25 `cargo clippy -p specforge --all-targets -- -D warnings` errors cleared
  idiomatically.
- `cargo fmt --all --check` is clean.
- `cargo test -p specforge --lib` still passes at the established count (>= 1191).
- `scripts/run_ci.sh` passes end to end.
- Each completed leaf committed through `COMMIT.md` with leaf-ID traceability.

## Task Tree

- ID: `SIGNOFF-REMEDIATION`
  Status: `done`
  Goal: `Restore signoff quality (fmt + clippy + CI green) at HEAD.`
  Children: `SIGNOFF-REMEDIATION.1`, `SIGNOFF-REMEDIATION.2`

- ID: `SIGNOFF-REMEDIATION.1`
  Status: `done`
  Goal: >
    Clear all 25 clippy `-D warnings` errors and restore `cargo fmt`
    cleanliness across the affected files in one cohesive signoff slice.
  Acceptance: >
    `cargo clippy -p specforge --all-targets -- -D warnings` clean,
    `cargo fmt --all --check` clean, `cargo test -p specforge --lib`
    >= 1191 passed, `scripts/run_ci.sh` green. Errors to fix:
    nlp_enrich.rs (530, 553, 672, 684);
    adapters.rs (878, 879);
    intent.rs (1288, 1316, 1317, 1318, 1391, 1392, 1446, 1447, 1649,
    1676, 1839, 1899, 1901, 2014);
    isf_ir.rs (630, 650, 676, 774, 851).
  Verification: `passed` — clippy `-D warnings` clean, `cargo fmt --all --check` clean, `cargo test -p specforge --lib` 1191 passed, `scripts/run_ci.sh` exit 0
  Commit: `SIGNOFF-REMEDIATION.1 SIGNOFF-REMEDIATION.2 — restore signoff`

- ID: `SIGNOFF-REMEDIATION.2`
  Status: `done`
  Goal: `Verify completeness and close the tree.`
  Acceptance: `scripts/run_ci.sh green; TASK_TREE.md + live docs synced; tree marked done.`
  Verification: `passed` — full `scripts/run_ci.sh` green; live docs + TASK_TREE.md synced
  Commit: `SIGNOFF-REMEDIATION.1 SIGNOFF-REMEDIATION.2 — restore signoff`

- ID: `SIGNOFF-REMEDIATION.3`
  State: `done` (`2026-08-28`)
  Goal: restore `cargo fmt --all --check` at `main` after the same condition recurred
  Acceptance: this tree's own goal statement — "HEAD fails `cargo fmt --all --check` … so the canonical
  CI entrypoint `scripts/run_ci.sh` would reject `main`" — is true again. Found `2026-08-28` while
  `SOURCE-IR-REPRODUCIBILITY.8` was checking the scope of its own formatting, and deliberately **not**
  folded into that slice, which owned an unrelated schema change. Two files fail:
  `crates/specforge/src/ir/source_to_intent_eval.rs` and
  `crates/specforge/src/test_support/trajectory_snapshot.rs`. Neither is a false positive of a local
  toolchain: `rustc --version` is **1.95.0**, exactly the version `.github/workflows/ci.yml` pins via
  `dtolnay/rust-toolchain@stable`, there is no `rustfmt.toml` and no ignore list, so it reproduces in CI.
  It is standing debt rather than a fresh regression — replaying each file's last five revisions through
  `rustfmt --check` reports **UNFORMATTED at every one**, so it has been red since at least `a3e9757d`
  (eval) and `be3b12e6` (snapshot). Restore formatting with no behavior change and no `#[rustfmt::skip]`
  suppression, and state what else still blocks the entrypoint
  Constraint: `trajectory_snapshot.rs` carries `REPLAY_PROJECTION_SHA256` and the replay digest pins
  `SOURCE-IR-REPRODUCIBILITY.2` documented as an eight-surface lockstep. A whitespace-only reflow must not
  change any pinned literal; prove that by showing the digests unchanged, not by assuming formatting is
  inert
  Prerequisite: none
  Evidence: `cargo fmt --all` restored both files; the whole workspace is now clean under
  `cargo fmt --all --check`. The constraint is discharged by measurement, not assumption. All **9**
  64-hex digest literals in `trajectory_snapshot.rs` (and the 1 in `source_to_intent_eval.rs`) are
  byte-identical before and after, compared as sorted sets. Stripping all whitespace and diffing the two
  revisions token by token leaves **exactly one opcode across the whole file: a single inserted `,`** —
  the trailing comma rustfmt adds when it splits an `assert!` across lines, which Rust treats as
  inert. `source_to_intent_eval.rs` is whitespace-only with a zero token delta. So no pinned literal,
  digest, or lockstep surface moved.
  Verified: `cargo fmt --all --check` clean workspace-wide; `cargo clippy --workspace --all-targets --
  -D warnings` clean; `cargo test --workspace` **470 / 168 / 1,371 / 4 / 5 passed, 0 failed, 9
  ignored** — including the snapshot validator that rejects any `REPLAY_PROJECTION_SHA256` mismatch, so
  the digests are re-checked by an oracle rather than only by inspection.
  Still blocking `scripts/run_ci.sh`, and deliberately not fixed here: the entrypoint runs
  `check_doctrines.sh --all` **first** under `set -euo pipefail`, and `CHAIN-CURRENCY` fails 0 current /
  24 stale. That is `SOURCE-IR-REPRODUCIBILITY.14`'s seal debt, not a formatting problem. Formatting was
  the entrypoint's *third* step and is now clear; the first is not

## Current Frontier

No executable leaves remain. Tree closed 2026-05-17.

## Decisions

- `2026-05-17`: clippy and fmt are not independently signoff-able (a
  clippy-only commit would still fail the pre-existing fmt drift and CI), so
  they are fixed together in one leaf `.1` rather than split.
- `2026-05-17`: idiomatic fixes only; no blanket `#[allow]`; zero production
  behavior change so the 1191-test baseline must hold.

## Open Questions

- None.

## Blockers

- None.

## Acceptance Checklist (enforced) — `SIGNOFF-REMEDIATION.3`

- [x] **REPRODUCE / MEASURE** — `cargo fmt --all --check` fails at `main` on exactly two files,
  `crates/specforge/src/ir/source_to_intent_eval.rs` and
  `crates/specforge/src/test_support/trajectory_snapshot.rs`. Not a local-toolchain artifact: `rustc
  --version` is **1.95.0**, the exact version `.github/workflows/ci.yml` pins, with no `rustfmt.toml`
  and no ignore list. Standing debt, not a fresh regression — replaying each file's last five revisions
  through `rustfmt --check` reports **UNFORMATTED at every one**.
- [x] **ROOT CAUSE (WHY + WHERE)** — no mechanism beyond unformatted source: rustfmt wants to wrap a
  `.map(…).unwrap_or(&[])` chain and several long `assert!`/`push(… .to_string())` calls that were
  authored past the width limit. It went unnoticed because `scripts/run_ci.sh` is the only gate that
  runs `cargo fmt --all --check`, and the CI policy moved that entrypoint to the push boundary, so no
  per-slice gate observes formatting.
- [x] **ADDRESSED (verified)** — `cargo fmt --all` applied; the workspace is clean under
  `cargo fmt --all --check`. The digest constraint is discharged by measurement: all 9 + 1 pinned 64-hex
  literals byte-identical as sorted sets, and the whitespace-stripped token diff is **one inserted
  trailing comma** for the whole of `trajectory_snapshot.rs`, zero for `source_to_intent_eval.rs`.
- [x] **NO REGRESSION** — `cargo test --workspace` **470 / 168 / 1,371 / 4 / 5 passed, 0 failed, 9
  ignored**; `cargo clippy --workspace --all-targets -- -D warnings` clean; `scripts/check_doctrines.sh`
  9/9 executed gate-tier doctrines PASS. The trajectory snapshot validator, which fails closed on any
  `REPLAY_PROJECTION_SHA256` mismatch, passes — so an oracle re-checks the pins, not just inspection.
- [x] **GENERICITY (ADR 0006)** — N/A: whitespace and one trailing comma, no rule, no vocabulary, no
  behavior.
- [x] **LOCKSTEP** — this tree reopened from `done` to `active` with `.3`; no book or product surface
  changes because no behavior changed. What still blocks `run_ci.sh` after this slice is named here and
  routed to `SOURCE-IR-REPRODUCIBILITY.14` rather than left implied.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-17` | `SIGNOFF-REMEDIATION.1` | clippy `-D warnings`, `cargo fmt --all --check`, `cargo test -p specforge --lib`, `scripts/run_ci.sh` | `passed` (1191 tests, CI exit 0) |
| `2026-05-17` | `SIGNOFF-REMEDIATION.2` | `scripts/run_ci.sh`; live-doc + TASK_TREE.md sync | `passed` |
| `2026-08-28` | `SIGNOFF-REMEDIATION.3` | `cargo fmt --all --check`; clippy `-D warnings`; `cargo test --workspace`; pinned-digest set comparison; whitespace-stripped token diff | `passed` — workspace fmt clean; 470 / 168 / 1,371 / 4 / 5 tests, 0 failed; all 10 pinned 64-hex literals byte-identical; whole-file token delta is **one inserted trailing comma**. Standing debt confirmed at 5+ revisions per file under CI's own pinned rustc 1.95.0. `run_ci.sh` still blocked earlier, at `check_doctrines.sh --all` / CHAIN-CURRENCY 0-current-24-stale (`SOURCE-IR-REPRODUCIBILITY.14`) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SIGNOFF-REMEDIATION.1` | `SIGNOFF-REMEDIATION.1 SIGNOFF-REMEDIATION.2 — restore signoff` | 25 clippy errors + fmt; .1 and .2 share one commit (.2 is pure closure bookkeeping; user scoped this as one slice) |
| `SIGNOFF-REMEDIATION.2` | shares commit with .1 | Tree closure |
| `SIGNOFF-REMEDIATION.3` | `SIGNOFF-REMEDIATION.3 — restore workspace formatting at main` | Reopened for the same condition the tree was created for; whitespace + one inert trailing comma, no pinned digest moved |

## Changelog

- `2026-05-17`: Created task tree. Scoped from the bootstrap re-analysis that
  found HEAD failing fmt + clippy (25 errors) — non-signoff state at `main`.
- `2026-05-17`: Completed .1 (25 clippy errors cleared idiomatically + fmt
  restored, zero behavior change, 1191 tests, full CI green) and .2 (tree
  closure + live-doc sync). Tree closed.
- `2026-08-28`: Reopened. The condition this tree exists for recurred — `cargo fmt --all --check`
  fails at `main` on two files, under the exact rustc CI pins, unformatted for 5+ revisions each.
  Found while `SOURCE-IR-REPRODUCIBILITY.8` scoped its own formatting and deliberately kept out of
  that slice. `.3` restores it and names what still blocks the entrypoint ahead of formatting.
