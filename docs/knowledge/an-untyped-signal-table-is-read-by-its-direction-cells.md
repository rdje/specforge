---
id: an-untyped-signal-table-is-read-by-its-direction-cells
title: A table SourceIR left untyped is read as a signal table when its header names a signal column and another column holds direction words; a row the converter shifted keeps the table untyped
answers:
  - "why did CoreSight TMC not declare its AXI master interface signals"
  - "what is untyped_table_is_a_signal_description"
  - "when is an unknown-kind table read as a signal description"
  - "why was PCLKENDBG the TMC system clock"
  - "is a clock enable a clock"
  - "which actor does a section-heading signal table relate its rows to"
  - "why is TMC's memory interface table not read"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.b.i"
date: 2026-09-29
status: current
tags: [evidence-ir, signal-tables, table-kind, relations, system-contract, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (effective_table_kind, untyped_table_is_a_signal_description, literal_direction_column, extract_relations_from_signal_tables_with_prior_guidance, synthesize_system_contract_from_table_descriptions); crates/specforge/src/ir/source/docling_backend.rs (classify_table_kind); docs/tasks/extraction-representation-gaps/census-leaves.md (.3.b.i)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib an_untyped_signal_table_with_a_direction_column_is_read_as_one; then the CoreSight TMC evidence_ir.json holds 49 table_signal_declaration_provenance entries, the statement 'Clock CLK.' and no 'Clock PCLKENDBG.', and its adapter.json .isf declares (input ATVALIDS (width 1)) and no MEMQ port"
---

The ingest (`classify_table_kind`) types a signal table from a caption naming signals, a `Direction` header, or a
signal column beside a width column. CoreSight TMC's appendix tables for its ATB slave, APB debug and AXI master
interfaces have none of these: no caption, and a direction column headed `Type`. They stayed `unknown`, and since
`effective_table_kind` offered an untyped table only corpus-memory guidance, no declaration reader visited them.

Since `EXTRACTION-REPRESENTATION-GAPS.3.b.i` (`2026-09-29`) `untyped_table_is_a_signal_description` makes such a table
a signal description when its header names a signal column (`Signal`, `Name`, `Port`, `Pin`) and another column holds
direction words in most of its cells (`literal_direction_column`, the `.2h.1` rule). It is checked before corpus
memory. A name cell ending in a direction word (`MEMADDR a Output`) marks a row the converter shifted, and the table
stays untyped, so TMC's memory table does not read `MEMQ` (read data) as an output (`.3.c.viii`). Over all 78
SourceIRs this admits five TMC tables and four of GIC-600's (legacy). TMC gains 30 declarations. Twenty bracketed
names (`AWADDRM[39:0]`) stay refused as `name_not_an_identifier` (`SIGNAL-DECLARATION-ROW-DROP.2f`).

Two readers were wrong in ways the new rows exposed, and both are fixed in the same leaf. The table clock reader took
any description starting *clock*, so *Clock enable for APB.* made `PCLKENDBG` the system clock; *clock enable* is
now excluded, and `CLK` stays the clock. When the table relation reader falls back to a section heading's actor, it
now takes the row's stated direction when the table has a direction column: the actor reads an `Input` row, drives an
`Output` or `Inout` row, and takes no relation where the row states none. It had given every row as a drive.
TMC's `.isf` now lowers `master` instead of the caption actor `table_a_3_atb_slave_interface`, with every port's
direction as the document states it. The component is split across three actor names (`KG-ISF-COMPLETENESS.1i`).
See also [[untyped-uncited-tables-fall-into-fifteen-families]] and [[per-row-column-drift-rule]].
