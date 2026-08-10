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
- Active unit: `CORPUS-COVERAGE.2.43a.ii`; typed transaction-phase qualifier precision follows completed #43.
- Current state: 43 refreshes are complete / 13 remain, with 80 SourceIR / 12 normalized / 80 EvidenceIR / 79
  SemanticIR / 79 IntentIR / 79 adapters. Ready Definition #43 is complete: one guarded CPU ingest produced 15
  pages / six visuals / three tables / 30 sections / 169 elements at 62% peak sampled memory. Source 171→169
  removed two flattened diagram labels; Evidence 176→173 additionally removed synthetic `Signal DL is width 1.`.
  Release `5a43f1b5…11f` now yields three actors, zero interfaces/phases/gates, 21 invariants, 19 contracts, 19
  Intent behaviors, and 25 constraints. Two final cascades reproduce all hashes. The adapter blocks only on no
  signals and emits nothing. Nine WIRE/I2C/SWD datasets, KG 156/156, 66/66 FSMGen strict, mdBook, six doctrines,
  2,008-artifact paths, locality, exact nine-file cleanup, and zero residue pass.
- Next action: classify all 82 retained `transaction_phases[]` records, pin the Wishbone/CAN false-positive paths
  and APB/AHB/SWD controls, then tighten only through universal phrase grammar under `.2.43a.ii`.
- In-flight uncommitted: none after the parent signoff commit. No background job.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
