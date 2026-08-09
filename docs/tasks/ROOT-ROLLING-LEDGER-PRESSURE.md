# ROOT-ROLLING-LEDGER-PRESSURE: restore bounded headroom in current continuity ledgers

## Metadata

- Tree ID: `ROOT-ROLLING-LEDGER-PRESSURE`
- Status: `active`
- Roadmap lane: process / continuity / rolling-ledger pressure
- Created: `2026-08-09`
- Last updated: `2026-08-09`
- Owner: continuity maintainers through the repo-local workflow

## Goal

Remeasure all four migrated root continuity ledgers at a clean committed boundary and restore warning-safe,
directly retrievable headroom wherever the existing rollover protocol requires it. `CHANGES.md` is the immediate
trigger; development and status pressure must be evaluated in the same census so adjacent ledgers are not allowed
to fail one commit later.

## Non-Goals

- Do not trim, summarize, reorder, or rewrite historical records to manufacture headroom.
- Do not widen a health target, warning/rollover milestone, enforcement ceiling, or archive capacity.
- Do not alter product behavior, canonical product artifacts, shared toolchain state, or the active PDF task.
- Do not redesign the accepted per-ledger index/manifest topology unless exact current evidence proves it unsafe.

## Acceptance Criteria

- A clean-boundary census measures records, lines, bytes, maximum line width, pressure, live-window composition,
  archive capacity, reader/writer seams, and reconstruction identity for all four roots.
- Every ledger at or beyond its declared rollover signal receives an exact whole-record segment transaction; any
  ledger left unrotated has measured headroom and a durable reason.
- Live root, segment, per-ledger index/manifest, landing, registry, and generic surface metrics remain aligned and
  reconstruct all prior bytes under the existing newest-first chronology.
- Focused protocol cases, exact before/after reconstruction, complete link/chain/residue checks, all doctrines,
  full CI, rustdoc, mdBook, and final project-data locality pass.
- Each completed leaf is committed through `COMMIT.md`, and the repository is handoff-ready between leaves.

## Task Tree

- ID: `ROOT-ROLLING-LEDGER-PRESSURE`
  Status: active
  Goal: restore bounded, lossless continuity-ledger headroom
  Children: `ROOT-ROLLING-LEDGER-PRESSURE.0`, `ROOT-ROLLING-LEDGER-PRESSURE.1`,
  `ROOT-ROLLING-LEDGER-PRESSURE.2`

- ID: `ROOT-ROLLING-LEDGER-PRESSURE.0`
  Status: done
  Goal: pin, census, and lock the exact four-ledger pressure plan
  Acceptance: the committed roots, live windows, archive chains/capacity, consumers, pressure signals, and
  triggered ledger set are measured before changing archive authority; each triggered ledger has
  a deterministic whole-record cut, segment/index/manifest identity plan, reconstruction proof, same-volume
  write/rollback order, and bounded result before materialization
  Verification: exact opening identities, four candidate segments, bounded resulting roots, archive capacity,
  JSONL plan schema/bounds, current protocol reconstruction, current surface report, catalogs/map, doctrines,
  mdBook, and full CI pass
  Commit: `ROOT-ROLLING-LEDGER-PRESSURE.0 — lock the four-ledger rollover plan`

- ID: `ROOT-ROLLING-LEDGER-PRESSURE.1`
  Status: in_progress
  Goal: materialize the required ledger rotations under the existing protocol
  Acceptance: exact planned segments and bounded roots land atomically with complete chronology, retrieval,
  reconstruction, pressure, residue, doctrine, and full-CI proof
  Verification: pending
  Commit: pending

- ID: `ROOT-ROLLING-LEDGER-PRESSURE.2`
  Status: pending
  Goal: independently audit the resulting continuity plane and close this tree
  Acceptance: a clean committed-tree audit reconstructs every source byte, proves direct bounded retrieval and
  future append capacity, synchronizes the mdBook method, and closes the tree
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ROOT-ROLLING-LEDGER-PRESSURE.1` | `in_progress` | bind health authority and materialize the four exact cuts before another ordinary append |

### Acceptance Checklist (enforced) — `ROOT-ROLLING-LEDGER-PRESSURE.0`

- [x] **REPRODUCE / MEASURE** — exact committed record/line/byte/width metrics, live-window composition,
  per-dimension pressure, archive-chain membership/capacity, and readers/writers close for all four ledgers.
- [x] **ROOT CAUSE (WHY + WHERE)** — each pressure signal is attributed to whole-record live-window growth and
  its exact controlling registry/surface dimensions, not treated as permission to trim or widen limits.
- [x] **ADDRESSED (verified)** — the triggered ledger set and exact deterministic transaction plan are durable;
  only required `.0` continuity records prepend after the pinned roots, while segments, indexes, manifests,
  limits, and product state remain byte-identical; only the required mdBook aggregate-change authority advances.
- [x] **NO REGRESSION** — source/index/archive diffs, protocol report, live-size report, catalogs/map, doctrines,
  mdBook, locality, and final residue checks pass.
- [x] **GENERICITY** — measurement follows the four data registry records and grammar adapters without hardcoded
  record titles, project values, host paths, or ledger-specific content guesses.
- [x] **LOCKSTEP** — this tree, roadmap/status/resume, rolling-ledger fact, live doctrine, and mdBook identify the
  same boundary, triggered set, transaction plan, and `.1` materialization frontier.

## Opening Signal

The independent active-task closure audit at clean commit `06eb3941` measured `CHANGES.md` at 1,631/1,800 health
lines, already above its 90% rollover milestone. The closure recording commit necessarily added one more whole
record and also left `DEVELOPMENT_NOTES.md` close to line rollover and `LIVE_ACHIEVEMENT_STATUS.md` above its byte
warning. `.0` must remeasure the exact new clean boundary rather than copy those preliminary values.

## Decisions

- `2026-08-09`: Open a separate top-level tree only after active-task containment closed cleanly. Root-ledger
  pressure is independent continuity work and cannot be hidden inside the PDF evidence audit.
- `2026-08-09`: Census all four roots even though changes triggered the work. Coordinating measurement prevents
  a correct changes rotation from handing off to an immediately failing adjacent ledger.
- `2026-08-09`: Reuse the accepted per-ledger index/manifest protocol unless `.0` finds a concrete safety or
  capacity defect. No threshold or ceiling increase is authorized.
- `2026-08-09`: Treat the generic surface health target as milestone authority and the enforcement ceiling as a
  hard quarantine boundary. The focused checker must bind and agree with that authority; its current denominator
  is a defect, not an alternate policy.
- `2026-08-09`: Rotate all four roots. Changes is already at rollover; development and status would cross during
  this three-leaf task; Rust has only 28 line-health points before rollover. The exact cuts are 29/24/12/8 whole
  records and restore every root below warning without widening a control.
- `2026-08-09`: Lock the cuts in the bounded registry-ordered JSONL companion. `.1` must rediscover each exact
  contiguous range despite later task prepends and refuse any ambiguous or drifting identity.

## Open Questions

- None for `.0`. `.1` owns implementation of the selected generic transaction, not a topology or limit decision.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-09` | `.0` | four opening Git blobs/SHA-256 identities; JSONL schema/bounds; 24 protocol and 7 task-catalog self-tests; current protocol/reconstruction report; 244-member canonical catalogs; 161-card/1,146-question map; generic 48-surface report; archive/index/control diff; six doctrines; 1,779 Rust tests / five ignored; Clippy; rustdoc; 36-file / 13,291-line / 808,887-byte mdBook; final locality/residue | exact 29/24/12/8-record plan is reproducible, bounded, and green; required continuity prepends are the only ledger changes |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `ROOT-ROLLING-LEDGER-PRESSURE.0 — lock the four-ledger rollover plan` | opening boundary, pressure root cause, exact transaction plan, no record movement |

## Changelog

- `2026-08-09`: `.0` pinned commit `4d24b13c`, all four root identities and pressure dimensions, the generic/
  focused denominator defect, archive/consumer capacity, and exact 29/24/12/8-record transaction inputs. The
  machine-readable plan and current-truth planes agree; `.1` is active.
- `2026-08-09`: `.0` opened from the clean active-task containment closure and owns the four-ledger pressure
  census before any further ordinary continuity-ledger append.
