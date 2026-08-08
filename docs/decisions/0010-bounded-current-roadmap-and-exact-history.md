# 0010 — Roadmap keeps current direction; an exact capsule keeps accumulated history

- Date: 2026-08-08
- Status: accepted
- Deciders: project owner, SpecForge repository workflow

## Context

`ROADMAP.md` is the canonical high-level direction surface, but its 1,487-line / 183,445-byte current
form also contains accumulated delivery chronology. The 23 workstream records occupy 1,291 lines /
168,815 bytes; their top-level `done:` blocks alone occupy 722 lines / 129,242 bytes. Current goals,
remaining scope, obsolete status labels, superseded adapter history, and delivery detail coexist
inside the same H3 records, so no heading-only split can separate current direction losslessly.

The audit also proved current-truth drift. R14 still says `Not Started` although
`R14-SIGNAL-RESOLVE` is done; R7 and the adapter lane retain `In Progress` labels after their owning
trees closed; the recommended order still asks for the completed R15 graph migration. Later
chronology describes some completions, but it did not correct the canonical lane summaries.

## Decision

Keep `ROADMAP.md` as the stable, human-authored current-direction path. Its bounded form will contain:

1. the objective, canonical pipeline, and current implementation doctrine;
2. current strategic priorities;
3. exactly one concise row for each of the 23 high-level workstreams, with a direct owning task-tree
   route;
4. only forward implementation order; and
5. direct routes to the complete task catalog, current change history, and roadmap history archive.

Before rewriting any roadmap record, freeze the exact current source at
`docs/archive/roadmap/source-through-2026-08-08.md`. A bounded archive index and JSON manifest record
the capsule path, source SHA-256 and metrics, seal date, reason, and verifier. The capsule is an
immutable terminal and preserves every original byte, including contradictory or superseded prose as
historical evidence. Current truth is corrected in the root rather than by editing the capsule.

`doctrine/live_document_size/roadmap_projection.json` pins the pre-migration identity, five exhaustive
source regions, all 23 workstream ids and owners, four drift findings, every known reader/writer, the
archive topology, and independent current-root limits. The executable checker runs in the live-
document doctrine before migration. After migration, it switches identity authority to the capsule,
validates manifest/index retrieval, enforces the bounded root, requires all 23 workstream rows and
owner routes exactly once, and rejects delivery-chronology blocks in the current roadmap.

The current root targets at most 256 lines / 32,768 bytes / 384 bytes per line and hard-fails above
384 lines / 49,152 bytes / 512 bytes per line. These limits accommodate the measured 70-line doctrine
prologue, 23 bounded rows, current priorities/order, and retrieval routes without banking capacity for
another delivery ledger.

## Consequences

- `.5e.i` changes no roadmap record; it only pins the source and activates the pre-migration contract.
- `.5e.ii` must copy and verify the exact capsule before replacing `ROADMAP.md`, resolve the four
  measured stale claims against task-tree/code truth, migrate all readers and surface records in the
  same commit, and prove direct current/history retrieval.
- `docs/TASK_TREE.md` remains the complete live execution catalog. The roadmap summarizes high-level
  direction and never mirrors leaf/frontier/commit chronology.
- Future delivery detail belongs in task trees and `CHANGES.md`; material high-level direction or
  milestone changes update the bounded root under `COMMIT.md`.
- Git history remains useful evidence but is not the only terminal for the pre-containment roadmap.

## Links

- `doctrine/live_document_size/roadmap_projection.json`
- `scripts/check_roadmap_projection_contract.pl`
- `docs/knowledge/roadmap-current-history-boundary.md`
- `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md`
