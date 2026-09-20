---
id: direction-column-drift
title: Nine signal tables put their direction in a different column on different rows — eight of them under the rule vocabulary — and the defect is not only dropped rows: TMC table_0074 emitted six declarations of which one was minted from a description cell and three contradicted the table
answers:
  - "which signal tables have per-row column drift"
  - "how many tables does a whole-table column offset fail to serve"
  - "why can a name-shape heuristic not find a drifted table"
  - "what is wrong with ADIv6 table_0108"
  - "why does ADIv6 table_0108 produce no signal declarations"
  - "does column drift only lose rows or does it also fabricate declarations"
  - "where does the phantom signal DATA in the CoreSight TMC come from"
  - "is the DATA phantom mechanism established"
  - "which cell did the CoreSight TMC DATA phantom come from"
  - "how many drifted tables can a production rule reach"
  - "why does the drift census count nine tables and the rule eight"
  - "what is the CoreSight TMC document direction split"
  - "what unblocks SIGNAL-DECLARATION-ROW-DROP.2h.2"
  - "how do I re-derive the direction column drift census"
  - "is rows minus declarations a loss count"
date: 2026-09-20
status: current
tags: [signal-declaration-row-drop, evidence-ir, table-extraction, census, adjudication, ingest]
evidence: scripts/measure_direction_column_drift.py; docs/research/direction-column-drift-census.md; docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.2j, .2j.1, .2h.2)
reverify: "python3 scripts/measure_direction_column_drift.py --self-test — expect 22/22, pinning 9 drifted tables across 5 documents (91 body rows, 40 declarations, 51 rows with none) under the census vocabulary and 8 tables across 4 documents (81 rows) under the production reader vocabulary; the four discriminations (whole-cell direction test, two direction cells give no opinion, a uniform offset is not drift, drift is row disagreement whatever the header says); the five adjudicated-instance cases, one of which still pins that the phantom statement has NO evidence span; and the four cases that establish the phantom source cell and refute both accounts the audit named."
---

A `signal_description` table is read on the assumption that a column means the same thing on every
row. **Nine tables across five documents break it**, putting the direction value in a different
column on different rows — and a whole-table offset (`SIGNAL-DECLARATION-ROW-DROP.2e`) cannot serve
them by construction.

Measured over **571** `signal_description` tables: **169 consistent**, **393 with no whole-cell
direction value at all**, **9 drifted** — 91 body rows, 40 declarations, **51 rows with none**.
`.2j` had sized this as *"9 of the 22 rows, in one table"*; ADIv6 `table_0108` is one of the nine.

## A name heuristic cannot find them, and that is measured

| test | misses | wrongly accepts |
| --- | --- | --- |
| the cell is one identifier-shaped token | `PWDATA_S [31:0]`, `sample_req ,`, `PRDATA_S Output` | — |
| the cell's FIRST token is identifier-shaped | — | `Test Clock`, `Port Connected`, `Return Clock` |

Two cuts of this census reported 30 tables / 116 rows and then 11 rows; neither survived reading its
own selection. The census is therefore built from the **direction values alone** — a closed set,
matched whole-cell, needing no judgement. The header is not consulted either: a header-scoped version
missed TMC `table_0074`, which heads its direction column `Type`.

## The defect is not only dropped rows

`rows − declared` is an **upper bound** — a body row may be a note or a continuation — and TMC
`table_0074` shows it can also understate. It reads *one* undeclared row and in fact emits six
declarations of which **four are wrong**: `ATVALIDM`, `ATBYTESM` and `ATDATAM` are published as
`input` where the table says `Output`, and `DATA` is published although **it is no signal of that
table**, while `ATIDM[6:0]` and `AFREADYM` go missing. The direction is not a blanket default: this
document's own declarations split **15 `input` / 8 `output`**.

**`.2j.1a` corrected two things the first version published without earning them**, and one of those
corrections has since been discharged. It said `DATA` was *"minted from the English word data"*; the
declaration's statement carries **no evidence span**, and the token `ATDATA` in a neighbouring
description fit the observation equally, so the mechanism was recorded as unestablished. The
*"121 input / 101 output"* figures were CoreSight **SDC-600**'s, cited as if they were the TMC's;
this document's own split is 15/8.

**`.2h.2` established the mechanism and refuted both accounts.** The reader does not scan a cell for
a name — it takes the cell's **first whitespace token** — so the decidable question is which cells
could have produced the name at all, and there is exactly one: **row 6, column 2**, the DESCRIPTION
cell of the single name-first row, reading `Data flush complete, AFVALID can be deasserted`. The two
accounts the audit named begin `Trace` and `Number`. Column 2 is where `.2e`'s whole-table override
put the name column for every row of this table, which is the whole mechanism. Four RED cases pin it
and a Rust test reproduces it on an alpha-renamed shape, where the phantom is still called `Data` —
which is what a name taken from English prose does and what a name taken from a signal token cannot.

## ADIv6 `table_0108`, in full

Ten rows, **zero declarations**. Nine are garbled and one — `nSRSTOUT | Out | Subsystem Reset` — is
structurally correct and lost only to the direction-abbreviation refusal `.2j` adjudicated
corpus-wide and kept. The nine split by whether the identifier reached its row at all:

- **five keep the name fused with the direction** (`TDI Out`, `TDO In`, `RTCK In`,
  `PORTCONNECTED In`, `PORTENABLED In`) — a cell-splitting failure a reader could repair;
- **four do not carry their name at all** — `TCK`, `TMS`, `TRST*` and `SRSTCONNECTED` sit inside
  row-spanning Notes cells shared with their neighbours, and rejoining them needs positional evidence
  from the page.

## Routing

This census was the stated prerequisite of `[[SIGNAL-DECLARATION-ROW-DROP]].2h.2`, which is now
**shipped** — see [[per-row-column-drift-rule]]. Of the nine tables, a production rule can reach
**eight**: ADIv6 `table_0108` writes only abbreviations, which no rule may read.

The census was deliberately **not** routed to `PDF-VARIANT-DIGESTION` (no eligible frontier; a
continuation must be scoped as a new top-level activity) or to `[[TEXT-LAYER-IDENTIFIER-SPLIT]]`
(one identifier split by a missing underscore is a different failure from a row whose columns are in
the wrong order).

One caveat this card owed. The `declared` counts come from the **persisted** `evidence_ir.json`,
and for a legacy proofless document that is not necessarily what the current binary produces: this
very table's artifact records `DATA` where the current reader emits `Data`. The 51 legacy chains stop
at `build_unproved_from_source_ir` and cannot be re-derived through the product's own path, so
`CORPUS-CHAIN-CURRENCY.11` has since measured how far the ones that CAN be checked have drifted:
22 of 51 — see [[legacy-artifact-declaration-drift]].

Links: [[a-cheap-structural-rule-overfires-until-you-read-its-selection]],
[[per-row-column-drift-rule]], [[legacy-artifact-declaration-drift]].
