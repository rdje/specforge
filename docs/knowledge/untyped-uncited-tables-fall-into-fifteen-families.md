---
id: untyped-uncited-tables-fall-into-fifteen-families
title: The 1,165 untyped tables no canonical record cites fall into 15 families — two in five non-contract, a third contract content with an owner, the rest read table by table
answers:
  - "what are the untyped tables no record cites"
  - "what is scripts/measure_untyped_table_families.py"
  - "how many captured tables are typed unknown and uncited"
  - "which untyped tables are contract content"
  - "why does CoreSight TMC lack its AXI master interface signals"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.b"
  - "which tables are non-contract"
date: 2026-09-29
status: current
tags: [source-ir, tables, captured-regions, census, extraction-representation-gaps, measured]
evidence: scripts/measure_untyped_table_families.py; docs/research/category-family-census.md (The 1,165 untyped tables no record cites); docs/tasks/extraction-representation-gaps/census-leaves.md (.3.b, .3.b.i, .3.b.ii); docs/book/src/pipeline/semanticir.md (Captured regions that reach no carrier)
reverify: "python3 scripts/measure_untyped_table_families.py — expect (2026-09-29) 1165 tables: residue 257, structure_or_message_layout 207, document_apparatus 180, figure_fragment 106, parameter_definition 78, software_or_performance 73, presence_or_compliance_matrix 71, physical_coding_or_electrical 70, register_summary 56, and signal_declaration_untyped 6 with 55 rows, 0 in the catalog"
---

The population is every table SourceIR typed `unknown` whose region SemanticIR lists in `captured_region_residuals`,
on the 31 retained chains: 1,165 on `2026-09-29` (515 header shapes, 274 with no header row). The `.3` census had
counted 1,161 on `2026-09-26`.

`scripts/measure_untyped_table_families.py` places each table by its first header row's words, or with no header
row by its body's shape (empty; one column; mostly bit numbers), first match wins. 451 tables (38.7%) state nothing
the contract lowers: document apparatus 180, figures read as tables 106 (ADIv6 62), instruction timings and
programming examples 73, line coding and electrical parameters 70, empty 15, waveform narrations 7. 389 (33.4%) are
contract content with an owner: structure and message layouts 207 (`DOC-INTENT-TAXONOMY.4b`), AXI's parameter
definitions 78 (`KG-ISF-COMPLETENESS.2a.vii.c`), presence and compliance matrices 71 (`.3.c.v`), multi-key encodings
27 (`.3.g.iii`) and TMC's signal tables 6 (`.3.b.i`). 325 (27.9%) are read table by table in `.3.b.ii`: 68 register
indexes, register-figure pieces and missed field tables, and a residue of 257.

The one loss found: CoreSight TMC r0p1's appendix tables for the ATB slave, APB debug, memory and AXI master
interfaces (`Signal | Type | Description`) are typed `unknown` at ingest, so no declaration reader visits them, and
none of their 55 signals reaches the catalog. `SIGNAL-DECLARATION-ROW-DROP.6` fixed the same header only on typed
tables. Sample precision (seven random members read per family) and the family table are in the census record.
See also [[a-tables-reading-order-is-the-gap-it-left]].
