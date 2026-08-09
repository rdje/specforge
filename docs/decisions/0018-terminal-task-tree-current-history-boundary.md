---
id: terminal-task-tree-current-history-boundary
date: 2026-08-09
status: accepted
scope: documentation, continuity, task-tree, archive, retrieval
---

# ADR 0018: A completed oversized task tree becomes a bounded closed root over an exact source capsule

## Context

`docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md` grew from 214 lines / 13,414 bytes to
2,451 lines / 232,649 bytes in 38 commits. It is 9,826 bytes beyond the 80% byte warning and has
18,026 bytes of pre-rollover headroom. The last nine completed updates averaged 49.7 lines / 5,370
bytes, so continued completion evidence would consume that margin in about three same-sized slices.

The file contains four different information roles: 651 lines of program structure and current
navigation, 356 lines of durable decisions, 1,148 lines of historical slice contracts, and 296 lines
of verification/commit/changelog evidence. Mechanical heading sharding would make the task catalog
and resume reader reconstruct current state from historical partitions. Trimming would destroy the
revision-bound evidence that the task-tree doctrine requires.

## Decision

Use a one-time terminal current/history boundary, and only after every program leaf is complete.

1. Commit a final pre-migration source boundary under `.10b.i`. That commit keeps the stable task path
   in its complete uncompressed form and pins its exact byte identity and semantic markers.
2. Under `.10b.ii`, copy that committed source byte-for-byte to
   `docs/archive/tasks/live-document-size-containment-adoption/source-through-2026-08-09.md`.
3. Replace the stable task path with a bounded closed summary. It retains the tree id, `done` metadata,
   goal/outcome, completed activity map, empty frontier, key decision routes, final verification and
   commit summary, and direct archive retrieval instructions.
4. Put the capsule behind a bounded `INDEX.md` and exact JSON manifest. A project-neutral checker must
   verify safe repository-relative paths, source hash and metrics, prior-boundary provenance, root/index
   limits, required closed-state markers, direct links, and archive immutability.

The stable root keeps its existing `task_evidence` membership. A dedicated contract ratchets this one
closed root to health targets of 256 lines / 32,768 bytes / 512 maximum line bytes and ceilings of
384 / 49,152 / 1,024. The archive index receives 64 lines / 6,144 bytes / 256 maximum line bytes as
health targets and 96 / 8,192 / 512 as ceilings. The capsule receives exact terminal metrics at the
`.10b.i` boundary. No existing ceiling increases.

The root continues to contain one `## Verification Log` and its capture-boundary table header, so the
current maintained-surface derived-state declaration remains valid. The exact historical rows live in
the capsule. Generic task-catalog readers continue to read only the stable root's H1 and first metadata
status; historical readers use root → bounded archive index → exact capsule.

This topology is not a general license to compact an active task. Any future work that would reopen this
program must use a new top-level task tree. Active oversized trees require a separately designed live
partition, not this terminal migration.

## Consequences

- The complete source is retained byte-for-byte and independently hash-verifiable.
- Ordinary startup keeps the stable task path and a small closed-state read; history remains at most two
  bounded hops away and is excluded from mandatory bootstrap reads.
- `.10b` is split into `.10b.i` (committed source/verifier boundary) and `.10b.ii` (atomic migration and
  closure), avoiding an unverifiable self-referential capsule created only inside one commit.
- The active `PDF-VARIANT-DIGESTION` tree is already 222,616 bytes, only 207 bytes below warning. It must
  receive separately task-owned active-tree containment before another PDF-variant leaf appends to it.

## Links

- `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md` (`.10a` / `.10b`)
- `docs/research/task-evidence-terminal-containment-design.md`
- `docs/knowledge/live-document-coverage-authority.md`
- `docs/TASK_TREE_README.md`
- `docs/decisions/0003-task-tree-and-commit-doctrine.md`
