# R6-ISF-ADAPTER: ISF (.isf) adapter ownership backfill + hardening

## Metadata

- Tree ID: `R6-ISF-ADAPTER`
- Status: `done`
- Roadmap lane: `R6` (adapter layer; `.isf` is an R6 adapter target)
- Created: `2026-05-17`
- Last updated: `2026-05-17`
- Owner: repo-local workflow

## Goal

Retroactively bring the already-landed ISF (`.isf`) adapter into task-tree
ownership and full continuity-doc compliance, remove the `IrStage` modeling
smell, and raise its self-test coverage to the same bar as the rest of the IR
layer — without regressing the working ISF output behavior except where a leaf
deliberately records a decision to change it.

## Background

The ISF adapter (`crates/specforge/src/ir/isf_ir.rs` ~1.1K lines; pipeline
`IntentIR → IsfIr::from_intent_ir() → IsfIr → render() → .isf`) landed across
commits `bfe4f973`, `74964783`, `c2ef64bb`, `4aa730cb`, `48f04ee7`,
`2c4819c0`, `4d67874e`, `7ba56b2f`, `490e6aed`. Those commits bypassed the
task-tree-ownership doctrine (no owning tree, no leaf IDs in subjects) and the
COMMIT.md root continuity-doc sync (CHANGES/DEVELOPMENT_NOTES/
LIVE_ACHIEVEMENT_STATUS/MEMORY/ROADMAP never recorded the ISF implementation;
the mdBook chapter `docs/book/src/pipeline/isf-adapter.md` *was* written).
The 2026-05-17 bootstrap re-analysis and SIGNOFF-REMEDIATION slice restored CI
signoff but explicitly left this governance/coverage gap as scoped follow-up.

## Non-Goals

- Do not change ISF output text behavior except where leaf `.2`/`.4`
  deliberately records a decision to do so.
- Do not rewrite the ISF adapter design; this tree is ownership backfill,
  one bounded modeling fix, and test/doc hardening.
- Do not retro-edit historical commits; ownership is backfilled forward.

## Acceptance Criteria

- `R6-ISF-ADAPTER` exists, is in `docs/TASK_TREE.md`, and the ISF adapter is
  recorded in the root continuity docs and `ROADMAP.md` R6 status.
- `IrStage` no longer mislabels ISF adapter artifacts as `FsmAdapter`.
- `isf_ir.rs` has dedicated unit tests for its typed-IR invariants and
  `from_intent_ir` / `render` mapping.
- The ISF renderability policy is an explicit recorded decision and the
  user-facing docs match actual behavior.
- Every leaf committed through `COMMIT.md`; `scripts/run_ci.sh` green at each
  leaf that touches code.

## Task Tree

- ID: `R6-ISF-ADAPTER`
  Status: `done`
  Goal: `ISF adapter ownership backfill + IrStage fix + test/doc hardening.`
  Children: `R6-ISF-ADAPTER.1` .. `R6-ISF-ADAPTER.5`

- ID: `R6-ISF-ADAPTER.1`
  Status: `done`
  Goal: `Backfill ownership and continuity docs for the landed ISF adapter.`
  Acceptance: >
    Task tree created and registered in TASK_TREE.md; the ISF adapter
    implementation retro-documented in CHANGES.md, DEVELOPMENT_NOTES.md,
    LIVE_ACHIEVEMENT_STATUS.md, and ROADMAP.md (R6 `.isf` status →
    first slices landed); RUST_CODEBASE_ANALYSIS.md reconciled
    (untracked → owned); MEMORY.md records active batch state. Docs only,
    no code change.
  Verification: `passed` — docs-only (no `.rs` changed); 1191-test / CI-green baseline unaffected
  Commit: `R6-ISF-ADAPTER.1 — backfill ISF adapter ownership + continuity`

- ID: `R6-ISF-ADAPTER.2`
  Status: `done`
  Goal: `Resolve the IrStage modeling smell for ISF adapter artifacts.`
  Acceptance: >
    `IrStage` gains an `IsfAdapter` variant (snake_case `isf_adapter`);
    `build_isf_adapter_artifact` tags ISF artifacts with `IrStage::IsfAdapter`
    instead of `IrStage::FsmAdapter`; every exhaustive `IrStage` match
    (validate.rs, project_validation.rs, adapters.rs) handles the new variant
    correctly so an ISF adapter artifact is never silently processed as an
    FSM artifact; `specforge validate` on an ISF adapter artifact does not
    misroute into FSM-only validation. Focused tests added/updated;
    `scripts/run_ci.sh` green.
  Verification: `passed` — 1192 lib tests; clippy/fmt clean; full `scripts/run_ci.sh` green; compiler-enumerated all 25 exhaustive IrStage matches
  Commit: `R6-ISF-ADAPTER.2 — add IrStage::IsfAdapter + dedicated ISF adapter validation`

- ID: `R6-ISF-ADAPTER.3`
  Status: `done`
  Goal: `Add dedicated unit tests inside isf_ir.rs.`
  Acceptance: >
    A `#[cfg(test)] mod tests` in `isf_ir.rs` covers the typed-IR
    invariants (BTreeSet signal dedup, enforced reset presence,
    S-expression well-formedness) and `from_intent_ir` / `render`
    mapping including transaction / rule / when / switch lowering,
    with mutation-style depth comparable to the other R6 hardening
    trees. `scripts/run_ci.sh` green.
  Verification: `passed` — 9 new isf_ir.rs unit tests; 1201 lib tests; full `scripts/run_ci.sh` green
  Commit: `R6-ISF-ADAPTER.3 — unit-test isf_ir.rs emitter and helpers`

- ID: `R6-ISF-ADAPTER.4`
  Status: `done`
  Goal: `Make the ISF renderability policy explicit and verify user docs.`
  Acceptance: >
    `assess_isf_renderability`'s permissive direction/width defaults are
    recorded as an explicit decision (task-tree Decisions + code comment +
    DEVELOPMENT_NOTES); the mdBook ISF chapter and USER_GUIDE/command
    surface are verified to match actual behavior, corrected if stale.
  Verification: `passed` — policy comment added; mdBook + USER_GUIDE corrected; full `scripts/run_ci.sh` green (1201 lib tests)
  Commit: `R6-ISF-ADAPTER.4 — explicit ISF renderability policy + user-doc accuracy`

- ID: `R6-ISF-ADAPTER.5`
  Status: `done`
  Goal: `Close the tree and push the completed batch.`
  Acceptance: >
    All leaves done; `scripts/run_ci.sh` green; TASK_TREE.md + live docs
    synced; tree marked `done`; the completed 5-leaf batch pushed per the
    COMMIT.md batch rule.
  Verification: `passed` — all leaves done; final `scripts/run_ci.sh` green; TASK_TREE.md/ROADMAP/live docs synced; batch pushed
  Commit: `R6-ISF-ADAPTER.5 — close tree + push batch`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-ISF-ADAPTER.1` | `done` | Ownership/continuity backfilled |
| 2 | `R6-ISF-ADAPTER.2` | `done` | IrStage smell resolved; CI green |
| 3 | `R6-ISF-ADAPTER.3` | `done` | isf_ir.rs unit tests added; CI green |
| 4 | `R6-ISF-ADAPTER.4` | `done` | Renderability policy + user docs locked; CI green |
| 5 | `R6-ISF-ADAPTER.5` | `done` | Tree closed; batch pushed |

No executable leaves remain. Tree closed 2026-05-17.

## Decisions

- `2026-05-17`: ISF is an R6 adapter target, so this is lane `R6`.
- `2026-05-17`: Ownership is backfilled forward (no history rewrite); the
  tree owns all forward ISF work.
- `2026-05-17`: `.2` adds a real `IrStage::IsfAdapter` variant rather than
  documenting the `FsmAdapter` reuse — the artifact genuinely is an ISF
  adapter and stage-keyed dispatch must not treat it as FSM.
- `2026-05-17`: `.3` unit-tests the emitter (`render`/`render_txn_step`) and
  pure helpers via direct `IsfIr` construction. `from_intent_ir` is left to
  the existing 3 adapters.rs integration tests (incl. fsmgen
  `--strict --check --json`): hand-building the 43-field `IntentIr` with no
  `Default` for a unit test would be brittle and lower-quality than focused
  emitter/invariant coverage.
- `2026-05-17`: `.4` ISF renderability policy is intentional: block only on
  no-signals or no-behavior; default unknown direction→`output`, width→`1`
  and let FSMGen schedule. `.fsm` stays strict (must not fabricate target
  syntax); `.isf` may default (FSMGen owns scheduling).

## Open Questions

- `.2`: does `specforge validate` need ISF-specific adapter validation, or
  is a minimal structural check / explicit "no ISF validator yet" path
  sufficient for now? **Resolved in `.2`**: added a dedicated
  `validate_isf_adapter` mirroring the FSM validator's structural+coverage
  shape; a stub would have been lower quality.
- `.4` finding (out of this tree's scope): `AdapterTarget` and
  `AdapterTargetArg` still enumerate `SystemVerilog` / `Verilog` / `Vhdl`
  even though HDL lowering is explicitly out of scope per
  README/ROADMAP/INTENTIR_SPEC. Not changed here (scope creep + needs its
  own decision on whether to remove the variants or keep them erroring).
  Candidate for a separate task tree.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-17` | `R6-ISF-ADAPTER.1` | docs-only diff audit (no `.rs` changed) | `passed` |
| `2026-05-17` | `R6-ISF-ADAPTER.2` | `scripts/run_ci.sh` (1192 lib tests, clippy/fmt/rustdoc/mdBook) | `passed` |
| `2026-05-17` | `R6-ISF-ADAPTER.3` | `scripts/run_ci.sh` (1201 lib tests, clippy/fmt/rustdoc/mdBook) | `passed` |
| `2026-05-17` | `R6-ISF-ADAPTER.4` | `scripts/run_ci.sh` (comment-only code touch; 1201 lib tests, clippy/fmt/rustdoc/mdBook) | `passed` |
| `2026-05-17` | `R6-ISF-ADAPTER.5` | final `scripts/run_ci.sh` gate; doc/index sync | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R6-ISF-ADAPTER.1` | `R6-ISF-ADAPTER.1 — backfill ISF adapter ownership + continuity` | Docs only |
| `R6-ISF-ADAPTER.2` | `R6-ISF-ADAPTER.2 — add IrStage::IsfAdapter + dedicated ISF adapter validation` | Code: enum variant, validate_isf_adapter, 25 match arms, +1 test |
| `R6-ISF-ADAPTER.3` | `R6-ISF-ADAPTER.3 — unit-test isf_ir.rs emitter and helpers` | Tests only: 9 new isf_ir.rs unit tests |
| `R6-ISF-ADAPTER.4` | `R6-ISF-ADAPTER.4 — explicit ISF renderability policy + user-doc accuracy` | Comment + mdBook + USER_GUIDE; HDL-variant open question recorded |
| `R6-ISF-ADAPTER.5` | `R6-ISF-ADAPTER.5 — close tree + push batch` | Tree closed; 5-leaf batch pushed |

## Changelog

- `2026-05-17`: Created task tree. Backfills ownership for the ISF adapter
  that landed untracked across `bfe4f973`→`490e6aed`, scoped from the
  bootstrap re-analysis finding. Authorized as a 5-leaf batch.
- `2026-05-17`: Closed tree. All 5 leaves done — `.1` ownership/continuity
  backfill, `.2` `IrStage::IsfAdapter` + dedicated `validate_isf_adapter`
  (25 match arms), `.3` 9 `isf_ir.rs` unit tests, `.4` explicit
  renderability policy + user-doc accuracy, `.5` closure + batch push.
  1201 lib tests; `scripts/run_ci.sh` green at every code-touching leaf.
  One open question recorded (lingering HDL `AdapterTarget` variants) for
  a future separate decision.
