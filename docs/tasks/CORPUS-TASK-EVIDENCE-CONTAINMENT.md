# CORPUS-TASK-EVIDENCE-CONTAINMENT: keep the active corpus task bounded and lossless

## Metadata

- Tree ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT`
- Status: `active`
- Roadmap lane: `R15e`/`R16` continuity prerequisite for `CORPUS-COVERAGE.2`
- Created: `2026-08-10`
- Last updated: `2026-08-10`
- Owner: repo-local workflow

## Goal

Losslessly partition the active `CORPUS-COVERAGE` task evidence before its next product leaf would exceed the
registered live-document ceiling, while preserving stable identity, truthful current state, complete historical
evidence, bounded retrieval, and an unambiguous write transaction for refresh #49 and later work.

## Non-Goals

- Do not implement, select, or claim corpus refresh #49 inside this containment tree.
- Do not trim, summarize away, rewrite, or renumber established corpus task evidence.
- Do not widen live-document thresholds or exempt the source from containment.
- Do not copy the `PDF-VARIANT-DIGESTION` topology or limits without corpus-specific measurement.
- Do not change generated corpus artifacts, extraction behavior, public CLI/schema behavior, or source PDFs.

## Acceptance Criteria

- The committed pre-containment source identity is pinned before the source changes.
- Every source byte and established leaf id remains durably retrievable from the stable task route in bounded hops.
- Current corpus state, frontier, blockers, and the next write destination are explicit and mechanically checked.
- Every new live surface has measured health targets, enforcement ceilings, index coverage, and a rollover/closure
  rule derived from the corpus task's actual shape and growth.
- Migration is atomic, repository-relative, same-volume, reversible on failure, and independently verifies exact
  provenance plus the canonical partition.
- Focused containment checks, the composed doctrine driver, mdBook checks, and risk-proportionate broader gates pass.
- The closing leaf updates the topically correct mdBook implementation-and-verification subsection and completes
  the `COMMIT.md` workflow.

## Task Tree

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT`
  Status: `active`
  Goal: keep the active corpus task bounded, lossless, truthful, and ready for refresh #49
  Children: `CORPUS-TASK-EVIDENCE-CONTAINMENT.1`, `CORPUS-TASK-EVIDENCE-CONTAINMENT.2`,
  `CORPUS-TASK-EVIDENCE-CONTAINMENT.3`, `CORPUS-TASK-EVIDENCE-CONTAINMENT.4`

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.1`
  Status: `done` (`2026-08-10`, DOC/BOUNDARY)
  Goal: own and pin the untouched committed source boundary before any target mutation
  Acceptance: record the exact commit, Git blob, SHA-256, line/byte/width metrics, and prove the working source is
  byte-identical to that boundary while all containment destinations remain absent
  Verification: exact source identity, zero target diff, destination-absence census, task catalog, doctrines,
  and mdBook checks pass
  Commit: `CORPUS-TASK-EVIDENCE-CONTAINMENT.1 — lock untouched corpus task source`

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.2`
  Status: `pending`
  Goal: measure source roles, current-state contradictions, readers, writers, routes, growth, and candidate topologies
  Acceptance: a task-owned census partitions every source byte, inventories all consumers and write obligations,
  reconciles current authority from durable evidence, and selects or rejects each topology with measured reasons
  Verification: pending
  Commit: pending

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.3`
  Status: `pending`
  Goal: lock the corpus-specific migration contract and neutral verifier before moving source bytes
  Acceptance: committed contract, decision record, source lock, destination bounds, route map, writer transaction,
  rollback behavior, and positive future-append proof all validate while the source remains byte-identical
  Verification: pending
  Commit: pending

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.4`
  Status: `pending`
  Goal: migrate atomically, verify losslessness and bounded continuity, close containment, and return to refresh #49
  Acceptance: the stable root and every declared destination pass focused and composed gates; exact provenance and
  route coverage hold; no residue remains; live docs and mdBook are synchronized; the next action is refresh #49
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CORPUS-TASK-EVIDENCE-CONTAINMENT.2` | `pending` | Measure this corpus task before choosing a topology or limit. |

## Locked Source Boundary

The target remains byte-identical to committed `d78d842e4e7fc8d1fd902937cf6902f07d96b68e`:

- Path: `docs/tasks/CORPUS-COVERAGE.md`
- Git blob: `d7ac9aa2c07723cb4a7a8f3a15332ef84fe09f07`
- SHA-256: `5d7acb0c973a75d821c9d5a963aac5c3899da19261dcf6427274cb123804123f`
- Metrics: 2,308 lines / 277,636 bytes / 4,747 maximum line bytes (line 1,722)
- History: 94 path-touching commits through the boundary
- Registered byte ceiling: 278,528 bytes; exact remaining headroom: 892 bytes

At this boundary no corpus-specific nested task collection, task archive, or focused containment contract exists.
The candidate paths `docs/tasks/corpus-coverage/`, `docs/archive/tasks/corpus-coverage/`, and
`doctrine/live_document_size/corpus_task_evidence.json` are absent. Their names reserve no topology; `.2` must
measure and decide the actual destinations. `git diff -- docs/tasks/CORPUS-COVERAGE.md` is empty.

## Decisions

- `2026-08-10`: Open a corpus-specific containment tree. The completed
  `ACTIVE-TASK-EVIDENCE-CONTAINMENT` tree must not be reopened, and ADR 0019 permits another active task to adopt
  its architecture only through a separately measured contract.
- `2026-08-10`: Keep `docs/tasks/CORPUS-COVERAGE.md` byte-identical throughout `.1`; diagnostics may read it, but
  no partition or current-state rewrite is authorized until the measured design and locked migration leaves.

## Open Questions

- Can the corpus task use a smaller continuation boundary than the hybrid active-root architecture, while keeping
  all established ids and the active `.2` parent truthful? `.2` must answer from the measured source and consumers.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-10` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.1` | source commit/blob/SHA/metrics; target diff; candidate destination absence; `perl scripts/check_task_tree_catalog.pl --check`; `scripts/check_doctrines.sh`; `mdbook test docs/book`; `mdbook build docs/book` | pass |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT.1` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.1 — lock untouched corpus task source` | Exact untouched-source ownership boundary. |

## Changelog

- `2026-08-10`: Created the corpus-specific containment tree and opened `.1` as the only executable frontier.
- `2026-08-10`: `.1` pinned the untouched committed source and advanced the frontier to the corpus-specific census.
