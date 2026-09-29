---
id: a-register-layout-table-sizes-the-register-its-heading-names
title: A field table takes the register name its heading's parenthetical gives and the layout table's width, only when the layout lists that name and the fields end exactly at that width
answers:
  - "why are RISC-V IOMMU registers named ddtp, cqb, cqcsr instead of register_table_00NN"
  - "how does a register get size_bits from a register layout table"
  - "what is name_and_size_registers_from_layout"
  - "what is register_layout_rows"
  - "is a layout table's Size column in bytes or bits"
  - "why is msi_cfg_tbl not sized"
  - "can a heading without a hex address name a register"
  - "why do register records cite table_0030"
  - "what is EXTRACTION-REPRESENTATION-GAPS.41.c"
  - "why does a register already named not take its summary table's width"
date: 2026-09-29
status: current
tags: [evidence-ir, registers, layout-table, reading-order, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (register_layout_rows, table_reading_orders, register_named_by_heading_parenthetical, name_and_size_registers_from_layout); docs/tasks/extraction-representation-gaps/clause-and-register-leaves.md (.41.c, .41.d); docs/book/src/pipeline/register-tables.md
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_field_table_takes_the_register_its_heading_names_when_the_layout_sizes_it a_layout_tables_size_is_read_in_the_unit_it_states_or_its_offsets_advance_by; then generated/evidence_ir/1_0_1_2026_02_22_risc_v_iommu_architecture_specification/evidence_ir.json holds 29 register_records, 22 with size_bits (ddtp 64 at offset 16 citing table_0030, table_0037, table_0038), and register_table_0078 with none"
---

RISC-V IOMMU lists every register in *Table 15. IOMMU Memory-mapped register layout* (`Offset | Name | Size |
Description | Is Optional?`, `ddtp` at 16, size 8) and gives each one a heading whose parenthetical names it
(`6.5. Device-directory-table pointer ( ddtp )`) above its field table. `EXTRACTION-GAP-FIX.3` names a register from a
heading only when the parenthetical also states a hex address, so every field table stayed `register_table_00NN`, and
since `.41.b` none had a width.

`EXTRACTION-REPRESENTATION-GAPS.41.c` (`2026-09-29`) joins them when two witnesses agree. The heading nearest before
the table in reading order ([[a-tables-reading-order-is-the-gap-it-left]]) ends in a bare parenthetical the layout
lists. And the fields of every table taking that name, a split table's halves together, end exactly at the row's
width. The size column's unit is the table's own (`bit`/`byte` in the header, `32-bit` in a cell), or else what most
offsets advance by: IOMMU's go 0+8, 8+4, 12+4, so bytes. The register takes the row's offset cell as written and cites
the layout table.

On the 31 chains only IOMMU moves. 26 of 33 records are named, the four split pairs (`ddtp`, `cqcsr`, `fqcsr`,
`pqcsr`) merge, 29 records remain, and 22 reach `.isf` under their names at the stated width. The widths are the same
numbers as before, since the extent test demands it. The width packet shrinks from 33 names to 7. `msi_cfg_tbl` is
refused because its row sizes a 256-byte table of entries while its heading's field table is one 64-bit entry. A
register another reader already named is not joined: 68 such matches exist, all in legacy documents whose SourceIR
cannot be replayed (`.41.d`).
See also [[a-register-width-is-stated-never-inferred]] and [[register-field-table-extraction]].
