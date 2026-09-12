---
id: declared-population-is-not-the-candidate-row-population
title: 318 candidate rows correspond to 17 phrase declarations — a filter over source rows counts rows the reader never used, and over-states the declared population by ~19x
answers:
  - "how many signal declarations come from a name cell that is a phrase"
  - "how do I recover the source row behind a table_signal_declaration_provenance entry"
  - "what shapes does a declaring name cell take"
  - "is a candidate-row count the same as a declaration count"
  - "why does a row that looks like it would declare a signal not declare one"
  - "which documents declare signals from tables in the current stratum"
  - "does AXI mint a signal from a prose name cell"
  - "how many phrase name cells does the current reader accept"
  - "why did the 345-row phrase approximation not re-derive"
  - "how do I tell a comma family from a footnote marker from a phrase without a word list"
  - "which table minted Clock and Reset as AHB signals"
  - "why does a rotated table with two body rows keep the wrong name column"
  - "what does the name-column override score, and what does it refuse"
  - "how do I know my census script still mirrors the reader"
  - "does refusing a phrase name cell recover the wire the row was hiding"
  - "can a row-level rule refuse the eMMC bus-mode matrix"
  - "how do I measure a declaration population through the reader rather than through a filter"
date: 2026-09-12
status: current
tags: [evidence-ir, declarations, census-method, adr-0006, legacy-artifacts, prose-name-cell-declaration]
evidence: scripts/measure_declaration_name_cell_shapes.py; crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations; name_cell_is_read_whole; signal_names_in_name_cell; NAME_COLUMN_OVERRIDE_MARGIN; infer_signal_table_row_width_hint); docs/tasks/PROSE-NAME-CELL-DECLARATION.md (.0, .2)
reverify: "python3 scripts/measure_declaration_name_cell_shapes.py — expect candidate-row approximation 318 rows / 27 documents; CURRENT stratum 604 declarations, 604 joined (100.0%), 530 single-token / 72 comma-family / 2 footnote-marked / 0 bracket-suffixed / 0 repeated-token / 0 phrase; LEGACY 2085 declarations, 1251 joined (60.0%), 11 phrase. A join rate below 100.0% on the CURRENT stratum means this census has drifted from the reader, not that the reader is wrong."
---

A filter that selects rows *shaped like* the defect is not a measurement of the defect. It counts rows
the reader may never have used, and in this repository the gap was a factor of nineteen.

`PROSE-NAME-CELL-DECLARATION` opened on a count of **345 rows across 28 documents** — signal-description
rows whose name cell holds two or more whitespace tokens beginning with an identifier. The real
question was how many *declarations* the reader minted from such a cell. Joining every persisted
`table_signal_declaration_provenance` entry back to the source row carrying its name answers it:

| stratum | documents | declarations | joined | `phrase` name cells |
| --- | ---: | ---: | ---: | ---: |
| current (proof-carrying) | 4 | 604 | **604 (100.0%)** | **2 → 0** (closed by `.2`) |
| legacy (inspection-only) | 22 | 2,085 | 1,251 (60.0%) | 15 → 11 |

**17 declarations, not 345 rows.** The reader discards a row that offers neither a direction nor a
width long before the name cell's shape matters, so most candidate rows never reach the name at all.
The opening figure is also not reproducible: restated under its own description the filter yields
318 rows / 27 documents, and two other readings of "share a two-character prefix or suffix" give 310
and 308. An untracked probe's number survived one session and then could not be re-derived — which is
why the census ships as `scripts/measure_declaration_name_cell_shapes.py`.

## How to recover the row behind a declaration

`TableSignalDeclarationProvenanceRecord` carries `statement_id`, `signal_name` and `table_id` — no row
index. Replay the reader's own name-column selection over the persisted SourceIR table (header keyword
`signal`/`name`/`port`/`pin`, then the content-based rotation override, whose `max_by_key` keeps the
**last** maximum on a tie), run `signal_names_in_name_cell` over each candidate cell, and match the
provenance name against what the cell would emit. Check `header_rows` as well as `body_rows`: the
trapped-row path declares from a table's header rows.

**The join rate is the control, and it must be reported.** A replay of the reader is worth nothing
unless it recovers the names the reader actually emitted. It recovers every one of the 604 current
declarations; it misses 834 of 2,085 legacy ones, because a folded name is not the cell's spelling and
cannot join (`[[declared-spelling-is-the-document-spelling]]`, the same finding from a second
direction). Never sum the two strata.

The control earns its keep in both directions. When `.2` changed the reader's column score, the census
script still carried the old one and its current-stratum join fell to **602/604** — the two signals
`.2` had just repaired. That is the census reporting that *it* had drifted, not that the reader had
broken. A mirror of a producer must move with the producer, and the join rate is how you find out that
it has not.

## The five shapes, none of which needs a word list (ADR 0006)

Applied in this order to a cell that produced a declaration:

- **`comma-family`** — `signal_names_in_name_cell` itself admitted two or more names. 72 of the current
  stratum, all AXI-L, 35 distinct cells (`AWSIZE, ARSIZE` … `AWIDUNQ, BIDUNQ, ARIDUNQ, RIDUNQ`).
  **AXI's phrase count is zero**, which is the result the tree's acceptance criteria named as the risk.
- **`single-token`** — one whitespace token. 530 current.
- **`footnote-marked`** — every later token is one alphanumeric character (`HSELx a`, `PSELxCHK b`).
- **`bracket-suffixed`** — every later token carries no letter (`ARMPAM [10:0]`).
- **`repeated-token`** — a token recurs *and* every other token is a fragment, so the whole cell is one
  identifier the text layer broke apart (`waitrequest waitrequest _ n`). Requiring only "some token
  recurs" mis-shelves prose, because a sentence repeats words as a matter of course.
- **`phrase`** — everything else.

## The two phrases were a column defect, and refusing them would have hidden it

Both current-stratum phrases were AHB `table_0004`: `Clock source` → `Clock`, `Reset controller` →
`Reset`. That table is **rotated** exactly like AHB `table_0033`, but it has two body rows, so the real
name column scored 2 distinct tokens against the header column's 2 and `NAME_COLUMN_OVERRIDE_MARGIN =
2` could never be cleared. The same two rows also carried a whole sentence as their width, accepted as
`WidthHint::Parametric` alongside 62 legitimate forms such as `ceil(DATA_WIDTH/8)`.

The fix is not to refuse the phrase — that silences the row. It is to score the column by the cells the
reader reads WHOLE (`name_cell_is_read_whole`), so a prose column stops scoring like a name column.
`.2` shipped that, and AHB `table_0004` now declares `Signal HCLK is output width 1.` and
`Signal HRESETn is width 1.`: two phantoms and two prose widths gone, and `HRESETn` — the document's
own active-low spelling — reaching IntentIR for the first time (AHB's interface signal set 40 → 41,
nothing removed). **Refusing the phrase would have deleted the evidence that the column was wrong.**

Corpus-wide the score changes the name column on 7 of 602 `signal_description` tables in the current
stratum: that one repair, and six USB 3.2 register/field tables mis-typed as signal tables which
declare nothing either way — proven inert, because the proof seal accepted all 27 current EvidenceIR
artifacts and only AHB's needed rebuilding.

## What no shape can separate

The residue is not phrase-versus-split. It is **phrase-versus-single-token**. eMMC `table_0020` is a
bus-mode matrix typed `signal_description`, and its `HS400` and `HS200` rows are single tokens
indistinguishable in shape from a wire. A row-level rule refuses three of its five rows and no more, so
that table is provably not answerable at the row.
