# LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION: bounded live docs, stable README, and same-volume project data

## Metadata

- Tree ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION`
- Status: `active`
- Roadmap lane: process / continuity / repository portability
- Created: `2026-08-08`
- Last updated: `2026-08-08`
- Owner: SpecForge maintainers through the repo-local workflow
- Owner directives:
  - adopt the FSMGEN `README_POLICY.md` locally so `README.md` stops growing;
  - evaluate and, when suitable, adopt FSMGEN's
    `docs/LIVE_DOCUMENT_SIZE_CONTAINMENT_ADOPTION_GUIDE.md`;
  - keep every project-owned artifact, cache, output, log, fixture, and temporary
    workspace on the repository volume, with persisted paths relative to the repository root;
  - treat the boot-volume `~/.rustup` and `~/.cargo` stores as explicit shared toolchain
    dependencies, not project-owned storage.

## Goal

Adopt locally owned, mechanically enforced documentation-containment and project-data-locality
contracts without losing durable information. The resulting repository must expose a small bounded
working set, retain exact history through controlled terminals, keep the GitHub README useful as a
landing page, and reject project-owned writes that escape the repository volume.

The donor FSMGEN files are templates and evidence, not upstream authorities. SpecForge derives its
own classifications, routes, measurements, targets, ceilings, transition debt, and migrations.

## Measured adoption baseline (`2026-08-08`, read-only)

The repository already satisfies the bounded-resume-pointer premise (`MEMORY.md`: 27 lines), but
pressure has moved into neighboring mandatory/live surfaces:

| Surface | Lines | Bytes | Max content-line bytes | Initial finding |
| --- | ---: | ---: | ---: | --- |
| `README.md` | 602 | 170,891 | 2,928 | changing current-state chronology overwhelms the landing-page contract |
| `CHANGES.md` | 32,593 | 2,622,176 | 1,629 | append-only history presented as a live file |
| `DEVELOPMENT_NOTES.md` | 20,900 | 2,168,613 | 1,401 | append-only engineering chronology presented as a live file |
| `LIVE_ACHIEVEMENT_STATUS.md` | 1,950 | 572,231 | 5,467 | current status mixed with historical detail |
| `RUST_CODEBASE_ANALYSIS.md` | 9,039 | 1,046,679 | 3,041 | current architecture mixed with historical analysis log |
| `ROADMAP.md` | 1,487 | 183,445 | 717 | product direction plus accumulated delivery detail |
| `KNOWLEDGE_MAP.md` | 2,107 | 1,023,320 | 7,836 | generated projection is bounded neither by root nor shard controls |
| `docs/TASK_TREE.md` | 362 | 166,712 | 18,932 | bounded line count hides pathological generated-table width |
| mdBook temporal chapter | 1,986 | 87,730 | not yet classified | maintained prose includes large historical program sections |

The commit contract also requires broad live-document review on every slice and mandatory
`MEMORY.md` refresh before every commit even when resumable state did not change. That is ceremonial
co-staging pressure: a changed path proves neither current truth nor semantic alignment.

The same-volume census found a separate portability defect. Production Docling staging and numerous
Rust tests call `tempfile::tempdir()` without `tempdir_in`; absent an explicit `TMPDIR`, macOS places
those project-owned workspaces on the boot volume. The vendored FSMGen submodule has its own analogous
temporary-file calls and is governed at its own repository authority; SpecForge must control the
environment at the submodule boundary without editing the pinned submodule inline.

Two mdBook current-state drifts surfaced during the required full review and belong to the adoption's
content-truth work:

1. `domain/actor-connectivity.md` says the `.isf` adapter does not re-derive actor-relative direction,
   while current `isf_ir.rs` and `pipeline/isf-adapter.md` lower initiator-perspective directions.
2. `direction/temporal-intent-capture.md` contains an old design paragraph saying constrained
   extraction ships no code, beside the later delivered `extract-contracts` implementation.

These are not permission for an opportunistic edit: they are owned below and require code/record
verification before correction.

## Non-Goals

- No copied FSMGEN thresholds, registry records, paths, task IDs, migrations, or retention claims.
- No deletion, compression, or arbitrary line-count sharding merely because a file is large.
- No weakening of the bounded `MEMORY.md` contract.
- No automatic synchronization from FSMGEN after adoption; SpecForge owns its copies.
- No moving durable project data into user-home caches or system temp directories.
- No deletion of ambiguous shared `~/.cargo`, `~/.rustup`, Ollama, or other machine-global stores.
- No inline modification of the pinned `subs/fsmgen` submodule to satisfy SpecForge-local policy.

## Program acceptance criteria

- A project-owned `README_POLICY.md` and `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, each with a fenced
  SpecForge adoption note and an otherwise neutral normative body.
- A layer-C decision records authority, independence, lifecycle architecture, data locality, and the
  rule for raising any enforcement ceiling.
- A complete data-only registry classifies every live Markdown surface and every routed destination;
  routes close transitively and distinguish reader navigation from author overflow.
- A deterministic resulting-tree checker enforces path locality, line/byte/line-width and collection
  controls, transition debt, immutable/frozen terminals, route closure, and declared currency/freshness
  verifiers; it is registered in `scripts/check_doctrines.sh` and therefore runs in hook + CI.
- `README.md` is reduced to a verified landing page and protected by independent local line and byte
  ceilings derived from the retained SpecForge survivor.
- `MEMORY.md` remains a bounded overwrite-only pointer; the commit workflow updates it only when
  resumable state changes, not ceremonially.
- Every overgrown live surface has either landed a lossless lifecycle migration or carries immutable
  measured transition debt with a named owner and ordered remediation leaf.
- The mdBook remains complete and accurate through semantic partitioning; containment never means
  suppressing user-facing documentation.
- Project-owned generated/cache/temp/log/build data defaults to a repository-root-derived location on
  the repository volume. The shared boot-volume Rust toolchain stores are explicitly documented
  read/tool dependencies, and no project artifacts are written into them.
- The final census finds no undeclared route and no project-owned off-volume artifact; all gates pass;
  every leaf is committed through `COMMIT.md`.

## Task Tree

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION`
  Status: `active`
  Children: `.0`–`.7`

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.0`
  Status: `done` (`2026-08-08`)
  Goal: own the adoption, read the donor policy/guide/doctrine in full, measure SpecForge's pressure
  and locality baselines, record the migration graph, and commit the ownership slice before changing
  policy, gates, or content topology.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.1`
  Status: `pending`
  Goal: stabilize `MEMORY.md` and `COMMIT.md`: remove ceremonial commit coupling, define exactly when
  resumable state changes, and keep the existing memory cap. Prove the resume pointer still answers
  active unit / state / next action / in-flight work / blocker without chronology or a shadow HEAD.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.2`
  Status: `pending`
  Goal: adopt `README_POLICY.md`, prove duplication against canonical homes, trim `README.md` to its
  landing-page contract, derive SpecForge-local line+byte ceilings with explicit headroom, and add an
  unconditional README/routing guard. Preserve the minimal quick start, architecture, navigation,
  license, and support/contribution routes.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3`
  Status: `pending`
  Goal: adopt `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`; add the local registry and deterministic checker;
  classify the complete Markdown and route graph; register the check with doctrine enforcement; record
  exact transition debt without widening any baseline; add positive and fail-closed checker tests.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4`
  Status: `pending`
  Goal: migrate chronological live surfaces losslessly at whole-record boundaries. Children will own
  `CHANGES.md`, `DEVELOPMENT_NOTES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, and
  `RUST_CODEBASE_ANALYSIS.md` independently; each migration must prove source identity, semantic/current
  coverage, retrieval, indexes, and no residue before any live duplicate is removed.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5`
  Status: `pending`
  Goal: classify and contain collections/projections: task trees + bounded index, decision/fact stores,
  generated `KNOWLEDGE_MAP.md`, roadmap, research ledgers, and the mdBook maintained reference. Semantic
  partitioning and complete direct navigation are required; aggregate product-scope change must carry
  exact fresh authority rather than a decorative cap.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5a`
  Status: `pending`
  Goal: root-cause and correct the two verified mdBook current-state contradictions found during ramp-up,
  then add focused drift locks or canonical pointers so historical program narrative cannot override
  current implementation behavior.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.6`
  Status: `pending`
  Goal: implement the project-data-locality contract: repository-derived artifact/cache/temp roots;
  production and test temporary workspaces on the repository volume; explicit read-only shared-toolchain
  exception for `~/.rustup` and `~/.cargo`; submodule-boundary environment control; residue census and
  copy/verify/use/delete for exact project-owned off-volume data only.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.7`
  Status: `pending`
  Goal: final closure audit: resulting-tree doctrine check, documentation build, retrieval drills,
  route/locality census, roadmap/code/mdBook lockstep, transition-debt disposition, and clean handoff.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `.0` | `done` (`2026-08-08`) | Ownership, measured baseline, full donor review, and the migration/locality graph are recorded; no deletion or migration. |
| 1 | `.1` | `pending` | Stabilize the bounded pointer and stop ceremonial pressure before routing other surfaces. |
| 3 | `.2` | `pending` | The README is both the requested policy adoption and the most acute landing-page breach. |
| 4 | `.3` | `pending` | The complete registry/checker must govern every later migration. |
| 5 | `.4` / `.5` / `.6` | `pending` | Independently committable migrations and locality enforcement after the common contract exists. |
| 6 | `.7` | `pending` | Close only after every transition and retrieval/locality proof passes. |

## Decisions

- `2026-08-08`: **GO on local adoption.** SpecForge meets multiple guide triggers: bounded-pointer
  routing pressure, mandatory multi-megabyte live reads, unbounded append-only neighbors, generated
  projection width, and ceremonial co-staging. The donor values are not copied.
- `2026-08-08`: **One umbrella, independent leaves.** README stability, live-document containment,
  content-truth repairs, and same-volume project data share one repository-portability invariant but
  remain separately committable. No pivot to unrelated product work while this tree is dirty.
- `2026-08-08`: **No destructive migration before proof.** Existing Git history is not by itself an
  archive guarantee. Any removal from the live tree requires a controlled terminal, identity/retrieval
  evidence, consumer updates, and an atomic commit.
- `2026-08-08`: **Submodule boundary is an authority boundary.** `subs/fsmgen` is a pinned Git submodule.
  SpecForge may constrain its invocation environment but does not rewrite FSMGEN-owned files inline.

## Blockers

- None for `.0`–`.3`.
- Later migrations may stop for an authority conflict, unique unclassified content, or an unprovable
  retrieval promise, exactly as required by the adopted doctrine.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-08` | `.0` | full README/roadmap/code/mdBook ramp-up; all 14 active task-tree frontiers; FSMGEN README policy, adoption guide, and neutral doctrine read; `wc` + max-line + route/locality census; Git/submodule state | measured; adoption is justified; no policy/content migration performed |
| `2026-08-08` | `.0` | `bash scripts/check_doctrines.sh`; `mdbook build docs/book`; `git diff --check` | green: 3/3 doctrines; book built under repository-local `generated/mdbook/specforge`; clean diff |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.0 — own and measure the containment/locality program` | ownership + measurement only; commit hash recorded by Git history |

## Changelog

- `2026-08-08`: Created from the owner's external-SSD move, README-policy directive, and FSMGEN
  live-document-containment adoption guide. Recorded local measurements, task-owned the two ramp-up
  mdBook drifts, and separated common enforcement from lossless migrations and same-volume remediation.
