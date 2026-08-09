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
- Active unit: `CORPUS-COVERAGE.2.42a`; administrative-workflow semantic-authority repair exposed by the first
  guarded OpenCAPI 3.0 Certified Definition cascade.
- Current state: 41 refreshes are complete / 15 remain, with 80 SourceIR / 11 normalized / 80 EvidenceIR / 79
  SemanticIR / 79 IntentIR / 79 adapters. The child projection finds 238 administrative/reference statements
  across 23 retained docs. Compound section/statement authority now excludes organizational process without a
  vendor/document/single-token rule. On unchanged 167-statement #42 evidence, actors fall 5→2, phases 5→0, gates
  6→0, invariants 20→8, behaviors 15→4, and constraints 20→8; real device/host compliance obligations survive.
  Two cascades reproduce six hashes. All 402 semantic tests, nine WIRE/I2C/SWD datasets, KG 156/156, and 66/66
  emitted ISFs through FSMGen strict pass; full CI passes 1,793 tests/five ignored plus mdBook and all doctrines.
- Next action: complete the final diff/staging/commit workflow for `.2.42a`, then resume parent `.2.42` with the
  second guarded ingest and final signoff.
- In-flight uncommitted: child code/docs/fact are implemented but not yet committed. Parent `.2.42` retains its
  authenticated six-file / 364142-byte rollback and first-pass evidence under repository-derived task storage.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
