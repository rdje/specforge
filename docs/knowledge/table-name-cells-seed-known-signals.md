---
id: table-name-cells-seed-known-signals
title: EvidenceIR's known signal names are seeded from every signal-table row's first word, so a shifted table's words became signals
answers:
  - "why did ADIv6 have an enum named In"
  - "why did ADIv6 have relations like Your reads In"
  - "what seeds EvidenceIR's known signal names"
  - "what does collect_signal_names_from_tables return"
  - "why did CoreSight TMC have relations on Chip, Write and Address"
  - "can a word in a signal table's name column become a known signal"
  - "where is inferred_name_is_an_ordinary_word applied"
date: 2026-09-29
status: current
tags: [evidence-ir, signals, known-signals, converter-damage, enums, relations, adr-0006]
evidence: crates/specforge/src/ir/evidence.rs (collect_signal_names_from_tables, inferred_name_is_an_ordinary_word, the convergence loop's known_signals); docs/tasks/extraction-representation-gaps/census-leaves.md (.3.e.i, .3.c.viii)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_word_in_a_signal_tables_name_column_is_not_a_known_signal; then python3 over generated/intent_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/intent_ir.json: symbol_definitions holds only Size (no In), and actor_signal_relations names no signal In or Out"
---

`collect_signal_names_from_tables` gives the convergence loop its first *known signal names*: for every table
treated as a top-level signal description, the first whitespace token of each body row's first cell, kept when it is
an identifier. It runs whether or not the row declares anything, and the set feeds the polarity, relation, semantic
hint and signal-anchor enum readers (and the visual-motif matcher). A declaration is NOT made here; the declaration
reader runs separately and drops a row that states no direction and no width.

So a converter-damaged signal table leaks words. ADIv6 *Table C3-1 JTAG Access Port JTAG port signals* arrives with
its columns shifted (`Out | Test Clock`, `TDI Out | Test Data In`, a bare `In`), and CoreSight TMC *Table 2-1 Memory
interface signals* with name and description swapped (`Chip enable, active LOW. | MEMCEN`). Until
`EXTRACTION-REPRESENTATION-GAPS.3.e.i` (`2026-09-29`) `In`, `Out`, `Chip`, `Write`, `Address` were known signals: ADIv6
carried 76 relations on `In`/`Out`, 21 hints, an `Out` polarity and an enum `In` minted by the signal-anchor scan from
two unrelated tables (bank-select rows and JEP106 field widths), which reached the `.isf` text; TMC carried 16 junk
relations. The actors those relations named (66 in ADIv6, 13 in TMC) and the behaviours whose every actor was one of
them (165, 3) existed only because of it.

The seed now skips a token `inferred_name_is_an_ordinary_word` recognises: an initial capital over a lower-case
remainder, the orthography that selected 3 of 3 relation-inferred phantoms and 0 of 1,605 table-declared names
(`ACTOR-NOUN-RELATION-DECLARATION.1`). Its doctrine forbids it on a table *declaration*, where the document's spelling
is authoritative; the seed declares nothing, and a declared wire re-enters the set through its declaration
statement. Two chains moved, junk only; every wire-gold score was unchanged. The real signals of both damaged tables
(`RTCK`, `MEMCEN`, `MEMADDR`, `MEMWEN`, `MEMD`, `MEMQ`) remain undeclared: `.3.c.viii`. See also
[[captured-region-residual-carrier]].
