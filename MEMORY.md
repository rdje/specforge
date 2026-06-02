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
- latest_commit: `aa7c3f6e` — "LITERATURE-GROUNDING.3 — ground aspect 4 (protocol & temporal semantics) with verified citations" (this `LITERATURE-GROUNDING.4-.12` commit pending → becomes 5 ahead of pushed `d92a73e3`; push at ~30). NOTE: `target/` was `cargo clean`ed earlier (12.7 GiB reclaimed); this lane is docs-only so no rebuild was needed.
- active_work_unit: `LITERATURE-GROUNDING` — `.1`–`.12` DONE (all 11 SpecForge aspects grounded in verified published research; `.4`–`.12` = a 9-agent sweep, verified-citations-only, 1 unverifiable survey dropped). Only `.13` synthesis + reach-full-potential backlog remains → then CLOSE the tree.
- next_action: PNT — `LITERATURE-GROUNDING.13`: synthesize the unifying SpecForge↔literature map + a prioritized reach-full-potential backlog (adopt-now / claimed-novelty / research-suggested improvements → future owned trees); **`.13` is the CLOSING leaf → BOOK-METHOD-DOC close-rule applies** (add/refresh the tree's mdBook subsection per `docs/tasks/BOOK-METHOD-DOC.md` placement map); then CLOSE. After that, pick the next tree. Gated candidates: column-less residual (~112; needs a precision/recall decision, reverses approach A), CHI Class-A timing matrices.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none for `.13` (docs-only synthesis).
