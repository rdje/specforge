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
- latest_commit: `0ad715d1` — "TEMPORAL-RULE-EVAL — record .3 producer calibration …" (this `TEMPORAL-RULE-EVAL.3` commit pending → becomes 10 ahead of pushed `d92a73e3`; push at ~30). NOTE: workspace fully recompiled; CI green baseline = 1215 tests.
- active_work_unit: `TEMPORAL-RULE-EVAL` — `.1` design + `.2` scorer + `.3` gold seed DONE. `.3` = `crates/specforge/test_data/llm_eval/seed_apb_temporal.json` (6 APB temporal_rule items: 2 TPs, 1 antecedent-under-capture FN/FP, 3 negatives; real statement_ids; labels judged from prose) + loader test + README schema. CI green 1215. LITERATURE-GROUNDING CLOSED `2026-06-02`.
- next_action: PNT — `TEMPORAL-RULE-EVAL.4` (the CLOSING leaf): wire the deterministic-producer runner — build SemanticIR from a TEMP COPY of the doc's EvidenceIR (mirror `eval_extraction::extract_on_copy`'s temp-redirect so the corpus artifact is never mutated; for temporal there is no LLM command — just run `semantic` on the copy and read `temporal_rules`), index via `index_temporal_rule_predictions`, score vs `seed_apb_temporal.json`, integrate into the `eval-extraction` report (add a `TaskRecords::TemporalRules` arm / temporal producer path so the `EvalTask::TemporalRule` arm no longer errors); verify on the real APB EvidenceIR (expect ~P=0.4/R=0.67 given the seed's FP/FN cases); book note `quality/extraction-eval.md`; close per BOOK-METHOD-DOC. APB doc_key = `ihi0024_e_2023_02_amba_5_apb_protocol_specification`; producer cmd = `cargo run -p specforge -- semantic <evidence_ir> --dry-run`. Other Tier-1 backlog (future trees): LTL/MTL templates + `.isf`→PSL/SVA export; conformal LLM-tier calibration.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none for `.4` (code; producer artifact exists). Gated elsewhere: column-less residual (~112; needs a precision/recall decision), CHI Class-A timing matrices.
