# R6-CONVERGE-HARDENING: Converge Module Test Assertion Hardening

## Metadata

- Tree ID: `R6-CONVERGE-HARDENING`
- Status: `done`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-14`
- Owner: repo-local workflow

## Goal

Add regression-only test assertions to `converge.rs` and related command-module struct fields that are populated in production but have zero test coverage.

## Non-Goals

- Do not change production behavior. This is regression-only hardening.
- Do not add test coverage to fields already tested by existing checks.
- Do not add assertions to structs with zero existing test coverage.
- Do not add assertions that require new test fixture infrastructure.

## Acceptance Criteria

- Zero-coverage converge command fields hardened with at least one non-empty assertion each.
- All tests pass after every completed leaf.
- Each leaf is committed through `COMMIT.md` with leaf-ID traceability.

## Task Tree

- ID: `R6-CONVERGE-HARDENING`
  Status: `active`
  Goal: `Harden zero-coverage converge.rs and prior_memory.rs test assertion gaps.`
  Children: `R6-CONVERGE-HARDENING.1`, `R6-CONVERGE-HARDENING.2`

### Batch 1: Converge rescan plan report

- ID: `R6-CONVERGE-HARDENING.1`
  Status: `done`
  Goal: `Harden ConvergenceRescanPlanReport.executed_validated_changed and executed_validated_no_change — two fields on a struct where 7 other fields already have assertions.`
  Acceptance: `2 assertions in converge_rebuilds_pipeline_until_snapshot_stabilizes: executed_validated_changed = 0, executed_validated_no_change = 0 (dry-run path).`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `pending — about to commit`

### Batch 2: Prior memory + SourceSnapshot

- ID: `R6-CONVERGE-HARDENING.2`
  Status: `deferred`
  Goal: `Harden prior_memory.rs CorpusMemoryUpdatePolicyRecord and PriorSourceArtifactRecord fields, plus SourceSnapshot fields — all populated in production, zero assertions.`
  Reason: `prior_memory.rs has no test module. SourceSnapshot is a private struct not exposed through the public ConvergenceReport API. Neither can be asserted without new test infrastructure.`
  Acceptance: `—`
  Verification: `—`
  Commit: `—`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
No executable leaves remain. Leaf 2 deferred (no test infrastructure for prior_memory or private SourceSnapshot).

## Decisions

- `2026-05-14`: Scoped to regression-only test assertion additions. Zero production behavior changes.
- `2026-05-14`: Prior memory fields tracked to kg_bench fixture writer for population verification.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-14` | `R6-CONVERGE-HARDENING.1` | `cargo test -p specforge --lib` | 666/666 passed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |
| `R6-CONVERGE-HARDENING.1` | `4d8197a7` Add executed_validated_changed + executed_validated_no_change assertions | 2 assertions on rescan plan report |

## Changelog

- `2026-05-14`: Created task tree with 2 hardening leaves targeting converge.rs and prior_memory.rs zero-coverage fields.
