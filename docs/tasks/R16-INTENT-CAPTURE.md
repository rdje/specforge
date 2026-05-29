# R16-INTENT-CAPTURE: SOTA design-intent capture (program umbrella)

## Metadata

- Tree ID: `R16-INTENT-CAPTURE`
- Status: `done`
- Roadmap lane: `R16` (new major workstream — SOTA temporal-intent capture)
- Created: `2026-05-19`
- Last updated: `2026-05-29`
- Owner: repo-local workflow

## Thesis (load-bearing — recorded by explicit user direction 2026-05-19)

A digital-design PDF (protocol / component) encodes design intent as the
**temporal behavior of actors observed at their boundary (pins/ports)**.
The single hardest, highest-value problem is **accurate and reliable
extraction of temporal behavior from prose AND from timing diagrams** into
a **typed knowledge graph**. Once that typed KG holds accurate temporal
behavior for every involved signal/port, the remainder
(KG → IntentIR → `.isf` → FSMGen) is **almost mechanical**.

Therefore the program is sequenced so that:

1. the typed target the extraction must land in exists and is shaped like
   a timed contract over actor boundaries (mechanical to lower from), and
2. capture fidelity is **objectively measurable early** (the spec's own
   figures/waveforms are near-ground-truth conformance vectors), so that
3. the bulk of effort concentrates on **prose + timing-diagram → typed KG
   extraction fidelity**, measured against that objective metric.

This thesis is preserved as memory `feedback-temporal-capture-thesis` and
in ROADMAP `R16` and the mdBook direction chapter. It must not be eroded
by treating extraction gaps as mere lowering problems, nor by fabricating
temporal intent the source does not license (residual-honesty doctrine).

## Goal

Steer the IR pipeline to SOTA design-intent capture via six owned
sub-trees (the "6 points"), executed in the recorded order with the
recorded dependency DAG. This umbrella owns the program scaffolding,
ordering, and cross-tree invariants; each sub-tree owns its
implementation.

## Non-Goals

- This umbrella performs **no production code change** — sub-trees do,
  each under its own ownership.
- Do not promote a sub-tree to `active` out of dependency order without
  recording the rationale here.
- Do not weaken the residual-honesty doctrine: unverifiable temporal
  intent is preserved as an explicit residual, never fabricated.

## The six sub-trees (points) and their order

| Order | Sub-tree | Point | Role | Depends on |
| --- | --- | --- | --- | --- |
| 1 | `R16-CONTRACT-IR` | #1 | Typed timed-contract IR layer (assume/guarantee per actor; sequence/stability/bounded-liveness/until operators). The mechanical-to-lower target shape. | — |
| 2 | `R16-KG-PROTOCOL-ONTOLOGY` | #2 | First-class `Channel` / `Transaction` / `Phase` / `HandshakePair` KG nodes+edges so IntentIR is a systematic projection of protocol structure. | `R16-CONTRACT-IR` |
| 3 | `R16-CAPTURE-FIDELITY-GATES` | #5 | Realizability/consistency check + replay of the spec's own figures/waveforms as conformance vectors → the **objective capture-fidelity metric** + residual/repair driver. Pulled early: it is the objective function for the hard problem. | `R16-CONTRACT-IR` |
| 4 | `R16-MULTIMODAL-CONTRACT-FUSION` | #3 | Cluster cross-modal evidence (prose + table + figure + state diagram) keyed by (actor, channel/group, phase) into one contract object with provenance + typed merge + explicit disagreement surface. | 1, 2, 3 |
| 5 | `R16-WAVEFORM-CONTRACT-MINING` | #4 | Timing diagram → structured partial trace (lanes/edges/value-spans/relative-delay annotations/causal arrows) → generalized contract; cross-check vs prose-derived contract. **The crux extraction thrust.** | 1, 3 (feeds 4) |
| 6 | `R16-CONSTRAINED-VERIFIED-EXTRACTION` | #6 | Schema-constrained LLM/VLM extraction emitting directly into ContractIR + entailment verifier (source span must license the contract) + protocol-pattern template library seeded into prior memory + uncertainty-driven converge. **The crux, continuous.** | 1, 3 |

Dependency DAG: `1 → 2`; `1 → 3`; `{1,2,3} → 4`; `{1,3} → 5`; `{1,3} → 6`.
`5` and `6` produce/clean contract candidates that `4` fuses; `3` measures
all of `4/5/6`. `1` is the spine; `3` is the objective function.

## Acceptance Criteria (program-level)

- All six sub-trees exist as registered `proposed` task files with
  precise goals, non-goals, acceptance, dependencies, and the recorded
  order; the dependency DAG is consistent.
- ROADMAP lane `R16` and an mdBook chapter capture the program, the six
  points, the order, and the thesis.
- `docs/TASK_TREE.md` indexes all seven trees.
- Promotion of each sub-tree to `active` happens in DAG order (or with a
  recorded exception); every sub-tree's code lands under its own
  `COMMIT.md`-tracked leaves; `scripts/run_ci.sh` green per code leaf.

## Task Tree

- ID: `R16-INTENT-CAPTURE`
  Status: `done`
  Goal: own the program scaffolding, ordering, cross-tree invariants
  Children: `R16-INTENT-CAPTURE.1`, `.2`

- ID: `R16-INTENT-CAPTURE.1`
  Status: `done`
  Goal: >
    Scaffold the program: create the 6 `proposed` sub-tree files with
    precise goals/non-goals/acceptance/deps/order; add ROADMAP `R16`;
    add the mdBook direction chapter; register all 7 trees in
    `docs/TASK_TREE.md`; record the thesis (memory + docs). Docs only.
  Acceptance: `7 trees registered + ROADMAP R16 + mdBook chapter + thesis memory; mdBook builds; no code change.`
  Verification: `passed` — 6 sub-tree files + this umbrella created
    (`docs/tasks/R16-*.md`), all 7 registered in `docs/TASK_TREE.md`
    with status/lane/dep order; ROADMAP lane `R16` added (thesis, the
    six points in order, DAG, completion criteria) + listed #1 in
    "Recommended implementation order" + "Immediate next milestone";
    mdBook chapter `direction/temporal-intent-capture.md` + `SUMMARY.md`
    entry added; thesis recorded as memory `project-r16-intent-capture`.
    No production code change. Full `scripts/run_ci.sh` green (mdBook
    builds with the new chapter).
  Commit: `see Commit Log`

- ID: `R16-INTENT-CAPTURE.2`
  Status: `done`
  Goal: >
    Drive the program: promote sub-trees to `active` in DAG order,
    keep the order/DAG/thesis consistent as sub-trees split, and close
    when all six are `done`. (Long-running sequencing/governance leaf —
    stays `active` until all six sub-trees are `done`.)
  Acceptance: `Sub-trees executed in DAG order; program closed when all six done; ordering integrity maintained.`
  Verification: >
    passed (`2026-05-29`) — all six sub-trees were promoted in DAG
    order and closed at their honest scope boundaries on `2026-05-20`
    (#1 `R16-CONTRACT-IR`, #2 `R16-KG-PROTOCOL-ONTOLOGY`,
    #5/order-3 `R16-CAPTURE-FIDELITY-GATES`,
    #3 `R16-MULTIMODAL-CONTRACT-FUSION`,
    #4 `R16-WAVEFORM-CONTRACT-MINING`,
    #6 `R16-CONSTRAINED-VERIFIED-EXTRACTION`); ordering integrity and
    the dependency DAG held throughout (every promotion waited on its
    predecessors). The acceptance condition "program closed when all
    six done" is met, so this governance leaf closes the umbrella. The
    two remaining crux items — waveform raster/vector figure extraction
    and CVE prose-extractor producer wiring — are blocked on upstream
    capability that does not yet exist and are recorded as
    honestly-deferred FUTURE trees, not re-opened leaves of any closed
    R16 tree (residual-honesty doctrine). Docs-only; lib `1128/0`; full
    `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

## Current Frontier

Empty — **tree closed `2026-05-29`**. Both leaves are `done` and all six
sub-trees are `done`; no eligible leaf remains. The forward path is two
honestly-deferred FUTURE trees (see Decisions `2026-05-29`), each blocked
on upstream capability that does not yet exist:

- waveform raster/vector figure extraction — feeds
  `R16-WAVEFORM-CONTRACT-MINING`'s `figure_region_to_partial_trace`;
- prose LLM/VLM producer wiring for `parse_constrained_contract` — feeds
  `R16-CONSTRAINED-VERIFIED-EXTRACTION`.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-INTENT-CAPTURE.1` | `done` | Program scaffolding |
| 2 | `R16-INTENT-CAPTURE.2` | `done` | Governance close — all six sub-trees `done`; acceptance met |

## Decisions

- `2026-05-29` (`.2` closed → umbrella closed): all six sub-trees are
  `done` (closed `2026-05-20` at their honest scope boundaries), so this
  governance leaf's acceptance ("program closed when all six done") is
  met and the `R16-INTENT-CAPTURE` umbrella closes. The program
  delivered the *mechanical-to-lower typed target* (#1 ContractIR / #2
  protocol ontology), the *objective capture-fidelity metric* (#5
  fidelity gates), and *structural honesty enforcement* (#3 fusion / #4
  waveform / #6 constrained-verified — four doctrines: fidelity, fusion,
  waveform-verifier, and entailment Fail→Residual). The thesis crux —
  accurate prose+timing-diagram extraction *into* that target — is
  deliberately not wired to a live extractor: two pieces stay deferred
  to FUTURE trees because each is blocked on upstream capability that
  does not exist yet — (1) a PDF → `FigureRegion` raster/vector figure
  extractor, and (2) a prose LLM/VLM provider feeding
  `parse_constrained_contract`. Recording them as deferred future trees
  rather than re-opened leaves is the residual-honesty doctrine applied
  to the program itself: do not pretend the crux is solved before the
  evidence to solve it exists. Closing the umbrella exhausts the active
  task-tree frontier; remaining roadmap crux work is upstream-blocked.

- `2026-05-19` (`.2` opened): the systematic+reliable extraction
  methodology for (1) actor names, (2) actor-relative boundary
  pins/ports, (3) prose+timing-diagram temporal behavior was presented
  to and accepted by the user. Core method: tiered strongest-source-wins
  extraction, multi-source consensus, **cross-modal corroboration**
  (prose ⇄ table ⇄ waveform), the spec's own signal-tables/figures as
  near-ground-truth cross-checks, realizability gate, and
  residual-not-fabricate enforced by an entailment verifier. The plan is
  the R16 program (traceability table in the mdBook chapter / this
  tree). Authorized to begin → `R16-CONTRACT-IR` promoted to `active`.
- `2026-05-19`: Six sub-trees, one umbrella, lane `R16`. Order pulls
  point #5 (`R16-CAPTURE-FIDELITY-GATES`) to 3rd: capture fidelity is the
  program's objective function, and the spec's own figures/waveforms are
  near-ground-truth — you must measure the hard problem before/while
  attacking it. #1/#2 are the foundational typed target ("mechanical to
  lower from", per the thesis); #3/#4/#6 are the extraction-fidelity
  thrust (the crux: prose + timing-diagram → typed KG).
- `2026-05-19`: Sub-trees created `proposed`, not `active` — captured,
  owned, ordered; execution begins on promotion (`.2`), DAG-ordered.
  This matches the doctrine "no code change without task-tree ownership"
  while not pre-committing implementation detail before promotion.

## Open Questions

- ContractIR placement: a new IR stage vs. a typed layer extending
  `SemanticIR`/`IntentIR`. Resolved inside `R16-CONTRACT-IR.1` (design
  leaf), not here.

## Blockers

- None. Sub-trees are `proposed`; promotion is gated on authorization +
  DAG order, not on an external blocker.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-19` | `R16-INTENT-CAPTURE.1` | 7 trees + ROADMAP R16 + mdBook chapter + index + thesis memory; full `scripts/run_ci.sh` (mdBook) | `passed` |
| `2026-05-29` | `R16-INTENT-CAPTURE.2` | all six sub-trees `done` (DAG order, honest scope); tree/index/ROADMAP/live-docs/book reconciled; lib `1128/0`; full `scripts/run_ci.sh` (mdBook) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-INTENT-CAPTURE.1` | `R16-INTENT-CAPTURE — scaffold SOTA intent-capture program (6 ordered trees + ROADMAP + mdBook)` | docs-only; sub-trees `proposed` |
| `R16-INTENT-CAPTURE.2` | `R16-INTENT-CAPTURE.2 — close umbrella governance leaf (all 6 sub-trees done); reconcile tree/ROADMAP/live-docs/book` | docs-only; umbrella + R16 program CLOSED; 2 deferred future trees recorded |

## Changelog

- `2026-05-29`: `.2` closed → **`R16-INTENT-CAPTURE` umbrella CLOSED /
  R16 program complete**. All six sub-trees `done` at their honest scope
  boundaries (`2026-05-20`); acceptance "program closed when all six
  done" met. Reconciled the stale frontier table (was still showing
  `R16-CONTRACT-IR.1` as the real frontier), the umbrella + `.2`
  statuses, the `docs/TASK_TREE.md` index, ROADMAP R16 / Recommended
  implementation order / Immediate next milestone,
  `LIVE_ACHIEVEMENT_STATUS.md`, `CHANGES.md`, `DEVELOPMENT_NOTES.md`, and
  the mdBook program chapter (intro program-status banner + governance-
  closed recap). Two crux items deferred to FUTURE trees (upstream-
  blocked): waveform raster/vector extraction + CVE prose-extractor
  producer wiring. Docs-only; full `scripts/run_ci.sh` green.

- `2026-05-19`: Created by explicit user direction — capture the SOTA
  6-point intent-capture program precisely as ordered task-trees +
  ROADMAP + mdBook, with the prose/timing-diagram-extraction-is-the-crux
  thesis recorded. No code change (scaffolding only).
