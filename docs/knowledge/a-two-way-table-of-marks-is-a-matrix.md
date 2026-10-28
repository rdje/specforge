---
id: a-two-way-table-of-marks-is-a-matrix
title: A two-way table is a matrix (matrix_tables), unless another reader holds it or its shape is not a cross-tab
answers:
  - "what does a presence code like YM mean (bind_presence_legends, .3.c.v.iii)"
  - "what does AHB's presence code OC mean (Table A-1, legend_table_id, .66)"
  - "what does a footnoted matrix cell like Y 2 or a title like Update 1 mean (MatrixNote, bind_matrix_notes, .64)"
  - "what is the isf_protocol_matrix_table residual (.3.c.v.ii)"
  - "what is matrix_tables / MatrixTableRecord / read_matrix_tables (.3.c.v.i)"
  - "how is AXI Table A12.11 Legal tag operations read"
  - "how is I2C Table 3 (M = mandatory; O = optional) read"
  - "how are AXI's property interoperability and compatibility tables read"
  - "why are TMC's Permitted register writes tables not matrices"
  - "what is a requirement, legality, version or effect matrix (.3.c.v)"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.c.v and .3.c.v.i"
  - "what is EXTRACTION-REPRESENTATION-GAPS.68 / .68.a (prose and value cross-tabs)"
  - "how does a two-header cross-tab keep a statement per cell"
date: 2026-10-28
status: current
tags: [evidence-ir, tables, matrices, requirements, legality, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (read_matrix_tables; matrix_cell_kind; is_matrix_code_literal; matrix_caption_legend; matrix_table_surface; trailing_note_marks; bind_matrix_notes); docs/archive/tasks/extraction-representation-gaps/layout-and-binding-leaves.md (.3.c.v, .3.c.v.i); docs/tasks/extraction-representation-gaps/table-reading-leaves.md (.68); .project-data/tmp/erg-68/
reverify: "python3 .project-data/tmp/erg-68/census.py && python3 .project-data/tmp/erg-68/refine.py; cargo test --offline -p specforge-core --lib a_two_way_table_of_marks_is_a_matrix"
---

Many specification tables answer per pair of keys: AXI *A12.11* (opcode x `MTE_Support` x tag operation,
`Y` or `-`), I2C *Table 3* (feature x configuration, `M`/`O`/`n/a`), USB 3.2 *10-2* (hub connection x
port type, *On* or *May be off*), AXI's property interoperability tables (manager value x subordinate
value, *Compatible.* or *Not compatible.* with a reason). `.3.c.v` (`2026-10-22`) sorted 34 unread ones
into legality, requirement, version and effect matrices, plus one legend, and decided one record for all
of them.

**The reader (`EXTRACTION-REPRESENTATION-GAPS.3.c.v.i`).** `read_matrix_tables` runs after the
convergent loop. It takes the label columns as every leading column before the first whose filled cells
are mostly marks or modal words (`matrix_cell_kind`, a closed grammatical class; a footnote mark such as
`O [2]` dropped). The header is the leading run of non-data rows, at most the header rows EvidenceIR
keeps after `.57.a`'s lifting, and seven in ten filled data cells must be marks or modal words. It
refuses, and counts, a slice-titled data column, all-number column titles, code-literal label columns
(`is_matrix_code_literal`) and two or more columns titled by declared register fields. It leaves
presence, register, value-combination and layout tables to their readers. Each record keeps the row key
titles, each column's header path, the rows as written, the cell class and a caption's legend
(`matrix_caption_legend`).

**Measured `2026-10-22` (31 chains):** 48 matrices in 10 documents; TMC's three *Permitted register*
tables are refused (their columns are register fields with `0`/`1`/`X`, a value combination the
combination reader refuses for its emptied cells). The prototype had read B2.4's last page with 3 header
rows; EvidenceIR keeps 1 there after `.57.a`, so the header is capped at the kept rows and the sparse
rows below it are data. The `matrix_tables` manifest entry is recorded only when a table had the shape.

**Carried (`.3.c.v.ii`, `2026-10-22`).** SemanticIR and IntentIR carry `matrix_tables` unchanged, and
the adapter states one `isf_protocol_matrix_table_*` residual per matrix, citing its tables: no rule,
guard or drive names a matrix's keys. A legality matrix's lowering is `.3.c.v.iv`'s to decide; a
requirement, version or effect matrix states configuration the `.isf` does not carry (ADR 0033 §6).

**A legend binds presence codes (`.3.c.v.iii`, `2026-10-22`).** `bind_presence_legends` reads a matrix
with one label column and modal cells as a legend. A presence table binds to it when every code the
table uses is a row key of exactly one such legend. Each `VariantPresenceEntry` then gains `legend`
(column, meaning) and the record `legend_matrix_id`. AXI: 297 of 306 presence rows bind to *B2.1*;
*A13.3* uses `C`, which *B2.1* does not define. Since `.66` (`2026-10-22`) a two-column key table
(distinct codes, every row filled, no matrix or presence table) competes on the same terms and is
recorded as `legend_table_id`: AHB's 80 presence entries bind to *Table A-1*.

**A cell's note (`.64`, `2026-10-22`).** `bind_matrix_notes` reads each data-column title's and cell's
trailing marks (`trailing_note_marks`: `Y 2`, `O [2]`, `State 1, 2`; a cell of marks alone has none)
and binds each mark to the one note with that mark printed beside the matrix's tables
(`footnote_texts_by_page`). `MatrixTableRecord.notes` gives each note's text, its columns and its
cells; the cells stay as written. Measured on 31 chains: AXI *A12.13* 3 notes (`Update 1`, one `Y 2`,
two `Y 3`), USB 3.2 *9-2* 2 on both columns; 29 chains identical. I2C *Table 3*'s notes are list
items whose numbers the converter drops, so `O [2]` binds nothing (`.65`).

**Prose and value cells (`.68`, decided `2026-10-28`).** A 31-chain census found 32 unclaimed candidates, then
refused 22 one-header maps or sequences, bit layouts, timing tables, navigation rows and the I2C continuation `.73`
owns. Ten cross-tabs in 6 documents hold 45 rows / 213 non-empty cells: AHB `table_0024`, OpenCAPI DLL
`table_0050`/`table_0052`, OpenCAPI discovery `table_0025`, I2C `table_0013`, USB 3.2
`table_0057`/`table_0097`/`table_0108`/`table_0211`, and Wishbone `table_0004`. They need at least two header levels
and independent keys, with the label count read from the headers' leading blank run. Existing readers, timing
included, keep their tables; timing-limit roles and navigation refuse. `MatrixRow.cells[i]` is already the exact
statement at `row.keys` x `column_keys[i]`, so no duplicate statement surface or synthesized sentence is needed.
`.68.a` owns the `Value`/`Prose` classes and fallback; existing mark/modal records must not change.
