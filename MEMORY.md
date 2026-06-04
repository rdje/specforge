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
- active_work_unit: `FSMGEN-ASSERT-LOWERING` — `.1` (design + re-pin `92d7036b`) DONE; `.2` (stable) **DONE as verified-NEGATIVE — DO NOT implement the stable lowering**. Investigation caught a fidelity trap: every mined stability obligation is phase-scoped (`Obligation::Stable { during: Window::Between{tick_phases} }`, `contract.rs` ~L432 = "stable DURING `[from_phase,to_phase]`"); FSMGen's `(stable s)` is UNCONDITIONAL per-tick `$stable(s)` → `(assert (stable s))` would over-assert; the faithful `(=> g (stable s))` needs a boolean `g` for the phase interval but tick-phases aren't `.isf` signals → no guard → the existing residual is CORRECT. KM card `stable-obligation-phase-scoped-residual`. No code change.
- next_action: PNT — **`FSMGEN-ASSERT-LOWERING.3`** = the genuinely-faithful enabled lowering: general **antecedent→consequent** where the antecedent is a representable boolean → `(assert (=> A B))` / `(=> A (next B))` / `(=> A (within B MIN MAX))` (min>1 via the shipped window-range). New `TemporalRuleDisposition::AssertProperty { rule_id, prop }` (renders `(assert <prop>)`; add `asserts: Vec<String>` to `IsfTransaction` — ~11 ctor sites, OR generalize) + a predicate→ISF-boolean helper (`value HIGH→sig`/`LOW→(! sig)`/`V→(== sig V)`, `handshake→(& v r)`, conjunction `(& …)`) + **MANDATORY parity** (`classify_temporal_rule` ≡ `classify_actor_contract`); declared-only else residual; empirically strict-validate. CAUTION: check fidelity FIRST (like `.2`) — only lower shapes that faithfully map (e.g. don't drop a min>1 lower bound, don't over-assert). This is a big careful slice in the highest-care `.isf` adapter — do NOT rush. Reminder: grep `KNOWLEDGE_MAP.md`; **no push until ~200 ahead or ask.**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none — both primitives shipped; re-pin validated. (Stable lowering is correctly blocked-by-fidelity, not by FSMGen.)
