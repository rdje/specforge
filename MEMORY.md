# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Derive the current repository revision on read with `git rev-parse HEAD`; never store a
  latest-commit shadow that the recording commit would immediately invalidate.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (the 4th
  architecture — doctrines are mechanically gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh` (the registry/driver for memory,
  knowledge-map, task-acceptance, README policy, and live-document containment); hooks + CI
  run it too. Retrieval starts at bounded `KNOWLEDGE_MAP.md`, then searches its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.3.2` — independently audit and close the bounded active-task
  containment program.
- Current state: `.3.1` migrated the contract to `migrated/complete`. The stable PDF task path is a 106-line
  active current root over a 79-line index, seven bounded semantic parts, 52 primary routes, and the exact
  2,393-line / 222,616-byte source capsule. Thirty-four focused cases and full CI pass.
- Next action: start `.3.2` from the clean `.3.1` commit and independently prove fresh-clone retrieval, semantic
  equivalence, ordinary future append safety, and final book/task closure.
- In-flight uncommitted: none after the `.3.1` commit; product artifacts, shared inputs, thresholds, and ceilings
  are unchanged.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
