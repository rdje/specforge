---
id: vlm-table-strategy
title: VLM (Qwen2.5VL) reads table images to reclassify "unknown" tables — the second, best-wins table strategy
answers:
  - "how does the VLM understand tables / can a VLM read PDF tables"
  - "what is the VLM table strategy / PDF-VARIANT-DIGESTION.2b"
  - "how are unknown tables reclassified by the VLM"
  - "does encryption block the VLM from reading tables (no)"
date: 2026-06-07
tags: [vlm, qwen, tables, enrich, pdf-variant-digestion, multi-strategy, lever-a]
evidence: crates/specforge/src/commands/enrich.rs (vlm_image_query, build_table_classify_prompt, parse_vlm_table_kind, classify_unknown_tables_via_vlm)
reverify: ./target/debug/specforge enrich generated/source_ir/<key>/source_ir.json --vlm-provider ollama --vlm-model qwen2.5vl:7b   # prints tables_reclassified_by_vlm
---

The deterministic header-grammar classifier (ingest) is strategy 1; the VLM is strategy 2, best-wins-per-PDF
([[feedback_multi_strategy_best_wins]]). docling renders a `table_region` image per table (asset_id →
VisualAsset.image_path); **Qwen2.5VL reads those images directly** — validated `2026-06-07` on the RISC-V
`dmcontrol` table: the model returned `{"kind":"register_field","fields":["haltreq","resumereq","hartreset",
"ackhavereset","ackunavail"]}` straight from the picture. Encryption is irrelevant — docling renders the
pages regardless ([[pdf-encryption-and-read-access]]); the VLM never touches the (unreliable) Read tool.

Wiring (`enrich.rs`, gated behind `--vlm-provider`; `skip` = no-op): `classify_unknown_tables_via_vlm` sends
each `unknown`-kind table's image to the VLM (`vlm_image_query`, shared with diagram enrichment) with
`build_table_classify_prompt`, and `parse_vlm_table_kind` maps the JSON reply to a `TableKind` to APPLY.
**Best-wins:** only `unknown` tables are touched — confident deterministic kinds are never overridden. The
reclassification is written back to `source_ir.structured_tables[].table_kind`, so a re-run of `evidence`
fires the deterministic extractor for the now-known kind. Run order: `ingest` → `enrich --vlm-provider
ollama` → `evidence`.

Conservative mapping (no over-reclassification): `register_field` → **not** reapplied (the deterministic
grammar path `synthesize_register_field_tables` already recovers these from `unknown`); `table_of_contents`/
`other` → left `unknown` (correctly unextracted). Reapplied kinds: signal_description, encoding,
timing_parameter, feature_matrix, register_map.

**Verification gate (the VLM proposes, structure disposes) — essential, no-garbage.** The VLM OVER-classifies:
on RISC-V it labelled register-field (`Field|…|Access|Reset`) and operation (`Op|Address|Value`) tables as
`signal_description`, which then yielded garbage "signals" (`FIELD/EXECUTE/HALT/TRIGGER`). So a proposed kind
is APPLIED only when `vlm_kind_structurally_consistent` confirms the table HEADER matches that kind (e.g.
signal_description needs a name/signal column AND a width/direction/source column AND is not a field+access/
reset table). Measured on RISC-V: WITHOUT the gate, 22 reclassified → +9 real DMI signals but +5 garbage;
WITH the gate, **1 reclassified (the real DMI table) → 9 genuine DMI signals (`REQ_*/RSP_*`), 0 garbage**.
This is "best-wins must be MEASURED, no faking" ([[feedback_scoring_rigor]]) — same proposer/verifier pattern
as the WIRE-BASED-100 garbage detection. +2 hermetic tests (`parse_vlm_table_kind`,
`vlm_kind_structurally_consistent`).

**VLM EXTRACTION / grid repair (`.2b'`).** ~10% of corpus tables (262) are *degenerate* — Docling failed to
structure them (≤1 column). `repair_degenerate_tables_via_vlm` asks the VLM to TRANSCRIBE the table image to
a JSON grid (`build_table_extract_prompt` → `parse_vlm_grid`, serde_json, ≥2 columns) and REPLACES the
degenerate `header_rows`/`body_rows` so the deterministic extractors can run — best-wins at the STRUCTURE
level (Docling grid vs VLM grid). Kind set only when the repaired header is structurally consistent (same
gate). Run order: `ingest` → `enrich --vlm-provider ollama` (repair → classify) → `evidence`. Live demo:
RISC-V → 2 degenerate tables grid-repaired, 0 errors. +1 hermetic test (`parse_vlm_grid`).

**Scaling finding (`2026-06-08`):** `enrich --vlm` fires a VLM call per unknown/degenerate table (plus every
diagram), so on TABLE-HEAVY docs (NVMe has 100s of tables) a full-doc pass is prohibitively slow. The VLM is
therefore a TARGETED / SAMPLED tool, not a default full-doc pass: apply it to a specific doc's degenerate
tables (`.6`) or a random audit sample (`.4b`), not the whole corpus. (The `.2b'` end-to-end grid-repair is
proven on RISC-V — 2 tables repaired — and by the `parse_vlm_grid` hermetic test; NVMe full-enrich was
stopped as impractically slow, confirming the targeted-only design.)
