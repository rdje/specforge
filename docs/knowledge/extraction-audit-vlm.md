---
id: extraction-audit-vlm
title: audit-extraction — VLM proposer/verifier precision estimate over the broadened table-driven extraction
answers:
  - "how is the precision of the broadened (non-gold) extraction measured / estimated"
  - "what is audit-extraction / PDF-VARIANT-DIGESTION.4b"
  - "how do you audit registers/signals against the table image with the VLM"
  - "what is the table-kind precision estimate and the flagged-mismatch list"
  - "is the extraction audit chip-spec-PDF agnostic (yes)"
date: 2026-06-08
tags: [vlm, qwen, audit, tables, precision, pdf-variant-digestion, lever-a, scoring-rigor]
evidence: crates/specforge/src/commands/audit_extraction.rs (audited_kind, select_sample, build_audit_prompt, parse_audit_verdict, aggregate); reuses commands/enrich.rs::vlm_image_query + ir/evidence.rs::is_register_field_header
reverify: ./target/debug/specforge audit-extraction generated/source_ir/1_0_risc_v_debug_specification/source_ir.json --sample 8           # plan-only: lists 8 sampled intent-bearing tables, no VLM calls
---

Only four wire-based specs (APB/AHB/AXI/SWD) have gold-verified precision, but the `.2`/`.2b`/`.2c`
work broadened table-driven extraction (registers/fields/signals) across the whole 82-PDF corpus. `audit-extraction`
estimates the precision of the *rest* WITHOUT a per-document gold — a **proposer/verifier audit** ([[feedback_scoring_rigor]]):
the deterministic pipeline already *proposed* a table classification; the VLM (Qwen2.5VL) independently *verifies* it by
re-reading the table's rendered image — the [[vlm-table-strategy]] `.2b` consistency gate run as an AUDIT.

**Flow** (`commands/audit_extraction.rs`, gated behind `--provider`; default `skip` = plan-only, the CI-safe path):
1. `audited_kind(table)` selects the **intent-bearing** tables by the SAME structural predicates the extractors use —
   `RegisterMap`/`SignalDescription`/`Encoding`/`TimingParameter` table-kinds, PLUS `Unknown` tables whose header matches
   `is_register_field_header` (the `.2` register-FIELD grammar). Noise tables (`table_is_noise`) and image-less tables excluded.
2. `select_sample(ids, n, seed)` picks a **bounded, reproducible** sample — order candidates by `FNV-1a(seed \0 table_id)`,
   take the first `n` (no RNG dep; same seed → same sample; verified seed-sensitive on real data). The VLM is a TARGETED/SAMPLED
   tool, not a full-doc pass (the `.2b` scaling finding — a VLM call per table is too slow on table-heavy docs).
3. Live (`--provider ollama`): for each sampled table, `build_audit_prompt(kind)` asks the VLM (via the shared
   `enrich::vlm_image_query`) "an extractor read this as a <kind> table — looking only at the image, is that correct?"
   `parse_audit_verdict` reads `{"consistent": bool, "reason": str}` (tolerant of fences/prose; no verdict = an error, never agreement).
4. `aggregate` → **`table_kind_precision_estimate` = consistent / judged** (errors excluded from the denominator; `None` when
   nothing was judged — never a fabricated number) + a **flagged-mismatch list** (every disagreement printed by name for human review).

**Honest naming:** it is a *table-kind* precision estimate (does the VLM agree a sampled table is the kind we extracted from) —
it catches the dominant false-positive mode (extracting records from the wrong kind of table); it is NOT a per-field fact check,
and the metric name says so. The VLM is an imperfect oracle → it is an ESTIMATE, surfaced with flagged items, never a score.

**Agnostic by construction** ([[feedback_no_hardcoded_chip_spec_names]], ADR 0006): selection and judgment are purely by table
STRUCTURE (register/field/signal/encoding/timing — universal digital-design "how" vocabulary, the [[temporal-logic-choice]] /
LOGIC-LEVEL-BOUNDARY class); the runtime prompt carries no chip/vendor/protocol names (a hermetic test asserts none leak);
sampling is structural, never keyed on document identity. Verified live: RISC-V Debug `table_0080` → VLM `consistent` → estimate
1.000, 0 flagged. 5 hermetic tests (classification, deterministic+seed-sensitive sampling, tolerant verdict parse, agnostic prompt,
precision/flagging math); plan-only proven on RISC-V (78 intent-bearing tables) + I2C (7 timing tables).
