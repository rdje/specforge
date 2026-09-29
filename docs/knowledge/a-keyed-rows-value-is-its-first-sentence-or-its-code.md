---
id: a-keyed-rows-value-is-its-first-sentence-or-its-code
title: A two-cell row keyed on a declared signal (| X | Must be … |) states its value in its first sentence, or in the one parenthesized code that encodes a compound description
answers:
  - "why was AXI ARCACHE's DVM Complete constraint not read"
  - "what is keyed_row_value_predicate"
  - "does a keyed row with a second sentence yield a constraint"
  - "why did ARBURST have two identical constraints"
  - "how does the keyed-row reader treat a comma in the value"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.b.iii.f"
date: 2026-09-29
status: current
tags: [evidence-ir, signal-constraints, keyed-rows, tables, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (signal_keyed_obligation_row, signal_keyed_obligation_value_is_readable, keyed_row_value_predicate, extract_signal_keyed_obligation_row_constraints, dedup_appended_signal_constraints); docs/book/src/pipeline/evidence-failure-modes.md; docs/tasks/extraction-representation-gaps/table-census-leaves.md (.3.b.iii.f)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib the_three_guards_each_refuse_their_own_row an_established_repeat_of_the_same_statement_is_one_record; then AXI's evidence_ir.json signal_constraints under condition 'DVM Complete request' hold ARCACHE must_be_value 0B0010, ARMMUVALID 0B0, and exactly one ARBURST (target_value INCR)"
---

`EXTRACTION-GAP-FIX.5b` reads a two-cell Markdown row whose key is a declared signal and whose other cell opens with
*must be* (`| ARSNOOP | Must be 0b1110 . |`) as that signal's `must_be_value`, after three guards: exactly two cells,
no comma, no disjunction or leading *equal to*. A trailing `( … )` is stripped first because it encodes the value
(`Shareable ( 0b01 or 0b10 )` is *Shareable*).

Since `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.f` (`2026-09-29`), `keyed_row_value_predicate` reads the value from the
cell's first sentence, so AXI's `ARMMUVALID | Must be 0b0 . If not present, …` yields `0b0` instead of being refused by
its second sentence's comma. When the words before a parenthesis are compound and the parenthesis holds exactly one
code literal, the code is the value: `ARCACHE | Must be Modifiable, Non-cacheable ( 0b0010 )` is `0b0010`. A compound
with no code is still refused, so *Modifiable* alone is never published.

The keyed-row reader and the dynamic reader both read such a row (`ARBURST | Must be INCR ( 0b01 )`), and both belong
to the "established" surface `dedup_appended_signal_constraints` preserves. A record that repeats another from the same
statement, counting a missing `target_value` as the kind's own value, is now dropped, and the kept one takes the target.
AXI's *DVM Complete request constraints* table yields 12 of its 14 rows; `ARID` and `ARSIZE` state relations.
See also [[captured-region-residual-carrier]].
