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
  Status: `done` (`2026-08-10`, DOC/MEASURE/DESIGN)
  Goal: measure source roles, current-state contradictions, readers, writers, routes, growth, and candidate topologies
  Acceptance: a task-owned census partitions every source byte, inventories all consumers and write obligations,
  reconciles current authority from durable evidence, and selects or rejects each topology with measured reasons
  Verification: seven-region reconstruction, route/id/reader census, target identity, Knowledge Map, task
  catalog, doctrines, and mdBook checks pass
  Commit: `CORPUS-TASK-EVIDENCE-CONTAINMENT.2 — select measured corpus task partition`

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.3`
  Status: `done` (`2026-08-10`, CODE/DOC/CONTRACT)
  Goal: lock the corpus-specific migration contract and neutral verifier before moving source bytes
  Children: `CORPUS-TASK-EVIDENCE-CONTAINMENT.3a`, `CORPUS-TASK-EVIDENCE-CONTAINMENT.3b`
  Verification: both children pass; the complete corpus contract is invoked unconditionally while source and
  declared destinations remain respectively byte-identical and absent

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.3a`
  Status: `done` (`2026-08-10`, CODE/DOCTRINE)
  Goal: let the neutral active-task checker preserve exact wide legacy rows and formal container routes
  Acceptance: the portable active-part cap permits at most the existing 6,400-byte task-evidence line ceiling;
  a source-backed `structural` origin covers formal ids absent from commit subjects; focused positive/fail-closed
  cases pass; the corpus source and all destinations remain untouched
  Verification: Perl syntax; 39/39 focused cases; existing active-task contract; live-document gate; source
  identity/destination absence; task catalog; Knowledge Map; doctrines; mdBook test/build; full CI pass
  Commit: `CORPUS-TASK-EVIDENCE-CONTAINMENT.3a — support exact structural task routes`

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.3b`
  Status: `done` (`2026-08-10`, CODE/DOC/CONTRACT)
  Goal: commit the complete corpus-specific migration contract and unconditional verifier invocation
  Acceptance: decision record, source lock, seven regions, 41 legacy plus seven structural routes, bounds, writer
  transaction, destination absence, rollback, and future-append proof validate with the source byte-identical
  Verification: JSON/shell syntax; 7-region / 48-route / 41-legacy / 7-structural census; source-locked report;
  39/39 neutral cases; unconditional live-document driver; source identity/destination absence; task catalog;
  Knowledge Map; doctrines; mdBook test/build; full CI pass
  Commit: `CORPUS-TASK-EVIDENCE-CONTAINMENT.3b — lock corpus task migration contract`

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.4`
  Status: `active`
  Goal: migrate atomically, verify losslessness and bounded continuity, close containment, and return to refresh #49
  Children: `CORPUS-TASK-EVIDENCE-CONTAINMENT.4a`, `CORPUS-TASK-EVIDENCE-CONTAINMENT.4b`

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.4a`
  Status: `done` (`2026-08-10`, DATA/DOC/MIGRATION)
  Goal: invoke and commit the guarded root-last migration with every resulting surface registered
  Acceptance: bounded root, seven exact semantic parts, 48-route index, manifest, and exact capsule pass the
  corpus contract and composed live-document gate; source reconstructs exactly; no writer/template residue remains
  Verification: 65-line root; 75-line index; seven-part 2,357-line collection; 488-line manifest; exact
  2,308-line capsule; independent seven-region reconstruction and 48-route-set proof; no residue; 51 governed
  surfaces; focused contract; task catalog; Knowledge Map; roadmap/book; doctrines; mdBook test/build; full CI pass
  Commit: `CORPUS-TASK-EVIDENCE-CONTAINMENT.4a — migrate corpus task evidence losslessly`

- ID: `CORPUS-TASK-EVIDENCE-CONTAINMENT.4b`
  Status: `pending`
  Goal: independently reproduce the committed migration and prove bounded future continuation before closure
  Acceptance: a same-volume no-hardlink clean clone verifies Git/capsule/source/region/route/manifest identity,
  all doctrines, and one temporary post-migration active-part/eligible-frontier append with exact cleanup
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CORPUS-TASK-EVIDENCE-CONTAINMENT.4b` | `pending` | Reproduce the committed migration and prove one real bounded continuation before closure. |

## Locked Source Boundary

The target remains byte-identical to committed `d78d842e4e7fc8d1fd902937cf6902f07d96b68e`:

- Path: `docs/tasks/CORPUS-COVERAGE.md`
- Git blob: `d7ac9aa2c07723cb4a7a8f3a15332ef84fe09f07`
- SHA-256: `5d7acb0c973a75d821c9d5a963aac5c3899da19261dcf6427274cb123804123f`
- Metrics: 2,308 lines / 277,636 bytes / 4,746 maximum content-line bytes (line 1,722)
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
- `2026-08-10`: `.2` corrected `.1`'s width notation from 4,747 bytes including LF to the doctrine-defined 4,746
  content-line bytes excluding LF/optional CR. The locked commit/blob/SHA/line/total-byte identity is unchanged.
- `2026-08-10`: `.2` selects a bounded active root, seven exact semantic legacy parts, a 48-route index/manifest,
  and the exact source capsule. The seven measured regions total 2,308 lines / 277,636 bytes exactly; no route
  alias is needed. A new top-level continuation, archive-only authority, and chronological-only topology are
  rejected because they respectively break `.2.<refresh>` identity, demote browsable task authority, or lose task
  hierarchy. Corpus-specific bounds and the root+one-part+index+manifest writer transaction are recorded in
  `docs/research/corpus-task-evidence-containment-census.md`.
- `2026-08-10`: `.3` split before implementation after the real source-locked check found two neutral-contract
  blockers: exact corpus ledger rows require 4,746 content-line bytes while the verifier capped parts at 1,024,
  and seven formal container ids lack completion-subject history. `.3a` owns a bounded generic 6,400-byte cap and
  source-backed `structural` route origin; `.3b` alone may lock/invoke the corpus contract afterward.
- `2026-08-10`: `.4` split before its first commit so the actual migration (`.4a`) becomes a durable clean
  boundary before `.4b` independently clones it and proves exact recovery plus a real bounded continuation.
- `2026-08-10`: `.4a` final diff hygiene caught a terminal scaffold blank line on every generated semantic part.
  The generic writer now removes only that final separator and its positive self-test rejects recurrence; source
  payload bytes and capsule identity remain unchanged, while part hashes/metrics were resealed before final gates.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-10` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.1` | source commit/blob/SHA/metrics; target diff; candidate destination absence; `perl scripts/check_task_tree_catalog.pl --check`; `scripts/check_doctrines.sh`; `mdbook test docs/book`; `mdbook build docs/book` | pass |
| `2026-08-10` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.2` | seven raw regions/source reconstruction; 42 exact-path readers at `3a40b152`; 48 formal / 41 subject / 48 union ids; growth census; target diff; Knowledge Map derive-and-diff; task catalog; doctrines; mdBook test/build | pass; selected bounded root + seven parts + exact capsule |
| `2026-08-10` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.3a` | Perl syntax; 39/39 neutral cases; existing real contract; exact source/destination absence; live-document gate; task/Knowledge Map/doctrine gates; mdBook test/build; full CI | pass; 6,400 cap and source-backed structural routes fail closed |
| `2026-08-10` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.3b` | JSON/shell syntax; real 7-region / 48-route source-locked report; 39/39 neutral cases; unconditional driver; exact source/destination absence; task/Knowledge Map/doctrine gates; mdBook test/build; full CI | pass; complete corpus migration contract locked before mutation |
| `2026-08-10` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.4a` | guarded root-last writer; migrated report; independent source reconstruction/route-set proof; template/transaction residue; 51 live surfaces; task/Knowledge Map/doctrine gates; mdBook test/build; full CI | pass; exact source retained and bounded current/task authorities installed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT.1` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.1 — lock untouched corpus task source` | Exact untouched-source ownership boundary. |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT.2` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.2 — select measured corpus task partition` | Complete census and corpus-specific topology/bounds. |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT.3a` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.3a — support exact structural task routes` | Neutral checker supports bounded exact-wide rows and formal container routes. |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT.3b` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.3b — lock corpus task migration contract` | Complete source lock and unconditional verifier invocation. |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT.4a` | `CORPUS-TASK-EVIDENCE-CONTAINMENT.4a — migrate corpus task evidence losslessly` | Guarded root-last migration and resulting-surface activation. |

## Changelog

- `2026-08-10`: Created the corpus-specific containment tree and opened `.1` as the only executable frontier.
- `2026-08-10`: `.1` pinned the untouched committed source and advanced the frontier to the corpus-specific census.
- `2026-08-10`: `.2` corrected width semantics, accounted for every source byte/route/reader/writer, selected the
  measured hybrid, and advanced the frontier to the locked migration contract.
- `2026-08-10`: `.3a` aligned the neutral part-width cap with the existing direct task-evidence ceiling, added
  source-backed `structural` container routes, and advanced the frontier to the corpus-specific contract.
- `2026-08-10`: `.3b` locked every source/region/route/bound/writer input, invoked the independent corpus
  contract unconditionally, and advanced the frontier to the guarded migration.
- `2026-08-10`: `.4a` installed the bounded root, seven semantic parts, route index/manifest, and exact capsule;
  independent reconstruction is exact and the frontier advances to clean-clone/continuation proof.

### Acceptance Checklist (enforced) — `CORPUS-TASK-EVIDENCE-CONTAINMENT.2`

- [x] **REPRODUCE / MEASURE** — seven raw regions reconstruct the exact 2,308-line / 277,636-byte source and
  locked SHA; reader, id, history, current-state, and recent-growth censuses are explicit.
- [x] **ROOT CAUSE (WHY + WHERE)** — recent minimum growth (1,215 bytes) exceeds 892 bytes of headroom; the source
  mixes active current authority, semantic leaf evidence, a dense result table, and chronology in one append path.
- [x] **ADDRESSED (verified)** — selected seven exact semantic parts plus a bounded root/index/manifest and capsule;
  corpus-derived bounds and the future atomic writer transaction cover all 48 formal routes.
- [x] **NO REGRESSION** — the target remains byte-identical to `d78d842e`; Knowledge Map, task catalog, doctrines,
  and mdBook test/build pass; no product artifact, code, schema, CLI, source PDF, or public behavior changed.
- [x] **GENERICITY** — the existing neutral active-task checker is reused through a separate data contract; no
  corpus/document/vendor rule or copied first-target threshold enters executable policy.
- [x] **LOCKSTEP** — research census, durable fact card, owning task, change history, roadmap, and resume pointer
  agree that `.3` must lock the complete contract before `.4` may mutate the source.

### Acceptance Checklist (enforced) — `CORPUS-TASK-EVIDENCE-CONTAINMENT.3a`

- [x] **REPRODUCE / MEASURE** — the exact corpus ledger needs 4,746 content-line bytes and seven formal container
  ids are absent from the 41-ID completion-subject boundary; both measured blockers reproduce before change.
- [x] **ROOT CAUSE (WHY + WHERE)** — the neutral checker capped nested part lines at 1,024 bytes and modeled only
  completion-subject `legacy` or future `post_migration` routes, so neither exact source shape could validate.
- [x] **ADDRESSED (verified)** — part ceilings may reach but not exceed the existing 6,400-byte direct
  task-evidence cap; source-backed `structural` routes require a full/tree-relative literal in their primary part.
- [x] **NO REGRESSION** — 39/39 focused cases, the existing migrated active-task contract, live-document gate,
  task/Knowledge Map/doctrine gates, mdBook test/build, and full CI pass; the corpus source/destinations are untouched.
- [x] **GENERICITY** — policy is expressed only through task identity, source literals, origin class, and the
  repository-wide evidence cap; no corpus, document, vendor, refresh, or path-specific rule enters the checker.
- [x] **LOCKSTEP** — ADR 0024, census, fact card, mdBook doctrine reference, task tree, changes, and resume pointer
  agree that `.3b` must lock 41 legacy plus seven structural corpus routes before migration.

### Acceptance Checklist (enforced) — `CORPUS-TASK-EVIDENCE-CONTAINMENT.3b`

- [x] **REPRODUCE / MEASURE** — the real source-locked report reproduces exact 2,308/277,636/4,746 source
  metrics and the seven planned payload metrics, with 48 formal routes split 41 legacy / seven structural.
- [x] **ROOT CAUSE (WHY + WHERE)** — the old monolith has 892 bytes of headroom and no independent contract;
  the new data file binds its exact Git/source authority, semantic cut, routes, bounds, and root-last writer inputs.
- [x] **ADDRESSED (verified)** — the complete schema-closed contract authenticates every region/route and declares
  root/index/part/capsule limits, migration metadata, future root requirements, and destination absence.
- [x] **NO REGRESSION** — JSON/shell syntax, 39/39 neutral cases, both real active-task contracts, the unconditional
  live-document driver, task/Knowledge Map/doctrine gates, mdBook test/build, and full CI pass; source is untouched.
- [x] **GENERICITY** — corpus identity, measurements, regions, and policy live only in the separate data contract;
  the shared checker and shell driver contain no corpus document/vendor/refresh parsing or special-case behavior.
- [x] **LOCKSTEP** — ADR 0024, census, fact, task tree, changes, mdBook, and resume pointer agree that only `.4`
  may invoke the reviewed root-last writer and refresh #49 remains a later separately owned product slice.

### Acceptance Checklist (enforced) — `CORPUS-TASK-EVIDENCE-CONTAINMENT.4a`

- [x] **REPRODUCE / MEASURE** — resulting metrics are root 65/2,822/120, index 75/4,349/93, manifest
  488/15,856/88, seven parts 2,357/279,157 with maxima 492/59,454/4,746, and exact capsule 2,308/277,636/4,746.
- [x] **ROOT CAUSE (WHY + WHERE)** — the 277,636-byte mixed-role root had only 892 bytes left; the accepted
  partition moves literal history to exact semantic payloads/capsule while retaining a bounded truthful root.
- [x] **ADDRESSED (verified)** — the guarded writer installed capsule → parts → index → manifest → migrated
  contract → root, then validated the complete tree; three dedicated surfaces govern the new Markdown authorities.
- [x] **NO REGRESSION** — an independent raw marker reconstruction equals the capsule/source SHA and exact 48-id
  set; focused contract, 51-surface live gate, task/Knowledge Map/doctrine gates, mdBook, and full CI pass.
- [x] **GENERICITY** — migration used only the neutral contract-driven writer and corpus data contract; no task
  payload was reworded, no limit widened, and no document/vendor/product special case entered executable code.
- [x] **LOCKSTEP** — bounded root, semantic parts, exact capsule, contract, surfaces, roadmap, fact, task, changes,
  mdBook, and memory agree; `.4b` alone owns the clean-clone/future-continuation proof before closure.
