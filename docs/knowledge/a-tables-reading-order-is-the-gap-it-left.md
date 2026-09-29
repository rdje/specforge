---
id: a-tables-reading-order-is-the-gap-it-left
title: SourceIR stores no reading order for a table or figure, but each leaves a gap in the text's reading-order sequence, and when gaps equal visual assets the n-th gap is the n-th asset
answers:
  - "does a SourceIR table have a reading order"
  - "how do I find the heading a table sits under"
  - "why is placing a table by its page wrong"
  - "what is table_reading_orders"
  - "why do content_elements reading_order values skip numbers"
  - "does nearest_section_title_original place a table correctly"
  - "why were RISC-V Debug register fields in the wrong register"
  - "what is heading_above_table"
date: 2026-09-29
status: current
tags: [source-ir, reading-order, tables, docling-backend, evidence-ir, measured]
evidence: crates/specforge/src/ir/source/docling_backend.rs (element_reading_order increments for every iterate_items item; only text elements record it); crates/specforge/src/ir/evidence.rs (table_reading_orders, heading_above_table, nearest_section_title_original); docs/tasks/extraction-representation-gaps/clause-and-register-leaves.md (.41.c)
reverify: "python3 over generated/source_ir/1_0_1_2026_02_22_risc_v_iommu_architecture_specification/source_ir.json: the reading_order values missing from content_elements up to its highest are 196, the number of visual_assets; cargo test --offline --workspace --exclude specforge-production-graph --lib a_field_table_takes_the_register_its_heading_names_when_the_layout_sizes_it"
---

The Docling backend numbers every item `iterate_items` yields (`element_reading_order += 1`) but writes the number only
on the text elements it keeps, so a table or figure takes a number and leaves a gap. `StructuredTableRecord` and
`VisualAsset` carry no reading order. When the gaps are exactly as many as `visual_assets`, the n-th gap is the n-th
asset, the order the ingest appended both in. On RISC-V IOMMU (`2026-09-29`) 196 gaps meet 196 assets, and every
asset's page lies between its neighbouring elements' pages.

`table_reading_orders` (`EXTRACTION-REPRESENTATION-GAPS.41.c`) makes that placement, and places nothing when the counts
differ: a batched run or an empty text the ingest dropped would also leave a gap. The page-based
`nearest_section_title_original` takes the last heading on or before a table's page. That is wrong wherever two
sections share a page: IOMMU's `ddtp` field table sits on page 70 below `cqb`'s heading, and `cqh` and `cqt` share
page 71. Since `.41.f` (`2026-09-29`) the register-field reader names a table from `heading_above_table`: the heading nearest
before it in reading order, the page rule only where the order is not recoverable. On the 31 chains that renamed 30
RISC-V Debug field tables, each named one register late before (`dmstatus`'s fields had sat in `dmcontrol`).
See also [[a-register-layout-table-sizes-the-register-its-heading-names]].
