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
- latest_commit: `ca3c18de` — "LITERATURE-GROUNDING.13 — synthesis map + reach-full-potential backlog + book mirror; close tree" (this `TEMPORAL-RULE-EVAL.1` design commit pending → becomes 7 ahead of pushed `d92a73e3`; push at ~30). NOTE: `target/` was `cargo clean`ed earlier; `.1` is docs-only (no rebuild) — `TEMPORAL-RULE-EVAL.2`+ (code) triggers a full recompile.
- active_work_unit: `TEMPORAL-RULE-EVAL` — `.1` design DONE (supervised P/R/F1 for mined temporal rules; first Tier-1 item from the LITERATURE-GROUNDING backlog; reuses `eval.rs` closed-world scorer + a new semantic provenance-free canonical key over `TemporalRuleRecord`; producer = deterministic temporal parser; zero extraction-behavior change). LITERATURE-GROUNDING CLOSED `2026-06-02`.
- next_action: PNT — `TEMPORAL-RULE-EVAL.2`: add `EvalTask::TemporalRule` + `GoldFact::TemporalRule` + `temporal_rule_record_key` + a `temporal_predicate_key` helper + prediction indexing in `crates/specforge/src/eval.rs`; unit tests (gold↔record agreement, antecedent/consequent order-insensitivity, negative item, cycle-window equality); `scripts/run_ci.sh` green (first run = full recompile after the clean). Then `.3` gold seed, `.4` runner+report+book+close. Other Tier-1 backlog candidates (future trees): LTL/MTL templates + `.isf`→PSL/SVA export; conformal LLM-tier calibration.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none for `.2` (code; recompile expected). Gated elsewhere: column-less residual (~112; needs a precision/recall decision), CHI Class-A timing matrices.
