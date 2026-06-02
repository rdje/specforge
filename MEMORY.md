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
- latest_commit: `265c6fc3` — "TEMPORAL-ANTECEDENT-RECALL.2 — distribute shared assertion-value across coordinated condition signals; close tree" (13 ahead of pushed `d92a73e3`; push at ~30). NOTE: workspace fully recompiled; CI green baseline = 1218 tests.
- active_work_unit: none — `TEMPORAL-ANTECEDENT-RECALL` **CLOSED `2026-06-02`** (guarded shared-value distribution in `parse_temporal_condition_predicates` recovered dropped antecedent preconditions; eval recall 0.667→1.000 on real APB `P=0.600 R=1.000 F1=0.750`; +2 tests; CI green 1218, no kg-bench regression). `TEMPORAL-RULE-EVAL` + `LITERATURE-GROUNDING` also CLOSED `2026-06-02`. This continuation closed 3 trees + delivered a live eval that found + fixed a real bug.
- next_action: PNT — pick the next tree. Candidates: (a) **constraint-tier FP follow-up** the temporal eval surfaced — degenerate self-referential rule on list-introducer headers ("The following signals must be valid when PSEL asserted:" → bogus PSEL→PSEL) + `*_WIDTH`-subject; both upstream in `evidence.rs`, partly `CONSTRAINT-SUBJECT-PRECISION`-class, **re-ingest-gated** (the on-disk APB EvidenceIR is stale — a clean re-ingest may already clear them; confirm before coding). (b) other LITERATURE-GROUNDING Tier-1 backlog (LTL/MTL property-template vocabulary + `.isf`→PSL/SVA export; conformal LLM-tier calibration). (c) pre-existing gated: column-less residual (~112; needs a precision/recall decision), CHI Class-A timing matrices, CORPUS-HARDENING CHI re-ingest. Consider a PUSH soon (13 ahead; threshold ~30).
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
