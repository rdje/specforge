# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (the 4th
  architecture — doctrines are mechanically gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh` (the registry/driver: memory-arch +
  knowledge-map + task-acceptance); hooks + CI run it too. Retrieval: `KNOWLEDGE_MAP.md`.

## Current state (OVERWRITE this block each update — do not append)
- Git baseline: `70534fe0` on `main`, equal to `origin/main`; last substantive slice is
  `1abfb49c` (`PDF-VARIANT-DIGESTION.10i`). The user-owned untracked
  `.claude/settings.json` remains untouched.
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.0` — ownership and local
  pressure/locality measurement. Full roadmap/code/mdBook ramp-up and the FSMGEN
  README policy, adoption guide, and neutral containment doctrine review are complete.
- Current result: adoption is GO; no deletion or migration has occurred. The new task
  tree owns pointer stabilization, README policy, common containment enforcement,
  lossless migrations, mdBook drift repair, and repository-volume data locality.
- Next action after `.0` commits: execute `.1` (remove ceremonial MEMORY/live-doc
  coupling while preserving the bounded resume contract), then `.2` (README policy,
  trim, locally derived caps, unconditional guard).
- In flight: this `.0` docs-only commit. Blockers: none. Push remains deferred to the
  configured ~200-commit batch threshold.
