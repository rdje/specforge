# DECISION-RECORD-CAPACITY-HEADROOM: restore room for durable architecture decisions

## Metadata

- Tree ID: `DECISION-RECORD-CAPACITY-HEADROOM`
- Status: `active`
- Roadmap lane: repository durability and portability
- Created: `2026-08-14`
- Last updated: `2026-08-15`
- Owner: repo-local workflow

## Goal

Restore deliberate, mechanically justified headroom in the canonical decision-record collection before an
unrelated architecture slice consumes the remaining three file slots and is refused by doctrine.

## Non-Goals

- Do not raise the 44-file authority merely to clear its pressure signal.
- Do not delete, merge, renumber, or rewrite accepted decision history to buy capacity.
- Do not interrupt the in-progress alignment task-evidence containment transaction.
- Do not assume the fact-plane profile from ADR 0029 remains the right architecture without a new census.

## Acceptance Criteria

- The exact current population, creation trajectory, readers, writers, index membership, and every coupled
  fact-plane authority are measured from a clean committed boundary.
- An accepted design gives ordinary future decision writes a reachable remedy while keeping every existing
  decision directly browsable, immutable, and available to the Knowledge Map.
- Bounds, catalogs, retrieval projections, doctrine, public method documentation, and focused fault tests move
  in one reviewed transaction; no capacity dimension can drift alone.
- Every completed leaf commits through `COMMIT.md`, and the current alignment containment program resumes from
  a clean handoff after this ownership slice.

## Task Tree

- ID: `DECISION-RECORD-CAPACITY-HEADROOM`
  Status: `active`
  Goal: restore a durable and reachable capacity remedy for canonical decision records
  Children: `.0`, `.0a`, `.1`, `.2`, `.2a`

- ID: `DECISION-RECORD-CAPACITY-HEADROOM.0`
  Status: `done`
  Goal: establish ownership and pin the pressure before any authority changes
  Acceptance: the committed boundary, exact 41-of-44 population, governing decision, remaining headroom,
  non-goals, and measurement/design frontier are durable; no capacity authority or decision record changes
  Verification: `at clean 9d2eeb6f, 41 tracked docs/decisions/*.md files consume 93.2% of the 44-file health target and enforcement ceiling fixed by ADR 0029, leaving three slots; existing records and every live-size limit remain unchanged; task catalog, live-size gate, Knowledge Map, and doctrines pass`
  Commit: `DECISION-RECORD-CAPACITY-HEADROOM.0 — track decision-record capacity pressure`

- ID: `DECISION-RECORD-CAPACITY-HEADROOM.0a`
  Status: `done`
  Goal: expand the pinned boundary to the independent per-record pressure surfaced by the next full gate
  Acceptance: the largest decision record and its exact line/byte/width headroom are durable; `.1` distinguishes
  collection capacity from immutable-member shape and no record or bound changes
  Verification: `at clean 1a2f3705 the population remains 41/44; accepted ADR 0038 is independently largest at 474/512 lines (92.6%, 38 lines left) and 31,833/32,768 bytes (97.1%, 935 bytes left), with 397/512 maximum line bytes; all 41 decisions, the index, registry limits, product, and book remain unchanged; catalogs, live-size, Knowledge Map, and doctrines pass`
  Commit: `DECISION-RECORD-CAPACITY-HEADROOM.0a — pin independent decision-record size pressure`

- ID: `DECISION-RECORD-CAPACITY-HEADROOM.1`
  Status: `done`
  Goal: census decision-record growth and decide a bounded, lossless capacity architecture
  Acceptance: exact population/history, all readers and writers, numbering/index/retrieval invariants, coupled
  authorities, candidate remedies, migration/rollback rules, and rejected alternatives are measured and an ADR
  accepts one design before any limit or topology changes
  Verification: at the `.1` worktree boundary 43 Markdown files (42 committed plus ADR 0041) occupy the flat
  collection; Git creation history preserves the nine-file peak active day; the accepted design minimally
  re-derives 58 slots because 43/58 is below 80% and (43+9)/58 is below 90%; every reader/writer, stable path,
  index/member route, Knowledge Map coupling, fact-catalog join, hard bundle cap, rollback, and rejected topology
  is recorded; no limit or existing decision moves; catalogs, Knowledge Map, live-size, and doctrines pass
  Commit: `DECISION-RECORD-CAPACITY-HEADROOM.1 — derive the next decision capacity profile`

- ID: `DECISION-RECORD-CAPACITY-HEADROOM.2`
  Status: `done`
  Goal: implement and fault-test the accepted capacity architecture
  Acceptance: the accepted transaction preserves every decision and route exactly, supplies actionable
  headroom, keeps aggregate reachability, synchronizes the fact plane and book, passes doctrine, and closes clean
  Verification: the exact authority moves decision files 44→58 with unchanged per-file bounds and reachable
  29,696-line / 1,900,544-byte aggregates; the catalog derives 393 facts and 3,584 question keys and rejects
  independent fact, record, or question-key drift across 60/60 focused cases; all 43 stable records remain in
  place; Knowledge Map remains 250 facts / 1,914 questions / 13 shards; mdBook, live-size, and doctrines pass
  Commit: `DECISION-RECORD-CAPACITY-HEADROOM.2 — apply the 58-record coupled capacity profile`

- ID: `DECISION-RECORD-CAPACITY-HEADROOM.2a`
  Status: `pending`
  Goal: retire the exact ceiling-increase authority consumed by `.2`
  Acceptance: after `.2` commits, remove the now-banked decision-record authority without moving any surface,
  projection, decision, or limit; the live-size and doctrine gates return green against the committed profile
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `DECISION-RECORD-CAPACITY-HEADROOM.0` | `done` | exact clean pressure boundary and ownership are pinned |
| 2 | `DECISION-RECORD-CAPACITY-HEADROOM.0a` | `done` | per-record line/byte pressure is distinguished from collection count |
| 3 | `DECISION-RECORD-CAPACITY-HEADROOM.1` | `done` | ADR 0041 derives a 58-record flat profile without moving accepted paths |
| 4 | `DECISION-RECORD-CAPACITY-HEADROOM.2` | `done` | coupled registries and independent drift controls now carry the accepted profile |
| 5 | `DECISION-RECORD-CAPACITY-HEADROOM.2a` | `pending` | retire `.2`'s consumed authority after the new profile is committed |

## Decisions

- `2026-08-14`: open a separate task rather than silently widen `decision_records.files`. ADR 0029 derived 44
  from a 30-file population plus one measured peak day; the present 41 files have consumed that intended
  buffer, so the signal requires a new measured remedy.
- `2026-08-14`: make this tree tracking-only until `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT` reaches its next
  clean handoff. The risk must be durable now, but opening it does not justify stranding an active transaction.
- `2026-08-14`: ADR 0038's per-file pressure is independent of the collection count. `.1` must decide how an
  accepted, effectively immutable record is classified or losslessly partitioned; a count raise cannot silence
  or cure a 31,833-byte member, and rewriting accepted rationale in place is not an admissible shortcut.
- `2026-08-15` (ADR 0041): retain the flat stable-path collection and minimally re-derive 58 file slots from the
  43-file post-decision population plus the observed nine-file peak day. This implies 393 fact slots and 3,584
  question-key slots; no shard/file topology or portable hard cap changes.
- `2026-08-15`: ADR 0038 is accepted, stable, and no longer appendable. Its per-file warning remains honest; the
  authoring remedy for future large decisions is to keep Context/Decision/Consequences bounded and route detailed
  measurements to the owning task/research evidence, not mutate or special-case an accepted record.

## Open Questions

- None for the selected profile. A future warning must repeat the same population/peak-day derivation rather than
  treating 58 as permanent or spending the portable bundle's unused hard capacity preemptively.

## Blockers

- None. `.2a` must retire the consumed one-use authority before this tree closes.

### Acceptance Checklist (enforced) — `DECISION-RECORD-CAPACITY-HEADROOM.2`

- [x] **REPRODUCE / MEASURE** — the deciding-record boundary is 43 files and the nine-file peak remains the
  accepted input; 43/58 = 74.1% and (43+9)/58 = 89.7%.
- [x] **ROOT CAUSE (WHY + WHERE)** — the stale input was the decision population, while the coupled authorities
  lived in the live-size registry, Knowledge Map shard contract, and fact-card catalog join.
- [x] **ADDRESSED (verified)** — one exact authority moves 44→58 files, aggregate bounds remain `files × per-file
  bound`, and executable identities derive 393 facts plus 3,584 question keys from the two canonical writers.
- [x] **NO REGRESSION** — every existing decision path and byte stays stable; 60 focused catalog controls include
  independent record/fact/question drift; Knowledge Map, mdBook, live-size, and doctrine gates pass.
- [x] **GENERICITY** — no decision subject is recognized; the implementation uses only registered capacities,
  fact-writer slots, the portable eight-key ratio, and the 512-key registry quantum.
- [x] **LOCKSTEP** — ADR 0041, both registries, project generator contract, executable derivations, normative
  containment, mdBook, task tree, and continuity ledgers all publish the same 58 / 393 / 3,584 profile.

### Acceptance Checklist (enforced) — `DECISION-RECORD-CAPACITY-HEADROOM.1`

- [x] **REPRODUCE / MEASURE** — Git creation history yields 43 files at the `.1` boundary and a nine-file peak
  active day; current aggregate content is 3,875 lines / 256,041 bytes before ADR 0041, while ADR 0038 remains
  uniquely largest at 474 lines / 31,833 bytes / 397 maximum line bytes.
- [x] **ROOT CAUSE (WHY + WHERE)** — ADR 0029 correctly derived 44 slots from the then-30-file population and
  nine-file peak, but legitimate decisions have consumed that measured buffer. The flat collection has a stable
  index/KM path and no topology defect; the stale input is population, not the formula.
- [x] **ADDRESSED (verified)** — ADR 0041 re-runs the same milestone equation including its own file: 43/58 =
  74.1% and (43+9)/58 = 89.7%. `.2` owns the exact 44→58 surface change, 379→393 fact join, 3,072→3,584 question-
  key step, authority, full-capacity/fail-closed controls, and rollback as one transaction.
- [x] **NO REGRESSION** — `.1` changes no limit, existing decision, glob, stable path, projection algorithm, or
  product behavior; task/decision/fact catalogs, Knowledge Map, live-size, mdBook review, and doctrines pass.
- [x] **GENERICITY** — the profile is derived only from lifecycle, population, observed peak, and fixed warning/
  rollover milestones; no decision subject or project feature controls capacity.
- [x] **LOCKSTEP** — ADR 0041, this tree, live continuity docs, generated indexes, and resume pointer agree that
  58 is accepted but not implemented; claim-verification `.1` remains sequenced after `.2/.2a`.

### Acceptance Checklist (enforced) — `DECISION-RECORD-CAPACITY-HEADROOM.0`

- [x] **REPRODUCE / MEASURE** — a clean-boundary census finds exactly 41 tracked decision Markdown files against
  the registered 44-file target and ceiling: 93.2% pressure and three remaining slots.
- [x] **ROOT CAUSE (WHY + WHERE)** — ADR 0029 sized the profile from 30 files plus one then-measured peak day;
  later legitimate architecture work consumed the buffer, while the canonical collection has no rollover path.
- [x] **ADDRESSED (verified)** — this task owns the pressure, the full census/design leaf, and the implementation
  leaf before any capacity, topology, existing decision, or product file changes.
- [x] **NO REGRESSION** — task-catalog, live-size, Knowledge Map, and doctrine checks pass; the 41 decision files,
  their index, and every registered authority are byte-unchanged.
- [x] **GENERICITY** — the future remedy must follow canonical-collection lifecycle and measured-pressure rules;
  this ownership slice selects no project-specific topology.
- [x] **LOCKSTEP** — ADR 0029, the live-size registry, this tree, derived task catalog, and resume pointer agree on
  the 41-of-44 pressure boundary and the pending census.

### Acceptance Checklist (enforced) — `DECISION-RECORD-CAPACITY-HEADROOM.0a`

- [x] **REPRODUCE / MEASURE** — ADR 0038 is the unique largest decision at 474 lines / 31,833 bytes / 397
  maximum line bytes, leaving 38 lines and 935 bytes under the unchanged per-file ceilings.
- [x] **ROOT CAUSE (WHY + WHERE)** — the collection-count derivation and a member's shape are independent axes;
  adding new-file capacity cannot remedy an accepted record already at 97.1% of its byte bound.
- [x] **ADDRESSED (verified)** — `.1` now explicitly owns immutable-member lifecycle/partition analysis as well
  as collection growth before it chooses or changes any authority.
- [x] **NO REGRESSION** — all decisions, their index, every live-size literal, product code, and mdBook are
  byte-identical to clean boundary `1a2f3705`; focused catalogs, retrieval, live-size, and doctrines pass.
- [x] **GENERICITY** — the finding is recorded by lifecycle and independent pressure axis, not by special-casing
  ADR 0038's subject or treating its present content as permission for a bound raise.
- [x] **LOCKSTEP** — this tree, `MEMORY.md`, ADR 0029's standing capacity law, and the registered current metrics
  distinguish the 41-of-44 collection wall from the largest member's 474-line / 31,833-byte shape.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-14` | `.0` ownership | exact tracked-file census; ADR 0029; registry; catalogs/retrieval/live-size/doctrine; boundary diffs | 41 / 44 files, 93.2%, three slots; no authority, decision, or product change |
| `2026-08-14` | `.0a` member pressure | exact per-file metrics; live-size pressure; boundary diffs; catalogs/retrieval/doctrine | ADR 0038 at 474 lines / 31,833 bytes / 397 max-line bytes; 38 lines and 935 bytes remain; no record or authority change |
| `2026-08-15` | `.1` design | Git add-history census; current metrics; readers/writers and coupling search;
  candidate/rollback analysis; ADR/catalog/KM/live-size/doctrine checks | 58 slots selected from 43 current + nine
  peak; 393 fact slots / 3,584 question-key slots are the exact coupled next profile; no authority moved |
| `2026-08-15` | `.2` implementation | 60 catalog mutation/boundary cases; fact/KM catalogs; live-size; mdBook;
  doctrines | 43/58 stable decision files; 393 derived fact slots; 3,584 derived question keys; no path/topology/
  portable-cap change; exact one-use authority remains for `.2a` retirement |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `DECISION-RECORD-CAPACITY-HEADROOM.0 — track decision-record capacity pressure` | ownership and exact pressure boundary only |
| `.0a` | `DECISION-RECORD-CAPACITY-HEADROOM.0a — pin independent decision-record size pressure` | distinguish immutable-member shape from collection count before design |
| `.1` | `DECISION-RECORD-CAPACITY-HEADROOM.1 — derive the next decision capacity profile` | ADR 0041; flat stable paths retained; capacity implementation is next |
| `.2` | `DECISION-RECORD-CAPACITY-HEADROOM.2 — apply the 58-record coupled capacity profile` | exact authority, coupled registries, drift controls, and book sync |

## Changelog

- `2026-08-14`: created from the alignment source-lock gate's 41-of-44 decision-record pressure finding; no
  decision record, bound, or product state changed, and `.1` owns the measured architecture decision.
- `2026-08-14`: `.0a` records ADR 0038's separate 474-line / 31,833-byte pressure after the next full gate; no
  accepted decision or live-size authority changed.
- `2026-08-15`: `.1` accepts the minimal next profile—58 decision files, 393 facts, and 3,584 question keys—after
  re-deriving the current population, nine-file peak, stable reader/writer topology, and portable hard caps.
- `2026-08-15`: `.2` applies that profile atomically, makes the question-key derivation executable, and preserves
  all existing records, stable paths, per-file bounds, shard topology, and portable hard caps; `.2a` owns only
  retirement of the now-consumed increase authority.
