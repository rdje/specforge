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

**Live findings (2026-06-10, qwen2.5:14b-instruct):** the gauge first made visible that the
canonical artifacts carried the Pattern surface — APB 28.6% (Info only), AHB 60%, degraded CHI
69%, AXI 91% (majority-flagged) — which spawned the `LLM-PRIMARY-PROMOTION` tree. After its
`.2`/`.3`: APB and AHB canonical artifacts carry the PROMOTED surface (gauges 23.8% / 33.3%),
AXI was reverted to Pattern pending the `.3a` coordinated-subject typo fix (its gauge honestly
re-reads 91/100 — reproduced EXACTLY across two independent measurements, a strong
oracle-reproducibility datapoint). I2S end-to-end converge demo: 2 passes, gauge 1/1
not-entailed (`SCK must_be_asserted` read from an edge-synchronization permission sentence — a
genuine mis-extraction, correctly flagged). See [[llm-primary-promotion-stage]].
Related: [[llm-primary-condition-subject-gate]], [[llm-primary-permissive-frame-gate]],
[[conformal-tier-agreement-degenerate]], [[nli-gate-real-apb-validation]].
