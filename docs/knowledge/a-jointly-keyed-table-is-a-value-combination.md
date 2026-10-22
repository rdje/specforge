---
id: a-jointly-keyed-table-is-a-value-combination
title: A table whose rows bind several declared signals' or fields' values to a member or a meaning is an EvidenceIR value combination, not one key's enum; a member column titled ...state names protocol states
answers:
  - "what is EvidenceIR value_combinations (ValueCombinationRecord, CombinationKey, CombinationRow)"
  - "how are APB PNSE | PPROT[1] | Physical address space and AXI A15.26 SYSCOREQ | SYSCOACK read"
  - "why did AXI have a BTAGMATCH, SYSCOACK or AxPROT enum with repeated codes (the single-column reader read a jointly keyed table)"
  - "which column titles are keys (signal, bit or slice, collective, REG.FIELD, capitalized register field)"
  - "how is AHB's per-bit HPROT table read (one signal's bits as keys, .3.c.iv.a)"
  - "why is A4.4 ARCACHE | AWCACHE not a value combination (one collective's members: one key per channel)"
  - "why does A4.7 keep its AxPAS enum while also being a value combination"
  - "where do the Coherency Connection states Disabled, Connect, Enabled, Disconnect come from (merge_combination_states, supporting_combination_ids)"
  - "what is read_value_combinations / combination_key_name / is_combination_member_name / merge_combination_states"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b and .3.g.iii.b.i"
  - "does value_combinations reach IntentIR and the ISF adapter (.3.g.iii.b.ii: carried verbatim; one isf_protocol_value_combination residual per record)"
  - "what is CombinationRow.value_encodings / CombinationValueEncoding / bind_combination_value_encodings"
  - "can .isf state a decode of a value combination (FSMGen strict probe: an enum plus one guarded rule per row writing actor storage, 0 diagnostics)"
  - "why is a value combination not lowered to .isf (clocked-only assignments, invented storage target; .3.g.iii.b.iii)"
  - "how is a key title with a footnote mark read (AXI A7.4 ARDOMAIN 1, .62)"
  - "are AXI A7.3 AWSNOOP and A7.4 ARSNOOP encodings value combinations (the opcode decode)"
  - "why may RESERVED repeat in a combination's member column"
  - "how does a domain list NSH, SYS reach AxDOMAIN's codes (CombinationKey.note, note_label_definitions, .63)"
date: 2026-10-22
status: current
tags: [evidence-ir, encodings, value-combinations, protocol-states, multi-key-tables, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (ValueCombinationRecord; read_value_combinations; combination_key_columns; combination_key_name; without_note_number; note_numbers_by_page; footnote_texts_by_page; note_label_definitions; defined_labels_encoding; is_combination_member_name; merge_combination_states; value_combination_surface; scan_encoding_tables_by_signal_anchor's combination_tables skip; bind_combination_value_encodings); crates/specforge/src/ir/semantic.rs and intent.rs (value_combinations carried); crates/specforge/src/ir/adapters.rs (the value_combination residual); docs/tasks/extraction-representation-gaps/encoding-and-state-leaves.md (.3.g.iii.b, .3.g.iii.b.i)
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
  capitals (`TYPE`); a leading *Value of* is dropped. Since `.3.c.iv.a`, distinct bits of one signal count too,
  when a bit cell is no plain bit (AHB *3-5*'s `HPROT[3] Modifiable` … with `-` marks; *3-7*), and a title may
  name its bit after the slice. Every column is titled and none only by a number. Keys that are
  all members of one collective (A4.4's `ARCACHE | AWCACHE`) are one key per channel, left to `.3.g.iii.c.i`.
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
byte-identical.

**Carried (`.3.g.iii.b.ii`, `2026-10-22`).** SemanticIR and IntentIR carry `value_combinations` verbatim
(`semantic.carried_evidence`, `intent.carried_semantics`). After the convergent loop,
`bind_combination_value_encodings` joins each key value that is a label of its key's encoding to that encoding's
codes (`CombinationRow.value_encodings`), the way a stated absent value is joined. It tries the key's own enum, then
its collectives, then, for a collective key, its members' enums. AXI had 34 joined values (`AxMMUSECSID`
`Non-secure` -> `0b00`; `AxDOMAIN` `Shareable` -> `0b01`, `0b10`; `AxPAS` `Non-secure` -> `0b001` since `.3.g.iii.b.iv`
matches a member written with its abbreviation, `Non-secure (NS)`). Lists of alternatives and classes of codes join
nothing. The ISF adapter states one `isf_protocol_value_combination_*`
residual per record: 13 across AXI 7, ADIv6 3, RISC-V IOMMU 2 and APB 1. Every `.isf` text is unchanged. An FSMGen
strict probe lowered APB's decode as an enum for the member column plus one rule per row guarded by the keys' values,
writing actor storage, with 0 diagnostics.

**Not lowered (`.3.g.iii.b.iii`, `2026-10-22`).** Every `.isf` assignment is clocked, so a decode in storage adds a
register, a one-cycle lag and an invented storage target (ADR 0016). A combination is a guard vocabulary instead
(a condition naming a member resolves to its keys' values), built when a condition needs it; none does on the
current chains. Since `.3.g.iii.b.iv` each combination residual says so, and a state a combination keys names
that combination as the statement of its encoding instead of listing the encoding as missing.

**A footnote-marked key (`.62`, `2026-10-22`).** AXI *A7.3*/*A7.4* title their domain key `AWDOMAIN 1`/`ARDOMAIN 1`.
`combination_key_name` now drops a trailing mark (`without_note_number`) when a note beside the table, or beside a
fragment its caption names, uses it (note numbers and footnote letters). A table's notes follow its last fragment:
*A7.3*'s note 1 is two pages after its head. A reserved row names no member, so `RESERVED` may repeat in the member
column. Measured on 31 chains: 30 identical; AXI 7 -> 9 records, *A7.3* (25 rows, 3 fragments) and *A7.4* (19 rows,
2), member `Opcode`; 15 residuals across 4 documents. Records are renumbered by table order (*A15.26* is
`value_combination_0009`).

**A note's labels (`.63`, `2026-10-22`).** A marked key keeps its note (`CombinationKey.note`): the one text
footnotes with that mark print beside the table's fragments (`footnote_texts_by_page`). `note_label_definitions`
reads a note that is wholly `<label> is <name> [( codes )]` items, and `defined_labels_encoding` joins a value
listing such labels (`NSH, SYS`): each label resolved by its name, all in one table, a printed code equal to the
table's. AXI's joined values go from 34 to 69: all 35 domain values of *A7.3*/*A7.4* (`NSH, SYS` -> `0b00`, `0b11`
in `table_0111`); the other 30 chains are identical.
