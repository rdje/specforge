# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Derive the current repository revision on read with `git rev-parse HEAD`; never store a
  latest-commit shadow that the recording commit would immediately invalidate.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (the 4th
  architecture — doctrines are mechanically gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh` (the registry/driver for memory,
  knowledge-map, task-acceptance, README policy, and live-document containment); hooks + CI
  run it too. Retrieval starts at bounded `KNOWLEDGE_MAP.md`, then searches its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.8g` — independent resulting-tree closure audit; complete
  in this commit, closing `.8`.
- Current state: all 14 primary plus three secondary members and 17 markers agree with Cargo/Git authorities; no
  live FSMGen value remains in executable source. Historical values are evidence and Rust script values are
  self-contained fixtures. All 83 focused cases and full CI pass with no canonical/product/ceiling change.
- Next action: from the clean `.8g` commit, activate `.9a`; measure shared archive-index readers and design a
  bounded landing/partition topology before another status segment crosses the 218-byte rollover boundary.
  `.9b` then implements and independently verifies it before returning to the corpus frontier.
- In-flight uncommitted: none after the `.8g` commit. `.project-data/tmp` contains only `.gitkeep` plus `xcrun_db`.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
