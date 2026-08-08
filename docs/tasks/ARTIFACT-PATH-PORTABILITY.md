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
- ID: `ARTIFACT-PATH-PORTABILITY.2` · Status: `pending` (depends on `.1`) · Goal: migrate SourceIR and
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

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ARTIFACT-PATH-PORTABILITY.2` | `pending` | Source/Evidence own the largest path surface and feed every downstream stage. |
| 2 | `ARTIFACT-PATH-PORTABILITY.3` | `pending` | Canonical downstream consumers must retain validation and learning behavior after moves. |
| 3 | `ARTIFACT-PATH-PORTABILITY.4` | `pending` | Migrate data only after code can safely read both old and new forms. |
| 4 | `ARTIFACT-PATH-PORTABILITY.5` | `pending` | Independent closure proves no path or documentation surface escaped. |

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

## Open Questions

- `.2` must derive legacy SourceIR origin safely: relative `requested_path` is repository-owned; an absolute
  request is external unless it is below the current root or a recognized old-root suffix resolves uniquely.
  New SourceIR persists the stable `repository_owned` / `external_input` label beside `canonical_path`.
- Can the 262,592 repeated evidence `source_path` values be safely normalized during loading/writing without
  changing fact identity or deterministic hashes? Owner: `.2`; verify before migration.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-08` | `.0` | old-root absence; generated path/file/stage/key census; producer assignment and consumer-use cross-read; doctrine + mdBook checks | 335 affected files / 262,996 values; root cause localized; ownership complete; no generated mutation |
| `2026-08-08` | `.1` | 13 focused codec/resolver cases; full Rust suite; `cargo fmt --check`; all-target `clippy -D warnings`; doctrines; mdBook | 1,748 pass / 5 ignored; contract and all gates pass; no IR/generated mutation |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `ARTIFACT-PATH-PORTABILITY.0 — own the move-portability repair` | Measurement/ownership only; `.1` is the first implementation gate. |
| `.1` | `ARTIFACT-PATH-PORTABILITY.1 — land the move-safe path contract` | Common seam only; `.2` activates it in SourceIR/EvidenceIR. |

## Changelog

- `2026-08-08`: Created after the SWD `.4e` fresh-build path exposed stale absolute lineage. Measured the
  complete present generated surface, localized producer and consumer seams, and split design, stage migration,
  data migration/enforcement, and closure into independently committable leaves.
- `2026-08-08`: Landed the schema-compatible persisted-path codec/resolver with stable origin labels and
  fail-closed current-root, legacy-rebase, external-input, traversal, ambiguity, missing, and symlink behavior.
