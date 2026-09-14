---
id: a-dropped-declaration-row-is-usually-not-a-signal
title: Admitting a declaration with an identity but no attribute is 24% precise — 14 real rows against 45 phantoms — and every table in the population is uniformly real or uniformly phantom
answers:
  - "should a table row declare a signal when it states no direction and no width"
  - "what do the rows the declaration reader drops actually offer"
  - "how many dropped declaration rows are real signals"
  - "why is LOOP_W_WIDTH not declared as a signal"
  - "why are SINGLE INCR and WRAP4 not signals"
  - "is the declaration drop a recall bug or correct refusal"
  - "how many documents carry declaration_row_accounting"
  - "can I classify the 482 dropped rows from the runtime accounting"
  - "what would admitting an identity without an attribute cost"
  - "is the signal-or-not decision a row property or a table property"
  - "which tables would recover real signals if identity alone were enough"
  - "does every column header being a name header separate a signal grid from a legend"
date: 2026-09-14
status: current
tags: [evidence-ir, declarations, recall, precision, adr-0006, census-method, signal-declaration-row-drop]
evidence: scripts/measure_dropped_row_offerings.py; crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations, DeclarationRowDropReason); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.1, .1d, .1e); docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md (.6)
reverify: "python3 scripts/measure_dropped_row_offerings.py — expect 4 documents carrying the accounting, 71 dropped rows, 71 joined (100.0%), 59 no_direction_and_no_width (36 prose / 15 names-another-signal / 8 nothing) and 12 name_not_an_identifier. A document rebuilt since gains accounting and changes these counts; the adjudication must then be redone."
---

`synthesize_signal_declarations` refuses a row that offers neither a direction nor a width, and that
refusal is the whole of the measured 18.3% declaration loss. The tempting reading is that it is pure
recall debt — a row names a signal, so the signal exists. Measured, it is mostly not.

## The population is the reader's own accounting

`SIGNAL-DECLARATION-ROW-DROP.1` made the drop countable at runtime, so
`extraction_manifest.declaration_row_accounting` records every row the real reader really dropped, with
the reason. **Only documents rebuilt since that shipped carry it — 4 of the 27 proof-carrying ones** —
giving **71 dropped rows, 100% joined** back to their source row: 59 `no_direction_and_no_width` and 12
`name_not_an_identifier`. `.0`'s corpus-wide figure of 482 comes from a direct scan of all 78 persisted
artifacts and is a **different population**; never sum them.

## 14 real against 45 phantom

| table | rows | what it is | admitting it |
| --- | ---: | --- | --- |
| AXI `table_0092` | 4 | a signal-name GRID — every cell a real AXI signal (`AxLEN`, `AxSIZE`, …) | **real** |
| ADIv6 `table_0058` | 5 | pin equivalence — `SWDIOTMS \| SWDIO \| TMS` | **real** |
| ADIv6 `table_0039` | 3 | `signal \| programmers' model`, rotated — `CDBGPWRUPREQ` … | **real** |
| ADIv6 `table_0041` | 2 | `DBGTDO`, `DBGTRSTn` | **real** |
| AXI `table_0199` | 10 | a PAS encoding — `Secure`, `Root`, `Realm`, `SA` | phantom |
| ADIv6 `table_0108` | 10 | a garbled body — name cells are `Out`, `TDI Out`, `TDO In` | phantom |
| AHB `table_0014` | 8 | the HBURST encoding — `SINGLE`, `INCR`, `WRAP4` | phantom |
| AHB `table_0019` | 6 | the HPROT encoding — `Opcode fetch`, `Data access` | phantom |
| AXI `table_0265` | 6 | a LEGEND — `Y`, `YM`, `YS`, `O`, `NS` | phantom |
| AXI `table_0183`/`0184` | 5 | PARAMETER tables — `LOOP_W_WIDTH`, `USER_REQ_WIDTH` | phantom |

**24% precision.** A rule admitting an identity with no attribute would mint 45 phantoms in the current
stratum alone — width parameters as wires, encoding names as wires, a legend as a catalog — which is
what `.2a`'s placeholder refusal and `[[agent-pure-inferred-phantom-drop]]`'s orthography rule exist to
stop. **The refusal is right.**

## The decision is a TABLE property, and two shape tests already failed

Every one of the ten tables is **uniformly** real or **uniformly** phantom; not one has a mix. So a
row-level rule cannot win and a table-level one might — the same conclusion
`[[declared-population-is-not-the-candidate-row-population]]` reached about eMMC's bus-mode matrix from
the other direction.

Two table-level discriminators were tried against these ten and **both fail**:

- *every column header is a name header* selects only ADIv6 `table_0058`. AXI `table_0092`'s headers are
  literal signal names (`axid`, `axaddr`) and match no name keyword at all.
- *every body cell is an identifier* admits AXI `table_0265`, whose cells are `Y | Mandatory |
  Mandatory` — `Mandatory` is a lone word the identifier test accepts.

What does hold over the four real tables is that their cells come from the same vocabulary as the
document's own declared catalog. That is a **grounding** test rather than a shape test, and it is the
one left to measure (`SIGNAL-DECLARATION-ROW-DROP.1e`).

Links: [[declared-population-is-not-the-candidate-row-population]],
[[a-width-cell-that-is-a-sentence-is-not-a-width]].
