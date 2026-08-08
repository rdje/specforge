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
  Status: `active`
  Goal: migrate chronological live surfaces losslessly at whole-record boundaries. Children will own
  `CHANGES.md`, `DEVELOPMENT_NOTES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, and
  `RUST_CODEBASE_ANALYSIS.md` independently; each migration must prove source identity, semantic/current
  coverage, retrieval, indexes, and no residue before any live duplicate is removed.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4a`
  Status: `done` (`2026-08-08`)
  Goal: measure the four rolling-ledger record grammars and consumers, then lock one shared lossless
  live-window/sealed-segment/archive-descriptor protocol before moving any historical record.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4b`
  Status: `pending`
  Goal: migrate `CHANGES.md` at a verified whole-record boundary under the `.4a` protocol.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4c`
  Status: `pending`
  Goal: migrate `DEVELOPMENT_NOTES.md` at a verified whole-record boundary under the `.4a` protocol.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4d`
  Status: `pending`
  Goal: migrate `LIVE_ACHIEVEMENT_STATUS.md` at a verified whole-record boundary under the `.4a`
  protocol while preserving its bounded current snapshot.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4e`
  Status: `pending`
  Goal: migrate `RUST_CODEBASE_ANALYSIS.md` at a verified whole-record boundary under the `.4a`
  protocol.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5`
  Status: `active`
  Goal: classify and contain collections/projections: task trees + bounded index, decision/fact stores,
  generated `KNOWLEDGE_MAP.md`, roadmap, research ledgers, and the mdBook maintained reference. Semantic
  partitioning and complete direct navigation are required; aggregate product-scope change must carry
  exact fresh authority rather than a decorative cap.

- ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5a`
  Status: `done` (`2026-08-08`)
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
| — | `.1` | `done` (`2026-08-08`) | Pointer semantics and impact-based documentation routing are now explicit and mechanically checked. |
| — | `.2` | `done` (`2026-08-08`) | The retained landing page, local policy, route registry, and unconditional doctrine guard are live. |
| — | `.3a` | `done` (`2026-08-08`) | Complete 550-file inventory, lifecycle mapping, schema, ceiling, and checker contracts are locked. |
| — | `.3b` | `done` (`2026-08-08`) | Doctrine, ADR, 24-surface registry, resulting-tree checker, and fifth driver entry are active atomically. |
| — | `.3c` | `done` (`2026-08-08`) | Forty-eight repository-volume fixtures prove every lifecycle and control-plane failure path; the common `.3` contract is closed. |
| — | `.5a` | `done` (`2026-08-08`) | Both contradictions are repaired against code and protected by the executed mdBook currency verifier. |
| — | `.4a` | `done` (`2026-08-08`) | Four real grammars, exact source identities, bounded survivor plans, consumers, and the repository-local source-capsule protocol are executable; no record moved. |
| 1 | `.4b` | `pending` | Atomically migrate `CHANGES.md`, including the two measured detached compatibility records, under the locked protocol. |
| 2 | `.4c`–`.4e` / `.5` / `.6` | `pending` / `active` | Independently committable migrations and locality enforcement after the first archive path is proved. |
| 5 | `.7` | `pending` | Close only after every transition and retrieval/locality proof passes. |

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

## Blockers

- None for `.0`–`.3`.
- Later migrations may stop for an authority conflict, unique unclassified content, or an unprovable
  retrieval promise, exactly as required by the adopted doctrine.

## `.4a` Rolling-Ledger Census And Protocol

### Exact source and reviewed survivor

All figures below are re-derived by `perl scripts/check_rolling_ledger_protocol.pl --report`. Source
SHA-256 values are pinned in the bounded registry; the table keeps the independently meaningful
working-set dimensions readable. “Archived records” is the count outside the future live view, not a
claim that `.4a` already moved them.

| Ledger | Source records / lines / bytes / max line | Planned live records / lines / bytes / max line | Records outside live view | Live limits: records / lines / bytes / max line |
| --- | --- | --- | ---: | --- |
| `CHANGES.md` | 1,798 / 32,682 / 2,629,033 / 1,629 | 87 / 1,357 / 199,055 / 1,629 | 1,711 | 128 / 2,000 / 262,144 / 2,200 |
| `DEVELOPMENT_NOTES.md` | 1,601 / 20,921 / 2,170,230 / 1,401 | 60 / 1,473 / 193,810 / 1,401 | 1,541 | 96 / 2,000 / 262,144 / 1,800 |
| `LIVE_ACHIEVEMENT_STATUS.md` | 1,918 / 1,958 / 579,393 / 5,467 | 50 / 89 / 92,375 / 5,107 | 1,868 | 80 / 640 / 131,072 / 6,800 |
| `RUST_CODEBASE_ANALYSIS.md` | 1,350 / 9,039 / 1,046,679 / 3,041 | 60 / 1,057 / 89,728 / 369 | 1,290 | 96 / 1,400 / 196,608 / 600 |

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
`VALIDATION_SNAPSHOT.md` and `corpus/SOURCE_PDF_REGISTRY.md` are bounded generated/current snapshots
whose existing producers become declared currency verifiers when available; missing verifier wiring is
transition debt, not a fabricated freshness claim.

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
