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
- latest_commit: `be25bbea` — "docs: LLM-EXTRACTION-EVAL — record the .4 runner design decision"  (ahead of origin `97aedd08`: ~21; push at ~30; will push around 30)
- active_work_unit: `LLM-EXTRACTION-EVAL`  →  frontier leaf: `.5` (pending) — run the baseline with `--provider ollama --model qwen2.5vl:7b` (Ollama-server-gated; record honestly if saturated), add the book note (`pipeline/` or a quality page), then close the tree. (`.4` `eval-extraction` runner landed: temp-redirect over the real command path; skip-mode baseline live = sigcon P0.5/R1.0, relation P0.5/R0.33.)
- next_action: implement `LLM-EXTRACTION-EVAL.5` — attempt the qwen2.5vl baseline run (server-gated), book note, close. The qwen3-vl:8b A/B is then `eval-extraction <seed> --provider ollama --model qwen3-vl:8b` vs qwen2.5vl.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: `.5` baseline is Ollama-server-gated (record honestly if saturated).
