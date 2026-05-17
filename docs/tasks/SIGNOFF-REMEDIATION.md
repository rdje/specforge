# SIGNOFF-REMEDIATION: Restore signoff quality at HEAD

## Metadata

- Tree ID: `SIGNOFF-REMEDIATION`
- Status: `done`
- Roadmap lane: `R0` (CI / quality-gate; cross-cutting signoff doctrine)
- Created: `2026-05-17`
- Last updated: `2026-05-17`
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

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-17` | `SIGNOFF-REMEDIATION.1` | clippy `-D warnings`, `cargo fmt --all --check`, `cargo test -p specforge --lib`, `scripts/run_ci.sh` | `passed` (1191 tests, CI exit 0) |
| `2026-05-17` | `SIGNOFF-REMEDIATION.2` | `scripts/run_ci.sh`; live-doc + TASK_TREE.md sync | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SIGNOFF-REMEDIATION.1` | `SIGNOFF-REMEDIATION.1 SIGNOFF-REMEDIATION.2 — restore signoff` | 25 clippy errors + fmt; .1 and .2 share one commit (.2 is pure closure bookkeeping; user scoped this as one slice) |
| `SIGNOFF-REMEDIATION.2` | shares commit with .1 | Tree closure |

## Changelog

- `2026-05-17`: Created task tree. Scoped from the bootstrap re-analysis that
  found HEAD failing fmt + clippy (25 errors) — non-signoff state at `main`.
- `2026-05-17`: Completed .1 (25 clippy errors cleared idiomatically + fmt
  restored, zero behavior change, 1191 tests, full CI green) and .2 (tree
  closure + live-doc sync). Tree closed.
