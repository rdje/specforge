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
- latest_commit: `9699abf0` — "TEMPORAL-RULE-EVAL.3 — hand-labeled APB temporal gold seed …" (this `TEMPORAL-RULE-EVAL.4` close commit pending → becomes 11 ahead of pushed `d92a73e3`; push at ~30 — getting close, consider a push soon). NOTE: workspace fully recompiled; CI green baseline = 1216 tests.
- active_work_unit: none — `TEMPORAL-RULE-EVAL` **CLOSED `2026-06-02`** (`.1`–`.4` done: supervised temporal-rule P/R/F1 live end-to-end on real APB at `P=0.400 R=0.667 F1=0.500`; deterministic-producer runner builds SemanticIR from a temp EvidenceIR copy; zero extraction-behavior change). LITERATURE-GROUNDING also CLOSED `2026-06-02`.
- next_action: PNT — pick the next tree. STRONG candidate: own a fix-tree for the real finding the temporal eval just surfaced — **temporal antecedent under-capture** (multi-condition "X valid when A, B, and C are asserted" keeps only the last condition) + **degenerate self-referential rules on list-introducer headers** ("The following signals must be valid when PSEL asserted:" → bogus PSEL→PSEL rule). This is the measure→catch→fix loop (like CONSTRAINT-SUBJECT-PRECISION); the temporal parser lives in `crates/specforge/src/ir/semantic.rs`. Other LITERATURE-GROUNDING Tier-1 backlog (future trees): LTL/MTL property-template vocabulary for temporal_rules + `.isf`→PSL/SVA export; conformal LLM-tier calibration. Pre-existing gated: column-less residual (~112; needs a precision/recall decision), CHI Class-A timing matrices, CORPUS-HARDENING CHI re-ingest.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
