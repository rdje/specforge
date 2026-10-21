---
id: a-fields-restated-sub-table-types-its-values-and-bits
title: A sub-header a packet or descriptor layout restates after a field row opens that field's sub-table — Value rows become the field's stated values, Bits rows its own bit layout, reserved ones citing the keyword
answers:
  - "what are MessageFieldRecord.values / MessageFieldValue and MessageFieldRecord.sub_fields / MessageSubField"
  - "where do a USB packet field's code values go (0001b | Set Link Function under Value | Type of LMP)"
  - "where do a field's own bits go (1 | Force_LinkPM_Accept under Bits | Description; bmAttributes Bit | Encoding)"
  - "what is field_sub_table_header / read_field_sub_table / sub_row_cells / raw_column_index / cite_reserved_values"
  - "what is ReservedSpanRecord.within_field_id (a reserved bit range inside a field, relative to it)"
  - "why did USB4's Unknown Packet Type, Not Supported and Not Ready stop being message fields"
  - "why is a spanning prose row under a Value sub-header no value (it repeats itself across both cells)"
  - "why is a fused Value Meaning sub-header or a 0 1 bit cell not read"
  - "what is EXTRACTION-REPRESENTATION-GAPS.55.b.iv.b.i"
date: 2026-10-21
status: current
tags: [evidence-ir, message-fields, field-values, sub-fields, reserved-spans, packet-layouts, descriptors, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (MessageFieldValue; MessageSubField; FieldSubTable; field_sub_table_header; sub_row_cells; raw_column_index; read_field_sub_table; cite_reserved_values; the sub_rows of extract_width_offset_layout_fields, ByteLocationRow and extract_dword_layout_fields; reserved_span_surface's within_field_id spans); docs/tasks/extraction-representation-gaps/packet-and-memory-leaves.md (.55.b.iv.b, .55.b.iv.b.i)
reverify: "cargo test --offline -p specforge-core --lib a_fields_restated_sub_header_opens_its_sub_table a_width_offset_fields_sub_table_reaches_its_record a_dword_layouts_value_sub_table_lists_values_not_fields a_fields_own_layout_reserves_bits_within_it"
---

USB lays a field's detail out under its own row. The width and offset (or offset, field and size) cells are left
empty, and a sub-header is restated in the description columns: *Value | Type of LMP*, *Bits | Description*, *Bit |
Encoding*, or *Bits Description* fused in one cell. Every message-field family skipped those rows, and the dword
family nested every empty-dword row as a field of the dword above. So the LMP subtype codes, the bits of *Set Link
Function* and *bmAttributes*, and USB's reserved code values were unread. USB4's *Error Response* codes became three
bogus fields at bits 1, 3 and 4.

**Reading (`EXTRACTION-REPRESENTATION-GAPS.55.b.iv.b.i`, `2026-10-21`).** The width-and-offset, descriptor and dword
families collect each field row's sub-rows: rows whose cells before the description column are all empty
(`sub_row_cells`, with `raw_column_index` mapping the effective column to the raw one). `read_field_sub_table` then
reads them into the record.

- **Sub-header:** `field_sub_table_header` opens a sub-table. A fused *Value Meaning* is not read.
- **Value rows** hold two distinct cells. A spanning prose cell repeats itself, as an ellipsis row does, and a row
  with no value states none. Each becomes a `MessageFieldValue { value, meaning }`. `cite_reserved_values` gives one
  whose meaning opens with *Reserved* the document's *Reserved* `keyword_definition_id`.
- **Bit rows** become `MessageSubField { bit_range, name, description }`. The bits are relative to the field. The name
  is the description's head (`dword_layout_field_name`); *Reserved* has none. A row with an empty bit cell continues
  the sub-field above (*Force_LinkPM_Accept De-assert*), and a fused row splits at its first space. A bit cell holding
  two rows (*0 1*) is not read.
- **Reserved spans:** `reserved_span_surface` adds each reserved sub-field as a span `within_field_id` the field, its
  `bit_range` relative to that field.
- **Dword family:** under a `Value` sub-header its nested rows are values. Under *Bits* they are still fields of the
  dword (`.55.b.iii`).

**Measured `2026-10-21` (31 chains):** 29 identical. USB 3.2 keeps its 209 fields; 10 gain 33 values (7 reserved,
all citing `keyword_0005`) and 10 sub-fields (3 named), and the reserved spans go 38 -> 42 (4 within fields). USB4
inter-domain goes 56 -> 53 fields, losing the three bogus ones, and *Error Response* gains 4 values (1 reserved, cited).
Every later stage and `.isf` text is byte-identical. Values and sub-fields stop at EvidenceIR with the field records
(`DOC-INTENT-TAXONOMY.4b`).
