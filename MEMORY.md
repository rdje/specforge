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
- κ-study extended `2026-06-05`: **`actor_signal_relation` κ = 1.00** (11/11, 2nd blind agent); **Ollama IS reachable** (started `ollama serve`; `which ollama`=/opt/homebrew/bin/ollama; models pulled = `qwen3-vl:8b`, `qwen2.5vl:7b` ONLY). Cross-model run: **qwen3-vl:8b** correct on single items but thinking NOT disable-able (`think:false`/`/no_think` ignored, ~3400 tok/signal) → too slow (partial 5/6); **qwen2.5vl:7b** fast (21 s) but **κ = 0.285** — it mislabels condition/trigger signals as obligations + invents labels (a model-competence issue, NOT a gold flaw; capable Claude reviewer κ = 0.90 is the signal). Side-finding: qwen2.5vl:7b = SpecForge's DEFAULT extraction VLM → raw extraction likely over-constrains "when X …" conditions.
- LOCAL MODELS NOW (2026-06-05): removed `qwen3-vl:8b` (too-slow thinking); pulled **`qwen2.5:14b-instruct`** (text-only, 9 GB) per user. Local set = `qwen2.5:14b-instruct` (text) + `qwen2.5vl:7b` (vision). Ollama daemon started in-session (`ollama serve`). Results: qwen2.5:14b → **relation κ=1.00**, **constraint κ=0.498** (still trips on condition-vs-obligation in the LABELING framing), **NLI entailment 5/6** (got the subtle "PSEL must be asserted"→NOT-entailed RIGHT). KEY: **NLI framing > free-form labeling** for this nuance; **qwen2.5:14b-instruct is VIABLE for an NLI verifier** (the "NLI gated: needs a model" item is now UNBLOCKED). KM card `local-llm-for-text-reasoning`.
- next_action: PNT — **build the NLI entailment verifier** using `qwen2.5:14b-instruct` (now that it's proven viable, 5/6): a verification gate "does the source statement ENTAIL this extracted claim?" (frame as entailment-of-a-specific-claim, NOT free-form labeling). Wire it as a residual-honesty gate (claim not entailed → drop to residual). This is the real NLI-verifier tree the user was building toward. Confirm scope with user first (code change in the extraction/grounding path). Otherwise other backlog gated/marginal. Reminder: grep `KNOWLEDGE_MAP.md`; **no push until ~200 ahead or ask.**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none — NLI verifier now has a viable local model (`qwen2.5:14b-instruct`).
