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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.7` — completed by this commit; the adoption tree
  and every `.0`–`.7` activity are closed.
- Current state: all 604 Markdown paths classify once across 37 debt-free surfaces; exact history,
  bounded live views, collection/Knowledge Map routes, eight currency contracts, the compact README,
  current 36-part book, and repository-volume project data are enforced by six doctrines. Final
  cold-read defects in book status, route debt labels, and the active Claude hook are corrected and
  fail closed; two early content-pressure warnings remain named and below rollover.
- Next action: from the clean `.7` commit, run PNT selection against the bounded task catalog and
  current roadmap; do not reopen this closed tree unless a verified regression belongs here.
- In-flight uncommitted: none after this commit; no background job or disposable temp/log residue.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
