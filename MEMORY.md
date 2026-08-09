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
- Active unit: `CORPUS-COVERAGE.2.33d.iv.b`; `.iv.a` closes generated adapter output convergence.
- Current state: the repaired USB cascade removes all four weak names, 45 relations, and the phantom actor;
  `adapt` now removes obsolete `.isf` siblings. Signoff remains blocked because authority-empty SemanticIR emits
  918 low-confidence interfaces / 556 one-bit outputs from source-fact token groups despite zero actor graph.
- Next action: corpus-measure the authority-empty fallback and select the smallest structural SemanticIR repair;
  `.iv.c` owns the final preserved-source cascade, FSMGen/WIRE/KG/full gates, and rollback deletion.
- In-flight uncommitted: none after the `.iv.a` commit. Ignored rebuilt USB stages and the exact verified rollback
  at `.project-data/tmp/corpus-coverage-2-33d-iv-before/` remain intentional; do not delete before `.iv.c`.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
