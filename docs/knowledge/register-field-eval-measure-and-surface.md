---
id: register-field-eval-measure-and-surface
title: Register-field extraction quality is measured per-fact, with a "measure & surface" decomposition (RISC-V Debug = 0.588 field-name recall)
answers:
  - "how is register-field extraction quality measured / scored"
  - "what is the register-field eval surface (EvalTask::RegisterField)"
  - "what is the RISC-V Debug register-field recall / precision"
  - "why is the strict register-field per-fact score 0 on RISC-V Debug"
  - "what is register_field_name_recall / register_field_completeness"
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

**Measure & surface (`.4a.2`, owner directive):** on docs where the extractor recovers field NAMES but not the
owning register name (synthetic) or the bit extent, the strict per-fact score is a degenerate ~0 that hides
where the value is. So `eval-extraction` DECOMPOSES the register-field result into three honest numbers
(`register_field_name_recall` is register- and bit-agnostic; `register_field_completeness` reports the gaps):

- **field-name recall** — gold field NAMES found anywhere in the extracted field set (the "what works" view).
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

**Two fix leaves identified (future tree, NOT measurement):** (1) associate the register NAME from the
preceding heading; (2) parse the bit-layout graphic for field bit positions. Until then, RISC-V-Debug-class
register extraction is name-level only — known and now quantified. NVMe is the contrasting case: it DID capture
`bit_width` ([[register-field-table-extraction]]), so `.4a.3` golds it for a meaningful strict per-fact score.
