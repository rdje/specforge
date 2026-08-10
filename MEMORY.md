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
- Active unit: `CORPUS-COVERAGE.2.47a`; timing-table structural-authority repair exposed by parent #47.
- Current state: `.2.47a` is implemented and verified. The retained 39-document / 284-table timing surface moves
  2,144→608 records; Cortex moves 151→0, I2S holds at five, and every survivor is value-bearing. Eight complete
  cascades reproduce `58411566…6457a` twice; KG 156/156, full CI, and 60/60 FSMGen strict pass.
- Next action: record the `.2.47a` commit, clear `git_message_brief.txt`, then resume parent `.2.47` from the
  committed repair for release build, final Cortex replay/signoff, row 47, and parent rollback cleanup.
- In-flight uncommitted: verified `.2.47a` code, tests, task/book/Knowledge Map/live-doc recording, plus ignored
  parent generated/rollback evidence. No background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
