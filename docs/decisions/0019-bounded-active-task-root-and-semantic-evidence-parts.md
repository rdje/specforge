---
id: bounded-active-task-root-and-semantic-evidence-parts
date: 2026-08-09
status: accepted
scope: documentation, continuity, task-tree, active-work, archive, retrieval
---

# ADR 0019: An oversized active task uses a bounded current root over semantic evidence parts and exact provenance

## Context

`docs/tasks/PDF-VARIANT-DIGESTION.md` is active at 2,393 lines / 222,616 bytes, only 207 bytes below the
`task_evidence` byte warning. Its exact SHA-256 is `9284dce4…a19d4`; 70 commits have changed the path.

The `.1.1` census partitions every byte into 13 measured semantic regions. The section named `Current frontier`
is 1,392 lines / 124,201 bytes because it contains the complete `.9`–`.13` activity specifications and results,
not a bounded next-action view. The source has 37 formal ID declarations but 52 unique ID-bearing commit subjects.
Several older headers remain active or in progress after inline verification and commits close the work; `.6`
and `.7` are formally pending but narratively blocked; and the formal blocker section says none. It cannot safely
nominate an eligible current leaf without an explicit reconciliation.

At census input, 32 files contain the exact task path and 89 contain its id. The route-sensitive live consumers
are ROADMAP, 18 citations in two mdBook chapters, 24 Knowledge Map fact cards, one research report, and the
containment owner. None uses a Markdown fragment. Generic tools read only the stable root's H1/status/catalog
membership or staged task evidence. No executable opens or writes the target; `COMMIT.md` is the writer contract.

ADR 0018 cannot be reused: its root is terminal and may never reopen. This program remains active and must keep a
truthful current frontier plus a bounded place for future leaf contracts and evidence.

## Decision

Adopt a hybrid active topology with three distinct authorities:

1. **Current authority:** keep `docs/tasks/PDF-VARIANT-DIGESTION.md` as a bounded active root. It owns identity,
   metadata, goal/non-goals/acceptance, a normalized high-level activity table, current frontier, blockers, the
   current leaf contract/checklist when one exists, bounded recent verification/commit rows, and the route to
   detail. It contains no accumulated activity narrative.
2. **Browsable task authority:** create a bounded semantic collection under
   `docs/tasks/pdf-variant-digestion/`, led by `INDEX.md` and `manifest.json`. It owns complete leaf routing and
   detailed activity/verification evidence. The seven migration parts are:
   `program-foundation.md`, `activity-09-serial.md`, `activity-10a-10c-table-shapes.md`,
   `activity-10d-10e-layouts.md`, `activity-10f-10i-section-headings.md`, `activities-11-13.md`, and
   `verification-and-chronology.md`.
3. **Literal provenance:** copy the committed source byte-for-byte to
   `docs/archive/tasks/pdf-variant-digestion/source-through-2026-08-09.md`. The capsule is immutable and accepts
   no future writes. It is an independently hash-verified source boundary, not the active task authority.

The stable root links the bounded index. The index links the root, every live part, the manifest, and the exact
capsule. A reader reaches current work in the root, semantic detail in at most two bounded hops, and literal
legacy evidence in two hops plus the intentionally large archive terminal. Existing direct-path consumers remain
valid; none requires fragment preservation.

## Exact migration coverage

The manifest divides the source into these contiguous legacy regions and assigns each exactly once to a semantic
part; parts may reorder regions by meaning, but each marked payload must remain byte-identical to its source slice:

| Region | Source lines | Destination part |
| --- | --- | --- |
| program identity | 1–42 | `program-foundation.md` |
| serial nodes | 43–367 | `activity-09-serial.md` |
| triage node/results | 368–421 | `program-foundation.md` |
| serial frontier/history | 422–494 | `activity-09-serial.md` |
| `.10` overview through `.10c` | 495–792 | `activity-10a-10c-table-shapes.md` |
| `.10d`–`.10e` | 793–1068 | `activity-10d-10e-layouts.md` |
| `.10f`–`.10i` | 1069–1325 | `activity-10f-10i-section-headings.md` |
| `.11` | 1326–1383 | `activities-11-13.md` |
| `.12` | 1384–1585 | `activities-11-13.md` |
| `.13` | 1586–1813 | `activities-11-13.md` |
| `.2`–`.8` planning/results | 1814–2047 | `program-foundation.md` |
| decisions/questions/blockers | 2048–2064 | `program-foundation.md` |
| verification | 2065–2260 | `verification-and-chronology.md` |
| changelog | 2261–2383 | `verification-and-chronology.md` |
| tooling note | 2384–2393 | `program-foundation.md` |

Marker-delimited payload extraction, region hashes/metrics, contiguous source coverage, destination membership,
and the complete part union are executable checks. The exact capsule and semantic payloads deliberately overlap:
the capsule proves literal provenance; the parts provide bounded semantic reading and future task continuity.

### Legacy route identity

The committed path history contributes 52 fully qualified IDs through commit subjects, but the source body uses
both fully qualified IDs and tree-relative shorthand. Six history IDs—`.10p`, `.12a`, `.12b`, `.13a`, `.13c`,
and `.13d`—never occur fully qualified in the final source; four more are fully qualified only outside their
primary semantic payload. Requiring the route ID itself inside the chosen payload would therefore make the
declared `complete` state unreachable or misroute detail to chronology.

Each legacy route consequently carries both its canonical `leaf_id` and an exact `source_literal`. The latter is
schema-constrained to either the full ID or that ID with the tree prefix removed; arbitrary aliases are forbidden.
The checker extracts the canonical route set from commit subjects, then token-matches each source literal inside
its declared semantic payload. Index and manifest routes remain fully qualified. This preserves historical
spelling without weakening stable identity or inventing normalized text in the exact legacy payloads.

## Current-state precedence

The migrated root may normalize current state but may not alter literal history. Apply these rules in order:

1. A leaf with an ID-bearing completion commit plus inline verification/completion evidence is `done`, even when
   an older header says active or in progress.
2. A container is `done` when every declared child is terminal and the activity's later evidence says complete.
3. A pending/in-progress statement paired with an explicit unmet dependency is `blocked`, not eligible.
4. Conflicting or stale candidates without enough evidence remain blocked behind a named revalidation condition;
   they are never guessed onto the frontier.
5. Historical status text remains byte-identical in the capsule and marked legacy payloads.

Applying the conservative rule leaves no eligible PDF leaf at migration. `.6` and `.7` require current corpus and
cross-task revalidation; `.9.10` remains blocked on participation-based signal identity from
`NLP-SHALLOW-PARSE`. The root stays `active` because the broad goal is open, but its current frontier states
`No eligible frontier.` A future PDF continuation must create a newly scoped child under a new top-level activity,
with current evidence; it must not silently resume a stale legacy header.

## Writer transaction

After migration, every task-changing commit is one atomic transaction:

1. Update the stable root's normalized activity state, current frontier/blockers, current leaf contract, and
   bounded recent verification/commit window.
2. Update exactly one owning semantic activity part with the detailed leaf contract, rationale, verification,
   and completion evidence. Migrated legacy payload markers are immutable.
3. Update `INDEX.md` and `manifest.json` only when leaf routing, part membership, part state, or measured metrics
   change. Every leaf id has exactly one primary semantic route.
4. Stage the root for every product leaf so task acceptance and PNT see the current checklist/frontier. Stage the
   owning part whenever its detail changes.
5. Run the neutral active-task checker, all doctrines, and the ordinary `COMMIT.md` workflow in the same commit.

New post-migration work receives a new semantic activity part. An active part may grow only below warning. Before
the update that would reach rollover, split it at an existing child/container boundary and update the bounded
index/manifest in the same commit. Completed parts are sealed against further payload changes; reopening requires
a new continuation activity/part with a new leaf id and a link to the legacy activity.

## Local limits

These limits are derived from a planned root below 160 lines, an index with 52 legacy leaf routes and seven parts,
and migrated semantic parts no larger than 488 legacy lines / about 43 KiB. Warning is 80%; rollover is 90%.

| Surface | Health targets | Inclusive ceilings |
| --- | --- | --- |
| stable active root | 256 lines / 24,576 bytes / 512 max-line bytes | 384 / 36,864 / 1,024 |
| bounded index | 160 lines / 16,384 bytes / 384 max-line bytes | 224 / 24,576 / 768 |
| semantic parts | 16 files; 640 lines / 65,536 bytes / 512 max-line bytes each; 6,400 lines / 786,432 bytes aggregate | 24 files; 896 / 98,304 / 1,024 each; 9,600 / 1,179,648 aggregate |
| exact source capsule | exact 2,393 lines / 222,616 bytes / 191 max-line bytes | exact same values |

The manifest is schema-closed at 65,536 bytes, 1,024 maximum record/scalar bytes, 24 parts, 32 source regions,
and 128 leaf routes. No existing ceiling increases. The generic `task_evidence` surface continues to classify the
stable root; dedicated index, semantic-part, and archive-terminal surfaces classify the new Markdown paths exactly
once. The neutral checker applies the tighter root/manifest rules.

## Migration stages

1. `.2.1` implements a project-neutral data contract/checker in `source_locked` state, pins the exact source,
   validates limits/region/route schema, rejects every premature destination, and runs through doctrine.
2. `.2.2` independently pins the committed `.2.1` source boundary and complete region/leaf/part migration inputs
   while every destination remains absent.
3. `.3.1` copies that committed source byte-for-byte, generates the marked semantic parts/index/manifest, replaces
   the stable root with normalized active current state, switches to `migrated`, updates surface/reader contracts,
   and proves the full transaction.
4. `.3.2` performs a fresh-reader audit: startup/PNT, all leaf routes, exact provenance, future-writer simulation,
   limits, residue, mdBook, and full CI. It closes this containment tree, not the active PDF program.

## Consequences

- Startup reads a small truthful active root instead of a 222 KiB mixed-history document.
- Existing path citations keep working, and every legacy leaf/detail remains directly browsable or exactly
  retrievable.
- Current-state correction is explicit and evidence-backed; stale headers survive only as historical evidence.
- Future task work has a single declared write transaction and bounded growth/rotation rule.
- Exact provenance duplicates semantic payload bytes by design, so both archive and live-part aggregates are
  measured and governed separately.
- Another active task tree may adopt this architecture only through its own measured contract; ADR 0019 does not
  authorize heuristic or threshold-copying migration.

## Links

- `docs/tasks/ACTIVE-TASK-EVIDENCE-CONTAINMENT.md` (`.1.1` / `.1.2`)
- `docs/research/active-task-evidence-containment-census.md`
- `docs/decisions/0018-terminal-task-tree-current-history-boundary.md`
- `docs/TASK_TREE.md`
- `COMMIT.md`
