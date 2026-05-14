# R6-EVIDENCE-HARDENING: Evidence Module Test Assertion Hardening

## Metadata

- Tree ID: `R6-EVIDENCE-HARDENING`
- Status: `active`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-14`
- Owner: repo-local workflow

## Goal

Add regression-only test assertions to `ExtractedStatement.modality` and `ExtractedStatement.related_visual_evidence_ids` — two fields populated in production with zero test coverage.

## Non-Goals

- Do not change production behavior. This is regression-only hardening.
- Do not add assertions to fields already tested by existing checks.
- Do not add assertions that require new test fixture infrastructure.

## Acceptance Criteria

- Zero-coverage `modality` and `related_visual_evidence_ids` fields hardened with at least one non-empty assertion each.
- All tests pass after every completed leaf.
- Each leaf is committed through `COMMIT.md` with leaf-ID traceability.

## Task Tree

- ID: `R6-EVIDENCE-HARDENING`
  Status: `active`
  Goal: `Harden zero-coverage ExtractedStatement.modality and related_visual_evidence_ids assertion gaps.`
  Children: `R6-EVIDENCE-HARDENING.1`

### Batch 1: ExtractedStatement modality + related_visual_evidence_ids

- ID: `R6-EVIDENCE-HARDENING.1`
  Status: `pending`
  Goal: `Harden ExtractedStatement.modality and related_visual_evidence_ids — zero assertions codebase-wide despite being populated in every EvidenceIr build.`
  Acceptance: `4 assertions in builds_evidence_ir_from_markdown_source_ir: modality = Text for both statements, related_visual_evidence_ids empty for both.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-EVIDENCE-HARDENING.1` | `pending` | Only leaf — harden two zero-coverage ExtractedStatement fields |

## Decisions

- `2026-05-14`: Scoped to regression-only test assertion additions. Zero production behavior changes.
- `2026-05-14`: Selected `builds_evidence_ir_from_markdown_source_ir` test — already asserts on extracted_statements, minimal insertion friction.

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

- `2026-05-14`: Created task tree with 1 hardening leaf targeting ExtractedStatement.modality and related_visual_evidence_ids zero-coverage fields.
