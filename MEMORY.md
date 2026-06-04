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
- latest_commit: `53b50ba2` — "KM — record .isf temporal lowering-completeness is already guaranteed (no-build)" (this push-threshold-change commit pending → ~29 ahead of pushed `d92a73e3`). **PUSH THRESHOLD RAISED 30 → 200 (user directive 2026-06-04)** — authoritative in `COMMIT.md`; auto-memory `feedback_push_cadence` updated; no push due until ~200 ahead or an explicit `push`. CI green baseline = 1229 tests. KM live (10 facts).
- active_work_unit: `PRIOR-DECAY` — user picked the next trees: **prior-decay first, then the Dempster combiner.** prior-decay = add staleness/decay/revision to the `CorpusMemory` cross-document learning plane (grounded in Parisi continual-learning + the `cross-document-learning.md` gap "priors only accrete; add decay/revision when a later validated doc contradicts a prior"). Touches `ir/prior_memory.rs` + `commands/learn_priors.rs` — careful, behavior-affecting (signoff quality; CI incl. kg-bench/learn-priors tests).
- next_action: PNT — (1) own + design `PRIOR-DECAY` (the decay/revision mechanism: what triggers staleness, how a contradicting validated doc revises a prior, advisory-only invariant preserved); (2) implement + tests + book + KM card + close; (3) then `DEMPSTER-FUSION-COMBINER` (replace `min()` confidence in `ir/fusion.rs` with a Dempster-Shafer belief-mass combiner + conflict mass K; guard the high-conflict Zadeh pathology; watch fixture churn). Reminder: card durable facts; grep `KNOWLEDGE_MAP.md` first; **no push until ~200 ahead or explicit ask.**
- paused_work: `TEMPORAL-RULE-SVA-RENDER` (deferred, awaiting user decision vs FSMGen-native LTL/MTL).
- in_flight_uncommitted: none.
- blockers: none for prior-decay (self-contained learning-plane change).
