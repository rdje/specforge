# COMPLETENESS-RECALL-GAUGE: a calibrated capture–recapture recall estimate

## Metadata

- Tree ID: `COMPLETENESS-RECALL-GAUGE`
- Status: `done`
- Roadmap lane: `R15e` (KG-quality / completeness)
- Created: `2026-06-01`
- Owner: repo-local workflow

## Goal

Now that `PER-EXTRACTOR-FACT-TAGGING` records, per independent extractor, the
facts each found (with a canonical key for overlap), compute the
**capture–recapture recall estimate** designed in
[`recall-estimation-and-report.md`](../research/recall-estimation-and-report.md):
estimate the unseen population of signal-constraint facts from the overlap of the
Pattern and Nlp tiers, and surface it in `validate` as a calibrated
**lower bound on remaining misses** — honestly, with its assumptions printed.

## Method (first slice — 2 extractors)

For `fact_kind = SignalConstraint`, from `EvidenceIr.fact_provenance`:
- `a` = distinct canonical keys the **Pattern** tier found; `b` = the **Nlp** tier;
  `m = |a ∩ b|` (overlap); `D = |a ∪ b|` (distinct found).
- With exactly two extractors, use the **Lincoln–Petersen** estimator (the
  documented 2-pass fallback per the design; Chao Mh needs the singleton/doubleton
  structure of ≥3 occasions): `N̂ = (|a|·|b|) / m`; `estimated_remaining_misses =
  max(0, round(N̂) − D)`; `estimated_recall = D / N̂`.
- **Gating / honesty (load-bearing):**
  - compute only when **both** tiers have tagged finds (Nlp tags only after
    `nlp-enrich` runs) AND `m > 0`; otherwise report "insufficient overlap —
    estimate not computed" (never a fabricated number, never "0 misses");
  - report `estimated_remaining_misses` as a **lower bound** (capture–recapture
    under-estimates content; the Pattern and Nlp tiers share the same prose input
    so are partially correlated → optimistic), with the assumptions printed
    alongside the number.

## Non-Goals

- NOT Chao Mh / ≥3 heterogeneous extractors yet (only Pattern + Nlp produce
  signal-constraint facts today; the VLM produces different fact kinds). Lincoln–
  Petersen with explicit caveats is the honest first realization.
- NOT extending to other fact kinds yet (SignalConstraint first; relations/rules
  follow once tagged).
- NOT changing extraction — a pure read over `fact_provenance` + a `validate`
  surface.

## Acceptance Criteria

- A pure `signal_constraint_recall_estimate(&[FactProvenanceRecord]) ->
  Option<RecallEstimate>` in `ir/completeness.rs` (None when <2 tiers or no
  overlap); `RecallEstimate { pattern, nlp, overlap, distinct, estimated_total,
  estimated_remaining_misses, estimated_recall_pct }`.
- Surfaced in `validate` (a `Recall Estimate` line — value or "insufficient" —
  + metrics), with the lower-bound + assumptions framing.
- Unit tests (overlap → estimate matches Lincoln–Petersen; no overlap / single
  tier → None). Extraction-neutral; full CI green; book note.

## Task Tree

- ID: `COMPLETENESS-RECALL-GAUGE`
  Status: `done`
  Goal: Lincoln–Petersen recall estimate over the fact-provenance index + validate surface
  Children: `.1`, `.2`

- ID: `COMPLETENESS-RECALL-GAUGE.1`
  Status: `done`
  Goal: own + design (this file); register. Docs-only.
  Verification: >
    passed (`2026-06-01`) — owned; designed the 2-extractor Lincoln–Petersen
    estimate over `fact_provenance` (Pattern vs Nlp signal-constraint keys) with
    explicit gating (≥2 tiers + overlap>0) and honest lower-bound/assumptions
    framing, grounded in the `.5` design. Registered.
  Commit: `see Commit Log`

- ID: `COMPLETENESS-RECALL-GAUGE.2`
  Status: `done`
  Goal: >
    Implement `signal_constraint_recall_estimate` + `RecallEstimate` in
    `ir/completeness.rs`; surface in `validate_evidence_ir` (line + metrics, with
    lower-bound/assumptions framing + the "insufficient" path); unit tests; book
    note; full CI; close.
  Acceptance: estimator + tests + validate surface (gated + honest); CI green; book; tree CLOSED.
  Verification: >
    passed (`2026-06-01`) — `signal_constraint_recall_estimate(&[FactProvenanceRecord])
    -> Option<RecallEstimate>` in `ir/completeness.rs`: builds the Pattern/Nlp
    canonical-key sets, returns `None` when <2 tiers tagged or overlap=0 (no
    fabricated estimate), else Lincoln–Petersen `N̂ = |a|·|b|/m` →
    `estimated_remaining_misses = N̂ − distinct` (lower bound) + recall%. Surfaced
    in `validate` as a `Recall Estimate` section (value or "insufficient — run
    nlp-enrich") + `recall_estimate_remaining_misses`/`recall_estimate_pct`
    metrics, with the lower-bound + correlated-extractors assumptions printed.
    Pure read over `fact_provenance` (extraction-neutral). 3 unit tests
    (Lincoln–Petersen from overlap [4·4/2=8, misses=2, recall 75%]; None without
    2 tiers; None without overlap). fmt/clippy clean; full `scripts/run_ci.sh`
    GREEN; book note in `pipeline/evidenceir.md`. **The capture–recapture recall
    gauge is live** — the research-to-impl recall-estimation arc is complete.
  Commit: `see Commit Log`

## Current Frontier

**Tree CLOSED `2026-06-01`** — the capture–recapture recall gauge (2-extractor
Lincoln–Petersen over the fact-provenance index) is live in `validate`, gated +
honest (lower bound, assumptions printed, "insufficient" when no overlap). The
research→impl recall-estimation arc is complete. A future 3rd independent
extractor upgrades it to Chao Mh.

| Order | Leaf | Status | Why |
| --- | --- | --- | --- |
| 1 | `COMPLETENESS-RECALL-GAUGE.1` | `done` | owned + design |
| 2 | `COMPLETENESS-RECALL-GAUGE.2` | `done` | estimator + validate surface + 3 tests + book; CI green |

## Decisions

- `2026-06-01`: Lincoln–Petersen (2-extractor) with printed assumptions, not a
  fabricated single number — honest given only Pattern + Nlp produce
  signal-constraint facts and they share prose input (partially correlated →
  report misses as a lower bound). Chao Mh / ≥3 extractors is a future upgrade.

## Open Questions

- None for the first slice. Future: add a 3rd independent extractor (e.g.
  two-source RTL fusion) to move from Lincoln–Petersen to Chao Mh and tighten the
  estimate (research `.5` §1, `.6` AssertionForge).

## Blockers

- None (precondition met by `PER-EXTRACTOR-FACT-TAGGING`).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | owned + designed the gated 2-extractor Lincoln–Petersen estimate over `fact_provenance`; registered; docs-only | `passed` |
| `2026-06-01` | `.2` | `signal_constraint_recall_estimate` + `RecallEstimate` (Lincoln–Petersen, gated None on <2 tiers / no overlap); `validate` Recall Estimate section + 2 metrics + lower-bound/assumptions framing; 3 unit tests; extraction-neutral; fmt/clippy clean; full CI green; book note | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `COMPLETENESS-RECALL-GAUGE.1` | `COMPLETENESS-RECALL-GAUGE.1 — own + design the capture–recapture recall estimate` | docs-only |
| `COMPLETENESS-RECALL-GAUGE.2` | `COMPLETENESS-RECALL-GAUGE.2 — Lincoln–Petersen recall estimate + validate surface; close` | code + book; recall-estimation arc complete |

## Changelog

- `2026-06-01`: `.2` — implemented `signal_constraint_recall_estimate` (Lincoln–
  Petersen over `fact_provenance`) + `validate` Recall Estimate surface (gated +
  honest lower bound) + 3 tests + book note. **Tree CLOSED** — capture–recapture
  recall gauge live; the research→impl recall-estimation arc is complete.
- `2026-06-01`: Created — own the capture–recapture recall gauge (2-extractor
  Lincoln–Petersen over the fact-provenance index, honest lower bound). Frontier
  → `.2` (implement).
