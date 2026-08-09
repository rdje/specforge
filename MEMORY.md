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
- Active unit: `CORPUS-COVERAGE.2`; `.2.37` signoff is complete and the next action is to select/own `.2.38`.
- Current state: 37 refreshes are complete / 19 remain, with 80/6/80/79 stage census and all 66 current emitted
  ISFs strict-clean. The 25-page AArch64 External Debug Guide refresh is portable and deterministic. Current
  generic authority removes 64 stale heuristic interfaces, one visual-label actor, and the fabricated 73-signal
  target while retaining 243 statements, five conditionals, 78 behaviors, and the genuine methodology-guide
  classification. The adapter blocks honestly with exactly `adapter.json`.
- Next action: commit `.2.37`, then select and own `.2.38` from the remaining ARM-guide/OpenCAPI tail.
- In-flight uncommitted: `.2.37` generated stages are verified; live-doc/mdBook/fact synchronization and commit
  signoff are in progress. Authenticated task evidence must remain until the final gates are green.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
