---
id: a-check-coverage-table-binds-the-class-of-covered-signals
title: A check-coverage table is a typed relation (check signal to covered signals), and it resolves the class sentence about missing covered signals
answers:
  - "what is check_signal_records / CheckSignalRecord and where does it come from"
  - "how is a parity check-signal table (Check signal | Signals covered) read"
  - "how does SpecForge know which signals a check signal covers"
  - "how is 'the missing signals are assumed to be LOW' (signals covered by a check signal) bound now"
  - "what is sentence_names_relation / header_words"
  - "why does APB5's PPROT presence record carry absent_value LOW with a supporting_table_ids entry"
  - "does a sentence naming a signal outrank a class sentence for the same signal"
  - "why does a check table headed Name (AXI Table A16.4) still read"
  - "how is a footnote letter beside a check signal name (PSELxCHK b) handled"
  - "is a parity check signal lowered to isf (not yet: .46 decided per shape, .46.a-.46.c)"
  - "what is parity_sense / ParitySense / stated_parity_sense on check_signal_records (.46.a)"
  - "what is lower_parity_checks / isf_parity_checks_not_lowered / parity_<CHK> rules (.46.b)"
  - "why did adding check_signal_records re-seal every chain"
date: 2026-10-20
status: current
tags: [evidence-ir, parity, check-signal, signal-presence, absent-value, table, carried-evidence, adr-0006, adr-0016]
evidence: crates/specforge/src/ir/evidence.rs (CheckSignalRecord; check_coverage_tables; check_signal_surface; sentence_names_relation; bind_stated_absent_values); crates/specforge/src/ir/semantic.rs and intent.rs (check_signal_records); docs/archive/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.ii.a.i); docs/gaps/specforge.md (S25)
reverify: "cargo test --offline -p specforge-core --lib check_coverage && python3 -c \"import json; e=json.load(open('generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json')); print(len(e['check_signal_records']), sum(1 for r in e['signal_presence_records'] if r.get('absent_value')))\"   # expect 14 15"
---

Parity specifications state which signals each check signal protects in a table headed `Check signal | Signals
covered | Width | Granularity | Check enable` (APB5 *Table 5-1*, AHB5 *Table 12-1*; AXI *Table A16.4* heads the
check column `Name` and has no granularity). Since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a.i` (`2026-10-03`)
EvidenceIR carries each row as a `CheckSignalRecord { check_id, check_signal, covered_signals, width,
granularity, check_enable, table_id }` in `evidence.declaration`, carried unchanged by SemanticIR and IntentIR.
The gate is structural: a header cell whose words are exactly *signals covered*, and the nearest preceding
column headed by a cell containing *signal* or *name*; width, granularity and enable columns are found by their
header word and kept verbatim (`ceil(ADDR_WIDTH/8)`, `PSEL & PWRITE`, AXI's literal `None`). A name cell's
trailing single lowercase letter is a footnote mark and is dropped (`PSELxCHK b`); a covered cell splits on commas
and spaces; a row whose covered cell holds anything but identifiers is refused and keeps its position out of the
ids (`check_table_0014_001`).

**The same reading resolves the class sentence.** `.3.d.ii.a` could not bind *If some of the signals covered by
a check signal are not present on an interface, then the missing signals are assumed to be LOW* because the
sentence names no presence record. `sentence_names_relation` matches the sentence's absence clause against the
table's two headers by words: every word of `Check signal` and `Signals covered` must appear (`check`, `signal`,
`signals`, `covered`), so the vocabulary is the document's own header, not a list (ADR 0006). Each covered signal
with a presence record then takes the literal, citing the statement and, in `supporting_table_ids`, the table it
came through. A sentence naming the signal itself outranks a class sentence for that signal; within one scope,
disagreeing statements still bind nothing.

**Measured `2026-10-03`.** 28 of 31 chains identical at EvidenceIR. AXI 123 records over five page fragments
(166 distinct covered signals; its class sentence states *the default for that signal*, no literal, so nothing
binds). APB5 14 records, 15 presence rows take `LOW` (16 covered; `PADDR`'s matrix row is refused for a fused
`Y Y` cell, see `.45`). AHB5 16 records, 22 rows take `LOW`. The unread `assumed_by_word` family falls
109 -> 107.

**Not lowered.** `.isf` has `(^ A B)` but no parity form; a check signal is a combinational XOR chain over its
covered signals under its enable. `EXTRACTION-REPRESENTATION-GAPS.46` decided per shape (`2026-10-20`): the
documents' stated parity sense (*Odd parity is used*) is read first (`.46.a`); the 104 one-bit checks over signals of
fixed width lower as one rule `(! (^ …))` under a readable enable (`.46.b`); the 49 byte-granular checks over a
parameterised width need a per-byte reduction `.isf` cannot state and are requested from FSMGen (`.46.c`). Since
`.46.a` every record carries `parity_sense` (`odd` on all 153) and `parity_statement_ids`, from `stated_parity_sense`:
*odd/even parity is used* or *… means* in a non-apparatus, non-table statement; both senses give none. Since
`.46.b` the adapter lowers a one-bit check as `(rule parity_<CHK> <enable> (<CHK> (! (^ …))))` in the actor that
drives its covered signals (`lower_parity_checks`): every covered bit, a covered width only when stated as a number
(`stated_numeric_widths`; the adapter's width 1 is also a default), the enable as declared signals or the actor's
reset; the rest go to one `isf_parity_checks_not_lowered` packet by reason. AXI 45 of 123, APB5 2 of 14, AHB5 1 of
16, FSMGen-strict clean. The
records carry no residual each: a check signal is interface shape, like a presence row, not a protocol observation.

**Cost.** `check_signal_records` joins `EVIDENCE_RULE_FIELDS`, inside the EvidenceIR seal closure
([[stage-seal-closure-membership]]), so every chain re-seals from EvidenceIR down; the three table-bearing chains
are rebuilt by hand and the rest by `scripts/rebuild_stage_cascade.sh --write`.

Links: [[a-signals-stated-absent-value-rides-its-presence-record]], [[signal-presence-matrix-capture]],
[[property-table-is-not-a-signal-inventory]].
