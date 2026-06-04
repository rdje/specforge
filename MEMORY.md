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
- latest_commit: `7590a511` — "DEMPSTER-FUSION-COMBINER.2 …; close tree" (this `FSMGEN-ASSERT-MIGRATE.2` close commit — which RE-PINS `subs/fsmgen` + migrates the emission — pending → ~35 ahead of pushed `d92a73e3`). **PUSH THRESHOLD = 200** — no push until ~200 ahead or explicit `push`. CI green baseline = **1239 tests**. KM live (13 facts after this commit). **`subs/fsmgen` now pinned at `43b29f5c`** (was `c0b7eaa7`) — the verification-family generalization.
- active_work_unit: none — `FSMGEN-ASSERT-MIGRATE` **CLOSED `2026-06-04`** (consequence of FSMGen's response). FSMGen removed `(contract … (eventually …))` and generalized into `(assert/assume/cover …)`; SpecForge re-pinned `43b29f5c` + migrated the bounded-eventually emission → `(assert (monitor (within s N)))` (`isf_ir.rs`; dropped `IsfContract.name`); fsmgen-binary strict-check re-validated on the new pin (CI green 1239); `TEMPORAL-RULE-SVA-RENDER` **retired as superseded**; KM `fsmgen-temporal-isf-form`. **`stable` + `min>1` obligations stay residual** until FSMGen ships those primitives (user is requesting them).
- next_action: PNT — back to the **big/gated/marginal** backlog (clean ungated tier exhausted). Possible NEW follow-ups from FSMGen's verification family: lower MORE temporal shapes into the full `(assert (=> A B))` / `(assert (=> A (next B)))` / `(assert (=> A (within B N)))` family (today only the bounded-eventually monitor is lowered; the antecedent→consequent forms could now lower instead of residualizing) — a real future tree once worth it. Otherwise: **big** (NLI verifier [needs model]; TEDS/GriTS [needs table gold]); **gated** (conformal [held-out set]; κ/α [2nd annotator]); **marginal-clean** (MUC near-miss eval diagnostic; per-relation P/R/F1; consultation down-weight of contested priors). Recommend a user steer. Reminder: card durable facts; grep `KNOWLEDGE_MAP.md` first; **no push until ~200 ahead or explicit ask.**
- paused_work: none (`TEMPORAL-RULE-SVA-RENDER` superseded/retired).
- in_flight_uncommitted: none.
- blockers: remaining trees are gated (data/annotator) or marginal/big; `stable`/`min>1` ISF lowering blocked on FSMGen primitives (requested).
