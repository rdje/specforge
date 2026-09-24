---
id: a-table-premise-is-recaptured-from-retained-converter-output-not-re-ingested
title: A SourceIR table premise may be recaptured from the retained converter document, one field at a time and only where it reads differently. A re-ingest is not the remedy for a table-reading fix
date: 2026-09-24
status: accepted
scope: source-ir, proof-premise, retained-capture, ingest-drift, kg-isf-completeness, source-proof-migration
evidence: crates/specforge/src/ir/source.rs (recapture_tables_from_retained_converter, locate_recaptured_tables, refresh_from_retained_context); crates/specforge/src/ir/source/docling_backend.rs (recapture_retained_tables, extract_table_grid); crates/specforge/examples/source_table_recapture.rs; docs/tasks/kg-isf-completeness/relation-precision.md (.1d.i.a.i, .1d.i.a.ii.a, .1d.i.a.ii.b); docs/book/src/pipeline/sourceir.md
reverify: "cargo run -q --features source-proof-migration -p specforge --example source_table_recapture -- --retained-manifest doctrine/chain_currency/retained_bundles.json — before .1d.i.a.ii.c writes, expect would-change for exactly AHB (5 tables) and APB (3) and unchanged for the other 25; after it, 27 unchanged"
answers:
  - "what is ADR 0052"
  - "how does a change to how ingest reads tables reach a persisted SourceIR"
  - "why not re-ingest AHB and APB to carry a table fix"
  - "is it legitimate to change a SourceIR proof premise"
  - "what is the difference between source_proof_migrate and source_table_recapture"
  - "what does source_table_recapture refuse"
---

# ADR 0052: A SourceIR table premise may be recaptured from the retained converter document, one field at a time and only where it reads differently. A re-ingest is not the remedy for a table-reading fix

## Context

`KG-ISF-COMPLETENESS.1d.i.a.i` changed how the ingest helper reads a table: a body cell Docling indexed into
the wrong column is re-homed to the header column its geometry sits under. Over the 1,256 retained Docling
tables, that changes 8 (AHB 5, APB 3), which are two of the wire-based gold documents.

A persisted SourceIR cannot pick the change up by itself. Its table rows are a proof premise
(`proof_context.field_premises.structured_tables`): captured helper output, not something the proof derives.
`source_proof_migrate` re-executes the retained context and deliberately never recaptures, so it re-seals the
old rows.

That left two ways to carry the fix:

1. **Re-ingest the two PDFs.** Measured re-ingest drift (recovered figure text, ordinal id shifts; see the
   book's `pipeline/sourceir.md`) would arrive in the same change as the table fix, in two gold chains.
   WIRE-BASED-100 could then not attribute a score move to one cause or the other.
2. **Recapture the one premise field from the retained converter document.** The persisted `backend.json`
   *is* Docling's output. Re-running only the helper's `extract_table_grid` over it re-reads the tables
   without running a model.

`.1d.i.a.ii.a` measured the precondition for option 2. The premise is exactly the pre-repair helper's output
from the retained document: **1,256 of 1,256 tables are equal**. A recapture therefore reproduces the existing
capture everywhere except where the repair applies.

## Decision

A change to how ingest reads **tables** reaches a persisted current-schema SourceIR by **recapture**, not
re-ingest:

- Only the `structured_tables` premise is re-derived, only for tables whose recaptured rows differ, and only
  their `header_rows`, `body_rows` and `row_count`. Every other premise, and every table that reads the same, is
  kept byte-for-byte.
- Every premise table must be located unambiguously in the retained document before anything changes. The
  match is `source_ref` for a single document, and `(source_batch, source_ref)` for a batched one. A
  pre-coordinate batched artifact matches by position, and only when counts and every `source_ref` agree.
  Anything unlocated, ambiguous or mixed refuses the whole artifact.
- The proof is then refreshed exactly as a retained-capture migration refreshes it: classifications replayed,
  the validation premise cleared, the ledger re-proved, validation re-applied.
- The path is maintenance-only. It is feature-gated (`source-proof-migration`), exposed as
  `examples/source_table_recapture.rs`, and dry-runs by default.

## Consequences

- A table-reading fix and its corpus refresh no longer need a Docling run. The dry run is the reach
  measurement: before `.1d.i.a.ii.c`, 2 of the 27 retained documents would change.
- Downstream stages still have to be rebuilt stage by stage (`retained-chain-rebuild-order`), because a changed
  table can change EvidenceIR.
- Only documents that retain their bundle can be recaptured. The 51 that retain none are out of reach, as they
  are for `source_proof_migrate`.
- The promoted markdown (`normalized/*.md`) is Docling's own export and is **not** recaptured. It keeps
  Docling's column order for these tables. This decision does not rely on the markdown, and changes the table
  record only.
- The recapture is **not** a general licence to edit premises. It applies only to fields that are a
  deterministic function of a retained converter document, and only after equality with the existing capture
  has been measured. Any other premise field needs its own decision.
- Feature-gated code is not compiled by CI today. That gap is owned separately (see the owning tree), not
  assumed away.

## Links

- Related: [[docling-tables-live-under-documents-in-a-batched-bundle]], [[retained-chain-rebuild-order]],
  [[producer-change-and-corpus-rebuild-are-one-transaction]].
