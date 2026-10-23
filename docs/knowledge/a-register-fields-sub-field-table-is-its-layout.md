---
id: a-register-fields-sub-field-table-is-its-layout
title: A table whose bit column a declared register titles or captions, every range inside one of its fields, lays that field out in sub-fields
answers:
  - "what is RegisterFieldRecord.sub_fields / RegisterSubField / RegisterSubFieldCell (.60.a)"
  - "how is ADIv6 Table B3-10 (JEP106 field, Bits in IDCODE) read"
  - "why does IDCODE.DESIGNER have sub-fields (Continuation code, Identity code)"
  - "does a register field's sub-field reach the .isf"
  - "what is EXTRACTION-REPRESENTATION-GAPS.60 and .60.a"
date: 2026-10-23
status: current
tags: [evidence-ir, registers, sub-fields, layouts, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (fill_field_sub_fields; sub_field_bit_range); crates/specforge/src/ir/source.rs (RegisterSubField; RegisterSubFieldCell); docs/tasks/extraction-representation-gaps/table-reading-leaves.md (.60, .60.a)
reverify: "cargo test --offline -p specforge-core --lib a_register_fields_sub_field_table_lays_it_out"
---

ADIv6 gives `IDCODE`'s `DESIGNER` (`bits[11:1]`) as ARM's JEP106 code, then lays the code out in *Table B3-10*
(`JEP106 field | Width (bits) | Bits in IDCODE | ARM registered value`): *Continuation code* `Bits[11:8]` `0b0100 (
0x4 )` and *Identity code* `Bits[7:1]` `0b0111011 ( 0x3B )`. Until `.60.a` `RegisterFieldRecord` had no place for
them.

**The carrier (`.60`, decided `2026-10-22`).** `RegisterFieldRecord.sub_fields`, one `RegisterSubField` per row: its
name, its bits as the register numbers them, the table it came from, and every other cell with its column title
(`RegisterSubFieldCell`). It parallels `MessageSubField` (`.55.b.iv.b.i`), whose rows state field-relative bits.

**The reader (`.60.a`, `2026-10-23`).** `fill_field_sub_fields` runs last in the register pipeline. It takes a table
with at least two rows and a column whose every cell is a bit range (`sub_field_bit_range`: `Bits[11:8]`, `Bit[3]`,
`[7:1]`, `11:8`; a bare number is a width, not a range). The column's title, or else the table's caption, names a
declared register as a word. The ranges are disjoint and all sit inside one field of that register, so none is the
whole field. A column of numbers must restate each range's width, or the table is refused. The name is the first other
non-numeric column of distinct, non-empty cells. A field already laid out keeps its first table.

**Measured `2026-10-23` (31 chains, dry run):** only ADIv6's `IDCODE.DESIGNER` gains sub-fields (2, from
`table_0050`); the other 30 chains' EvidenceIR is identical. `sub_fields` is skipped when empty, so no other register
record changes.

**Carried, not lowered.** SemanticIR and IntentIR clone register records whole, so the sub-fields reach IntentIR.
`.isf` `(fields …)` is one level and no rule names a sub-field, so the adapter declares `DESIGNER` whole and `.isf`
does not change (ADR 0033 §6).
