---
id: offset-suffixed-dword-relative-bit-cells
title: Offset-suffixed bit cells (31:28 +04) are dword-relative — capture the literal bit_range + byte_offset, never derive absolute positions
answers:
  - "how are offset-suffixed bit cells like 31:28 +04 extracted"
  - "what does MessageFieldRecord.byte_offset mean"
  - "why is the absolute bit position never derived from offset*8+bit"
  - "are description bracket slices like Store Data[63:32] positions or values"
  - "how do dword-relative page fragments chain"
  - "when is a bracket-slice leading token a field name"
  - "when is a single letter a field name"
  - "how is a fused Fields(Continued) caption handled"
  - "why do plain English words like Address or Vector become field names under the bracket frame"
date: 2026-06-11
tags: [message-fields, structures, tables, bit-position, byte-offset, dword-relative, name-grammar, digestion, amd-class]
evidence: crates/specforge/src/ir/evidence.rs (parse_offset_suffixed_bit_position, bit_position_chain_adjacent, dword_rows_forward, bracket_slice_field_name, single_letter_framed_name, fused_name_lead_count_key, trim_continued_marker); docs/tasks/PDF-VARIANT-DIGESTION.md (.10d)
reverify: cargo test -p specforge --lib offset_suffixed -- --nocapture ; cargo test -p specforge --lib dword_relative ; plus old-vs-new `evidence --dry-run` parity over generated/source_ir/*/ (intact bundles) and the stub-copy protocol for the AMD-class doc
---

`PDF-VARIANT-DIGESTION.10d` (2026-06-11). The offset-suffixed bit-cell family
(`31:28 +04` = bits 31:28 of the dword at DECIMAL byte offset 4 of a 16-byte
command/event/PPR-log entry) is corpus-wide a ONE-document family: 43 tables / 309 rows,
offsets exactly `{00, 04, 08, 12}`; every other corpus `+`-cell is SYMBOLIC (`13+ ITSnum`,
`15+HL:16`, `9+N`) and stays rejected because the strict grammar demands
digits-colon-digits, whitespace, `+`, pure decimal digits.

THE HONESTY RULE (probe-measured, the card's core fact): description bracket notations
are field-VALUE slices, NOT positions — `Store Data[63:32]` at `31:0 +12` slices the
64-bit value, and 54 of 66 bracket rows MISMATCH `offset*8 + bit`. Deriving an "absolute"
position would make the IR contradict the document's own notation on most rows. So the
capture is LITERAL: dword-relative `bit_range` + additive
`MessageFieldRecord.byte_offset: Option<u32>` (serde-skipped); consumers derive
`byte_offset*8 + bit` themselves if they need an ordering. A row whose offset was lost in
ingest records `byte_offset: None` — honest absence, never inferred from neighbors.

Chains: dword-relative fragments join on the `(offset asc, bit desc)` lexicographic
successor — same-dword `next_hi == prev_lo - 1` (14 measured joins) or next-dword
`prev_lo == 0 → next_hi == 31, offset exactly +4` (1 measured join) — with a
`dword_rows_forward` order guard, BOTH boundary rows required to carry offsets, and the
pure/offset conventions never joining each other.

Name grammar (the unlock — and a probe-assertion CORRECTION): the existing
leading-identifier form already recovers multi-char colon/dot heads (`AttrV:`, `VCmd .`,
CCIX `ESMEnable .`) because `identifier_shaped_token` trims a trailing `:` — the offset
tables yielded 0 only because the CELL gate rejected them. The genuinely new forms are
(a) the VERBATIM bracket-slice name `DeviceID[15:0] .` (slice kept — `Address[31:0]` and
`Address[63:32]` are distinct records; the bracket+boundary frame admits plain English
heads `Address`/`Vector`/`Destination` that the bare form rightly rejects) and (b) the
framed SINGLE LETTER (`f: flush queue`, `U . The U bit …` — colon/dot frame required,
per-table uniqueness via 1-char count keys that are disjoint from the existing 2–40-char
keys by construction). A `(Continued)` caption marker FUSED to the previous word by lost
spacing (`… Fields(Continued)`) is stripped by `trim_continued_marker` in BOTH caption
readers (container label + register name) — found live when it minted a bogus container.

Live: AMD-class 82 → 217 fields / 15 → 30 containers (+135, 114 with `byte_offset`),
ZERO pre-existing records changed, all 12 intact docs FULLY byte-identical (no new
strategy registered — the family rides `message_fields.bit_position_table`, so the
corpus fingerprint vocabulary is unchanged). Honest residuals: 8 lost-caption capless
chains (re-ingest lever), 25 opcode value-rows (`01h . COMPLETION_WAIT command number.`),
two-word heads (`Store Address[31:3]`), one malformed-cell table (`16: +04`), two
conditional-layout tables (`Description, RX=0 | RX=1`). Related:
[[bit-position-structure-field-extraction]], [[bit-location-register-field-vocabulary]],
[[bit-assignment-register-table-extraction]].
