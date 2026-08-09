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
- Active unit: `ROOT-ROLLING-LEDGER-PRESSURE.2`; `.1` has materialized and verified the four exact rotations in
  its work-unit commit.
- Current state: roots are warning-safe at changes 87 records / 1,246 lines / 188,183 bytes, development
  62 / 1,294 / 175,215, status 59 / 101 / 82,836, and Rust 55 / 1,064 / 89,706. Exact 29/24/12/8-record
  segments, reciprocal manifests, and complete indexes pass the focused checker; old members are byte-identical.
- Next action: commit `.1`, then `.2` independently audits the clean committed continuity plane, future append
  capacity, reconstruction, routes, generic/focused alignment, and residue before closing this tree.
- In-flight uncommitted: none after the `.1` implementation commit; product behavior, shared inputs, thresholds,
  ceilings, capsules, and pre-existing segment bytes are unchanged.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
