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
- latest_commit: `8ed47545` — "KNOWLEDGE-MAP-ADOPTION.1 — own + design adoption of the portable Knowledge Map retrieval layer" (this `KNOWLEDGE-MAP-ADOPTION.2` close commit pending → becomes 18 ahead of pushed `d92a73e3`; **PUSH due-ish, currently 18, threshold ~30**). NOTE: CI green baseline = 1219 tests. The KM is now live: `KNOWLEDGE_MAP.md` (3 facts/15 keys); before re-deriving any fact, grep it first.
- active_work_unit: none — `KNOWLEDGE-MAP-ADOPTION` **CLOSED `2026-06-04`** (adopted the portable Knowledge Map retrieval layer: `knowledge-map/` bundle copied verbatim; 3 seed cards under `docs/knowledge/`; derived `KNOWLEDGE_MAP.md`; gated derive-and-diff in pre-commit + `run_ci.sh` beside the memory-arch gate; registered in every bootstrap surface; negative-tested; CI green 1219). FIVE trees CLOSED this run: LITERATURE-GROUNDING, TEMPORAL-RULE-EVAL, TEMPORAL-ANTECEDENT-RECALL, RECALL-CHAO-ESTIMATOR, KNOWLEDGE-MAP-ADOPTION.
- next_action: PNT — pick the next tree. **Going forward: write a `docs/knowledge/<id>.md` card whenever a durable fact is established or archaeology is caught** (the pre-commit hook auto-regenerates+stages `KNOWLEDGE_MAP.md`). Remaining LITERATURE-GROUNDING backlog candidates: Tier-1 LTL/MTL property-template vocabulary for `temporal_rules` + `.isf`→PSL/SVA export (BIG — IR + adapter + FSMGen contract, read the handoff first); conformal LLM-tier calibration (needs more labeled data). Tier-2 NLI verifier; Dempster fusion; κ/α (gold single-source → needs 2nd annotator). Gated: column-less residual (~112; needs a precision/recall decision), CHI Class-A, APB/CHI re-ingest (Docling works w/ DOCLING_DEVICE=cpu). PUSH due-ish (18 ahead).
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
