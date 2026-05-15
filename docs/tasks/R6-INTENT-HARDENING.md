# R6-INTENT-HARDENING: Intent Module Test Assertion Hardening

## Metadata

- Tree ID: `R6-INTENT-HARDENING`
- Status: `done`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-15`
- Owner: repo-local workflow

## Goal

Fix missed cargo-mutants in intent.rs builder functions — 10 missed mutants across `build_intent_actors`, `build_behaviors`, `build_constraints`, `build_assumptions`, and `overlaps`.

## Non-Goals

- Do not change production behavior. This is regression-only hardening.
- Do not refactor production code to make it more testable.

## Acceptance Criteria

- All 10 missed mutants caught (verified via `cargo mutants -p specforge -f intent.rs`).
- All tests pass after every completed leaf.
- Each leaf committed through `COMMIT.md` with leaf-ID traceability.

## Task Tree

- ID: `R6-INTENT-HARDENING`
  Status: `done`
  Goal: `Fix 10 missed cargo-mutants in intent.rs builder functions.`
  Children: `R6-INTENT-HARDENING.1`, `R6-INTENT-HARDENING.2`

### Batch 1: overlaps unit tests + dedup + build_assumptions + build_intent_actors

- ID: `R6-INTENT-HARDENING.1`
  Status: `done`
  Goal: `Add unit tests for overlaps() function + dedup behavior in build_behaviors/build_constraints + missing build_assumptions packet_ids + build_intent_actors statement-vs-section overlap.`
  Acceptance: `31 caught, 9 unviable, 1 false positive (||→&& in empty-check is equivalent). Both batches committed together.`
  Verification: `cargo test -p specforge --lib — 680/680 passed`
  Commit: `2dc9562f`

- ID: `R6-INTENT-HARDENING.2`
  Status: `superseded`
  Goal: `Merged into .1 — build_intent_actors ||→&& was fixed in same commit.`

## Current Frontier

No executable leaves remain. Tree closed 2026-05-14 (work completed), task file backfilled 2026-05-15.

## Decisions

- `2026-05-14`: Use `IntentContext`-direct construction pattern (like `preserves_semantic_residual_decisions_in_intent_ir`) for targeted tests.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |

## Changelog

- `2026-05-14`: Created task tree with 2 hardening leaves fixing 10 missed cargo-mutants in intent.rs.
