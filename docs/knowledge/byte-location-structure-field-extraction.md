---
id: byte-location-structure-field-extraction
title: Byte-location placement tables are in-memory STRUCTURE layouts, not register maps — read them into message_field_records
answers:
  - "how are byte location | size | register description tables extracted"
  - "are byte-location placement tables registers or structures"
  - "why did the register-at-offset placement-map hypothesis get overturned"
  - "where do CCIX PER error structure fields land in the IR"
  - "how are multi-word field names like Validation Bits or FRU ID recovered"
  - "what does byte_offset mean on a message field record when bit_range is None"
  - "how do byte-granular page fragments chain (offset plus size adjacency)"
  - "which strategy is message_fields.byte_location_table"
date: 2026-06-11
tags: [message-fields, structures, tables, grammar, digestion, ccix-class, probe-first]
evidence: crates/specforge/src/ir/evidence.rs (byte_location_layout_columns, collect_byte_location_tables, byte_location_chain_adjacent, byte_location_field_name, extract_byte_location_structure_fields); docs/tasks/PDF-VARIANT-DIGESTION.md (.10e)
reverify: cargo test -p specforge --lib byte_location -- --nocapture ; plus the stub-copy dry-run protocol over generated/source_ir/ccix_*/source_ir.json
---

`PDF-VARIANT-DIGESTION.10e` (2026-06-11). The `byte location | size (bytes) | register
description [| attribute(s) | m/o]` family (exactly 60 tables corpus-wide, one CCIX-class
family × 4 versions) was recorded by [[bit-location-register-field-vocabulary]] as
"register-AT-OFFSET placement maps → future lever: register-map records". The per-item
probe OVERTURNED that: 0 of 60 captions say register — every caption names an in-memory
error-record STRUCTURE (`CCIX PER Memory Error Type Structure`, `Vendor-Specific Log
Info`), rows are byte-granular record fields with M/O vocabulary. Typed home =
`message_field_records` via strategy `message_fields.byte_location_table` (the `.10c`
caption-decides rule; the RO/RsvdZ attribute column does NOT outvote the caption).
Capture is literal: `byte_offset` = stated byte location (beside `bit_range: None` this
reading is unambiguous — `.10d` dword-relative needs `bit_range: Some`), `bit_width` =
plain-count size × 8 (exact unit arithmetic; symbolic `(indicated by VenLen)` → None).
Names are the document's own MULTI-WORD English heads before the definitional frame
(`Validation Bits`, `FRU ID`) — family-LOCAL grammar, never the shared bare-identifier
form (it would TRUNCATE: `FRU ID`→`FRU`, `CCIX Message`→`CCIX`); a trailing parenthesized
mnemonic before the frame outranks the head (`(Chan)`) and is trusted past wrapped-cell
bleed; a head containing a sentence period is bleed → refused (2 rows, honest residuals).
Chains: byte-exact adjacency `next_offset == prev_offset + size` (31/31 measured), fresh
structures restart at 0, symbolic-size tails close the chain. Live: 35–45 fields / 4–6
containers per version (≈161 total), zero pre-existing records changed, 12-doc parity
manifest-only. Residual: lost-caption chains (Port 7-9 in all 4 versions; ATC + Log Info
in r1.0a; Cache in rev1.1) — the re-ingest lever. Related:
[[bit-position-structure-field-extraction]], [[offset-suffixed-dword-relative-bit-cells]].
