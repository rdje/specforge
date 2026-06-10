---
id: extraction-quality-gauge-standing
title: The NLI extraction-quality gauge is persisted on EvidenceIR and re-measured by converge
answers:
  - "where does the nli-verify measurement go / is the extraction-quality gauge persisted"
  - "what is EvidenceIr.extraction_quality_gauge and who writes it"
  - "how does converge report per-document extraction quality after stabilization"
  - "what are the extraction_quality_* validate metrics and when do they read n/a"
  - "when does evidence_extraction_quality_majority_not_entailed or _gauge_stale fire"
  - "why does a rebuild drop the extraction-quality gauge"
  - "what is the standing per-doc quality report wired into converge/CI"
  - "how erroneous are the canonical Pattern constraint surfaces on the persisted corpus"
date: 2026-06-10
tags: [extraction-quality, nli, validation, converge, evidence-ir, production-readiness]
evidence: crates/specforge/src/ir/evidence.rs (ExtractionQualityGaugeRecord); crates/specforge/src/ir/nli_verify.rs (gauge_from_conformal_pass, gauge_is_stale); crates/specforge/src/commands/nli_verify.rs (measure_and_persist_gauge); crates/specforge/src/commands/converge.rs (measure_extraction_quality); crates/specforge/src/commands/validate.rs (extraction_quality_* metrics/findings); crates/specforge/test_data/kg_quality/extraction_quality_gauge_persisted_gold; docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.0)
reverify: cargo run -p specforge --quiet -- validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>&1 | grep -A2 "Extraction-Quality Gauge"
---

The NLI-oracle extraction-quality gauge (fraction of `signal_constraints` whose own source
sentence does NOT entail them) is a STANDING persisted measurement, not a one-off print:

- **Persist**: `nli-verify` back-annotates `EvidenceIr.extraction_quality_gauge`
  (`ExtractionQualityGaugeRecord`: model, constraints_total, entailed/not_entailed/abstained,
  not-entailed constraint ids). Built by `gauge_from_conformal_pass` from the ONE existing
  `nli_conformal_pass` — never a second LLM sweep. A pass that labeled nothing (provider dead →
  all `Unknown`) is NOT persisted: vacuous data must not overwrite a real measurement.
- **Re-measure**: `converge` calls the same `measure_and_persist_gauge` after the loop
  stabilizes (and after any rescan step), when `--nlp-provider` ≠ `skip`, and prints the gauge
  in its convergence summary. Rebuilds drop the field by construction
  (`carry_forward_existing_knowledge` never carries it) — new surface ⇒ new measurement.
- **Report provider-free** (the CI-safe surface): `validate <evidence-ir>` emits
  `extraction_quality_labeled/_not_entailed/_abstained/_not_entailed_pct` (`n/a` until
  measured — honest absence, the `recall_estimate_pct` precedent), an Info finding whose
  `related_ids` are the not-entailed ids, Warning
  `evidence_extraction_quality_majority_not_entailed` when not_entailed×2 > labeled (scale-free
  "more wrong than right" — no corpus-tuned threshold), and Warning
  `evidence_extraction_quality_gauge_stale` when the surface changed since measurement (count
  mismatch OR a measured id gone — catches `extract-constraints-llm`'s id re-keying replace).
  Locked by kg fixture `extraction_quality_gauge_persisted_gold` + 5 unit tests.

**Live finding (2026-06-10, qwen2.5:14b-instruct):** the CANONICAL persisted artifacts still
carry the Pattern extractor's surface — the cleaned LLM-primary surfaces (`.3a`/`.3b`/`.4`,
P=1.000 ×3) were only ever measured on /tmp redirected copies, never promoted. The gauge makes
that visible: APB 4/14 (28.6%, Info only), AHB 9/15 (60%), degraded CHI 9/13 (69%), AXI 91/100
(91%) — all three majority-flagged. I2S end-to-end converge demo: 2 passes, gauge 1/1
not-entailed (`SCK must_be_asserted` read from an edge-synchronization permission sentence — a
genuine mis-extraction, correctly flagged). Promoting the LLM-primary surface into canonical
artifacts is the obvious follow-up lever this gauge now motivates.
Related: [[llm-primary-condition-subject-gate]], [[llm-primary-permissive-frame-gate]],
[[conformal-tier-agreement-degenerate]], [[nli-gate-real-apb-validation]].
