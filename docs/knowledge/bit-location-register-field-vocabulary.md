---
id: bit-location-register-field-vocabulary
title: Bit-location register tables fuse the field name into the description cell — read it as a gated leading identifier
answers:
  - "how are bit location | register description | attributes tables extracted"
  - "how does the register reader recover a field name with no name column"
  - "what gates protect the leading-identifier mnemonic form from bleed"
  - "why is a field name accepted or rejected from a description cell"
  - "where does a register's byte offset come from when only the caption states it"
  - "why do CCIX-class docs extract hundreds of register fields now"
  - "which register-table family stays residual (byte location size tables)"
date: 2026-06-10
tags: [registers, tables, mnemonic, grammar, digestion, ccix-class]
evidence: crates/specforge/src/ir/evidence.rs (is_bit_position_header, recover_field_mnemonic, leading_field_identifier, field_identifier_from_paren_frame, register_offset_from_caption); docs/tasks/PDF-VARIANT-DIGESTION.md (.10a)
reverify: cargo test -p specforge --lib bit_location -- --nocapture ; plus the .10a stub-copy dry-run protocol over generated/source_ir/ccix_*/source_ir.json
---

`PDF-VARIANT-DIGESTION.10a` (2026-06-10). The `bit location | register/field description |
attributes` family (~600 tables, CCIX-class; the corpus's biggest register gap per
[[corpus-register-table-shape-gap]]) has NO name column: the field name leads the
description cell (`CCID This field indicates …`). Extraction grammar (all structural,
ADR 0006): (1) `bit location` joins the shared bit-position header vocabulary
(`is_bit_position_header`, used by BOTH the register-field gate and the bits-column
resolver — they cannot drift); (2) a name-ish header containing `description` is a
description column, never the name column; (3) on the bit-range-name path,
`recover_field_mnemonic` tries the untouched `(MNEMONIC):` form, then paren+frame
(`Full Name (Ident) This field …` — mixed-case `SevNocomm`/`LogLen` class), then the
gated leading identifier: identifier SHAPE (plain Titlecase/lowercase English words
rejected), ≠ the row's own access cell (`RO Reserved bit…` bleed), remainder not
`Reserved…`-led, unique among the table's leading tokens (a repeated leader is prose),
and no mid-cell `<Ident> This field` defined-term (page-wrap bleed) — every gate measured
per-item on the real corpus BEFORE coding; (4) the caption locator `at Byte Offset 04h` →
`offset_address` verbatim (`from…through` ranges never collapse to a point).

Measured: CCIX rev2.0 8 regs/11 fields → 143/389 (40 with offsets); CoreSight 0100/0200
+13 genuine names each; ALL 12 intact-bundle docs byte-identical (old-vs-new dry-run);
exactly ONE residual mis-name corpus-wide (`register_table_0149`, bleed without a marker).
Honest boundary: `byte location | size | register description` tables are register-AT-OFFSET
placement maps, NOT bit fields (byte→bit would fabricate) — future leaf; the `attibutes`
typo family (4 tables) stays residual (typo vocabulary is a list). Related:
[[register-field-table-extraction]], [[eval-scores-persisted-evidence]].
