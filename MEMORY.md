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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.v.ii` is complete and awaiting its clean commit. `.e.v.i` is
  committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: all five proof-bearing stages use compiler-derived production-semantic implementation identity,
  and the exact live denominator is 77 production modules / 41 claim families / 168 artifact fields. The new
  dependency-disconnected analyzer accounts for all 77 inventoried files, derives four targets / 77 target
  modules / 3,180 items / 1,192 imports / 25,347 calls / five macro definitions / 9,610 invocations, and fails
  closed on unresolved structure. No registered whole-core AST information-flow doctrine exists yet.
- Next action: after this clean commit, activate `.6d.ii.e.v.iii` and enforce raw/identity noninterference plus
  proof-only canonical promotion over the derived graph.
- In-flight uncommitted: completed `.e.v.ii` code/docs awaiting the current commit workflow. No background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
