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
- Active unit: `CORPUS-COVERAGE.2.35`; refresh the USB4 Connection Manager Guide with the current binary.
- Current state: #34 is closed at 34 done / 22 remaining, with 80/3/80/79 stage census and 69/69 current emitted
  ISFs strict-clean. #35 is now owned on the bounded 96-page USB4 Connection Manager sibling. Its same-SSD source
  hash is `09f44419…d42`; the retained 46-visual / 23-table / 1,509-element chain ends in a stale renderable
  three-output (`SB`/`USB`/`USB4`), three-rule, one-enum `device_also.isf`, making it a direct generic authority
  and path-portability transfer check without presuming the current result.
- Next action: authenticate the complete retained #35 stage chain and exact same-volume rollback, then run the
  guarded CPU ingest and deterministic cascade from the resolved caller-authorized SSD input.
- In-flight uncommitted: only the `.2.35` ownership/resume-pointer commit; no generated artifact has changed.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
