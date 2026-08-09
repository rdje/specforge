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
- Active unit: `CORPUS-COVERAGE.2`; refresh #34 is closed and #35 needs selection/ownership.
- Current state: USB4 Inter-Domain now has a complete portable 198-file source-through-adapter chain. Two guarded
  ingests reproduce all final hashes at 18% peak system memory used; 51/51 page paths are repository-relative,
  and the only difference from the preserved defective bundle is `rendered_image.path`. Current generic authority
  removes the stale one-signal/two-rule/eight-enum model: the adapter blocks at zero interfaces/signals/rules,
  retains six storage records, emits no target, and leaves exactly `adapter.json`. Corpus refresh is 34 done /
  22 remaining; 80/3/80/79 stage census and 69/69 strict-clean current emitted ISFs are verified.
- Next action: select refresh #35 from the remaining 22 real chip-spec documents, add its owning task-tree leaf,
  authenticate its retained chain/source, then run only that guarded slice.
- In-flight uncommitted: none after the `.2.34b.ii.b` commit. Its exact rollback and defective-chain evidence
  were deleted only after final gates; exact task-id residue is zero.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
