# LIVE-DOCUMENT-PRESSURE-HEADROOM: keep current-facing canonical surfaces writable

## Metadata

- Tree ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM`
- Status: `active` (tracking-only while `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT` closes)
- Roadmap lane: repository durability and portability
- Created: `2026-08-14`
- Last updated: `2026-08-14`
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
  Children: `.0`, `.1`, `.2`, `.3`, `.4`

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
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.0` | `done` | exact clean pressure and owner boundaries are pinned |
| 2 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.1` | `pending` | one line remains before the next current structural fact is refused |
| 3 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.2` | `pending` | the task plane is already at its 90% file milestone |

## Decisions

- `2026-08-14`: open one pressure-frontier tree rather than one task per warning. The surfaces need distinct
  remediation transactions, but one bounded owner can preserve the exact measured ordering without consuming
  several more task-file slots at the already-triggered collection milestone.
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
