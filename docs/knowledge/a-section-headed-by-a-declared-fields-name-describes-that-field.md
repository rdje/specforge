---
id: a-section-headed-by-a-declared-fields-name-describes-that-field
title: A section whose heading names a declared message field, and whose first statement speaks of a field, describes that field; a field its own description reserves is a reserved span citing that statement
answers:
  - "what is MessageFieldRecord.description_statement_ids"
  - "what is section_field_name / bind_field_sections / describes_reserved_field"
  - "why does CHI C2C's RsvdZero become reserved space without a Reserved keyword definition"
  - "what is ReservedSpanRecord.supporting_statement_ids"
  - "why is USB's 8.4.2 Set Link Function section not the Set Link Function field's description (its first statement speaks of the LMP)"
  - "which headings name a field (B4.2.1.5 RsvdZero; B4.2.1.1 Message type, MsgType; not an unnumbered heading)"
  - "what is EXTRACTION-REPRESENTATION-GAPS.55.b.iv.b.ii"
date: 2026-10-21
status: current
tags: [evidence-ir, message-fields, field-descriptions, reserved-spans, sections, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (section_field_name; bind_field_sections; describes_reserved_field; reserved_span_surface's description path; MessageFieldRecord.description_statement_ids; ReservedSpanRecord.supporting_statement_ids); docs/tasks/extraction-representation-gaps/packet-and-memory-leaves.md (.55.b.iv.b, .55.b.iv.b.ii)
reverify: "cargo test --offline -p specforge-core --lib a_section_headed_by_a_declared_fields_name_describes_that_field"
---

CHI C2C declares its message fields in container tables with no description column, and describes each one in a
section of its own: *B4.2.1.1 Message type, MsgType* … *B4.2.1.5 RsvdZero*. USB describes its setup fields the same
way (*9.3.1 bmRequestType*). No reader joined those sections to the field records. So *RsvdZero*, whose section says
*This field indicates that the use of the bits is Reserved and a Transmitter must set the field value to zero, a
Receiver must ignore the field and an intermediate agent must forward them unchanged*, was a field like any other.

**Binding (`EXTRACTION-REPRESENTATION-GAPS.55.b.iv.b.ii`, `2026-10-21`).** After convergence, `bind_field_sections`
reads each section anchor.

- **Heading:** `section_field_name` takes the part past the outline number (`outline_number`). That part is the
  declared field's name alone, or a title, a comma and the name. An unnumbered heading names no field.
- **First statement:** it must speak of a field: a word `field` (*This field identifies …*, *The FlitFormat field in
  …*). A section opening on something else describes that instead. USB's *8.4.2 Set Link Function* and *8.4.3 U2
  Inactivity Timeout* open on their LMPs, and *9.6.5 Interface* on its descriptor, so none binds to the
  same-named field.
- **Result:** every record of that name gains the section's statements, in order, as `description_statement_ids`.

**Reserved by its own description.** `reserved_span_surface` adds a named span (`field_id`) for a field whose
description holds a sentence in its frame (*This field …*) saying it, or the use of its bits, *is Reserved*
(`describes_reserved_field`). The span's `supporting_statement_ids` cite that statement, which states the
obligations. It cites a keyword only where the document defines one, and C2C defines none. The keyword reader's
*and*-split would garble C2C's comma list of agent-modal clauses, so the statement is cited rather than re-parsed.

**Measured `2026-10-21` (31 chains):** 29 identical. CHI C2C: 28 records under 8 names gain descriptions (*MsgType* in
11 containers, *RsvdZero* in 9), and the 9 *RsvdZero* records become reserved spans citing `statement_0470`, uncited
by any keyword. USB 3.2: 6 fields gain descriptions (*bmRequestType*, *bRequest*, *wValue*, *wIndex*, *wLength*,
*CRC-16*). Every later stage and `.isf` text is byte-identical.
