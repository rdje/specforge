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
- latest_commit: `be2b95d8` — "RECALL-CHAO-ESTIMATOR.1 — own + design the Chao second recall estimate …" (this `RECALL-CHAO-ESTIMATOR.2` close commit pending → becomes 16 ahead of pushed `d92a73e3`; **push due-ish — consider pushing now or after the next 1–2 commits**). NOTE: workspace fully recompiled; CI green baseline = 1219 tests. FINDING (standing): the temporal eval's remaining 2 FPs (degenerate PSEL-header / `*_WIDTH` subject) are **stale-artifact only, NOT a live bug** — current `evidence.rs` already strips `" when "` + excludes `*_WIDTH`; a clean APB re-ingest would simply confirm.
- active_work_unit: none — `RECALL-CHAO-ESTIMATOR` **CLOSED `2026-06-02`** (Chao 1987 heterogeneity-robust second recall estimate added alongside Lincoln–Petersen; `validate` reports the LP/Chao range + 2 Chao metrics; book note; additive, zero behavior change; CI green 1219). FOUR trees CLOSED this continuation: LITERATURE-GROUNDING, TEMPORAL-RULE-EVAL, TEMPORAL-ANTECEDENT-RECALL, RECALL-CHAO-ESTIMATOR.
- next_action: PNT — pick the next tree. Remaining LITERATURE-GROUNDING backlog candidates (each a fresh tree): **Tier-1** LTL/MTL property-template vocabulary for `temporal_rules` + `.isf`→PSL/SVA export (BIG — touches IR + adapter + the FSMGen contract, read the FSMGen handoff first); conformal LLM-tier calibration (needs more labeled data). **Tier-2** NLI-based entailment verifier for LLM claims; Dempster-rule fusion confidence combiner; inter-annotator agreement (κ/α) — but gold is single-source so κ needs a 2nd annotator first. Pre-existing gated: column-less residual (~112; needs a precision/recall decision), CHI Class-A timing matrices, CORPUS-HARDENING CHI re-ingest, APB re-ingest + eval re-confirm (now ungated; Docling works w/ DOCLING_DEVICE=cpu). PUSH is due-ish (16 ahead).
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
