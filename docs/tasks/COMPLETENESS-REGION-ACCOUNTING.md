# COMPLETENESS-REGION-ACCOUNTING: surface intent-bearing source regions that produced no fact

## Metadata

- Tree ID: `COMPLETENESS-REGION-ACCOUNTING`
- Status: `active`
- Roadmap lane: `R15e` (KG-quality / completeness) — cross-cutting R15c
- Created: `2026-06-01`
- Last updated: `2026-06-01`
- Owner: repo-local workflow

## Goal

Implement the **foundational miss detector** from the completeness research
([`docs/research/region-accounting-design.md`](../research/region-accounting-design.md),
`INTENT-COMPLETENESS-RESEARCH.3`): the input-side reframe — every *intent-bearing*
source region must produce ≥1 extracted fact or be explicitly non-intent;
**an intent-bearing region that produced nothing is a candidate miss**, surfaced
as a residual. This catches misses no existing finding covers, and needs no
ground truth.

First slice (this tree's `.2`): **intent-bearing TABLE coverage** — the cleanest,
reliable-provenance region accounting. A `StructuredTableRecord` recognized as a
register/signal/timing table that yielded **zero** downstream records is an
unexplained table region. (Prose/figure region accounting + the unified
backward-traceability index are later slices/trees.)

## Why tables first

The research flagged a *provenance-completeness precondition* (facts must carry a
resolvable region id). Verified `2026-06-01` that the table→record links ARE
reliable today:
- SignalDescription → `TableSignalDeclarationProvenanceRecord.table_id` (direct).
- RegisterMap → `register_record.register_id` embeds the `table_id`
  (`reg_table_0026_000` ⊃ `table_0026`; 21/21 on AHB).
- TimingParameter → `timing_constraint.constraint_id` embeds the `table_id`
  (`timing_table_0022_000`).

So table-coverage accounting is exact today with no provenance fix. (Encoding
tables → `ExtractedStatement` link is less clean; deferred.)

## Non-Goals

- NOT the full region-accounting framework yet (prose/figure regions, the unified
  backward index, the intent/non-intent/deferred classifier) — those are later
  slices once this proves the surface + plumbing.
- NOT changing extraction — a flag-only detector; never mutates the IR.
- NOT fixing the misses it finds — each becomes its own owned tree (e.g. a table
  that yields nothing may trace to an extractor gap or a misclassification).

## Acceptance Criteria

- A pure, unit-tested `unexplained_intent_bearing_tables(...)` in
  `ir/completeness.rs` over the existing IR types: for each SignalDescription /
  RegisterMap / TimingParameter table, covered iff ≥1 corresponding record links
  to its `table_id`; else an `UnexplainedTableResidual`.
- Surfaced in `validate_evidence_ir` (a `Region Accounting` section + finding +
  metrics), mirroring the register-tiling/convergence surfaces.
- Extraction-neutral; full `scripts/run_ci.sh` green; book note (per close-rule).

## Task Tree

- ID: `COMPLETENESS-REGION-ACCOUNTING`
  Status: `active`
  Goal: intent-bearing table-coverage region accounting + validate surface
  Children: `.1`, `.2`

- ID: `COMPLETENESS-REGION-ACCOUNTING.1`
  Status: `done`
  Goal: own + design (this file); verify the table→record provenance is reliable; register.
  Acceptance: tree created with the table-coverage design + verified provenance links; registered.
  Verification: >
    passed (`2026-06-01`) — owned + designed the bounded first slice (intent-
    bearing table coverage); verified the table→record backward links are
    reliable today (signal provenance `table_id` direct; register/timing ids
    embed `table_id`, 21/21 on AHB) so the detector is exact with no provenance
    fix. Registered in `docs/TASK_TREE.md`. Docs-only.
  Commit: `see Commit Log`

- ID: `COMPLETENESS-REGION-ACCOUNTING.2`
  Status: `pending`
  Goal: >
    Implement `unexplained_intent_bearing_tables` (pure) in `ir/completeness.rs`
    + `UnexplainedTableResidual`; unit tests (covered table / zero-yield table /
    non-intent kind skipped, per kind); wire into `validate_evidence_ir`
    (`Region Accounting` section + finding [Info none / Warning some] + metrics);
    book note; full CI; close.
  Acceptance: detector + tests + validate surface; extraction-neutral; CI green; book updated; tree CLOSED.
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `COMPLETENESS-REGION-ACCOUNTING.1` | `done` | owned + design + provenance verified |
| 2 | `COMPLETENESS-REGION-ACCOUNTING.2` | `pending` | implement the table-coverage detector + validate + tests + book + close — next |

## Decisions

- `2026-06-01`: tables-first (reliable provenance, exact today) before prose/
  figure region accounting (which need the intent classifier + a backward index).
- `2026-06-01`: surface as `validate` findings + metrics (consistent with the
  register-tiling / convergence surfaces), not a new persisted report yet — the
  unifying `CompletenessReport` is research `.5` + a later tree.

## Open Questions

- Encoding-table coverage: confirm the `synthesize_encoding_declarations` →
  `ExtractedStatement` link carries a resolvable `table_id` before including it.
- Severity: a zero-yield intent table is a *candidate* miss (could be a genuinely
  empty/mis-OCR'd table) → likely Info/Warning, not Error.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | table→record provenance verified reliable (signal `table_id` direct; register/timing ids embed `table_id`); design + tree registered; docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `COMPLETENESS-REGION-ACCOUNTING.1` | `COMPLETENESS-REGION-ACCOUNTING.1 — own + design intent-bearing table-coverage region accounting` | docs-only |

## Changelog

- `2026-06-01`: Created — own the region-accounting detector (research `.3`),
  first slice = intent-bearing table coverage (reliable-provenance, exact today).
  Frontier → `.2` (implement).
