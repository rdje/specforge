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
- Active unit: `CORPUS-COVERAGE.2`; `.2.43a.ii` typed transaction-phase qualifier precision is complete.
- Current state: the 82 retained typed phase records classify as 59 valid / 23 false; a complete old-rule replay
  is 101 candidates and repaired release `463a79e3…510` emits 70 valid records across 27 documents. Positive phrase
  authority removes all 31 false candidates with zero retained-provenance change, zero non-empty rejected signal
  sets, and byte-identical populated/repaired IntentIR variants. Focused + 407 SemanticIR tests, warning-deny
  Clippy, nine WIRE/I2C/SWD datasets, KG 156/156, full CI 1,791/five ignored, 66/66 FSMGen strict, mdBook,
  doctrines, persisted paths, and locality pass. Corpus remains 43 refreshes complete / 13 remaining with
  80/12/80/79 stage counts.
- Next action: select and authenticate the smallest roadmap-aligned refresh #44 candidate, create its owned leaf,
  then run the guarded repository-local ingest and deterministic cascade.
- In-flight uncommitted: none. No background job.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
