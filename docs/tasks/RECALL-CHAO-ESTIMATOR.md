# RECALL-CHAO-ESTIMATOR: a heterogeneity-robust second recall estimate (Chao) alongside Lincoln–Petersen

## Metadata

- Tree ID: `RECALL-CHAO-ESTIMATOR`
- Status: `done` (CLOSED `2026-06-02` — Chao 1987 heterogeneity-robust second recall estimate
  added alongside Lincoln–Petersen; reported as a range; additive, zero behavior change; CI green)
- Roadmap lane: `R15e` (completeness / recall estimation)
- Created: `2026-06-02`
- Owner: repo-local workflow
- Parent context: a **Tier-2 adopt item** from the `LITERATURE-GROUNDING` reach-full-potential
  backlog — specifically `docs/research/grounding/extraction-evaluation.md`: *"Chao's
  estimator as a second N̂ alongside Lincoln–Petersen: the two tiers share prose input
  (positive dependence → LP under-estimates), and Chao tolerates unequal catchability; report
  both as a range."* SpecForge's recall gauge currently reports only the Lincoln–Petersen
  estimate (`ir/completeness.rs::recall_estimate`), which assumes **equal catchability** —
  an assumption the Pattern and Nlp tiers violate (they share the prose and have different
  strengths). This grounds + adds the complementary estimator.

## Prior art (verified)

- **A. Chao, "Estimating the Population Size for Capture-Recapture Data with Unequal
  Catchability," Biometrics 43(4):783–791, 1987** (DOI
  [10.2307/2531532](https://doi.org/10.2307/2531532)); the **Chao1** lower-bound richness
  estimator (Chao, 1984). Robust to heterogeneous "catchability", giving a lower bound on
  total richness from the counts of rarely-captured items.

## The estimator (2-source incidence mapping)

For the two independent extractor tiers (Pattern = `a`, Nlp = `b`) over a `FactKind`:
- **doubletons** `f2` = facts captured by **both** tiers = `overlap`;
- **singletons** `f1` = facts captured by **exactly one** tier = `|a| + |b| − 2·overlap`
  = `distinct − overlap`;
- observed richness `S_obs` = `distinct` = `|a ∪ b|`.

**Chao1**: `N̂_chao = S_obs + f1² / (2·f2)` (the gauge only fires when `overlap > 0`, so
`f2 > 0` and the simple form applies — no division-by-zero, no need for the `f2 = 0`
bias-corrected branch in this code path). `N̂_chao ≥ S_obs` by construction.

Worked example (the existing recall test): Pattern `{A,B,C,D}`, Nlp `{C,D,E,F}` → `overlap=2`,
`distinct=6`, so `f1=4`, `f2=2`, `N̂_chao = 6 + 16/4 = 10` (vs Lincoln–Petersen `N̂=8`). The
two together report a **range**: estimated total **8–10**, remaining misses **2–4**. Because
the tiers are positively dependent (shared prose), LP under-estimates; Chao's heterogeneity
term pushes the estimate up — an honest *wider* bound, not a contradiction.

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: `RecallEstimate` gains `chao_estimated_total` + `chao_estimated_remaining_misses`,
  computed in `recall_estimate` (additive; only when the gauge already fires); unit test(s)
  (the worked example → Chao 10 / misses 4; `N̂_chao ≥ distinct`); `validate` reports the
  LP+Chao **range**; a Chao metric is emitted; book note refreshed in the recall-estimation
  chapter; full CI GREEN; tree CLOSED.

## Non-Goals

- NOT changing the `None`-gating of `recall_estimate` (still requires two tiers + overlap).
  The `f2 = 0` Chao-can-still-estimate extension (where LP is undefined) is a deliberate
  future enhancement, not this tree — it would change the gating contract other code relies
  on.
- NOT a third extractor tier / closed-population `Mₜ` model (a separate larger tree).
- NOT changing any extraction behavior — pure additive measurement.

## Task Tree

- ID: `RECALL-CHAO-ESTIMATOR`
  Status: `active`
  Children: `.1` (design) · `.2` (code + tests + validate report + book + close)

- ID: `RECALL-CHAO-ESTIMATOR.1`
  Status: `done`
  Goal: own + design (this file) — the Chao1 second estimate, the 2-source incidence mapping,
    the additive scope (fires only when the gauge fires), the verified citation, and the
    report-as-a-range plan. Docs-only.
  Acceptance: design + verified citation recorded; registered.
  Verification: passed (`2026-06-02`) — estimator fixed against the real `recall_estimate`
    shape (`a`/`b`/`overlap`/`distinct`); `f1 = distinct − overlap`, `f2 = overlap`;
    `N̂_chao = distinct + f1²/(2·f2)`; worked example computed (Chao 10 vs LP 8 on the existing
    test data); Chao 1987 citation web-verified (DOI 10.2307/2531532); additive scope set
    (no change to `None`-gating). Docs-only.
  Commit: `see Commit Log`

- ID: `RECALL-CHAO-ESTIMATOR.2`
  Status: `done`
  Goal: add the Chao fields + computation to `recall_estimate`; unit test(s); report the
    LP+Chao range in `validate` + a Chao metric; book note; close.
  Acceptance: tests green; `validate` shows the range; CI GREEN; tree CLOSED.
  Verification: passed (`2026-06-02`) — `RecallEstimate` gained `chao_estimated_total` +
    `chao_estimated_remaining_misses`, computed in `recall_estimate` (`ir/completeness.rs`):
    `f1 = distinct − overlap`, `f2 = overlap`, `chao = (distinct + f1²/(2·f2)).round().max(distinct)`.
    Additive — the single construction site updated, `None`-gating unchanged, zero behavior
    change. The existing recall test now also asserts Chao (10 / misses 4 on the worked
    example), plus a new edge test (full overlap ⇒ f1=0 ⇒ Chao = observed, 0 misses). `validate`
    prints `estimated_total LP {} / Chao {} | remaining_misses >= LP {} / Chao {}` (order-
    independent labels, not a dash-range that could read descending), and emits
    `recall_estimate_chao_remaining_misses` + `recall_estimate_relation_chao_remaining_misses`
    metrics. User-friendly book subsection added to `pipeline/evidenceir.md` (and the prior
    `COMPLETENESS-RECALL-GAUGE` "future Chao" forward-reference updated to point at it). Full
    `scripts/run_ci.sh` GREEN (1218→1219 tests; no fixture regression). Tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `RECALL-CHAO-ESTIMATOR.1` | `done` | owned + designed (estimator + verified citation) |
| 2 | `RECALL-CHAO-ESTIMATOR.2` | `done` | code + 2 tests + validate range + 2 metrics + book + close (CI green 1219) |

**Tree CLOSED `2026-06-02`.** The recall gauge now reports a Lincoln–Petersen **and** a Chao
(1987) estimate of the unseen — an honest range robust to the two tiers' unequal catchability —
operationalizing a Tier-2 item from the `LITERATURE-GROUNDING` extraction-evaluation backlog.
Additive measurement, zero behavior change.

## Decisions

- `2026-06-02`: scope to the additive Chao1 second estimate computed alongside LP whenever
  the gauge fires (overlap>0 ⇒ f2>0 ⇒ simple form). Keep `None`-gating unchanged. Report both
  as an honest range; never claim a point truth.

## Blockers

- None. Pure additive measurement over the existing provenance index.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-02` | `.1` | estimator + 2-source incidence mapping fixed vs real `recall_estimate`; worked example (Chao 10 vs LP 8); Chao 1987 DOI verified; additive scope; docs-only | `passed` |
| `2026-06-02` | `.2` | `chao_*` fields + `distinct + f1²/(2·f2)` in `recall_estimate` (additive, None-gating unchanged); recall test asserts Chao 10/misses 4 + new full-overlap edge test (f1=0 ⇒ Chao=observed); `validate` LP/Chao range + 2 Chao metrics; user-friendly book subsection in `pipeline/evidenceir.md` + forward-ref updated; full CI GREEN 1218→1219, no fixture regression; tree CLOSED | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `RECALL-CHAO-ESTIMATOR.1` | `RECALL-CHAO-ESTIMATOR.1 — own + design the Chao second recall estimate (verified citation)` (`be2b95d8`) | docs-only |
| `RECALL-CHAO-ESTIMATOR.2` | `RECALL-CHAO-ESTIMATOR.2 — add the Chao heterogeneity-robust recall estimate alongside Lincoln–Petersen; close tree` | code + 2 tests + validate range + 2 metrics + book; CI green 1219; CLOSED |

## Changelog

- `2026-06-02`: Created — adopt the Chao (1987) heterogeneity-robust richness estimator as a
  second recall N̂ alongside Lincoln–Petersen, per the `LITERATURE-GROUNDING`
  extraction-evaluation backlog. Report both as an honest range; additive, no behavior change.
- `2026-06-02`: **Tree CLOSED.** `.2` done — `RecallEstimate` gained `chao_estimated_total` +
  `chao_estimated_remaining_misses` (`distinct + f1²/(2·f2)`, `f1=distinct−overlap`,
  `f2=overlap`); recall test asserts Chao (10 / misses 4) + a full-overlap edge test;
  `validate` prints the LP/Chao range + 2 Chao metrics; user-friendly book subsection in
  `pipeline/evidenceir.md`. Additive, `None`-gating unchanged, zero behavior change. Full CI
  green (1218→1219), no fixture regression.
