---
id: a-split-name-cell-does-not-say-which-join-it-wants
title: `t PERIOD` and `a opcode` are the same cell shape and want DIFFERENT joins — only the document's own spelling separates them, and it does so 2 of 126 with zero false positives
answers:
  - "can a rule join two adjacent tokens in a name cell"
  - "how many name cells in the corpus are a split identifier"
  - "is a subscript split the same defect as an underscore split"
  - "why can a shape-only token-join rule not be shipped"
  - "how do I tell a concatenation join from an underscore join"
  - "what does scripts/measure_subscript_split_name_cells.py measure"
  - "how many TileLink name cells look like a split identifier"
  - "which corpus documents declare from a subscript-split name cell"
  - "does eMMC table_0221 declare from t PERIOD"
  - "why is a concatenating join more dangerous than an underscore join"
  - "what stops AWSIZE, ARSIZE or HSELx a from being joined"
date: 2026-09-14
status: current
tags: [extraction, declaration, name-cell, text-layer, census, text-layer-identifier-split]
evidence: scripts/measure_subscript_split_name_cells.py; scripts/measure_split_identifier_name_cells.py; docs/tasks/TEXT-LAYER-IDENTIFIER-SPLIT.md (.0, .2)
reverify: "python3 scripts/measure_subscript_split_name_cells.py — expect 573 boundary tables; name-column tier A 9 cells / 2 reachable, tier B 126 cells / 2 reachable; 81 of tier B TileLink with 0 reachable; the two reachable cells eMMC table_0221 r0c0 and r1c0. Read-only, seconds."
---

A PDF text layer loses the typography that joined an identifier, and SpecForge reads two tokens where
the document wrote one. The reader keeps the first token, so the row is not dropped and not refused —
it is **renamed**, and every row of the table is renamed to the same thing.

There are two such classes, and **their cells are indistinguishable**:

| the document writes | the text layer gives | the join |
| --- | --- | --- |
| `a_opcode` (TileLink) | `a opcode` | insert an **underscore** |
| `t`+subscript `PERIOD` (eMMC) | `t PERIOD` | **concatenate** |

`a opcode` and `t PERIOD` are the same shape: a one-character lead and a following word. Nothing in
the cell says which join it wants, and choosing wrong is silent — `aopcode` is not a name anyone can
trace back, and a concatenation leaves no mark of having guessed.

## Measured, over the whole corpus

`scripts/measure_subscript_split_name_cells.py`, read-only, over 573 boundary tables. Scope is the
column the reader actually reads (header-designated or reader-chosen), because no other cell can
become a declaration; 1,677 matches outside it across 38 documents are the false-positive surface.

| signature, in the NAME column | cells | with an in-document concatenation |
| --- | ---: | ---: |
| continuation is UPPER-CASE (`t PERIOD`) | **9** | **2** |
| continuation is any word (`t PERIOD` *and* `a opcode`) | **126** | **2** |

**81 of the 126 are TileLink** — `a opcode`, `b param`, `c valid`, … across both revisions' five
channel tables. Their correct join is the underscore, so a shape-keyed concatenation rule would be
wrong about essentially all of them.

## The document's own spelling is the discriminator, and it is exact

Asking whether the document anywhere writes the **concatenation** selects **2 of 126 and is right
about both**: eMMC `table_0221` `t PERIOD` → `tPERIOD` and `t TLH , t THL` → `tTLH`/`tTHL` (one cell
carrying a comma family *and* two splits). Every one of TileLink's 81 is refused — correctly, because
TileLink never writes `aopcode`; its joining spelling is `a_opcode`, and `TEXT-LAYER-IDENTIFIER-SPLIT.0`
found even that only as vector text inside three figures the ingest stores as images.

**Zero false positives against a counter-population of 81 with the same shape and the opposite
answer** is the strongest discrimination anything in this area has shown. The general rule: *ask the
document which spelling it uses; never infer a join from the shape of the gap.*

## It still does not ship, and the reason is the same one that stopped `.0`

The entire correct population is **one table in one document, and that document is legacy**
(`jesd84_b50`, unrebuildable). A rule would change **zero declarations in the current stratum**, so the
next document to exercise it would be its first test rather than its hundredth. `.0` refused a rule
that was right about 3 of 120 and inert; this one is right about 2 of 126 and inert for the same
reason.

## What keeps the legitimate shapes safe

A lead of at most **two characters**. Verified directly against the tokenizer: `AWSIZE, ARSIZE`,
`HSELx a`, `ARMPAM [10:0]`, `PADDR [31:0]`, `Duty Cycle` and `Clock source` select **nothing** at
either tier. A three-character-or-longer first token is a word, and a word followed by a word is
prose — which is the whole subject of [[a-dropped-declaration-row-is-usually-not-a-signal]].

eMMC's remark column also writes `C DEVICE` for `CDEVICE`, so the class is not confined to the `t`
family; a remark cell cannot declare, so it is counted and not acted on.

Links: [[evidence-statement-markdown-escape-truncates-identifiers]].
