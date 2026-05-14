# R6-SOURCE-HARDENING: SourceIR Field Test Assertion Hardening

## Metadata

- Tree ID: `R6-SOURCE-HARDENING`
- Status: `done`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-14`
- Owner: repo-local workflow

## Goal

Add regression-only test assertions to `source.rs` struct fields that are populated in production but have zero test coverage. SourceIR is the entry point for all SpecForge pipelines — untested fields here are a regression risk for document ingestion and adapter targeting.

## Non-Goals

- Do not change production behavior. This is regression-only hardening.
- Do not add test coverage to fields already tested by existing checks.
- Do not add assertions to structs with zero existing test coverage.
- Do not add assertions that require new test fixture infrastructure.

## Acceptance Criteria

- 6 zero-coverage SourceIR fields hardened with at least one non-empty assertion each.
- All tests pass after every completed leaf.
- Each leaf is committed through `COMMIT.md` with leaf-ID traceability.

## Task Tree

- ID: `R6-SOURCE-HARDENING`
  Status: `active`
  Goal: `Harden zero-coverage SourceIR struct fields with regression test assertions.`
  Children: `R6-SOURCE-HARDENING.1`, `R6-SOURCE-HARDENING.2`, `R6-SOURCE-HARDENING.3`

### Batch 1: SourceIr top-level + metadata fields

- ID: `R6-SOURCE-HARDENING.1`
  Status: `done`
  Goal: `Harden SourceIr.adapter_targets and DocumentIdentity.origin_kind — top-level SourceIR fields with zero assertions.`
  Acceptance: `4 assertions across 2 tests: markdown_source_ir_reuses_existing_markdown (adapter_targets non-empty + contains Fsm, origin_kind = Markdown) and pdf_source_ir_plans_conversion_outputs (adapter_targets non-empty, origin_kind = Pdf).`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `pending — about to commit`

- ID: `R6-SOURCE-HARDENING.2`
  Status: `done`
  Goal: `Harden SourceRegistration.size_bytes and NormalizationPlan.notes — metadata fields populated during SourceIR::build and normalization.`
  Acceptance: `4 assertions in pdf_source_ir_materialization_uses_backend_helper_and_writes_manifests: size_bytes Some + >0, notes non-empty + contains "docling materialized".`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `pending — about to commit`

### Batch 2: Visual asset / page geometry fields

- ID: `R6-SOURCE-HARDENING.3`
  Status: `done`
  Goal: `Harden VisualAsset.diagram_kind and PageArtifact.width_px/height_px — figure classification and page geometry fields.`
  Acceptance: `3 assertions in pdf_source_ir_materialization test: width_px = Some(800), height_px = Some(600), diagram_kind = Unknown (stub default).`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `pending — about to commit`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
No executable leaves remain. All 3 R6-SOURCE-HARDENING leaves complete.

## Decisions

- `2026-05-14`: Scoped to regression-only test assertion additions. Zero production behavior changes.
- `2026-05-14`: Split into 3 leaves by field family — top-level identity/metadata fields, build/normalization metadata, and visual asset geometry.
- `2026-05-14`: `adapter_targets` is the highest-priority gap given the R6 adapter focus — SourceIR carries the adapter target list but no test ever checks it.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-14` | `R6-SOURCE-HARDENING.1` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `R6-SOURCE-HARDENING.2` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `R6-SOURCE-HARDENING.3` | `cargo test -p specforge --lib` | 666/666 passed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |
| `R6-SOURCE-HARDENING.1` | `983b4af9` Add adapter_targets and origin_kind assertions | 4 assertions across 2 tests |
| `R6-SOURCE-HARDENING.2` | `ac92d5f5` Add size_bytes and notes assertions | 4 assertions in materialization test |
| `R6-SOURCE-HARDENING.3` | pending — about to commit | 3 assertions: width_px, height_px, diagram_kind |

## Changelog

- `2026-05-14`: Created task tree with 3 hardening leaves targeting 6 zero-coverage SourceIR fields across source.rs.
- `2026-05-14`: Tree closed. All 3 leaves complete (11 total assertions). 666/666 tests.
