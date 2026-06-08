---
id: register-field-eval-measure-and-surface
title: Register-field extraction quality is measured per-fact, with a "measure & surface" decomposition (RISC-V Debug = 0.588 field-name recall)
answers:
  - "how is register-field extraction quality measured / scored"
  - "what is the register-field eval surface (EvalTask::RegisterField)"
  - "what is the RISC-V Debug register-field recall / precision"
  - "what is the NVMe register-field recall / precision"
  - "why is the strict register-field per-fact score 0 on RISC-V Debug / NVMe"
  - "what is register_field_name_recall / register_field_completeness / register_bit_structure_recall"
  - "where is the register-field gold seed"
date: 2026-06-08
tags: [eval, registers, pdf-variant-digestion, measure-and-surface, scoring-rigor]
evidence: crates/specforge/src/eval.rs (EvalTask::RegisterField, GoldFact::RegisterField, register_field_record_key, register_field_name_recall, register_field_completeness); crates/specforge/src/commands/eval_extraction.rs (register-field surface block); crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json; docs/tasks/PDF-VARIANT-DIGESTION.md (.4a.1/.4a.2)
reverify: ./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json --provider skip 2>&1 | grep -A3 "register-field surface"
---

`PDF-VARIANT-DIGESTION.4a.1`/`.4a.2`. The broadened register-field extraction ([[register-field-table-extraction]])
is now MEASURED per-fact, with WIRE-BASED-100 rigor, so the breadth is trustworthy rather than just present.

**The eval surface (`.4a.1`, additive — no extraction change):** `EvalTask::RegisterField` +
`GoldFact::RegisterField { register, field, bits_high?, bits_low?, bit_width? }` in `eval.rs`. A field's strict
*identity* is the tuple `register | field | offset | width` (a `[high:low]` range and an `offset + width` form
normalize to the same `(offset, width)`; access/reset are excluded — free-string vendor notation). Records are
read straight from `EvidenceIR.register_records` (deterministic, no LLM — like the SWD surfaces). The runtime
stays PDF-agnostic (ADR 0006): the scorer holds NO chip vocabulary — names arrive via the gold answer-key
(`test_data/llm_eval/seed_*.json`) and the extracted records.

**Measure & surface (`.4a.2`/`.4a.3`, owner directive):** the strict per-fact score is a degenerate ~0 on real
docs — but for OPPOSITE reasons per doc — so `eval-extraction` DECOMPOSES the register-field result into four
honest numbers (`register_field_name_recall` + `register_bit_structure_recall` are the two recall views;
`register_field_completeness` reports the gaps):

- **field-name recall** (register- and bit-agnostic) — gold field NAMES found anywhere in the extracted field
  set (the "names work" view).
- **bit-structure recall** (register-scoped, mnemonic-agnostic, `.4a.3`) — gold `(offset, width)` extents found
  in a register whose name TOKEN-matches the gold register (`register_name_has_token`), pooling the register's
  page-split fragments. Register-scoped because the same mnemonic/extent recurs across registers (NVMe
  `CAP.CSS` 44:37 vs `CC.CSS` 6:4). This is the "bits work" view.
- **register-name association gap** — registers with a real (non-`register_table_*`-placeholder) name.
- **bit-extent completeness gap** — extracted fields carrying a bit position.

**RISC-V Debug Spec 1.0 result** (gold = `dmstatus` 20 fields + `dmcontrol` 14, bit positions transcribed by
hand from the spec's bit-layout graphics, independently of the extractor): **field-name recall 20/34 = 0.588**
(`dmcontrol` 14/14 = 100%; `dmstatus` 6/20 — Docling dropped the middle page-fragment of the page-split
`dmstatus` field table); **register-name association 0/60** (the extractor reads the per-field `Field|…|Access|
Reset` table but not the heading above it); **bit-extent completeness 0/179** (bit positions live in the
layout GRAPHIC above the table, which the reader does not parse) → strict per-fact **0.000**, surfaced not
hidden. The 14 name-misses were independently verified REAL (the apparent near-matches were spurious
single-letter fragments / different registers' fields), so the recall is honest, not gold-spelling drift.

**NVMe Base Spec 2.0a result (`.4a.3`) — the INVERSE failure** (gold = CAP 15 + CC 8 + CSTS 6 = 29 fields, bit
ranges + mnemonics transcribed from the `Bits | Type | Reset | Description` tables): **bit-structure recall
27/29 = 0.931** (the 2 misses are `CAP.CRMS` at the register top + `CC.EN` on its own page fragment — both real
drops); **field-name recall 0/29** (NVMe puts the mnemonic in the DESCRIPTION and the bit-range string in
`field_name`); register-name association **44/44** (caption-derived names like `Offset 0h: CAP - …`);
bit-extent completeness **199/199**. So NVMe captures bits + register names but not mnemonics — the exact
inverse of RISC-V Debug (names but not bits/register-names). The strict per-fact score is `0.000` on BOTH, for
opposite reasons, which is why the decomposition (two recall views) is essential. Measured against the
verified-current persisted evidence (mtime postdates the last extraction commit; the corpus PDF is git-tracked
so a fresh re-ingest reproduces it).

**Fix leaves identified (future trees, NOT measurement):** (1) associate the register NAME from the preceding
heading (RISC-V); (2) parse the bit-layout graphic for field bit positions (RISC-V); (3) extract the field
MNEMONIC from the description (NVMe). Each is now quantified, so a fix can be measured against these golds.
