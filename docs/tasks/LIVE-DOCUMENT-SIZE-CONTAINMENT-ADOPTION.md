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
  Status: `done` (`2026-08-08`)
  Goal: stabilize `MEMORY.md` and `COMMIT.md`: remove ceremonial commit coupling, define exactly when
  resumable state changes, and keep the existing memory cap. Prove the resume pointer still answers
  active unit / state / next action / in-flight work / blocker without chronology or a shadow HEAD.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.2`
  Status: `done` (`2026-08-08`)
  Goal: adopt `README_POLICY.md`, prove duplication against canonical homes, trim `README.md` to its
  landing-page contract, derive SpecForge-local line+byte ceilings with explicit headroom, and add an
  unconditional README/routing guard. Preserve the minimal quick start, architecture, navigation,
  license, and support/contribution routes.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3`
  Status: `done` (`2026-08-08`)
  Children: `.3a`–`.3c`
  Goal: adopt `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`; add the local registry and deterministic checker;
  classify the complete Markdown and route graph; register the check with doctrine enforcement; record
  exact transition debt without widening any baseline; add positive and fail-closed checker tests.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3a`
  Status: `done` (`2026-08-08`)
  Goal: measure and classify the complete tracked Markdown surface, lock the local authority/schema/
  lifecycle/ceiling decisions in a task-owned design, and produce the exact `.3b` implementation
  contract. This leaf changes no active doctrine or enforcement registry.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3b`
  Status: `done` (`2026-08-08`)
  Goal: atomically adopt the SpecForge-owned neutral containment doctrine and ADR, populate the
  complete data-only surface registry, implement the deterministic resulting-tree checker, and wire
  it into the doctrine driver so the prose rule is never active without its gate.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3c`
  Status: `done` (`2026-08-08`)
  Goal: exercise positive and fail-closed checker paths for every lifecycle/control class, close the
  README route registry against the complete surface registry, update contributor/book guidance,
  and verify the full `.3` acceptance contract before migration begins.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4`
  Status: `done` (`2026-08-08`)
  Goal: migrate chronological live surfaces losslessly at whole-record boundaries. Children will own
  `CHANGES.md`, `DEVELOPMENT_NOTES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, and
  `RUST_CODEBASE_ANALYSIS.md` independently; each migration must prove source identity, semantic/current
  coverage, retrieval, indexes, and no residue before any live duplicate is removed.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4a`
  Status: `done` (`2026-08-08`)
  Goal: measure the four rolling-ledger record grammars and consumers, then lock one shared lossless
  live-window/sealed-segment/archive-descriptor protocol before moving any historical record.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4b`
  Status: `done` (`2026-08-08`)
  Goal: migrate `CHANGES.md` at a verified whole-record boundary under the `.4a` protocol.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4c`
  Status: `done` (`2026-08-08`)
  Goal: migrate `DEVELOPMENT_NOTES.md` at a verified whole-record boundary under the `.4a` protocol.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4d`
  Status: `done` (`2026-08-08`)
  Goal: migrate `LIVE_ACHIEVEMENT_STATUS.md` at a verified whole-record boundary under the `.4a`
  protocol while preserving its bounded current snapshot.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4e`
  Status: `done` (`2026-08-08`)
  Goal: migrate `RUST_CODEBASE_ANALYSIS.md` at a verified whole-record boundary under the `.4a`
  protocol.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5`
  Status: `active`
  Children: `.5a`–`.5i`
  Goal: classify and contain collections/projections: task trees + bounded index, decision/fact stores,
  generated `KNOWLEDGE_MAP.md`, roadmap, research ledgers, and the mdBook maintained reference. Semantic
  partitioning and complete direct navigation are required; aggregate product-scope change must carry
  exact fresh authority rather than a decorative cap.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5a`
  Status: `done` (`2026-08-08`)
  Goal: root-cause and correct the two verified mdBook current-state contradictions found during ramp-up,
  then add focused drift locks or canonical pointers so historical program narrative cannot override
  current implementation behavior.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5b`
  Status: `done` (`2026-08-08`)
  Goal: measure the exact collection/projection pressure, membership, navigation, producer, and
  currentness seams after `.4` closes; decompose `.5` into dependency-ordered leaves without moving
  canonical content; settle only the completed four-ledger index's provisional health target.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c`
  Status: `done` (`2026-08-08`)
  Children: `.5c.i`–`.5c.iii`
  Goal: replace incomplete or opaque collection retrieval with bounded direct navigation.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c.i`
  Status: `done` (`2026-08-08`)
  Goal: rebuild `docs/TASK_TREE.md` as a bounded complete catalog whose concise rows link every real
  task tree exactly once, exclude the author template from active work, and never mirror leaf history.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c.ii`
  Status: `done` (`2026-08-08`)
  Goal: close direct decision/fact retrieval: preserve the already-complete decision index, define a
  bounded human fact-card route that composes with the generated question index, and prove membership.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c.iii`
  Status: `done` (`2026-08-08`)
  Goal: add bounded direct indexes for research and the remaining canonical collections currently
  relying only on `git:query`, with complete one-hop membership and no duplicated canonical prose.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5d`
  Status: `done` (`2026-08-08`)
  Children: `.5d.i`–`.5d.ii`
  Goal: contain the generated Knowledge Map without weakening question-key retrieval or freshness.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5d.i`
  Status: `done` (`2026-08-08`)
  Goal: lock a deterministic bounded landing-index plus generated-shard contract, collision/order/
  derive-and-diff tests, canonical-input identity, and reader migration before changing the generator.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5d.ii`
  Status: `done` (`2026-08-08`)
  Goal: implement and migrate the Knowledge Map bundle atomically, ratchet the root out of transition
  debt, and prove every question key resolves directly to one canonical fact card.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5e`
  Status: `done` (`2026-08-08`)
  Children: `.5e.i`–`.5e.ii`
  Goal: separate current roadmap direction from accumulated delivery chronology without losing either.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5e.i`
  Status: `done` (`2026-08-08`)
  Goal: measure ROADMAP semantic boundaries, task-tree overlap, readers, and a lossless current/history
  design; no roadmap record moves before source identity and current milestone coverage are proved.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5e.ii`
  Status: `done` (`2026-08-08`)
  Goal: execute the approved roadmap migration, add bounded direct history retrieval, ratchet transition
  debt, and align the task catalog plus mdBook without duplicating live execution state.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5f`
  Status: `done` (`2026-08-08`)
  Children: `.5f.i`–`.5f.ii`
  Goal: migrate the FSMGen feedback ledger without losing requests, answers, resolutions, or issue links.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5f.i`
  Status: `done` (`2026-08-08`)
  Goal: lock feedback record grammar, direction/status semantics, consumers, source identity, reviewed
  live window, and archive/index protocol before moving any request or response.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5f.ii`
  Status: `done` (`2026-08-08`)
  Goal: execute the feedback migration atomically and prove unresolved/current requests stay directly
  visible while complete historical conversation remains retrievable.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g`
  Status: `active`
  Children: `.5g.i`–`.5g.iii`
  Goal: replace labelled currency debt with non-mutating producer-aware oracles.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.i`
  Status: `done` (`2026-08-08`)
  Goal: make `VALIDATION_SNAPSHOT.md` currentness executable against its declared artifact/report inputs
  without mutating validation artifacts during the check.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.ii`
  Status: `done` (`2026-08-08`)
  Goal: make `corpus/SOURCE_PDF_REGISTRY.md` membership and document-key/path currentness executable
  against the tracked corpus without reading host-local source libraries.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.iii`
  Status: `pending`
  Goal: make every `corpus_kb/` managed block reproducible/current against declared tracked inputs while
  preserving human synthesis and forbidding canonical-IR or typed-prior mutation.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5h`
  Status: `pending`
  Goal: audit root maintained references and the mdBook as one user-facing truth plane; remove only
  proven duplication, preserve unique canonical content, close direct routes, and reset exact aggregate
  authority to the resulting maintained-reference baseline.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5i`
  Status: `pending`
  Goal: close `.5` with a complete membership/navigation/currentness census, zero collection/projection
  transition debt, bounded indexes, mdBook/code/roadmap alignment, and full doctrine/CI retrieval drills.

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
| — | `.1` | `done` (`2026-08-08`) | Pointer semantics and impact-based documentation routing are now explicit and mechanically checked. |
| — | `.2` | `done` (`2026-08-08`) | The retained landing page, local policy, route registry, and unconditional doctrine guard are live. |
| — | `.3a` | `done` (`2026-08-08`) | Complete 550-file inventory, lifecycle mapping, schema, ceiling, and checker contracts are locked. |
| — | `.3b` | `done` (`2026-08-08`) | Doctrine, ADR, 24-surface registry, resulting-tree checker, and fifth driver entry are active atomically. |
| — | `.3c` | `done` (`2026-08-08`) | Forty-eight repository-volume fixtures prove every lifecycle and control-plane failure path; the common `.3` contract is closed. |
| — | `.5a` | `done` (`2026-08-08`) | Both contradictions are repaired against code and protected by the executed mdBook currency verifier. |
| — | `.4a` | `done` (`2026-08-08`) | Four real grammars, exact source identities, bounded survivor plans, consumers, and the repository-local source-capsule protocol are executable; no record moved. |
| — | `.4b` | `done` (`2026-08-08`) | Exact capsule, manifest/index, 87-record survivor plus first post-capsule entry, surface ratchet, and retrieval proof landed atomically. |
| — | `.4c` | `done` (`2026-08-08`) | Exact 1,601-record capsule, 60-record live suffix plus first prepend, common index/manifest extension, surface ratchet, and byte-sensitive retrieval proof landed. |
| — | `.4d` | `done` (`2026-08-08`) | Exact 1,920-record capsule, bounded 51-record snapshot, unchanged writer-managed trailer, and real writer-seam proof landed. |
| — | `.4e` | `done` (`2026-08-08`) | Exact 1,350-record capsule, Purpose prologue plus 60-record current view, common archive completion, and all consumer proofs landed. |
| — | `.5b` | `done` (`2026-08-08`) | Exact post-ledger collection/currency census, dependency graph, explicit child leaves, and scratch-locality finding landed without moving canonical content. |
| — | `.5c.i` | `done` (`2026-08-08`) | Derived 121-tree catalog, separate author-template route, seven fail-closed metadata/bound tests, unconditional checker, and transition-debt ratchet landed. |
| — | `.5c.ii` | `done` (`2026-08-08`) | Preserved 9/9 ADR routes and added a bounded derived catalog for all 136 fact cards plus the collection guide. |
| — | `.5c.iii` | `done` (`2026-08-08`) | Closed all eight query-only collections with direct membership over 367 canonical Markdown files. |
| — | `.5d.i` | `done` (`2026-08-08`) | ADR 0009, executable six-shard feasibility contract, collision correction, reader census, and 55 common fixtures landed without changing output topology. |
| — | `.5d.ii` | `done` (`2026-08-08`) | Seven-shard output, exact staging/derive-and-diff, raw-byte identity correction, reader migration, and normal-surface ratchet landed. |
| — | `.5e.i` | `done` (`2026-08-08`) | Exact source/regions, 23-workstream ownership, drift evidence, ADR 0010, readers, archive/root design, and nine-case unconditional gate are locked; no roadmap record moved. |
| — | `.5e.ii` | `done` (`2026-08-08`) | Exact capsule, bounded current root/index, corrected 23-row status, migrated verifier, reader alignment, and three-surface debt ratchet landed atomically. |
| — | `.5f.i` | `done` (`2026-08-08`) | Exact five-region source, six closed directed exchanges, zero-open status, two drift findings, 26 consumers, ADR 0011, archive/root design, and ten-case unconditional gate are locked; no feedback record moved. |
| — | `.5f.ii` | `done` (`2026-08-08`) | Exact capsule, 81-line zero-open current channel, six evidence-linked closed routes, migrated enforcement, and three-surface debt ratchet landed atomically. |
| — | `.5g.i` | `done` (`2026-08-08`) | The exact reviewed boundary, producer regions, live projection, ten fail-closed cases, and unconditional read-only currency oracle are enforced. |
| — | `.5g.ii` | `done` (`2026-08-08`) | Exact 22-PDF Git membership, registry paths/keys/directories/signatures, Rust derivation seams, and 13 fail-closed cases are enforced. |
| 1 | `.5g.iii` | `pending` | Make every managed corpus-KB block reproducible against declared tracked inputs without changing human synthesis. |
| 2 | `.5h`–`.5i` | `pending` | Maintained-reference audit and final collection/projection closure. |
| 3 | `.6` | `pending` | Enforce repository-volume project-data locality and audit old project-owned residue. |
| 4 | `.7` | `pending` | Close only after every transition and retrieval/locality proof passes. |

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
- `2026-08-08`: **Documentation synchronization is impact-based, never ceremonial (`.1`).** The
  owning task leaf changes on every completed slice. `MEMORY.md`, README, status, architecture,
  roadmap, ledgers, and book change only when the truth owned by that surface changes. An unchanged
  canonical file is the evidence that review found no impact; a no-op append proves nothing.
- `2026-08-08`: **Git owns revision truth (`.1`).** The pointer no longer mirrors `HEAD` or
  ahead/behind state. Resume uses `git rev-parse HEAD` and `git status --short --branch`; a named
  revision is allowed only for a distinct, precisely labelled semantic.
- `2026-08-08`: **README containment deletes duplication; it does not hide unique truth (`.2`).**
  The pre-trim 602-line/170,891-byte file devoted 293 lines to current repository state, 45 to a
  documentation index, 142 to the file map before Quick Start, and 42 to command detail. Those
  classes already have richer canonical homes in `docs/tasks/`/`CHANGES.md`, mdBook `SUMMARY.md`,
  the tracked tree, and the book's Getting Started/Commands chapters. The 127-line/4,834-byte
  survivor keeps every landing-page duty and links to those homes; no unique feature semantics were
  discarded.
- `2026-08-08`: **Local ceilings come from the survivor (`.2`).** README is capped independently at
  150 lines and 5,800 bytes: explicit headroom of 23 lines (18.1%) and 966 bytes (20.0%). A ceiling
  increase requires reviewed expansion of the landing-page contract, never ordinary feature growth.
- `2026-08-08`: **Reader and author routes are different controlled sets (`.2`).** The TSV registry
  contains every one of the README's 21 unique reader links plus the seven author-overflow
  destinations. Every record is repository-relative or an explicitly owned external/Git terminal,
  has a lifecycle and pressure control, and closes without another route. Existing oversized
  neighbors are declared transition debt owned by `.5`, not reusable health targets; `.3` must pin
  their measured ceilings before migration begins.
- `2026-08-08`: **The donor is precedent, not upstream (`.2`).** Root `README_POLICY.md` is now
  SpecForge-owned; its neutral body is locally authoritative and later donor changes do not sync
  automatically. The route guard also exposed a stale root `KNOWLEDGE_MAP_ARCHITECTURE.md` pointer;
  bootstrap/navigation references now use the real `knowledge-map/` bundle path, recorded as a fact
  card so it is not re-derived again.
- `2026-08-08`: **`.3` is decomposed before implementation.** `.3a` owns the full tracked-Markdown
  census and schema decisions only; `.3b` activates the neutral doctrine, complete data registry,
  checker, ADR, and driver entry atomically; `.3c` owns lifecycle-specific negative tests and route
  integration. This avoids landing an active prose doctrine without a mechanical gate.
- `2026-08-08`: **The parent Git index is the coverage authority (`.3a`).** Exactly 550 tracked
  SpecForge Markdown files are in scope. The `subs/fsmgen` gitlink contributes no parent-tracked
  Markdown and stays outside local ownership; generated untracked mdBook output is artifact data,
  not a live-document surface.
- `2026-08-08`: **The local registry is JSONL and the checker uses core Perl (`.3a`).** JSON::PP is
  available without a project dependency download and gives deterministic typed validation. A shell
  adapter resolves the repository root and feeds parent-tracked Markdown paths to the checker. The
  README route registry remains data-only TSV and is composed through its existing guard.
- `2026-08-08`: **Debt never masquerades as health (`.3a`).** Rolling ledgers and any already-
  overgrown single file are pinned to their final `.3b` resulting-tree measurements with zero growth.
  Healthy snapshots/collections get locally derived explicit headroom. Maintained-reference aggregate
  evolution requires exact task-owned delta authority; no donor threshold is copied.
- `2026-08-08`: **Completeness is exactly-once coverage (`.3a`).** A path absent from the registry or
  matched by two surface records is a hard failure. Every nullable limit must be justified by a
  lifecycle-specific authority contract, and every generated/current claim names a runnable verifier
  or explicit transition debt.
- `2026-08-08`: **Continuity allowance is separate from the immutable debt baseline (`.3b`
  correction).** `.3a`'s shorthand “zero growth” was too strong for the containment program's own
  mandatory task/status/book updates before each `.4`/`.5` migration. The baseline remains exact and
  never moves; only a separately bounded `transition.max_growth`, owned by this umbrella and enforced
  per dimension, may carry those continuity records. This is not health and does not authorize
  unrelated growth.
- `2026-08-08`: **The complete registry has 24 non-overlapping surfaces (`.3b`).** It classifies the
  final 553-file resulting set, including the new doctrine and ADR. Heterogeneous root files are
  exact-target records; homogeneous collections use closed patterns. Book aggregate authority is
  measured against `bc7546b3` and records the exact `.3b` line/byte delta.
- `2026-08-08`: **The checker is a local implementation, not a donor dependency (`.3b`).** Core Perl
  JSON::PP/Digest::SHA plus a path-resolving shell adapter validate registry self-bounds, safe paths,
  exactly-once coverage, all independent dimensions, lifecycle fields, index membership/query
  contracts, executed generated freshness, currency debt, frozen hashes, immutable baselines,
  maintained-reference authority, README routes, and ceiling-increase authority. It mutates nothing.
- `2026-08-08`: **Resulting-tree enforcement found two pre-existing classification drifts (`.3b`).**
  Thirteen nested `docs/research/grounding/*.md` files required recursive collection coverage, and
  `docs/decisions/INDEX.md` omitted the existing genericity ADR because two historical records share
  the `0006` prefix. The registry pattern and index now include both; no file was deleted or renamed.
- `2026-08-08`: **Currentness is claimed only where proved (`.3b`).** The Knowledge Map's existing
  derive-and-diff check is executed. Validation snapshot, source registry, and corpus-KB currency are
  labelled owned transition debt until `.5` supplies non-mutating currentness oracles; presence alone
  is not called freshness.
- `2026-08-08`: **The registry schema is closed and bounded below the raw-record layer (`.3c`).**
  Fixture design exposed that `.3b` bounded record count, total bytes, and record bytes but did not
  yet reject unknown keys or independently cap array cardinality and scalar bytes. Both JSONL
  metadata records now declare those local limits; schema version, field sets, identifiers, locators,
  arrays, scalars, and ceiling-authority records all fail closed.
- `2026-08-08`: **Proof fixtures are same-volume and unconditional (`.3c`).** The 48-case suite creates
  disposable workspaces only under `generated/`, exercises all seven locally governed lifecycle
  classes plus membership/query, currency, route, every size dimension, immutable baseline, and
  ceiling-history paths, and is invoked in quiet mode by `LIVE-DOC-SIZE` on every doctrine run.
- `2026-08-08`: **The README route registry is independently self-bounded (`.3c`).** Its reviewed
  31-record/3,837-byte survivor has a 146-byte widest raw record. Local ceilings are 40 records,
  5,000 total bytes, and 192 raw bytes per record; the README guard's self-test proves each boundary
  rejects independently.
- `2026-08-08`: **Both mdBook drifts were time-layer collisions, not code ambiguity (`.5a`).** The
  ISF adapter did default unresolved direction, but later `KG-ISF-COMPLETENESS.2a.ii` added
  initiator selection and initiator-perspective graph lowering; an older audit narrative and the
  actor-connectivity current section still described the pre-change state. Likewise, the constrained
  extraction chapter kept its pre-delivery “no code today” design introduction after the later
  `extract-contracts` delivery section landed in the same file.
- `2026-08-08`: **Current mdBook truth is now executable currency (`.5a`).**
  `scripts/check_book_current_truth.sh` rechecks the load-bearing code seams, rejects both stale
  formulations, and requires pointers from the repaired current prose to the canonical detailed
  ISF-direction and command chapters. The maintained-reference surface executes it through
  `LIVE-DOC-SIZE`; historical narrative is explicitly time-labelled rather than treated as current.
- `2026-08-08`: **Ledger boundaries are grammar-specific and lossless (`.4a`).** A closed, bounded
  registry declares mixed H3/H2, H2-with-prologue, and snapshot-bullet grammars. The checker parses
  6,667 records across 6,425,335 source bytes and reconstructs every source byte-for-byte before
  deriving a planned whole-record survivor. A generic Markdown heading split is prohibited.
- `2026-08-08`: **Initial migrations use an exact source capsule plus a bounded current view
  (`.4a`).** The immutable repository-local capsule preserves complete-source SHA-256 identity; the
  stable root retains the reviewed newest-first window. Their intentional overlap is declared and
  measured. Later rollover seals only new aged-out records, not another full source copy.
- `2026-08-08`: **The `CHANGES.md` tail anomaly is compatibility evidence, not permission to rewrite
  history (`.4a`).** `git blame` ties the two detached `.0`/`.1` records to commits `46390523` and
  `ad07eb3f`: those initial slices appended after the 2026-03-31 legacy tail, while `.2` and later
  slices prepended. The live migration will promote those exact records after its newest prefix, but
  the source capsule retains their original ordinals and bytes. No silent reorder occurs.
- `2026-08-08`: **The status writer owns a stable trailer seam (`.4a`).**
  `project_validation.rs` writes `LIVE_ACHIEVEMENT_STATUS.md` between the
  `validation_projection:start` / `validation_projection:end` markers. Both markers and the root
  path are required consumer literals; the bullet record parser leaves the entire trailer outside
  the rollover region.
- `2026-08-08`: **`CHANGES.md` is the first migrated ledger (`.4b`).** The exact `.4a` boundary is
  preserved at `docs/archive/rolling-ledgers/changes/source-through-2026-08-08.md`: 1,798 records,
  32,682 lines, 2,629,033 bytes, SHA-256
  `d89809322857aab3c506dde1cc6caaf57e22d0349655b37ddaab1f7bf22ba994`. The root is mechanically
  derived from 87 capsule records; `.4b` is its first new prepend. No capsule byte was reformatted.
- `2026-08-08`: **Archive identity and live pressure are independently governed (`.4b`).** The
  capsule is an `archive_terminal` with exact ceilings and no ordinary pressure warning; the root is
  a normal `rolling_ledger` at 1,368 lines / 199,851 bytes with a 2,000-line / 262,144-byte / 2,200-
  max-line ceiling. The rolling checker still reopens and hashes the capsule on every doctrine run.
- `2026-08-08`: **The width ceiling increase is exact and separately authorized (`.4b`).** Total
  live line/byte ceilings ratchet down by more than 92%; the legacy 1,629-byte exact quarantine width
  becomes the `.4a`-reviewed 2,200-byte transaction ceiling. The data-only ceiling-authority record
  names `.4b`, the complete old/new vectors, rationale, and owner; it cannot be reused or banked.
- `2026-08-08`: **Planned status identity advances only by a measured leading record (`.4b`).** The
  new current-status bullet changed the future `.4d` source, so its exact measurement and 50-record
  survivor were re-derived in the same commit: 1,919 source records / 580,272 bytes and an 88,146-byte
  planned view. This is the `.4a` exception for a legitimate pre-freeze prepend, not baseline drift.
- `2026-08-08`: **`DEVELOPMENT_NOTES.md` is the second migrated ledger (`.4c`).** Its exact capsule
  retains 1,601 records / 20,921 lines / 2,170,230 bytes at SHA-256
  `76b51a3f450cdb1e764922dc366cf6ff55529cb95f7a6f410bfba1f1f378fedc`. The root retains the H1
  prologue and newest 60 H2 records; `.4c` is the first post-capsule prepend.
- `2026-08-08`: **A successor separator is boundary structure, not retained record content (`.4c`).**
  The H2 chunker initially assigned the blank immediately before record 61 to record 60. Keeping that
  LF after record 61 is removed creates a blank-at-EOF artifact; dropping it failed raw suffix identity.
  The protocol now omits exactly one terminal blank separator only when an H2 live view cuts before a
  successor. The exact capsule still preserves every source byte and every record-content byte must
  match. This is a closed boundary rule, not general whitespace normalization.
- `2026-08-08`: **Engineering-note width headroom is separately authorized (`.4c`).** Aggregate live
  ceilings ratchet by more than 90%; the 1,401-byte legacy exact quarantine becomes the reviewed
  1,800-byte whole-record transaction ceiling through one exact `.4c` old/new authority.
- `2026-08-08`: **The status ledger is the third migrated ledger (`.4d`).** Its exact capsule retains
  1,920 records / 1,960 lines / 581,239 bytes at SHA-256
  `b00ff5f5c4a29554a20eea9d901a95848d54a644749eba74ad9618798dd9bd6a`. The stable root retains the
  H1/current heading, newest 50 snapshot bullets, complete gap/validation trailer, and `.4d` as the
  first post-capsule prepend.
- `2026-08-08`: **Status rotation never owns the generated validation projection (`.4d`).** The exact
  marker pair and all trailer bytes remain outside the parsed record region. The focused
  `project_validation_writes_snapshot_doc_and_updates_live_status` test executed with repository-local
  temporary storage and proved the real Rust writer still replaces its managed block at the stable root.
- `2026-08-08`: **Status width headroom is separately authorized (`.4d`).** Count/line/byte ceilings
  ratchet down by more than 78%; the 5,467-byte legacy exact quarantine becomes the reviewed
  6,800-byte whole-record transaction ceiling through one exact `.4d` old/new authority.
- `2026-08-08`: **The Rust architecture ledger completes the four-ledger migration (`.4e`).** Its
  exact capsule retains 1,350 records / 9,039 lines / 1,046,679 bytes at SHA-256
  `95e1665628b615498e94f67d6dc6e0083d4d104a6ca8a815b4c23cf2f97cc7ce`. The stable root is exactly
  the H1/Purpose prologue plus newest 60 H2 records; all pressure dimensions ratchet down, so no
  ceiling-increase authority is needed.
- `2026-08-08`: **A lifecycle migration is not a Rust architecture fact (`.4e`).** The root receives
  no ceremonial `.4e` H2 record because no subsystem boundary, public integration, implementation,
  or current risk changed. The owning task, change/status/rationale ledgers, ADR, fact card, and book
  document the migration; the architecture root remains a byte-derived current suffix.
- `2026-08-08`: **Collection containment is a dependency graph, not one bulk rewrite (`.5b`).** Direct
  task navigation comes first; decision/fact/research collection routes follow; the Knowledge Map can
  then shard against stable fact routes; roadmap and feedback migrations retain independent source
  proofs; producer-backed currency oracles follow; maintained-reference and aggregate closure is last.
- `2026-08-08`: **The task catalog is incomplete and pathologically wide (`.5b`).** The entry census
  found 122 tracked task files but only 117 unique task links. Four real task trees are absent
  (`CONSTRAINT-DRIVE-LEVEL-RECALL`, `EVIDENCE-MATERIALIZE-IDEMPOTENCY`,
  `FSMGEN-REFRESH-INTEGRATE-3`, `NLI-CLAIM-CONNECTOR`) and `TEMPLATE.md` is correctly not active work;
  the widest row is 18,932 bytes. `.5c.i` must rebuild, not append to, the catalog.
- `2026-08-08`: **Already-honest collection controls are preserved (`.5b`).** The nine ADRs have
  complete one-hop membership in `docs/decisions/INDEX.md`; the 33-part mdBook has `SUMMARY.md` plus
  executed current-truth and exact aggregate-change controls. `.5` will not replace these with another
  index merely for uniformity.
- `2026-08-08`: **Diagnostic scratch is project data (`.5b`, surfaced finding).** A read-only task-link
  comparison accidentally materialized two exact lists under `/private/tmp`. They were immediately
  deleted and absence-proved. The root cause is shell scratch locality, not collection content;
  `docs/knowledge/repository-local-scratch.md` records the rule and `.6` owns enforcement/census.
- `2026-08-08`: **The completed four-ledger index needed a health-target settlement, not a higher cap
  (`.5b`).** Its final 58 lines / 3,283 bytes legitimately consumed the provisional 64-line / 4,096-
  byte target while remaining far below the 128-line / 8,192-byte enforcement ceiling. Health moves
  to 96 lines / 6,144 bytes; the enforcement ceiling is unchanged and no authority is banked.
- `2026-08-08`: **The task catalog is derived navigation, not a status mirror (`.5c.i`).** One concise
  row comes from each real task file's filename/H1, first metadata status, and title. `TEMPLATE.md` is
  excluded from rows and linked separately for authors. Resume state remains in `MEMORY.md`; frontier,
  decisions, evidence, and commit history remain in the owning tree.
- `2026-08-08`: **Task catalog completeness and bounds are executable (`.5c.i`).** The checker rejects
  unsafe filenames, filename/H1 mismatch, missing or unknown status, more than 160 tasks, a title over
  240 bytes, a row over 512 bytes, a section over 49,152 bytes, missing/duplicate markers, or derive-
  and-diff drift. Seven parser/bound cases and the 121-tree real catalog run unconditionally.
- `2026-08-08`: **Line capacity is separately authorized while byte and width ceilings collapse
  (`.5c.i`).** The resulting file is 375 lines / 27,228 bytes / max line 207, versus 362 / 167,583 /
  18,932. The exact authority changes the transition-era line ceiling 462→512 to cover the registered
  160-file task capacity plus fixed workflow, while bytes ratchet 200,295→65,536 and width
  18,932→512; it is not reusable headroom.
- `2026-08-08`: **Decision and fact-card navigation remain separate views (`.5c.ii`).** The existing
  decision index stays canonical for nine ADRs. A derived `docs/knowledge/INDEX.md` will provide one
  bounded human row per immediate fact card, while `KNOWLEDGE_MAP.md` remains the question-keyed
  generated route until `.5d` shards it. Neither index copies card or decision prose.
- `2026-08-08`: **Fact-card membership must fail closed (`.5c.ii`).** The catalog generator will
  require safe filename/id identity, valid front matter, a non-empty question list, date, a valid
  explicit status or the architecture-defined `current` default, evidence or reverify, deterministic
  ordering, exact member coverage, and independent source-field, row, count, and section bounds. The
  existing Knowledge Map validator remains independently active.
- `2026-08-08`: **Route state must follow landed controls (`.5c.ii`, surfaced finding).** The README
  route registry still called the task catalog and already-complete decision collection transition
  debt after their executable membership controls existed. Their reader/author routes move to those
  composed live-document gate, and the new fact-card catalog receives its own bounded reader route;
  no healthy route keeps a ceremonial debt label.
- `2026-08-08`: **External catalogs are a real membership topology (`.5c.iii`, surfaced finding).**
  Eight canonical collections still declare `git:query`. Task evidence already has an exact derived
  catalog at `docs/TASK_TREE.md`, but the generic doctrine rejects any membership index outside its
  member surface. The schema will add an explicit fail-closed `external_membership` kind rather than
  mislabel the route, duplicate the task catalog inside `docs/tasks/`, or weaken locality.
- `2026-08-08`: **One bounded catalog plane closes the remaining query-only routes (`.5c.iii`).** Six
  generated indexes under `docs/catalogs/` will cover workflow standards, root architecture, FSMGen
  issue packets, research, corpus-KB pages, and KG fixture Markdown. A seventh catalog-of-catalogs
  indexes that plane. The four-file Knowledge Map bundle keeps its own README, converted to complete
  direct links. Every row derives only repository-relative path plus first H1 (filename fallback for
  the titleless legacy `SESSION_BOOTSTRAP.md`); canonical prose stays put.
- `2026-08-08`: **A question key must have one canonical destination (`.5d.i`, surfaced finding).**
  The 978 generated rows contained 977 unique questions: one exact register-bit-field question mapped
  to both the broad per-category scorecard and the focused lowering-gap card. The focused card owns
  that answer; the duplicate key is removed from the broad card. The shard contract rejects all future
  question collisions instead of silently emitting multiple destinations.
- `2026-08-08`: **The Knowledge Map becomes one bounded generated projection set (`.5d.i`).**
  `KNOWLEDGE_MAP.md` remains the stable landing path, directly indexes deterministic whole-entry
  question shards, and routes id/title browsing to the existing fact catalog. Question text is the
  retrieval key; repeated fact metadata and the duplicate generated fact catalog are not.
- `2026-08-08`: **Canonical identity hashes raw bytes before decoding (`.5d.ii`, surfaced finding).**
  The first real migration exposed different simulator/generator SHA-256 values. Perl `decode_utf8`
  with a check flag consumed the source scalar before the simulator hashed it, effectively identifying
  paths plus empty contents. Raw bytes are now hashed first and decoded from a copy; a ninth focused
  case locks content sensitivity and both independent implementations report the same identity.
- `2026-08-08`: **Aggregate projection limits are explicit (`.5d.ii`).** Per-shard and shard-count
  limits mathematically bounded the set, but would bank their full product. The generator and doctrine
  additionally enforce 4,096 aggregate lines / 393,216 aggregate bytes across landing plus shards.
- `2026-08-08`: **Roadmap history requires an exact capsule plus semantic rewrite (`.5e.i`).** The 23
  workstream records mix active goals and remaining scope with 722 lines / 129,242 bytes of `done:`
  chronology. Whole-heading partitioning cannot preserve one role without the other. ADR 0010 keeps
  the stable root as bounded current direction and preserves every old byte in a verified terminal.
- `2026-08-08`: **Canonical lane summaries drifted behind appended chronology (`.5e.i`, surfaced
  finding).** R6/R7 still say in progress, R14 says not started, and recommended order still names
  completed R15 work. Git blame and owning task trees prove these lines predate closure; `.5e.ii` must
  correct the current root while the exact capsule preserves the stale prose as history.
- `2026-08-08`: **Current roadmap coverage is 23 high-level rows, not a mirrored task ledger
  (`.5e.i`).** Each R0–R16/R15b–g row links one owning task tree, while `docs/TASK_TREE.md` remains the
  complete execution/status catalog. Delivery bullets and applied-tree chronology are forbidden in
  the bounded root and remain directly retrievable through task, change, and archive routes.
- `2026-08-08`: **Roadmap containment landed as two independently governed products (`.5e.ii`).**
  The stable root is 153 lines / 12,033 bytes of current direction and exact owner-linked status. The
  1,487-line / 183,445-byte pre-containment source remains byte-identical at its pinned SHA-256 in an
  immutable terminal behind a bounded index/manifest. Neither product substitutes for the other.
- `2026-08-08`: **Transition debt closes only when semantic and identity proofs compose (`.5e.ii`).**
  The general surface registry independently governs root, archive index, and capsule; the roadmap
  contract then proves exact source identity, current row semantics, reader routes, and the relationship
  among all three on every doctrine run. A smaller root alone would not have closed the leaf.
- `2026-08-08`: **FSMGen feedback is a directed channel over heterogeneous history (`.5f.i`).** The
  source has five ordinary directed H2 exchanges, one three-H2 two-bug episode, a current scope override,
  and a legacy primer. All six exchanges are closed; future open records need explicit direction, kind,
  status, owner, and evidence rather than an inferred newest-first ledger grammar.
- `2026-08-08`: **The feedback primer has two time-layer collisions (`.5f.i`, surfaced finding).**
  April prose still calls `.fsm` a SpecForge adapter, and a June line calls `030f8c273` the latest
  response pin. Git blame proves both predate the `.isf`-only consolidation/current `d327129b7` gitlink.
  The exact capsule will retain them as history; the bounded channel must state current truth directly.
- `2026-08-08`: **Detailed feedback and reproductions already have canonical homes (`.5f.i`).**
  FSMGen's response stays authoritative in the pinned submodule and bugs stay in the bounded issue-
  bundle catalog. The current channel owns active correspondence plus concise closed routing, not copied
  response chapters or embedded reproduction archives.

## Blockers

- None for `.0`–`.3`.
- Later migrations may stop for an authority conflict, unique unclassified content, or an unprovable
  retrieval promise, exactly as required by the adopted doctrine.

## `.5e.i` Roadmap Current/History Boundary Contract

The pinned pre-migration `ROADMAP.md` is 1,487 lines / 183,445 bytes / 717 maximum content-line
bytes with SHA-256 `20a71e88c15133398879fb11620dbb4c3ed20a6b689f2bcea733ef82b2a47d88`.
Five exhaustive regions reconstruct that source exactly:

| Region | Lines | Bytes | Role |
| --- | ---: | ---: | --- |
| objective, pipeline, and cross-cutting doctrine | 70 | 4,759 | stable current direction |
| major-workstream heading plus 23 H3 records | 1,291 | 168,815 | current goals/status mixed with delivery history |
| post-lock applied-tree section | 45 | 3,316 | delivery/alignment chronology plus catalog route |
| recommended order | 24 | 1,888 | current sequence mixed with completed work |
| immediate milestone | 57 | 4,667 | current priorities mixed with dated progress |

The H3 workstream grammar is structurally complete: exactly R0–R16 plus R15b–R15g, in order, each
with one top-level status. It is not semantically separable at whole-heading boundaries. Top-level
`done:` blocks alone occupy 722 lines / 129,242 bytes, while active goals, remaining scope, obsolete
status labels, superseded `.fsm` history, and completion chronology share the same workstream records.
The three largest records—R15e, R15, and R15f—consume 110,829 bytes. A section split would preserve
the wrong role; arbitrary line deletion would not be a lossless migration.

The reader/writer census found ten direct readers and one human workflow writer. README, architecture,
user guide, task doctrine, three mdBook chapters, route data, and the README guard all retain the
stable root path. `COMMIT.md` remains the only writer contract: the roadmap changes only for program
direction, milestones, or roadmap-level status. No script or Rust producer mutates it.

Four current-truth defects are pinned rather than normalized away:

1. R6 says `In Progress` after the ISF-only consolidation and named adapter trees closed or were
   superseded.
2. R7 says `In Progress` and links its tree as `active`, while `R7-VALIDATION` closed on 2026-05-20.
3. R14 says `Not Started`, although `R14-SIGNAL-RESOLVE` shipped and closed on 2026-05-31.
4. recommended item 3 still asks to finish R15 graph direction, while its owning migration is done and
   the R15 record itself says no forward adapter-direction work remains.

Git blame establishes the cause: these canonical status/order lines predate their owning task-tree
closures. Later completion material was appended to other roadmap regions without correcting the
lane summaries. The exact capsule must preserve those stale statements as historical evidence; the
bounded current root must resolve them against task-tree/code truth.

ADR 0010 adopts a snapshot-plus-terminal design. Before any root replacement, `.5e.ii` must copy the
exact source to `docs/archive/roadmap/source-through-2026-08-08.md`, verify byte count and SHA-256,
and land a bounded index plus JSON manifest. The rewritten root keeps the objective, pipeline, and
cross-cutting doctrine; adds current strategic priorities; lists all 23 workstreams exactly once with
direct owning-tree routes; contains forward order only; and links the task catalog, current change
history, and roadmap archive. It cannot contain `done:` blocks, dated progress bullets, or the applied-
tree chronology section.

The current root targets 256 lines / 32,768 bytes / 384 maximum line bytes and hard-fails above 384 /
49,152 / 512. The archive index is independently capped at 96 lines / 8,192 bytes / 256 maximum line
bytes. These values are derived from the 70-line retained doctrine, 23 bounded rows, priorities/order,
and direct routes; they do not inherit the 1,487-line transition ceiling or bank capacity for another
chronology.

`doctrine/live_document_size/roadmap_projection.json` locks source/region identities, ordered
workstreams and owners, drift evidence, current-root structure and limits, archive topology, and all
known consumers. `scripts/check_roadmap_projection_contract.pl` has nine in-memory fail-closed cases
for valid structure, duplicate/reordered/missing source records, valid bounded current structure,
missing/duplicate current rows, forbidden chronology, and unsafe paths. It runs unconditionally via
`LIVE-DOC-SIZE`. In `planned` state it verifies the untouched root identity; `.5e.ii` switches it to
`migrated`, where it verifies capsule/manifest/index identity and bounded current-root coverage.

This design leaf does not alter `ROADMAP.md`, create the archive, change program direction, or remove
transition debt. Those changes are one atomic `.5e.ii` slice after the source and reader contracts are
durable.

## `.5e.ii` Atomic Roadmap Migration

The exact pre-containment source is now sealed at
`docs/archive/roadmap/source-through-2026-08-08.md`. Direct comparison with commit `cd677aec` proves
the capsule retains 1,487 lines / 183,445 bytes / 717 maximum line bytes and SHA-256
`20a71e88c15133398879fb11620dbb4c3ed20a6b689f2bcea733ef82b2a47d88`. The independently bounded
`docs/archive/roadmap/INDEX.md` links the current root, capsule, and machine-readable identity
manifest; the manifest names only repository-root-relative paths.

The stable root is 153 lines / 12,033 bytes / 205 maximum line bytes. It retains the original
objective, canonical pipeline, and cross-cutting doctrine; adds the present strategic priorities;
contains exactly one status/owner row for R0–R16 and R15b–R15g; gives only forward implementation
order; and links the task catalog, change/status/pointer surfaces, archive, and Git retrieval. It
corrects the four `.5e.i` drifts from owning task-tree truth: R6/R7/R14 are done and completed R15 is
not recommended as future work. No `done:` chronology, applied-tree mirror, or dated progress block
remains live.

The roadmap projection state is `migrated`. Its nine focused cases still prove malformed-source and
malformed-current failures, while the real check now authenticates the capsule, reconstructs all five
source regions, validates all 23 ordered current rows and exact owner routes, rejects forbidden
chronology, and verifies manifest/index retrieval. The common registry classifies the root as a normal
bounded snapshot, the index as an independent bounded snapshot, and the capsule as an exact terminal.
The old 1,487-line transition ceiling is gone; root health is 256 lines / 32,768 bytes / 384 maximum
line bytes and hard enforcement remains 384 / 49,152 / 512.

## `.5f.i` FSMGen Feedback Current/History Contract

The pinned `docs/FSMGEN_FEEDBACK.md` is 936 lines / 57,980 bytes / 1,705 maximum content-line
bytes with SHA-256 `5bf938d54559d1f80c5aed4d3a72a1ad3ec5792c6991b846beffb35d42739f03`.
Its complete source partitions into five exact regions:

| Region | Lines | Bytes | Role |
| --- | ---: | ---: | --- |
| H1 prologue | 2 | 34 | stable path identity |
| `.isf`-only scope policy | 15 | 800 | current downstream boundary and historical override |
| five directed exchanges | 359 | 26,862 | detailed questions/answer/suggestion/request plus embedded outcomes |
| legacy primer | 456 | 23,709 | April integration orientation plus later baseline notes |
| two-bug issue episode | 104 | 6,575 | tracked findings, bundle routes, and upstream resolution |

The five ordinary exchange spans and one composite bug episode reconstruct their source regions
exactly. Their controlled status ledger is complete:

| Record | Direction | Kind | Status | Independent closure evidence |
| --- | --- | --- | --- | --- |
| `enum-type-clarity` | SpecForge → FSMGen | clarity request | resolved | upstream response heading |
| `min-window-answer` | SpecForge → FSMGen | answer | answered | upstream follow-up/shipped-window heading |
| `ltl-mtl-suggestion` | SpecForge → FSMGen | suggestion | answered and integrated | upstream verification-family response |
| `phase-membership` | SpecForge → FSMGen | question | answered | upstream phase-membership response |
| `field-structured-storage` | SpecForge → FSMGen | feature request | shipped | upstream storage-field response |
| `stage-contract-bugs` | SpecForge → FSMGen | bug report | resolved | both repository-local bundle resolutions |

There are zero open records. The source is not an ordinary newest-first ledger: the bug episode is
three linked H2 sections below a general primer, and later responses modify earlier request bodies.
The safe initial migration is therefore an exact source capsule plus a semantic current channel, not
whole-record suffix retention.

The legacy primer also cannot remain current verbatim. Lines 412–413, blamed to `4f0cf815` on
2026-04-17, present `.fsm` alongside future HDL adapters; lines 424/434, last updated by `e15c6b1e` on
2026-06-16, call `030f8c273` latest. The 2026-05-18 scope override and
`FSMGEN-REFRESH-INTEGRATE-5` prove current truth is strictly `IntentIR → .isf → FSMGen` at pinned
gitlink `d327129b7`. The capsule preserves the chronology; the future root forbids both stale literals.

ADR 0011 keeps `docs/FSMGEN_FEEDBACK.md` as the bounded current channel and freezes the exact source at
`docs/archive/fsmgen-feedback/source-through-2026-08-08.md` before replacement. The future root carries
the current boundary, explicitly marked open correspondence, six-row closed register, authoring schema,
and direct response/issue/archive routes. A bounded index plus JSON manifest makes the capsule directly
retrievable and independently verifiable.

Future open records declare `Direction`, `Kind`, `Status`, `Owner`, and `Evidence`. The controlled live
set admits at most eight records, each at most 96 lines / 12,288 bytes / 512 bytes per line; detailed
designs and reproductions route to task/research records or issue bundles. Root health targets are 256
lines / 32,768 bytes / 512 maximum line bytes; hard ceilings are 384 / 49,152 / 1,024. The archive index
hard-fails above 96 / 8,192 / 256.

`doctrine/live_document_size/fsmgen_feedback.json` pins all five regions, six exchanges and their
direction/status/evidence, two drift findings, 26 stable-path consumers, open-record schema, current-
root bounds, and archive topology. `scripts/check_fsmgen_feedback_protocol.pl` runs unconditionally via
`LIVE-DOC-SIZE`; ten planned/migrated cases fail closed on identity, evidence, record heading, unsafe
path, marker, register, forbidden-history, or manifest drift. This design leaf moves no feedback record,
creates no archive, and leaves the transition baseline unchanged; `.5f.ii` owns the atomic migration.

## `.5f.ii` Atomic FSMGen Feedback Migration

The stable channel is now 81 lines / 5,243 bytes / 394 maximum content-line bytes. It carries the
current `SPECFORGE IntentIR → .isf → FSMGEN` boundary, full pinned gitlink
`d327129b718ab29fc889db026c19257b0f7fcc49`, one explicitly marked zero-open region, the six closed
records in contract order, the bounded authoring schema, and direct archive/response/issue/tooling
routes. The two stale primer claims are absent from current prose.

Before replacing that root, `.5f.ii` mechanically copied and byte-compared the complete pinned source
to `docs/archive/fsmgen-feedback/source-through-2026-08-08.md`. The 936 lines / 57,980 bytes /
1,705-byte maximum line retain SHA-256
`5bf938d54559d1f80c5aed4d3a72a1ad3ec5792c6991b846beffb35d42739f03`. A 26-line bounded index links
the stable root, exact capsule, manifest, upstream response, and issue catalog. The JSON manifest pins
source path, capsule path, digest, all dimensions, sealed date/reason, and verifier.

The closed register is deliberately semantic rather than positional. Each row preserves controlled
id, status, and direction and links to both its complete historical span and independent closure
evidence. Future open records must have an owning task-tree leaf and declare `Direction`, `Kind`,
`Status`, `Owner`, and `Evidence`; detailed designs remain in task/research records and reproducible
bugs remain in indexed issue bundles. `TOOLBOX.md` now teaches that exact workflow.

The data contract is now `migrated`. Its real check authenticates the capsule regions and record spans,
all closure evidence and 26 consumers, current-truth evidence, root H1/H2 order/literals/markers/open
schema/register/bounds, and archive index/manifest. The live-document registry replaces the single
zero-growth transition record with a normal bounded current channel, a normal bounded archive index,
and an exact archive terminal. ADR 0011, the feedback fact, continuity ledgers, and both mdBook doctrine
explanations now describe the landed lifecycle; Rust, product behavior, roadmap direction, and FSMGen
submodule content are unchanged.

## `.5g.i` Executable Reviewed-Validation Boundary

### Root cause and authority

`VALIDATION_SNAPSHOT.md` is 544 lines / 63,517 bytes and has remained byte-identical since commit
`a44323d57f5f5f8ae3adad54ef48dad498434038` (`2026-06-10`). That boundary projects four IntentIR
reports and 29 rescan recommendations. It is intentionally a **last reviewed** validation surface, not
an automatic view of whichever git-ignored artifact happens to exist locally: `CANONICAL-PROMOTION-
SWEEP` explicitly withheld later promoted scores because they were not owner-approved.

The producer does not currently make that distinction executable. `project-validation` calls mutating
`validate::run` for every artifact, writes the local rescan plan, replaces the tracked snapshot, and
rewrites the live-status managed block. A doctrine check therefore cannot safely invoke it. The four
current IntentIR files also contain zero embedded validation reports. Three local sidecars still match
their reviewed snapshot fingerprint/score, but AHB has moved from reviewed `f2b3e0591b4a5fbc` /
`65/100` to unreviewed local sidecar `8b5311edb20f7998` / `63/100`. Treating host-local latest data as
the tracked authority would silently bypass the review gate; treating the old Markdown as unexplained
“latest” state is equally misleading.

### Decision and acceptance

`.5g.i` therefore defines snapshot currentness as exact agreement with a tracked, task-owned **reviewed
projection boundary**. A data-only contract must declare the source/review commit, four artifact paths,
stages, report fingerprints, scores/grades/finding counts, recommendation/execution counts, live-status
markers, review-gate evidence, relevant producer regions, snapshot identity, and independent limits.
The tracked root must say “last reviewed”, while unreviewed git-ignored artifacts remain explicitly
outside its truth authority.

The currency verifier must be read-only and must fail closed on unsafe/duplicate/missing inputs, root
identity or metrics drift, summary/report/recommendation mismatch, live-projection mismatch, producer-
region drift, review-evidence drift, schema/control overflow, and missing paths. Its focused fixtures
must stay under repository-local `generated/`, include positive plus each material negative path, and
leave zero residue. `LIVE-DOC-SIZE` must execute it unconditionally before the registry may move
`validation_snapshot.currency` from transition debt to enforced.

### Acceptance Checklist (enforced) — `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.i`

- [x] **ROOT CAUSE (WHY + WHERE)** — `project_validation.rs:327-364` calls mutating `validate::run`
  and writes three outputs, so it cannot be a doctrine checker; the tracked snapshot is byte-identical
  to reviewed commit `a44323d5`, while current artifacts carry 0 embedded reports and the AHB local
  sidecar moved `f2b3…` / `65` → `8b53…` / `63`. The explicit review-gate decision is in
  `CANONICAL-PROMOTION-SWEEP`.
- [x] **ADDRESSED (verified)** — the tracked contract and read-only checker prove the exact four-report,
  29-recommendation, zero-execution, snapshot, live-block, producer, and review-evidence boundary; ten
  fail-closed cases plus residue rejection pass, neutral producer wording is tested, and the registry
  ratchet executes under `LIVE-DOC-SIZE`.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, the real `project_validation` writer integration
  test, Clippy across all workspace targets with warnings denied, the ten-case oracle, and the composed
  34-surface live-document gate pass. Full resulting-tree `run_ci.sh` also passes 1,724 Rust tests with
  five ignored, rustdoc, and the repository-local mdBook build; extraction, canonical artifacts, and
  validation scores remain byte-untouched.

### Implemented boundary

`doctrine/live_document_size/validation_snapshot.json` is the sole tracked authority for this
projection. It pins the 544-line / 63,628-byte snapshot at SHA-256 `69467b06…b45a`, four ordered
artifact/report identities, 29 recommendation headings, zero execution summaries, the exact managed
live-status block, reviewed source commit and task evidence, four Rust producer regions, and independent
control limits. The snapshot's report content is unchanged; only its introduction now explains the
review gate. `project_validation.rs` emits the same neutral wording on a future reviewed refresh.

`scripts/check_validation_snapshot_currentness.pl` validates that tracked boundary without invoking
validation or reading ambient generated reports. Its ten cases cover valid state plus snapshot,
fingerprint, recommendation, live projection, producer, review evidence, unsafe path, duplicate
identity, and schema drift. Fixtures live under guarded repository-local `generated/` paths; cleanup
is exception-safe, error-checked, and residue-rejecting. The checker is unconditional in
`scripts/check_live_document_size.sh`, so `validation_snapshot.currency` is now `enforced` rather than
transition debt. ADR 0012 and the Knowledge Map card preserve the authority distinction for future
refreshes and fresh-clone CI.

## `.5g.ii` Executable Source-PDF Registry Membership

### Root cause and authority

The bounded registry entered this slice at 60 lines / 7,066 bytes with 22 table rows. Git also tracks
exactly 22 `.pdf` files below `corpus/`; all 22 rows currently resolve, begin with `%PDF-`, name their
actual parent directory, and match the real `stable_stem` → `document_key` derivation. The defect is
missing execution, not stale membership: no checker prevented a PDF-only or row-only change, key/path
drift, duplicate identity, or a filename-normalization code change.

The table cannot define its own denominator because a PDF and row could disappear together. The
owner's larger host-local library and git-ignored generated artifact roots cannot define it because a
fresh clone does not contain them. ADR 0013 therefore makes the Git-indexed PDF set below `corpus/`
the exact durable membership authority. Class labels remain human description; membership, path,
parent directory, key, and file signature are mechanical.

### Implemented contract

`doctrine/live_document_size/source_pdf_registry.json` pins the resulting 63-line / 7,374-byte registry,
22-row/file denominator, required scope/addition wording, two exact Rust key-derivation regions, three
`SourceIR::build` seams, and independent control bounds. The registry rows are unchanged; the prose now
states the complete consecutive-separator algorithm, currentness route, and non-authority of local or
generated state while remaining below its line-health warning.

`scripts/check_source_pdf_registry_currentness.pl` compares `git ls-files` membership with the parsed
table, validates uniqueness/safe containment/regular files/parent columns/derived keys/PDF signatures,
and authenticates the Rust seams. Its 13 guarded repository-volume cases cover valid state plus root
identity, missing row, extra tracked PDF, untracked row, key drift, duplicate key/path, directory drift,
unsafe path, bad signature, producer drift, and unknown schema. Cleanup is exception-safe and residue-
rejecting. `LIVE-DOC-SIZE` runs the suite and real oracle unconditionally; the surface currency record
is now `enforced` under `.5g.ii`.

### Acceptance Checklist — `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.ii`

- [x] **ROOT CAUSE (WHY + WHERE)** — exact probe: registry 22 rows == 22 tracked corpus PDFs, with zero
  missing/extra/bad key/directory/signature rows; `source.rs` owns two unguarded derivation functions.
- [x] **ADDRESSED (verified)** — syntax, 13/13 focused cases, real 22/22 report, zero fixture residue,
  below-warning 63-line root, and composed 34-surface execution pass.
- [x] **NO REGRESSION** — staged scope proves zero PDF/Rust-source changes; five doctrines, full
  `run_ci.sh` (formatting, warning-deny Clippy, 1,724 Rust tests with five ignored, rustdoc, mdBook),
  Knowledge Map/fact/catalog derive-and-diff, exact book aggregate, and diff hygiene pass.

## `.5d.ii` Atomic Knowledge Map Projection Migration

The stable landing now fronts seven generated question shards. With the identity-oracle regression fact
included, 139 participating facts produce 981 globally unique question keys. `KNOWLEDGE_MAP.md` is 19
lines / 898 bytes / 93 maximum line bytes; the seven shards total 2,000 lines / 183,756 bytes, the
largest is 300 lines / 30,427 bytes, and the complete eight-file set is 2,019 lines / 184,654
bytes / 384 maximum UTF-8 line bytes. Every root shard link is direct, and every question entry links one
existing canonical fact. The current path/content identity is
`13ee50d3c2508d813fe5e22f51ebcdbe25bea54dccb3a47e46e461ef07bd06d7`.

The portable bundle keeps Bash 3.2 plus POSIX awk/core-tool compatibility. Its generator scans only
immediate Markdown members of configured fact directories, validates safe paths/ids/questions and
independent input limits, rejects duplicate ids or questions, sorts questions in C/UTF-8 byte order,
wraps them without truncation, packs whole entries, and verifies part/landing/aggregate bounds before
replacement. Temporary files are exact hidden non-Markdown siblings on the repository volume; all
parts render before installation, the landing moves after current parts, only exact obsolete shard
names are deleted, and traps remove residue. SHA-256 works through `sha256sum`, `shasum`, or OpenSSL.

The checker validates required fact metadata, regenerates the same logical tree beneath a guarded
`generated/.knowledge-map-check.<pid>` workspace, compares landing content, exact shard filenames, and
every shard byte, then removes the workspace. The hook calls `--print-output-paths`, whose sorted union
includes the current set plus tracked obsolete shards, and applies `git add -A` to each path; deletions
cannot be stranded. Eight repository-local integration cases prove rollover, a clean sync, drift
failure, collision failure without output replacement, obsolete cleanup, deletion-aware staging,
aggregate-bound failure without output replacement, invalid-name rejection, and zero residue.

The first generator/simulator comparison exposed a foundational identity defect before classification:
`decode_utf8($raw, 1)` may consume `$raw`, so the Perl preflight hashed empty post-decode scalars. It
now hashes raw bytes before decoding a copy. The focused suite is 9/9 and the simulator/generator
identities match exactly; a content change can no longer leave the identity unchanged.

The live `fact_index` surface is now a normal `generated_projection` collection with complete landing
membership and executed freshness. Health is 16 files / 3,072 lines / 262,144 aggregate bytes; hard
limits are 33 files / 384 lines and 49,152 bytes per part / 4,096 aggregate lines / 393,216 aggregate
bytes / 512 maximum line bytes. The one-commit exact authority covers only the topology-driven file
1→33 and aggregate-line 2,244→4,096 increases; per-file lines/bytes/width and aggregate bytes all
ratchet sharply down. No old monolith or generated metadata copy remains.

All `.5d.i` readers migrated in the same slice: bootstraps, README, continuity/commit standards,
portable bundle architecture/FAQ/install/config/hook/CI, repository hook/driver, fact browse guidance,
ADR/fact, live doctrine/registry, and both mdBook governance chapters. Root-relative commands remain
valid after repository moves. Rust code, product behavior, roadmap direction, and submodule content are
unchanged.

## `.5d.i` Bounded Knowledge Map Projection Contract

The committed `.5c.iii` baseline is one generated file at 2,231 lines / 1,038,010 bytes / 7,778
maximum content-line bytes. It contains 138 facts and 978 generated question rows, but only 977
unique question strings. The exact duplicate asks why register bit-fields do not appear in emitted
ISF and points both to a broad completeness scorecard and the focused lowering-gap fact. The focused
card is the canonical destination; removing the duplicate answer from the broad card changes no
durable fact and lets the new gate reject every future collision.

The pressure is projection repetition, not canonical knowledge. Before the duplicate correction,
question text accounts for 102,661 bytes and links for 85,050, while repeated date/reverify metadata
accounts for 595,555. The question section is 981 lines / 795,972 bytes; the generated fact section is
1,243 lines / 241,692 bytes even though `docs/knowledge/INDEX.md` already provides the bounded fact
browse route. The migration therefore preserves every unique question and direct canonical link but
does not preserve redundant presentation.

ADR 0009 and `doctrine/knowledge_map/shard_contract.json` lock this topology before output changes:

| Element | Deterministic contract |
| --- | --- |
| stable landing | `KNOWLEDGE_MAP.md`; at most 96 lines / 8,192 bytes / 256 bytes per line |
| question shards | sequential `docs/knowledge-map/questions-NNNN.md`; whole entries sorted by UTF-8 bytes |
| shard bounds | at most 384 lines / 49,152 bytes / 512 bytes per line; at most 32 shards |
| input bounds | at most 200 facts / 2,048 unique questions; 96-byte ids, 192-byte paths, 2,048-byte questions |
| retrieval | one direct canonical fact link per globally unique question; question text wraps at 480 bytes |
| fact browse | direct landing link to the separately gated `docs/knowledge/INDEX.md`; no generated fact-copy section |
| identity | SHA-256 over every participating repository-relative canonical path and source-content hash |

The executable simulation parses the same `docs/knowledge/*.md` and `docs/decisions/*.md` fact
contract without writing output. With this slice's fact card, it proves 139 facts / 980 unique
questions fit in six shards. The landing simulation is 18 lines / 832 bytes / 93 maximum line bytes;
the largest shard is 384 lines / 35,724 bytes / 480 maximum line bytes; all shards total 1,991 lines /
183,441 bytes. The preflight reported canonical-input identity
`97242603370b856aa3e4afef1bbb9d086a6da0488466319e007c40e4679bb642`; `.5d.ii` later proved that
value identified paths plus empty post-decode scalars, not source content, and superseded it with the
raw-byte-first identity above. Eight focused cases at this boundary cover
block and inline answers, collision rejection, UTF-8 byte order, wrapping, question-byte overflow,
shard rollover, and landing overflow. The common lifecycle suite adds a valid indexed generated set
and a stale-shard-index failure, reaching 55 cases.

The `.5d.ii` reader/writer migration is closed, not open-ended. Bootstrap and contribution readers
are `AGENTS.md`, `CLAUDE.md`, `README.md`, `MEMORY.md`, `MEMORY_ARCHITECTURE.md`, and `COMMIT.md`.
Portable-bundle readers are its README, architecture, FAQ, installer, hook snippet, CI snippet, and
configuration. Executable writers/checkers are the bundle generator and checker, the repository
pre-commit hook, the doctrine driver/adapter, the fact-card catalog pointer, the live-document surface
registry, and the mdBook live-doc/doctrine chapters. `.5d.ii` must update every topology-sensitive
literal atomically, keep the portable bundle POSIX shell + awk, write all outputs through
repository-local same-directory temporary paths, expose the complete output-path set for staging,
remove only obsolete generated shards, and prove derive-and-diff plus exact root membership. No reader
may be left assuming that all questions or the repeated fact catalog remain inline in the landing.

This design slice does not change generated output topology or reclassify the live surface: the
monolith remains current transition debt until `.5d.ii`. It adds only the executable feasibility
contract, fail-closed collection lifecycle support, the accepted ADR/fact, the collision correction,
and synchronized design guidance. No ceiling increase or migration authority is requested.

## `.5c.iii` Remaining Canonical Collection Navigation

The clean-tree entry census found exactly eight `partitioned_canonical` surfaces still using
`git:query`:

| Surface | Markdown members | Direct-route disposition |
| --- | ---: | --- |
| workflow standards | 11 | generated external catalog; one titleless legacy file uses its filename |
| root architecture | 4 | generated external catalog |
| FSMGen issue packets | 7 | generated external catalog |
| research records | 38 | generated external catalog, including the grounding subtree |
| task evidence | 122 | preserve the existing 121-real-tree catalog plus separate template link |
| corpus knowledge base | 24 | generated external catalog; `.5g.iii` separately owns producer currency |
| KG fixture documents | 157 | generated external catalog; no fixture content changes |
| Knowledge Map bundle | 4 | make its existing README the complete direct membership index |

All 367 member files exist inside the repository and only `SESSION_BOOTSTRAP.md` lacks an H1. The
largest member H1 is 139 UTF-8 bytes (task evidence); the longest member path is 143 bytes (KG
fixtures). Catalog rendering can therefore use conservative 240-byte title, 192-byte path, and
512-byte row bounds without truncating current metadata. The catalog plane is a separate bounded
surface so a 157-row KG-fixture index does not inherit the fixtures' intentionally small per-file
limit or become benchmark input.

Implementation remains navigation-only: a data-only registry names six surface/index/label mappings;
one deterministic generator/checker reads the canonical surface targets, expands repository-relative
Markdown membership, emits six indexes plus their root index, and proves derive-and-diff. The generic
checker will accept `external_membership` only for a safe existing Markdown index and will apply the
same complete direct-link proof as internal membership. Focused fixtures must prove the positive path,
missing external index, missing member link, and off-root index rejection before registry migration.

The resulting catalog plane contains seven bounded files:

| Product | Files / members | Lines | Bytes | Max content line |
| --- | ---: | ---: | ---: | ---: |
| catalog root | 6 catalog routes | 13 | 535 | 85 |
| six member catalogs | 241 canonical routes | 301 | 48,770 | 317 |
| whole plane | 7 files | 314 | 49,305 | 317 |

The largest part is the 157-route KG-fixture index at 167 lines / 34,615 bytes. The generator owns at
most eight configured catalogs, 256 members per catalog, 65,536 bytes per member index, 8,192 root-
index bytes, 512 row bytes, 192 path bytes, 240 H1-title bytes, and 80 label bytes. The seven-line /
835-byte JSONL registry is independently self-bounded. Eight in-memory cases cover H1 extraction,
filename fallback, table escaping, unsafe/oversized paths, oversized titles, member overflow, and
label overflow; all output replacement is same-directory atomic.

The generic checker now accepts `external_membership` only when the index is a safe `.md` path outside
the member surface, is classified by another surface, exists, and directly links every member. Four
new fixtures cover stale, missing/unclassified, off-root, and incorrectly internal indexes, bringing
the common lifecycle/control suite to 53. The existing task generator supplies stronger exactness for
its 122-file external route; the portable bundle README supplies direct internal membership for its
four files. A registry query finds zero remaining `git:query` collection contracts. No canonical
member or collection currency claim changed, and no ceiling-increase authority is required.

## `.5c.ii` Decision And Fact-Card Navigation

Entry evidence shows two deliberately different durable stores. `docs/decisions/INDEX.md` is already
a concise human catalog and its registered membership gate covers every ADR. `docs/knowledge/` has
136 Markdown files but only 135 atomic fact cards plus the collection README. Their only routes are
`git:query` and the 2,207-line generated question map; a human cannot open one bounded collection
route and select a card by id/title. The map's 136-fact total includes ADR 0007 from the separately
indexed decision collection, so it is not a directory-card count.

This slice preserves the decision index and reproves its membership. It adds a derived index inside
`docs/knowledge/`, excluding that index itself from fact inputs. The fact index is navigation only:
one id/title/date/status row links each canonical card, while questions, evidence, reverify commands,
and bodies stay in the cards and generated map. The generic collection membership gate and a focused
derive-and-diff checker must both cover the resulting route. This slice's own durable fact raises the
result to 136 cards plus the README and generated index, while the generated map reaches 137 facts
because ADR 0007 remains its additional cross-layer input.

The resulting `docs/knowledge/INDEX.md` is 150 lines / 28,392 bytes / 251 maximum line bytes. Its
focused ceiling is 160 cards, 32,768 index bytes, 320 row bytes, 64 id bytes, 1,024 source-title bytes,
112 rendered-title bytes, 64 questions per card, and 2,048 bytes per question. Historical titles over
the rendered allowance are UTF-8-safe previews ending in an ellipsis; the canonical title is never
rewritten. Nine parser/bound cases cover default and explicit status, inline/block questions,
Markdown escaping, missing questions, id mismatch, invalid date, missing evidence, oversized title,
and compaction.

The complete decision index remains unchanged at 30 lines / 2,617 bytes / 222 maximum line bytes;
an exact direct comparison proves all nine ADR members appear once and no extra Markdown member route
exists. The root README exposes the fact catalog beside the generated question map. Its route registry
now points the healthy task, decision, and fact browse paths to the composed live-document checker
instead of retaining stale transition labels. The `.5c.i` ceiling authority was consumed by its
commit and removed; no ceiling increase is requested or banked by this slice.

## `.5c.i` Bounded Complete Task Catalog

The migration replaces historical-status prose in the index with data derived from each canonical
task file:

| Dimension | Before | After | Contract |
| --- | ---: | ---: | --- |
| real task routes | 117 of 121 | 121 of 121 | every `docs/tasks/*.md` except `TEMPLATE.md`, exactly once |
| index lines | 362 | 375 | fixed workflow plus one concise row per real tree |
| index bytes | 167,583 | 27,228 | 83.8% reduction |
| maximum content line | 18,932 | 207 | 98.9% reduction; row ceiling 512 |

`scripts/check_task_tree_catalog.pl` reads only safe immediate task filenames, requires filename/H1
identity, extracts the first closed-vocabulary metadata status, escapes Markdown title separators,
sorts by tree id, and derives the managed section. `--write` uses a same-directory atomic replacement;
`--check` is derive-and-diff. `TEMPLATE.md` has a separate authoring link and is never counted as work.

The checker is composed into `LIVE-DOC-SIZE` for the real repository and runs seven in-memory parser/
bound cases before the 121-tree check. It owns a maximum of 160 rows, a 49,152-byte managed section,
240-byte source titles, and 512-byte rendered rows. The generic surface checker independently owns
the whole file's 512-line / 65,536-byte / 512-max-line limits.

This changes navigation only. No task file, status, frontier, decision, verification record, or commit
record is synthesized or moved; users open the canonical tree after selecting its concise route.

## `.5b` Collection/Projection Census And Dependency Plan

The post-`.4` resulting tree was measured before any collection content moved:

| Surface | Entry measurement | Retrieval/currentness finding | Owning leaf |
| --- | --- | --- | --- |
| task catalog / task evidence | index 362 lines / 167,583 bytes / max line 18,932; 122 files / 30,640 lines / 2,280,859 bytes | 117 unique links; four real trees missing; verbose rows mirror history | `.5c.i` |
| decisions | 10 files / 591 lines / 35,949 bytes | nine ADRs all linked exactly once from bounded index | preserve; reprove in `.5c.ii` |
| fact cards / generated map | 134 cards / 6,782 lines / 615,483 bytes; map 2,183 lines / 1,032,740 bytes / max line 7,836 | cards have query-only route; generated root is fresh but unbounded transition debt | `.5c.ii`, then `.5d.i`–`.5d.ii` |
| research | 38 files / 5,893 lines / 399,504 bytes | bounded parts but only `git:query`, no human one-hop catalog | `.5c.iii` |
| roadmap | 1,487 lines / 183,445 bytes / max line 717 | exact zero-growth debt; current direction mixed with delivery chronology | `.5e.i`–`.5e.ii` |
| FSMGen feedback | 936 lines / 57,980 bytes / max line 1,705 | exact zero-growth debt; request/answer/resolution grammar not yet locked | `.5f.i`–`.5f.ii` |
| validation snapshot | 544 lines / 63,517 bytes | real producer exists; currentness is labelled debt and live size is at warning | `.5g.i` |
| source PDF registry | 60 lines / 7,066 bytes | bounded; tracked-corpus membership/document-key oracle missing | `.5g.ii` |
| corpus KB | 24 files / 1,784 lines / 119,240 bytes | managed writers/tests exist; currency is labelled debt; max-line width warns | `.5g.iii` |
| root references / mdBook | 4 roots / 1,340 lines / 68,206 bytes; book 33 files / 11,978 lines / 715,829 bytes | roots need uniqueness/route audit; book already has direct membership and executed currency | `.5h`, then `.5i` |

The order prevents circular authority: a bounded task catalog exists before other leaves rely on it;
fact navigation exists before the generated question map changes topology; source identities and
semantic boundaries precede roadmap/feedback movement; producer oracles are non-mutating; the final
maintained-reference audit sees the resulting routes rather than designing against temporary ones.

The census itself moved no canonical collection record. The only data-plane adjustment settles the
now-complete four-ledger archive index's health target inside its unchanged enforcement ceiling.

## `.4e` `RUST_CODEBASE_ANALYSIS.md` Atomic Migration

The final migration closes the protocol across all four roots without manufacturing architecture
content:

| Product | Records | Lines | Bytes | Max content line | Identity / role |
| --- | ---: | ---: | ---: | ---: | --- |
| pre-migration source capsule | 1,350 | 9,039 | 1,046,679 | 3,041 | immutable SHA-256 `95e1…cc7c`; complete architecture source |
| final root | 60 | 1,056 | 89,727 | 369 | exact H1/Purpose prologue + newest H2 records |
| records outside current view | 1,290 | — | — | — | retrieved from the capsule/index |

The exact copy passed `cmp`, digest, line, and byte checks before the guarded emitter touched the
root. The H2 boundary renderer reused `.4c`'s successor-owned terminal-separator rule; retained
content and prologue remain exact. The common manifest/index now routes all four source capsules,
`rust_analysis` is normal, and `rust_analysis_archive` is an exact archive terminal.

`COMMIT.md` and `SESSION_BOOTSTRAP.md` remain verified consumers. Because the migration changed no
Rust architecture, `.4e` deliberately did not prepend a migration record to this semantic surface.
That preserves impact-based documentation routing while the other durable layers record the complete
lifecycle transition.

## `.4d` `LIVE_ACHIEVEMENT_STATUS.md` Atomic Migration

The third migration preserves a live writer seam as well as a whole-record boundary:

| Product | Records | Lines | Bytes | Max content line | Identity / role |
| --- | ---: | ---: | ---: | ---: | --- |
| pre-migration source capsule | 1,920 | 1,960 | 581,239 | 5,467 | immutable SHA-256 `b00f…bd6a`; complete status source |
| retained capsule view in root | 50 | 89 | 87,505 | 4,824 | H1/current heading + newest bullets + unchanged trailer |
| final root including `.4d` | 51 | 90 | 88,414 | 4,824 | bounded current snapshot; first post-capsule prepend |
| records outside current view | 1,870 | — | — | — | retrieved from the capsule/index |

The exact copy passed `cmp`, digest, line, and byte checks before the guarded emitter touched the
root. The emitter shortened only the bullet region between the exact snapshot/gap headings; the gap,
validation heading, projection markers, and generated content stayed outside rotation. The common
manifest/index now routes the exact capsule, `achievement_status` is normal, and
`achievement_status_archive` is an exact archive terminal.

The real Rust writer seam was exercised through
`project_validation_writes_snapshot_doc_and_updates_live_status` with `TMPDIR` under repository-local
`generated/tmp`. It produced and replaced the managed projection in a same-volume repository fixture,
confirming that the stable root and markers still satisfy production code rather than only a text
probe.

## `.4c` `DEVELOPMENT_NOTES.md` Atomic Migration

The second migration reused the proved control path and independently froze its own semantic boundary:

| Product | Records | Lines | Bytes | Max content line | Identity / role |
| --- | ---: | ---: | ---: | ---: | --- |
| pre-migration source capsule | 1,601 | 20,921 | 2,170,230 | 1,401 | immutable SHA-256 `76b5…fedc`; complete rationale source |
| retained capsule suffix in root | 60 | 1,472 | 193,809 | 1,401 | H1 prologue + newest 60 H2 records; successor separator omitted at EOF |
| final root including `.4c` | 61 | 1,480 | 194,412 | 1,401 | bounded rationale view; first post-capsule prepend |
| records outside current view | 1,541 | — | — | — | retrieved from the capsule/index |

The exact copy passed `cmp`, digest, line, and byte checks before the guarded emitter touched the root.
The common archive manifest gained one bounded source-capsule record, and the index gained a direct
root/capsule route. `engineering_notes` is now normal; `engineering_notes_archive` is an exact archive
terminal. Retrieval also proves the still-current `COMMIT.md` writer route.

The only failed intermediate check was useful: after adding the first new H2 record, suffix identity
reported survivor 60 changed. A byte diff showed that the blank before removed record 61 had become a
terminal ambiguity. The root cause was fixed in the shared renderer/verifier: an H2 cut omits exactly
one successor-owned separator at EOF and accepts no other retained-content drift. The capsule was not
changed to make the check fit, and `git diff --check` now passes without weakening whitespace policy.

## `.4b` `CHANGES.md` Atomic Migration

The migration followed copy/verify/use without relying on the disappearing root as its backup:

1. `CHANGES.md` still matched `.4a`'s pinned SHA-256 and dimensions.
2. A same-volume exact copy was created at the declared capsule path; `cmp`, SHA-256, lines, and bytes
   matched before any root rewrite.
3. The guarded emitter refused to operate without that capsule, then atomically replaced the root
   from the parsed 85-record prefix plus the exact `.1`/`.0` promoted records.
4. The root prepended `.4b` as the first post-capsule record and remained within every live limit.
5. The bounded manifest and index recorded complete-source identity and current/history retrieval.
6. The rolling registry switched only `changes` to `migrated`; the general surface registry removed
   its transition debt and added exact index/archive classifications.

| Product | Records | Lines | Bytes | Max content line | Identity / role |
| --- | ---: | ---: | ---: | ---: | --- |
| pre-migration source capsule | 1,798 | 32,682 | 2,629,033 | 1,629 | immutable SHA-256 `d898…a994`; complete source |
| retained capsule suffix in root | 87 | 1,357 | 199,055 | 1,629 | newest 85 + exact `.1`/`.0` compatibility records |
| final root including `.4b` | 88 | 1,368 | 199,851 | 1,629 | bounded current view; first post-capsule prepend |
| records outside current view | 1,711 | — | — | — | retrieved from capsule/index; not copied into root |

The 87 capsule records intentionally overlap the exact source capsule. The manifest describes that
overlap; semantic closure and complete-source identity are therefore not misreported as disjoint
storage arithmetic. The archive index is 21 lines / 1,048 bytes and directly links the root and
capsule. `perl scripts/check_rolling_ledger_protocol.pl --report` verifies the full retrieval path.

## `.4a` Rolling-Ledger Census And Protocol

### Exact source and reviewed survivor

All figures below are re-derived by `perl scripts/check_rolling_ledger_protocol.pl --report`. Source
SHA-256 values are pinned in the bounded registry; the table keeps the independently meaningful
working-set dimensions readable. “Archived records” is the count outside the future live view, not a
claim that `.4a` already moved them.

| Ledger | Source records / lines / bytes / max line | Planned live records / lines / bytes / max line | Records outside live view | Live limits: records / lines / bytes / max line |
| --- | --- | --- | ---: | --- |
| `CHANGES.md` | 1,798 / 32,682 / 2,629,033 / 1,629 | 87 / 1,357 / 199,055 / 1,629 | 1,711 | 128 / 2,000 / 262,144 / 2,200 |
| `DEVELOPMENT_NOTES.md` | 1,601 / 20,921 / 2,170,230 / 1,401 | 60 / 1,472 / 193,809 / 1,401 | 1,541 | 96 / 2,000 / 262,144 / 1,800 |
| `LIVE_ACHIEVEMENT_STATUS.md` | 1,920 / 1,960 / 581,239 / 5,467 | 50 / 89 / 87,505 / 4,824 | 1,870 | 80 / 640 / 131,072 / 6,800 |
| `RUST_CODEBASE_ANALYSIS.md` | 1,350 / 9,039 / 1,046,679 / 3,041 | 60 / 1,056 / 89,727 / 369 | 1,290 | 96 / 1,400 / 196,608 / 600 |

Every selected survivor begins below 80% of each local limit, independently. The values are derived
from the reviewed retained records and their largest normal transaction; they are not donor values or
the old bloated measurements relabelled as health. A migrated live root reaching 90% must roll over.

The `CHANGES.md` survivor is the newest 85 records followed by the exact `.1` and `.0` detached
records. The status survivor includes its two-line prologue, 50 current bullets, one separating blank
line, and the unchanged 37-line trailer; pressure arithmetic therefore covers the writer-managed
block rather than counting only bullets. The architecture survivor keeps its six-line Purpose
prologue, which is why its live line count is greater than the sum of its 60 record bodies alone.

### Grammar and consumer closure

| Ledger | Parser boundary | Load-bearing writers/readers preserved |
| --- | --- | --- |
| `CHANGES.md` | leading modern `###` records; legacy `##` records after the first H2; two exact detached H3 records | `COMMIT.md`; README and mdBook routes |
| `DEVELOPMENT_NOTES.md` | every H2 after the H1 prologue | `COMMIT.md`; README and mdBook routes |
| `LIVE_ACHIEVEMENT_STATUS.md` | each `- ` line between exact Current-snapshot and gap markers | `COMMIT.md`; `project_validation.rs` stable root and managed markers; README and mdBook routes |
| `RUST_CODEBASE_ANALYSIS.md` | every H2 after the H1 + `## Purpose` prologue | `COMMIT.md`; `SESSION_BOOTSTRAP.md`; README and mdBook routes |

The registry stores only repository-relative paths and fails closed on unknown fields, schema
version, record/file bytes, arrays, and scalars. Each plan names the common bounded archive manifest
and index plus a ledger-specific capsule path under `docs/archive/rolling-ledgers/`; planned paths are
not treated as existing archives.

### Atomic migration and retrieval contract

For `.4b`–`.4e`, one commit must copy the exact pinned source to its capsule, create or update the
self-bounded manifest/index, derive the root survivor solely from parsed whole records, switch the
ledger record from `planned` to `migrated`, add archive/current surface classifications, execute
retrieval, and only then remove the historical bytes from the root. The migrated checker requires:

- capsule SHA/record/line/byte/max-line identity equal to the pinned source;
- manifest source/path/date/reason/verifier and first/last record identity;
- index references to the ledger id, stable root, and capsule;
- root pressure below the rollover threshold and all grammar/trailer invariants;
- the originally retained record bytes as an exact suffix behind any later prepended records.

Tool-neutral retrieval is `perl scripts/check_rolling_ledger_protocol.pl --report`. A capsule is an
archive terminal and cannot receive ordinary appends. Subsequent corrections supersede sealed
evidence; they never mutate it.

## `.3a` Complete Markdown Census and `.3b` Contract

### Resulting-tree census at `.3a` entry

The inventory source is `git ls-files '*.md'` from the parent SpecForge repository. That makes the
pinned `subs/fsmgen` Git authority out of scope automatically while still covering every tracked
SpecForge Markdown path. The entry snapshot contains **550 files / 132,211 lines / 12,253,499 bytes**.
Its largest file is `CHANGES.md` at 32,626 lines / 2,624,882 bytes; its widest content line is
`docs/TASK_TREE.md` at 18,932 bytes. Recording this design changes task/index bytes but not membership;
the required coverage-authority fact card adds one classified path, so the committed `.3a` set is
551 files. `.3b` must remeasure its final resulting tree when it pins enforcement values.

| Exclusive path family | Files | Lines | Bytes | Largest file (lines) | Largest file (bytes) | Widest line (bytes) |
| --- | ---: | ---: | ---: | --- | --- | --- |
| `.github/` | 1 | 15 | 918 | 15 | 918 | 91 |
| `corpus/` | 1 | 60 | 7,066 | 60 | 7,066 | 278 |
| `corpus_kb/` | 24 | 1,784 | 119,240 | 940 | 30,837 | 1,062 |
| `crates/specforge/test_data/` | 157 | 1,588 | 29,650 | 61 | 3,592 | 169 |
| `docs/book/` | 33 | 11,826 | 704,974 | 1,986 | 98,504 | 2,030 |
| `docs/decisions/` | 8 | 419 | 24,648 | 71 | 4,675 | 224 |
| `docs/fsmgen-issues/` | 7 | 285 | 15,756 | 53 | 3,554 | 291 |
| `docs/knowledge/` | 130 | 6,656 | 607,825 | 243 | 29,329 | 5,031 |
| `docs/research/` | 38 | 5,893 | 399,504 | 493 | 36,790 | 434 |
| `docs/tasks/` | 122 | 30,199 | 2,237,705 | 2,393 | 222,616 | 4,946 |
| top-level `docs/*.md` | 4 | 1,447 | 236,187 | 936 | 167,588 | 18,932 |
| `knowledge-map/` | 4 | 417 | 22,031 | 218 | 11,299 | 164 |
| repository-root Markdown | 21 | 71,622 | 7,847,995 | 32,626 | 2,624,882 | 7,836 |

The 13 families above are exhaustive and mutually exclusive. `.3b` will refine the heterogeneous
root and top-level-`docs` families into exact records but may use family patterns for homogeneous
collections. Coverage succeeds only when every tracked Markdown path matches exactly one record.

### Local lifecycle classification

| Surface | Lifecycle | Control contract |
| --- | --- | --- |
| `README.md`, `MEMORY.md`, roadmap/status snapshots, bootstrap/workflow standards | `bounded_snapshot` | independent per-file line/byte/max-line ceilings; existing dedicated check remains authoritative where stronger |
| `CHANGES.md`, `DEVELOPMENT_NOTES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `docs/FSMGEN_FEEDBACK.md` | `rolling_ledger` | exact `.3b` transition baseline and zero growth until the `.4` whole-record migration supplies shards/index/archive manifest |
| task trees, decisions, fact cards, research, issue packets, fixtures, corpus KB | `partitioned_canonical` | complete index/query contract plus per-file, file-count, aggregate-line, aggregate-byte, and max-line ceilings |
| mdBook | `maintained_reference` | bounded `SUMMARY.md`, one-hop membership, per-part review ceilings, and exact task-owned aggregate-change authority rather than a decorative aggregate cap |
| `KNOWLEDGE_MAP.md` | `generated_projection` | size ceilings plus the existing derive-and-diff freshness verifier over fact cards/decisions |
| dated closed snapshots or later archive artifacts | `frozen_legacy` / `archive_terminal` | content hash or immutable manifest; never an author-overflow destination |

`USER_GUIDE.md` and the architecture/spec roots are maintained references until `.5` proves whether
their content is uniquely current or duplicated by the mdBook; they are not deleted by classification.
At `.3a`, `VALIDATION_SNAPSHOT.md` and `corpus/SOURCE_PDF_REGISTRY.md` were classified as bounded
current snapshots with missing-verifier transition debt rather than fabricated freshness claims.
`.5g.i` later bound validation to its reviewed producer boundary; `.5g.ii` bound the manually maintained
registry to Git-indexed corpus membership and the code-owned key derivation.

### Registry and checker contract for `.3b`

- Project-owned data lives at `doctrine/live_document_size/surfaces.jsonl`. Record zero is a registry
  control record with schema version plus registry file/record byte and record-count ceilings.
- Every surface record carries: stable id, repository-relative targets, file/collection locator,
  closed lifecycle, state, owner, index/query contract where applicable, verifier, health targets,
  enforcement ceilings, warning/rollover milestones, and optional baseline/transition/currency/
  maintained-reference/frozen/archive data required by its lifecycle.
- Limit dimensions are independent: file count, lines/bytes per file, aggregate lines/bytes, and
  max content-line bytes. `null` means inapplicable only where the lifecycle supplies a distinct
  authority contract; it never silently means unlimited.
- Healthy surfaces receive modest, explicitly documented headroom derived from the local survivor.
  Existing breaches receive their final `.3b` resulting-tree measurement as an immutable debt
  baseline with zero growth. No donor value is a SpecForge default.
- The checker resolves its root from its own path, accepts an explicit `--root` for same-volume test
  fixtures, reads the resulting tree unconditionally, rejects absolute/parent/off-root targets,
  proves exactly-once tracked-Markdown coverage, applies lifecycle-required fields and all dimensions,
  verifies controlled indexes/freshness/hashes, and invokes the existing README route guard.
- `scripts/check_live_document_size.sh` is registered as `LIVE-DOC-SIZE` in the one doctrine driver in
  the same commit that adds the normative root doctrine and decision record. `.3c` then exercises one
  positive and at least one negative path for every lifecycle plus registry overflow, uncovered/double-
  covered files, off-root targets, missing/stale index, missing freshness proof, modified frozen hash,
  route mismatch, and independent dimension overflow.
- Ceiling increases require a data-only authority record naming the work-unit, old/new values, owner,
  and rationale; the checker compares against the committed registry when Git history is available.
  A first adoption baseline is not an increase. Decreases need no exception and become the new ceiling.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-08` | `.0` | full README/roadmap/code/mdBook ramp-up; all 14 active task-tree frontiers; FSMGEN README policy, adoption guide, and neutral doctrine read; `wc` + max-line + route/locality census; Git/submodule state | measured; adoption is justified; no policy/content migration performed |
| `2026-08-08` | `.0` | `bash scripts/check_doctrines.sh`; `mdbook build docs/book`; `git diff --check` | green: 3/3 doctrines; book built under repository-local `generated/mdbook/specforge`; clean diff |
| `2026-08-08` | `.1` | `bash -n scripts/check_memory_architecture.sh`; `bash scripts/check_doctrines.sh`; forced line/byte/max-line cap probes; `mdbook build docs/book`; `git diff --check` | green: current pointer passes 50 lines / 4,096 bytes / 160 max-line bytes + required fields + no HEAD shadow; all three forced caps fail closed; doctrines 3/3; book and diff clean |
| `2026-08-08` | `.2` | `git show ad07eb3f:README.md` heading/size probes; canonical mdBook heading probes; retained `wc`/max-line census; route extraction | old 602 lines/170,891 bytes → survivor 127 lines/4,834 bytes (max line 103 chars); duplicate section classes proven against richer homes; 21 unique reader routes registered |
| `2026-08-08` | `.2` | `bash -n scripts/check_readme_policy.sh scripts/check_doctrines.sh`; `bash scripts/check_readme_policy.sh --self-test`; `bash scripts/check_doctrines.sh`; `cargo run --manifest-path Cargo.toml -- --help`; `cargo run --manifest-path Cargo.toml -- doctor --strict`; `cargo run --manifest-path Cargo.toml -- inspect README.md` | green: line/byte, off-repository path, unknown-lifecycle/control, and missing-route probes fail closed; doctrines 4/4; current CLI resolves; repo-local Docling selected; inspect recognizes the retained README as a 4,834-byte Markdown file. Doctor correctly reports the currently stopped local model servers with recovery guidance. |
| `2026-08-08` | `.2` | neutral-body `diff -u` against the reviewed FSMGen policy; `bash knowledge-map/scripts/gen_knowledge_map.sh`; `bash scripts/run_docs_ci.sh`; `git diff --check`; pointer/README/policy census | green: neutral body byte-equivalent after the fenced local note; derived map 129 facts/947 question keys; mdBook writes under repository-local `generated/mdbook/specforge`; diff clean; pointer and README remain within enforced bounds |
| `2026-08-08` | `.3a` | parent `git ls-files '*.md'` exactly-once family census; per-family/per-file `wc` and byte-width scan; submodule authority check | 550 files / 132,211 lines / 12,253,499 bytes at entry; 13 mutually exclusive families; largest file `CHANGES.md` 32,626 lines / 2,624,882 bytes; widest line `docs/TASK_TREE.md` 18,932 bytes; submodule excluded by parent index |
| `2026-08-08` | `.3a` | lifecycle/route/storage topology review against the fully read neutral doctrine and adoption guide; `perl -MJSON::PP` availability probe; Knowledge Map regeneration; `bash scripts/check_doctrines.sh`; `bash scripts/run_docs_ci.sh`; `git diff --check` | local JSONL schema/checker contract locked; core parser available; map 130 facts/950 question keys; active doctrine unchanged pending atomic `.3b`; doctrines 4/4, docs, and diff green |
| `2026-08-08` | `.3b` | neutral-body `diff -u` against reviewed doctrine; `perl -c scripts/check_live_document_size.pl`; `bash -n scripts/check_live_document_size.sh`; complete checker `--report`; final `bash scripts/check_live_document_size.sh` | neutral body byte-equivalent after local note; 553 Markdown files classified exactly once across 24 surfaces; no off-root target, overlap, missing path, size-ceiling, lifecycle, index, route, freshness, frozen-hash, debt, reference-authority, or increase-authority violation |
| `2026-08-08` | `.3b` | Knowledge Map regeneration; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --check`; registry/authority raw-size census | green: map 131 facts/954 question keys; doctrines 5/5 including `LIVE-DOC-SIZE`; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build all pass; diff clean; both JSONL registries remain within their own record/file ceilings |
| `2026-08-08` | `.3c` | `perl -c scripts/check_live_document_size.pl scripts/test_live_document_size.pl`; direct 48-case fixture suite; `bash -n scripts/check_readme_policy.sh scripts/check_live_document_size.sh`; composed `bash scripts/check_live_document_size.sh` | green: positive and fail-closed paths cover all seven lifecycles, both index contracts, canonical inputs, generated/currency execution, archive manifests, frozen/reference/debt controls, registry schema/count/file/record/array/scalar bounds, exact coverage, locality, all six size dimensions, routes, and Git-history ceiling/baseline/maintained-reference authority; every workspace is under repository-local `generated/` and cleans itself |
| `2026-08-08` | `.3c` | staged resulting-tree `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check` | green: 554 Markdown files across 24 surfaces; 48/48 fixtures; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build all pass; staged diff clean |
| `2026-08-08` | `.5a` | code/book cross-read: `select_initiator_actor`, `initiator_perspective_directions`, CLI `ExtractContracts`, dispatch to `commands::extract_contracts::run`; full-chapter stale-phrase census; `bash -n scripts/check_book_current_truth.sh`; direct checker execution | verified root causes: initiator-relative direction lowering and live constrained extraction both ship; both stale formulations removed; canonical detail pointers resolve; focused currency verifier green |
| `2026-08-08` | `.5a` | staged resulting-tree checker; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check` | green: 555 Markdown files / 24 surfaces; book currency executes; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build all pass; staged diff clean |
| `2026-08-08` | `.4a` | Git-blame/order root cause; grammar/consumer census; `perl -c scripts/check_rolling_ledger_protocol.pl`; `perl scripts/check_rolling_ledger_protocol.pl --self-test`; real four-ledger `--report` | 6,667 records / 64,600 lines / 6,425,335 bytes reconstruct exactly; ten parser/control cases pass; all four planned survivors start below every 80% warning; status writer path/markers present; no record moved |
| `2026-08-08` | `.4a` | staged resulting-tree `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check` | green: 557 Markdown files / 24 surfaces; rolling-ledger checker runs unconditionally; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build all pass; staged diff clean |
| `2026-08-08` | `.4b` | exact pre-copy `--report`; `cp -p` to declared same-volume capsule; `cmp -s`; dual SHA-256 and `wc`; guarded `--emit-planned changes`; migrated `--report` | capsule and source identical at 1,798 records / 32,682 lines / 2,629,033 bytes / `d898…a994`; root mechanically 87 records before `.4b`, then 88; manifest/index/capsule identity and exact retained suffix pass |
| `2026-08-08` | `.4b` | capsule vs `git show 73424daf:CHANGES.md`; staged resulting-tree `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check` | byte-identical capsule; green: 559 Markdown files / 26 surfaces; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build all pass; staged diff clean |
| `2026-08-08` | `.4c` | exact pre-copy `--report`; same-volume `cp -p`; `cmp -s`; dual SHA-256 and `wc`; guarded `--emit-planned development-notes`; retained-suffix failure + byte/root-cause diff; terminal-boundary renderer/verifier correction; migrated `--report` | capsule/source identical at 1,601 records / 20,921 lines / 2,170,230 bytes / `76b5…fedc`; root 60 records before `.4c`, then 61; capsule exact, successor separator omitted only at live EOF, all record content/manifest/index/retrieval pass |
| `2026-08-08` | `.4c` | capsule vs `git show 2d7eaff1:DEVELOPMENT_NOTES.md`; staged resulting-tree `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check` | byte-identical capsule; green: 560 Markdown files / 27 surfaces; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build all pass; staged diff clean |
| `2026-08-08` | `.4d` | exact pre-copy `--report`; same-volume `cp -p`; `cmp -s`; dual SHA-256 and `wc`; guarded `--emit-planned live-achievement-status`; migrated `--report`; repository-local-TMPDIR focused writer test | capsule/source identical at 1,920 records / 1,960 lines / 581,239 bytes / `b00f…bd6a`; root 50 records before `.4d`, then 51; capsule, exact suffix, stable trailer, both markers, manifest/index, and 1/1 real writer test pass |
| `2026-08-08` | `.4d` | capsule vs `git show e4239e3f:LIVE_ACHIEVEMENT_STATUS.md`; staged resulting-tree `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check` | byte-identical capsule; green: 561 Markdown files / 28 surfaces; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build all pass; staged diff clean |
| `2026-08-08` | `.4e` | exact pre-copy `--report`; same-volume `cp -p`; `cmp -s`; dual SHA-256 and `wc`; guarded `--emit-planned rust-codebase-analysis`; migrated `--report` | capsule/source identical at 1,350 records / 9,039 lines / 1,046,679 bytes / `95e1…cc7c`; root exactly 60 records / 1,056 lines / 89,727 bytes; prologue, capsule, exact suffix, manifest/index, retrieval, and consumers pass |
| `2026-08-08` | `.4e` | capsule vs `git show 8310c3a1:RUST_CODEBASE_ANALYSIS.md`; staged resulting-tree `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check` | byte-identical capsule; green: 562 Markdown files / 29 surfaces; all four ledgers migrated; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build all pass; staged diff clean |
| `2026-08-08` | `.5b` | resulting-tree live-size `--report`; `wc`/max-line collection census; process-substitution task/decision membership comparison; producer/managed-marker code census; exact off-volume scratch deletion + absence proof | dependency graph locked from measured state; tasks 122 files vs 117 links with four real omissions + template; decisions 9/9 linked; map 134 cards / 1,032,740 bytes; roadmap/feedback exact debt pinned; two accidental `/private/tmp/specforge-task-*.txt` files absent after exact deletion; no canonical collection content moved |
| `2026-08-08` | `.5b` | Knowledge Map regeneration; staged resulting-tree `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check`; archive-index health/cap report | green: map 135 facts / 969 question keys; 563 Markdown files / 29 surfaces; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build pass; staged diff clean; archive index 58 lines / 3,283 bytes under 96/6,144 health and unchanged 128/8,192 ceiling |
| `2026-08-08` | `.5c.i` | filename/H1 and metadata-status census; pre-migration process-substitution membership diff; `perl -c`; seven-case `--self-test`; guarded `--write`; derive-and-diff `--check`; post-migration membership/pressure census | 121/121 real tasks linked exactly once; four missing routes recovered; template separate; old 362 lines / 167,583 bytes / max 18,932 → 375 / 27,228 / 207; syntax, 7/7 cases, safe atomic writer, ordering, escaping, and independent generator bounds pass |
| `2026-08-08` | `.5c.i` | Knowledge Map regeneration; staged resulting-tree catalog/checker; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check` | green: map 136 facts / 972 question keys; 564 Markdown files / 29 surfaces; task index normal; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, and repository-local mdBook build pass; staged diff clean |
| `2026-08-08` | `.5c.ii` | exact directory/front-matter denominator census; direct decision membership comparison; `perl -c`; nine-case `--self-test`; guarded `--write`; derive-and-diff `--check`; Knowledge Map regeneration | root-caused 136 entry files as 135 cards + README and 136 map facts as those cards + ADR 0007; resulting own fact gives 136 cards / 137 map facts; decision index 9/9 exactly once; catalog 150 lines / 28,392 bytes / max 251; syntax, 9/9 cases, atomic writer, ordering, title compaction, exact membership, and independent bounds pass |
| `2026-08-08` | `.5c.ii` | staged resulting-tree catalog/routes/authority cleanup; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; live-size `--report`; `git diff --cached --check` | green: 566 Markdown files / 29 surfaces; map 137 facts / 975 question keys; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, repository-local mdBook build, exact book aggregate 33 files / 12,030 lines / 719,303 bytes, and staged diff pass; knowledge-card membership is enforced and no ceiling authority remains banked |
| `2026-08-08` | `.5c.iii` | eight-surface query/H1/path census; `perl -c`; eight-case catalog `--self-test`; guarded `--write`; derive-and-diff `--check`; 53-case lifecycle/control suite; zero-query registry probe | 367 canonical members classified; six generated catalogs cover 241, task catalog 122, bundle README 4; catalog plane 7 files / 314 lines / 49,305 bytes / max 317; syntax, 8/8 catalog cases, 53/53 common fixtures, exact ordering/target expansion, external membership, all independent bounds, and zero query-only surfaces pass |
| `2026-08-08` | `.5c.iii` | staged resulting-tree catalogs/schema/routes; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; live-size `--report`; `git diff --cached --check` | green: 574 Markdown files / 30 surfaces; map 138 facts / 978 question keys; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, repository-local mdBook build, exact book aggregate 33 files / 12,051 lines / 720,903 bytes, and staged diff pass; catalog surface exactly 7 files / 314 lines / 49,305 bytes and no ceiling authority is required |
| `2026-08-08` | `.5d.i` | committed-baseline pressure/decomposition census; duplicate-question audit; ADR/contract review; Perl syntax; eight-case shard `--self-test`; real feasibility `--check`/`--report`; 55-case lifecycle suite; current Knowledge Map and fact-catalog derive-and-diff | baseline 138 facts / 978 rows / 977 unique questions at 2,231 lines / 1,038,010 bytes; focused card now exclusively owns the collision; 139 facts / 980 unique questions fit six bounded shards; syntax, 8/8 shard cases, 55/55 common cases, current monolith freshness, and 138-card catalog membership passed. The then-reported identity `9724…b642` was invalidated by `.5d.ii`'s post-decode-scalar root cause and is not a content identity. |
| `2026-08-08` | `.5d.i` | staged resulting-tree contract/ADR/fact/docs; exact 10-ADR membership; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; live-size `--report`; `git diff --cached --check` | green: 576 Markdown files / 30 surfaces; decisions 10/10 exactly once; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, repository-local mdBook build, exact book aggregate 33 files / 12,080 lines / 722,754 bytes, and staged diff pass; current monolith remains within its unchanged transition envelope and no ceiling authority is requested |
| `2026-08-08` | `.5d.ii` | portable generator/checker syntax; nine-case contract self-test/check/report; eight-case repository-local bundle integration; exact output-path staging drill; 981-link existence/retrieval drill; 55-case common lifecycle suite; live-size `--report`; residue census | green: 139 facts / 981 unique questions, seven shards; landing 19 lines / 898 bytes, shards 2,000 lines / 183,756 bytes, set 2,019 lines / 184,654 bytes / max 384 UTF-8 bytes; identity `13ee…06d7` agrees independently; 981/981 canonical targets exist; deletion staging, drift/collision/aggregate fail-before-replace, exact membership/content, health pressure, and zero check/generator fixture residue pass |
| `2026-08-08` | `.5d.ii` | staged resulting-tree topology/reader/surface ratchet; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; repository-local mdBook build; `git diff --cached --check` | green: 583 Markdown files / 30 surfaces; fact projection 8 files, normal and below health warning; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, exact book aggregate 33 files / 12,089 lines / 723,488 bytes, and staged diff pass; one exact topology authority is consumed by this commit |
| `2026-08-08` | `.5e.i` | exact source/region SHA-256 and metric census; 23-workstream/status/owner parse; reader/writer grep; task-tree and Git-blame root-cause review; Perl syntax; nine-case `--self-test`; real `--check`/`--report` | green: source 1,487 lines / 183,445 bytes / `20a7…7d88`; five regions reconstruct exactly; 23/23 ordered workstreams and indexed owners; `done:` blocks 722 lines / 129,242 bytes; ten readers + one writer; four stale-current findings evidenced; syntax, 9/9 focused cases, planned identity, limits, paths, and consumers pass; no roadmap record moved |
| `2026-08-08` | `.5e.i` | staged resulting-tree contract/ADR/fact/readers; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; repository-local mdBook build; `git diff --cached --check`; exact book aggregate | green: 585 Markdown files / 30 surfaces; 11/11 ADR routes; 139 fact cards / 140 generated facts / 985 unique questions; roadmap source remains byte-untouched; doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, book 33 files / 12,124 lines / 726,004 bytes, and staged diff pass; prior `.5d.ii` ceiling authority removed and no ceiling authority remains banked |
| `2026-08-08` | `.5e.ii` | capsule vs commit `cd677aec`; dual SHA-256 and exact `wc`; nine-case `--self-test`; migrated real `--check`/`--report`; fact catalog and Knowledge Map derive-and-diff; book current-truth check; live-size composed gate | green: capsule byte-identical at 1,487 lines / 183,445 bytes / `20a7…7d88`; root 153 lines / 12,033 bytes / max 205; index 20 / 595 / 100; 23/23 rows, five capsule regions, four corrections, manifest/index retrieval, 139-card catalog, 140 facts / 987 unique questions, and 587 files / 32 surfaces pass |
| `2026-08-08` | `.5e.ii` | staged resulting-tree migration/readers/surface ratchet; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; repository-local mdBook build; `git diff --cached --check`; exact book aggregate | green: doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, book 33 files / 12,126 lines / 726,111 bytes, and staged diff pass; roadmap debt is closed across three governed surfaces and no ceiling-increase authority is requested |
| `2026-08-08` | `.5f.i` | exact source/region/record SHA-256 and metric census; heading/status/direction parse; response + issue-resolution evidence; consumer grep; Git-blame current-truth audit; Perl syntax; ten-case `--self-test`; real `--check`/`--report`; resulting-tree live-size + mdBook build | green: source 936 lines / 57,980 bytes / `5bf9…9f03`; five regions reconstruct exactly; six closed exchanges and zero open; all response/resolution evidence and 26 consumers resolve; two stale primer claims evidenced; syntax, 10/10 planned/migrated cases, 588 Markdown files / 32 surfaces, 12/12 ADR routes, 139-card catalog, 140 facts / 990 unique questions, and book 33 files / 12,167 lines / 728,963 bytes pass; no feedback record moved |
| `2026-08-08` | `.5f.i` | staged resulting-tree contract/checker/ADR/fact/readers; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; repository-local mdBook build; `git diff --cached --check` | green: doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, mdBook, and staged diff pass; the feedback source remains byte-untouched transition debt and no ceiling-increase authority is requested |
| `2026-08-08` | `.5f.ii` | capsule vs commit `abbc7663`; `cmp -s`, dual SHA-256 and exact `wc`; ten-case `--self-test`; migrated real `--check`/`--report`; fact catalog and Knowledge Map derive-and-diff; composed live-size gate; residue census | green: capsule byte-identical at 936 lines / 57,980 bytes / `5bf9…9f03`; current root 81 lines / 5,243 bytes / max 394; index 26 / 1,331 and manifest resolve; five source regions, six closed rows, zero open, all evidence/consumers, current literals, markers/schema/register, 139-card catalog, 140 facts / 990 unique questions, and 590 Markdown files / 34 surfaces pass; no feedback fixture residue |
| `2026-08-08` | `.5f.ii` | staged resulting-tree capsule/current channel/index/manifest/readers/surface ratchet; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; repository-local mdBook build; `git diff --cached --check`; exact book aggregate | green: doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, book 33 files / 12,173 lines / 729,486 bytes, and staged diff pass; feedback debt is closed across three governed surfaces and no ceiling-increase authority is requested |
| `2026-08-08` | `.5g.i` | Git-history/review-boundary and ambient-sidecar audit; Perl syntax; ten-case `--self-test`; real `--check`/`--report`; fixture residue census; focused Rust writer test; composed live-size gate | green: reviewed commit and four report identities resolve; snapshot 544 lines / 63,628 bytes / `6946…b45a`; 29 recommendations / zero executions / four producer regions and the exact live block pass; 10/10 drift cases clean without residue; writer wording test 1/1; 592 Markdown files / 34 surfaces pass |
| `2026-08-08` | `.5g.i` | staged resulting-tree contract/checker/ADR/fact/producer/readers/surface ratchet; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check`; exact book aggregate | green: doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, book 33 files / 12,215 lines / 732,677 bytes, and staged diff pass; validation data and canonical artifacts are unchanged, currency debt is closed, and no ceiling-increase authority is requested |
| `2026-08-08` | `.5g.ii` | exact registry/Git/path/key/directory/signature probe; Rust derivation-region identities; Perl/Bash syntax; 13-case `--self-test`; real `--check`/`--report`; residue census; composed live-size gate | green: 22 registry rows == 22 Git-indexed corpus PDFs with zero missing/extra/bad items; root 63 lines / 7,374 bytes / max 278 / `0ea6…12e6` stays below warning; two regions/three seams pass; 13/13 cases clean without residue; currency executes under all 34 surfaces |
| `2026-08-08` | `.5g.ii` | staged resulting-tree contract/checker/ADR/fact/readers/surface ratchet; no-PDF/no-Rust-source scope proof; `bash scripts/check_doctrines.sh`; `bash scripts/run_ci.sh`; `git diff --cached --check`; exact book aggregate | green: doctrines 5/5; formatting, Clippy with warnings denied, 1,724 Rust tests (5 ignored), rustdoc, 141-card catalog, 142 facts / 998 unique questions, book 33 files / 12,250 lines / 734,934 bytes, and staged diff pass; source-registry currency debt is closed without changing rows, PDFs, Rust, extraction behavior, or roadmap direction |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.0 — own and measure the containment/locality program` | ownership + measurement only; commit hash recorded by Git history |
| `.1` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.1 — stabilize pointer and commit semantics` | impact-based doc routing + stronger bounded-pointer check |
| `.2` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.2 — bound the README and close its routes` | project-owned policy + retained landing page + unconditional doctrine guard |
| `.3a` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3a — lock the complete surface contract` | full tracked-Markdown census + local lifecycle/schema/checker design; no active doctrine change |
| `.3b` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3b — activate complete live-document enforcement` | neutral doctrine + ADR + 24-surface registry + checker + fifth driver entry |
| `.3c` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.3c — prove every lifecycle and control path` | same-volume 48-case fixture gate + closed registry/route control planes |
| `.5a` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5a — lock current mdBook truth to code` | two drift repairs + executed maintained-reference currency verifier |
| `.4a` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4a — lock the lossless rolling-ledger protocol` | exact grammars + source identities + bounded survivor/archive/consumer contract; no migration |
| `.4b` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4b — migrate CHANGES losslessly` | exact capsule + bounded root + manifest/index + surface debt ratchet |
| `.4c` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4c — migrate DEVELOPMENT_NOTES losslessly` | exact capsule + bounded H2 rationale window + successor-boundary proof + debt ratchet |
| `.4d` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4d — migrate live status losslessly` | exact capsule + bounded bullet snapshot + unchanged generated trailer + writer proof |
| `.4e` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4e — migrate Rust analysis losslessly` | exact capsule + Purpose prologue/current suffix + completed common archive + debt ratchet |
| `.5b` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5b — decompose collection containment` | exact collection/currency census + dependency leaves + scratch-locality fact + archive-index health settlement |
| `.5c.i` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c.i — derive the complete task catalog` | 121-tree bounded projection + fail-closed generator/checker + transition ratchet |
| `.5c.ii` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c.ii — derive bounded fact-card navigation` | preserve 9/9 ADR route + 136-card projection + exact count semantics + composed gate |
| `.5c.iii` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5c.iii — close canonical collection navigation` | external-membership contract + 367-member route closure + bounded catalog plane |
| `.5d.i` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5d.i — lock the bounded Knowledge Map shard contract` | accepted topology + executable feasibility/collision contract + complete reader migration census; no output migration |
| `.5d.ii` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5d.ii — migrate the bounded Knowledge Map projection` | portable landing/shard transaction + exact hook/checker + identity correction + reader/surface ratchet |
| `.5e.i` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5e.i — lock the roadmap current/history boundary` | exact source/semantic regions + ADR 0010 + 23-owner/drift/reader/archive/root contract; no roadmap migration |
| `.5e.ii` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5e.ii — migrate the bounded current roadmap` | exact history capsule + 153-line current root + corrected status + manifest/index + reader/surface ratchet |
| `.5f.i` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5f.i — lock the bounded FSMGen feedback contract` | exact source/regions + ADR 0011 + six status/direction/evidence records + consumer/archive/root contract; no migration |
| `.5f.ii` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5f.ii — migrate the bounded FSMGen feedback channel` | exact capsule + 81-line current root + zero-open/six-closed routes + manifest/index + reader/surface ratchet |
| `.5g.i` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.i — enforce reviewed validation currentness` | tracked reviewed-boundary contract + non-mutating ten-case oracle + producer/live projection proof + debt ratchet |
| `.5g.ii` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.ii — enforce tracked source-PDF registry currentness` | Git-indexed 22-PDF denominator + key/path/signature oracle + derivation seams + debt ratchet |

## Changelog

- `2026-08-08`: Created from the owner's external-SSD move, README-policy directive, and FSMGEN
  live-document-containment adoption guide. Recorded local measurements, task-owned the two ramp-up
  mdBook drifts, and separated common enforcement from lossless migrations and same-volume remediation.
- `2026-08-08`: `.1` removed ceremonial documentation coupling from `COMMIT.md`, removed the
  hand-maintained HEAD shadow from `MEMORY.md` and its architecture template, and strengthened the
  pointer gate with locally derived line/byte/max-line ceilings plus required resume fields.
- `2026-08-08`: `.2` replaced the duplicate-heavy README with a verified landing page, adopted the
  local stability policy, classified reader versus author-overflow routes in a data-only registry,
  wired an unconditional structural guard as doctrine four, and corrected stale knowledge-map
  architecture pointers across the bootstraps.
- `2026-08-08`: `.3a` decomposed the full enforcement slice, classified all 550 tracked Markdown
  files, and locked the SpecForge-local lifecycle, registry, ceiling/debt, route, and fail-closed
  checker contract without prematurely activating the doctrine.
- `2026-08-08`: `.3b` atomically activated the project-owned neutral doctrine, ADR 0007, complete
  24-surface JSONL data plane, deterministic checker, route composition, and `LIVE-DOC-SIZE` driver
  entry; it also repaired nested research coverage and the missing genericity-ADR index entry.
- `2026-08-08`: `.3c` closed the common enforcement program with a 48-case, repository-volume
  lifecycle/control-plane fixture gate, explicit array/scalar/schema closure, self-bounded README
  route data, and unconditional execution from the fifth doctrine adapter.
- `2026-08-08`: `.5a` repaired two pre-existing mdBook time-layer contradictions, routed each
  current statement to its detailed canonical chapter, and made the load-bearing code/book facts an
  executed maintained-reference currency check.
- `2026-08-08`: `.4a` pinned four grammar-specific source identities and reviewed survivors, recorded
  the `CHANGES.md` detached-record root cause, adopted ADR 0008, and added the bounded JSONL plan plus
  unconditional reconstruction/consumer/archive-protocol checker. No historical record moved.
- `2026-08-08`: `.4b` copied and verified the exact `CHANGES.md` source capsule before mechanically
  deriving the bounded root, landed its manifest/index and archive classification, ratcheted the live
  surface out of transition debt, and proved retrieval plus exact retained-record identity.
- `2026-08-08`: `.4c` repeated the lossless transition for `DEVELOPMENT_NOTES.md`, extended the common
  archive route, and root-caused the inter-record separator as successor-owned boundary structure
  before ratcheting the rationale surface out of debt.
- `2026-08-08`: `.4d` froze the exact status source, shortened only the whole-bullet current region,
  extended the common archive route, preserved the entire generated trailer, and proved the real Rust
  projection writer in a repository-volume fixture before ratcheting the status surface out of debt.
- `2026-08-08`: `.4e` froze the exact Rust architecture source, derived its Purpose prologue plus
  newest 60 whole records, completed the four-capsule route, and removed the final rolling-ledger
  transition debt without adding a migration-only architecture record.
- `2026-08-08`: `.5b` measured the post-ledger collection/projection plane, decomposed every remaining
  transition and currentness obligation into dependency-ordered leaves, recorded the incomplete task
  catalog, preserved already-honest decision/book controls, and captured the exact-remediated
  off-volume diagnostic-scratch incident for `.6` enforcement.
- `2026-08-08`: `.5c.i` replaced mirrored task-history rows with a deterministic 121-tree catalog,
  recovered four missing routes, separated the author template, wired seven fail-closed cases plus
  derive-and-diff into doctrine enforcement, and ratcheted the task index out of transition debt.
- `2026-08-08`: `.5c.ii` preserved and reverified the complete decision index, added a bounded
  generated catalog for 136 actual fact cards, separated directory-card and cross-layer-map counts,
  wired nine focused cases plus exact membership into doctrine enforcement, corrected stale route
  debt labels, and removed `.5c.i`'s consumed one-commit ceiling authority.
- `2026-08-08`: `.5c.iii` replaced all eight remaining query-only canonical collection contracts
  with direct membership over 367 files, introduced a separately bounded seven-file catalog plane,
  made external membership explicit and fail-closed, expanded the common fixture gate to 53 cases,
  and closed `.5c` without moving or copying canonical member prose.
- `2026-08-08`: `.5d.i` root-caused the monolithic Knowledge Map's repetition pressure and one
  duplicate question destination, accepted ADR 0009, locked deterministic bounded landing/shard and
  canonical-input identity contracts, proved the current corpus fits in six shards, expanded the
  common fixture gate to 55 cases, and enumerated the atomic `.5d.ii` writer/reader migration.
- `2026-08-08`: `.5d.ii` atomically replaced the monolith with a bounded landing plus seven exact
  question shards, made hook staging deletion-aware and checking membership-sensitive, fixed the
  preflight's post-decode empty-content identity bug, added packing targets plus aggregate caps and
  eight integration cases, migrated all readers, and removed the generated surface's transition debt.
- `2026-08-08`: `.5e.i` pinned the exact roadmap and five semantic regions, proved that 722 lines of
  delivery chronology are interleaved with current scope, root-caused four stale canonical claims,
  accepted ADR 0010, and activated a nine-case source/workstream/owner/reader/archive/root contract
  without moving a roadmap record. `.5e.ii` owns the atomic capsule and bounded-current migration.
- `2026-08-08`: `.5e.ii` sealed the byte-identical source capsule behind a bounded index/manifest,
  replaced the stable root with 153 lines of current priorities plus 23 exact owned status rows,
  corrected four stale claims, switched the contract to migrated identity, updated all changed
  readers, and ratcheted roadmap transition debt into three independently governed surfaces.
- `2026-08-08`: `.5f.i` pinned the exact heterogeneous feedback source, proved all six directed
  exchanges closed with independent evidence and no open record, root-caused stale `.fsm`/pin claims
  in the legacy primer, accepted ADR 0011, and activated a ten-case source/status/consumer/archive/root
  contract without moving correspondence. `.5f.ii` owns the atomic capsule/current-channel migration.
- `2026-08-08`: `.5f.ii` sealed the byte-identical feedback source behind a bounded index/manifest,
  replaced the stable root with an 81-line current channel carrying zero open and six evidence-linked
  closed exchanges, switched the protocol to migrated enforcement, aligned all semantic readers, and
  ratcheted one transition record into separately governed current, index, and immutable-history
  surfaces.
- `2026-08-08`: `.5g.i` defined validation-snapshot truth at its explicit owner-reviewed boundary,
  pinned four report identities plus the live projection and mutating producer regions in a tracked
  contract, added a ten-case read-only oracle with exception-safe residue rejection, aligned producer
  and book wording, and ratcheted snapshot currency from labelled debt to enforced currentness without
  changing validation data or canonical artifacts.
- `2026-08-08`: `.5g.ii` made the Git-indexed PDF set below `corpus/` the source-registry denominator,
  proved exact 22-row membership and key/path/directory/signature agreement, pinned the Rust derivation
  seams, added a 13-case read-only oracle, aligned durable readers, and ratcheted registry currency from
  debt to enforced without changing a registry row, PDF, Rust source, extraction behavior, or roadmap.
