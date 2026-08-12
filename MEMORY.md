# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey. History lives in
> `git log`; work state lives in the task-trees (`docs/tasks/`); durable facts/decisions live in
> `docs/decisions/`. Do **not** append session narration — overwrite the "Current state" block.

## How to resume (any AI, any harness)
- Derive the current revision on read with `git rev-parse HEAD`; never store a latest-commit shadow.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (doctrines are mechanically
  gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`); follow `COMMIT.md`
  after every slice (unit id in the commit subject).
- Non-negotiable doctrine: `docs/decisions/0003-task-tree-and-commit-doctrine.md` (no code change without
  an owning task-tree first; signoff quality; zero ROADMAP↔code↔mdBook drift; push ~every 200 commits;
  artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh`; hooks + CI run it too. Retrieval starts at bounded
  `KNOWLEDGE_MAP.md`, then its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iii` is complete pending its commit under `.e.iv`.
  `.6d.ii.e.iv.ii`, `.6d.ii.e.iv.i`, `.6d.ii.e.iii`, `.6d.ii.e.ii`, `.6d.ii.e.i`, `.6d.ii.d.iv`, `.6d.ii.d.iii`, `.6d.ii.d.ii`, `.6d.ii.d.i`,
  `.6d.ii.c`, `FSMGEN-REFRESH-INTEGRATE-8.1`, `.6d.ii.b`, `.6d.ii.a`, and
  `.6d.i` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2`
  and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.e.iv.iii` implementation, corpus migration, full CI, and cleanup are complete pending commit.
  EvidenceIR schema 3 proves all 39 fields / 11 families, preserves exact SourceIR proof, and closes all productive
  mutation paths. The retained 24 have zero field delta; 54 legacy EvidenceIRs are inspection-only. Currency is
  EvidenceIR 24 current, SemanticIR 24 current / 54 proof-unmeasurable, and later stages 78/78 stage-local.
- Next action: commit `.e.iv.iii`, verify clean/zero-byte brief, then activate `.e.iv.iv` from that clean boundary.
- In-flight uncommitted: the owned `.e.iv.iii` code, schemas, corpus proof metadata, ADRs, live docs, mdBook,
  Knowledge Map projection, and task evidence. Exact rollback/comparison scratch and rebuildable incremental cache
  are removed; no background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
