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
- latest_commit: `54970b5b` — "FSMGEN-ASSERT-MIGRATE.2 … re-pin 43b29f5c …" (this `FSMGEN-MIN-WINDOW-CONFIRM` close commit pending → ~36 ahead of pushed `d92a73e3`). **PUSH THRESHOLD = 200** — no push until ~200 ahead or explicit `push`. CI green baseline = **1239 tests**. KM 13 facts. **`subs/fsmgen` committed pin = `43b29f5c`** (working tree was bumped to `24a768c1` only to READ FSMGen's question; reverted back — NO re-pin committed yet).
- active_work_unit: none — `FSMGEN-MIN-WINDOW-CONFIRM` **CLOSED `2026-06-04`**: answered FSMGen's gating question for the proposed `min>1` window slice (`(within B MIN MAX)` → `##[MIN:MAX]`). Verified our `cycle_window` bounds are always **integer literals** (`Option<u32>`); `MIN=0` only as the same-cycle `[0,0]` (already residual) or a `0`-to-`N` range (= the anchored `(monitor (within S N))` form); SpecForge **guarantees `1<=MIN<=MAX`** for the consequent form → recommended FSMGen lock `1<=MIN<=MAX`. Filed in `docs/FSMGEN_FEEDBACK.md`. **FSMGen status:** `(stable/changed/rose/fell)` predicates SHIPPED (`6700fbb4`); `min>1` window will ship on this confirmation (HEAD `24a768c1`).
- next_action: **WAIT for FSMGen to ship `min>1`** (per user "wait for FSMGen feedback"). THEN the queued **FSMGen-temporal-integration** tree: re-pin `subs/fsmgen` past `min>1` (latest), migrate stability obligations off residuals → `(assert (=> <ante> (stable <sig>)))`, and lower the richer antecedent→consequent temporal family (`(assert (=> A B))`/`(next B)`/`(within B MIN MAX)`) that currently residualizes — re-validate against the new binary. Otherwise backlog: big (NLI/TEDS-GriTS), gated (conformal/κα), marginal-clean (MUC near-miss; per-relation P/R/F1; contested-prior down-weight). Reminder: card durable facts; grep `KNOWLEDGE_MAP.md` first; **no push until ~200 ahead or explicit ask.**
- paused_work: the FSMGen-temporal-integration (re-pin + migrate stable + richer lowering) — awaiting FSMGen's `min>1` ship.
- in_flight_uncommitted: none (submodule working tree reverted to the committed `43b29f5c`).
- blockers: `min>1` ISF lowering awaits FSMGen shipping the primitive (answer filed; FSMGen will ship on confirmation).
