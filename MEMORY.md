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
- next_action: PNT — **user's converging insight (3 Qs): the text-reasoning tasks (κ annotation, NLI verifier) need a STRONG TEXT-ONLY LLM, not a VLM** (vision is only for figure/diagram/table-image extraction). We have NO text-only instruct model pulled locally (only the 2 VLMs). Proposed direction: pull a strong text instruct LLM (e.g. `qwen2.5:7b-instruct` / larger `qwen2.5:14b`/`32b-instruct`, or `qwen3:8b` text) → serves BOTH a real cross-model κ AND a future **NLI entailment verifier** (NLI = pure text semantic reasoning: negation/scope/"is asserted" vs "must be asserted" — exactly what qwen2.5vl flubbed). **CONFIRM with user before pulling** (a multi-GB download on their machine). Otherwise backlog gated/marginal. Reminder: grep `KNOWLEDGE_MAP.md`; **no push until ~200 ahead or ask.**
- paused_work: cross-model κ (capable local rater) + NLI verifier — both want a strong text-only LLM that isn't pulled yet.
- in_flight_uncommitted: none.
- blockers: no strong text-only local LLM pulled (the 2 local models are VLMs: one too slow [qwen3-vl thinking], one too weak at text [qwen2.5vl]).
