---
id: a-dword-packet-layout-reaches-message-field-records
title: A packet layout written as DW | Bits | Description, with nested bit sub-rows and dword spans, is read into message_field_records
answers:
  - "how are USB4 packet layouts (DW | Bits | Description) extracted"
  - "what is message_fields.dword_layout / extract_dword_layout_fields / dword_layout_field_name / parse_dword_span / continues_dword_layout"
  - "how is a field spanning several dwords stored (7:4, 9:12)"
  - "how are nested Bits | Description sub-rows inside a dword read"
  - "when does a caption-less dword layout fragment continue the previous one"
  - "what is EXTRACTION-REPRESENTATION-GAPS.55.b.iii"
date: 2026-10-21
status: current
tags: [message-fields, packets, tables, layouts, usb4, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (DwordLayoutExtractor; dword_layout_field_name; parse_dword_span; continues_dword_layout; extract_dword_layout_fields; message_field_surface); docs/archive/tasks/extraction-representation-gaps/hint-and-notation-leaves.md (.55.b.iii)
reverify: "cargo test --offline -p specforge-core --lib a_dword_layout"
---

USB4's inter-domain specification lays its packets out as `DW | Bits | Description` (*Table 2-1 Inter-Domain Packet
Format Fields*), naming a field before a dash (*HEC - Header Error Control*) and nesting a dword's own bit layout as
sub-rows that restate `Bits | Description`. Since `EXTRACTION-REPRESENTATION-GAPS.55.b.iii` (`2026-10-21`) a sixth
registered strategy, `message_fields.dword_layout`, reads it:

- **Rows:** a dword cell stating one dword or a span (`3`, `7:4`, `9:12`; `parse_dword_span`) and a bit range is a
  field — `byte_offset` the first dword's byte offset, `bit_range` relative to it, a span of whole dwords
  `(32 × dwords − 1, 0)`. A nested sub-row (empty dword and bit cells) is a field of the single dword above, unless
  a `Value` sub-header opens it: since `.55.b.iv.b.i` such rows are the field's `values` (USB4's *Error Response*
  codes had been three bogus nested fields). A value sub-table repeating its field's dword and bits, and a symbolic
  dword (`9+N`), state nothing.
- **Name** (`dword_layout_field_name`): the head before the first dash, period or colon a space follows, or a short
  cell that is all name (*Route String High*); *Reserved* is padding.
- **Container:** the caption's label, a caption the converter left beside the table, or the layout a caption-less
  fragment continues bit-exactly within a page (`continues_dword_layout`: the next dword from bit 0, or the same dword
  above the last bit).

**Measured `2026-10-21`:** USB4 inter-domain gains 56 packet fields in 14 containers from 17 of its 20 tables; no other
chain has the header. Message fields stop at EvidenceIR (`DOC-INTENT-TAXONOMY.4b`).
