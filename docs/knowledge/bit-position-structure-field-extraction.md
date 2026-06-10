---
id: bit-position-structure-field-extraction
title: Two-column bits|description tables are STRUCTURE layouts, not registers — they extract into message_field_records with literal bit ranges
answers:
  - "how are two-column bits | description tables extracted"
  - "why do AMD DTE / NVMe command dword tables go to message fields and not registers"
  - "where do in-memory structure layouts (queue entries, table entries, dwords) live in EvidenceIR"
  - "how are caption-less page fragments of a split table stitched together"
  - "what is the bit-exact adjacency chain rule"
  - "why does a symbolic or offset-suffixed bit cell reject the whole table"
  - "what does MessageFieldRecord.bit_range mean and when is it set"
  - "why does adding an extractor change every doc's extraction manifest"
date: 2026-06-10
tags: [message-fields, structures, tables, bit-position, fragments, digestion, amd-class, nvme-class]
evidence: crates/specforge/src/ir/evidence.rs (extract_bit_position_structure_fields, parse_pure_bit_position, bit_position_chain_adjacent, bit_position_container_label, message_field_surface); docs/tasks/PDF-VARIANT-DIGESTION.md (.10b)
reverify: cargo test -p specforge --lib bit_position -- --nocapture ; plus old-vs-new `evidence --dry-run` parity over generated/source_ir/*/ (intact bundles) and the stub-copy protocol for AMD
---

`PDF-VARIANT-DIGESTION.10b` (2026-06-10). The two-column `bits | description` family
(334 `unknown`-kind tables / 6 docs; AMD IOMMU 163, NVMe 156) describes in-memory
STRUCTURES (AMD 256-bit Device Table Entry, NVMe command dwords / queue entries), NOT MMIO
registers — the family carries zero access/reset vocabulary, so register semantics would be
fabricated. TYPED HOME: `message_field_records` ("structured content fields, not wires"),
via a second strategy `message_fields.bit_position_table` beside the field-titled reader
([[register-field-table-extraction]] keeps register tables; the two are disjoint because
this family is exactly-2-column). `MessageFieldRecord` gained additive
`bit_range: Option<(u32, u32)>` (serde-skipped; the existing strategy emits `None`, so
CHI-class artifacts are byte-identical).

Gates (each measured per-item BEFORE coding): (1) STRICT cell parser
`parse_pure_bit_position` — pure `255:248`/`247`/`[7:4]` only; the lenient
`parse_bit_range` digit-filter would read NVMe's `31 + (Element Length*8) :32` as
`318:32` (fabrication); ANY failing eligible row rejects the WHOLE table (eligible row =
exactly 2 effective cells; NVMe's 3-cell value-encoding sub-rows are skipped, not counted).
(2) Container = caption label after `Table|Figure <ref>` + trailing `(Continued)` +
trailing `Field Definitions/Descriptions/Fields` strip; NO structure-noun requirement
(probe: `Reservation Register - Command Dword 10` names a COMMAND — caption nouns mislead;
the shape gate + name grammar carry precision). (3) Caption-less fragments join a chain
ONLY on BIT-EXACT adjacency (`first_hi == prev_last_lo - 1`, or ascending mirror) with
page distance ≤ 1, sense-guarded (a descending chain never "ascends" into a fresh
`31:1`-led table); chains form by adjacency FIRST (the AMD DTE head is itself
caption-less), take their container from captioned members, and an all-capless chain
yields NOTHING (corpus: 23 adopt exactly, 80 fresh structures never do, zero gray cases).
(4) Names reuse the `.10a` `recover_field_mnemonic` chain verbatim (access cell None);
`Reserved…` rows and name-less rows yield no record.

Live: NVMe 216 fields / 113 containers (CID 31:16 of Command Dword 0); AMD 82 / 15
(stitched DTE = 31 fields, 247 down to 0); USB 3.2 / USB4 / TMC / eMMC = 0 (value-encoding
tables recover no names — correct). All 12 intact docs: every extraction surface
byte-identical; the only delta is the manifest's new registered-extractor entry (the
framework records eligible entries for ALL registered extractors — adding one changes
every doc's manifest bytes BY DESIGN; it is the behavioral fingerprint). NVMe register
gold re-measured identical (0.966/0.966, 42/42, 201/201). Honest residuals: 41
offset-suffixed AMD tables (`31:28 +04` — dword-relative; future offset-aware leaf),
capless chains without captions, `SnoopAttribute` (its own description later contains
`…PTE. This field…` → the mid-cell bleed guard rightly yields; 1 true name traded for the
wrapped-cell protection). Canonical NVMe evidence NOT rebuilt this slice — a rebuild drops
the standing Pattern gauge (LLM-PRIMARY-PROMOTION.4 state); refresh rides the next
gauge-bearing sweep. Related: [[bit-location-register-field-vocabulary]],
[[corpus-register-table-shape-gap]].
