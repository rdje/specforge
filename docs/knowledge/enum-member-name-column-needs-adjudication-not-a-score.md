---
id: enum-member-name-column-needs-adjudication-not-a-score
title: An enum member is named from the column the document names it in — the signal seam's calibrated score, transposed FAITHFULLY, already refuses AxSIZE's byte counts; what it cannot refuse alone is a bit-position value column, a repeated-name grid, and a multi-key table
answers:
  - "which column names an enum member"
  - "why does an enum member have a whole sentence for a name"
  - "can the signal-table name-column override be reused for enums"
  - "why is a content score not enough to pick the member name column"
  - "what is a multi-key encoding table"
  - "why can AXI Table A7.3 not be a flat enum"
  - "how many minting tables repeat a code in their value column"
  - "which replayable enums publish a colliding code"
  - "why does PPROT have members named 0 and 1"
  - "why is HSIZE BYTE equal to HALFWORD"
  - "what happens when the value column is one bit of a split field"
  - "how do I re-derive the .5.xiii population"
  - "why could the 262-table upper bound not be reproduced"
  - "why does AxBURST publish FIXED instead of FIXED_BURST"
  - "why is AWSNOOP Table A7.3 not published as an enum"
  - "what does adjudicate_member_name_column do"
  - "why did RRESP ARTAGOP AWTAGOP start minting"
  - "which tables does the member name-column override refuse"
date: 2026-09-22
status: current
tags: [intentir, enums, extraction-quality, kg-isf-completeness, fidelity, measurement, wire-based-100]
evidence: crates/specforge/src/ir/evidence.rs (adjudicate_member_name_column, read_whole_name_token, NAME_COLUMN_OVERRIDE_MARGIN, infer_encoding_column_indices); crates/specforge/test_data/kg_quality/enum_member_name_column_gold; scripts/adjudicate_enum_member_name_column.py (--landed); docs/research/enum-member-name-column.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5.xiii, .5.xiii.a)
reverify: "python3 scripts/adjudicate_enum_member_name_column.py --landed — expect 4416 contested LEGACY pairs: notation 3644, score 736, document 27 pairs / 26 tables, multi_key 9 pairs / 6 tables (the replayable stratum is measured by evidence --dry-run over both binaries, not by this locator); cargo test -p specforge-core --lib kg_isf_completeness_5_xiii_a — expect 6 passed; target/debug/specforge kg-bench enum_member_name_column_gold — expect pass. The dated .5.xiii measurement re-derives only at c317f6bd (before the AXI chain was rebuilt with document names): python3 scripts/adjudicate_enum_member_name_column.py — expect population 1,177 pairs / 445 tables / 35 documents with 12 replayable tables; 82 pairs / 66 tables multi-key (7.0%); 654 of 5,456 minting tables multi-key corpus-wide (12.0%); 19 pairs / 14 tables whose value column is one bit of a split field; and 266 enums already publishing a colliding code, of which 9 replayable — HPROT 6, AWSNOOP 5, BTAGMATCH 5, HSIZE 4, ARSNOOP 3."
---

**LANDED by `.5.xiii.a` (`2026-09-22`).** `adjudicate_member_name_column` (`evidence.rs`) decides, per
table, which column names an encoding table's members. The header's choice stands unless all four
gates pass; content shape only (ADR 0006):

1. the header chose a **descriptive** column (`meaning`/`description`) — a `Name` header is never
   contested;
2. every value cell on a minting row is written as a **code** (`looks_like_encoding_literal`) — a
   register FIELD table read as an enum carries a bit position (`[31]`), a range or a width there;
3. another column's **net score** — identifier-shaped names (`read_whole_name_token`, shared with the
   signal seam) that determine one code, minus rows whose name labels several codes — leads the
   header column's by `NAME_COLUMN_OVERRIDE_MARGIN` (2), which refuses `RW`/`RO` access columns and a
   lane-remap grid;
4. if the elected column puts two names on one code the table is **multi-key** and mints nothing.

## The `.5.xiii` instrument was not a faithful transposition — the AxSIZE "refutation" is its artefact

`.5.xiii` reported that a content score *"cannot separate a member-name column from a quantity
column"*, because AxSIZE's `Label` (`1 2 4 … 128`) out-scored its prose. Its instrument scored the
**sanitized** cell; the signal seam it claimed to transpose also requires the token to be
**identifier-shaped** (`is_hardware_signal_token`), and under that test the column scores **0**. The
faithful score refuses AxSIZE, AHB `HSIZE`'s `Size (bits)` and BTAGMATCH `table_0178`'s `0b0`/`0b1`
on its own. What the score genuinely cannot refuse was found by reading the corpus population, not the
twelve replayable pairs: bit-position value columns (gate 2) and repeated-name columns (gate 3).

## Effect, replayed

`evidence --dry-run` over all 78 chains with both binaries: 27 replayed, 26 byte-identical, AXI
`ihi0022_l` **−29 / +46** member statements — every re-named member the document's own name
(`FIXED`/`INCR`/`WRAP`, `CLR`…`UMIN`, `CLEANINVALID`…, `NONE`/`SEPARATE`/`FAIL`/`PASS`), Table A7.3
and A7.4 withdrawn as multi-key, and **three enums recovered** that the `.5.ii` spine gate had emptied
because every `Meaning` cell is a sentence: `RRESP` (8), `ARTAGOP` (4), `AWTAGOP` (4). The instrument
cannot see a recovery — it enumerates only enums that are already minted — so the replay, not the
instrument, is the authority on reach.

Oracle: `kg-bench` fixture `enum_member_name_column_gold`, RED on the pre-change binary.

## Superseded body (`.5.xiii`, dated)

The rest of this card is `.5.xiii`'s measurement as it stood. Its figures still re-derive from the
default report; its conclusion that the score alone is wrong on AxSIZE is corrected above.

`infer_encoding_column_indices` picks `name_col` as the FIRST header matching
`name|meaning|description|state|transfer|response|type`, so a prose `Description` column outranks the
symbolic column that names the member.

## Do not invent a shape test — this repository already calibrated one

`synthesize_signal_declarations` solves the same problem for SIGNAL tables:
`name_cell_is_read_whole` decides name-versus-prose, the score is DISTINCT names, and
`NAME_COLUMN_OVERRIDE_MARGIN = 2` is the lead content needs to overrule a header. Reuse it; it needs no
new vocabulary or constant (ADR 0006). [[qualified-role-header-proves-no-role]]

**The handed-down "262 tables / 34 documents" cannot be reproduced** — its shape thresholds were never
recorded, and re-deriving from the prose lands anywhere in 255-268 tables / 32-33 documents. A shape
test with unrecorded thresholds is not a measurement.

## But the score alone is WRONG IN BOTH DIRECTIONS

| table | the score's pick | truth |
| --- | --- | --- |
| AXI `A3.9` AxBURST | `Label` = `FIXED`, `INCR`, `WRAP` | **right** — the document's own names |
| AXI `A3.2` AxSIZE | `Label` = `1 2 4 8 16 32 64 128` | **wrong** — byte COUNTS; `Enum AxSIZE 1 = 0.` is worse than `1_BYTE_PER_TRANSFER` |

A content score cannot separate a member-name column from a QUANTITY column, so the repair must be
adjudicated per table and the override may not ship alone.

## 12% of minting tables cannot be a flat enum

AXI `Table A7.3` has 13 rows and **8 distinct AWSNOOP codes**, with `0b00000` on five of them:
`(AWSNOOP, AWDOMAIN) -> opcode`. Re-pointing the name column would emit five members equal to 0 —
trading a naming defect for the value collision `.5.vii` calls *provably wrong*.

**82 of the 1,177 population pairs (7.0%) are multi-key; corpus-wide it is 654 of 5,456 minting tables
— 12.0%.** A multi-key table must mint a RESIDUAL, not an enum.

## Nine replayable enums already publish a colliding code, all in the wire golds

They survive `.5.vii.a`'s fallback removal **because the cell does parse** — the value is
document-stated and simply read from the wrong column:

```text
Enum PPROT 0 = 0.   Enum PPROT 0 = 1.   Enum PPROT 1 = 0.   Enum PPROT 1 = 1.
```

AHB `HSIZE` is the same shape: its table is `HSIZE[2] | HSIZE[1] | HSIZE[0] | Size (bits) |
Description`, so `value_col` matched the TOP BIT and `BYTE`, `HALFWORD`, `WORD` and `DOUBLEWORD` all
come out as 0. HPROT collides on 6 of 8 members. **`value_col` inference, not `name_col`, is the defect
here** — owned by `.5.xiv`.

Related: [[minted-enum-values-are-mostly-row-indices]],
[[minted-enums-are-mostly-cross-table-conflations]],
[[decimal-before-binary-needs-a-notational-witness]].
