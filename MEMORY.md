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
- latest_commit: `542bfdf3` — "TEMPORAL-RULE-EVAL.1 — own + design supervised temporal-rule eval (first Tier-1 grounding-backlog item)" (this `TEMPORAL-RULE-EVAL.2` commit pending → becomes 8 ahead of pushed `d92a73e3`; push at ~30). NOTE: workspace is now fully recompiled (post `cargo clean`); CI green baseline = 1214 tests.
- active_work_unit: `TEMPORAL-RULE-EVAL` — `.1` design + `.2` scorer DONE (supervised P/R/F1 for mined temporal rules; `eval.rs` now has `EvalTask::TemporalRule` + `GoldFact::TemporalRule` + `temporal_predicate_key`/`temporal_rule_key`/`temporal_rule_record_key` + `index_temporal_rule_predictions`; `score_dataset` unchanged-because-generic; 3 new tests; runner kept compiling honestly; zero extraction-behavior change; CI green 1214). LITERATURE-GROUNDING CLOSED `2026-06-02`.
- next_action: PNT — `TEMPORAL-RULE-EVAL.3`: hand-labeled APB temporal gold seed under `crates/specforge/test_data/llm_eval/` (≥6 items incl. ≥2 negatives; gold judged independently from the APB prose; extend `load_eval_dataset`/`committed_seed_dataset` test coverage for the new variant). PRODUCER CALIBRATION ALREADY DONE (in `TEMPORAL-RULE-EVAL.md` `.3` node): APB SemanticIR emits 153 temporal_rules via `cargo run -p specforge -- semantic generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --dry-run`; conventions = edge `rising`, clock `PCLK`, "must be valid"→`signal_value VALID post_tick` cycle_window {1,1} (+ `actor_drives_signal` when driver known), rules per-signal. Real temporal statements incl. statement_0201/0202. Then `.4` runner (build SemanticIR from a temp EvidenceIR copy → read `temporal_rules` → score; no corpus mutation) + report + book note (`quality/extraction-eval.md`) + close per BOOK-METHOD-DOC. Other Tier-1 backlog candidates (future trees): LTL/MTL templates + `.isf`→PSL/SVA export; conformal LLM-tier calibration.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none for `.3` (test-data + small loader test). Gated elsewhere: column-less residual (~112; needs a precision/recall decision), CHI Class-A timing matrices.
