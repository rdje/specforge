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
- latest_commit: `6cec44db` — "MEMORY-ARCHITECTURE-DOC.5 — verify end-to-end (full CI green incl. memory-arch check) + sync live docs; close tree"  (ahead of origin `97aedd08`: ~16; push at ~30)
- active_work_unit: `LLM-EXTRACTION-EVAL`  →  frontier leaf: `.3` (pending) — seed the labeled dataset (~24 items, ≥8/text-task) under `crates/specforge/test_data/llm_eval/`, drafted from real AMBA/i2c statements (agent_drafted + note), incl. negatives.
- next_action: implement `LLM-EXTRACTION-EVAL.3` (seed labeled data) — `.2` pure scorer + dataset format/loader landed in `crates/specforge/src/eval.rs`. Then `.4` provider-gated runner, `.5` baseline + book + close.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: `.5` baseline is Ollama-server-gated (record honestly if saturated).
