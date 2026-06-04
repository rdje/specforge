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
- latest_commit: `86ece53c` — "DECISION-0005 — temporal behavior in LTL/MTL …" (this FSMGen-suggestion + SVA-deferral commit pending → becomes 27 ahead of pushed `d92a73e3`; **PUSH approaching ~30**). NOTE: CI green baseline = 1229 tests. KM live (now 9 facts after this commit).
- active_work_unit: none. Just handled a burst of user directives (2026-06-04): (a) **`FSMGEN-LTL-MTL-SUGGESTION` CLOSED** — filed a suggestion in `docs/FSMGEN_FEEDBACK.md` that ISF gain first-class LTL/MTL temporal properties (full `G(ante→X/F[min,max] cons)` template, generalizing the existing `(eventually s (within N))`) + a concrete proposed ISF shape (1:1 with `TemporalRuleRecord`) so SpecForge could lower `temporal_rules` directly into ISF; (b) **`TEMPORAL-RULE-SVA-RENDER` DEFERRED/logged** — the SpecForge-side `.isf`/IntentIR→SVA export designed (`.1`) but NOT built, pending the user's decision between it and the FSMGen-native route; (c) KM card `fsmgen-feedback-channel`; (d) ADR-0005 (LTL/MTL not CTL/TLA+). TEN trees touched/closed this run.
- next_action: PNT — "pick the next tree and roll to exhaustion." Do NOT build TEMPORAL-RULE-SVA-RENDER (deferred, user-decision-pending). Pick a DIFFERENT tree from the `adopt-defer-ledger.md` deferred-adopt items / backlog: candidates — MLIR-style `.isf` legalization/stage verifier (assert no un-lowered Semantic/Intent construct survives); MUC partial-match / near-miss eval diagnostic; per-relation gold scorer; conformal LLM-tier calibration (needs data); NLI verifier; Dempster combiner; prior decay. Reminder: card durable facts; grep `KNOWLEDGE_MAP.md` before re-deriving. **PUSH approaching (~27 ahead; push at ~30 or on ask).**
- paused_work: `TEMPORAL-RULE-SVA-RENDER` (deferred, awaiting user decision vs FSMGen-native LTL/MTL).
- in_flight_uncommitted: none.
- blockers: none.
