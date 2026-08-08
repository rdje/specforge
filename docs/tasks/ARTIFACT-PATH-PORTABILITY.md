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
- ID: `ARTIFACT-PATH-PORTABILITY.3` · Status: `pending` (depends on `.2`) · Goal: migrate SemanticIR,
  IntentIR, adapter, validation, project-validation, learning, recovery, and convergence consumers to the
  common path contract without losing cross-stage checks.
- ID: `ARTIFACT-PATH-PORTABILITY.4` · Status: `pending` (depends on `.3`) · Goal: add the fail-closed
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

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ARTIFACT-PATH-PORTABILITY.3` | `pending` | Canonical downstream consumers must retain validation and learning behavior after moves. |
| 2 | `ARTIFACT-PATH-PORTABILITY.4` | `pending` | Migrate data only after code can safely read both old and new forms. |
| 3 | `ARTIFACT-PATH-PORTABILITY.5` | `pending` | Independent closure proves no path or documentation surface escaped. |

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

## Open Questions

- `.3` must decide each downstream and learning-plane path field's ownership from its typed role, then prove
  validation, project-validation, learning, recovery, and convergence retain cross-stage behavior after a move.
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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `ARTIFACT-PATH-PORTABILITY.0 — own the move-portability repair` | Measurement/ownership only; `.1` is the first implementation gate. |
| `.1` | `ARTIFACT-PATH-PORTABILITY.1 — land the move-safe path contract` | Common seam only; `.2` activates it in SourceIR/EvidenceIR. |
| `.2` | `ARTIFACT-PATH-PORTABILITY.2 — make Source and Evidence paths portable` | First two stages serialize relative and load runtime paths; `.3` owns downstream activation. |

## Changelog

- `2026-08-08`: Created after the SWD `.4e` fresh-build path exposed stale absolute lineage. Measured the
  complete present generated surface, localized producer and consumer seams, and split design, stage migration,
  data migration/enforcement, and closure into independently committable leaves.
- `2026-08-08`: Landed the schema-compatible persisted-path codec/resolver with stable origin labels and
  fail-closed current-root, legacy-rebase, external-input, traversal, ambiguity, missing, and symlink behavior.
- `2026-08-08`: Activated the contract across SourceIR/EvidenceIR registration, layouts, normalized sidecars,
  text and visual provenance, upstream/prior lineage, validation entry, and convergence roots while preserving
  runtime-absolute in-memory compatibility and leaving present generated artifacts unchanged.
