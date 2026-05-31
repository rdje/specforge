# R15C-CONVERGENCE-REPORT: make the convergent extraction loop first-class + inspectable

## Metadata

- Tree ID: `R15C-CONVERGENCE-REPORT`
- Status: `active`
- Roadmap lane: `R15c` (KG-guided multimodal rescans)
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Advance ROADMAP §R15c by satisfying its named-but-unbuilt criterion —
**"convergence reporting counts genuinely new persisted facts instead of
duplicate vector growth"** and **"stop iterating only when backannotated
knowledge stabilizes."** Today `converge_evidence_extractions` already runs a
monotone anchored-rescan loop and breaks on a fixpoint (no new deduplicated
statement texts), but it emits **no convergence report** — a user cannot see how
many passes ran, how many genuinely-new anchored facts each pass recovered, or
whether the loop truly stabilized vs hit its pass cap.

This leaf makes that loop **first-class and inspectable**: emit a typed
`EvidenceConvergenceReport` (passes run, new facts per pass, total, converged
flag), persist it on `EvidenceIR`, and have `specforge validate` surface it —
honestly flagging the cap-limited (non-converged) case rather than implying
silent completion.

Owned under `R15C-R15G-LEARNING-PLANE-BACKFILL.1` (R15c) remaining scope; this
is the first concrete advance of that lane.

## Non-Goals

- NOT changing extraction outcomes — the loop's facts/constraints/relations are
  unchanged; this adds only an accounting/reporting surface (additive).
- NOT changing the convergence STOP condition (still the new-deduped-text
  fixpoint) — only making it observable.
- NOT carrying the report to SemanticIR/IntentIR — it is an EvidenceIR-build
  property; `validate` reads it at the evidence stage.

## Acceptance Criteria

- `converge_evidence_extractions` returns a typed `EvidenceConvergenceReport`
  (`passes_run`, `max_passes`, `new_facts_per_pass`, `total_new_facts`,
  `converged`); per-pass counts are the **deduplicated** new statement count
  (genuinely-new facts, not vector growth).
- `EvidenceIr` persists `convergence_report` (serialized; `serde(default)` for
  back-compat) so standalone `validate <evidence_ir.json>` can read it.
- `validate_evidence_ir` emits a finding: Info when converged (passes + facts);
  Warning when it hit the pass cap still discovering facts (convergence not
  proven). Zero artifact churn for unchanged inputs beyond the new field.
- Unit tests: the report on a converging input (`converged == true`, sane
  counts) + the validate finding. Full `scripts/run_ci.sh` green; book updated.

## Task Tree

- ID: `R15C-CONVERGENCE-REPORT`
  Status: `active`
  Goal: typed convergence report on the EvidenceIR rescan loop + validate surface
  Children: `.1`, `.2`

- ID: `R15C-CONVERGENCE-REPORT.1`
  Status: `done`
  Goal: own + design (this file); register in `docs/TASK_TREE.md`. Docs-only.
  Acceptance: tree created with the typed-report design; registered; committed.
  Verification: pending
  Commit: `see Commit Log`

- ID: `R15C-CONVERGENCE-REPORT.2`
  Status: `pending`
  Goal: >
    Implement: add `EvidenceConvergenceReport`; track per-pass deduped new-fact
    counts + `converged` in `converge_evidence_extractions` (return it); persist
    `EvidenceIr.convergence_report`; emit the `validate_evidence_ir` finding
    (Info converged / Warning cap-limited). Unit tests + book note in
    `pipeline/evidenceir.md`; full CI green; close.
  Acceptance: report emitted + persisted + validated; behavior-neutral on facts; tests pass; CI green; book updated; tree CLOSED.
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R15C-CONVERGENCE-REPORT.1` | `done` | tree + design landed |
| 2 | `R15C-CONVERGENCE-REPORT.2` | `pending` | implement the report + validate finding + tests + book + close — next |

## Decisions

- `2026-05-31`: report is EvidenceIR-stage-only (not carried downstream) — it
  describes the evidence-build loop, and `validate` reads it at that stage; keeps
  the change small and avoids SemanticIR/IntentIR churn.
- `2026-05-31`: per-pass count = the deduplicated new-statement count already
  computed by the loop (`HashSet::insert`), so the metric is "genuinely new
  facts," matching the ROADMAP wording, not raw vector length.
- `2026-05-31`: cap-limited termination (loop runs all `max_passes` still
  finding new facts) is reported as a **Warning** — honest that convergence was
  not proven — rather than silently presented as done.

## Open Questions

- Future R15c leaves (separate trees): use the now-visible per-pass metric to
  drive accuracy work — anchored prose/figure rescans that recover MORE facts,
  measured against this report. Out of scope here.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `.1` | tree created (typed convergence-report design); registered in `docs/TASK_TREE.md`; docs-only (CI invariant) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R15C-CONVERGENCE-REPORT.1` | `R15C-CONVERGENCE-REPORT.1 — own + design the EvidenceIR convergence report` | docs-only |

## Changelog

- `2026-05-31`: Created — own + design a typed convergence report for the
  EvidenceIR anchored-rescan loop (R15c "convergence reporting counts genuinely
  new persisted facts"). Frontier → `.2` (implement).
