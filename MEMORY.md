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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5h` — complete and ready to commit.
- Current state: unique extraction/IntentIR contracts now live in two directly indexed book parts;
  four stable root references are bounded compatibility pointers, their obsolete generated catalog is
  retired, and composed currentness binds root routes plus product claims to code. The resulting book
  is exactly 35 files / 12,490 lines / 748,629 bytes; full CI passes 1,727 tests with 5 ignored and
  product behavior is unchanged.
- Next action: commit `.5h`, clear `git_message_brief.txt`, verify the clean handoff, then activate
  `.5i` and close the collection/projection program including the corpus-KB warning disposition.
- In-flight uncommitted: completed, verified `.5h` resulting tree awaiting its atomic commit; no
  background job and no generated/temp residue.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
