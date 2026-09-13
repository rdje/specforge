---
id: a-width-cell-that-is-a-sentence-is-not-a-width
title: A prose width cell is refused by a CONJUNCTION (>6 tokens AND a terminator before whitespace) — each bound alone refuses a real width, and the classes overlap on length
answers:
  - "how does SpecForge tell a parametric width from a description sentence"
  - "why is the prose-width rule two conditions and not a token threshold"
  - "what is the longest legitimate parametric width expression in the corpus"
  - "can a width expression contain a question mark"
  - "why is LTI_GPC == True ? 2:1 not refused as prose"
  - "how many width cells read as prose corpus-wide"
  - "does refusing a prose width lose a signal"
  - "why does TileLink declare signals named C, D, V and R"
  - "which tables have a name column that scores zero under name_cell_is_read_whole"
  - "does name_cell_is_read_whole cost any recall"
  - "why does TileLink lose its Signal column to its Type column"
  - "how do I measure the parametric width cell population"
  - "what boundary does a signal-declaration census need beyond table_kind signal_description"
  - "why did my declaration census join at 92.8 percent"
  - "can a table_kind unknown table declare signals"
  - "does effective_table_kind promote an unknown table"
  - "where does parse_table_width_hint_text refuse a cell"
  - "what did PROSE-NAME-CELL-DECLARATION.3 ship"
  - "is a width-only declaration with a fabricated width worth keeping"
date: 2026-09-14
status: current
tags: [evidence-ir, declarations, widths, census-method, adr-0006, legacy-artifacts, prose-name-cell-declaration]
evidence: scripts/measure_parametric_width_cell_shapes.py; scripts/measure_name_column_whole_cell_score.py; crates/specforge/src/ir/evidence.rs (width_expression_reads_as_prose; parse_table_width_hint_text; infer_signal_table_row_width_hint; name_cell_is_read_whole; effective_table_kind); docs/tasks/PROSE-NAME-CELL-DECLARATION.md (.3, .5)
reverify: "python3 scripts/measure_parametric_width_cell_shapes.py — expect CURRENT stratum 116 tables, 685 declaring rows, 195 parametric width cells, 0 refused as prose, join control 601/601 (100.0%); LEGACY 271 parametric and 47 refused as prose, of which 43 are TileLink. A join rate below 100.0% on the CURRENT stratum means this census has drifted from the reader, not that the reader is wrong."
---

A width cell is admitted as `WidthHint::Parametric` on one test: does it hold an ASCII letter. That is
right for every expression a specification writes — `ceil(DATA_WIDTH/8)`, `clog2(Num_RP_AR)`,
`LTI_MMU ? 8 : ceil(LTI_LRADDR_WIDTH/8)` — and it is why a table whose width column has been handed a
**description** declares the description as the wire's width:

```text
Signal C is width Operation code. Identifies the type of message carried by the channel. (Table 5.2).
```

## The rule is a conjunction, because neither bound survives alone

A cell reads as prose when it carries **more than six whitespace tokens** *and* a **sentence
terminator (`.`/`?`/`!`) immediately followed by whitespace**. Measured over every width cell the
reader examines corpus-wide, each condition applied alone refuses a real width:

| condition alone | the real width it refuses |
| --- | --- |
| `> 6 tokens` | `ceil((ID_R_WIDTH+1)/8) if ARIDUNQ is not present: ceil(ID_R_WIDTH/8)` (7); `LTI_MMU == True: 64 LTI_MMU == False: LTI_LRADDR_WIDTH` (8) |
| terminator before whitespace | `LTI_GPC == True ? 2:1`; `LTI_SSID_WIDTH > 0 ? 1:0`; `LTI_MMU ? 8 : ceil(LTI_LRADDR_WIDTH/8)` — `?` is a ternary operator |
| **both** | **none. 47 selected corpus-wide, every one prose.** |

**The classes overlap on length, so no threshold separates them.** Legitimate expressions reach **8**
tokens; the shortest prose the rule selects is **7** (`Unique, per-link master source identifier.
(Section 5.4)`). An earlier reading of this margin — "widest legitimate 5, narrowest prose 7" — was
measured on the proof-carrying stratum alone and does not re-derive over the corpus. A footnote marker
rides its expression directly (`ceil(ADDR_WIDTH/8) a`), carries no terminator, and is untouched.

Shape only, no vocabulary (ADR 0006): the test cannot tell what a word means, only that the cell holds
more tokens than any expression does *and* ends a clause the way an author ends a sentence.

## The population is entirely legacy, and the cost is two identities

The proof-carrying stratum holds **195 parametric width cells and 0 prose**. All 47 refusals are in
documents frozen at an older generation, so no artifact moves — replayed with `evidence --dry-run`,
all 27 rebuildable documents reproduce byte for byte.

Of the 47: **43 are TileLink**, where the declared name is a single letter; 2 are MMU-700 field rows
carrying a 30-token paragraph; and **2 are real identities** — GIC `ARCHREV` (whose "width" is a
bulleted list of architecture revisions) and CXS `CXSCNTL` (a sentence pointing at another table).
Both are declared width-only, so refusing the width drops the declaration and the row is counted
`NoDirectionAndNoWidth` instead. That is the trade `SIGNAL-DECLARATION-ROW-DROP.2e` took deliberately
and this leaf takes again: a **counted** refusal is worth more than a declaration carrying a width the
document never states, because the first is visible to `SIGNAL-DECLARATION-ROW-DROP.1`'s accounting and
the second propagates.

## The 43 are one cause, and it is a SPLIT IDENTIFIER — not the whole-cell score

TileLink's `Signal | Type | Width | Description` channel tables hand their column to the one-letter
`Type` column, so the width column rotates onto `Description` and every declaration is named `C`, `D`,
`V` or `R`. The obvious suspect is `name_cell_is_read_whole` — a genuine name column whose cells carry a
space scores zero under it — and **the measurement refutes that**:

| TileLink `table_0012` column | whole-cell score | leading-token score (before that rule) |
| --- | ---: | ---: |
| 0 `Signal` — `c opcode`, `c param`, … | 0 | **1** |
| 1 `Type` — `C`, `D`, `V`, `R` | **4** | **4** |

`Type` clears `NAME_COLUMN_OVERRIDE_MARGIN` under **both** scores, so the rotation predates the rule and
removing the rule would not undo it. The cause is that `c opcode` is the single identifier `c_opcode`
with its underscore missing, so every row's leading token is the same letter and the column can offer
only ONE distinct name however it is scored. eMMC `table_0221` carries the same split (`t PERIOD`,
`t TLH , t THL`) and declares. Tracked as `[[TEXT-LAYER-IDENTIFIER-SPLIT]]`.

Measured separately (`scripts/measure_name_column_whole_cell_score.py`), the whole-cell rule's own recall
cost is **zero**: it zeroes 11 name columns across 573 tables, all eleven genuinely prose, six of which
are then correctly overridden onto the real name column.

## A declaration census needs more than `table_kind`

Scoping a width census to tables whose persisted `table_kind` is `signal_description` joins at
**92.8%**, and every miss is one table: AXI `table_0251`, `Name | Width | Source | Description`, 24
rows, persisted kind **`unknown`**. `effective_table_kind` promotes an `unknown` table to
`SignalDescription` through corpus memory, so the persisted kind is a lower bound on what the reader
reads. The boundary is that set **union** the tables named in `table_signal_declaration_provenance`,
and the join is then 601/601.

The join rate is the control, and it must be reported: a mirror of a producer is worth nothing until it
recovers what the producer emitted, and it is how you learn the boundary was wrong before the number is
published (`[[declared-population-is-not-the-candidate-row-population]]`).

Links: [[declared-population-is-not-the-candidate-row-population]],
[[persisted-table-kind-is-a-classifier-generation-artefact]], [[declared-spelling-is-the-document-spelling]],
[[arithmetic-width-drops-the-declaration]].
