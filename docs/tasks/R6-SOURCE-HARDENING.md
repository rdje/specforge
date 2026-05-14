# R6-SOURCE-HARDENING: SourceIR Field Test Assertion Hardening

## Metadata

- Tree ID: `R6-SOURCE-HARDENING`
- Status: `active`
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
  Status: `pending`
  Goal: `Harden SourceRegistration.size_bytes and NormalizationPlan.notes — metadata fields populated during SourceIR::build and normalization.`
  Acceptance: `Non-empty assertion on size_bytes (Some with >0 value) in a file-based test. Non-empty assertion on notes in a test that exercises normalization.`
  Verification: `pending`
  Commit: `pending`

### Batch 2: Visual asset / page geometry fields

- ID: `R6-SOURCE-HARDENING.3`
  Status: `pending`
  Goal: `Harden VisualAsset.diagram_kind and PageArtifact.width_px/height_px — figure classification and page geometry fields.`
  Acceptance: `At least one non-empty assertion each for diagram_kind, width_px, and height_px in a visual-asset-bearing test.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-SOURCE-HARDENING.2` | `pending` | Metadata fields (size_bytes, notes) — populated during build/normalization, never asserted. |
| 2 | `R6-SOURCE-HARDENING.3` | `pending` | Visual asset fields (diagram_kind, width_px, height_px) — geometry and classification metadata. |

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |
| `R6-SOURCE-HARDENING.1` | pending — about to commit | 4 assertions across 2 tests |

## Changelog

- `2026-05-14`: Created task tree with 3 hardening leaves targeting 6 zero-coverage SourceIR fields across source.rs.
