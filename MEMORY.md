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
- Active unit: `CORPUS-COVERAGE.2.33d.iii`; root-ledger pressure is closed through its independently audited `.2`.
- Current state: the four ledgers have exact 29/24/12/8-record segments and warning-safe current roots at changes
  88 records / 1,259 lines / 189,197 bytes, development 63 / 1,306 / 176,145, status 60 / 102 / 83,464, and Rust
  55 / 1,064 / 89,706. A clean same-SSD clone of `10d182ff` reproduces every chain and a valid next append.
- Next action: close `.2.33d.iii` as measured-unnecessary if the already-landed direct convergence proof still
  covers every weak candidate path, then `.2.33d.iv` rebuilds and signs off the real USB cascade.
- In-flight uncommitted: none after the root-ledger `.2` closure commit; product behavior, shared inputs,
  thresholds, ceilings, capsules, and segment bytes are unchanged.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
