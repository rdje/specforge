---
id: a-table-of-declared-names-is-a-rules-operand-set
title: A table whose every cell is a declared signal name is a rule's operand set (AXI A6.5); it becomes EvidenceIR signal_sets
answers:
  - "how is AXI Table A6.5 Signals that should be the same in an exclusive sequence read"
  - "what is EXTRACTION-REPRESENTATION-GAPS.67 / .67.a / .67.b"
  - "what is EvidenceIR signal_sets / citing_statement_ids"
  - "which tables list only declared signal names"
  - "where does a rule that cites a table by number (signals shown in Table A6.5) get its operands"
date: 2026-10-28
status: current
tags: [evidence-ir, tables, rules, signal-sets, extraction-representation-gaps, measured]
evidence: docs/tasks/extraction-representation-gaps/table-reading-leaves.md (.67, .67.a, .67.b); generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json (signal_set_0001); .project-data/tmp/erg-67b/replay.txt
reverify: "Replay AXI's IntentIR through specforge adapt --target isf --dry-run; require isf_protocol_signal_set_signal_set_0001 to name all 25 members, the caption and statement_1958, with manager.isf unchanged"
---

AXI *Table A6.5 Signals that should be the same in an exclusive sequence* (`table_0092`) is a five-column grid of 25
cells, each a declared collective (`AxID`, `AxADDR` … `AxMMUSSIDV`) or a slice of one (`AxCACHE[1:0]`). The caption
states the expectation. The sentence before it states the consequence and cites the table by number: *If any of
the signals shown in Table A6.5 are different between the read and write requests in an exclusive sequence, the
exclusive write might fail even if the location has not been updated by another agent.* Before `.67.a`, no record
carried the table, the caption or that sentence.

**Measured `2026-10-23` (31 chains).** It is the only table whose every cell is a declared name. The tables most of
whose cells are declared names are parity check tables (AXI *A16.4*), which `check_signal_records` owns. A sentence
citing a table by number is common (43 in 5 chains), but the others cite encodings and field layouts.

**Decided (`.67`).**
- **The record (read since `.67.a`):** EvidenceIR `signal_sets` (`read_signal_sets`, `signal_set_surface`), one per
  table (continuations joined by caption) whose every non-empty cell names a declared signal, collective or slice of
  one, with at least four cells. It holds the caption, the members in reading order (as written, with name and bits)
  and `citing_statement_ids`: the statements naming the table by its caption number (`names_table`). AXI's is
  `signal_set_0001`, citing `statement_1958`; it reaches IntentIR unchanged.
- **The rule stays as written:** the caption and citing sentences state it; the record gives them their operands. An
  equality between two transactions' values has no SemanticIR rule form and no `.isf` form.
- **Not lowered:** since `.67.b`, one `isf_protocol_signal_set_<id>` adapter residual per set names its caption, every
  member as written, and its citing statements and tables; the rendered `.isf` does not change.
