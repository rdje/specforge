---
id: a-layouts-reserved-space-is-a-typed-span-citing-the-keyword
title: A row a captured layout marks Reserved stays padding, not a field, and is a reserved_spans record of its container at the family's own position, citing the document's Reserved keyword definition
answers:
  - "what is reserved_spans / ReservedSpanRecord / reserved_span_surface / record_reserved_span"
  - "why is a Reserved row of a packet or descriptor layout not a message field"
  - "where does the reserved space of a packet layout go (Reserved (R) . at 3:19)"
  - "which keyword definition does a reserved bit cite (keyword_definition_id)"
  - "why is bReserved a field and also a reserved span (field_id)"
  - "why is Route String/Reserved, RSVDC or RsvdZero not a reserved span (is_reserved_field_name)"
  - "why is a value sub-row 0000b | Reserved not a reserved span"
  - "what is EXTRACTION-REPRESENTATION-GAPS.55.b.iv, .55.b.iv.a, .55.b.iv.b, .55.b.iv.b.i and .55.b.iv.b.ii"
date: 2026-10-21
status: current
tags: [evidence-ir, message-fields, reserved-spans, keyword-definitions, packet-layouts, descriptors, padding, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (ReservedSpanRecord; is_reserved_led; is_reserved_field_name; record_reserved_span; reserved_span_surface; the reserved sink of extract_container_message_fields, extract_bit_position_structure_fields, extract_byte_location_structure_fields, extract_width_offset_layout_fields, extract_dword_layout_fields); docs/archive/tasks/extraction-representation-gaps/hint-and-notation-leaves.md (.55.b.iv); docs/tasks/extraction-representation-gaps/packet-and-memory-leaves.md (.55.b.iv.a, .55.b.iv.b)
reverify: "cargo test --offline -p specforge-core --lib reserved"
---

Every message-field family skips a row whose name or description cell opens with *Reserved* (*Reserved (R) .*,
*Reserved; set to zero*): it is padding, and making it a field would rename every family's records. Since
`EXTRACTION-REPRESENTATION-GAPS.55.b.iv.a` (`2026-10-21`) that space is typed instead. EvidenceIR's `reserved_spans`
(family `evidence.message_structure`) holds one `ReservedSpanRecord { span_id, container, text, bit_width, bit_range,
byte_offset, field_id, keyword_definition_id, supporting_table_ids }` per reserved row. The message-field records are
unchanged.

**Reading.** Each family function takes a `reserved` sink and records a padding row where it skips one, with its own
position reading (`is_reserved_led`, the one test all four name functions share). The readings are: the container
family's width; a bit layout's range, plus the byte offset when it has one; a byte layout's location and size;
a width-and-offset layout's dword and bit; a dword layout's dword span. `record_reserved_span` merges a fragment that
restates a positioned span; a span with no position is one per row. `reserved_span_surface` runs the families in
`message_field_surface`'s order. It then adds each field named as reserved — *Reserved* alone, or after a lowercase
type prefix of at most three letters (`bReserved`, `wReserved`) — linked by `field_id`. Last, every span cites the
document's `keyword_definitions` record for *Reserved*, when there is one. Not reserved spans: a field reserved only
in some packets (*Route String/Reserved*), one reserved for another use (`RSVDC`), an abbreviation (`RsvdZero`), and a
value sub-row (*0000b | Reserved*), which reserves a code value, not space.

**Measured `2026-10-21` (31 chains):** 28 chains identical. USB 3.2 has 38 spans: 33 padding rows and 5 named
fields, in 22 containers. USB4 inter-domain has 5. Both cite *Reserved* (`keyword_0005`). The CoreSight TMC has 1
uncited span (*Reserved, SBZ*), in a page-table entry whose fields no family reads. Of USB 3.2's 47 reserved rows in
field-bearing tables, 8 are value sub-rows, 5 are bits nested inside a field, 1 has a unit-suffixed offset (`0:16b`),
and 2 are the named `wReserved`. USB4's 3 left over are value sub-rows. `.55.b.iv.b` found the value sub-rows and
nested bits to be a field's restated sub-table (`.55.b.iv.b.i`, now read), and CHI C2C's `RsvdZero` to be defined in
its own field section (`.55.b.iv.b.ii`: its 9 records are now reserved spans citing that statement, with no keyword);
`0:16b` stays a declared residual. `reserved_spans` stops at EvidenceIR, like `message_field_records`; carrying both onward is
`DOC-INTENT-TAXONOMY.4b`'s.
