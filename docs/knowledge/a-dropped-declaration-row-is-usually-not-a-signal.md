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
  - "should a declaration's identity survive a width the reader cannot read"
  - "why are MMU-700's LTI observation-interface signals missing from its catalog"
  - "is the SIGQUAL column a width"
  - "does SpecForge read a width written as N bit or N bits"
  - "how many times has the identity-without-attribute question been answered"
date: 2026-09-15
status: current
tags: [evidence-ir, declarations, recall, precision, adr-0006, census-method, signal-declaration-row-drop]
evidence: scripts/measure_dropped_row_offerings.py; crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations, DeclarationRowDropReason); crates/specforge/src/ir/semantic.rs (read_explicit_signal_declaration); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.1, .1d, .1e, .4b, .4d, .4e); docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md (.6)
reverify: "Semantic-stage leg (.4d): `./target/release/specforge replay-declarations --evidence-root generated/evidence_ir --json` — expect 186 `width_text_unread` refusals, 69 distinct unrecovered identities, of which 47 are MMU-700 rows whose `Bits` column states the width; and `width <N> bit(s)` matching 9 refusals in ONE document (CXS) recovering 1 identity. Evidence-stage leg: python3 scripts/measure_dropped_row_offerings.py — expect 4 documents carrying the accounting, 71 dropped rows, 71 joined (100.0%), 59 no_direction_and_no_width (36 prose / 15 names-another-signal / 8 nothing) and 12 name_not_an_identifier. A document rebuilt since gains accounting and changes these counts; the adjudication must then be redone."
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

## The same question, answered a third time — one stage later (`.4d`, `2026-09-15`)

`SIGNAL-DECLARATION-ROW-DROP.4d` put the question to the SemanticIR reader instead of the
EvidenceIR one: should a declaration whose WIDTH text cannot be read keep its identity and the
direction already parsed? All **69** distinct identities the current reader refuses that way were
adjudicated against their source rows (`replay-declarations`, the real reader over all 78 artifacts).
**NO again**, and for a sharper reason than precision.

* **47 are MMU-700's LTI observation interface, and the document STATES their width.** `table_0259`
  (*"Table B-6: LTI TBU observation interface signals"*) and `table_0260` share the header
  `SIGNALGRP<n> | Bits | Signal name | SIGQUAL<n> 4'b{MSB..LSB} | Number of cycles of delay`, and a row
  reads `0 | [125:110] | latlbloc | 3'b000 , lavalid | 1`. The reader takes the **fourth** column as
  the width and never reads the **second** — `[125:110]` is 16 bits, one cell away. So admitting the
  identity without a width would permanently hide a stated width. These belong to
  `SIGNAL-DECLARATION-ROW-DROP.2f`, whose `Unused` blocker is the first body row of both tables.
* **18 are not signals**: Avalon section headings, AXI-H transaction names, GICv3 peripheral-ID
  register FIELDS, eMMC `NOTE`, and `group` from the prose *"Signal group output ports are present on
  each component"* — which reaches this arm at all only because `output` parses as a direction.
* **4 are genuine losses with a width the reader mis-parses**, three of them malformed in the source.
  The fourth, `CXSACTIVEREQ is width 1 bit`, prompted its own measurement: `width <N> bit(s)` matches
  **9 refusals in ONE document**, 7 of whose 8 identities are already read elsewhere, so the grammar
  recovers **one** identity corpus-wide and **zero** in the current stratum. Measured and refused.
* **20 of the 47 carry a spelling the document never writes** — `LCVALID_0`…`_7`, `LCCTAG_0`…`_7` and
  four index-7 singletons, per-bit expansions of buses written `lcvalid[7:0]`.

**The rule this establishes, now on three independent measurements (24%, 1-in-8, and this one): an
identity with no readable attribute is not a signal, and a reader that cannot read the attribute
should be taught the notation rather than allowed to drop the attribute.** Reaching for
identity-only admission is reaching past a fix that is usually sitting in the next column.

See `[[declaration-replay-reads-the-legacy-stratum]]`, `[[arithmetic-width-drops-the-declaration]]`.
