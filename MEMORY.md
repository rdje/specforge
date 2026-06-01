# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (the memory system) and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 30 commits; artifact cleanup ≥ every 24h).
- Durable cross-cutting facts: `docs/decisions/` (e.g. Docling CPU device 0001,
  LLM/VLM provider default 0002).

## Current state (OVERWRITE this block each update — do not append)
- latest_commit: `583aa204` — "MEMORY-ARCHITECTURE-DOC.3 — demote MEMORY.md to the bounded resume pointer (1395->25 lines)"  (ahead of origin `97aedd08`: ~14; push at ~30)
- active_work_unit: `MEMORY-ARCHITECTURE-DOC`  →  frontier leaf: `.5` (pending) — verify end-to-end (self-check + full `run_ci.sh` green) + sync live docs + close. (`.4` enforcement kit installed: check script wired into CI, `.githooks/` active via `core.hooksPath`, bootstrap pointers AGENTS/CLAUDE/.cursorrules/copilot.)
- next_action: run `MEMORY-ARCHITECTURE-DOC.5` — full `scripts/run_ci.sh` green incl. the new memory-architecture check; then close the tree.
- paused_work: `LLM-EXTRACTION-EVAL` (`.1` design done; `.2` pure scorer + dataset format is its next step) — resume after MEMORY-ARCHITECTURE-DOC closes.
- in_flight_uncommitted: none.
- blockers: none.
