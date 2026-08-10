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
- Active unit: `CORPUS-COVERAGE.2.49`; closing the Generic Interrupt Controller Overview Guide refresh.
- Current state: refresh #49 is complete at 49 done / seven remaining, 80 SourceIR / 21 normalized / 80 EvidenceIR /
  79 downstream chains, and 58/58 current emitted ISFs FSMGen-strict clean. Two guarded CPU ingests reproduce the
  430-element SourceIR, 139-file / 30,731,394-byte normalized bundle, and all ten artifact/report hashes. Current
  authority retires three false timings, 83 heuristic interfaces, 26 generic phases, 54 prose gates, and the stale
  `controller.isf`; the methodology guide retains grounded invariants, contracts, behaviors, and constraints.
- Next action: commit the `.2.49` completion/alignment checkpoint, remove its authenticated repository-local
  rollback evidence with a zero-residue census, then create and own `CORPUS-COVERAGE.2.50`.
- In-flight uncommitted: `.2.49` completion/task/status/book/fact alignment only; authenticated ignored rollback
  evidence remains until the durable completion commit; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
