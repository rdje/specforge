---
id: direction-column-drift
title: Nine signal tables put their direction in a different column on different rows, and the defect is not only dropped rows — TMC table_0074 emits six declarations of which one is minted and three contradict the table
answers:
  - "which signal tables have per-row column drift"
  - "how many tables does a whole-table column offset fail to serve"
  - "why can a name-shape heuristic not find a drifted table"
  - "what is wrong with ADIv6 table_0108"
  - "why does ADIv6 table_0108 produce no signal declarations"
  - "does column drift only lose rows or does it also fabricate declarations"
  - "where does the phantom signal DATA in the CoreSight TMC come from"
  - "what unblocks SIGNAL-DECLARATION-ROW-DROP.2h.2"
  - "how do I re-derive the direction column drift census"
  - "is rows minus declarations a loss count"
date: 2026-09-19
status: current
tags: [signal-declaration-row-drop, evidence-ir, table-extraction, census, adjudication, ingest]
evidence: scripts/measure_direction_column_drift.py; docs/research/direction-column-drift-census.md; docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.2j, .2j.1, .2h.2)
reverify: "python3 scripts/measure_direction_column_drift.py --self-test — expect 9/9, pinning 9 drifted tables across 5 documents, 91 body rows, 40 declarations and 51 rows with none, plus the four discriminations (whole-cell direction test, two direction cells give no opinion, a uniform offset is not drift, drift is row disagreement whatever the header says)."
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
`input` where the table says `Output`, and `DATA` is **minted from the English word *data*** in a
description cell, while `ATIDM[6:0]` and `AFREADYM` go missing. The direction is not a blanket
default: the same document emits 121 `input` and 101 `output`, and its own `table_0034` is correct
throughout. An automated phantom test does not catch `DATA`, because *data* supplies the token — the
adjudication is hand-read.

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

This census is the stated prerequisite of `[[SIGNAL-DECLARATION-ROW-DROP]].2h.2`, which was blocked
on it; that leaf is now unblocked and inherits a population of nine tables rather than one, plus a
named precision defect. It is deliberately **not** routed to `PDF-VARIANT-DIGESTION` (no eligible
frontier; a continuation must be scoped as a new top-level activity) or to
`[[TEXT-LAYER-IDENTIFIER-SPLIT]]` (one identifier split by a missing underscore is a different
failure from a row whose columns are in the wrong order).

Links: [[a-cheap-structural-rule-overfires-until-you-read-its-selection]].
