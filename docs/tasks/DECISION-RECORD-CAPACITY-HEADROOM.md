# DECISION-RECORD-CAPACITY-HEADROOM: restore room for durable architecture decisions

## Metadata

- Tree ID: `DECISION-RECORD-CAPACITY-HEADROOM`
- Status: `active` (tracking-only while `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT` closes)
- Roadmap lane: repository durability and portability
- Created: `2026-08-14`
- Last updated: `2026-08-14`
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
  Children: `.0`, `.0a`, `.1`, `.2`

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
  Status: `pending`
  Goal: census decision-record growth and decide a bounded, lossless capacity architecture
  Acceptance: exact population/history, all readers and writers, numbering/index/retrieval invariants, coupled
  authorities, candidate remedies, migration/rollback rules, and rejected alternatives are measured and an ADR
  accepts one design before any limit or topology changes
  Verification: `pending`
  Commit: `pending`

- ID: `DECISION-RECORD-CAPACITY-HEADROOM.2`
  Status: `pending`
  Goal: implement and fault-test the accepted capacity architecture
  Acceptance: the accepted transaction preserves every decision and route exactly, supplies actionable
  headroom, keeps aggregate reachability, synchronizes the fact plane and book, passes doctrine, and closes clean
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `DECISION-RECORD-CAPACITY-HEADROOM.0` | `done` | exact clean pressure boundary and ownership are pinned |
| 2 | `DECISION-RECORD-CAPACITY-HEADROOM.0a` | `done` | per-record line/byte pressure is distinguished from collection count |
| 3 | `DECISION-RECORD-CAPACITY-HEADROOM.1` | `pending` | measure and decide before any future record exhausts the remaining three slots |

## Decisions

- `2026-08-14`: open a separate task rather than silently widen `decision_records.files`. ADR 0029 derived 44
  from a 30-file population plus one measured peak day; the present 41 files have consumed that intended
  buffer, so the signal requires a new measured remedy.
- `2026-08-14`: make this tree tracking-only until `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT` reaches its next
  clean handoff. The risk must be durable now, but opening it does not justify stranding an active transaction.
- `2026-08-14`: ADR 0038's per-file pressure is independent of the collection count. `.1` must decide how an
  accepted, effectively immutable record is classified or losslessly partitioned; a count raise cannot silence
  or cure a 31,833-byte member, and rewriting accepted rationale in place is not an admissible shortcut.

## Open Questions

- Should the canonical collection gain a bounded routed partition, a newly derived count profile, or both?
  `.1` must answer from reader/writer and growth evidence before any authority moves.

## Blockers

- Execution is intentionally sequenced after the current alignment containment activity; measurement is not
  otherwise blocked.

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `DECISION-RECORD-CAPACITY-HEADROOM.0 — track decision-record capacity pressure` | ownership and exact pressure boundary only |
| `.0a` | `DECISION-RECORD-CAPACITY-HEADROOM.0a — pin independent decision-record size pressure` | distinguish immutable-member shape from collection count before design |

## Changelog

- `2026-08-14`: created from the alignment source-lock gate's 41-of-44 decision-record pressure finding; no
  decision record, bound, or product state changed, and `.1` owns the measured architecture decision.
- `2026-08-14`: `.0a` records ADR 0038's separate 474-line / 31,833-byte pressure after the next full gate; no
  accepted decision or live-size authority changed.
