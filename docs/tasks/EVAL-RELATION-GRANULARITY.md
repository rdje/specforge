# EVAL-RELATION-GRANULARITY: per-relation-kind P/R/F1 + MUC near-miss diagnostic

## Metadata

- Tree ID: `EVAL-RELATION-GRANULARITY`
- Status: `done` (CLOSED `2026-06-05`; `.1`)
- Roadmap lane: `R16`/`R15e` (eval quality)
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: owner directive ("do all 5 bullets") — item 1 of 5, picked **first** as
  measurement infrastructure (sharper relation metrics *before* changing extraction).

## What

- `crate::eval::score_relations_by_kind(items, predicted) -> BTreeMap<String, Scorecard>` — splits
  the `ActorSignalRelation` task's P/R/F1 by **relation kind** (Drives vs Reads), keying on the
  middle field of the `ACTOR|kind|SIGNAL` fact key. Newly relevant after VERB-COVERAGE-CORPUS added
  many Drives/Reads edge verbs — lets a regression in one direction show without being masked by the
  other.
- `relation_near_misses(items, predicted) -> RelationNearMiss { wrong_direction, wrong_actor }` —
  MUC-style partial-match diagnostic: a missed gold relation a prediction *almost* matched (same
  actor+signal, flipped direction; or same direction+signal, different actor) — more informative
  than plain FP+FN.
- Wired into `eval-extraction`'s report (a "relations by kind" block + a near-miss line).

## Verification

Passed (`2026-06-05`) — +1 unit test (`per_kind_scoring_and_near_misses_split_relations`:
drives tp/fp, reads fn, one flipped-direction near-miss). Full `scripts/run_ci.sh` GREEN
(1259→1260).

## Task Tree

- ID: `EVAL-RELATION-GRANULARITY` · Status: `done` · Children: `.1`
- ID: `EVAL-RELATION-GRANULARITY.1` · Status: `done` · Goal: per-kind P/R/F1 + near-miss + report
  wiring + test. Verification above.

## Changelog

- `2026-06-05`: Created + CLOSED — per-relation-kind P/R/F1 and MUC near-miss diagnostic in
  `crate::eval`, surfaced in `eval-extraction`. (Owner "do all 5" — item 1/5.)
