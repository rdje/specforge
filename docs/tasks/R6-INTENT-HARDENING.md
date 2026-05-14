# R6-INTENT-HARDENING: Intent Module Test Assertion Hardening

## Metadata

- Tree ID: `R6-INTENT-HARDENING`
- Status: `active`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-14`
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
  Status: `active`
  Goal: `Fix 10 missed cargo-mutants in intent.rs builder functions.`
  Children: `R6-INTENT-HARDENING.1`, `R6-INTENT-HARDENING.2`

### Batch 1: overlaps unit tests + dedup + build_assumptions

- ID: `R6-INTENT-HARDENING.1`
  Status: `pending`
  Goal: `Add unit tests for overlaps() function + dedup behavior in build_behaviors/build_constraints + missing build_assumptions packet_ids.`
  Acceptance: `overlaps tests catch 3 mutants (true/false/||→&&), dedup test catches 4 delete-! mutants, build_assumptions test catches 2 ==→!= mutants. 10/10 caught after fix.`
  Verification: `pending`
  Commit: `pending`

### Batch 2: build_intent_actors overlaps || → &&

- ID: `R6-INTENT-HARDENING.2`
  Status: `pending`
  Goal: `Add test where actor overlaps with phase via statements but not sections, catching ||→&& mutant at line 477.`
  Acceptance: `1 mutant caught: line 477 ||→&&.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-INTENT-HARDENING.1` | `pending` | Bulk of missed mutants (9/10) in overlaps, dedup, and assumptions |
| 2 | `R6-INTENT-HARDENING.2` | `pending` | Last remaining mutant in build_intent_actors |

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
