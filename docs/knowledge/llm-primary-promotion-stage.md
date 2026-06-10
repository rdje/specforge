---
id: llm-primary-promotion-stage
title: converge --promote-constraints-llm replaces the Pattern constraint surface post-stability
answers:
  - "how do I get the clean LLM-primary constraint surface onto the canonical artifacts"
  - "what does converge --promote-constraints-llm do and when does it run"
  - "why does constraint promotion run outside the convergence loop"
  - "why can't extract-constraints-llm run inside a converge pass"
  - "what happens to the extraction-quality gauge when the constraint surface is replaced"
  - "how is a promoted constraint surface visible in the extraction manifest"
  - "is the LLM-primary promotion a recall improvement"
date: 2026-06-10
tags: [extraction-quality, llm-primary, promotion, converge, constraints, manifest]
evidence: crates/specforge/src/commands/extract_constraints_llm.rs (promote_constraints); crates/specforge/src/commands/converge.rs (maybe_promote_constraints); crates/specforge/src/ir/extractor.rs (record_surface_manifest); docs/tasks/LLM-PRIMARY-PROMOTION.md
reverify: cargo test -p specforge --lib promote_constraints_records 2>&1 | tail -2
---

`converge --promote-constraints-llm` (opt-in, default OFF) replaces the final EvidenceIR's
Pattern `signal_constraints` with the LLM-primary grounded surface and rebuilds
Semantic → Intent → adapter ONCE. Load-bearing mechanics:

- **Post-stability placement is FORCED**, not stylistic: the convergence loop errors on any
  pass-to-pass fact-count shrink, and promotion IS a shrink by design (noisy 102 → clean ~50).
  Order: stabilize → rescan step → promote → downstream rebuild → quality gauge (so the
  standing gauge measures the PROMOTED surface).
- The flag with `--nlp-provider skip` is an **explicit early error** (an opt-in that silently
  does nothing is worse); provider-free runs never promote — the deterministic Pattern surface
  stays the CI-safe default.
- The swap is **manifest-recorded**: `ExtractionManifest.record_surface_manifest` (same
  replace-per-surface semantics as `record`) writes surface `signal_constraints`, extractor
  `constraints.llm_primary`, tier Nlp, produced/kept — visible to the fingerprint/cluster
  plane. Standalone `extract-constraints-llm` records it too.
- A replace **drops any persisted `extraction_quality_gauge`** (its measured constraint ids
  are definitively gone); converge immediately re-measures, standalone runs leave honest
  absence (`validate` reports `n/a`).
- **Recall universe = the Pattern surface's own distinct source sentences** (one LLM call
  each) — promotion is a refinement/precision play, never a discovery pass
  ([[llm-primary-must-be-value-recall]] measured 16/16 doc-level gold recall inside that
  universe on the wire docs).
- Default-flip is a SEPARATE owner-visible decision gated on the `.4` corpus sweep
  (`docs/tasks/LLM-PRIMARY-PROMOTION.md`); until then promotion is per-run opt-in.

Live (`2026-06-10`): APB end-to-end converge (`--promote-constraints-llm`): 2 passes → 18 → 21
kept (on APB the promoted surface GROWS — validity recoveries; the shrink case is the
dense-spec shape) → gauge 23.8% vs 28.6% → all gates HELD. `.3` canonical-artifact battery:
AHB 15→12 kept, gauge 60.0%→33.3%, ALL gates 1.000 → **APB+AHB canonical artifacts now carry
the promoted surface**. AXI 102→50 kept, gauge 91.0%→36.0%, BUT seed_axi_temporal FAILED
(1/3) → **AXI REVERTED** (Pattern surface restored, gates re-verified 1.000, gauge re-measured
— exactly 91/100 again, a strong oracle-reproducibility datapoint). Root cause of the AXI
gate failure (probed temp 0, ×2 identical): the coordinated-subject sentence "SYSCOREQ and
SYSCOACK must be deasserted when ARESETn is asserted." — the model emits both records but
MISSPELLS the first subject (`SYCOREQ`), and entity typing rightly rejects the undeclared
token. NEW defect class: model-misspelled subject on an otherwise-grounded proposal; fix =
`.3a` document-grounded typo snap (edit distance 1, candidate must literally appear in the
sentence AND pass signal typing, exactly one candidate — else the honest drop stands).
Default-flip stays blocked until `.3a` re-clears AXI.
Related: [[extraction-quality-gauge-standing]], [[llm-primary-condition-subject-gate]],
[[llm-primary-permissive-frame-gate]].
