---
id: a-jointly-keyed-table-is-a-value-combination
title: A table whose rows bind several declared signals' or fields' values to a member or a meaning is an EvidenceIR value combination, not one key's enum; a member column titled ...state names protocol states
answers:
  - "what is EvidenceIR value_combinations (ValueCombinationRecord, CombinationKey, CombinationRow)"
  - "how are APB PNSE | PPROT[1] | Physical address space and AXI A15.26 SYSCOREQ | SYSCOACK read"
  - "why did AXI have a BTAGMATCH, SYSCOACK or AxPROT enum with repeated codes (the single-column reader read a jointly keyed table)"
  - "which column titles are keys (signal, bit or slice, collective, REG.FIELD, capitalized register field)"
  - "why is A4.4 ARCACHE | AWCACHE not a value combination (one collective's members: one key per channel)"
  - "why does A4.7 keep its AxPAS enum while also being a value combination"
  - "where do the Coherency Connection states Disabled, Connect, Enabled, Disconnect come from (merge_combination_states, supporting_combination_ids)"
  - "what is read_value_combinations / combination_key_name / is_combination_member_name / merge_combination_states"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b and .3.g.iii.b.i"
date: 2026-10-21
status: current
tags: [evidence-ir, encodings, value-combinations, protocol-states, multi-key-tables, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (ValueCombinationRecord; read_value_combinations; combination_key_columns; combination_key_name; is_combination_member_name; merge_combination_states; value_combination_surface; scan_encoding_tables_by_signal_anchor's combination_tables skip); docs/tasks/extraction-representation-gaps/encoding-and-state-leaves.md (.3.g.iii.b, .3.g.iii.b.i)
reverify: "cargo test --offline -p specforge-core --lib a_jointly_keyed_table_is_a_value_combination"
---

Some tables key each row by several signals' or fields' values at once. APB's untitled `PNSE | PPROT[1] | Physical
address space` names an address space per pair of bits, and AXI *A15.26* names the Coherency Connection states by
`SYSCOREQ | SYSCOACK`. No single column is an enum there. The single-column encoding reader took one value column
anyway: AXI got `BTAGMATCH` (7 members on 4 codes), `SYSCOACK` (`DISABLED = 0`, `CONNECT = 0`, …) and, from *A4.7*,
`AxPROT` (`SECURE = 0`, `ROOT = 0`). SemanticIR dropped all 15 statements, so none reached an `.isf`.

**The record (`EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.i`, `2026-10-21`).** `read_value_combinations` runs in each
convergence pass and makes one `value_combinations` record per table:

- **Keys** (`combination_key_columns`): two or more columns titled by different declared names. A name is a signal, a
  bit or slice of one, a collective, a register's field (`CSW.ERRSTOP`), or a field a register declares, written in
  capitals (`TYPE`); a leading *Value of* is dropped. Every column is titled and none only by a number. Keys that are
  all members of one collective (A4.4's `ARCACHE | AWCACHE`) are one key per channel, left to `.3.g.iii.c`.
- **Member and meanings:** the member column is the first other column whose cells are distinct names (no full stop,
  no bullet, more than a number). The other columns are meanings. Every cell is kept as written.
- **Continuations** join the table their caption names when the headers match (A15.26 spans `table_0239` and
  `table_0240`). A table with an empty key cell is refused, and the manifest's `jointly_keyed_tables` entry counts
  it (`produced` vs `kept`).

**Precedence.** `scan_encoding_tables_by_signal_anchor` skips a combination's tables. `.3.g.iii.a`'s column reader is
not restricted, because it mints only a column whose codes determine the member, so A4.7 keeps `AxPAS`.

**States.** A member column whose title ends in *state* names protocol states. `merge_combination_states` merges
them by name, case-insensitively, citing the row's table and the combination (`supporting_combination_ids`).

**Measured `2026-10-21` (31 chains):** 13 records in 4 documents, none refused. AXI-L has 7 (A4.7, A9.1, A12.15,
A13.10, A13.11, A13.14, and A15.26 with its continuation), ADIv6 3, RISC-V IOMMU 2 and APB 1. AXI-L gains 4 protocol
states (4 ISF residual packets) and loses the 15 false statements. That frees its real `BTAGMATCH` encoding (*A12.14*,
`table_0177`), which SemanticIR had dropped over the conflicting codes: `manager.isf` gains `(BTAGMATCH (NONE 0)
(SEPARATE 1) (FAIL 2) (PASS 3))`, FSMGen strict-clean. The other 30 chains' later stages and `.isf` texts are
byte-identical. The record stops at EvidenceIR until `.3.g.iii.b.ii` carries it.
