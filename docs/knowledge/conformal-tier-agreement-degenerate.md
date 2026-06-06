---
id: conformal-tier-agreement-degenerate
title: Tier-agreement is a degenerate conformal axis — the extraction tiers complement, they don't corroborate
answers:
  - "why doesn't the NLI-oracle conformal calibration produce a threshold"
  - "is tier-agreement a good confidence axis for conformal calibration"
  - "do the Pattern and Nlp extraction tiers find the same constraints"
  - "what confidence axis correlates with extracted-constraint correctness"
  - "why is conformal calibration still blocked at CHI scale"
date: 2026-06-06
tags: [conformal, calibration, extractor-tier, nli, corpus-hardening]
evidence: docs/tasks/TABLE-GRITS-CONFORMAL.md
reverify: grep -n "tier_count_by_fact_key\|nli_conformal_pass" crates/specforge/src/ir/nli_verify.rs
---

The NLI-oracle conformal calibration uses **tier-agreement** (how many `ExtractorTier`s found a fact)
as its confidence axis. On real docs this axis is **degenerate**: the `Pattern` (deterministic) and
`Nlp` (LLM) tiers extract **disjoint** constraints — on CHI they share **0 / 115** fact-keys despite
both using the same `signal_constraint_fact_key`. The tiers **complement** (cover different facts);
they do **not** corroborate (agree on the same fact), so nearly every fact is tier-1 → no confidence
variance → conformal cannot discriminate. **Scale does not help** (CHI n=159, still no α=0.2
threshold) — the axis was the problem, not the sample count.

The **usable** axis is *which* tier found the fact, and it correlates with correctness: on CHI, `Nlp`
constraints are NLI-entailed **33%** vs `Pattern` **11%** — the LLM tier is ~3× more reliable on
complex prose. But a useful threshold also needs adequate base extraction quality (a ≥80%-correct
subset must exist), and CHI is only **17%** entailed overall, so conformal stays blocked there. That
itself is a **corpus-hardening** signal: the Pattern extractor over-generates on CHI's complex text
(**89%** NLI-not-entailed vs **29%** on APB). Caveat: the NLI verdict is a *noisy* oracle (some
not-entailed are NLI false-negatives on complex claims), but the Pattern≪Nlp gap is a strong
*relative* signal. See `[[nli-entailment-verifier]]` and `docs/tasks/TABLE-GRITS-CONFORMAL.md`.
