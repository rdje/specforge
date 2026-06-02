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
- latest_commit: `7ae56bde` — "MEMORY.md — sync resume-pointer baseline …" (this `RECALL-CHAO-ESTIMATOR.1` design commit pending → becomes 15 ahead of pushed `d92a73e3`; push at ~30). NOTE: workspace fully recompiled; CI green baseline = 1218 tests. FINDING (recorded): the temporal eval's remaining 2 FPs (degenerate PSEL-header / `*_WIDTH` subject) are **stale-artifact only, NOT a live bug** — current `evidence.rs` already strips `" when "` (`text_before_condition_marker`) + excludes `*_WIDTH` (`collect_subject_signal_tokens`); a clean APB re-ingest would simply confirm. Do not chase as a live fix.
- active_work_unit: `RECALL-CHAO-ESTIMATOR` — `.1` design DONE (Tier-2 adopt from the LITERATURE-GROUNDING backlog: add the Chao 1987 heterogeneity-robust second recall estimate alongside Lincoln–Petersen in `ir/completeness.rs::recall_estimate`). LITERATURE-GROUNDING + TEMPORAL-RULE-EVAL + TEMPORAL-ANTECEDENT-RECALL all CLOSED `2026-06-02`.
- next_action: PNT — `RECALL-CHAO-ESTIMATOR.2`: add `chao_estimated_total` + `chao_estimated_remaining_misses` to `RecallEstimate` (`ir/completeness.rs:223`); compute in `recall_estimate` (line 270 ctor) — `f1 = distinct − overlap`, `f2 = overlap`, `n_chao = (distinct as f64 + (f1*f1) as f64/(2.0*f2 as f64)).round() as usize`, `.max(distinct)`; unit test (worked example → chao 10 / misses 4; chao ≥ distinct); update the `validate` report line (`commands/validate.rs:2704`) to show the LP+Chao range + add a chao metric (near 3023/3030); book note in the recall-estimation chapter; full CI; close. Additive only (no `None`-gating change). Then other backlog: LTL/MTL templates + `.isf`→PSL/SVA export; conformal calibration. Consider a PUSH soon (15 ahead; threshold ~30).
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
