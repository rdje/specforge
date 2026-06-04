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
- latest_commit: `68b7e7bb` — "PRIOR-DECAY.2 …; close tree" (this `DEMPSTER-FUSION-COMBINER.2` close commit pending → ~33 ahead of pushed `d92a73e3`). **PUSH THRESHOLD = 200** (user directive 2026-06-04; `COMMIT.md` authoritative) — no push due until ~200 ahead or explicit `push`. CI green baseline = **1239 tests** (was 1233; +6 Dempster). KM live (12 facts after this commit).
- active_work_unit: none — **BOTH user picks delivered.** `PRIOR-DECAY` CLOSED (read-only `contested_priors()` detects cross-doc contradicting priors; additive; KM `contested-priors`). `DEMPSTER-FUSION-COMBINER` **CLOSED `2026-06-04`** (replaced `min` confidence in `ir/fusion.rs::merge_cluster` with Dempster corroboration on the agreement path: mass High .9/Med .7/Low .5, `m=1−∏(1−mᵢ)`; Medium+Medium→High; disagreement still keeps `min`→Residual; K=0 here so Zadeh guard deferred; 6 tests; book `pipeline/semanticir.md`; KM `dempster-fusion`; churn = 1 unit test; CI green 1239).
- next_action: PNT — both user picks done; back to the **big/gated/marginal** backlog (clean ungated tier was exhausted earlier this run). Remaining: **big** (NLI entailment verifier [needs a model]; TEDS/GriTS table metric [needs table gold]; the deferred SVA export); **gated** (conformal calibration [held-out set]; κ/α [2nd annotator]; SVA-vs-FSMGen [user decision + FSMGen response]); **marginal-clean** (MUC near-miss eval diagnostic; per-relation P/R/F1 breakdown); **deferred-adopt ledger** items (MLIR-style stage verifier, per-relation gold scorer, consultation down-weight of contested priors, full graded-conflict Zadeh-guarded DS). Recommend a user steer for the big/gated ones, or take a marginal-clean one. Reminder: card durable facts; grep `KNOWLEDGE_MAP.md` first; **no push until ~200 ahead or explicit ask.**
- paused_work: `TEMPORAL-RULE-SVA-RENDER` (deferred, awaiting user decision vs FSMGen-native LTL/MTL).
- in_flight_uncommitted: none.
- blockers: remaining trees are gated (data/annotator/user-decision) or marginal/big.
