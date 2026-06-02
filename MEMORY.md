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
- latest_commit: `81cd24aa` — "docs: update MEMORY.md continuity baseline to pushed HEAD d92a73e3" (1 ahead of pushed `d92a73e3`; + the TRACE-SEVERITY-GATING-AUDIT close commit pending).
- active_work_unit: none — `TRACE-SEVERITY-GATING-AUDIT` CLOSED (clean: zero severity-masking; invariant in decision 0004). Session closed trees: COMPLETENESS-RECALL-RELATIONS, REGISTER-CLASSIFIER-ENCODING-FP, SIGNAL-TABLE-COLUMNLESS-RECALL, SYMBOL-CLOSURE-CORPUS-VALIDATION, DOCLING-DEVICE-CPU-DEFAULT, CORPUS-HARDENING.4, MEMORY-ARCHITECTURE-DOC, LLM-EXTRACTION-EVAL, CONSTRAINT-SUBJECT-PRECISION, TRACE-SEVERITY-GATING-AUDIT; owned LITERATURE-GROUNDING (.1). PNT frontier.
- next_action: PNT — open candidates: `LITERATURE-GROUNDING.2+` (web survey; user "later"); broad column-less residual (~112; needs a precision/recall decision); CHI Class-A; more corpus specs (CPU ingest works); expand eval seed (AHB).
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: `.5` baseline is Ollama-server-gated (record honestly if saturated).
