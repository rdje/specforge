# Which column names an enum's members — and the three defects that share one symptom (`KG-ISF-COMPLETENESS.5.xiii`)

`infer_encoding_column_indices` picks `name_col` as the FIRST header matching
`name|meaning|description|state|transfer|response|type`, so a prose `Description` column outranks the
symbolic column that actually names the member. Reproduce every figure below with:

```bash
python3 scripts/adjudicate_enum_member_name_column.py
```

## The handed-down population is not reproducible, and that is the first finding

`.5.xiii` was opened on an *"upper bound: 262 enum-minting tables across 34 documents"* whose shape test
— *"another column holds short, single-token, mostly-DISTINCT cells"* — was never recorded. Re-deriving
it from that description lands anywhere in **255-268 tables / 32-33 documents** depending only on the
length cut-off and the distinctness fraction:

| cell length cut-off | distinctness | tables | documents |
| ---: | ---: | ---: | ---: |
| 24 | 0.80 | 255 | 32 |
| 28 | 0.80 | 258 | 32 |
| 32 | 0.80 | 260 | 32 |
| 24 | 0.75 | 268 | 33 |

The published 262/34 sits inside that band. **A shape test with unrecorded thresholds is not a
measurement**, so this instrument declares no new threshold at all.

## It reuses the test this repository already calibrated

`synthesize_signal_declarations` answers the same question for SIGNAL tables, and has been corrected
against the corpus twice. `name_cell_is_read_whole` (`PROSE-NAME-CELL-DECLARATION.2`) decides whether a
cell is a NAME or prose left over; the column score is the count of DISTINCT names it yields; and
`NAME_COLUMN_OVERRIDE_MARGIN = 2` is the lead content needs before it may overrule a header —
deliberately not 1, because *"a prose column reaches it whenever one more sentence happens to open with
a capitalised word"*. Transposed to the member seam this needs no new vocabulary, constant or threshold
(ADR 0006).

**Population under that test: 1,177 (table, enum) pairs / 445 tables / 35 documents**, of which **12
replayable pairs** — and every one of those twelve is in an AXI or AHB WIRE-BASED-100 gold document.

## All twelve replayable pairs, adjudicated by hand

The leaf warned this is an upper bound *"because the shape test alone cannot tell a member-name column
from an incidental one"*. It is worse than that: **the shape test is wrong in both directions.**

| table | today | the override's pick | verdict |
| --- | --- | --- | --- |
| AXI `A3.9` AxBURST | `FIXED_BURST=0` from `Meaning` | `Label` = `FIXED`, `INCR`, `WRAP` | **repair correct** — those are the document's own names |
| AXI `A3.2` AxSIZE | `1_BYTE_PER_TRANSFER=0` from `Meaning` | `Label` = `1`, `2`, `4`, `8`, `16` … | **repair WORSE** — the label column is byte counts, and a member named `1` is not a name |
| AXI `A7.3`/`A7.4` AWSNOOP/ARSNOOP | prose from `Description` | `Opcode` = `WriteNoSnoop`, … | names right, but the table is MULTI-KEY (below) |
| AHB `3-2` HSIZE | `BYTE=0`, `HALFWORD=0` … | `Size (bits)` | neither — the VALUE column is one bit of a split field |
| the remaining 7 (AWATOP, AxDOMAIN, AWCMO, BTAGMATCH ×2, AxMMUSECSID, AWSNOOP `table_0134`) | prose | `Label`/`Operation`/`BComp` | repair correct on 6; `BTAGMATCH table_0178` is multi-key |

**AxSIZE is the case that refutes a blanket rule.** Its `Label` column holds `1 2 4 8 16 32 64 128` —
transfer sizes in bytes, which score as eight distinct "names" and beat the prose column by the margin,
yet `Enum AxSIZE 1 = 0.` is strictly worse than `Enum AxSIZE 1_BYTE_PER_TRANSFER = 0.`. A content score
cannot tell a member-name column from a quantity column, so `.5.xiii.a` must adjudicate per table and
may not ship the override alone.

## The table that cannot be a flat enum at all

AXI `Table A7.3` has **13 rows and 8 distinct AWSNOOP codes**; `0b00000` appears on **five** of them:

```text
0b00000 | NSH, SYS      | WriteNoSnoop      0b00001 | NSH, SYS | WriteNoSnoopFull
0b00000 | SH            | WriteUniquePtl    0b00001 | SH       | WriteUniqueFull
0b00000 | NSH, SH, SYS  | Atomic            0b00010 | -        | RESERVED
0b00000 | NSH, SYS      | WriteExclusive    0b00011 | SH       | WriteBackFull
0b00000 | SYS           | WriteACT
```

`AWSNOOP` alone does not determine the operation; `(AWSNOOP, AWDOMAIN)` does. **Re-pointing the name
column here would emit five members all equal to 0** — trading a naming defect for the value collision
`.5.vii`'s census already calls *provably wrong, since no encoding assigns one code to two members*.

**Measured: 82 of the 1,177 pairs (7.0%), in 66 tables, are multi-key.** Across the whole minting
population it is **654 of 5,456 tables — 12.0%, one minting table in eight.**

## The defect this measurement was not looking for

Asking which enums already publish a colliding code finds **266, of which 9 are REPLAYABLE — and all
nine are in the AMBA wire golds**:

| document | enum | members | colliding |
| --- | --- | ---: | ---: |
| AHB `ihi0033_c` | `HPROT` | 8 | **6** |
| AXI `ihi0022_l` | `AWSNOOP` | 8 | 5 |
| AXI `ihi0022_l` | `BTAGMATCH` | 9 | 5 |
| AHB `ihi0033_c` | `HSIZE` | 6 | 4 |
| AXI `ihi0022_l` | `ARSNOOP` | 8 | 3 |
| AXI `ihi0022_l` | `ARCACHE` / `SYSCOACK`, APB `ihi0024_e` `PPROT` | 10 / 4 / 4 | 2 each |

These survive `.5.vii.a`'s fallback removal **precisely because the cell does parse** — the value is
document-stated, it is simply read from the wrong column. APB `PPROT` is the clearest case and it is
published today:

```text
Enum PPROT 0 = 0.    Enum PPROT 0 = 1.    Enum PPROT 1 = 0.    Enum PPROT 1 = 1.
```

Two member names, each carrying two different codes. AHB `HSIZE` is the same shape: its table is
`HSIZE[2] | HSIZE[1] | HSIZE[0] | Size (bits) | Description`, so `value_col` matched the TOP BIT and
every member's code is that bit — `BYTE`, `HALFWORD`, `WORD` and `DOUBLEWORD` all come out as 0.
**19 pairs in 14 tables have a value column that is one bit of a split field**, 2 of them replayable.

This is a distinct defect from `.5.xiii` — the NAME column is not the problem — and it is live in the
documents the project's hardest gate scores. It is opened as `.5.xiv` rather than folded in here.

## Decision

1. **`.5.xiii.a` ships a per-table adjudication, not the override alone.** AxSIZE proves a content
   score cannot separate a member-name column from a quantity column, and 7.0% of the population cannot
   be a flat enum in the first place.
2. **The multi-key table mints a residual**, not an enum. That answers the question `.5.xiii` was
   opened with, using its own anchor as the evidence.
3. **`.5.xiv` owns the colliding codes**, because the value column — not the name column — is wrong
   there, and because 9 replayable enums in the wire golds is the largest measured fidelity defect the
   `.5` programme currently has open.

## Authoritative tracking

`docs/tasks/KG-ISF-COMPLETENESS.md` — `.5.xiii` (this measurement), `.5.xiii.a` (the naming repair),
`.5.xiv` (the colliding codes). Instrument:
`scripts/adjudicate_enum_member_name_column.py`. Knowledge Map:
`[[enum-member-name-column-needs-adjudication-not-a-score]]`,
`[[qualified-role-header-proves-no-role]]`, `[[minted-enum-values-are-mostly-row-indices]]`.
