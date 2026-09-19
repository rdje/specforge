---
id: per-row-column-drift-rule
title: A signal table whose rows disagree about where their columns are is read per row against one anchor, and the measured result is that the corpus declares the same 1,677 signals with three directions corrected and one phantom gone
answers:
  - "how does SpecForge read a table whose rows disagree about their column order"
  - "what is the per-row layout drift rule"
  - "what anchors a per-row column shift"
  - "why is a tie between two direction columns not an anchor"
  - "does the per-row drift rule change consistent tables"
  - "how many signals did the per-row drift rule add"
  - "did the per-row drift rule move any gold score"
  - "why did CoreSight SDC-600 table_0059 stop declaring anything"
  - "can recovering rows make a table be withheld as a base-name template"
  - "why must a declaration change be measured on the pass and not on one function"
  - "which documents does SIGNAL-DECLARATION-ROW-DROP.2h.2 change"
date: 2026-09-20
status: current
tags: [signal-declaration-row-drop, evidence-ir, table-extraction, measurement, adr-0006]
evidence: crates/specforge/src/ir/evidence.rs (per_row_layout_anchor_column, row_literal_direction_column, mod signal_declaration_row_drop_2h_2); docs/research/direction-column-drift-census.md; docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.2h.2)
reverify: "cargo test -p specforge-core --lib signal_declaration_row_drop_2h_2 — expect 8/8; and python3 scripts/measure_direction_column_drift.py --reader-vocabulary — expect 8 drifted tables across 4 documents, 81 body rows."
---

`.2e`'s content-based name-column override applies a **whole-table offset**, which is right when a
header row is shifted relative to its body and wrong when the body's own rows disagree. **Eight
`signal_description` tables in four documents** (81 body rows) put a whole-cell direction word in
different columns on different rows, so one answer is right for some rows and wrong for the rest.

## The rule

A table **drifts** when its body rows disagree about which column holds a whole-cell direction word.
Rows that all agree are **not** drift, whichever column they agree on — that uniform shift is what
`.2e` already serves — so the rule cannot reach the 152 consistent or 411 direction-less tables.

On a drifted table every row is measured against **one anchor**: the direction column the table
already resolved, when it resolved one **and at least one row uses it**; otherwise the column a
**strict plurality** of rows use. A row whose direction sits elsewhere has *all* of its columns
moved by the difference, because a rotated row rotates whole. A row stating no direction word, or
two, contributes no opinion and keeps the table's own columns.

Three refusals are part of the rule: **a tie is not an anchor** (one table splits two rows against
two with no header to prefer either); **an abbreviation does not make a table drift** (`In`/`Out`
are not direction words to this reader, which is why the census finds nine tables and the rule
reaches eight); and **an anchor no row uses is refused**, because "every row is shifted" is a
whole-table claim a per-row rule has no standing to make.

Structural only (ADR 0006): direction values the reader already had, column indices, and
disagreement between rows. No new vocabulary, no header text, no identity.

## The measured result is correctness, not count

Over all 78 persisted `source_ir.json`, through the **whole declaration pass** — prior-memory
guidance, trapped-row recovery and base-name-template withholding included:

- provenance rows **2,581 → 2,589**;
- **distinct declared signal names 1,677 → 1,677**;
- the 27 proof-carrying chains produce **the same 604 declarations, byte-identical**, so no stored
  current artifact and no gold moves (156/156 `kg-bench` fixtures unchanged). All four affected
  documents are legacy proofless; the improvement lands when they are re-ingested.

What moves: TMC `table_0074` loses the phantom `Data`, gains `AFREADYM` (declared by no other table
in that document), and has `ATVALIDM`/`ATBYTESM`/`ATDATAM` corrected `input` → `output` against its
own `Output` cells; AXI/ACE gains three subordinate-side `input` declarations and three manager-side
rows. Fourteen changed row readings, adjudicated **14 true positives, 0 false positives**.

## The result that only a pass-level measurement could show

SDC-600 `table_0059` published two declarations before and **zero** after. It is not a regression.
Recovering three rotated rows takes the table from two declared names to five, and at three it
crosses `WIRE-BASED-100.10b`'s base-name-template floor: the document declares `EXT_` and `INT_`
instantiations of **every one of the five members**, so the table is withheld whole as the base-name
template it is. The unqualified `CLK_QDENY`/`CLK_QACTIVE` it used to publish were base names, not
ports, and survived only because two names is below the three-member floor. SDC-600's declared
inventory is **114 names before and after** — nothing was lost.

The general lesson is procedural: **measuring one function is not measuring the change.** A
function-level probe of this same rule reported `+13` declarations and three newly declared signals.
The pass reports `+8` and none, because the template filter and the trapped-row pass both sit
downstream of the function and both had something to say.

Links: [[direction-column-drift]], [[a-cheap-structural-rule-overfires-until-you-read-its-selection]].
