# PRODUCTION-GRAPH-CENSUS-PIN: four repository-wide census numbers drifted under a fully green gate

## Metadata

- Tree ID: `PRODUCTION-GRAPH-CENSUS-PIN`
- Status: `active` (`2026-09-12`; `.0` done, `.1`-`.2` open)
- Roadmap lane: `process / doctrine enforcement`
- Created: `2026-09-12`
- Last updated: `2026-09-12`
- Owner: repo-local workflow

## Goal

`tools/production-genericity-graph` derives a repository-wide census — analyzed functions, helper
edges, decision sites, semantic macros — and `current_repository_flow_is_complete_local_and_deterministic`
pins all four exactly. When `PROSE-NAME-CELL-DECLARATION.2` ran `cargo test` it found that test red,
and red **before** its own change: the pins had drifted by **+18 functions, +179 helper edges, +226
decision sites, +3 semantic macros** while every commit reported a fully green doctrine gate.

Restore the pins to measured truth, and close the gap that let them drift silently.

## How it was found

`PROSE-NAME-CELL-DECLARATION.2` added one production function and three tests, ran `cargo test`, and
saw `left: 2399, right: 2380`. Stashing the change and re-running gave `left: 2398` — so 18 of the 19
predated the slice entirely. The same measurement across all four pins:

| pin | pinned in the test | at `4bb6c1c8` (pre-existing drift) | with `.2`'s change | `.2`'s own contribution |
| --- | ---: | ---: | ---: | ---: |
| `analyzed_functions` | 2,380 | 2,398 (+18) | 2,399 | **+1** |
| `helper_edges` | 14,763 | 14,942 (+179) | 14,946 | **+4** |
| `decision_sites` | 12,705 | 12,931 (+226) | 12,943 | **+12** |
| `semantic_macros` | 1,466 | 1,469 (+3) | 1,469 | **0** |

The drift is not the interesting part. **Two mechanisms had to both be true for it to stay invisible**,
and they are a textbook instance of `CLAIM_VERIFICATION.md` §2 — *what class of defect does this check
still permit?*

1. `scripts/check_production_genericity_flow.sh` is a **gate-tier doctrine that runs on every commit**,
   and it *prints* all four numbers:
   `production-genericity-flow: … 2399 functions; 14946 helper edges; 12943 decision sites; … 1469 semantic macros`.
   It compares none of them. A census that reports is not a check: it permits every drift, and it
   reported the true value while the pin beside it said something else.
2. The only comparison lives in a `cargo test`, which the repository's CI policy runs **before a push,
   not per commit**. That policy is deliberate and correct for throughput — but it means the one
   comparing oracle for this census fires at push cadence while the drift accrues at commit cadence.

The last time the pins were set is `0d4860ba` (`WIRE-BASED-100.8b`). The **29 commits since** have
been able to move them without any local signal.

## Non-Goals

- Do not weaken or re-scope the CI policy. Running the full suite per commit is the throughput cost
  that policy exists to avoid, and a 90-second graph analysis is precisely the kind of check it was
  written to keep out of the per-commit path.
- Do not delete the pins. A drifting census is not an argument for having no census; the numbers are
  the genericity boundary's own denominator and a silent change in them is exactly what
  `PRODUCTION-GENERICITY` exists to catch.
- Do not audit which historical commit contributed which unit of the drift. The pins are a whole-tree
  derivation, the commits that moved them are ordinary feature work, and attributing 226 decision
  sites across 29 commits buys nothing a re-pin does not.

## Acceptance Criteria

- The four pins equal the measured current derivation, and `cargo test` is green.
- A change to any of the four is detectable **at commit cadence**, not only before a push — or the
  decision not to make it detectable is recorded with the cost that was weighed.
- The distinction between a census that prints and a check that compares is stated where the next
  author will meet it, not only here.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `PRODUCTION-GRAPH-CENSUS-PIN` · Status: `active` (`2026-09-12`) · Children: `.0`-`.2`

- ID: `PRODUCTION-GRAPH-CENSUS-PIN.0` · Status: `done` (`2026-09-12`) · Goal: **re-pin at measured
  truth, and say what the drift was.** The four pins are set to the values the current tree derives,
  with the pre-existing / this-slice split stated above so no earlier drift is absorbed silently. The
  test's doc comment now carries why the pins could move at all.
  Verification: `cargo test -p specforge-production-graph --lib` → `4 passed; 0 failed` (89.03s);
  the derivation is compared against `scripts/check_production_genericity_flow.sh`'s own printed line,
  which is an independent execution path to the same analyzer.
  Commit: `PROSE-NAME-CELL-DECLARATION.2 / PRODUCTION-GRAPH-CENSUS-PIN.0`

- ID: `PRODUCTION-GRAPH-CENSUS-PIN.1` · Status: `pending` · Goal: **make the printing gate compare.**
  `check_production_genericity_flow.sh` already derives all four numbers on every commit; the missing
  half is a declaration to compare them against. Put the expected census in a tracked contract under
  `doctrine/` — where every other doctrine's expectations already live — and have the flow check fail
  closed on a mismatch, naming the field and both values. The analysis is already being paid for, so
  this adds a comparison, not a run.
  **The open question is answered and the design is chosen** (`2026-09-12`, four-for-four measurement
  above): neither an exact pin nor a band, but the repository's own `aggregate_change` idiom — the same
  one `doctrine/live_document_size/surfaces.jsonl` already uses for the book's byte total, another
  derived number that legitimately moves on ordinary work. A contract under `doctrine/` carries
  `baseline` + `delta` + `owner` + `rationale`; the flow check compares `baseline + delta` against what
  it already derives and fails closed naming the field and both values. The per-commit edit is not
  removed — it is converted from *re-pinning a literal in a test that cannot see it at commit cadence*
  into *attributing a census change to the leaf that caused it*, which is what the other doctrines
  already require. **`tools/production-genericity-graph`'s test then reads the same contract instead of
  carrying its own four literals**, which is the actual root cause: the pin and the deriver were two
  places that could disagree, and they did, for 29 commits.
  Prerequisite: `.0`. Verification: observed RED against a perturbed count in EACH of the four fields,
  and against a contract whose `baseline + delta` is right but whose `owner` is absent; green on the
  true tree; the Rust test carries no census literal afterwards.

- ID: `PRODUCTION-GRAPH-CENSUS-PIN.2` · Status: `pending` · Goal: **find the other censuses that print
  without comparing.** This one was found by accident, by a slice that happened to run `cargo test`.
  The general shape — a doctrine check that derives a number, reports it, and compares nothing —
  is mechanical to search for: every registered check in `scripts/check_doctrines.sh` that emits a
  numeral in its success line, adjudicated by hand against whether anything fails when that numeral
  changes. Report the population; open leaves only for the ones where a silent change would matter.
  Prerequisite: `.1` (its design decides what "comparing" should look like here).
  Verification: the census is the deliverable; a count with no adjudication is not.

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `PRODUCTION-GRAPH-CENSUS-PIN.1` — make the gate that already derives the numbers compare them.
2. `PRODUCTION-GRAPH-CENSUS-PIN.2` — the same shape elsewhere in the registry.

## Decisions

- `2026-09-12` — **re-pinned in the slice that found it, rather than in a separate commit.** The
  finding surfaced with `PROSE-NAME-CELL-DECLARATION.2` already uncommitted in the tree, and this
  repository's pivot rule forbids switching trees across a dirty tree. Landing both leaves in one
  commit with the per-pin attribution stated is the honest version of that constraint; stashing
  half-finished production work to slip a repair underneath it is not.
- `2026-09-12` — **the drift is not audited commit-by-commit.** See Non-Goals. What matters is the
  mechanism, and the mechanism is fully established without knowing which of the 29 commits added
  which decision site.
- `2026-09-12` — **the fix is not "run the full suite per commit".** That would answer this defect by
  reversing a throughput decision the owner made deliberately. The cheaper answer is that the check
  which *already runs and already derives these numbers* should compare them.
- `2026-09-12` — **the design is `aggregate_change`, not a band, and the reason is precedent not
  taste.** A declared band has to answer "how wide", and any answer is arbitrary: the census only ever
  grows, so a band either admits a jump worth noticing or has to be re-based on the same cadence an
  exact pin would. This repository already solved the identical problem for the book's byte total —
  a derived number that moves on ordinary work — with `baseline` + `delta` + `owner` + `rationale`, and
  that idiom makes the per-commit edit informative rather than ceremonial: it records *who* moved the
  census and *why*, which is exactly what was missing across the 29 silent commits.

## Open Questions

- ~~How often do the four numbers legitimately move?~~ **Answered `2026-09-12`: every slice that adds a
  production predicate, four for four.** `ACTOR-NOUN-RELATION-DECLARATION.1`,
  `INVARIANT-SHAPE-ADMISSION.1`, `PROSE-NAME-CELL-DECLARATION.2` and now
  `INVARIANT-SHAPE-ADMISSION.3` (+6 functions / +32 helper edges / +29 decision sites / +2 semantic
  macros) each had to edit the pins to land — and then `INVARIANT-SHAPE-ADMISSION.5` made it **five for
  five**, which is the sharpest case of all: that slice added ONE condition to an existing gate,
  removed two fabricated records and introduced no new behaviour, and still had to edit four literals
  in a test that cannot observe them until push. So an exact pin in a push-cadence test is the worst of
  both: it taxes every ordinary commit AND detects nothing until push. `.1` now has its measurement
  and a design to build; see the decision below.

## Blockers

None.

## Verification Log

- `2026-09-12` — `.0`. Baseline established by measurement, not inference: `git stash push` of the
  slice's only modified source file, then `scripts/check_production_genericity_flow.sh` at
  `4bb6c1c8` → `2398 functions; 14942 helper edges; 12931 decision sites; 1469 semantic macros`;
  `git stash pop`, re-run → `2399 / 14946 / 12943 / 1469`. `cargo test -p specforge-production-graph
  --lib` green at the re-pinned values. Full `cargo test` green (`472` / `168` / `1423` / `4`).

## Commit Log

- `.0` — `PROSE-NAME-CELL-DECLARATION.2 / PRODUCTION-GRAPH-CENSUS-PIN.0`.

## Changelog

- `2026-09-12` — tree created. A `cargo test` pin of four repository-wide census numbers was found
  stale by +18 / +179 / +226 / +3 while the gate-tier doctrine that derives the same numbers on every
  commit printed the true values and compared none of them.
