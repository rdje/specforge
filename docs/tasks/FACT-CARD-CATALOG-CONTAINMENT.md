# FACT-CARD-CATALOG-CONTAINMENT: keep fact-card browsing bounded before capacity fails

## Metadata

- Tree ID: `FACT-CARD-CATALOG-CONTAINMENT`
- Status: `active`
- Roadmap lane: process / continuity / durable knowledge retrieval
- Created: `2026-08-09`
- Last updated: `2026-08-09`
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
  Status: active
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
  Status: pending
  Goal: select and specify the bounded fact-card catalog topology
  Acceptance: a durable decision fixes authorities, membership, routing, partition/aggregate limits, update
  transaction, migration stages, and rejection cases without changing current catalog outputs
  Verification: pending
  Commit: pending

- ID: `FACT-CARD-CATALOG-CONTAINMENT.2`
  Status: pending
  Goal: implement, migrate, verify, and close fact-card catalog containment
  Acceptance: every canonical card remains unchanged and directly browsable through a bounded generated route;
  all readers and enforcement agree; no stale output or unowned capacity remains
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FACT-CARD-CATALOG-CONTAINMENT.1` | `pending` | choose the lossless bounded topology from the committed baseline before changing outputs |

## Pre-Containment Boundary

The immutable measurement boundary is commit `47e915409bbe6065544f86e49739b9e93c1ac75b`. At that revision:

- `docs/knowledge/` is Git tree `1efe8aafd1beefcd1eee30e8272419fab3c7b787`: 160 immediate Markdown files,
  8,029 lines, and 728,718 bytes. It contains 158 valid canonical cards plus `README.md` and `INDEX.md`.
- `docs/knowledge/INDEX.md` is Git blob `dcac9e587a26304e15eb15743b04142cb03c5081`, SHA-256
  `e774d9fcc1b12f9bb83932789ef4268711a7756f491dfd2a492be375b3529d1b`, 172 lines / 32,634 bytes /
  261 maximum content-line bytes. Its 158 data rows range from 132 to 261 bytes and average 204.7 bytes.
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

## Open Questions

- Which bounded browse topology best preserves one-hop id/title discovery while making per-file and aggregate
  capacity explicit? `.1` owns the decision; it does not block the read-only `.0` boundary census.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-09` | `FACT-CARD-CATALOG-CONTAINMENT.0` | Git tree/blob/SHA/metrics; card/index limits; reader/writer census; catalog/KM/live-size/task/doctrine/book gates | exact committed baseline; no catalog contract/output, fact evidence deletion, product, artifact, threshold, or ceiling changed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FACT-CARD-CATALOG-CONTAINMENT.0` | `FACT-CARD-CATALOG-CONTAINMENT.0 — own and pin fact catalog pressure` | ownership and exact pre-containment boundary |

## Changelog

- `2026-08-09`: Opened the separately owned containment program before changing the catalog contract or outputs.
- `2026-08-09`: `.0` pinned the 158-card/160-file collection, 32,634-byte monolithic index, conflicting
  independent limits, and all current reader/writer seams; frontier advances to topology decision `.1`.
