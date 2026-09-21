# KG-ISF-TASK-EVIDENCE-CONTAINMENT: bound the KG-ISF task evidence before it refuses its own next slice

## Metadata

- Tree ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT`
- Status: `active`
- Roadmap lane: `R15`/`R16` continuity prerequisite for `KG-ISF-COMPLETENESS.5.xi` and every later leaf
- Created: `2026-09-21`
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

- ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT` · Status: `active` · Children: `.0`, `.1`
- ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT.0` · Status: `done` (`2026-09-21`; MEASUREMENT + PLAN) · Goal: pin the
  source identity before it moves, measure the pressure against the registered ceilings, and fix the partition
  on the file's own boundaries. **DONE** — the tables above.
  Verification: the metrics re-derive from `docs/tasks/KG-ISF-COMPLETENESS.md` at `0d828e7e`; the part limits
  re-read from `doctrine/live_document_size/surfaces.jsonl`; the precedent re-reads from
  `doctrine/live_document_size/extraction_quality_gauge_task_evidence.json`'s `migration_metadata`.
  Commit: `KG-ISF-TASK-EVIDENCE-CONTAINMENT.0 — one slice of headroom left, and the partition that fixes it`
- ID: `KG-ISF-TASK-EVIDENCE-CONTAINMENT.1` · Status: `open` (`2026-09-21`; CODE/DATA) · Goal: author
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

## Current Frontier

`.1` — author the contract and run the migration transaction. The source is pinned and unmodified; nothing is
in flight.

## Changelog

- `2026-09-21`: **`.0` — the file has ONE slice of headroom, and that is the whole argument.** 260,478 of
  278,528 bytes (93.5%), against two `2026-09-21` slices that cost 12,269 and 13,148 bytes. The byte dimension
  binds; lines are only at 74.3%. Partition fixed on the file's own section boundaries into 7 parts, largest
  47,450 bytes (72.4% of the part health target). `EXTRACTION-QUALITY-GAUGE` is the precedent and its contract
  states the failure mode being avoided: a slice refused at the ceiling that then had to compact its own
  evidence to land.
