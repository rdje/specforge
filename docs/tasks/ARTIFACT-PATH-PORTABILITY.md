# ARTIFACT-PATH-PORTABILITY: repository-relative IR provenance and move-safe generated artifacts

## Metadata

- Tree ID: `ARTIFACT-PATH-PORTABILITY`
- Status: `active`
- Roadmap lane: cross-cutting repository durability and canonical pipeline integrity
- Created: `2026-08-08`
- Last updated: `2026-08-08`
- Owner: repo-local workflow

## Goal

Make every SpecForge-owned path persisted in SourceIR, EvidenceIR, SemanticIR, IntentIR, adapter artifacts,
and typed prior memory repository-root-relative, while retaining safe backward-compatible loading of legacy
absolute artifacts. Migrate or rebuild current generated state with copy/verify/use/delete evidence, then gate
both producer behavior and present-artifact residue so a repository move cannot silently disable provenance,
validation, learning, or recovery workflows.

## Non-Goals

- Do not rewrite sealed historical command evidence or external caller-authorized source paths.
- Do not delete or relocate the owner-approved shared `~/.cargo` and `~/.rustup` stores or ambiguous global
  caches.
- Do not bulk-substitute strings in ignored artifacts before the typed resolver, schema compatibility, and
  migration verification contract exist.
- Do not change extraction semantics, IR fact content, `.isf` lowering, or protocol gold expectations.

## Acceptance Criteria

- Repository-owned persisted paths are relative to the current root; runtime absolute paths are derived only
  while opening or writing a resource.
- Legacy absolute paths beneath a previous SpecForge root resolve safely to the current repository when their
  repository-relative suffix and expected target are valid; unrelated external absolute paths never rebase.
- Every producer and consumer of upstream-stage, source/provenance, artifact-layout, and prior-memory paths uses
  one common contract with positive, move, escape, ambiguity, and missing-target tests.
- Current generated artifacts follow copy/verify/use/delete or deterministic rebuild; file/byte/hash and real
  workflow evidence prove the new state before exact old-path residue is removed.
- A fail-closed doctrine check rejects new persisted repository-owned absolute paths and present generated
  residue without treating sealed history or explicit shared dependencies as project data.
- The roadmap, codebase analysis, locality standard, mdBook, Knowledge Map, live ledgers, and this tree remain
  aligned; every completed leaf is committed through `COMMIT.md`.

## Task Tree

- ID: `ARTIFACT-PATH-PORTABILITY` · Status: `active` · Children: `.0`–`.5`
- ID: `ARTIFACT-PATH-PORTABILITY.0` · Status: `done` (`2026-08-08`) · Goal: reproduce, census, and
  root-cause the post-move absolute-path boundary; open a detailed tree before implementation.
- ID: `ARTIFACT-PATH-PORTABILITY.1` · Status: `done` (`2026-08-08`) · Goal: design and test one common persisted-path
  codec/resolver: repository-relative serialization, current-root runtime resolution, narrowly recognized
  legacy-root rebasing, explicit external-input handling, and traversal/symlink/ambiguity refusal.
- ID: `ARTIFACT-PATH-PORTABILITY.2` · Status: `done` (`2026-08-08`; depends on `.1`) · Goal: migrate SourceIR and
  EvidenceIR producers/consumers, including source registration, normalization/artifact layouts, section/span/
  visual provenance, upstream pointers, and prior-memory references.
- ID: `ARTIFACT-PATH-PORTABILITY.3` · Status: `done` (`2026-08-08`; depends on `.2`) · Goal: migrate SemanticIR,
  IntentIR, adapter, validation, project-validation, learning, recovery, and convergence consumers to the
  common path contract without losing cross-stage checks.
- ID: `ARTIFACT-PATH-PORTABILITY.3a` · Status: `done` (`2026-08-08`; depends on `.3`) · Goal: repair the stale
  root locality-standard assertion found by the post-commit cold read and audit current-facing portability
  claims before present-data work begins; no Rust or generated-data change.
- ID: `ARTIFACT-PATH-PORTABILITY.4` · Status: `pending` (depends on `.3a`) · Goal: add the fail-closed
  producer/present-artifact locality gate and migrate or rebuild the current 700 MiB generated tree with exact
  copy/verify/use/delete evidence; leave zero deleted-root references.
- ID: `ARTIFACT-PATH-PORTABILITY.5` · Status: `pending` (depends on `.4`) · Goal: cold-read every path
  surface, run real moved-root workflows plus full CI/residue gates, reconcile all public/continuity docs, and
  close the tree before returning to `SWD-SERIAL-EXTRACTION.7`.

## Acceptance Checklist (enforced) — `ARTIFACT-PATH-PORTABILITY.1`

- [x] **REPRODUCE / MEASURE** — 13 focused cases exercise relative encoding/current-root resolution,
  pre-creation outputs, current absolute compatibility, moved-root rebasing, stable origin labels, explicit
  external inputs, outside-root ownership refusal, traversal, ambiguity, missing targets, and symlink escape.
- [x] **ROOT CAUSE (WHY + WHERE)** — `SourceIr::build` canonicalizes the source at
  `crates/specforge/src/ir/source.rs:581`, and `EvidenceIr::build` repeats that pattern for its upstream input at
  `crates/specforge/src/ir/evidence.rs:725`. Persisted `PathBuf` values therefore mixed storage identity with
  runtime I/O identity and provided no common repository-owned versus authorized-external distinction.
- [x] **ADDRESSED (verified)** — repository-owned values encode relative; runtime resolution anchors to the
  discovered current root; legacy absolute paths rebase only below recognized project-data roots to exactly
  one existing local target; external inputs never enter that compatibility branch.
- [x] **NO REGRESSION** — the focused suite passes 13/13, the full suite passes 1,748 with five ignored, and
  formatting plus all-target warning-deny Clippy pass; no IR producer, consumer, schema instance, generated
  artifact, extraction fact, or canonical gold changes in this design leaf.
- [x] **GENERICITY** — resolution uses typed origin, current repository discovery, declared project-data root
  classes, containment, existence, and uniqueness. It contains no username, mount point, old root, document
  key, protocol, vendor, or fixture allowlist.
- [x] **LOCKSTEP** — task tree, roadmap, locality standard/book, Knowledge Map, Rust architecture analysis,
  live ledgers, and resume pointer agree that the common contract is landed and `.2` owns first-stage use.

## Acceptance Checklist (enforced) — `ARTIFACT-PATH-PORTABILITY.2`

- [x] **REPRODUCE / MEASURE** — focused Markdown, PDF, legacy-root, visual-provenance, convergence, and tracked
  KG-fixture tests exercise SourceIR/EvidenceIR runtime loading plus repository-relative JSON and sidecar
  manifests. A corrected retired-root fixture proves it actually stores `/retired/specforge/...` before load.
- [x] **ROOT CAUSE (WHY + WHERE)** — SourceIR mixed canonical runtime identity into source registration,
  normalization, and artifact-layout fields; EvidenceIR then canonicalized its upstream pointer and promoted
  source before copying that absolute identity into section/span/visual provenance and prior-memory lineage.
- [x] **ADDRESSED (verified)** — SourceIR and EvidenceIR now operate on root-derived absolute paths in memory,
  serialize repository-owned fields relative, label text source origin, preserve exact authorized external
  sources, rebase unlabeled legacy repository values, and keep visual assets intrinsically repository-owned.
  Validation and convergence resolve repository-derived paths at I/O boundaries instead of trusting process CWD.
- [x] **NO REGRESSION** — full CI passes all six doctrines, formatting, all-target warning-deny Clippy, 1,753
  Rust tests with five ignored, rustdoc, mdBook build, and final locality/residue checks. Focused legacy, PDF
  sidecar, visual, convergence, and 156-fixture KG checks pass. The existing 335 affected generated artifacts
  and all extraction facts/golds remain unchanged for `.4` migration.
- [x] **GENERICITY** — behavior depends only on typed field ownership, canonical containment, existing-target
  identity, and repository discovery. No username, mount point, document key, protocol, vendor, or fixture is
  encoded in production paths; legacy compatibility remains the bounded `.1` resolver.
- [x] **LOCKSTEP** — task tree, roadmap, locality standard/book, SourceIR/EvidenceIR chapters, Knowledge Map,
  Rust architecture analysis, live ledgers, and resume pointer agree that `.2` is landed and `.3` owns the
  remaining canonical stages and consumers.

## Acceptance Checklist (enforced) — `ARTIFACT-PATH-PORTABILITY.3`

- [x] **REPRODUCE / MEASURE** — a complete downstream pipeline fixture proves SemanticIR, IntentIR, and adapter
  artifacts serialize repository-owned upstream/layout/emitted-target paths relative, then loads deliberately
  retired-root forms as current absolute runtime paths. Learning and recovery have separate focused proofs.
- [x] **ROOT CAUSE (WHY + WHERE)** — all three downstream builders used local
  `canonicalize_existing_path` helpers and serialized their structs directly; `learn-priors` persisted its
  canonical input paths, while recovery, project-validation, and convergence bypassed the common boundary.
- [x] **ADDRESSED (verified)** — downstream build/load/write paths now use normalized persisted clones and
  resolved runtime layouts; typed prior-memory sources serialize through the same contract; validation,
  project-validation, learning, recovery, KG fixtures, and convergence dereference repository paths through it.
- [x] **NO REGRESSION** — formatting and all-target warning-deny Clippy pass; the complete Rust suite passes
  1,758 tests with five ignored; `kg-bench` passes 156/156; adapter strict checks and final full CI pass. No
  current generated artifact or extraction/gold content changed.
- [x] **GENERICITY** — every migrated field is classified from its typed role as a repository-owned pipeline
  artifact/output. No workstation root, document key, protocol, fixture, or filename allowlist was added;
  ambiguous retired paths remain a hard error through the common resolver.
- [x] **LOCKSTEP** — task tree, roadmap, Knowledge Map, Rust architecture analysis, live ledgers, locality and
  generated-artifact book chapters, pipeline chapters, and resume pointer agree that code activation is complete
  and `.4` exclusively owns present-data migration plus residue enforcement.

## Acceptance Checklist (enforced) — `ARTIFACT-PATH-PORTABILITY.3a`

- [x] **REPRODUCE / MEASURE** — the post-commit cold read found one current root assertion claiming downstream
  activation remained open; a repository-wide current-facing phrase audit found no equivalent stale claim
  outside the explicitly time-layered `.2` architecture/status records.
- [x] **ROOT CAUSE (WHY + WHERE)** — `.3` aligned the roadmap, book, Knowledge Map, live status, and Rust
  analysis but omitted `PROJECT_DATA_LOCALITY.md` from the final changed-file set even though its current
  required-roots paragraph still described the `.2` frontier.
- [x] **ADDRESSED (verified)** — the root standard now names every activated canonical stage, typed prior
  memory, and consumer I/O seam while reserving only the unchanged present generated corpus for `.4`.
- [x] **NO REGRESSION** — no Rust, generated artifact, extraction fact, or book content changed; the complete
  mdBook doctest/build entrypoint and all six doctrines pass.
- [x] **GENERICITY** — the corrected standard describes typed stage/consumer roles and the active leaf, without
  a workstation root, document key, protocol, vendor, fixture, or special-case path.
- [x] **LOCKSTEP** — root locality standard, roadmap, book, Knowledge Map, Rust analysis, live ledgers,
  task tree, and resume pointer agree that code activation is complete and `.4` alone owns present-data work.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ARTIFACT-PATH-PORTABILITY.4` | `pending` | Migrate data now that code and every current authority agree on readiness. |
| 2 | `ARTIFACT-PATH-PORTABILITY.5` | `pending` | Independent closure proves no path or documentation surface escaped. |

## Decisions

- `2026-08-08`: This policy breach outranks resuming SWD protocol projection. The dirty-tree pivot rule was
  honored: `SWD-SERIAL-EXTRACTION.4e` committed cleanly first, then this tree opened.
- `2026-08-08`: Stored paths and runtime paths are distinct concepts. A path may be canonicalized in a local
  variable for I/O, but repository-owned serialization must not retain that absolute value.
- `2026-08-08`: Legacy support must be semantic and bounded, not a replacement of one known username/mount
  prefix with another. Exact rules are the `.1` design deliverable.
- `2026-08-08`: Keep existing path fields as schema-compatible strings. Their typed field role declares
  repository ownership; the sole mixed-origin source registration gains a stable adjacent origin label in
  `.2`. A wrapper object around every path would create needless whole-schema churn.
- `2026-08-08`: Legacy rebasing is available only to repository-owned fields, begins at one of the declared
  project-data roots, requires a present target below the current canonical repository, and refuses zero or
  multiple matches plus traversal or symlink escape. External inputs resolve only at their exact path.
- `2026-08-08`: Stage structs keep runtime-absolute `PathBuf` values in memory for compatibility with their
  existing consumers, but `to_pretty_json` and `write_to_disk` serialize normalized clones. `load_from_path`
  performs the inverse runtime projection, so storage portability does not force absolute-path assumptions
  through hundreds of extraction call sites.
- `2026-08-08`: Evidence section anchors and spans inherit the promoted text source origin. Visual evidence
  paths remain repository-owned because they originate from SourceIR's normalized asset bundle, even when the
  text source itself is an explicitly authorized external Markdown file.
- `2026-08-08`: Convergence derives production stage roots from repository discovery and injects a complete
  root bundle in tests. This removes process-CWD mutation and prevents concurrent tests from observing a false
  repository location.
- `2026-08-08`: SemanticIR, IntentIR, adapter upstream pointers, artifact layouts, emitted targets, and learned
  source-artifact records are intrinsically repository-owned. Unlike SourceIR registration, none is a mixed
  external-input field, so no additional schema origin label is needed.
- `2026-08-08`: Preserve the stage API contract used by validators and adapters: builders/loaders expose
  absolute runtime paths, while `to_pretty_json` and `write_to_disk` serialize normalized clones. Stage loading
  rejects the wrong stage before resolving that stage's typed internal paths.
- `2026-08-08`: Keep ambiguity fail-closed. A focused recovery probe initially matched both `.project-data`
  and `generated` suffixes because both exact targets existed; the resolver correctly refused to guess, and the
  isolated one-target command seam proves legacy argument rebasing separately.
- `2026-08-08`: A post-`.3` commit cold read found `PROJECT_DATA_LOCALITY.md` still describing downstream
  activation as pending. Treat that as a lockstep defect, open `.3a` before `.4`, audit equivalent live claims,
  and correct only current authority; the committed `.3` behavior and generated corpus remain unchanged.

## Open Questions

- `.4` must compare migrated current artifacts by file/byte/hash and real workflow behavior; producer support in
  `.2` does not authorize rewriting the 262,592 repeated legacy EvidenceIR provenance values early.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-08` | `.0` | old-root absence; generated path/file/stage/key census; producer assignment and consumer-use cross-read; doctrine + mdBook checks | 335 affected files / 262,996 values; root cause localized; ownership complete; no generated mutation |
| `2026-08-08` | `.1` | 13 focused codec/resolver cases; full Rust suite; `cargo fmt --check`; all-target `clippy -D warnings`; doctrines; mdBook | 1,748 pass / 5 ignored; contract and all gates pass; no IR/generated mutation |
| `2026-08-08` | `.2` | Source/Evidence legacy + persisted/runtime cases; PDF/visual/convergence/KG checks; full CI | 6/6 doctrines; 1,753 pass / 5 ignored; rustdoc/book/locality pass; no generated mutation |
| `2026-08-08` | `.3` | downstream persisted/runtime/legacy fixture; prior-learning and recovery command cases; project-validation/validation; 156 KG fixtures; fmt/Clippy; 1,763-test suite; full CI | 1,758 pass / 5 ignored; all focused/full gates pass; generated corpus unchanged |
| `2026-08-08` | `.3a` | current-facing portability phrase audit; canonical docs entrypoint; all doctrines | one stale root authority corrected; mdBook doctest/build and 6/6 doctrines pass; no code/data change |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `ARTIFACT-PATH-PORTABILITY.0 — own the move-portability repair` | Measurement/ownership only; `.1` is the first implementation gate. |
| `.1` | `ARTIFACT-PATH-PORTABILITY.1 — land the move-safe path contract` | Common seam only; `.2` activates it in SourceIR/EvidenceIR. |
| `.2` | `ARTIFACT-PATH-PORTABILITY.2 — make Source and Evidence paths portable` | First two stages serialize relative and load runtime paths; `.3` owns downstream activation. |
| `.3` | `ARTIFACT-PATH-PORTABILITY.3 — make downstream artifact paths portable` | Canonical stages/consumers activated; `.4` owns data migration and enforcement. |
| `.3a` | `ARTIFACT-PATH-PORTABILITY.3a — reconcile the root locality authority` | Corrective lockstep leaf; `.4` remains the first generated-data mutation. |

## Changelog

- `2026-08-08`: Created after the SWD `.4e` fresh-build path exposed stale absolute lineage. Measured the
  complete present generated surface, localized producer and consumer seams, and split design, stage migration,
  data migration/enforcement, and closure into independently committable leaves.
- `2026-08-08`: Landed the schema-compatible persisted-path codec/resolver with stable origin labels and
  fail-closed current-root, legacy-rebase, external-input, traversal, ambiguity, missing, and symlink behavior.
- `2026-08-08`: Activated the contract across SourceIR/EvidenceIR registration, layouts, normalized sidecars,
  text and visual provenance, upstream/prior lineage, validation entry, and convergence roots while preserving
  runtime-absolute in-memory compatibility and leaving present generated artifacts unchanged.
- `2026-08-08`: Activated the same contract across SemanticIR, IntentIR, adapters, typed prior-memory sources,
  and validation/learning/recovery/project-validation/convergence consumers. All canonical stage producers now
  emit move-safe paths; current generated data remains unchanged for the verified `.4` migration.
- `2026-08-08`: Corrected the root locality standard's stale `.2` frontier after the `.3` cold read and audited
  equivalent current-facing claims. Product behavior, generated data, and already-current book content did not
  change.
