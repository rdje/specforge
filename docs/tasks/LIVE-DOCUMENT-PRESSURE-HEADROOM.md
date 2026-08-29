# LIVE-DOCUMENT-PRESSURE-HEADROOM: keep current-facing canonical surfaces writable

## Metadata

- Tree ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM`
- Status: `active` (`.0`/`.5` done; `.1`–`.4` pending — `.4` re-ranked `2026-08-28` to a reachable stop)
- Roadmap lane: repository durability and portability
- Created: `2026-08-14`
- Last updated: `2026-08-28`
- Owner: repo-local workflow

## Goal

Restore actionable headroom in the non-rolling current-facing surfaces whose next ordinary update is near a
hard refusal, while distinguishing writable authorities from large immutable evidence and preserving every
canonical byte and route.

## Non-Goals

- Do not raise a bound, shrink evidence, or silence a warning merely to make the pressure report quiet.
- Do not rewrite accepted decisions, completed research, or historical task evidence in place.
- Do not combine independent lifecycle remedies into one migration transaction.
- Do not interrupt the active alignment task-evidence root-last transaction.

## Opening Pressure Boundary (`92e59c97`)

The composed live-size gate passes, but these non-rolling axes have little local room:

| Surface | Exact largest/current state | Registered target/ceiling | Remaining |
| --- | ---: | ---: | ---: |
| `knowledge_cards.lines_each` | `production-genericity-boundary.md` 299 | 300 | **1 line** |
| `task_evidence.files` | 144 root task Markdown files | 160 | **16 files** |
| `task_tree_index.lines_each` | `docs/TASK_TREE.md` 397 | 480 health / 512 ceiling | 83 / 115 lines |
| `shipped_behavior.bytes_each` | `pipeline/evidenceir.md` 118,004 | 131,072 | 13,068 bytes |
| `research_records.lines_each` | `production-genericity-pipeline-audit.md` 604 | 640 | 36 lines |
| `validation_snapshot.lines_each` | `VALIDATION_SNAPSHOT.md` 544 | 640 | 96 lines |
| `readme_entrypoint.line_bytes_each` | `README.md` 108 | 120 | 12 bytes |

The new ownership file deliberately consumes one task slot; after catalog regeneration the resulting task plane
is 145/160 files and the derived task index is 398 lines. That cost is explicit and buys one route for the
remedies instead of scattering unowned warnings across future product commits.

`decision_records` pressure is excluded because `DECISION-RECORD-CAPACITY-HEADROOM` already owns its independent
41/44 collection count and ADR 0038 member-shape axes. Rolling ledgers and the roadmap root have declared
repeatable rollover/remedy paths and remain under their existing owners.

## Acceptance Criteria

- Each pressure axis is classified by lifecycle, writer, expected growth, hard stop, and a remedy that is legal
  for that lifecycle before any authority moves.
- The 299-line current knowledge card is losslessly split or superseded through the fact-card protocol before a
  structural qualification fact needs to update it.
- Task-tree collection and derived-index capacity are redesigned together; adding an ownership task cannot move
  one axis while hiding the other.
- Maintained book content stays user-readable and current; immutable research/snapshot evidence is routed or
  partitioned only through an accepted lossless transaction.
- README policy and every canonical catalog remain exact; focused gates and full CI run in proportion to each
  leaf, and every slice commits through `COMMIT.md`.

## Task Tree

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  Status: `active`
  Goal: keep non-rolling current-facing canonical surfaces writable without losing evidence
  Children: `.0`, `.1`, `.2`, `.3`, `.4`, `.5`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.0`
  Status: `done`
  Goal: own and pin the exact pressure frontier before any surface or bound changes
  Acceptance: the clean boundary, exact axes, existing owner split, and ordered remediation leaves are durable;
  no governed content, bound, product, decision, research record, snapshot, or book page changes
  Verification: `opening 92e59c97: knowledge card 299/300 lines; task plane 144/160 files and index 397/480 health lines; EvidenceIR book page 118,004/131,072 bytes; largest research 604/640 lines; validation snapshot 544/640 lines; README max line 108/120 bytes; ownership adds one task file and one derived catalog row only; resulting 145 files / 398 index lines; catalogs, retrieval, live-size, and doctrines pass`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.0 — own the current live-surface pressure frontier`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`
  Status: `pending`
  Goal: restore writable headroom for the 299-line production-genericity fact card
  Acceptance: current and immutable fact roles are separated losslessly through the existing catalog/map
  lifecycle; every answer route remains exact; no card bound moves; the next ordinary fact update succeeds
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2`
  Status: `pending`
  Goal: re-derive task-tree collection and bounded catalog capacity as one profile
  Acceptance: exact growth, readers/writers, route cardinality, catalog shape, aggregate reachability, capacity,
  and boundary faults are measured and decided before changing either the file or index authority
  **Measured composition (`2026-08-29`, director question: why is there a cap at all, and can the structure
  make it stop mattering).** The pressured axis is `task_evidence.files` — the COUNT of root Markdown files
  matching `docs/tasks/*.md`, top level only; the three part-directories are separate surfaces with their own
  bounds. It is **151 of 160**, and both `health_targets.files` and `enforcement_ceilings.files` are 160, so
  like `.4`'s research axis there is no warning band. The per-file caps on the same surface (3,000 lines,
  278,528 bytes, 6,400 line-bytes) are NOT the pressure point.
  The decisive number is the composition, not the count: of those 151 roots, **120 are `done`, 5 are
  `superseded`, 24 are `active`, 1 is `proposed`**. So **83% of the live collection is finished work**, and the
  bound is currently measuring cumulative project history rather than actionable state. That is why it reads as
  an arbitrary ceiling — it is being spent on trees nothing will ever act on again.
  **Candidate design this leaf should decide against (not yet accepted).** Make the collection's membership
  SEMANTIC instead of cumulative: `docs/tasks/` holds what can still be acted on, and a root whose every leaf
  is `done`/`superseded` with a closed commit log is TERMINAL and retires. Retirement already exists and is
  already proven four times — `archive_terminal` plus `check_task_tree_archive.pl`'s `source_locked` ->
  `migrated` transition seals a byte-exact capsule under `docs/archive/tasks/` reachable from a bounded index —
  but it has only ever been applied to PARTS of one oversized tree, never to a whole finished root. Extending it
  to terminal roots converts the bound from a countdown against project lifetime into a statement about
  concurrent WIP, which is a signal worth having.
  Why NOT simply raise 160: this tree's own Non-Goals forbid raising a bound to quiet a warning, and doubling it
  only moves the countdown. The caps exist because every one of these surfaces is read by a fresh session with a
  bounded context; an unbounded `docs/tasks/` makes the frontier unfindable and pushes `docs/TASK_TREE.md` past
  its own 512-line ceiling. The cap is a proxy for "a fresh session can still find the frontier".
  Risk to design against, stated before building: retirement must not dangle a live pointer. Task trees
  cross-reference each other by leaf id, and claim records carry `retained_evidence` paths into
  `docs/tasks/*.md`. A terminal-root transaction therefore needs a route-rewrite step and a gate proving no live
  reference resolves into the archive by accident — the same obligation the parts migration already discharges,
  widened from parts to roots
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.3`
  Status: `pending`
  Goal: keep the maintained EvidenceIR book chapter current below actionable pressure
  Acceptance: content is reorganized by reader concern without losing examples, links, or public behavior; the
  book aggregate and current-truth authorities remain exact and no size bound moves
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`
  Status: `pending`
  Goal: classify and remedy the research, validation-snapshot, and README member warnings
  Acceptance: each axis has a lifecycle-correct local remedy or a measured reason it is immutable/healthy;
  accepted transactions preserve exact evidence and no generic warning is merely suppressed
  Re-measured (`2026-08-28`, found while running `SOURCE-IR-REPRODUCIBILITY.9`): the research axis moved
  from a line warning to a **membership stop**, which the `2026-08-14` boundary above did not record
  because it did not exist then. `docs/research/*.md` is **63 of a 64-file ceiling**, and for this surface
  `health_targets.files` and `enforcement_ceilings.files` are both 64 — so there is no warning band left
  and the *next* research record is the last one this collection can accept. The line axis is equally
  tight at 639 of 640. Two active trees write research records as their normal output
  (`SOURCE-IR-REPRODUCIBILITY` published three in two days), so this is reachable within a slice or two,
  and unlike a rolling ledger this surface has **no declared rollover transaction** to release it. That is
  the condition `LIVE-DOC-STOP-RISK` exists to prevent: a bound a surface can reach with no remedy
  compliant work can take. Note the `.jsonl` rollover plans under the same directory do **not** count —
  the surface targets `*.md` only.
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.5`
  Status: `done` (`2026-08-28`)
  Goal: give the resume pointer a band it can live in
  Acceptance: `MEMORY.md` is the `active_resume` surface, and its `health_targets` and
  `enforcement_ceilings` are **identical** — 50 lines, 32,768 bytes, 160-byte lines. A surface whose health
  target equals its ceiling has no warning band: it reports "at or above rollover" from 45 lines onward and
  then simply refuses. Measured `2026-08-28` it is 46 of 50 lines (92.0%), and every slice this session had
  to hand-compress the pointer to stay inside it — three times, which is a workflow tax rather than a
  containment control. `MEMORY_ARCHITECTURE.md` requires this file to carry one active unit, current state,
  one next action, in-flight work, and blockers; that is five sections plus a fixed how-to-resume preamble
  of 18 lines, leaving roughly 27 lines for all five. Decide whether the preamble belongs in the bounded
  pointer at all — it is stable prose that never changes and could be routed to `MEMORY_ARCHITECTURE.md`,
  which would give the mutable half of the file twice its current room without moving a bound. Do not raise
  the ceiling to buy space that a routing change already provides
  Prerequisite: none; it blocks nothing today
  Decided and delivered (`2026-08-28`, owner-delegated, "it's your call but it has to be signoff"): the
  history settles it and authoring discipline does not. Across the **last 30 commits that touched
  `MEMORY.md` the preamble is 19 lines in every single one** — a constant, never varying — while the
  mutable block grew 17 -> 28 against the 31 lines that leaves, i.e. **90% of its real budget already
  spent**. The growth is in the half that is supposed to grow, so the remedy is routing, not tighter
  prose. Every route the preamble stated is already reached *before* the pointer is read: a harness reads
  `AGENTS.md` first and `MEMORY.md` is step 4, and all eight of its tokens resolve upstream
  (`git rev-parse HEAD`, the no-shadow rule, `DOCTRINE_ENFORCEMENT.md`, `docs/TASK_TREE.md`, `COMMIT.md`,
  ADR 0003, `check_doctrines.sh`, `KNOWLEDGE_MAP.md`) — verified token by token before deleting a line.
  Nothing unique was moved and nothing was lost; this is deduplication.
  Delivered: fixed region **19 -> 8 lines**, so the mutable budget goes **31 -> 42** (+35%) with no bound
  moved. The four fields `check_memory_architecture.sh` requires are untouched.
  Gated, because an ungoverned split just drifts back: the checker now derives
  `MEMORY_POINTER_LINE_CAP / MEMORY_POINTER_FIXED_SHARE_DIVISOR` = 50/4 = **12 lines** for everything
  above and including the `## Current state` marker, reports the remaining mutable room on every run, and
  names routing as the remedy rather than a bigger cap. The bound is derived from the existing cap, so it
  cannot go stale the way a carried literal does.
  Correction caught by the gate, not by me (`2026-08-28`): the token-presence check above was necessary
  and **not sufficient**. One routed line was also a *registered* anchor — the derived-state contract
  `active_resume_repository_revision` pinned the exact heading `## How to resume (any AI, any harness)`
  as its `field_marker`, and the current-claim census pinned an evidence region on the same line. Route
  resolution says nothing about registry pins, so both broke and `check_doctrines.sh` refused the commit.
  Repaired by repointing both at the surviving declaration rather than restoring a heading to satisfy a
  literal: the contract now anchors on ``on read: revision from `git rev-parse HEAD` `` — the declaration
  itself, which is what the contract exists to pin — and the census evidence moves to that line with its
  identity re-derived. Verified after: derived-state 14 contracts / 47 self-test checks green, census 39
  surfaces / 66 evidence units, zero unresolved. **The lesson is the general one:** before routing a line
  out of a governed surface, check the registries that pin it by exact literal, not only the routes it
  states.
  Acceptance: `the fixed region is measured and capped at a derived share of the pointer cap; the mutable marker is required; the four resume fields still validate; every routed line is proven to resolve upstream; a known-bad pointer is observed RED; every registry pin on a routed line is repointed at surviving content, not restored as a literal`
  Verification: `fixed region 19 -> 8 lines, mutable budget 31 -> 42; four RED/boundary cases observed — the exact pre-change pointer at HEAD fails at 19 > 12 (a control observed failing on real shipped content, not a fixture), a missing '## Current state' marker fails as "no overwritable resume signal", 13 lines fails and exactly 12 passes; check_memory_architecture.sh green after; the eight preamble tokens each verified present in AGENTS.md or MEMORY_ARCHITECTURE.md before removal`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.5 — stop the resume pointer spending its budget on prose that never changes`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.0` | `done` | exact clean pressure and owner boundaries are pinned |
| 2 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.1` | `pending` | one line remains before the next current structural fact is refused |
| 3 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.2` | `pending` | the task plane is already at its 90% file milestone |
| 4 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.4` | `pending` | re-ranked `2026-08-28`: `docs/research/*.md` is 63 of a 64-file ceiling with no warning band and no rollover, and two active trees write research records |
| 5 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.5` | `done` | routed the 19-line constant preamble out; mutable budget 31 -> 42 with no bound moved, and the split is now gated |

## Decisions

- `2026-08-14`: open one pressure-frontier tree rather than one task per warning. The surfaces need distinct
  remediation transactions, but one bounded owner can preserve the exact measured ordering without consuming
  several more task-file slots at the already-triggered collection milestone.
- `2026-08-28`: `.5` closed. The decision was delegated with one condition — signoff quality — so it was
  taken on measurement rather than taste: the preamble is a 19-line constant in all 30 sampled commits and
  the mutable half had spent 90% of what remained, which rules out authoring discipline as the remedy.
  Every routed line was proven to resolve upstream before deletion, and the split is now a derived,
  RED-controlled bound rather than a convention, so it cannot creep back.
- `2026-08-28`: re-measured while running an unrelated slice, and two axes are worse than the opening
  boundary recorded. `docs/research/*.md` is 63 of 64 files and 639 of 640 lines on its widest member, with
  `health_targets.files == enforcement_ceilings.files`, so there is no warning band and no declared
  rollover — the next research record is the last one. `.4` now owns that explicitly. `MEMORY.md` has the
  same shape at 46 of 50 lines, which is why every slice this session had to hand-compress it; `.5` opens
  to decide whether the 18-line fixed preamble belongs inside the bounded pointer at all.
- `2026-08-14`: prioritize the current knowledge card, then the task plane. The card has one line left and is a
  likely `.f` writer target; the task collection is already at 90% but still has 16 opening-boundary slots.
- `2026-08-14`: record maintained and immutable large-member warnings without assuming they share a remedy.
  Book content is writable product documentation; accepted research/snapshot evidence may require a route or
  lifecycle change rather than an in-place edit.

## Open Questions

- Which fact-card content is current authority versus immutable structural-qualification history? `.1` owns the
  exact route-preserving split.
- Should task capacity use routed partitioning, a newly derived profile, or both? `.2` must decide from census.

## Blockers

- Execution is intentionally sequenced after alignment containment. The tracking boundary blocks no current
  migration leaf; `.1` must close before behavioral `.f` needs another production-genericity fact update.

### Acceptance Checklist (enforced) — `LIVE-DOCUMENT-PRESSURE-HEADROOM.0`

- [x] **REPRODUCE / MEASURE** — exact repository metrics reproduce every opening row; the resulting ownership
  cost is also measured at 145 task files and a 398-line derived catalog.
- [x] **ROOT CAUSE (WHY + WHERE)** — canonical current surfaces grew under individually valid writes, while their
  fixed count/per-member authorities have no shared pressure frontier or ordered remedy owner.
- [x] **ADDRESSED (verified)** — this task owns four lifecycle-specific leaves and prioritizes the one-line card
  stop before task-plane capacity; no content or bound is changed by ownership.
- [x] **NO REGRESSION** — governed source/content, decisions, research, validation, README, mdBook, product code,
  and all live-size literals are byte-identical; catalogs, Knowledge Map, live-size, and doctrines pass.
- [x] **GENERICITY** — leaves are separated by lifecycle and authority coupling, not by document subject; a future
  remedy must remain portable and measured rather than special-casing current filenames in a checker.
- [x] **LOCKSTEP** — this tree, derived task catalog, live-size registry metrics, existing decision-pressure owner,
  and `MEMORY.md` agree on the exact boundary and next eligible remediation.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-14` | `.0` ownership | exact metrics; Knowledge Map routing; existing owner census; task catalog; content/authority diffs; live-size/doctrine | seven axes pinned; ownership-only resulting tree 145 task files / 398 index lines; no governed content or bound change |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.0 — own the current live-surface pressure frontier` | one bounded owner over ordered independent remedies |

## Changelog

- `2026-08-14`: created from the post-`.2.2` live-size report; pins seven non-rolling pressure axes, excludes the
  separately owned decision plane, and leaves the active alignment migration transaction unchanged.
