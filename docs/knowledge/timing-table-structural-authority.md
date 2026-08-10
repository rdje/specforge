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
date: 2026-08-10
tags: [timing, table, source-ir, evidence-ir, structural-authority, corpus-coverage, adr-0006]
evidence: docs/tasks/CORPUS-COVERAGE.md (CORPUS-COVERAGE.2.47a); crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/source/docling_backend.rs
reverify: "cargo test -p specforge timing_table_ --lib && target/debug/specforge kg-bench"
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
   value. A name or unit alone is not a constraint. A real table with several variant-specific min/max pairs can
   remain timing-category evidence, but it stays an unexplained-table residual until a dimension-preserving type
   exists; it is neither flattened nor handed to an unrelated extractor.

The retained production boundary contained 284 timing-labeled tables and 2,144 timing records across 39
documents. Replaying the repaired timing surface produces 608 records, changes 38 documents, keeps I2S at its
five structurally trapped but genuine records, removes Cortex 151→0, and guarantees that every survivor has at
least one min/typ/max value. Thirty affected SourceIR artifacts no longer retain their intentionally reclaimed
normalized Markdown, so only eight documents can run a complete EvidenceIR→adapter cascade without re-ingest.
Those eight reproduced one combined content hash twice; seven controls were then restored exactly and the active
Cortex chain remained repaired. See also [[timing-table-trapped-row-recovery]].
