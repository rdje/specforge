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

Live (`2026-06-10`, APB end-to-end converge with `--vlm-provider skip --nlp-provider ollama
--promote-constraints-llm`): stabilized in 2 passes (Pattern+NLP3 surface 18 records) →
promotion 18 → 21 grounded → 21 kept over 15 sentences (on APB the promoted surface GROWS —
the `.8` validity recovery reads facts the pattern grammar mis-read; the shrink case is the
dense-spec shape) → gauge on the promoted surface 5/21 not-entailed (23.8%, vs 28.6% on the
old Pattern artifact) → eval gate HELD on the promoted canonical artifact:
signal_constraint P=R=F1=1.000 (6/6), WIRE-BASED-100 filtered relations 1.000, doc-level
recall 6/6, conformal empirical_error 0.000.
Related: [[extraction-quality-gauge-standing]], [[llm-primary-condition-subject-gate]],
[[llm-primary-permissive-frame-gate]].
