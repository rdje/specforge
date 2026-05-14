# R6-EVIDENCE-HARDENING: Evidence Module Test Assertion Hardening

## Metadata

- Tree ID: `R6-EVIDENCE-HARDENING`
- Status: `active`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-14`
- Owner: repo-local workflow

## Goal

Add regression-only test assertions to zero-coverage `EvidenceIr` and `ExtractedStatement` fields populated in production.

## Non-Goals

- Do not change production behavior. This is regression-only hardening.
- Do not add assertions to fields already tested by existing checks.
- Do not add assertions that require new test fixture infrastructure.

## Acceptance Criteria

- Zero-coverage fields hardened with at least one non-empty assertion each.
- All tests pass after every completed leaf.
- Each leaf is committed through `COMMIT.md` with leaf-ID traceability.

## Task Tree

- ID: `R6-EVIDENCE-HARDENING`
  Status: `active`
  Goal: `Harden zero-coverage evidence.rs field assertion gaps.`
  Children: `R6-EVIDENCE-HARDENING.1`, `R6-EVIDENCE-HARDENING.2`, `R6-EVIDENCE-HARDENING.3`

### Batch 1: ExtractedStatement modality + related_visual_evidence_ids

- ID: `R6-EVIDENCE-HARDENING.1`
  Status: `done`
  Goal: `Harden ExtractedStatement.modality and related_visual_evidence_ids — zero assertions codebase-wide despite being populated in every EvidenceIr build.`
  Acceptance: `4 assertions in builds_evidence_ir_from_markdown_source_ir: modality = Text for both statements, related_visual_evidence_ids empty for both.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `906597ad`

### Batch 2: EvidenceIr.signal_alias_map

- ID: `R6-EVIDENCE-HARDENING.2`
  Status: `pending`
  Goal: `Harden EvidenceIr.signal_alias_map (BTreeMap<String, String>) — zero assertions despite being populated in 4 tests with non-trivial alias data.`
  Acceptance: `3 assertions in alias_grounded_prose_descriptions_produce_semantic_handshake_hints: len = 2, get("request phase") = "XREQ", get("accept phase") = "XACK".`
  Verification: `pending`
  Commit: `pending`

### Batch 3: FsmSignalCandidate.direction_hint_conflicted

- ID: `R6-EVIDENCE-HARDENING.3`
  Status: `pending`
  Goal: `Harden FsmSignalCandidate.direction_hint_conflicted (bool) — only 2 assertions across ~3300 assertions in adapters.rs, controls renderability branching in production.`
  Acceptance: `2+ assertions alongside existing direction_hint/width_hint/graph_direction_hint sibling assertions in signal_inventory tests.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-EVIDENCE-HARDENING.2` | `pending` | signal_alias_map has zero assertions, 4 tests populate non-trivial data |
| 2 | `R6-EVIDENCE-HARDENING.3` | `pending` | direction_hint_conflicted has only 2 assertions, critical production role |

## Decisions

- `2026-05-14`: Scoped to regression-only test assertion additions. Zero production behavior changes.
- `2026-05-14`: Selected `builds_evidence_ir_from_markdown_source_ir` test — already asserts on extracted_statements, minimal insertion friction.
- `2026-05-14`: Expanded tree scope to cover additional zero-coverage evidence.rs and adapters.rs fields discovered after leaf 1.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-14` | `R6-EVIDENCE-HARDENING.1` | `cargo test -p specforge --lib` | 666/666 passed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |
| `R6-EVIDENCE-HARDENING.1` | `906597ad` | 4 assertions on modality + related_visual_evidence_ids |

## Changelog

- `2026-05-14`: Created task tree with 1 hardening leaf targeting ExtractedStatement.modality and related_visual_evidence_ids.
- `2026-05-14`: Added leaves 2 (signal_alias_map) and 3 (direction_hint_conflicted) — zero-coverage fields discovered after leaf 1 completion.
