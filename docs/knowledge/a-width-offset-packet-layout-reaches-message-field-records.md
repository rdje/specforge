---
id: a-width-offset-packet-layout-reaches-message-field-records
title: A packet layout written as Width (bits) | Offset (DW:bit) | Description is read into message_field_records — name before the first period or colon, dword byte offset and dword-relative bit range
answers:
  - "how are USB packet header layouts (Width (bits) | Offset (DW:bit) | Description) extracted"
  - "what is message_fields.width_offset_layout / extract_width_offset_layout_fields / width_offset_field_name"
  - "how is a DW:bit offset stored on a message field (byte_offset and bit_range)"
  - "why is Delayed (DL) named DL"
  - "why are Reserved rows not message fields"
  - "what is EXTRACTION-REPRESENTATION-GAPS.55.b.ii and .55.b.v"
date: 2026-10-21
status: current
tags: [message-fields, packets, tables, layouts, usb, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (WidthOffsetLayoutExtractor; width_offset_layout_columns; width_offset_field_name; extract_width_offset_layout_fields; message_field_surface); docs/archive/tasks/extraction-representation-gaps/hint-and-notation-leaves.md (.55.b.ii, .55.b.v)
reverify: "cargo test --offline -p specforge-core --lib a_width_offset_layout"
---

USB 3.2 lays out its link and transaction packets as `Width (bits) | Offset (DW:bit) | Description` (*Link Control
Word Format*: `3 | 3:16 | Header Sequence Number . The valid values …`), a header none of `message_field_surface`'s
extractors read. Since `EXTRACTION-REPRESENTATION-GAPS.55.b.ii` (`2026-10-21`) a fifth registered extractor,
`message_fields.width_offset_layout`, reads it:

- **Header** (`width_offset_layout_columns`): a `width`-led column, an `offset`-led column naming its `dw`, and a
  description column, whatever kind the converter guessed.
- **Rows:** one stating a width and a `DW:bit` offset is a field; `byte_offset` is the dword's byte offset (`3` ->
  `12`) and `bit_range` the dword-relative range from the stated start bit and width (`(18, 16)`), the record's
  dword-relative convention. Value sub-rows (`0000b`, `Value`) are skipped, never the table.
- **Name** (`width_offset_field_name`): the head before the first period or colon a space follows (*Type. These 5
  bits …* -> `Type`), at most eight words, letter-led, no inner period; a trailing two-character mnemonic outranks it
  (*Delayed (DL)* -> `DL`). A *Reserved* row is padding, as in every message-field family.
- **Container:** the caption's label, or a caption the converter left beside the table; a table with neither continues
  the open layout when its first field starts bit-exactly where that one ended, within a page.

**Measured `2026-10-21`:** USB 3.2 gains 83 packet fields in 25 containers; no other chain has the header. The
`Bit | Description` status-word tables (*Hub Status Field, wHubStatus*) are `.55.b.v`'s. Like every message field,
these stop at EvidenceIR (`DOC-INTENT-TAXONOMY.4b`).
