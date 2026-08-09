# ACTIVE-TASK-EVIDENCE-CONTAINMENT: keep active task history bounded and resumable

## Metadata

- Tree ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT`
- Status: `active`
- Roadmap lane: process / continuity / active task evidence
- Created: `2026-08-09`
- Last updated: `2026-08-09`
- Owner: repo-local workflow

## Goal

Establish and enforce a bounded, lossless information architecture for an active task tree before its next
ordinary append. The first migration target is `PDF-VARIANT-DIGESTION`; its stable task route must continue to
provide an exact current frontier and direct access to all historical task evidence while the program remains
open.

## Non-Goals

- Do not implement any pending PDF extraction leaf or change product behavior.
- Do not use the terminal closed-tree topology accepted by ADR 0018 for an active program.
- Do not widen an existing health target, warning threshold, rollover threshold, or enforcement ceiling.
- Do not trim, summarize away, renumber, or silently correct historical task evidence.
- Do not generalize the migration beyond evidence established from the first active target.

## Acceptance Criteria

- The untouched active-tree source boundary, current frontier, readers, writers, and information roles are
  measured at an exact committed revision before migration.
- A durable decision defines an active-tree topology with bounded startup/current-state reads, lossless history,
  direct navigation, stable leaf identifiers, and an append contract that remains usable while work continues.
- Neutral enforcement proves source identity, partition completeness, route closure, same-volume locality,
  lifecycle limits, and fail-closed active-frontier behavior.
- `PDF-VARIANT-DIGESTION` is migrated atomically under the accepted contract without changing its program
  meaning, leaf status, or next eligible work.
- The roadmap, task catalog, continuity pointer, doctrine, Knowledge Map, and mdBook are synchronized where their
  owned truth changes.
- Each completed leaf is committed through `COMMIT.md`, and the repository remains handoff-ready between leaves.

## Task Tree

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT`
  Status: active
  Goal: keep active task evidence bounded, complete, and directly resumable
  Children: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.0`, `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1`,
  `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2`, `ACTIVE-TASK-EVIDENCE-CONTAINMENT.3`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.0`
  Status: done
  Goal: establish ownership and pin the untouched first-target baseline
  Acceptance: this tree exists before any target edit; the exact committed source identity, pressure, governing
  constraints, and next design leaf are durable; `PDF-VARIANT-DIGESTION` remains byte-identical
  Verification: target matches HEAD with 2,393 lines / 222,616 bytes / max 191, SHA-256 `9284dce4…a19d4`,
  Git blob `7d89ea4f…b555`; source diff is empty; catalog, doctrines, mdBook, and live-size checks pass
  Commit: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.0 — own and pin the active task boundary`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1`
  Status: active
  Goal: measure and decide the active-tree information architecture
  Children: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1`, `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.2`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1`
  Status: pending
  Goal: census the target's semantic roles, active frontier, readers, writers, and reconstruction invariants
  Acceptance: a repository-local report distinguishes current state from historical evidence, enumerates every
  consumer and update seam, and derives candidate partition boundaries without editing the target
  Verification: pending
  Commit: pending

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.2`
  Status: pending
  Goal: select and specify the bounded active-tree topology
  Acceptance: an accepted decision fixes stable routes, partition and aggregate bounds, update/rotation rules,
  source provenance, migration stages, and rejection cases; no policy question remains hidden in implementation
  Verification: pending
  Commit: pending

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2`
  Status: active
  Goal: implement neutral enforcement before moving active evidence
  Children: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.1`, `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.1`
  Status: pending
  Goal: implement the data contract and neutral verifier in source-locked state
  Acceptance: the checker pins the committed target, rejects premature destinations and malformed contracts,
  runs unconditionally through doctrine enforcement, and passes focused fail-closed tests
  Verification: pending
  Commit: pending

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2`
  Status: pending
  Goal: commit the final active source boundary and migration inputs
  Acceptance: a clean commit leaves the target complete and unpartitioned at an exact identity while every
  migration input, semantic marker, and destination-absence condition is independently verifiable
  Verification: pending
  Commit: pending

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.3`
  Status: active
  Goal: migrate, prove, and close the first-target containment program
  Children: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.3.1`, `ACTIVE-TASK-EVIDENCE-CONTAINMENT.3.2`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.3.1`
  Status: pending
  Goal: atomically migrate the active target under the accepted topology
  Acceptance: current state and complete history occupy their declared bounded routes; exact provenance,
  navigation, partition completeness, frontier semantics, writer behavior, and all doctrine gates pass
  Verification: pending
  Commit: pending

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.3.2`
  Status: pending
  Goal: independently audit the resulting active route and close containment work
  Acceptance: a fresh-clone-equivalent audit proves ordinary PNT can resume and append safely, every source byte
  remains retrievable, no target meaning/status changed, the mdBook method section is current, and this tree closes
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1` | `pending` | measure all semantic roles and consumers before selecting topology |

## First-target baseline

At repository revision `684079b1d8c30b2c94a2d8efb2d2b8efdd3f3195`, the tracked
`docs/tasks/PDF-VARIANT-DIGESTION.md` source is unchanged from its Git object and measures 2,393 lines /
222,616 bytes / 191 maximum content-line bytes. Its SHA-256 is
`9284dce40ad896c3de3811e95c3fdd347132b1849083499e9c543fc9026a19d4`; its Git blob is
`7d89ea4fe59e53e55614b5730bded40d99b4b555`. The path has 70 commits, last changed by `1abfb49c` on
`2026-06-24`. The existing `task_evidence` health target is 278,528 bytes per file, so the source is only 207
bytes below its 80% warning and cannot accept an ordinary PDF-task append before containment.

The stable route serves at least three distinct consumers before the detailed `.1.1` census: startup follows
`MEMORY.md` → `docs/TASK_TREE.md` → this task root; the generated task catalog parses only the root H1 and first
metadata status; and multiple mdBook method sections cite the stable root and historical leaf ids. Ordinary task
writes currently mix topology, current frontier, decisions, verification, commits, and chronology in this one
source.

The read-only boundary also exposes a current-state contradiction that migration must not hide. The declared
`## Current frontier` still names `.9.3` as active, although the same section says `.9.3a` and `.9.3b` are done;
several node headers remain `in_progress` while their bodies record later `done` evidence, and newer `.10i`
completion is not reflected in that frontier paragraph. `.1.1` must distinguish literal historical evidence
from authoritative current state and record any reconciliation explicitly before a compact active root can claim
an exact next leaf.

## Decisions

- `2026-08-09`: Use a new top-level tree because the completed containment program is closed and ADR 0018
  expressly forbids applying its terminal topology to active work.
- `2026-08-09`: Treat `PDF-VARIANT-DIGESTION` as read-only until this tree has measured and accepted an active
  topology. Its byte-warning proximity authorizes containment work, not an ordinary task append.
- `2026-08-09`: Separate measurement, policy, enforcement, source locking, migration, and independent closure so
  each irreversible boundary is reviewable and committed.
- `2026-08-09`: Pin both content and Git identities at the opening boundary. The target remains untouched in
  `.0`; later source locking must name a committed boundary rather than a transient working tree.
- `2026-08-09`: Treat the target's stale frontier/status mismatch as a correctness input, not material to copy
  silently into a new current view or permission to alter historical records.

## Open Questions

- Which semantic partition boundaries preserve the target's current-frontier read while keeping every historical
  leaf and cross-reference directly retrievable? Owned by `.1.1` and `.1.2`; does not block `.0`.
- Should active completion evidence rotate into immutable segments, maintained semantic partitions, or a hybrid?
  Owned by `.1.2`; measurement must decide rather than analogy with the terminal tree.
- Which target statements constitute the authoritative next eligible PDF work when the legacy frontier and node
  status text disagree? Owned by `.1.1`; the answer must be evidence-backed and separately recorded.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-09` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.0` | SHA-256/blob/metrics/history; HEAD diff; task catalog; six doctrines; mdBook; live-size | exact untouched source boundary; all gates green |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ACTIVE-TASK-EVIDENCE-CONTAINMENT.0` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.0 — own and pin the active task boundary` | ownership and untouched baseline |

## Changelog

- `2026-08-09`: `.0` created the separately owned program, pinned the untouched PDF-task identity/pressure and
  target consumers, surfaced the stale current-frontier/status contradiction, and handed off to `.1.1`.
