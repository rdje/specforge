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
timing_parameter, feature_matrix, register_map. +1 hermetic test (`parse_vlm_table_kind`). Next: VLM
EXTRACTION (read rows/fields from the image into typed records as a `Vlm`-tier fact) where the deterministic
extractor still yields nothing.
