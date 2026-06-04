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
- latest_commit: `39803c60` — "RECALL-CHAO-ESTIMATOR.2 — add the Chao heterogeneity-robust recall estimate …; close tree" (this `KNOWLEDGE-MAP-ADOPTION.1` design commit pending → becomes 17 ahead of pushed `d92a73e3`; **PUSH due-ish at ~30, currently 17**). NOTE: CI green baseline = 1219 tests. FINDING (standing): the temporal eval's remaining 2 FPs (degenerate PSEL-header / `*_WIDTH` subject) are **stale-artifact only, NOT a live bug** — current `evidence.rs` already strips `" when "` + excludes `*_WIDTH` (a clean APB re-ingest would simply confirm). This finding is being seeded as a Knowledge Map card.
- active_work_unit: `KNOWLEDGE-MAP-ADOPTION` — `.1` design DONE (user-directed: adopt the portable `KNOWLEDGE_MAP_ARCHITECTURE.md` bundle — additive question-keyed retrieval layer that kills archaeology; bundle at `/Users/richarddje/Documents/github/pgen/knowledge-map/`). Defaults match SpecForge → copy verbatim, no config override. FOUR trees CLOSED `2026-06-02` (LITERATURE-GROUNDING, TEMPORAL-RULE-EVAL, TEMPORAL-ANTECEDENT-RECALL, RECALL-CHAO-ESTIMATOR).
- next_action: PNT — `KNOWLEDGE-MAP-ADOPTION.2`: (1) `cp -r /Users/richarddje/Documents/github/pgen/knowledge-map <repo>/` + `chmod +x` the scripts; (2) `mkdir docs/knowledge`; write 3 seed cards (`docling-device-cpu`, `llm-vlm-provider-default`, `temporal-eval-residual-fps-are-stale`) — front-matter w/ `answers:`/`id`/`title`/`date`/`evidence`+`reverify`; (3) `bash knowledge-map/scripts/gen_knowledge_map.sh` → `KNOWLEDGE_MAP.md`; (4) rewrite `.githooks/pre-commit` to run BOTH `check_memory_architecture.sh` AND the KM gen+stage+`check_knowledge_map.sh` (de-`exec`); (5) add the KM check to `scripts/run_ci.sh` (beside memory-arch step) + `.github/workflows/ci.yml`; (6) register KM in `MEMORY_ARCHITECTURE.md` + `README.md` + `AGENTS.md`/`CLAUDE.md`/`.cursorrules`/copilot; (7) verify gen/check/derive-and-diff + negative hook test + full `run_ci.sh` GREEN; close. Repo-infra + docs only (no crate rebuild). Then remaining LITERATURE-GROUNDING backlog (LTL/MTL templates; conformal; NLI/Dempster).
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
