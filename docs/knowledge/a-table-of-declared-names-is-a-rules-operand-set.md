---
id: a-table-of-declared-names-is-a-rules-operand-set
title: A table whose every cell is a declared signal name is a rule's operand set (AXI A6.5); it becomes EvidenceIR signal_sets
answers:
  - "how is AXI Table A6.5 Signals that should be the same in an exclusive sequence read"
  - "what is EXTRACTION-REPRESENTATION-GAPS.67 / .67.a / .67.b"
  - "what is EvidenceIR signal_sets / citing_statement_ids"
  - "which tables list only declared signal names"
  - "where does a rule that cites a table by number (signals shown in Table A6.5) get its operands"
date: 2026-10-23
status: current
tags: [evidence-ir, tables, rules, signal-sets, extraction-representation-gaps, measured]
evidence: docs/tasks/extraction-representation-gaps/table-reading-leaves.md (.67); generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json (table_0092); .project-data/tmp/erg-67/census.py
reverify: "In AXI's SourceIR, table_0092 (Table A6.5) has 25 cells, each a declared collective or a slice of one; AXI EvidenceIR statement_1958 cites Table A6.5 by number and no record cites it"
---

AXI *Table A6.5 Signals that should be the same in an exclusive sequence* (`table_0092`) is a five-column grid of 25
cells, each a declared collective (`AxID`, `AxADDR` … `AxMMUSSIDV`) or a slice of one (`AxCACHE[1:0]`). The caption
states the expectation. The sentence before it states the consequence and cites the table by number: *If any of
the signals shown in Table A6.5 are different between the read and write requests in an exclusive sequence, the
exclusive write might fail even if the location has not been updated by another agent.* No record carries the table,
the caption or that sentence.

**Measured `2026-10-23` (31 chains).** It is the only table whose every cell is a declared name. The tables most of
whose cells are declared names are parity check tables (AXI *A16.4*), which `check_signal_records` owns. A sentence
citing a table by number is common (43 in 5 chains), but the others cite encodings and field layouts.

**Decided (`.67`).**
- **The record:** EvidenceIR `signal_sets`, one per table (continuations joined by caption) whose every non-empty
  cell names a declared signal, collective or slice of one, with at least four cells. It holds the caption, the
  members in reading order (as written, with name and bits) and `citing_statement_ids`: the statements naming the
  table by its caption number (`.67.a`).
- **The rule stays as written:** the caption and citing sentences state it; the record gives them their operands. An
  equality between two transactions' values has no SemanticIR rule form and no `.isf` form.
- **Not lowered:** one adapter residual per set (`.67.b`).
