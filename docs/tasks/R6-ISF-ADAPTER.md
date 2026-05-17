# R6-ISF-ADAPTER: ISF (.isf) adapter ownership backfill + hardening

## Metadata

- Tree ID: `R6-ISF-ADAPTER`
- Status: `active`
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
  Status: `active`
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
  Status: `pending`
  Goal: `Add dedicated unit tests inside isf_ir.rs.`
  Acceptance: >
    A `#[cfg(test)] mod tests` in `isf_ir.rs` covers the typed-IR
    invariants (BTreeSet signal dedup, enforced reset presence,
    S-expression well-formedness) and `from_intent_ir` / `render`
    mapping including transaction / rule / when / switch lowering,
    with mutation-style depth comparable to the other R6 hardening
    trees. `scripts/run_ci.sh` green.
  Verification: `pending`
  Commit: `pending`

- ID: `R6-ISF-ADAPTER.4`
  Status: `pending`
  Goal: `Make the ISF renderability policy explicit and verify user docs.`
  Acceptance: >
    `assess_isf_renderability`'s permissive direction/width defaults are
    recorded as an explicit decision (task-tree Decisions + code comment +
    DEVELOPMENT_NOTES); the mdBook ISF chapter and USER_GUIDE/command
    surface are verified to match actual behavior, corrected if stale.
  Verification: `pending`
  Commit: `pending`

- ID: `R6-ISF-ADAPTER.5`
  Status: `pending`
  Goal: `Close the tree and push the completed batch.`
  Acceptance: >
    All leaves done; `scripts/run_ci.sh` green; TASK_TREE.md + live docs
    synced; tree marked `done`; the completed 5-leaf batch pushed per the
    COMMIT.md batch rule.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-ISF-ADAPTER.1` | `done` | Ownership/continuity backfilled |
| 2 | `R6-ISF-ADAPTER.2` | `done` | IrStage smell resolved; CI green |
| 3 | `R6-ISF-ADAPTER.3` | `pending` | Next — harden once the type model is correct |
| 4 | `R6-ISF-ADAPTER.4` | `pending` | Lock the policy + user docs last |
| 5 | `R6-ISF-ADAPTER.5` | `pending` | Close + push the batch |

## Decisions

- `2026-05-17`: ISF is an R6 adapter target, so this is lane `R6`.
- `2026-05-17`: Ownership is backfilled forward (no history rewrite); the
  tree owns all forward ISF work.
- `2026-05-17`: `.2` adds a real `IrStage::IsfAdapter` variant rather than
  documenting the `FsmAdapter` reuse — the artifact genuinely is an ISF
  adapter and stage-keyed dispatch must not treat it as FSM.

## Open Questions

- `.2`: does `specforge validate` need ISF-specific adapter validation, or
  is a minimal structural check / explicit "no ISF validator yet" path
  sufficient for now? Resolve during `.2` from the actual validate.rs path.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-17` | `R6-ISF-ADAPTER.1` | docs-only diff audit (no `.rs` changed) | `passed` |
| `2026-05-17` | `R6-ISF-ADAPTER.2` | `scripts/run_ci.sh` (1192 lib tests, clippy/fmt/rustdoc/mdBook) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R6-ISF-ADAPTER.1` | `R6-ISF-ADAPTER.1 — backfill ISF adapter ownership + continuity` | Docs only |
| `R6-ISF-ADAPTER.2` | `R6-ISF-ADAPTER.2 — add IrStage::IsfAdapter + dedicated ISF adapter validation` | Code: enum variant, validate_isf_adapter, 25 match arms, +1 test |

## Changelog

- `2026-05-17`: Created task tree. Backfills ownership for the ISF adapter
  that landed untracked across `bfe4f973`→`490e6aed`, scoped from the
  bootstrap re-analysis finding. Authorized as a 5-leaf batch.
