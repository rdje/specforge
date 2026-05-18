# FSMGEN-ISSUE-REPORTING: File the FSMGen doc-vs-strict findings via the bundle protocol

## Metadata

- Tree ID: `FSMGEN-ISSUE-REPORTING`
- Status: `done`
- Roadmap lane: `R0` (downstream contract hygiene; unblocks ISF lowering trust)
- Created: `2026-05-18`
- Last updated: `2026-05-18`
- Owner: repo-local workflow

## Goal

File the genuine FSMGen findings discovered during `ISF-TEMPORAL-LOWERING.2.1`
as reproducible issue bundles using the official protocol
`subs/fsmgen/docs/DOWNSTREAM_ISSUE_REPORTING.md`
(`bin/fsmgen-issue-bundle`), with bundles stored under SPECFORGE's own tree so
the pinned `subs/fsmgen/` submodule working tree is never modified.

Honest count: **two** genuine FSMGen findings, not three.

- F1 — `(contract n (eventually sig within N))`: spec §11.8 prints the flat
  `within N`, but `fsmgen --strict --check --json` rejects it and requires the
  nested `(eventually sig (within N))`. Doc-vs-shipped-strict contradiction.
- F2 — `(stage p (ready r)(valid v))`: spec §11.8 prints this exact shape and
  calls `ready_valid_barrier` the shipped stage kind, but `--strict --check`
  rejects it as "unsupported subclause 'ready'". Doc-vs-shipped-strict
  contradiction.
- NOT a finding — `(within 0)` rejected: the spec never documents `(within 0)`
  as supported; rejecting a zero-cycle window is defensible strictness. The
  real defect was SPECFORGE-side (about to emit `(within 0)` for
  `max_cycles==0`), caught by picky verification and already guarded in
  `ISF-TEMPORAL-LOWERING.2.2`. Per protocol §9 this is FSMGen following its
  public contract → downstream bug, fixed; no FSMGen bundle.

## Non-Goals

- Patching the `subs/fsmgen/` submodule (forbidden by standing doctrine).
- Deciding whether FSMGen fixes the checker or the spec doc; the bundle gives
  maintainers the reproduction and they triage per protocol §9.
- Re-litigating the `(within 0)` SPECFORGE-side fix (owned by
  `ISF-TEMPORAL-LOWERING.2.2`, already committed).

## Acceptance Criteria

- Two issue bundles built with `bin/fsmgen-issue-bundle`, conforming to the
  protocol's required summary (§1) and bundle layout (§2).
- Each bundle's `commands.sh` reproduces the rejection from the FSMGen
  repository root using only files inside the bundle (protocol §9 triage
  contract met: original failing command captured exactly; observed exit
  status / stdout / stderr / JSON present; expected behavior specific).
- Each bundle ships a known-good counterpart under `expected/` proving the
  accepted shape, so maintainers can distinguish a checker bug from a doc bug.
- Bundles live under SPECFORGE `docs/fsmgen-issues/`; `git status` for
  `subs/fsmgen` stays clean (submodule untouched).
- `docs/FSMGEN_FEEDBACK.md` references the filed bundle ids and records the
  evidence-backed 2-not-3 determination.
- Live docs / roadmap status updated; each leaf committed through `COMMIT.md`.

## Task Tree

- ID: `FSMGEN-ISSUE-REPORTING`
  Status: `done`
  Goal: file the two genuine FSMGen findings as protocol bundles
  Children: `FSMGEN-ISSUE-REPORTING.1`, `.2`, `.3`

- ID: `FSMGEN-ISSUE-REPORTING.1`
  Status: `done`
  Goal: build + verify the F1 bundle (`sf-isf-contract-eventually-flat`) —
  minimal `.isf` whose only deviation is the flat `(eventually sig within N)`;
  known-good nested counterpart in `expected/`
  Acceptance: helper-built bundle under `docs/fsmgen-issues/`; rerunning its
  `commands.sh` from `subs/fsmgen` reproduces strict rejection of the flat form
  and acceptance of the nested form; submodule tree clean
  Verification: `passed` — bundle built via `bin/fsmgen-issue-bundle` at
  `docs/fsmgen-issues/sf-isf-contract-eventually-flat/`; `commands.sh` rerun
  from `subs/fsmgen` reproduces `original.exit=255`, stderr `Transaction
  'txn_demo': contract 'c_demo' supports only '(eventually signal (within
  cycles))'`, empty stdout (no JSON despite `--json`);
  `expected/baseline-good.isf` strict result `success:true`; FSMGen HEAD
  `effe591d` captured; `git -C subs/fsmgen status` clean
  Commit: `see Commit Log`

- ID: `FSMGEN-ISSUE-REPORTING.2`
  Status: `done`
  Goal: build + verify the F2 bundle (`sf-isf-stage-ready-valid`) — minimal
  `.isf` whose only deviation is the spec-documented `(stage p (ready r)(valid
  v))`; `expected/` notes the spec lines that document it as shipped
  Acceptance: helper-built bundle under `docs/fsmgen-issues/`; rerunning its
  `commands.sh` reproduces strict rejection "unsupported subclause 'ready'";
  submodule tree clean
  Verification: `passed` — bundle at
  `docs/fsmgen-issues/sf-isf-stage-ready-valid/`; `commands.sh` rerun from
  `subs/fsmgen` reproduces `original.exit=255`, stderr `Transaction
  'txn_demo': stage 's_demo' has unsupported subclause 'ready'`, empty stdout
  (no JSON despite `--json`); `expected/baseline-good.isf` strict
  `success:true`; FSMGen HEAD `effe591d`; `git -C subs/fsmgen status` clean
  Commit: `see Commit Log`

- ID: `FSMGEN-ISSUE-REPORTING.3`
  Status: `done`
  Goal: reference both filed bundle ids in `docs/FSMGEN_FEEDBACK.md`, record
  the 2-not-3 determination with evidence, sync live docs, close tree
  Acceptance: feedback doc cites the bundle paths/ids and the `(within 0)`
  non-finding rationale; CHANGES/LIVE_ACHIEVEMENT/MEMORY/TASK_TREE synced; tree
  Status `done`
  Verification: `passed` — `docs/FSMGEN_FEEDBACK.md` now has the "Filed issue
  bundles (2026-05-18)" section with the F1/F2 id+path+reproduce table, the
  count-is-two evidence, and the no-JSON-on-strict-die observation;
  `docs/TASK_TREE.md` index → `done`; live docs synced
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FSMGEN-ISSUE-REPORTING.1` | `done` | F1 bundle filed + reproduction verified; helper-output-outside-submodule pattern proven |
| 2 | `FSMGEN-ISSUE-REPORTING.2` | `done` | F2 bundle filed + reproduction verified |
| 3 | `FSMGEN-ISSUE-REPORTING.3` | `done` | Bundle ids wired into `docs/FSMGEN_FEEDBACK.md`; 2-not-3 recorded; tree closed |

Tree complete. PNT returns to `ISF-TEMPORAL-LOWERING.2.3`.

## Decisions

- `2026-05-18`: Evidence-backed count is **2**, not 3. F1 (§11.8 L837–838 flat
  form) and F2 (§11.8 L827–832 stage shape) are doc-vs-shipped-strict
  contradictions → reportable per protocol §9. `(within 0)` rejection is
  defensible strictness undocumented as supported → SPECFORGE-side bug already
  fixed in `ISF-TEMPORAL-LOWERING.2.2`; no third bundle.
- `2026-05-18`: Bundles stored under SPECFORGE `docs/fsmgen-issues/<id>/` and
  built with `--bundle-dir` pointing there while running the helper from
  `subs/fsmgen` (so `git rev-parse HEAD` captures the correct FSMGen submodule
  revision). The submodule working tree is never written to.

## Open Questions

- None blocking the frontier.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | `FSMGEN-ISSUE-REPORTING.1` | `bin/fsmgen-issue-bundle` build + `commands.sh` rerun from `subs/fsmgen` + baseline-good strict-pass + submodule-clean check | `passed` (orig exit 255; stderr names contract; baseline `success:true`; FSMGen HEAD `effe591d`; `subs/fsmgen` clean) |
| `2026-05-18` | `FSMGEN-ISSUE-REPORTING.2` | same protocol checks for the F2 stage bundle | `passed` (orig exit 255; stderr `stage 's_demo' has unsupported subclause 'ready'`; baseline `success:true`; FSMGen HEAD `effe591d`; `subs/fsmgen` clean) |
| `2026-05-18` | `FSMGEN-ISSUE-REPORTING.3` | feedback-doc cross-reference + index/live-doc sync review | `passed` (FSMGEN_FEEDBACK.md cites both bundle ids/paths + reproduce + 2-not-3 evidence; TASK_TREE.md → done) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-ISSUE-REPORTING` (tree) | `FSMGEN-ISSUE-REPORTING — track filing the genuine FSMGen findings` (`1c3a5d37`) | tree + index |
| `FSMGEN-ISSUE-REPORTING.1` | `FSMGEN-ISSUE-REPORTING.1 — file F1 bundle (contract eventually flat-vs-nested)` (`1b028453`) | bundle under `docs/fsmgen-issues/sf-isf-contract-eventually-flat/`; submodule untouched |
| `FSMGEN-ISSUE-REPORTING.2` | `FSMGEN-ISSUE-REPORTING.2 — file F2 bundle (stage ready/valid strict-rejected)` (`c415fc25`) | bundle under `docs/fsmgen-issues/sf-isf-stage-ready-valid/`; submodule untouched |
| `FSMGEN-ISSUE-REPORTING.3` | `FSMGEN-ISSUE-REPORTING.3 — wire bundle ids into FSMGEN_FEEDBACK.md; close tree` | tree closed; PNT resumes `ISF-TEMPORAL-LOWERING.2.3` |

## Changelog

- `2026-05-18`: Created task tree; goal = file the two genuine FSMGen
  doc-vs-strict findings (F1 contract-eventually-flat, F2 stage-ready-valid)
  via the official bundle protocol, submodule untouched. Recorded the
  evidence-backed 2-not-3 determination.
- `2026-05-18`: `.1` done — F1 bundle
  `docs/fsmgen-issues/sf-isf-contract-eventually-flat/` built with
  `bin/fsmgen-issue-bundle`, derived from a real strict-passing emitted `.isf`
  reduced to a one-line flat-vs-nested deviation; `commands.sh` rerun
  reproduces exit 255 + the contract diagnostic; `expected/` carries the
  strict-passing nested counterpart; `subs/fsmgen` verified clean.
- `2026-05-18`: `.2` done — F2 bundle
  `docs/fsmgen-issues/sf-isf-stage-ready-valid/` built the same way; the only
  deviation from the strict-accepted baseline is the spec §11.8
  `(stage … (ready)(valid))` clause; `commands.sh` rerun reproduces exit 255 +
  `stage 's_demo' has unsupported subclause 'ready'`; `subs/fsmgen` clean.
- `2026-05-18`: `.3` done — `docs/FSMGEN_FEEDBACK.md` gained the "Filed issue
  bundles" section: F1/F2 id+path+reproduce table, "where FSMGEN can see each
  report" (committed/pushed bundles + this feedback doc as the stable
  channel), the count-is-two evidence, and the no-JSON-on-strict-die
  observation. `docs/TASK_TREE.md` index → `done`. Tree closed; PNT resumes
  `ISF-TEMPORAL-LOWERING.2.3`.
