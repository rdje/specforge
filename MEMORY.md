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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vi` is complete, fully qualified, and pending commit.
  `.6d.ii.e.iv.v`, `.6d.ii.e.iv.iv`, `.6d.ii.e.iv.iii`, `.6d.ii.e.iv.ii`, `.6d.ii.e.iv.i`, `.6d.ii.e.iii`,
  `.6d.ii.e.ii`, `.6d.ii.e.i`, `.6d.ii.d.iv`, `.6d.ii.d.iii`, `.6d.ii.d.ii`, `.6d.ii.d.i`,
  `.6d.ii.c`, `FSMGEN-REFRESH-INTEGRATE-8.1`, `.6d.ii.b`, `.6d.ii.a`, and
  `.6d.i` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2`
  and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: SourceIR through the ISF adapter have executable cumulative authority. Adapter schema 2 proves 12
  fields across four families plus every populated array record, nonblank rendered ISF line, and blocking reason;
  canonical build/load/serialize/write/reconcile and validation mutation reject proofless, stale, forged,
  unauthorized, legacy, and future authority. Exactly 24 reachable adapters migrated with zero pre-existing
  public-field delta; all are honestly blocked and reconcile to zero emitted files, while 54 chains remain
  proof-unmeasurable. The checked graph is 41 families / 168 rules / 116 entrypoints / 53 seams / four bypasses.
  All eight doctrines, 1,947/6/0 Rust tests, five compile-fail doctests, 156/156 KG fixtures, mdBook, and locality
  pass. `.e.iv.vii` owns whole-module digest over-invalidation.
- Next action: commit `.e.iv.vi` with its required subject, verify a clean boundary and zero-byte brief, then
  activate `.e.iv.vii`.
- In-flight uncommitted: fully qualified adapter proof implementation, exact 24-artifact migration,
  enforcement/currency correction, and public/live/book/retrieval/task alignment. No background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
