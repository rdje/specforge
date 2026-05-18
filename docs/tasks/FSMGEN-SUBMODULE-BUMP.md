# FSMGEN-SUBMODULE-BUMP: pin to upstream that fixed both reported findings

## Metadata

- Tree ID: `FSMGEN-SUBMODULE-BUMP`
- Status: `active`
- Roadmap lane: `R0` (downstream contract hygiene; consumes the
  `FSMGEN-ISSUE-REPORTING` outcome)
- Created: `2026-05-18`
- Last updated: `2026-05-18`
- Owner: repo-local workflow

## Goal

Update the pinned `subs/fsmgen` submodule from `effe591d` to upstream
`origin/main` HEAD `9bfb9a20`, which carries the explicit fixes for the two
SPECFORGE-filed findings:

- `610cb26e …STAGE-CONTRACT-BUGS.1: accept flat eventual contracts` → F1
- `d4d6dfab …STAGE-CONTRACT-BUGS.2: accept ready-valid stages` → F2
- `9bfb9a20 …STAGE-CONTRACT-BUGS.3: emit ISF check JSON failures` → the
  secondary "strict reject exits 255 with no JSON despite `--json`" surface
- `a60cc1ab …STAGE-CONTRACT-BUGS: track reproduced reports`

Bumping the pin is the sanctioned action (it moves the pin to an upstream
commit; the submodule working tree stays clean) — this is NOT patching the
submodule.

## Non-Goals

- Patching `subs/fsmgen` working tree (forbidden).
- Re-enabling `(stage …)` for `HandshakeComplete` temporal_rules just because
  F2 is fixed. That reverses `ISF-TEMPORAL-LOWERING` `.2.1`/`.2.3` decision #2
  and is a behavior change requiring its own verification + ownership. It is
  surfaced here as an explicit follow-up, NOT done in this tree.
- Switching SPECFORGE's emitted contract form from nested `(within N)` to the
  now-also-accepted flat form. SPECFORGE already emits the strict-valid nested
  form; no code change is warranted by F1's fix (only doc reconciliation).

## Acceptance Criteria

- `subs/fsmgen` pin == upstream `origin/main` `9bfb9a20`; submodule working
  tree clean; new binary runs (`--strict --check --json`, capability
  manifest smoke).
- The user's claim is **empirically audited**: the F1 and F2 bundle repro
  inputs now pass `--strict --check --json` on the NEW binary (and the
  nested `(within N)` form SPECFORGE actually emits still passes).
- Full `scripts/run_ci.sh` green against the new binary; any fsmgen-behavior
  fallout fixed (the pin is the pipeline's strict ground truth — high blast
  radius).
- `docs/FSMGEN_FEEDBACK.md`, the two issue bundles, and the
  `FSMGEN-ISSUE-REPORTING` tree are reconciled to "resolved upstream"; the
  `(stage …)` re-enablement opportunity is recorded as an explicit scoped
  follow-up.
- Live docs / roadmap synced; each leaf committed via `COMMIT.md`.

## Task Tree

- ID: `FSMGEN-SUBMODULE-BUMP`
  Status: `active`
  Goal: pin to `9bfb9a20`, verify fixes, reconcile, no regression
  Children: `.1`, `.2`, `.3`, `.4`

- ID: `FSMGEN-SUBMODULE-BUMP.1`
  Status: `done`
  Goal: bump the pin `effe591d → 9bfb9a20`; smoke the new binary; then
  **empirically audit the user's claim** — F1 (`sf-isf-contract-eventually-flat`)
  and F2 (`sf-isf-stage-ready-valid`) findings are fixed on the new binary,
  and the nested `(within N)` form SPECFORGE emits still passes
  Acceptance: pin == `9bfb9a20`; submodule clean; both reported findings
  verified fixed on the new binary; nested-contract baseline still
  `success:true`
  Verification: `passed` (audited, not assumed) — pin `effe591d →
  9bfb9a20`, `git -C subs/fsmgen status` CLEAN, `--capability-manifest`
  exit 0. **F1 fixed**: F1 bundle input `success:true`,
  `diagnostic_count:0`, and now emits JSON (was exit 255 / 0 bytes — the
  `.3` JSON-on-strict-reject fix also confirmed). **F2 finding fixed**:
  an isolated `(transaction … (stage s1 (ready REQ)(valid ACK)) …)` is
  `success:true` on the new binary (was "unsupported subclause 'ready'").
  Nested `(eventually s (within N))` baseline SPECFORGE emits still
  `success:true` (no regression). NUANCE recorded honestly: the F2
  *bundle* input still returns `success:false`, but with a NEW unrelated
  diagnostic `isf_priority_mixed_timing_conflict on ADDRESS` — the
  minimized repro injected the stage onto a corpus head that already
  rule-drives `ADDRESS`, so now that stages are processed FSMGen
  correctly flags that artifact's self-conflict. This is correct FSMGen
  behavior on a self-conflicting minimization artifact, NOT a remaining
  stage bug; `.3` refreshes the F2 bundle's resolution note accordingly.
  Commit: `see Commit Log`

- ID: `FSMGEN-SUBMODULE-BUMP.2`
  Status: `pending`
  Goal: full `scripts/run_ci.sh` against the new binary (esp. the 4
  fsmgen-binary tests via `run_fsmgen_strict_check`); diagnose and fix any
  behavior fallout from the pinned binary changing
  Acceptance: full CI green on the new pin; any fallout fixed with root cause
  recorded (not suppressed)
  Verification: `pending`
  Commit: `pending`

- ID: `FSMGEN-SUBMODULE-BUMP.3`
  Status: `pending`
  Goal: reconcile docs — `docs/FSMGEN_FEEDBACK.md` findings → RESOLVED (cite
  fixing commits); both issue bundles → resolution note; `FSMGEN-ISSUE-REPORTING`
  tree → resolution recorded; `ISF-TEMPORAL-LOWERING` decision cross-refs
  noted; the `(stage …)`-for-HandshakeComplete re-enablement recorded as an
  explicit scoped follow-up (own decision/tree, not done here)
  Acceptance: docs state the findings are fixed upstream and what (if
  anything) SPECFORGE should now change; follow-up explicitly scoped
  Verification: `pending`
  Commit: `pending`

- ID: `FSMGEN-SUBMODULE-BUMP.4`
  Status: `pending`
  Goal: close tree; sync live docs (CHANGES/LIVE/MEMORY/TASK_TREE)
  Acceptance: tree `done`; index updated; CI green
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FSMGEN-SUBMODULE-BUMP.1` | `done` | Pinned `9bfb9a20`; both findings audited fixed (F2-bundle conflict nuance recorded) |
| 2 | `FSMGEN-SUBMODULE-BUMP.2` | `pending` | Next — full CI regression on the new ground-truth binary |
| 3 | `FSMGEN-SUBMODULE-BUMP.3` | `pending` | Reconcile the reporting/feedback paper trail |
| 4 | `FSMGEN-SUBMODULE-BUMP.4` | `pending` | Close + sync |

## Decisions

- `2026-05-18`: Bump target is upstream `origin/main` HEAD `9bfb9a20`
  (not an intermediate fix commit) — take the whole released line, since
  `.3` (`9bfb9a20`) also fixes the JSON-on-strict-reject surface SPECFORGE
  observed. Verify empirically; do not trust the commit subjects alone.
- `2026-05-18`: F2 being fixed (`(stage …)` now accepted) does NOT
  auto-re-enable `(stage …)` lowering for `HandshakeComplete`
  temporal_rules. That is a deliberate behavior change reversing
  `ISF-TEMPORAL-LOWERING.2.1/.2.3` decision #2; it needs its own
  verification + task-tree ownership. Recorded as a scoped follow-up in
  `.3`, not executed here (scope discipline + `fsmgen-contract-authority`:
  re-confirm against the new binary before lowering to a previously-rejected
  construct).
- `2026-05-18` (`.1` audited finding — picky verification paid off): the
  commit subject `STAGE-CONTRACT-BUGS.2: accept ready-valid stages` is
  TRUE (isolated `(stage …)` now `success:true`), but the F2 *bundle*
  input still fails — with a NEW, correct `isf_priority_mixed_timing_conflict`
  because the minimized repro reused corpus signal `ADDRESS` for both a
  rule and the stage `valid`. Lesson: the reported bug is fixed; the
  bundle artifact has an unrelated self-conflict from minimization. Do
  not report "F2 still broken" (false) nor "F2 bundle passes" (false) —
  the precise truth is recorded. `.3` will annotate the F2 bundle with
  this resolution rather than claim a clean pass.

## Open Questions

- Does any form SPECFORGE currently emits behave differently under
  `9bfb9a20`? `.2` answers this via full CI on the new pin.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | `FSMGEN-SUBMODULE-BUMP.1` | pin bump + binary smoke + F1/F2/nested empirical audit on new binary | `passed` (F1 fixed+JSON; F2 stage construct fixed isolated; F2-bundle self-conflict nuance recorded; nested baseline no regression) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-SUBMODULE-BUMP` (tree) | `FSMGEN-SUBMODULE-BUMP — track pin to upstream that fixed both findings` (`d3f63f9c`) | tree + index |
| `FSMGEN-SUBMODULE-BUMP.1` | `FSMGEN-SUBMODULE-BUMP.1 — pin subs/fsmgen 9bfb9a20; audit both findings fixed` | submodule pin bump; F1✓ F2-construct✓ |

## Changelog

- `2026-05-18`: Created — user reports FSMGen addressed both filed findings;
  bump `subs/fsmgen` `effe591d → 9bfb9a20`, empirically audit the fix, run
  full regression on the new ground-truth binary, reconcile the reporting
  paper trail, and scope (not auto-do) the `(stage …)` re-enablement.
