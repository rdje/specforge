---
id: timing-scalar-rows-require-independent-cell-geometry
title: Scalar timing rows require independent source-cell geometry for parameter and populated value roles
answers:
  - "why did an OpenCAPI Notes footer become a timing constraint with parameter min typ max and unit"
  - "how does SpecForge handle Docling clones of a table cell with col_span greater than one"
  - "can identical values in separate timing cells remain valid"
  - "can a timing description or comment cell span columns without losing the timing record"
  - "how many false timing records were removed by the independent-cell geometry boundary (23; 608 to 585)"
  - "which retained documents had spanned informational timing rows (SWP HBM2 eMMC OpenCAPI)"
date: 2026-08-10
tags: [timing, table, evidence-ir, cell-geometry, docling, corpus-coverage, adr-0006]
evidence: docs/tasks/CORPUS-COVERAGE.md (CORPUS-COVERAGE.2.48a); crates/specforge/src/ir/evidence.rs
reverify: "cargo test -p specforge timing_table_ --lib && target/debug/specforge kg-bench"
---

OpenCAPI receiver-jitter table 5-11 contains two scalar timing rows followed by one `Notes:` cell spanning all
eight columns. Docling preserves that merged geometry by expanding the source cell into eight positional
`StructuredTableCellRecord` clones, each with the same text and `col_span: 8`. Positional extraction previously
indexed the clones as independent parameter, min, typ, max, and unit cells, so the footer survived the earlier
value-bearing gate as false `timing_table_0023_002`.

EvidenceIR now requires the parameter cell and every populated min/typ/max cell to have `col_span: 1`. Blank or
`-` scalar cells are absence markers rather than value authority and may be covered by a span. Unit, description,
and comment fields are optional context and do not establish scalar authority, so their spans remain legal.
Equal values in separate one-column cells remain valid; the rule tests source independence, not string identity.
No document key, vendor, table ID, parameter name, or note-prefix exception participates in production policy.

The complete retained boundary contains 80 SourceIR documents, 105 timing-classified tables, and 608 scalar
records before this check. Exact replay removes 23 false records—three SWP notes, seven HBM2 group rows, twelve
eMMC note/group rows, and the OpenCAPI footer—and leaves 585 survivors. No survivor carries a multi-column span
in the parameter or a populated scalar role. OpenCAPI, the only affected document with retained normalized
Markdown, reproduces its corrected 60-record downstream cascade twice; the three other current chains remain
byte-exact controls rather than being misrepresented as complete rebuilds. See also
[[timing-table-structural-authority]] and [[timing-table-trapped-row-recovery]].
