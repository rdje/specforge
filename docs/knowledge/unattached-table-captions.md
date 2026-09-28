---
id: unattached-table-captions
title: Docling files some table captions as page text; the bit-layout reader recovers them page by page
answers:
  - "why is a table's caption_text null when the caption is printed on the page"
  - "how are lost or unattached table captions recovered"
  - "why did CoreSight TMC registers have no fields"
  - "why were TMC's Bits | Name | Function tables uncited"
  - "which readers use a recovered table caption"
  - "why was a Reserved | Reserved row dropped from a bit table"
  - "why does a Function title spanning two columns lose rows"
  - "why does a field repeat on value sub-rows (row-spanning cells)"
  - "why did repeated Reserved block a register fragment merge"
  - "how is a table kept from being read by two register paths"
date: 2026-09-28
tags: [registers, tables, captions, evidence, sourceir, docling]
evidence: crates/specforge/src/ir/evidence.rs (unattached_table_captions, is_printed_table_caption, collect_bit_layout_tables, bit_assignment_row_cells, repeats_row_spanning_field, reserved_ranges_are_disjoint); docs/tasks/EXTRACTION-GAP-FIX.md (.9)
reverify: cargo test --offline --workspace --lib --exclude specforge-production-graph -- unattached_caption uncaptioned_bit_assignment title_row_is_read repeated_reserved_padding printed_table_caption
---

**The fact.** Docling does not always attach a table's caption to the table. It files the caption as an
ordinary text element on the same page — most often a `section_header`, also `caption`, `body_text` or
`list_item` — and leaves `structured_tables[].caption_text` null. CoreSight TMC r0p1 prints
`Table 3-3 STS Register bit assignments` as a heading above a table whose caption is `null`; 26 of its 58
`Bits | Name | Function` tables arrive that way. The caption is recoverable from SourceIR alone, with no PDF.

**The rule (`EXTRACTION-GAP-FIX.9`, `unattached_table_captions`).** Per page: a text in the printed caption
grammar (`Table`, a reference carrying a digit, a title) is a candidate, unless it is a sentence about the table
(ends in a period, or its title opens with a plain lowercase word: `Table 3-3 shows the …`). Each caption the
converter attached on that page claims one candidate of the same text — page-local and one-for-one, because
split tables legitimately repeat a `(continued)` caption. The rest pair with the page's uncaptioned tables in
reading order only when the counts are equal. Measured: 1,296 captions in 54 of 78 SourceIRs; of 1,906
neighbouring pairs involving one, 1,900 follow the document's table numbering and the 6 that do not were
adjudicated (3 correct by content, 2 oracle artefacts, 1 not provable from text).

**Who reads it.** Only `collect_bit_layout_tables` (the `.10b`/`.10c` bit-layout reader). Every other
caption reader is `EXTRACTION-GAP-FIX.12`.

**Row-level traps in the same tables** (all Docling grid facts, all fixed for the three-column family):
- `effective_row_cells` collapses equal neighbours by TEXT, so `[31:7] | Reserved | Reserved` shrinks to two
  cells. A real span is `col_span > 1` repeated per column; two equal `col_span = 1` cells are two columns. The
  three-column reader therefore reads by physical column (`bit_assignment_row_cells`); the helper itself is
  unchanged because five readers use it (16,282 equal `col_span = 1` neighbours corpus-wide).
- A `Function` title spanning two columns gives four-cell rows (`value | meaning`); the description is every
  cell under the title.
- A row-spanning cell is copied into every row it covers, so a field with value sub-rows arrives as several
  rows with the same bit and name cells (`repeats_row_spanning_field` reads them as sub-rows).
- A merged title such as SoC-600's `Reset Type Description` makes a table look like both a bit-layout table
  and a register-field table; `is_register_field_header` owns it and the bit-layout reader skips it. A merged
  two-column title (`Bits | Name Description`) cannot be told from a real `Bits | Description`, so an
  unmarked two-column title row is not read (`EXTRACTION-GAP-FIX.14`).

**Merge gate.** `Reserved` may repeat within a register group when every reserved range has bits and none
overlap (`reserved_ranges_are_disjoint`); any other repeated name still refuses the merge
([[register-field-table-extraction]]).

**Measuring chains the replay cannot reach.** `check_chain_currency.sh` replays only documents with a retained
bundle (31 of 78). For the rest, run the register and message-field surfaces directly over every SourceIR with
HEAD's code and the new code (a throwaway `#[ignore]` probe, root path canonicalized — the persisted-path
contract refuses `..`) and diff: `.9` found 6 latent movers and a 12-table double read that way.
Related: [[corpus-register-table-shape-gap]].
