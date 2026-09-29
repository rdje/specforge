---
id: timing-table-structural-authority
title: Timing tables require classified structural authority, and scalar records require a value-bearing min/typ/max schema
answers:
  - "why did Cortex-A76 instruction performance tables produce 151 timing constraints with no min typ max values"
  - "can timing table shape promote an unknown table to timing_parameter (no; shape validates upstream classification or a prior)"
  - "how does SpecForge distinguish a timing table category from a scalar min typ max layout"
  - "why does Instruction group not count as an ns unit and why does SMIN not count as min"
  - "what happens to multi-variant timing limit tables that cannot fit TimingConstraintRecord"
  - "what is the corpus timing-table authority repair measurement (2144 to 608 across 39 documents)"
  - "how does SpecForge read a timing table with one MIN/MAX pair per operating mode"
  - "what is operating_condition on a timing constraint"
  - "why does a USB 3.2 tBurst tRepeat timing table produce no timing records"
  - "what is row_conditions on a timing constraint"
  - "why does a timing record carry a Note 1 as its condition"
  - "how are USB 3.2 timer tables with a Timeout Value column read"
  - "can an untyped table produce timing records without being typed timing_parameter"
  - "why is a timeout value a typ value"
date: 2026-09-29
tags: [timing, table, source-ir, evidence-ir, structural-authority, corpus-coverage, adr-0006]
evidence: docs/tasks/CORPUS-COVERAGE.md (CORPUS-COVERAGE.2.47a); crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/source/docling_backend.rs
reverify: "cargo test --workspace --exclude specforge-production-graph timing_table_ --lib && target/debug/specforge kg-bench"
---

The Cortex-A76 Software Optimization Guide exposed a shared false-authority path. Docling can put a data row in
`header_rows` when its first cell is a row header. The old Python classifier flattened every such row, then used
raw substring matching. A mnemonic such as `SMIN` could therefore supply `min`, while the letters `ns` inside
`Instruction group` supplied a supposed nanosecond unit. EvidenceIR repeated the substring mistake, selected the
operation-name column as `unit`, and emitted a record even though min, typ, and max were all absent. Eleven
instruction-performance tables became 151 false scalar timing records.

The repaired boundary separates two questions:

1. **Does an upstream classification have structural support?** Only the leading prefix of rows whose non-empty
   cells are all column headers contributes timing vocabulary. Identifier tokenization keeps `_` inside a token,
   so neither embedded letter sequences nor names such as `OPTIMAL_TRIM_UNIT_SIZE` create standalone authority.
   The prefix must contain a min/typ/max role plus parameter/symbol, explicit unit, or timing/unit caption context.
   This check may demote an unsupported `timing_parameter` classification; it does not promote an otherwise
   `unknown` table. Category authority still comes from ingest classification or a protocol-matched prior.
2. **Can this category be represented as one scalar `TimingConstraintRecord` per row?** Min, typ, and max roles
   must each resolve to at most one distinct column, and an emitted row must carry at least one actual scalar
   value. A name or unit alone is not a constraint. A real table with several variant-specific min/max pairs
   remains timing-category evidence and is never flattened nor handed to an unrelated extractor; since
   `EXTRACTION-REPRESENTATION-GAPS.3.f` (`2026-09-28`) it is read one scalar layout per operating condition when its
   header states the conditions (see below), and otherwise stays an unexplained-table residual.

The retained production boundary contained 284 timing-labeled tables and 2,144 timing records across 39
documents. Replaying the repaired timing surface produces 608 records, changes 38 documents, keeps I2S at its
five structurally trapped but genuine records, removes Cortex 151→0, and guarantees that every survivor has at
least one min/typ/max value. Thirty affected SourceIR artifacts no longer retain their intentionally reclaimed
normalized Markdown, so only eight documents can run a complete EvidenceIR→adapter cascade without re-ingest.
Those eight reproduced one combined content hash twice; seven controls were then restored exactly and the active
Cortex chain remained repaired. See also [[timing-table-trapped-row-recovery]].

**Per-condition limits (`2026-09-28`, `EXTRACTION-REPRESENTATION-GAPS.3.f`).** `timing_table_layouts`
(`crates/specforge/src/ir/source.rs`) reads the conditions before the scalar layout: the last structural header
row names each value column's role in one word, exactly one row above it splits the value columns into two or more
runs each headed by one cell spanning exactly the run (no label twice, no role twice in a run), and the table must
name its parameter column `Symbol`/`Parameter`/`Name`. Each record then carries the run's label verbatim in
`TimingConstraintRecord.operating_condition` (id suffix `_<row>_<condition>`), and `is_temporal_rule_input` keeps it
out of temporal rules. On the 31 current chains only I2C's Tables 10, 11 and 13 qualify (37 -> 148 records). USB
3.2's five grouped tables name no parameter column, and two of them (`tBurst | tRepeat`) put the parameters in the
groups, so they stay residual (`.3.f.iii`); a single label over every value column (`Hs-mode`, `Limits`) is not a
condition (`.3.f.ii`). A cell holding only note references (`[2]`, `- [3]`) states no value.

**Row conditions (`2026-09-29`, `EXTRACTION-REPRESENTATION-GAPS.3.f.i`).** A column a header names `Condition(s)` or
`Test condition(s)` (and nothing else) is read into `TimingConstraintRecord.row_conditions`, verbatim; a heading
row (a condition, no value, the same parameter as the next row) is prepended to the rows that refine it (I2C Table
12's `V OL`, `t rCL`, `t fCL`). The cell is whatever the document wrote there (conditions, OpenCAPI note references,
an I2C remark, I2S derivations), so the field promises the cell, not a parsed condition, and a row-conditioned limit
is not a temporal-rule input. Four chains carry one (93 records); I2C's ambiguous (table, parameter, mode) keys 8 -> 0.

Shape still does not TYPE a table; it can let EvidenceIR read one it would otherwise ignore. Since
`EXTRACTION-REPRESENTATION-GAPS.3.b.iii.c` (`2026-09-29`) an untyped table heading a column `Timeout Value(s)`,
optionally with one duration unit, beside a `Name`/`Timer(s)`/`Parameter`/`Symbol` column is read by
`synthesize_timeout_table_constraints`, and SourceIR's classification (inside the SOURCE seal closure) is unchanged.
A timeout is the duration its timer is set to, so an unlabeled value is `typ`; `12 ms (min) 120 ms (max)` gives
roles; `12 ms (x1) 24 ms (x2)` or two header-labeled value columns give one record per label as
`operating_condition`, and one cell spanning both gives one unlabeled record; an `Initial State` column is the
row's condition; a trailing footnote number is dropped only when the page prints that note. USB 3.2 gains 34
records (74 -> 108) and nothing else on the 31 chains moves.

