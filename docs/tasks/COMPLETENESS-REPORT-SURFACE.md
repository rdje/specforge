# COMPLETENESS-REPORT-SURFACE: one honest completeness headline over the detectors

## Metadata

- Tree ID: `COMPLETENESS-REPORT-SURFACE`
- Status: `done`
- Roadmap lane: `R15e` (KG-quality / completeness)
- Created: `2026-06-01`
- Last updated: `2026-06-01`
- Owner: repo-local workflow

## Goal

Realize the unifying **completeness report headline** from the research framework
([`intent-capture-completeness.md`](../research/intent-capture-completeness.md) §10,
`INTENT-COMPLETENESS-RESEARCH.5` design): the completeness program now emits
several separate `validate` signals (convergence, register-tiling, region
accounting, plus the existing prose-residual / structural-KG / visual-enrichment
findings). A user has to add them up by hand. This tree adds **one aggregated
Completeness Summary** to `validate` — a single honest headline ("N completeness
checks run; K candidate misses, by kind") and a `completeness_candidate_misses`
metric — so "how completely did we capture this document?" has one answer.

## Non-Goals

- NOT a new IR type / persisted report yet (the typed `CompletenessReport` +
  capture–recapture recall estimate are later slices / research `.5`); this is a
  `validate`-side aggregation of already-computed signals.
- NOT new detection logic — purely sums what the detectors already produce, so it
  cannot introduce a false positive the detectors don't already have.
- NOT claiming completeness — it reports *candidate* misses + convergence status,
  honestly (residual-honesty: a count of surfaced gaps, never "0 = perfect").

## Acceptance Criteria

- `validate_evidence_ir` prints a `Completeness Summary` section aggregating the
  located-miss signals — register tiling (overlaps + interior gaps), region
  accounting (unexplained tables), prose residuals (partially-structured
  normative statements) — plus convergence status, into a single
  `completeness_candidate_misses` total + an Info headline finding.
- A `completeness_candidate_misses` metric (== the emitted total) and a
  `completeness_convergence_converged` metric.
- Behavior-neutral; unit/wiring test; full `scripts/run_ci.sh` green; book note.

## Task Tree

- ID: `COMPLETENESS-REPORT-SURFACE`
  Status: `done`
  Goal: aggregated completeness summary + headline in validate
  Children: `.1`, `.2`

- ID: `COMPLETENESS-REPORT-SURFACE.1`
  Status: `done`
  Goal: own + design (this file); register. Docs-only.
  Verification: >
    passed (`2026-06-01`) — owned the unifying completeness-summary surface
    (framework §10 headline); scoped as a validate-side aggregation of the
    existing detector signals (no new detection, no new IR type). Registered.
  Commit: `see Commit Log`

- ID: `COMPLETENESS-REPORT-SURFACE.2`
  Status: `done`
  Goal: >
    Implement the `Completeness Summary` section + `completeness_candidate_misses`
    / `completeness_convergence_converged` metrics + an Info headline finding in
    `validate_evidence_ir`; wiring test asserting the aggregate; book note; full
    CI; close.
  Acceptance: summary + metrics + headline finding; aggregate == sum of components; CI green; book updated; tree CLOSED.
  Verification: >
    passed (`2026-06-01`) — `validate_evidence_ir` now prints a `Completeness
    Summary`: `candidate_misses` = register overlaps + interior gaps + unexplained
    intent-bearing tables + prose residuals (partial normative statements), with
    the per-component breakdown + anchored-rescan convergence label
    (converged / cap-limited / unrecorded). Emits an Info
    `evidence_completeness_summary` headline finding + `completeness_candidate_misses`
    and `completeness_convergence` metrics. Pure aggregation of already-computed
    detector outputs — behavior-neutral, never a new false positive, never a
    perfection claim. Wiring test asserts the aggregate metric equals the sum of
    its component metrics ("metric == emitted content"). fmt/clippy clean; full
    `scripts/run_ci.sh` green; book note in `pipeline/evidenceir.md`.
  Commit: `see Commit Log`

## Current Frontier

**Tree CLOSED `2026-06-01`** — `validate` now emits one `Completeness Summary`
headline (`candidate_misses` total + breakdown + convergence) over the
completeness detectors. The typed `CompletenessReport` + capture–recapture recall
estimate remain future slices (research `.5`).

| Order | Leaf | Status | Why |
| --- | --- | --- | --- |
| 1 | `COMPLETENESS-REPORT-SURFACE.1` | `done` | owned + design |
| 2 | `COMPLETENESS-REPORT-SURFACE.2` | `done` | summary + metrics + headline finding + test + book; CI green |

## Decisions

- `2026-06-01`: aggregation-only (sum existing detector outputs), not a new typed
  report — keeps it behavior-neutral + signoff-trivial while delivering the
  framework's single-headline value; the typed `CompletenessReport` + recall
  estimate are deferred to research `.5` / a later tree.

## Open Questions

- Whether to fold the structural-KG-missing + visual-enrichment warnings into the
  candidate-miss total or keep them as separate flags (they are coarser signals);
  start with the located-miss counts (tiling + unexplained tables + prose
  residuals) and convergence, list the others as context.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | owned + scoped (validate-side aggregation of existing detector signals; no new detection/IR); registered; docs-only | `passed` |
| `2026-06-01` | `.2` | `Completeness Summary` section + Info `evidence_completeness_summary` headline + `completeness_candidate_misses`/`completeness_convergence` metrics; aggregate==sum-of-components wiring test; behavior-neutral; fmt/clippy clean; full CI green; book note | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `COMPLETENESS-REPORT-SURFACE.1` | `COMPLETENESS-REPORT-SURFACE.1 — own the unifying completeness-summary surface` | docs-only |
| `COMPLETENESS-REPORT-SURFACE.2` | `COMPLETENESS-REPORT-SURFACE.2 — Completeness Summary headline in validate; close` | code + book; aggregation-only |

## Changelog

- `2026-06-01`: `.2` — implemented the `Completeness Summary` (candidate-misses
  total + breakdown + convergence) + headline finding + metrics in `validate`;
  aggregate==sum wiring test; CI green; book note. **Tree CLOSED.**
- `2026-06-01`: Created — own the unifying completeness-summary headline
  (framework §10), a validate-side aggregation of the existing detector signals.
  Frontier → `.2` (implement).
