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
  Status: done
  Goal: measure and decide the active-tree information architecture
  Children: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1`, `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.2`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1`
  Status: done
  Goal: census the target's semantic roles, active frontier, readers, writers, and reconstruction invariants
  Acceptance: a repository-local report distinguishes current state from historical evidence, enumerates every
  consumer and update seam, and derives candidate partition boundaries without editing the target
  Verification: exact 13-region source accounting closes 2,393 lines / 222,616 bytes; 32 direct-path and 89
  identifier-reference input files classified; generic readers and sole manual writer enumerated; target diff empty
  Commit: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1 — census active task roles and consumers`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.2`
  Status: done
  Goal: select and specify the bounded active-tree topology
  Acceptance: an accepted decision fixes stable routes, partition and aggregate bounds, update/rotation rules,
  source provenance, migration stages, and rejection cases; no policy question remains hidden in implementation
  Verification: ADR 0019 fixes three authorities, seven semantic parts, 15 exact source regions, current-state
  precedence, atomic writer transaction, independent limits, and four migration stages; target diff empty;
  mandatory status rollover seals 12 exact records in authenticated segment 0003 and reconstructs the prior root
  Commit: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.2 — decide the bounded active task topology`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2`
  Status: done
  Goal: implement neutral enforcement before moving active evidence
  Children: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.1`, `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.1`
  Status: done
  Goal: implement the data contract and neutral verifier in source-locked state
  Acceptance: the checker pins the committed target, rejects premature destinations and malformed contracts,
  runs unconditionally through doctrine enforcement, and passes focused fail-closed tests
  Verification: exact commit/blob/SHA/metrics/index identity and 15-region/7-part topology pass; every destination
  absent; 29/29 focused source/topology/route/payload/bound cases plus composed live-size/doctrine gates and full
  CI green (1,779 passed / five ignored, rustdoc, mdBook, final locality)
  Commit: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.1 — enforce the source-locked active task contract`

- ID: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2`
  Status: done
  Goal: commit the final active source boundary and migration inputs
  Acceptance: a clean commit leaves the target complete and unpartitioned at an exact identity while every
  migration input, semantic marker, and destination-absence condition is independently verifiable
  Verification: exact boundary/regions/routes, 32/32 focused cases, six doctrines, full CI (1,779 passed / five
  ignored), rustdoc, mdBook, final locality, target diff, and destination absence pass
  Commit: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2 — complete the active source lock inputs`

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
| 1 | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.3.1` | `pending` | atomically materialize the committed bounded active topology |

### Acceptance Checklist (enforced) — `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2`

- [x] **REPRODUCE / MEASURE** — the real report reopens 2,393 lines / 222,616 bytes / max 191 at SHA-256
  `9284dce4…a19d4`, verifies 15 exact region identities, and derives seven payloads of 257–488 lines.
- [x] **ROOT CAUSE (WHY + WHERE)** — six commit-subject IDs occur only as tree-relative source shorthand, so a
  full-ID-only payload check made the `complete` contract unreachable; the mismatch is at route/source identity.
- [x] **ADDRESSED (verified)** — 52 canonical routes now bind only to their full or exact tree-relative source
  literal, token-match their primary payload, and remain constrained by commit-subject membership.
- [x] **NO REGRESSION** — 32/32 focused mutations, real report, six doctrines, full CI (1,779 pass / five ignored),
  rustdoc, mdBook, final locality, exact source diff, and destination absence pass.
- [x] **GENERICITY** — the checker derives permitted shorthand from the declared tree id; every production route,
  path, span, digest, marker, and limit remains contract data.
- [x] **LOCKSTEP** — contract/checker/ADR/fact/task/roadmap/status/resume/live-doc doctrine/mdBook, 160-fact
  Knowledge Map, 159-card catalog, canonical catalogs, and exact book aggregate authority agree on `.3.1` next.

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

## Census outcome

The [active task-evidence containment census](../research/active-task-evidence-containment-census.md) accounts for
every source byte across 13 semantic regions. The 1,392-line / 124,201-byte legacy `Current frontier` section is
actually an activity ledger containing `.9`–`.13`, not a bounded next-action view. The census classifies 32
direct-path and 89 identifier-reference input files, 18 mdBook citations, 24 Knowledge Map evidence cards, all
generic catalog/roadmap/acceptance/live-size readers, and the manual COMMIT writer; no executable opens or rewrites
the target and no consumer uses a Markdown fragment.

The target has no internally consistent eligible frontier: verified commits and inline evidence close several
headers still marked active/in progress, while `.6`/`.7` are formal pending nodes described as blocked and the
formal blocker section says none. The report therefore preserves the literal source and hands `.1.2` three
measured choices. It favors, but does not yet accept, a hybrid bounded active root + semantic live parts + exact
pre-migration capsule with an explicit future writer/rotation contract. The target remains byte-identical.

## Accepted active topology

[ADR 0019](../decisions/0019-bounded-active-task-root-and-semantic-evidence-parts.md) accepts the hybrid. The
stable PDF task path becomes a bounded active current root; `docs/tasks/pdf-variant-digestion/` carries a bounded
index/manifest and seven sealed semantic legacy parts; and
`docs/archive/tasks/pdf-variant-digestion/source-through-2026-08-09.md` retains exact provenance. Fifteen
contiguous source regions cover every byte and map exactly once into marked semantic payloads.

The normalized root initially reports `No eligible frontier.` Completion commits plus inline evidence outrank
stale open headers; explicit unmet dependencies are blocked; ambiguous candidates require revalidation. Future
work creates a new semantic activity part and updates root + part atomically. Parts split only at a child/container
boundary before rollover. Dedicated root/index/part/capsule limits and a schema-closed 24-part / 32-region /
128-leaf manifest prevent the architecture from becoming another unbounded route.

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
- `2026-08-09`: `.1.1` found no automated target writer and no fragment-sensitive consumer. Preserve the stable
  root and leaf ids, but let `.1.2` move detail behind a complete bounded route.
- `2026-08-09`: Do not nominate a PDF product leaf from contradictory legacy text. `.1.2` must define explicit
  current-state precedence and represent parked/blocked work honestly before migration.
- `2026-08-09`: ADR 0019 accepts the hybrid active root, seven semantic legacy parts, and exact capsule. It differs
  from terminal containment because the root stays active and all future writes use an atomic root+activity-part
  transaction under fixed collection limits.
- `2026-08-09`: Normalize the initial frontier to no eligible leaf. `.6`, `.7`, and `.9.10` remain non-eligible
  until a newly scoped continuation leaf revalidates their current evidence/dependency.
- `2026-08-09`: `.2.1` keeps `input_state: topology_declared`: exact region digests, result-part identities, and
  primary leaf routes are forbidden until `.2.2` derives them against the committed checker boundary.
- `2026-08-09`: Canonical history identity and literal source spelling are distinct. A legacy route may name only
  its full ID or exact tree-relative suffix as `source_literal`; index/manifest identity stays fully qualified.
- `2026-08-09`: Source locking means four matching authorities—boundary commit bytes/object, stage-zero index,
  current regular file, and SHA-256/metrics—and rejects the existence of either destination directory.
- `2026-08-09`: Git-backed sealing is necessarily a following transaction: finish and commit the part, then pin
  that ancestor commit/blob before later work proceeds. Legacy parts are independently fixed by exact markers.

## Open Questions

- None for implementation. ADR 0019 closes the topology, authority, state-precedence, update, limit, and migration
  policy; `.2.1` may split only on a concrete implementation dependency, not reopen the decision by convenience.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-09` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.0` | SHA-256/blob/metrics/history; HEAD diff; task catalog; six doctrines; mdBook; live-size | exact untouched source boundary; all gates green |
| `2026-08-09` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1` | 13-region byte closure; path/id/reader/writer census; target SHA/blob/diff; catalogs; doctrines; mdBook | complete read-only report; target unchanged; `.1.2` decision inputs closed |
| `2026-08-09` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.2` | ADR/index/KM/live-doc cross-links; limit arithmetic; target SHA/blob/diff; lossless status reconstruction/chain; catalogs; doctrines; mdBook | hybrid accepted; status segment 0003 authenticated; `.1` closed; implementation inputs complete |
| `2026-08-09` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.1` | Perl syntax; 29 focused cases; real report; six doctrines; full CI; target blob/diff; destination residue | 1,779 pass / five ignored; source/topology enforced; target untouched; destinations absent; `.2.2` ready |
| `2026-08-09` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2` | exact regions/routes; 32 focused cases; six doctrines; full CI; target diff; destination residue | source-locked/complete; 1,779 pass / five ignored; target untouched; destinations absent; `.3.1` ready |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ACTIVE-TASK-EVIDENCE-CONTAINMENT.0` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.0 — own and pin the active task boundary` | ownership and untouched baseline |
| `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1 — census active task roles and consumers` | exhaustive read-only architecture input |
| `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.2` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.2 — decide the bounded active task topology` | ADR 0019; implementation policy closed |
| `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.1` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.1 — enforce the source-locked active task contract` | neutral checker; topology-declared source lock |
| `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2` | `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2 — complete the active source lock inputs` | final boundary; exact regions and canonical/literal routes |

## Changelog

- `2026-08-09`: `.0` created the separately owned program, pinned the untouched PDF-task identity/pressure and
  target consumers, surfaced the stale current-frontier/status contradiction, and handed off to `.1.1`.
- `2026-08-09`: `.1.1` closed the semantic-region, current-authority, consumer, writer, and reconstruction census;
  no target byte moved, and `.1.2` now owns the active topology decision.
- `2026-08-09`: `.1.2` accepted ADR 0019's bounded active root, semantic live parts, exact provenance, normalized
  no-eligible-frontier state, and atomic writer/rotation contract. Its status entry triggered a lossless 12-record
  rollover into authenticated segment 0003; `.1` closed and `.2.1` is next.
- `2026-08-09`: `.2.1` implemented the neutral source-locked/complete/migrated contract, 29 fail-closed cases, and
  unconditional doctrine wiring. The PDF source remains byte-identical, every destination is absent, and `.2.2`
  now owns the committed boundary plus exact region/route inputs.
- `2026-08-09`: `.2.2` pinned clean boundary `f04db37a`, all 15 region identities, and 52 primary routes. A
  full-ID-only contradiction exposed mixed historical shorthand; constrained `source_literal` makes that relation
  exact without changing source bytes. Thirty-two focused cases and full CI pass; `.2` closes and `.3.1` is next.
