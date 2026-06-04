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
- latest_commit: `c941d4a0` — "FSMGEN-MIN-WINDOW-CONFIRM …" (this `FSMGEN-ASSERT-LOWERING.1` commit — which RE-PINS `subs/fsmgen` `43b29f5c → 92d7036b` — pending → ~36 ahead of pushed `d92a73e3`). **PUSH THRESHOLD = 200**. CI green baseline = **1239 tests** (re-validated on `92d7036b`). KM 13 facts. **`subs/fsmgen` re-pinned to `92d7036b`** (FSMGen shipped both deltas: `6700fbb4` stable predicates + `92d7036b` `(within B MIN MAX)`).
- active_work_unit: `FSMGEN-ASSERT-LOWERING` — `.1` design + re-pin DONE. Lower obligations that residualize today into the ISF verification family. Design: new `TemporalRuleDisposition::AssertProperty { rule_id, prop }` (renders `(assert <prop>)`) + a shared predicate→ISF-boolean helper (`value HIGH→sig`/`LOW→(! sig)`/`V→(== sig V)`, `stable→(stable sig)`, `handshake→(& v r)`, conjunction `(& …)`) + **MANDATORY parity** (`classify_temporal_rule` oracle ≡ `classify_actor_contract` production — the `classify_actor_contract_is_parity_equivalent_to_classify_temporal_rule` test); declared-only else residual; empirically strict-validate every form. Stable forms confirmed valid on `92d7036b`.
- next_action: PNT — **`FSMGEN-ASSERT-LOWERING.2` (stable)**: `Obligation::Stable{signal}` → `(assert (stable s))`, + optional declared guard (from `guard_candidates`, like the `Drive` arm) → `(assert (=> g (stable s)))`. Add `AssertProperty` to the disposition enum + render arm (`isf_ir.rs`); add the Stable arm to BOTH `classify_actor_contract` AND `classify_temporal_rule` (parity); the no-silent-drop metric counts it as lowered; **update the existing stable-residual tests** (~`isf_ir.rs:2300` `w_stable_decl`, `2316` `nw_stable`, `2473` "RVALID stable" residual) to expect the new lowering; add a fsmgen-binary strict-check test on `(assert (stable …))`; book + KM. NOTE windowed-stable (`w_stable_decl` has a window) — decide: drop the window (stable has none) or keep residual. Then `.3` = general `=>` + min>1; close. Reminder: grep `KNOWLEDGE_MAP.md`; **no push until ~200 ahead or ask.**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none — both primitives shipped; re-pin validated.
