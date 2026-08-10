# FACT-CARD-CATALOG-CONTAINMENT: keep fact-card browsing bounded before capacity fails

> **Bounds superseded (`2026-08-11`).** This tree's recorded numbers — the 198-card maximum, the 200-file
> collection ceiling, the four-part projection, and the 200-fact question authority — were correct when it
> closed and are preserved here as history. The fact plane's current capacity is
> [ADR 0029](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)'s
> derived profile: six 56-card title parts, 336 cards, 338 collection files, 44 decision records, and 379
> facts, delivered by [`FACT-CARD-CAPACITY-HEADROOM.3`](FACT-CARD-CAPACITY-HEADROOM.md). This tree's own
> conclusions about the *browse* plane are unchanged.

## Metadata

- Tree ID: `FACT-CARD-CATALOG-CONTAINMENT`
- Status: `done`
- Roadmap lane: process / continuity / durable knowledge retrieval
- Created: `2026-08-09`
- Closed: `2026-08-09`
- Owner: knowledge maintainers through the repo-local workflow

## Goal

Restore coherent, bounded growth capacity for the derived fact-card browse route before its independent file-count
or index-byte contract rejects ordinary durable knowledge capture. Preserve every canonical fact and keep question
retrieval, direct id/title browsing, derive-and-diff freshness, and repository-volume locality intact.

## Non-Goals

- Do not delete, merge, trim, or rewrite canonical fact evidence to manufacture headroom.
- Do not widen an existing health target, warning milestone, rollover milestone, or enforcement ceiling.
- Do not change Knowledge Map question-shard semantics or create a second source of canonical fact truth.
- Do not change SpecForge product behavior, generated product artifacts, or shared toolchain/cache inputs.

## Acceptance Criteria

- The current card, collection, generated-index, and applicable live-surface identities and limits are measured at
  an exact committed boundary before implementation.
- A durable decision defines a bounded catalog topology with complete deterministic membership, direct retrieval,
  fixed per-file and aggregate limits, fail-closed residue handling, and no canonical-content loss.
- The catalog generator/checker and generic live-document registry agree on collection capacity and lifecycle.
- Every bootstrap, README, Knowledge Map, doctrine, and mdBook reader follows the resulting bounded route.
- Focused fail-closed cases, composed doctrines, the mdBook build, and risk-proportionate broader gates pass.
- Each completed leaf is committed through `COMMIT.md`, and the repository is handoff-ready between leaves.

## Task Tree

- ID: `FACT-CARD-CATALOG-CONTAINMENT`
  Status: done
  Goal: keep durable fact-card navigation bounded and able to admit governed additions
  Children: `FACT-CARD-CATALOG-CONTAINMENT.0`, `FACT-CARD-CATALOG-CONTAINMENT.1`,
  `FACT-CARD-CATALOG-CONTAINMENT.2`

- ID: `FACT-CARD-CATALOG-CONTAINMENT.0`
  Status: done
  Goal: establish ownership and pin the exact pre-containment pressure boundary
  Acceptance: this tree exists before any catalog contract or output changes; exact independent count/byte limits,
  current metrics, reader/writer seams, and the next decision leaf are durable
  Verification: boundary commit `47e91540`; 158 cards / 160 immediate Markdown files; exact collection and
  generated-index Git identities; 32,634-byte index with 134 bytes below its focused ceiling; 13 unique current
  path/checker seams; focused catalog, Knowledge Map, live-size, task catalog, doctrines, and mdBook gates pass
  Commit: `FACT-CARD-CATALOG-CONTAINMENT.0 — own and pin fact catalog pressure`

- ID: `FACT-CARD-CATALOG-CONTAINMENT.1`
  Status: done
  Goal: select and specify the bounded fact-card catalog topology
  Acceptance: a durable decision fixes authorities, membership, routing, partition/aggregate limits, update
  transaction, migration stages, and rejection cases without changing current catalog outputs
  Verification: ADR 0020 initially accepted a stable ID landing plus 64-card title parts, derives 198 cards from
  the unchanged 200-file surface, fixes root/part/aggregate bounds and two-hop title retrieval, and separates
  legacy lock from migration; proposed current rendering at that boundary was 172 lines / 12,361 bytes plus
  three parts totaling 176 lines / 32,636 bytes; ADR 0022 later superseded only the packing count; source index
  diff remains empty; decision/catalog/KM/live-size/doctrine/book gates pass
  Commit: `FACT-CARD-CATALOG-CONTAINMENT.1 — decide the bounded fact catalog topology`

- ID: `FACT-CARD-CATALOG-CONTAINMENT.2`
  Status: done
  Goal: implement, migrate, verify, and close fact-card catalog containment
  Children: `FACT-CARD-CATALOG-CONTAINMENT.2.1`, `FACT-CARD-CATALOG-CONTAINMENT.2.2`

- ID: `FACT-CARD-CATALOG-CONTAINMENT.2.1`
  Status: done
  Goal: lock the legacy monolith and enforce both catalog states before migration
  Children: `FACT-CARD-CATALOG-CONTAINMENT.2.1.1`, `FACT-CARD-CATALOG-CONTAINMENT.2.1.2`

- ID: `FACT-CARD-CATALOG-CONTAINMENT.2.1.1`
  Status: done
  Goal: correct cross-directory row-link preservation before implementation
  Acceptance: a durable correction preserves every legacy row field and exact card destination while replacing
  only the now-relative link text required by the separate title-part directory; limits remain valid
  Verification: ADR 0021 proves literal `(card.md)` rows break after the sibling-directory move; semantic tuple
  and resolved-destination preservation replaces only that clause; current corrected parts are 176 lines /
  34,720 bytes aggregate, largest 70 lines / 14,306 bytes / max line 268; all ADR 0020 limits remain valid; source
  monolith unchanged and destination absent; decision/catalog/KM/live-size/doctrine/book gates pass
  Commit: `FACT-CARD-CATALOG-CONTAINMENT.2.1.1 — correct cross-directory fact routes`

- ID: `FACT-CARD-CATALOG-CONTAINMENT.2.1.2`
  Status: done
  Goal: lock the legacy monolith and enforce both catalog states before migration
  Acceptance: a schema-closed contract/checker pins the committed monolith, proves every source row and planned
  output, rejects premature title parts, validates the future migrated topology, and runs unconditionally
  Verification: schema-closed `legacy_locked` contract pins boundary commit `9ad08ac`, blob `dcac9e58…5081`,
  SHA-256, 172-line / 32,634-byte / 255-byte-line metrics, 158 ordered rows, and all current cards; deterministic
  56-card packing yields a 178-line / 12,390-byte landing plus three parts totaling 179 lines / 34,906 bytes,
  all below warning; 40/40 focused cases cover both states, routes, residue, authorities, the derived 198-card
  limit, and synthetic mandatory rollover; legacy `--write` leaves the source index unchanged and the destination
  absent; full verification is recorded below; ADR 0023 later superseded only this planned landing scaffold after checking
  pressure at the full card boundary
  Commit: `FACT-CARD-CATALOG-CONTAINMENT.2.1.2 — enforce the legacy-locked fact catalog`

- ID: `FACT-CARD-CATALOG-CONTAINMENT.2.2`
  Status: done
  Goal: migrate to the bounded landing/title-part projection and close containment
  Acceptance: every canonical card stays at its stable path with identity/evidence intact and is directly
  ID-routed through the landing; detailed rows occupy bounded generated parts; all readers/enforcement agree;
  stale output is absent; the owned containment fact may record the outcome; the tree closes
  Verification: migrated output matches the ADR 0023-corrected landing plus three pre-pinned part hashes and exact
  340-line / 46,890-byte aggregate; stable
  landing links 158 cards plus three title parts, whose ordered rows resolve all canonical destinations exactly;
  `fact_card_titles` is registered as the exact bounded generated-projection lifecycle; the Unicode semantic-row
  digest is raw-UTF-8 safe and covered by the non-ASCII focused fixture; the 198-card pressure boundary stays
  below mandatory rollover and 199 cards fail; 41/41 focused cases, derive-and-diff,
  catalogs/map, six doctrines, mdBook, full CI, and residue/locality gates pass
  Commit: `FACT-CARD-CATALOG-CONTAINMENT.2.2 — migrate and close bounded fact browsing`

## Current Frontier

No active frontier. The bounded catalog is migrated and this tree must not be reopened.

The direct-ID landing recorded throughout this tree is its boundary shape, not the current one.
`FACT-CARD-CAPACITY-HEADROOM.2` replaced that per-card list with a fixed-size router over the same title parts
(ADR 0026 decision 2, promoted by ADR 0027 decision 2). Every card, part, and capacity bound this tree derived is
unchanged by that reshape.

The next continuity action is `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2`: pin the final active PDF-task source and
migration inputs before its already-designed bounded evidence migration.

## Pre-Containment Boundary

The immutable measurement boundary is commit `47e915409bbe6065544f86e49739b9e93c1ac75b`. At that revision:

- `docs/knowledge/` is Git tree `1efe8aafd1beefcd1eee30e8272419fab3c7b787`: 160 immediate Markdown files,
  8,029 lines, and 728,718 bytes. It contains 158 valid canonical cards plus `README.md` and `INDEX.md`.
- `docs/knowledge/INDEX.md` is Git blob `dcac9e587a26304e15eb15743b04142cb03c5081`, SHA-256
  `e774d9fcc1b12f9bb83932789ef4268711a7756f491dfd2a492be375b3529d1b`, 172 lines / 32,634 bytes /
  255 maximum content-line bytes. Its 158 data rows range from 132 to 255 raw bytes and average 201.9 bytes; the
  `.2.1.2` checker corrected `.0`'s diagnostic-only double encoding without changing file identity or headroom.
- The focused checker permits 160 cards and a 32,768-byte monolithic index. Two card-count slots and only 134
  index bytes remain; the next average-sized row cannot fit. These are 98.75% and 99.59% utilized respectively.
- The generic `knowledge_cards` surface counts all 160 Markdown files against a 200-file health target/ceiling,
  so it is exactly at its 80% warning milestone. Its nominal 40-file remainder conflicts with the focused
  checker's two-card remainder; the generated question-map contract separately permits 200 total facts and
  currently contains 159 facts / 1,133 unique question keys across eight shards.
- At the same boundary, the already-sharded `fact_index` question projection is nine files / 2,328 lines /
  209,621 bytes, only 94 bytes below its 80% aggregate-byte warning. Recording this boundary adds three retrieval
  keys and produces a current 2,334-line / 209,962-byte projection at 80.1%; it remains below the independent 90%
  rollover and 393,216-byte enforcement ceiling.
- Eleven non-archive files name the stable catalog path; path plus checker references span 13 unique files.
  Readers include README/bootstrap routing, the generated Knowledge Map, its shard contract/self-test, the
  live-size registry, mdBook, ADR/fact/research evidence, and the catalog itself. `check_live_document_size.sh`
  invokes the focused self-test and derive-and-diff check. `check_fact_card_catalog.pl --write` is the sole
  catalog producer and uses a same-directory temporary replacement; canonical cards remain manually authored.

## Decisions

- `2026-08-09`: Open a separate top-level containment tree while the tracked worktree is clean. The 158-card
  collection has reached its generic 80% file warning, while the focused checker and its monolithic generated
  index impose tighter independent constraints that were not represented by another open task.
- `2026-08-09`: Treat pressure as an information-architecture problem. No cap increase or evidence deletion is
  authorized by this tree; `.1` must choose a lossless bounded route from measured inputs.
- `2026-08-09`: Pin the exact Git tree/blob plus independent focused, generic-surface, and question-projection
  limits. The apparent 40-file generic remainder is not usable catalog capacity while the monolithic index has
  134 bytes left; `.1` must reconcile the contracts rather than privilege whichever metric looks roomier.
- `2026-08-09`: Keep question-shard semantics out of the browse migration, but require `.1` to account for their
  newly crossed aggregate warning. The projection is already partitioned and has ample hard-ceiling headroom;
  the warning is a capacity input, not permission to conflate canonical cards, browse parts, and question shards.
- `2026-08-09`: Accept ADR 0020. Keep the stable root as an exhaustive direct ID membership index and move exact
  detailed rows into deterministic count-packed title parts. Derive the 198-card maximum from the unchanged
  200-file surface after two fixed routes; do not treat the old premature 160 literal as a second authority.
- `2026-08-09`: Use `legacy_locked` then `migrated` commits. Generated output needs no archive terminal, but the
  old monolith's committed identity and ordered row union remain executable provenance until migration closes.
- `2026-08-09`: ADR 0021 supersedes only byte-identical migrated rows. A sibling-directory copy of `(card.md)`
  would be broken; enforce exact semantic tuples and resolved `docs/knowledge/<card>.md` destinations while
  retaining byte-exact legacy provenance. All accepted topology and limits remain unchanged.
- `2026-08-09`: ADR 0022 supersedes only the original 64-card packing count. The executable renderer exposed a
  73-line full part above mandatory rollover; a minimal scaffold still left 64 cards inside warning. Pack 56
  cards so a full part is 63/80 health lines, without changing the four-part maximum, target, or ceiling.
- `2026-08-09`: Enforce `legacy_locked` and `migrated` from one schema-closed contract. Cross-check capacity
  against the generic surface and question contract, preserve exact committed provenance and resolved routes,
  reject stale/premature residue, write from a repository-local workspace with the landing last, and fail at the
  fixed 90% rollover milestone.
- `2026-08-09`: Migrate only after the source-locked commit is clean. Switch the state and register the exact
  `fact_card_titles` generated surface in the same slice, invoke the sole writer, and require all four pinned
  output hashes before closure; no archive is needed because the retired monolith is reproducible and already
  authenticated in Git.
- `2026-08-09`: Hash semantic row unions as explicit raw UTF-8. The first real migrated verification exposed that
  ASCII-only fixtures had not exercised decoded non-ASCII titles; `raw_scalar` now closes that boundary and the
  synthetic migrated fixture includes an em dash.
- `2026-08-09`: Accept ADR 0023 before closure. The verbose root would reach 218/224 health lines at 198 cards,
  past mandatory rollover; retain one direct link per card but reduce the scaffold to three lines so the full
  capacity is 201/224 lines. Add an exact-capacity pressure case; do not widen any limit or milestone.

## Open Questions

- None. ADR 0020 fixes identity/title retrieval, membership, capacity authority, bounds, write ordering, residue,
  and migration stages before implementation.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-09` | `FACT-CARD-CATALOG-CONTAINMENT.0` | Git tree/blob/SHA/metrics; card/index limits; reader/writer census; catalog/KM/live-size/task/doctrine/book gates | exact committed baseline; no catalog contract/output, fact evidence deletion, product, artifact, threshold, or ceiling changed |
| `2026-08-09` | `FACT-CARD-CATALOG-CONTAINMENT.1` | worst-case root/part arithmetic; current in-memory render simulation; ADR/index links; source-index diff; catalogs/KM/live-size/doctrines/mdBook | bounded stable landing + deterministic title parts accepted; implementation policy closed; existing output unchanged |
| `2026-08-09` | `FACT-CARD-CATALOG-CONTAINMENT.2.1.1` | cross-directory link resolution; corrected render metrics/worst-case bounds; ADR/index links; source blob/destination absence; catalogs/KM/live-size/doctrines/mdBook | broken relative-link assumption corrected before code; semantic fields/destinations lossless; other ADR 0020 decisions unchanged |
| `2026-08-09` | `FACT-CARD-CATALOG-CONTAINMENT.2.1.2` | Perl syntax; 40/40 focused cases; real check/report; legacy write/diff/absence; catalog/KM/live-size/doctrine/mdBook/full-CI gates | schema-closed two-state checker; exact Git/card/row/output authorities; 56-card plan below warning; no migrated output written |
| `2026-08-09` | `FACT-CARD-CATALOG-CONTAINMENT.2.2` | corrected output hashes/metrics; 198-card pressure; resolved link membership; non-ASCII row digest; 41/41 cases; catalog/KM/live-size/doctrine/mdBook/full-CI/locality gates | exact bounded projection migrated; advertised capacity executable; generated surface active; tree closed without fact, route, evidence, threshold, or ceiling loss |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FACT-CARD-CATALOG-CONTAINMENT.0` | `FACT-CARD-CATALOG-CONTAINMENT.0 — own and pin fact catalog pressure` | ownership and exact pre-containment boundary |
| `FACT-CARD-CATALOG-CONTAINMENT.1` | `FACT-CARD-CATALOG-CONTAINMENT.1 — decide the bounded fact catalog topology` | ADR 0020; output unchanged |
| `FACT-CARD-CATALOG-CONTAINMENT.2.1.1` | `FACT-CARD-CATALOG-CONTAINMENT.2.1.1 — correct cross-directory fact routes` | ADR 0021; no code/output change |
| `FACT-CARD-CATALOG-CONTAINMENT.2.1.2` | `FACT-CARD-CATALOG-CONTAINMENT.2.1.2 — enforce the legacy-locked fact catalog` | checker/contract/ADR 0022; output remains legacy |
| `FACT-CARD-CATALOG-CONTAINMENT.2.2` | `FACT-CARD-CATALOG-CONTAINMENT.2.2 — migrate and close bounded fact browsing` | four-file projection + generated surface + closure |

## Changelog

- `2026-08-09`: Opened the separately owned containment program before changing the catalog contract or outputs.
- `2026-08-09`: `.0` pinned the 158-card/160-file collection, 32,634-byte monolithic index, conflicting
  independent limits, and all current reader/writer seams; frontier advances to topology decision `.1`.
- `2026-08-09`: `.1` accepted ADR 0020's stable direct-ID landing, three current/at-most-four deterministic title
  parts, derived 198-card authority, independent bounds, stale-output contract, and two-stage migration.
- `2026-08-09`: `.2.1.1` caught the sibling-directory relative-link contradiction before implementation; ADR 0021
  preserves exact row semantics and destinations with a deterministic `../knowledge/` rewrite.
- `2026-08-09`: `.2.1.2` landed the schema-closed two-state contract and checker, corrected raw-byte diagnostics,
  tightened packing to 56 cards under ADR 0022, and proved the exact future projection without writing it;
  frontier advances to migration/closure `.2.2`.
- `2026-08-09`: `.2.2` wrote the exact 161-line landing and three bounded title parts, activated their generated-
  projection surface, made semantic digesting explicitly UTF-8-safe, proved full 198-card pressure under rollover,
  passed all gates, and closed containment.
