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
- Active unit: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.1` — enforce the source-locked active task contract; complete and
  verified, commit pending.
- Current state: the neutral contract/checker binds commit/blob/SHA/metrics/index identity, validates 15 contiguous
  source regions and seven warning-safe planned parts under fixed portable caps, and rejects either destination
  directory or any declared output. Twenty-nine focused cases and the real source-locked report pass.
- Next action: commit `.2.1`, then run `ACTIVE-TASK-EVIDENCE-CONTAINMENT.2.2` to pin the `.2.1` commit boundary and
  add exact region digests plus all boundary-history primary leaf routes while destinations remain absent.
- In-flight uncommitted: checker/contract/doctrine wiring and impact-routed live docs only; product code, PDF task
  source, migration destinations, generated artifacts, shared inputs, thresholds, and ceilings unchanged.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
