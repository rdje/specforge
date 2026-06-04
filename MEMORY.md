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
- latest_commit: `50ec944c` — "FSMGEN-ASSERT-LOWERING.2 …" (this `.3` close commit pending → ~38 ahead of pushed `d92a73e3`). **PUSH THRESHOLD = 200**. CI green baseline = **1243 tests** (+4 from `.3`). KM 14 facts. **`subs/fsmgen` pinned `92d7036b`** (both deltas shipped: stable predicates + `(within B MIN MAX)`).
- active_work_unit: none — `FSMGEN-ASSERT-LOWERING` **CLOSED `2026-06-04`** (`.1`–`.3`): `.1` re-pin `92d7036b`; `.2` stable = verified-negative (phase-scoped ≠ unconditional `(stable s)`; residual correct; KM `stable-obligation-phase-scoped-residual`); `.3` faithful fix shipped — a **guarded** windowed-eventual now keeps its antecedent: `(assert (=> g (within s [min] max)))` instead of the antecedent-dropping monitor (`min>1` → two-operand `(within s MIN MAX)`; guarded `min=0` → residual). `IsfContract`/`Contract` carry a pre-built `prop`; `windowed_eventual_prop` + `actor_guard_condition`; BOTH classify paths updated (parity oracle green); 4 tests incl. fsmgen-binary strict-check; CI green 1243.
- next_action: PNT — FSMGen temporal loop is fully integrated (eventually-migrate + stable-fidelity-finding + guarded-eventual lowering, both deltas consumed). Remaining backlog (NOT yet done): **unguarded `min>1`** eventuals still use the anchored monitor (no `##[min:max]` without an antecedent) — a known, noted limitation, low priority; general non-windowed antecedent→consequent value rules already lower via the `Rule` (`(rule (guard)(sig val))`) path. Otherwise the **big/gated/marginal** backlog: NLI verifier [needs model]; TEDS/GriTS [needs table gold]; conformal [held-out set]; κ/α [2nd annotator]; MUC near-miss eval diagnostic; per-relation P/R/F1; contested-prior consultation down-weight. Reminder: grep `KNOWLEDGE_MAP.md`; **no push until ~200 ahead or ask.**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: remaining backlog is gated (data/annotator) or marginal; no FSMGen blockers.
