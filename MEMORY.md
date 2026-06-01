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
- latest_commit: `3a413aa3` — "LLM-EXTRACTION-EVAL.2 — pure scorer + dataset format/loader (eval.rs); 6 tests"  (ahead of origin `97aedd08`: ~17; push at ~30)
- active_work_unit: `LLM-EXTRACTION-EVAL`  →  frontier leaf: `.4` (pending) — provider-gated runner (`eval-extraction` command): for each doc_key in the dataset run the task command with `--model`, read produced records by statement provenance (`index_*_predictions`), `score_dataset`, report per-task P/R/F1; `--provider skip` testable path. (`.2` `eval.rs` scorer + `.3` seed `test_data/llm_eval/seed_apb.json` 16 items landed.)
- next_action: implement `LLM-EXTRACTION-EVAL.4` (the runner command). Then `.5` baseline (qwen2.5vl) + book + close.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: `.5` baseline is Ollama-server-gated (record honestly if saturated).
