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
- latest_commit: `ed05e5b7` — "SPEC-MINING-PROVENANCE.3 …; close tree" (this `DECISION-0005` ADR-capture commit pending → becomes 26 ahead of pushed `d92a73e3`; **PUSH approaching ~30**). NOTE: CI green baseline = 1229 tests. KM live (`KNOWLEDGE_MAP.md`, now 8 facts).
- active_work_unit: none — `SPEC-MINING-PROVENANCE` **CLOSED `2026-06-04`**. Recording **ADR 0005** (user-surfaced design decision): temporal behavior = **LTL/MTL** (used now via `temporal_rules`/`ir/temporal_ltl.rs`; `G(ante→cons)`, MTL `F[min,max]`; Pnueli/GoldMine/Texada, all linear-time); **NOT CTL** (branching-time doesn't match a spec's single intended linear behavior); **NOT TLA+** (a full authoring+model-checking environment, orthogonal to mining — could be a future export target like PSL/SVA, not planned); SpecForge **mines, does not model-check** (checking is downstream/out-of-scope). ADR `docs/decisions/0005` + KM card `temporal-logic-choice` + INDEX updated. NINE trees CLOSED this run.
- next_action: PNT — continue. Backlog (each a fresh tree): `.isf`→PSL/SVA export (the LTL renderer's downstream consumer — read the FSMGen handoff contract FIRST); conformal LLM-tier calibration; NLI entailment verifier; Dempster-rule fusion combiner; or other deferred-adopt items in `adopt-defer-ledger.md` (TEDS/GriTS metric, per-relation gold scorer, κ/α, MLIR-style stage verifier, prior decay). **PUSH approaching (~26 ahead; push at ~30 or on ask).** Reminder: card durable facts; grep `KNOWLEDGE_MAP.md` before re-deriving.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
