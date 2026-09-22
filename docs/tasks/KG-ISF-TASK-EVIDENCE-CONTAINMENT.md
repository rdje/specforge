# KG-ISF-TASK-EVIDENCE-CONTAINMENT: bound the KG-ISF task evidence before it refuses its own next slice

## Metadata

- Tree ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT`
- Status: `active`
- Roadmap lane: `R15`/`R16` continuity prerequisite for `KG-ISF-COMPLETENESS.5.xi` and every later leaf
- Created: `2026-09-21`
- Last updated: `2026-09-22`
- Owner: repo-local workflow

## Goal

Losslessly partition `docs/tasks/KG-ISF-COMPLETENESS.md` before its next ordinary slice would exceed the
registered `task_evidence` ceiling, preserving stable leaf identity, a truthful current frontier, complete
historical evidence, bounded retrieval, and an unambiguous write destination for `.5.xi` and later work.

## Why now — the measurement, at `0d828e7e`

| dimension | value | ceiling | health |
| --- | ---: | ---: | ---: |
| bytes | **260,478** | 278,528 | **93.5% — past the 90% rollover milestone** |
| lines | 2,229 | 3,000 | 74.3% |
| longest line | 1,483 | 6,400 | 23.2% |

**The byte dimension binds and there is one slice of headroom left.** The two slices committed on
`2026-09-21` added **12,269** and **13,148** bytes; 18,050 bytes remain. `LIVE-DOC-SIZE` already reports
`task_evidence bytes_each is at or above rollover (93.5%)` on every commit.

**This is the precedent case, not a new judgement.** `EXTRACTION-QUALITY-GAUGE` was migrated on `2026-09-17`
for exactly this reason, and its contract records why waiting is wrong: *"an ordinary slice was refused at
3,012 and had to compact its own evidence to land, which is a policy defect rather than an author problem."*
Compacting evidence to fit is the outcome this tree exists to avoid.

## Non-Goals

- Do not implement `.5.x`/`.5.xi`/`.5.xii`/`.5.xiii`, or any other `KG-ISF-COMPLETENESS` leaf, inside this tree.
- Do not trim, summarize away, rewrite, or renumber established KG-ISF task evidence.
- Do not widen any health target, warning threshold, rollover threshold, or enforcement ceiling.
- Do not copy another tree's partition boundaries without KG-ISF-specific measurement.
- Do not change product behavior, generated artifacts, or any Rust source.

## Acceptance Criteria

- The committed pre-containment source identity is pinned before the source changes.
- Every source byte and every established leaf id stays durably retrievable from the stable task route in
  bounded hops.
- Current KG-ISF state, frontier, blockers, and the next write destination are explicit and mechanically checked.
- `scripts/check_active_task_evidence.pl --check` passes against a new registered contract, and
  `scripts/check_task_evidence_contracts.sh` lists it.
- The surface registry, task catalog, resume pointer, Knowledge Map, and mdBook agree on the migrated topology.

## Pinned source identity (`0d828e7e`, before any change)

- path: `docs/tasks/KG-ISF-COMPLETENESS.md`
- sha256: `7c3d88081e7285ce533362850ccbc21dcc3cc8a0bc08742345c6b1e2a3fca994`
- git blob: `b168d98230712ccc741610521f4571ec7c16f06d`
- metrics: 260,478 bytes / 2,229 lines / 1,483 max line bytes

## The partition, measured against the registered part limits

Part health targets are 65,536 bytes / 768 lines each; enforcement ceilings 98,304 / 896. Boundaries are the
file's OWN section and node boundaries, so no line is rewritten and every region is a contiguous slice.

| part_id | lines | bytes | reader concern |
| --- | --- | ---: | --- |
| `program-and-bar` | 1–117 | 10,574 | why the tree exists, the ISF-complete bar, the baseline, the node index |
| `agent-identity-prose-class` | 118–552 | 47,444 | `.1c.i` — agent-identity precision on dense prose |
| `isf-lowering-fidelity` | 553–774 | 29,150 | `.2a.iv`/`.2a.v`/`.2a.vi` — width, direction, rule-value gates |
| `enum-surface-early-leaves` | 775–1068 | 29,909 | `.5.i`–`.5.v` — the generic-`TABLE` conflation and its first gates |
| `enum-surface-recent-leaves` | 1069–1633 | 45,172 | `.5.vi`–`.5.xiii` — identity, value provenance, the open leaves |
| `enum-acceptance-checklists` | 1634–1747 | 47,450 | the enforced acceptance records for the `.5` family |
| `changelog` | 1748–2229 | 47,365 | dated chronology |

Largest part is 47,450 bytes (72.4% of the part health target) and 565 lines (73.6%), so the migration lands
every part inside health with room for the next several slices, and `enum-surface-recent-leaves` — the part
that will grow — is the one with the most headroom by line count.

## Task Tree

- ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT` · Status: `done` · Children: `.0`, `.1`, `.2`, `.3`
- ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT.0` · Status: `done` (`2026-09-21`; MEASUREMENT + PLAN) · Goal: pin the
  source identity before it moves, measure the pressure against the registered ceilings, and fix the partition
  on the file's own boundaries. **DONE** — the tables above.
  Verification: the metrics re-derive from `docs/tasks/KG-ISF-COMPLETENESS.md` at `0d828e7e`; the part limits
  re-read from `doctrine/live_document_size/surfaces.jsonl`; the precedent re-reads from
  `doctrine/live_document_size/extraction_quality_gauge_task_evidence.json`'s `migration_metadata`.
  Commit: `KG-ISF-TASK-EVIDENCE-CONTAINMENT.0 — one slice of headroom left, and the partition that fixes it`
- ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT.1` · Status: `done` (`2026-09-21`; CODE/DATA) · Goal: author
  `doctrine/live_document_size/kg_isf_completeness_task_evidence.json` in `source_locked`/`complete` state from
  the partition above, author the bounded root template, and run the transaction
  `perl scripts/check_active_task_evidence.pl --contract <path> --migrate <root-template>`. The materializer is
  atomic and **rolls back on any validation failure**, restoring the source and the contract, so a failed
  attempt leaves the tree exactly as it is now.
  Then register the three surfaces (`kg_isf_task_evidence_index` / `_parts` / `_archive`) in
  `doctrine/live_document_size/surfaces.jsonl`, add the contract to `scripts/check_task_evidence_contracts.sh`,
  and route the catalogs.
  Non-goal: changing any KG-ISF content while moving it.
  Prerequisite: `.0`. Verification: `check_active_task_evidence.pl --check` green; the archived capsule is
  byte-identical to the pinned sha256; `check_doctrines.sh` green; every `[[...]]`, catalog and book link that
  names `docs/tasks/KG-ISF-COMPLETENESS.md` still resolves.

- ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT.2` · Status: `done` (`2026-09-22`; CODE/DATA; opened and closed
  by the measurement below) · Goal: **separate the sealed pre-migration record from the stream that accretes,
  so the active part stops carrying 567 lines it can never change.**

  **Why now — measured at `1c320423`.** `.1` landed `enum-surface-recent-leaves` at 565 lines / 45,172
  bytes and called it *"the part that will grow ... the one with the most headroom by line count"*. One
  ordinary slice (`KG-ISF-COMPLETENESS.5.xi`) then cost **+122 lines / +10,416 bytes**, taking it to
  **687 lines (89.5% of the 768-line part health target) / 55,588 bytes (84.8%)**. `LIVE-DOC-SIZE`
  reports both as warnings today and the line dimension crosses the 90% rollover milestone on the next
  ordinary slice. `.0`'s own argument applies unchanged one level down: *compacting evidence to fit is
  the outcome this tree exists to avoid.*

  **The line dimension binds this time, and the cut follows from that.** 567 of the 687 lines are the
  SEALED region — bytes 1069-1633 of the capsule, immutable by contract, which no slice can ever shrink
  or grow. The remaining 120 are post-migration records, and they are the whole of the growth. Splitting
  on that boundary is not a judgement about reader concern; it is the one structural fact in the file:

  | destination | lines | bytes | after |
  | --- | ---: | ---: | --- |
  | new `legacy` part holding the sealed region | 572 | 45,752 | 74.5% / 69.8% — **frozen, cannot grow** |
  | `enum-surface-recent-leaves`, still `active` | 120 | 9,949 | 15.6% / 15.2% — about 4 slices |

  **The active part keeps its path and id**, because the contract's `current_frontier`, `MEMORY.md` and
  this tree's own frontier all name it as the write destination; moving the stable name would invalidate
  every one of those pointers to buy nothing.

  **One route must be reclassified, and the mechanism says which way.** `.5.xi` is the first leaf declared
  in BOTH strata — `open` in the sealed region, `done` in a post-migration record. `validate_route_lifecycles`
  re-derives a route's lifecycle from its primary part and prefers the post-migration stratum *"when it
  exists"*, so once the strata are in different parts the route must name the part that holds the
  authoritative declaration. Its origin therefore becomes `post_migration` and it drops `source_literal`,
  which is required only for `legacy`/`structural`. This is the first exercise of a rule the partition
  makes reachable: **a leaf declared in two strata routes to the authoritative one, not to the older one.**
  Non-goal: renaming, renumbering or rewriting any sealed byte; changing the capsule; changing region line
  ranges; implementing any `KG-ISF-COMPLETENESS` leaf.
  Prerequisite: `.1`. Verification: `perl scripts/check_active_task_evidence.pl --contract
  doctrine/live_document_size/kg_isf_completeness_task_evidence.json --check` green with **no part-collection
  warning**; the moved region's `sha256` re-derives byte-identically from the capsule; every leaf id keeps a
  route; `check_doctrines.sh` green.

  **LANDED `2026-09-22`.** `enum-surface-recent-leaves.md` **687 -> 118 lines / 55,588 -> 9,947 bytes**
  (89.5% -> 15.4% of the line health target, 84.8% -> 15.2% of the byte target); the sealed region moved
  verbatim to `enum-surface-sealed-contracts.md` at 572 lines / 45,764 bytes. **The moved payload's
  sha256 is `5a54a054…2cdc265`, the digest the manifest already carried for it**, so the transfer is
  byte-identical and the capsule is untouched. `--check` is green with **no part-collection warning** for
  the first time since the containment migration. 10 sealed routes follow the region; `.5.xi` becomes
  `post_migration` on the active part, as the goal predicted.

- ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT.3` · Status: `done` (`2026-09-22`; CODE/DATA; opened and closed
  as a BLOCKER discovered while landing `KG-ISF-COMPLETENESS.5.xiv.a`) · Goal: **`.2`'s lesson has a
  second instance, and it blocked an ordinary slice exactly as `.0` predicted.**

  `.5.xiv.a`'s enforced acceptance checklist took `enum-acceptance-checklists.md` to **61,445 of the
  65,536-byte part health target — 93.8%**, which is `mandatory rollover`, a hard refusal rather than a
  warning: `check_active_task_evidence.pl` returned *"semantic part collection bytes_each is at or
  above mandatory rollover"*. That is `.0`'s own failure mode reached one level down — *"an ordinary
  slice was refused at the ceiling and had to compact its own evidence to land"* — and compacting a
  Rust change's acceptance evidence to fit a byte budget is exactly what must not happen.

  **The growable fraction is 17%.** Measured before cutting: **50,627 of the 61,445 bytes are the
  SEALED region**, immutable by contract; only 10,705 are post-migration checklists, and they are the
  whole of the growth. This is the identical shape `.2` measured for `enum-surface-recent-leaves`
  (567 of 687 lines sealed), which is what makes it a RULE rather than an anecdote: **after a
  root-level partition, every part that still carries sealed history plus a live stream will reach its
  ceiling on the live stream's schedule, not its own size.**

  | destination | lines | bytes | after |
  | --- | ---: | ---: | --- |
  | new `legacy` part holding the sealed region | 121 | 50,740 | 77.4% — **frozen, cannot grow** |
  | `enum-acceptance-checklists`, now `active` | 26 | 10,818 | 16.5% |

  **The part becomes `active` rather than staying `legacy`**, because a `legacy` part must own at least
  one source region and this one now owns none. Nothing requires exactly one `active` part; the
  contract requires only that the FRONTIER route to one, which stays `enum-surface-recent-leaves`.
  Non-goal: rewriting any sealed byte; changing region line ranges; the remaining five parts, which
  carry no live stream and are therefore not on this trajectory.
  Prerequisite: `.2`. Verification: `--check` green with no rollover violation; the moved payload's
  sha256 re-derives byte-identically from the capsule; every leaf id keeps a route.

## Landed `2026-09-21`

**`docs/tasks/KG-ISF-COMPLETENESS.md`: 260,478 -> 7,423 bytes** (93.5% -> 2.7% of its ceiling), 2,229 -> 132
lines. The capsule `docs/archive/tasks/kg-isf-completeness/source-through-2026-09-21.md` is
**byte-identical to the pinned source** (`b7defaee…`), so nothing was summarized, renumbered or lost.

| destination | result |
| --- | --- |
| bounded root | 132 lines / 7,423 bytes — 51.6% of its line health target, 30.2% of its byte target |
| 7 semantic parts | largest 50,740 bytes (77.4% of the 65,536 part target) |
| index + route catalog | `kg-isf-completeness/INDEX.md`, `routes-0001.md` — all 41 leaves, 0 unverified |
| archive capsule | exact pre-migration bytes, `archive_terminal` |

**Two defects the transaction caught, both by refusing to run.**
1. The source must be COMMITTED at the pinned boundary. The `.5.xiii` placement fix was in the working
   tree, so the contract's `git_blob` disagreed with `HEAD` and the preflight refused. The fix is its own
   commit (`0e893e66`), which is now the pinned boundary.
2. **`lifecycle` is not "not open"**. The checker's rule is `closed` iff the node's status is `done` or
   `superseded`, so this tree's `active` (6) and `deferred` (3) nodes are OPEN. Deriving the lifecycle as
   "anything that is not literally `open`" mislabeled 9 of 41 routes, and the migration rolled back naming
   every one of them. **The rollback was verified, not assumed**: after it, the source sha256 still
   matched the pin, the collection directory did not exist, and the contract was still `source_locked`.

Six nodes carry `origin: structural` rather than `legacy` — `KG-ISF-COMPLETENESS` itself, the containers
`.1b` and `.2a`, and the four leaves opened on `2026-09-21` (`.5.x`-`.5.xiii`). `route_basis` is
`boundary_path_commit_subject_ids`, so a `legacy` route must appear in a commit subject touching the path,
and a node that has never been a commit subject cannot. That is the precedent's own classification.

## Current Frontier

None. The tree is closed again. The KG-ISF write destination is unchanged —
`docs/tasks/kg-isf-completeness/enum-surface-recent-leaves.md`, the one part marked `active` — and after `.2`
it holds only post-migration records, at 15.4% of its line health target.

## Changelog

- `2026-09-22`: **`.3` — the same defect, one part over, and this time it BLOCKED a slice.**
  `enum-acceptance-checklists.md` hit 93.8% (mandatory rollover) when `.5.xiv.a`'s enforced checklist
  landed; 50,627 of its 61,445 bytes were the sealed region. Split on the same boundary as `.2`:
  sealed part frozen at 50,740 bytes, the accreting part down to 16.5% and promoted to `active`
  because a region-less part may not be `legacy`. **Two instances make it a rule: after a root
  partition, any part carrying sealed history plus a live stream reaches its ceiling on the live
  stream's schedule.**

- `2026-09-22`: **`.2` — the part `.1` called "the one with the most headroom" reached 89.5% in a single
  slice.** 567 of its 687 lines were the SEALED region, which no slice can shrink; the other 120 were the
  whole of the growth. Split on that boundary — the one structural fact in the file — not on a judgement
  about reader concern. Active part 687 -> 118 lines; new `legacy` part frozen at 572. Moved payload digest
  matches the manifest's existing region digest exactly. **Lesson: a partition that leaves immutable bytes
  in the accreting part has not bounded it, it has only delayed it** — `.1`'s own headroom table measured
  the part, where it needed to measure the part's GROWABLE fraction.

- `2026-09-21`: **`.1` — migrated, losslessly, and the transaction refused twice before it ran.** Root
  260,478 -> 7,423 bytes; capsule byte-identical to the pin. The two refusals are the durable lessons: the
  pinned source must be COMMITTED, and `lifecycle` is `closed` only for `done`/`superseded`, so `active`
  and `deferred` nodes are open. Registered 4 surfaces, the contract in the driver, and its encoder.

- `2026-09-21`: **`.0` — the file has ONE slice of headroom, and that is the whole argument.** 260,478 of
  278,528 bytes (93.5%), against two `2026-09-21` slices that cost 12,269 and 13,148 bytes. The byte dimension
  binds; lines are only at 74.3%. Partition fixed on the file's own section boundaries into 7 parts, largest
  47,450 bytes (72.4% of the part health target). `EXTRACTION-QUALITY-GAUGE` is the precedent and its contract
  states the failure mode being avoided: a slice refused at the ceiling that then had to compact its own
  evidence to land.
