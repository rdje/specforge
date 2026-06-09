---
id: can-composition-frame-fields
title: A protocol's frame STRUCTURE is recovered from a prose composition list ("composed of N bit fields: A, B, C") + directly-stated widths (CAN SOF→EOF)
answers:
  - "how does specforge recover CAN's frame fields (SOF/Arbitration/Control/Data/CRC/ACK/EOF)"
  - "what is extract_composition_frame_fields / is_frame_field_name / stated_frame_field_bit_width / parse_count_word"
  - "how is a frame field's width kept honest (why is ARBITRATION FIELD width None not 11)"
  - "why does CAN serial_frame_fields use SerialFrameField with phase None and order = composition index"
  - "how is the composition-frame grammar kept free of corpus false positives"
date: 2026-06-09
tags: [can, serial, frame, extraction, evidence-ir, pdf-variant-digestion, adr-0006]
evidence: docs/tasks/PDF-VARIANT-DIGESTION.md (.9.3b); crates/specforge/src/ir/evidence.rs (extract_composition_frame_fields)
reverify: python3 -c "import json; e=json.load(open('generated/evidence_ir/bosch_can_specification_2_0_1991/evidence_ir.json')); print([(f['name'],f.get('bit_width'),f.get('order')) for f in e.get('serial_frame_fields',[])])"
---

Some protocols describe their frame not as `NAME[hi:lo]` bit-ranges (the SWD shape, `[[swd-serial-frame-surface]]`)
but as a **prose composition list**. CAN (Bosch CAN 2.0): *"A DATA FRAME is composed of seven different bit
fields: START OF FRAME, ARBITRATION FIELD, CONTROL FIELD, DATA FIELD, CRC FIELD, ACK FIELD, END OF FRAME"*,
with per-field widths in scattered prose (*"CONTROL FIELD consists of six bits"*, *"ACK FIELD is two bits
long"*). The SWD-tuned `extract_serial_frame_fields` (gated on serial-wire/packet/SWDIO markers) does not fire
on CAN, so CAN had 0 frame fields until `.9.3b`.

`extract_composition_frame_fields` recovers the frame, reusing the existing `SerialFrameField` surface without
a struct change — `phase` is left `None` (CAN's 7-field frame is a generic sequence, not SWD's
request/ack/data), `order` holds the composition index, `bit_width` is optional:
- **The composition list SCOPES the capture.** Only the fields named in "composed of N … bit fields: …" become
  frame fields — so the many scattered "N bits" mentions of non-frame items (ERROR FLAG, OVERLOAD DELIMITER,
  INTERMISSION, Base ID) are excluded. The list is the next statement when the composition sentence ends at the
  colon. Field names are multi-word ALL-CAPS noun phrases (`is_frame_field_name`).
- **Widths are recorded ONLY when directly + unambiguously stated.** `stated_frame_field_bit_width` accepts a
  width only when the field name is the direct subject of a **plural** `<num> bits` count
  (`parse_count_word`: number word or digit) — NEVER when `<num> bit` (singular) modifies a sub-field
  (*"consists of the 11 bit IDENTIFIER"* → ARBITRATION FIELD stays `None`, not a wrong 11-vs-12). This is the
  honesty guardrail: a residual `None` beats a mis-attributed value.

**Result (fresh CAN rebuild):** `serial_frame_fields` 0 → 7 ordered — START OF FRAME, ARBITRATION FIELD,
CONTROL FIELD (6), DATA FIELD, CRC FIELD, ACK FIELD (2), END OF FRAME; the four uneven/anaphoric/variable
widths stay honest `None`. **Additive + disjoint:** the composition path is corpus-probed to fire on CAN only
(0 false positives); SWD/ADI keeps its 11 SWD-path fields, parallel buses (APB/AHB/AXI) stay 0. Grammar, no
chip names (ADR 0006). Related: [[swd-serial-frame-surface]], [[agnostic-quoted-mode-fsm]],
[[transition-bound-state-fsm]].
