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
- Active unit: `CLAIM-VERIFICATION-ADOPTION.2` owns the bounded claim registry and doctrine gate.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: the 21-file workflow-standard profile is committed baseline and its exact 16→21 authority is
  retired. ADR 0042/0043, the author/reviewer contract, and clean capacity boundary are ready for enforcement;
  mechanical claim provenance is not yet active.
- Next action: implement `.2`'s bounded claim registry, digest/currentness semantics, fail-closed checker, and
  unconditional doctrine-driver entry. `MEMORY.md` remains capped at 32,768 B.
- In-flight uncommitted: `.1b` authority retirement and continuity updates await focused/doctrine gates; no
  background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
