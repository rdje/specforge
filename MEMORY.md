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
- latest_commit: `cc9dfb95` — "docs: record measured qwen2.5vl-vs-qwen3-vl:8b A/B in decision 0002 (provider default)"  (ahead of origin `97aedd08`: ~25; push at ~30 — PUSH SOON)
- active_work_unit: none — `CONSTRAINT-SUBJECT-PRECISION` CLOSED (the eval's first catch, fixed). This session also CLOSED: MEMORY-ARCHITECTURE-DOC, LLM-EXTRACTION-EVAL (+ qwen3-vl:8b A/B measured → decision 0002), and owned LITERATURE-GROUNDING (`.1` design; survey deferred). PNT frontier.
- next_action: PNT — pick next roadmap-aligned slice. Open candidates: `LITERATURE-GROUNDING.2+` (the research survey — web access), broad column-less recall residual (~112; needs a precision/recall decision), CHI Class-A tables, more corpus specs (CPU ingest works), or expand the eval seed for a firmer qwen3 verdict. **Push imminent (~25 ahead).**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: `.5` baseline is Ollama-server-gated (record honestly if saturated).
