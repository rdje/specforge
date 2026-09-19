# Per-row direction-column drift — the census two leaves were blocked on

Owning leaf: `SIGNAL-DECLARATION-ROW-DROP.2j.1` (MEASURE + ROUTE; no rule, no production change).
Producer: `python3 scripts/measure_direction_column_drift.py`.

## Why this census exists

Two leaves of this tree were waiting on the same number.

- **`.2j`** closed its rule half by refusing the direction-abbreviation arm — 837 rows write the full
  word, exactly one usable cell writes an abbreviation — and handed on a second deliverable: *size
  ADIv6 `table_0108`'s column garble and route it to the ingest tree*.
- **`.2h.2`** is blocked outright on *"a corpus census of per-row layout drift"*, because CoreSight
  TMC `table_0074` is **mixed, not rotated** (six rows name-last, the seventh name-first), so `.2e`'s
  whole-table offset cannot serve it. One table is not a grammar, and `.2h.0` paid for that lesson.

Both need the same thing: how many tables put their columns in a different order on different rows?

## What the census measures, and why it refuses a name heuristic

The obvious census asks which column holds the signal NAME and whether that column is stable. **Two
name tests were tried against this corpus and both failed, in opposite directions:**

| test | what it misses | what it wrongly accepts |
| --- | --- | --- |
| the cell is one identifier-shaped token | `PWDATA_S [31:0]`, `sample_req ,`, `PRDATA_S Output` | — |
| the cell's FIRST token is identifier-shaped | — | `Test Clock`, `Port Connected`, `Return Clock` |

The error was not small. The first cut of this census reported **30 tables / 116 rows**, the second
**11 rows**, and neither survived reading its own selection: description prose whose first word is an
ordinary English word is indistinguishable from a name by shape alone.

So the census is built out of the one vocabulary in a signal table that is **closed and
unambiguous** — the direction values themselves (`Input`, `Output`, `In`, `Out`, `Inout`, …), matched
whole-cell. A table whose direction cell sits in a different column on different rows is exactly the
table whose columns drifted, whatever its name column is doing, and no judgement is required.

**A uniform offset is not drift.** A table where every row agrees is CONSISTENT whether the direction
is in the first column, the last, or the one the header names — that is what `.2e` already serves.
Only disagreement *between rows of one table* is reported.

**The header is not consulted, and that matters.** An earlier scoping restricted the census to tables
whose header names a direction. It found seven tables and **missed `.2h.2`'s own table**: TMC
`table_0074` heads its direction column `Type`.

## The population

```bash
python3 scripts/measure_direction_column_drift.py
```

**571 `signal_description` tables: 169 consistent, 393 with no whole-cell direction value at all, and
9 DRIFTED** — across 5 documents, 91 body rows, 40 declarations, **51 rows with none**.

| document | table | rows | declared | undeclared | direction column |
| --- | --- | ---: | ---: | ---: | --- |
| AXI/ACE `ihi0022_h_c` | `table_0036` | 18 | 7 | 11 | `{0: 7, 1: 7, none: 4}` |
| ADIv6 `ihi0074_a` | `table_0108` | 10 | 0 | **10** | `{0: 4, 1: 1, none: 5}` |
| AXI/ACE `ihi0022_h_c` | `table_0037` | 18 | 9 | 9 | `{0: 3, 1: 15}` |
| CoreSight `ihi0029_e` | `table_0040` | 8 | 0 | **8** | `{0: 2, 1: 2, none: 4}` |
| SDC-600 `101130_0002_02` | `table_0059` | 8 | 2 | 6 | `{0: 3, 1: 2, none: 3}` |
| SDC-600 `101130_0002_02` | `table_0048` | 7 | 4 | 3 | `{0: 5, 1: 1, none: 1}` |
| SDC-600 `101130_0002_02` | `table_0056` | 7 | 5 | 2 | `{0: 1, 1: 5, none: 1}` |
| SDC-600 `101130_0002_02` | `table_0034` | 8 | 7 | 1 | `{0: 1, 1: 7}` |
| TMC `ddi0461_b` | `table_0074` | 7 | 6 | 1 | `{0: 6, 1: 1}` |

**`.2j` sized this at "9 of the 22 rows, in one table".** The population is **9 tables in 5
documents**, and ADIv6 `table_0108` is one of them.

## `rows − declared` is an upper bound, and it can also understate the defect

It is an **upper bound** because a body row may be a note, a continuation, or a merged
row that was never a declaration. It is **not a loss count**, and it must not be published as one.

It can also be far too kind, and TMC `table_0074` is the proof. It reads *one* undeclared row:

```text
Signal | Type | Description
Output   | Valid signals in this cycle from the master      | ATVALIDM
Input    | If there is valid data, that is, …               | ATREADYM
Output   | Trace source ID                                  | ATIDM[6:0]
Output   | Number of valid bytes on ATDATA , …              | ATBYTESM a
Output   | Trace data, LSB aligned                          | ATDATAM b
Input    | Any data remaining in any buffers …              | AFVALIDM
AFREADYM | Output                                           | Data flush complete, …
```

Six declarations come out of it, and **four of the six are wrong**:

```text
Signal ATVALIDM is input.     <- the table says Output
Signal ATREADYM is input.     <- correct
Signal ATBYTESM is input.     <- the table says Output
Signal ATDATAM  is input.     <- the table says Output
Signal AFVALIDM is input.     <- correct
Signal DATA     is input.     <- DATA is no signal of this table
```

`ATIDM[6:0]` and `AFREADYM` are **missing**, and `DATA` is **published although it is no signal of
this table**. The direction is not a blanket default either: this document's own declarations split
**15 `input` / 8 `output`**.

## `.2j.1a` — two corrections to the paragraph above, from the director's audit

The first version of this section published two things it had not earned, and both are corrected here
rather than quietly edited away.

**1. The phantom's MECHANISM was asserted, not established.** It read *"`DATA` is minted — from the
word *data* in a description cell"*. The declaration's statement (`statement_2047`) carries **no
evidence span at all**, so nothing in the artifact records where the name came from, and at least two
accounts fit the same observation equally: the word *data* in row 4's *"Trace data, LSB aligned"*, and
the token `ATDATA` in row 3's *"Number of valid bytes on ATDATA ,"*. `CLAIM_VERIFICATION.md` §3:
evidence consistent with both hypotheses illustrates, it does not test. **The mechanism is therefore
recorded as unestablished and gated as such** — a RED case asserts the statement has no span, so the
claim cannot quietly return. What is established, and is separately gated, is that `DATA` is
published and is not a signal of this table.

**2. The "not a blanket default" figures belonged to a different document.** *"121 `input` and 101
`output`, and its own `table_0034`"* are CoreSight **SDC-600**'s numbers and SDC-600's table, cited
here as if they were the TMC's. The TMC's own split is **15 / 8**, which supports the same conclusion
on this document's own evidence — which is what the claim needed in the first place.

**An automated phantom test does not work here and is not published.** Asking whether a declared name
appears anywhere in its own table returns zero phantoms, because the English word *data* supplies the
token. The adjudication is hand-read, declaration by declaration, and now pinned by four RED cases so
it is re-checked on every run rather than re-read by hand.

## ADIv6 `table_0108`, adjudicated in full — `.2j`'s own deliverable

`Table C3-1 JTAG Access Port JTAG port signals`, header `Signal | Direction a | Description | Notes`,
ten body rows, **zero declarations**:

| row | name cell | what happened |
| ---: | --- | --- |
| 0 | `Out` | name absent from the row; `TCK` sits in a `row_span`-4 Notes cell reading `TCK TMS` |
| 1 | `Out` | name absent; `TMS` is the second name in that same shared cell |
| 2 | `TDI Out` | name and direction fused into one cell |
| 3 | `TDO In` | name and direction fused |
| 4 | `Out` | name absent; `TRST*` sits in a Notes cell reading `TRST* Active LOWJTAG IEEE …` |
| 5 | `RTCK In` | name and direction fused; description shifted right |
| 6 | `nSRSTOUT` | **structurally correct** — lost only to the direction-abbreviation refusal |
| 7 | `In` | name absent; `SRSTCONNECTED` sits in a shared `row_span`-2 Notes cell |
| 8 | `PORTCONNECTED In` | name and direction fused |
| 9 | `PORTENABLED In` | name and direction fused; description split across two cells |

**Nine rows garbled, one row correct** — and the correct one is exactly the single usable
abbreviation cell `.2j`'s census found, refused because `Out` is not a full direction word. `.2j`
adjudicated that refusal corpus-wide and kept it: one row is not a grammar.

The nine split cleanly by whether the identifier survived into its row at all:

- **five rows keep the name, fused with the direction** (`TDI Out`, `TDO In`, `RTCK In`,
  `PORTCONNECTED In`, `PORTENABLED In`) — a cell-splitting failure;
- **four rows do not carry their name at all** — it is inside a row-spanning Notes cell shared with
  its neighbours, and mapping `TCK TMS` back onto rows 0 and 1 needs positional evidence from the
  page rather than anything in the artifact.

## Routing

**To `SIGNAL-DECLARATION-ROW-DROP.2h.2`**, whose stated prerequisite was exactly this census. It is
no longer blocked, and it inherits three things it did not have: the population is **9 tables, not
one**; its own table's defect is **wrong-direction and phantom declarations**, not only the six rows
`.2h.1` gave up; and the two sub-shapes above say which rows a reader could repair and which need the
page.

**Not to `PDF-VARIANT-DIGESTION`**, which declares no eligible frontier and requires a future
continuation to be scoped as a new top-level activity. **Not to `TEXT-LAYER-IDENTIFIER-SPLIT`**, whose
defect is a *single* identifier split by a missing underscore — a different failure from a row whose
columns are in the wrong order.

## Re-derivation

```bash
python3 scripts/measure_direction_column_drift.py              # the census
python3 scripts/measure_direction_column_drift.py --rows       # every drifted table, row by row
python3 scripts/measure_direction_column_drift.py --self-test  # 9/9 RED cases
```

The self-test pins the shape — 9 tables, 5 documents, 91 rows, 40 declarations, 51 with none — plus
four discriminations a count cannot give: the direction test is whole-cell, a row with two direction
cells contributes no opinion rather than its first, a uniform offset is not drift, and drift is
disagreement between rows whatever the header says.
