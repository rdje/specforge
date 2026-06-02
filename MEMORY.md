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
- latest_commit: `260afdac` — "LLM-EXTRACTION-EVAL.4 — eval-extraction runner (temp-redirect over the real command path); skip baseline live"  (ahead of origin `97aedd08`: ~21; push at ~30)
- active_work_unit: none — `LLM-EXTRACTION-EVAL` CLOSED (`.1`–`.5`: scorer + seed + `eval-extraction` runner + live qwen2.5vl baseline + book). `MEMORY-ARCHITECTURE-DOC` also CLOSED this session. PNT frontier.
- next_action: PNT — pick next roadmap-aligned slice. Candidate offered to user: the **qwen2.5-vs-qwen3-vl:8b A/B** (pull `qwen3-vl:8b` ~6.1G, then `eval-extraction crates/specforge/test_data/llm_eval/seed_apb.json --provider ollama --model qwen3-vl:8b` vs qwen2.5vl) — gives a measured answer to "is qwen3-vl better"; pending user OK on the pull. Other open: broad column-less recall residual (~112), CHI Class-A tables, more corpus specs (re-ingest now works on CPU).
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: `.5` baseline is Ollama-server-gated (record honestly if saturated).
