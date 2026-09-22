---
id: enum-member-name-column-needs-adjudication-not-a-score
title: A content score cannot pick an enum's member-name column — AxSIZE's `Label` column of byte counts beats its prose column and is strictly worse — and 12% of minting tables cannot be a flat enum at all
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
date: 2026-09-22
status: current
tags: [intentir, enums, extraction-quality, kg-isf-completeness, fidelity, measurement, wire-based-100]
evidence: crates/specforge/src/ir/evidence.rs (infer_encoding_column_indices, name_cell_is_read_whole, NAME_COLUMN_OVERRIDE_MARGIN); scripts/adjudicate_enum_member_name_column.py; docs/research/enum-member-name-column.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5.xiii)
reverify: "python3 scripts/adjudicate_enum_member_name_column.py — expect population 1,177 pairs / 445 tables / 35 documents with 12 replayable tables; 82 pairs / 66 tables multi-key (7.0%); 654 of 5,456 minting tables multi-key corpus-wide (12.0%); 19 pairs / 14 tables whose value column is one bit of a split field; and 266 enums already publishing a colliding code, of which 9 replayable — HPROT 6, AWSNOOP 5, BTAGMATCH 5, HSIZE 4, ARSNOOP 3."
---

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
