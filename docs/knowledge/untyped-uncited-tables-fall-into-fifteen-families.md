---
id: untyped-uncited-tables-fall-into-fifteen-families
title: The untyped tables no canonical record cites (1,160) are either non-contract (53.1%) or contract content with an owner (46.9%) — by family rule, and by reading the 125 no rule places
answers:
  - "what are the untyped tables no record cites"
  - "what is scripts/measure_untyped_table_families.py"
  - "how many captured tables are typed unknown and uncited"
  - "which untyped tables are contract content"
  - "why does CoreSight TMC lack its AXI master interface signals"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.b"
  - "which tables are non-contract"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.b.ii"
  - "who owns an untyped encoding table"
date: 2026-09-29
status: current
tags: [source-ir, tables, captured-regions, census, extraction-representation-gaps, measured]
evidence: scripts/measure_untyped_table_families.py; docs/research/category-family-census.md (The 1,165 untyped tables no record cites); docs/archive/tasks/extraction-representation-gaps/table-census-leaves.md (.3.b, .3.b.i, .3.b.ii); docs/book/src/pipeline/semanticir.md (Captured regions that reach no carrier)
reverify: "python3 scripts/measure_untyped_table_families.py — expect (2026-09-29, after .3.b.iii.c) 1119 tables (timing_untyped 1, the notes of USB 3.2 Table 7-12): document_apparatus 201, structure_or_message_layout 191, figure_fragment 164, residue 105, software_or_performance 94, parameter_definition 77, presence_or_compliance_matrix 69, physical_coding_or_electrical 69, response_or_requirement_matrix 44, register_field_table 12, register_summary 7, signal_declaration_untyped 1"
---

The population is every table SourceIR typed `unknown` whose region SemanticIR lists in `captured_region_residuals`,
on the 31 retained chains: 1,165 on `2026-09-29` when `.3.b` measured it (the `.3` census had 1,161 on `2026-09-26`),
and 1,160 once `.3.b.i` read five CoreSight TMC signal tables.

`scripts/measure_untyped_table_families.py` places each table by its first header row's words, its caption's words, or
with no header row by its body's shape, first match wins. `.3.b`'s first pass left 325 tables to be read; `.3.b.ii`
added rules (register-figure pieces, response and requirement matrices, captions saying *permitted*, *legal* or
*example*, acronym glossaries, letter-spaced figure text) and read the 125 that remain one by one. The dispositions are
in the census record.

The census, whole: 616 tables (53.1%) state nothing the contract lowers (document apparatus, figures read as tables,
instruction timings and worked examples, line coding and electrical parameters, empty tables, waveform narrations,
and 56 of the hand-read). 544 (46.9%) are contract content, every one with an owner: `DOC-INTENT-TAXONOMY.4b` 225
(structure and message layouts), `.3.c.v` 135 (presence, requirement and response matrices), AXI's parameter
definitions 78 (`KG-ISF-COMPLETENESS.2a.vii.c`), `.3.g.iii` 57 (multi-key encodings and behaviour; `.3.g.iii.b` since `2026-10-21`), `.3.b.iii` 43
(single-key value, timer and constraint tables the ingest left untyped), `.41.e` 5 (a field's bits mapped to a signal's)
and `.3.c.viii` 1 (TMC's shifted memory table).

The one loss `.3.b` found, TMC's untyped `Signal | Type | Description` tables, is read since `.3.b.i`
([[an-untyped-signal-table-is-read-by-its-direction-cells]]). `.3.b.ii` named one more: AXI's *DVM Complete* table
(`Signal | Constraint`, *ARSNOOP must be 0b1110*), which no constraint reader visits (`.3.b.iii`).
`.3.b.iii` (`2026-09-29`) read the 43 single-key tables: the encoding reader already takes an untyped table by its
header when the enum has an anchor, and 23 encode a structure, message or protocol field with no signal or register
to name the enum after, which `KG-ISF-COMPLETENESS.5.i` refuses by design (`DOC-INTENT-TAXONOMY.4b`). The rest split
into field value tables in unread header shapes, a constraint table, timer tables and an alias table (`.3.b.iii.a`-
`.d`), and one register misattribution (`.41.f`).
After `.3.b.iii.e` 38 of the 1,160 turned out to be carried: their rows had reached signal constraints and rules through
Markdown-row statements that cited no table. The population is 1,122 ([[captured-region-residual-carrier]]).
After `.3.b.iii.c` three of the four timer tables are read as timing, and it is 1,119.
See also [[a-tables-reading-order-is-the-gap-it-left]].
