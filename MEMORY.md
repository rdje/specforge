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
- latest_commit: `2a9eeecc` — "LITERATURE-GROUNDING.4-.12 — ground 9 remaining aspects with verified citations (9-agent sweep + spot-verify)" (this `LITERATURE-GROUNDING.13` close commit pending → becomes 6 ahead of pushed `d92a73e3`; push at ~30). NOTE: `target/` was `cargo clean`ed earlier (12.7 GiB reclaimed); LITERATURE-GROUNDING was docs-only so no rebuild was needed — next CODE tree triggers a full recompile.
- active_work_unit: none — `LITERATURE-GROUNDING` **CLOSED `2026-06-02`** (`.1`–`.13` done: all 11 SpecForge aspects grounded with verified citations + `.13` synthesis map & 3-tier reach-full-potential backlog + book mirror in `architecture-rationale.md`; mdBook green).
- next_action: PNT — pick the next tree. Highest-leverage NEW candidates surfaced by the grounding backlog (each would be its own code-owning tree): Tier-1 = LTL/MTL property-template vocabulary for `temporal_rules` + `.isf`→PSL/SVA export; per-relation P/R/F1 + `temporal_rule` fact kind in the eval harness; conformal calibration of the LLM/VLM tier. Pre-existing gated candidates: column-less residual (~112; needs a precision/recall decision, reverses approach A), CHI Class-A timing matrices, CONSTRAINT-SUBJECT-PRECISION aggregate eval re-confirm on a clean APB re-ingest, AHB eval-seed expansion.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
