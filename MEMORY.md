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
- latest_commit: `05d6b7d0` — "KNOWLEDGE-MAP-ADOPTION.2 — adopt the Knowledge Map …; close tree" (this `TEMPORAL-RULE-LTL-RENDER.1` design commit pending → becomes 19 ahead of pushed `d92a73e3`; **PUSH due-ish, threshold ~30**). NOTE: CI green baseline = 1219 tests. KM is live (`KNOWLEDGE_MAP.md`, 3 facts) — grep it before re-deriving; write a card for durable facts.
- active_work_unit: `TEMPORAL-RULE-LTL-RENDER` — `.1` design DONE (Tier-1 headline adopt from the LITERATURE-GROUNDING backlog: render `temporal_rules` in standard LTL/MTL `G(ante→cons)` notation; pure/derived/opt-in; no IR persistence, no .isf/FSMGen export). FIVE trees CLOSED `2026-06-02`/`-04` (LITERATURE-GROUNDING, TEMPORAL-RULE-EVAL, TEMPORAL-ANTECEDENT-RECALL, RECALL-CHAO-ESTIMATOR, KNOWLEDGE-MAP-ADOPTION).
- next_action: PNT — `TEMPORAL-RULE-LTL-RENDER.2`: new `crates/specforge/src/ir/temporal_ltl.rs` (`pub mod temporal_ltl;` in `ir/mod.rs`) with pure `temporal_rule_to_ltl(&TemporalRuleRecord)->String` + atom helper for all 7 `TemporalPredicateRecord` variants; form `G( <ante &-joined> -> X cons )` / `F[min,max]` w/ cycle_window / `G( cons )` if empty antecedents; unit tests (2 worked APB examples, empty-antecedent invariant, windowed). Add opt-in `--ltl` to `ValidateArgs` (`cli.rs`) → `validate` prints each temporal rule's clock-edge + LTL formula when set (default output UNCHANGED). Book subsection in `domain/temporal-semantics.md` (grounding: Pnueli/GoldMine/Texada + worked examples). Write a KM card (`docs/knowledge/temporal-rule-ltl-rendering.md`). Full CI; close. Then the `.isf`→PSL/SVA export is its OWN downstream tree (read FSMGen handoff first). Other backlog: conformal calibration; NLI/Dempster.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
