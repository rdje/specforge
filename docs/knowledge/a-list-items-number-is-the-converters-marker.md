---
id: a-list-items-number-is-the-converters-marker
title: A numbered list item's number is the converter's marker, kept as SourceIR's list_marker; only a bracketed one is a note
answers:
  - "why does I2C Table 3's O [2] reach no note"
  - "where is a list item's number (marker, enumerated) in the converter output and in SourceIR"
  - "what is EXTRACTION-REPRESENTATION-GAPS.65 / .65.a / .65.b / .71"
  - "is a numbered list item beside a table a note"
  - "does I2C Table 3 cite its notes (matrix_table_0001 [1] [2] [3])"
  - "what is ContentElementRecord.list_marker / list_marker_of / source_table_recapture --list-markers"
  - "how is a pre-coordinate batched record placed (page, source_ref)"
date: 2026-10-23
status: current
tags: [source-ir, ingest, notes, matrices, value-combinations, extraction-representation-gaps, measured]
evidence: docs/tasks/extraction-representation-gaps/table-reading-leaves.md (.65); generated/source_ir/um10204_rev7_0_2021_i2c_bus_specification/normalized/um10204_rev7_0_2021_i2c_bus_specification.backend.json (#/texts/199-201); crates/specforge/src/ir/source/docling_backend.rs (content element builder)
reverify: "In I2C's retained converter document, #/texts/199 has label list_item, enumerated true and marker [1]; its SourceIR content element elem_00102 has list_marker [1]"
---

The converter keeps a numbered list item's number. Each `list_item` in a retained converter document
(`normalized/<key>.backend.json`, every `documents[]` part of a batched one) carries `enumerated: true` and `marker`
(`[1]`, `2.`), with the number also in `orig`. Until `.65.a` the ingest helper copied only `text`, so I2C *Table 3*'s
notes reached SourceIR as three unnumbered list items. Since `.65.a` (`2026-10-23`) the helper's `list_marker_of`
keeps the marker as `ContentElementRecord.list_marker`, and persisted artifacts recaptured it
(`source_table_recapture --list-markers`): 1,928 markers in 23 chains. The 9 left over sit in lists inside I2C's
figures, which ingest leaves out of the prose stream. A record written before `source_batch` existed is placed by its
page, since a batched bundle's parts partition the pages (`SOURCE-IR-REPRODUCIBILITY.17`).

**Measured `2026-10-23` (31 chains; corrected by `.65.a`, whose census also read batched bundles' parts).** 1,937
marked list items in 23 chains: 1,546 numbered `1.`, 55 bracketed `[1]` (I2C 30, CoreSight BSA 12, AXI 7, I2S 6) and
336 lettered or other. `.65`'s first count, 1,256, missed the batched bundles (AXI, ADIv6, USB 3.2). Beside a table
whose title or cell closes with the same number:
- the bracketed items are notes: I2C *Table 3*, its general call table and its timing tables, and I2S's timing table.
  CoreSight BSA's twelve are a reference list that no table cites;
- the numbered `1.` items are mostly procedure steps (I2C page 7 beside `Target [1]`, the USB4 guide beside `NA 1`), so
  a number alone would bind by position;
- OpenCAPI's PHY specifications name a note in a cell (`Note 1`) and print the notes as a `1.` list after the table:
  another shape (`.71`).

**Decided (`.65`).** SourceIR keeps the marker as written (`ContentElementRecord.list_marker`), recaptured for persisted
artifacts from the retained converter document (`.65.a`). A list item marked `[N]` is note `N` and binds only a mark
written `[N]` on the pages beside the table, as a footnote binds a plain mark (`.65.b`).

**Bound (`.65.b`, `2026-10-23`).** `footnote_texts_by_page` takes a list item marked `[N]` as note `[N]`, and
`trailing_note_marks` keeps a bracketed mark bracketed, so I2C *Table 3* cites `[1]`, `[2]` and `[3]` and nothing else
moved in 31 chains. A combination key titled `PPROT [1]` reads as `PPROT` with its note. I2C's address and timing
tables cite such notes too, but no reader binds a note there (`.72`), and *Table 3*'s continuation, its header rows
inverted, never joins the matrix (`.73`).
