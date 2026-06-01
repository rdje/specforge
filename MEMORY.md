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
- latest_commit: `37fa9743` — "MEMORY-ARCHITECTURE-DOC.4 — install agnostic enforcement kit (check script + githooks + CI + bootstrap pointers)"  (ahead of origin `97aedd08`: ~15; push at ~30)
- active_work_unit: none — `MEMORY-ARCHITECTURE-DOC` CLOSED (portable standard + in-repo memory architecture w/ enforcement). PNT frontier.
- next_action: resume `LLM-EXTRACTION-EVAL` at `.2` (pure scorer + dataset format/loader: task canonical keys, TP/FP/FN→P/R/F1, closed-world on labeled statements; unit-tested), OR pick another roadmap-aligned slice.
- paused_work: `LLM-EXTRACTION-EVAL` (`.1` design done; `.2` next).
- in_flight_uncommitted: none.
- blockers: none.
