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
- latest_commit: `76644e48` — "TRACE-SEVERITY-GATING-AUDIT — … (CLEAN); record invariant; close" (then this LITERATURE-GROUNDING.2 commit pending; ~3 ahead of pushed `d92a73e3`). NOTE: `target/` was `cargo clean`ed (12.7 GiB reclaimed) — next build is a full recompile.
- active_work_unit: `LITERATURE-GROUNDING` (`.2` document-extraction + `.3` protocol-temporal-semantics grounded, verified citations; `.4`–`.12` remaining aspects + `.13` synthesis pending). Session closed 10 trees + owned/advanced LITERATURE-GROUNDING.
- next_action: PNT — continue `LITERATURE-GROUNDING.4+` (next aspects: staged IR/progressive lowering; requirements & spec mining; KG/relation extraction; multimodal fusion; neuro-symbolic/bounded-LLM; cross-document learning; IE eval methodology; uncertainty/residual-honesty; spec→RTL — web research, VERIFY every citation), then `.13` synthesis. Other gated candidates: column-less residual (~112; needs a precision/recall decision), CHI Class-A.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: `.5` baseline is Ollama-server-gated (record honestly if saturated).
