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
  - "why did the promoted surface lose the AXI reset temporal rules (DEASSERTED vs LOW)"
  - "does a replaced constraint surface get polarity refinement (apply_persisted_polarity_to_constraints)"
  - "must a post-build signal_constraints replace re-apply build-path invariants"
  - "where is the default-flip decision packet / should promote-constraints-llm become the default"
  - "what did the corpus promotion sweep measure (gauge deltas per doc)"
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

`.3b` (`2026-06-10`) closed the SECOND AXI gap and put **all three wire docs (APB/AHB/AXI) on
promoted canonical surfaces with every gold gate at 1.000**. Durable invariant: **a post-build
`signal_constraints` replace MUST re-apply the build path's polarity refinement** — the build
collapses `MustBeDeasserted`→`MustBeLow`/`MustBeHigh` via the document's resolved polarity
(`apply_signal_polarity_to_constraints`) BEFORE persisting, so every kind consumer (the typed
temporal layer, NLI gauge claim text, ISF adapter) assumes a refined surface; the promotion
replace skipped it, so promoted AXI fed the temporal layer symbolic `DEASSERTED` where gold
(and the document's persisted `active_high` records for SYSCOREQ/SYSCOACK) say `LOW` — both
reset rules lost even with correct spellings. Fix: `pub(crate)
evidence::apply_persisted_polarity_to_constraints` (resolved map from the artifact's persisted
`signal_polarities`) called in `promote_constraints` BEFORE dedup (the canonical key sees the
refined kind). Ungrounded polarity keeps the symbolic kind — never guessed. Probe method that
pinned it: synthetic /tmp evidence copy differing ONLY in the two reset kinds → rules identical
to gold except `DEASSERTED`≠`LOW`; antecedent parsing (`when `-stripping) and actor grounding
(connectivity-derived) probed and CLEARED. Honest gauge note: refinement trades NLI-gauge
optics for gold-gate correctness (the judge marks "must be LOW" not-entailed vs source "must
be deasserted" — it lacks the polarity fact); AXI promoted gauge reads 48.0% vs 91.0% Pattern.
`.4` (`2026-06-10`) measured the corpus: 12 docs swept on REDIRECTED copies — the gauge
improves on 14/15 measurable docs (DTI 99.1%→73.3% at 114→30 records; CHI 69.2%→16.7%;
OpenCAPI 100%→25%; HBM2 GROWS 14→20 and cleans 85.7%→40%); recall cost quantified per-item =
~3 genuine losses, all on the ungated AXI+ACE doc (table-cell-row subjects + a coordinated
stability sentence — levers R1/R2 in the tree). **The owner-visible DEFAULT-FLIP DECISION
PACKET lives in `docs/tasks/LLM-PRIMARY-PROMOTION.md` (§ Default-flip decision packet);
recommendation: FLIP for live-NLP converge runs; `.5` = the flip execution, OWNER-GATED — do
not flip without the owner's explicit go.** Composite validation scores are comparable only
within a validator version (the 2026-04→06 snapshot refresh re-scored the untouched
AXI-Stream artifact 90→67 purely from validator evolution).
Related: [[extraction-quality-gauge-standing]], [[llm-primary-condition-subject-gate]],
[[llm-primary-permissive-frame-gate]], [[model-misspelled-subject-snap]].
