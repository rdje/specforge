# Recall Estimation & the Typed CompletenessReport — design (research `.5`)

> Owned by `INTENT-COMPLETENESS-RESEARCH.5` — the last open research leaf. Pulls
> the recall-estimation machinery (framework §8, corrected by the `.6` literature
> grounding) and the unifying report (framework §10, now partly realized as the
> shipped `Completeness Summary`) into one **implementable blueprint** for the
> next phase. Design only — the impl needs the precondition noted in §3.

## 0. Where we are

The detector phase shipped four live `validate` completeness surfaces:
convergence report, register bit-tiling, region accounting (unexplained
intent-bearing tables), and the aggregated **Completeness Summary**
(`completeness_candidate_misses`). Those find *specific, located* misses. What
remains is the **calibrated residual-recall estimate** — "of everything the
document states, what fraction did we capture?" — which no located detector can
give, and the **typed report** that carries it durably.

## 1. The recall estimator — capture–recapture (corrected per `.6`)

Run independent extractors over the same regions; the overlap of what they each
recover estimates the unseen population. The `.6` survey corrected the framework's
naive version on three points that the design MUST honor:

- **Estimator:** use **Chao's 1987 Mh** `N̂ = D + f₁²/(2·f₂)` (with a confidence
  interval), where `D` = distinct facts found, `f₁`/`f₂` = facts found by exactly
  one / exactly two extractors. Lincoln–Petersen `N̂≈a·b/m` is only the 2-pass
  fallback; it degrades in the singleton-heavy regime chip extraction hits.
  Estimated misses `≈ N̂ − D`, reported as a **lower bound** (capture–recapture
  empirically *under*-estimates content — Briand/El Emam, Petersson/Wohlin).
- **Heterogeneity (the load-bearing requirement):** extractors must be as
  **independent** as possible. Two LLM passes on one backbone are positively
  correlated → they co-miss the same hard regions → the estimate is *optimistic*
  (hides misses). Require **≥3 heterogeneous** extractors. SpecForge already has
  genuinely different ones: the **pattern Tier-1/2** extractor vs the **LLM
  Tier-3** (`nlp-enrich`/`extract-contracts`) vs the **VLM** (`enrich`) — three
  different modalities/mechanisms, the cleanest independence available. (A future
  fourth: a two-source fusion against an independent structural KG, AssertionForge
  `.6`.)
- **Blind spots are invisible to it:** a fact missed by *every* extractor never
  enters the overlap statistics, so capture–recapture cannot see systematic blind
  spots — those are caught only by the ontology coverage matrix (`.2`). The two
  are complementary; the report must carry both.

**Fact identity (the engineering crux):** capture–recapture needs "the same fact
found by two extractors" to be *detectable* — a canonical fact key
(normalized signal name + relation/attribute + value), with HNEN-style synonym/
unit canonicalization (`.6`) applied first so terminology drift doesn't split one
fact into two "uniques" (which would inflate the miss estimate).

## 2. The STOP-OR-REINSPECT bridge

The Briand/El Emam rule turns the estimate into a *decision*: re-extract if the
estimated remaining misses exceed a threshold. This gives the shipped
`R15C-CONVERGENCE-REPORT` anchored-rescan loop a **principled statistical
stopping criterion** to replace its fixed pass cap: stop when the
capture–recapture estimate of remaining misses falls below ε (or when a rescan
adds no new distinct facts). A direct research→shipped-code bridge.

## 3. Implementation precondition

The estimator needs the per-extractor fact sets *with a shared fact identity*.
Today the three extractors write into the **same** EvidenceIR (merged), so "which
extractor found this fact" and "did two find the same fact" are not separable.
**Precondition tree:** tag each emitted fact with its producing extractor (or run
the extractors in a comparison harness that records per-extractor sets) + define
the canonical fact key. Until then the estimator can't be computed — this is why
`.5` is *design*, not impl, and the impl is a future owned tree.

## 4. The typed `CompletenessReport`

Generalize the shipped `Completeness Summary` (a `validate`-side aggregation)
into a **typed, persisted** record carried on the IR (like
`EvidenceConvergenceReport`), so completeness is a first-class artifact, not just
console output:

```
CompletenessReport {
  // located detectors (already shipped as Completeness Summary inputs)
  register_field_overlaps, register_field_interior_gaps: usize,
  unexplained_intent_bearing_tables: usize,
  prose_residuals: usize,
  convergence: Option<{ converged: bool, passes_run, total_new_facts }>,
  // recall estimate (this design; needs §3 precondition)
  recall_estimate: Option<{
    distinct_facts: usize,
    estimated_total: f64,        // Chao Mh N̂
    estimated_remaining_misses: usize,   // lower bound
    confidence_interval: (f64, f64),
    extractors: Vec<String>,     // the heterogeneous set used
    assumptions: Vec<String>,    // independence/catchability caveats, printed
  }>,
  // systematic blind spots (ontology coverage matrix, .2) — invisible to capture-recapture
  blind_spot_categories: Vec<String>,
  // honest headline
  candidate_misses: usize,
}
```

`validate` reports it; the headline stays "N candidate misses; estimated residual
recall R% ± e (lower bound, assumptions: …)" — never "complete." This is the
residual-honesty doctrine scaled to a single calibrated number with its caveats
attached.

## 5. Prioritized follow-on trees (from this design)

1. **Per-extractor fact tagging + canonical fact key** (§3 precondition) — unlocks
   capture–recapture; also valuable on its own (provenance: which tier found what).
2. **Capture–recapture recall gauge** (§1) — Chao Mh over the 3 heterogeneous
   extractors; report as a lower bound with assumptions.
3. **Typed `CompletenessReport`** (§4) — promote the `Completeness Summary` to a
   persisted record + fold in the recall estimate + blind-spot categories.
4. **STOP-OR-REINSPECT** (§2) — wire the estimate into the convergence loop's
   stopping criterion.
5. **Competency-question battery** (framework §8.2) — query-answerability
   completeness, paired with population completeness (region accounting).

## 6. Honest status

The recall estimator is **designed, not built** — it is blocked on the §3
per-extractor-identity precondition, recorded honestly rather than faked. The
*located* completeness detectors + the aggregated headline ARE shipped and
corpus-validated; this blueprint defines the path from "located misses" to a
"calibrated residual-recall number," when the precondition is met.
