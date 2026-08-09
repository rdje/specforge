# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
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
- Before committing run `scripts/check_doctrines.sh` (the registry/driver for memory,
  knowledge-map, task-acceptance, README policy, and live-document containment); hooks + CI
  run it too. Retrieval starts at bounded `KNOWLEDGE_MAP.md`, then searches its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SWD-SERIAL-EXTRACTION.7e.ii.b` — completed canonical promotion/signoff awaiting commit.
- Current state: the fresh portable tracked-PDF chain is exact 11/4/13/1 through IntentIR and 29/29 at 1.000;
  adapter/FSMGen, Evidence/Intent validation, 156/156 KG, seven WIRE evaluations, path portability, locality, and
  residue gates are green; full CI passes 1,775 tests / five ignored plus doctrines, rustdoc, and mdBook. Both
  rollback snapshots and exact temporaries are gone; only `.gitkeep` remains in project temp. The SWD parent and
  all `.7` containers are closed in the aligned roadmap/task/book/retrieval state.
- Next action: commit `.7e.ii.b` cleanly, then resume PNT by reading the first active tree in `docs/TASK_TREE.md`
  (`CORPUS-COVERAGE`) and its Knowledge Map pointers before selecting its next bounded frontier leaf.
- In-flight uncommitted: completed `.7e.ii.b` documentation/currentness closure awaiting commit; generated SWD
  canonical artifacts are repository-local and intentionally ignored. The user-owned `.claude/settings.json`
  remains untouched.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
