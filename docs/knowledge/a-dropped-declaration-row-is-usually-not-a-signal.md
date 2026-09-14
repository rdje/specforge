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
  - "is AxLEN a declared AXI signal"
  - "why is a grounding test against the declared catalog circular"
  - "does the Ax metavariable name a wire"
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
| ADIv6 `table_0058` | 5 | pin equivalence — `SWDIOTMS \| SWDIO \| TMS` | **real** |
| ADIv6 `table_0039` | 3 | `signal \| programmers' model`, rotated — `CDBGPWRUPREQ` … | **real** |
| ADIv6 `table_0041` | 2 | `DBGTDO`, `DBGTRSTn` | **real** |
| AXI `table_0199` | 10 | a PAS encoding — `Secure`, `Root`, `Realm`, `SA` | phantom |
| ADIv6 `table_0108` | 10 | a garbled body — name cells are `Out`, `TDI Out`, `TDO In` | phantom |
| AHB `table_0014` | 8 | the HBURST encoding — `SINGLE`, `INCR`, `WRAP4` | phantom |
| AHB `table_0019` | 6 | the HPROT encoding — `Opcode fetch`, `Data access` | phantom |
| AXI `table_0265` | 6 | a LEGEND — `Y`, `YM`, `YS`, `O`, `NS` | phantom |
| AXI `table_0183`/`0184` | 5 | PARAMETER tables — `LOOP_W_WIDTH`, `USER_REQ_WIDTH` | phantom |
| AXI `table_0092` | 4 | the `Ax` METAVARIABLE — `AxLEN`, `AxSIZE`; `AXLEN` is undeclared while `AWLEN`/`ARLEN` are | phantom |

**17% precision.** A rule admitting an identity with no attribute would mint 49 phantoms in the current
stratum alone — width parameters as wires, encoding names as wires, a legend as a catalog — which is
what `.2a`'s placeholder refusal and `[[agent-pure-inferred-phantom-drop]]`'s orthography rule exist to
stop. **The refusal is right.**

## The decision is a TABLE property, and two shape tests already failed

Every one of the ten tables is **uniformly** real or **uniformly** phantom; not one has a mix. So a
row-level rule cannot win and a table-level one might — the same conclusion
`[[declared-population-is-not-the-candidate-row-population]]` reached about eMMC's bus-mode matrix from
the other direction.

**Three table-level discriminators were tried against these ten and all three fail:**

- *every column header is a name header* selects only ADIv6 `table_0058`. AXI `table_0092`'s headers are
  literal signal names (`AxID`, `AxADDR`) and match no name keyword at all.
- *every body cell is an identifier* admits AXI `table_0265`, whose cells are `Y | Mandatory |
  Mandatory` — `Mandatory` is a lone word the identifier test accepts.
- *grounding* — are the cells drawn from the document's own declared catalog? — **earned its keep by
  falsifying a hand verdict** (AXI `table_0092` scored 0.00 and was right to: the cells are
  metavariables) and is then refuted for two stated reasons. It is **circular** for a table that
  declared its own signals — those score against a catalog they fed, reaching 1.00 — and with declaring
  tables excluded the actionable population is **11 non-declaring tables**: 9 at 0.00, one at 0.12 (a
  garbled phantom), **one at 0.55** (ADIv6 `table_0058`, real), none between. Any threshold in
  `(0.12, 0.55]` selects exactly one table — a rule fitted to a single positive. It is also
  structurally blind to the partially-declaring tables, which are where recall would come from.

**What remains: ~10 recoverable rows in three ADIv6 tables against 49 phantoms**, and no vocabulary-free,
non-circular table-level test that separates them. Reopen only with a discriminator that survives all ten.

Links: [[declared-population-is-not-the-candidate-row-population]],
[[a-width-cell-that-is-a-sentence-is-not-a-width]].
