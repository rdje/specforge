# COMPLETENESS-RECALL-RELATIONS: extend per-extractor tagging + recall gauge to actor-signal relations

## Metadata

- Tree ID: `COMPLETENESS-RECALL-RELATIONS`
- Status: `active`
- Roadmap lane: `R15e` (KG-quality / completeness)
- Created: `2026-06-01`
- Owner: repo-local workflow

## Goal

Extend the per-extractor fact tagging (`PER-EXTRACTOR-FACT-TAGGING`) and the
capture–recapture recall gauge (`COMPLETENESS-RECALL-GAUGE`) from signal
constraints to **actor-signal relations** — the second multi-produced fact type:
the structural **Pattern** tier (`extract_actor_signal_relations` + table
relations at `evidence` build) vs the **LLM** tier (`signal-resolve`). Tagging
both lets the gauge estimate the unseen population of actor→signal edges too.

## Design (mirrors the SignalConstraint slice)

- `FactKind::ActorSignalRelation` (new variant).
- `actor_signal_relation_fact_key(&ActorSignalRelation)` = normalized
  `actor | relation | signal` (so the same edge from Pattern and the LLM matches).
- **Pattern** tagging at `EvidenceIr::build`: every converge-produced
  `actor_signal_relation` → a `(Pattern, ActorSignalRelation, key)` entry.
- **Nlp** tagging in `signal_resolve.rs` at the `RelationOutcome::Accepted` arm
  (line ~246, **pre-dedup** — before the `is_duplicate`/`is_same_edge` check — so
  an LLM edge that duplicates a Pattern edge is recorded as the overlap).
- Generalize the estimator to `recall_estimate(&[FactProvenanceRecord], FactKind)`
  (the Lincoln–Petersen math is fact-kind-agnostic); keep
  `signal_constraint_recall_estimate` as a thin wrapper; `validate` reports the
  estimate for BOTH kinds.

## Non-Goals

- NOT changing extraction / relation outcomes — additive index + a pure read.
- NOT Chao Mh / 3rd extractor (future). Lincoln–Petersen with caveats, as before.

## Acceptance Criteria

- `FactKind::ActorSignalRelation` + key fn; Pattern@build + Nlp@signal-resolve
  tagging (pre-dedup, deduped index); generic `recall_estimate(_, fact_kind)`;
  `validate` shows relation provenance + recall estimate (gated/honest, same as
  constraints). Unit/wiring tests; extraction-neutral; full CI green; book note.

## Task Tree

- ID: `COMPLETENESS-RECALL-RELATIONS`
  Status: `active`
  Goal: extend tagging + recall gauge to actor-signal relations
  Children: `.1`, `.2`

- ID: `COMPLETENESS-RECALL-RELATIONS.1`
  Status: `done`
  Goal: own + design (this file); verify producers + struct; register. Docs-only.
  Verification: >
    passed (`2026-06-01`) — owned; verified `ActorSignalRelation`
    {actor_name, signal_name, relation: RelationKind} is produced by Pattern
    (converge `extract_actor_signal_relations` + table relations at build) and the
    LLM (`signal_resolve.rs:246` `RelationOutcome::Accepted`, pre-dedup), mirroring
    the SignalConstraint Pattern/Nlp pair. Registered.
  Commit: `see Commit Log`

- ID: `COMPLETENESS-RECALL-RELATIONS.2`
  Status: `pending`
  Goal: >
    Implement `FactKind::ActorSignalRelation` + `actor_signal_relation_fact_key`;
    tag Pattern@build + Nlp@signal-resolve (pre-dedup); generalize
    `recall_estimate(_, fact_kind)`; `validate` relation provenance + recall;
    unit/wiring tests; book note; full CI; close.
  Acceptance: relation tagging + generic gauge + validate surface; CI green; book; tree CLOSED.
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `COMPLETENESS-RECALL-RELATIONS.1` | `done` | owned + design + producers verified |
| 2 | `COMPLETENESS-RECALL-RELATIONS.2` | `pending` | implement tagging + generic gauge + validate + tests + book + close — next |

## Decisions

- `2026-06-01`: generalize the estimator (`recall_estimate(_, fact_kind)`) rather
  than duplicate it — the Lincoln–Petersen math is fact-kind-agnostic.
- `2026-06-01`: tag Nlp relations pre-dedup (capture overlap with Pattern), same
  discipline as the SignalConstraint slice.

## Open Questions

- Table-sourced relations are also `Pattern` here (structural, non-LLM) — fine
  for the Pattern-vs-LLM split; a finer Pattern/Table split is not needed yet.

## Blockers

- None (builds on the closed tagging + gauge trees).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | producers verified (Pattern@build, Nlp@signal_resolve:246 pre-dedup); design mirrors SignalConstraint slice; registered; docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `COMPLETENESS-RECALL-RELATIONS.1` | `COMPLETENESS-RECALL-RELATIONS.1 — own + design relation tagging + gauge extension` | docs-only |

## Changelog

- `2026-06-01`: Created — extend per-extractor tagging + recall gauge to
  actor-signal relations (Pattern@build vs LLM@signal-resolve). Frontier → `.2`.
