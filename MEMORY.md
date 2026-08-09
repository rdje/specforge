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
- Active unit: `FACT-CARD-CATALOG-CONTAINMENT.1` — decide the bounded fact-card catalog topology; complete and
  verified, commit pending.
- Current state: ADR 0020 accepts the stable direct-ID landing plus deterministic 64-card title parts, derives the
  198-card maximum from the unchanged 200-file collection, and fixes root/part/aggregate limits plus two-stage
  migration. The old 32,634-byte monolith is unchanged and every destination remains absent.
- Next action: commit `.1`, then run `FACT-CARD-CATALOG-CONTAINMENT.2.1` to pin the `.1` monolith boundary and land
  the schema-closed `legacy_locked`/`migrated` checker before moving generated rows.
- In-flight uncommitted: ADR/task/fact plus impact-routed live documentation only; catalog code/output, product
  code/artifacts, PDF task source/destinations, shared inputs, thresholds, and ceilings unchanged.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
