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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.v.iii` is complete in the handoff commit; `.e.v.iv` is the
  next leaf only after the director resumes from this clean state. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: all five proof-bearing stages use compiler-derived production-semantic implementation identity,
  and the exact live denominator is 77 production modules / 41 claim families / 168 artifact fields. The new
  dependency-disconnected analyzer now also joins a closed 140-row boundary registry and checks 2,169 functions /
  11,295 helper edges / 10,419 semantic decisions for raw/identity noninterference, protected authority, and
  proof-only canonical promotion. All eight doctrines, 1,952/6/0 Rust tests, mdBook, containment, locality, and
  cleanup pass. The standalone flow gate is deliberately not registered yet.
- Next action: on director resume, activate `.e.v.iv` from the clean tree and compose the existing dependency,
  inventory, rule, graph, and flow checks into one unconditional doctrine.
- In-flight uncommitted: none after the handoff commit. No background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
