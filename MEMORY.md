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
- Active unit: closing `FSMGEN-REFRESH-INTEGRATE-8.1`; `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b`, `.6d.ii.a`, `.6d.i`,
  `.6c.ii`, `FSMGEN-REFRESH-INTEGRATE-6.1`, and the verified no-op `FSMGEN-REFRESH-INTEGRATE-7.1` are committed
  complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: refresh cycle 8 independently live-verified FSMGen `origin/main` at `c0d8b668d`, audited the
  exact four-commit/15-path delta as downstream-contract-neutral, advanced the detached clean gitlink, passed all
  seven strict canaries, performed its induced exact `CHANGES.md` rollover, and passed full CI.
- Next action: commit the closed `.8.1` leaf through `COMMIT.md`, verify a clean handoff, then activate and resume
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c`.
- In-flight uncommitted: the fully verified, staged refresh commit only; no submodule-local source change and no
  background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
