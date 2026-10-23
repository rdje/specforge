---
id: a-list-items-number-is-the-converters-marker
title: A numbered list item's number is the converter's marker, which SourceIR's ingest drops; only a bracketed one is a note
answers:
  - "why does I2C Table 3's O [2] reach no note"
  - "where is a list item's number (marker, enumerated) in the converter output and in SourceIR"
  - "what is EXTRACTION-REPRESENTATION-GAPS.65 / .65.a / .65.b / .71"
  - "is a numbered list item beside a table a note"
  - "what is ContentElementRecord.list_marker"
date: 2026-10-23
status: current
tags: [source-ir, ingest, notes, matrices, value-combinations, extraction-representation-gaps, measured]
evidence: docs/tasks/extraction-representation-gaps/table-reading-leaves.md (.65); generated/source_ir/um10204_rev7_0_2021_i2c_bus_specification/normalized/um10204_rev7_0_2021_i2c_bus_specification.backend.json (#/texts/199-201); crates/specforge/src/ir/source/docling_backend.rs (content element builder)
reverify: "In I2C's retained converter document, #/texts/199 has label list_item, enumerated true and marker [1]; its SourceIR content element elem_00102 has kind list_item and no marker"
---

The converter keeps a numbered list item's number. Each `list_item` in a retained converter document
(`normalized/<key>.backend.json`) carries `enumerated: true` and `marker` (`[1]`, `2.`), with the number also in
`orig`. The ingest helper that builds SourceIR's `content_elements` copies only `text`, so `ContentElementRecord` has
no number. I2C *Table 3*'s notes reach SourceIR as three unnumbered list items, and `O [2]` cites nothing.

**Measured `2026-10-23` (31 chains).** 1,256 enumerated list items in 23 chains: 974 numbered `1.`, 48 bracketed `[1]`
(I2C 30, CoreSight BSA 12, I2S 6) and 234 lettered or other. Beside a table whose title or cell closes with the same
number:
- the bracketed items are notes: I2C *Table 3*, its general call table and its timing tables, and I2S's timing table.
  CoreSight BSA's twelve are a reference list that no table cites;
- the numbered `1.` items are mostly procedure steps (I2C page 7 beside `Target [1]`, the USB4 guide beside `NA 1`), so
  a number alone would bind by position;
- OpenCAPI's PHY specifications name a note in a cell (`Note 1`) and print the notes as a `1.` list after the table:
  another shape (`.71`).

**Decided (`.65`).** SourceIR keeps the marker as written (`ContentElementRecord.list_marker`), recaptured for persisted
artifacts from the retained converter document (`.65.a`). A list item marked `[N]` is note `N` and binds only a mark
written `[N]` on the pages beside the table, as a footnote binds a plain mark (`.65.b`).
