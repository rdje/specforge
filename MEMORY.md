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
- latest_commit: `f99157e9` — "TEMPORAL-RULE-EVAL.4 — deterministic-producer runner + live APB score + book; close tree" (this `TEMPORAL-ANTECEDENT-RECALL.1` design commit pending → becomes 12 ahead of pushed `d92a73e3`; push at ~30). NOTE: workspace fully recompiled; CI green baseline = 1216 tests.
- active_work_unit: `TEMPORAL-ANTECEDENT-RECALL` — `.1` design DONE (2nd catch of the measure→catch→fix loop, surfaced by the closed TEMPORAL-RULE-EVAL). Root cause = `parse_temporal_condition_predicates` (`ir/semantic.rs` ~7215) drops leading signals of a coordinated list because the shared trailing value isn't distributed (confirmed vs real APB `sigcon_0026`). TEMPORAL-RULE-EVAL + LITERATURE-GROUNDING both CLOSED `2026-06-02`.
- next_action: PNT — `TEMPORAL-ANTECEDENT-RECALL.2`: implement the guarded shared-value distribution in `parse_temporal_condition_predicates` (≥2 signals, exactly one distinct value among clauses, fill only value-less signal clauses with that value; keep `split_temporal_condition_*` clause-splitting; dedup + handshake-enrich unchanged). Add unit test(s). RE-RUN `cargo run -p specforge -- eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json` → expect P 0.400→0.600, R 0.667→1.000 (PBUSER FN→TP). Refresh the book eval numbers in `quality/extraction-eval.md` (they will change). Full CI incl. kg-bench temporal fixtures (UPDATE any fixture whose expected antecedent legitimately gains the distributed signals). Close per BOOK-METHOD-DOC. FP cohort (degenerate header / `*_WIDTH` subject) = upstream constraint-tier + stale-artifact, out of scope (re-ingest-gated). Other Tier-1 backlog: LTL/MTL templates + `.isf`→PSL/SVA export; conformal LLM-tier calibration.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
