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
- ID: `ARTIFACT-PATH-PORTABILITY.1` · Status: `pending` · Goal: design and test one common persisted-path
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

## Acceptance Checklist (enforced) — `ARTIFACT-PATH-PORTABILITY.0`

- [x] **REPRODUCE / MEASURE** — the old root is absent; 335 of 506 generated JSON/Markdown files across
  SourceIR 22, EvidenceIR 78, SemanticIR 78, IntentIR 78, adapters 78, and prior memory 1 contain the old
  root. Their 262,996 matching scalar values are dominated by 262,592 evidence `source_path` entries.
- [x] **ROOT CAUSE (WHY + WHERE)** — SourceIR stores `fs::canonicalize(source)`; the EvidenceIR,
  SemanticIR, IntentIR, and adapter builders canonicalize their input before assigning the persisted upstream
  path. Downstream validation/learning/recovery consumers later open those stored paths directly.
- [x] **ADDRESSED (verified)** — a dedicated tree now owns common path semantics, all producer/consumer
  migrations, generated-data migration, enforcement, and closure; no artifact was rewritten in `.0`.
- [x] **NO REGRESSION** — `.0` is read-only measurement plus documentation; all six doctrines and the
  mdBook gate pass, and the prior `.4e` full CI remains the clean code baseline.
- [x] **GENERICITY** — ownership is keyed to repository containment and typed path roles, never current/old
  usernames, mount points, document keys, vendors, protocols, or stage fixture names.
- [x] **LOCKSTEP** — tree/catalog, roadmap durability priority, locality book truth, Knowledge Map fact,
  architecture/status/change ledgers, and resume pointer agree on the measured current gap and `.1` frontier.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ARTIFACT-PATH-PORTABILITY.1` | `pending` | One reviewed path contract must exist before any producer or ignored artifact changes. |
| 2 | `ARTIFACT-PATH-PORTABILITY.2` | `pending` | Source/Evidence own the largest path surface and feed every downstream stage. |
| 3 | `ARTIFACT-PATH-PORTABILITY.3` | `pending` | Canonical downstream consumers must retain validation and learning behavior after moves. |
| 4 | `ARTIFACT-PATH-PORTABILITY.4` | `pending` | Migrate data only after code can safely read both old and new forms. |
| 5 | `ARTIFACT-PATH-PORTABILITY.5` | `pending` | Independent closure proves no path or documentation surface escaped. |

## Decisions

- `2026-08-08`: This policy breach outranks resuming SWD protocol projection. The dirty-tree pivot rule was
  honored: `SWD-SERIAL-EXTRACTION.4e` committed cleanly first, then this tree opened.
- `2026-08-08`: Stored paths and runtime paths are distinct concepts. A path may be canonicalized in a local
  variable for I/O, but repository-owned serialization must not retain that absolute value.
- `2026-08-08`: Legacy support must be semantic and bounded, not a replacement of one known username/mount
  prefix with another. Exact rules are the `.1` design deliverable.

## Open Questions

- Which source inputs qualify as explicitly authorized external paths, and how must an artifact label that
  exception so an absolute value cannot masquerade as repository-owned data? Owner: `.1`; blocking design.
- Should persisted paths remain `PathBuf` strings under schema v1 with loader normalization, or gain an
  explicit typed path-origin wrapper/schema revision? Owner: `.1`; decide by compatibility tests.
- Can the 262,592 repeated evidence `source_path` values be safely normalized during loading/writing without
  changing fact identity or deterministic hashes? Owner: `.2`; verify before migration.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-08` | `.0` | old-root absence; generated path/file/stage/key census; producer assignment and consumer-use cross-read; doctrine + mdBook checks | 335 affected files / 262,996 values; root cause localized; ownership complete; no generated mutation |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `ARTIFACT-PATH-PORTABILITY.0 — own the move-portability repair` | Measurement/ownership only; `.1` is the first implementation gate. |

## Changelog

- `2026-08-08`: Created after the SWD `.4e` fresh-build path exposed stale absolute lineage. Measured the
  complete present generated surface, localized producer and consumer seams, and split design, stage migration,
  data migration/enforcement, and closure into independently committable leaves.
