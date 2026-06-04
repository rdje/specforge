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
- latest_commit: `47f8487d` — "KM — contested-prior consultation down-weight … no-build" (this `EVAL-GOLD-INTERANNOTATOR-AGREEMENT` close commit pending → ~40 ahead of pushed `d92a73e3`). **PUSH THRESHOLD = 200**. CI green baseline = **1243 tests**. KM 15 facts. `subs/fsmgen` pinned `92d7036b`.
- active_work_unit: none — `EVAL-GOLD-INTERANNOTATOR-AGREEMENT` **CLOSED `2026-06-05`** (user unblocked the κ/α "is the gold trustworthy?" item): a blind independent agent re-annotated the 8 `signal_constraint` statements → **Cohen's κ = 0.90** (17/18, almost-perfect) → **eval gold is reliable**. One documented ambiguity (`0202`/PENABLE, "any other control signals" clause), not auto-fixed. Analysis-only; KM `eval-gold-interannotator-kappa`. (`FSMGEN-ASSERT-LOWERING` also CLOSED `2026-06-04`, `.1`–`.3`.)
- next_action: PNT — user suggested using **Ollama qwen (qwen3vl:8b)** as the 2nd rater — a cross-model, more-independent annotator; **but Ollama is NOT reachable from the sandboxed bash** (`ollama list` empty), so the qwen rater + the `actor_signal_relation` agreement are deferred to the production env (run there: re-annotate via qwen, recompute κ / 3-way Fleiss). Otherwise the clean/faithful/non-gated backlog is exhausted; remaining = gated (NLI [model], TEDS/GriTS [table gold], conformal [held-out set]) or marginal (MUC near-miss; per-relation P/R/F1; unguarded-`min>1` monitor). PRIOR-DECAY contested down-weight = already done (resolvers abstain). Reminder: grep `KNOWLEDGE_MAP.md`; **no push until ~200 ahead or ask.**
- paused_work: qwen cross-model 2nd-rater for the κ study (blocked on Ollama reachability from the shell — runs in production env).
- in_flight_uncommitted: none.
- blockers: Ollama/qwen not reachable from the sandboxed bash (so a local-LLM rater can't run here); other backlog gated (data) or marginal.
