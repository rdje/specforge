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
- Active unit: `CORPUS-COVERAGE.2.43a.i`; complete and awaiting its required recording commit before any pivot.
- Current state: 42 refreshes are complete / 14 remain, with 80 SourceIR / 11 normalized / 80 EvidenceIR / 79
  SemanticIR / 79 IntentIR / 79 adapters. Parent #43 retains its fresh 15-page / 173-statement no-hardware cascade.
  `.2.43a.i` proves 3,180 title-authorized legacy phases create 2,238 synthetic behaviors, preserve 48
  pure-inferred actors, and add 2,257 responsibilities with zero adapter executable value. The producer now
  emits `phases: []`; old populated records load/round-trip but Intent ignores them. Repaired release
  `834e335a…d3d3` makes all 79 populated/empty variants byte-identical at IntentIR and identical at the adapter.
  Its full gate is green: 404 SemanticIR tests, 50 IntentIR tests, warning-deny Clippy, nine WIRE/I2C/SWD gates,
  KG 156/156, full CI 1,789/five ignored, 66/66 FSMGen strict, mdBook, six doctrines, paths, and locality.
- Next action: commit completed `.2.43a.i`, then rebuild #43's downstream cascade with repaired release
  `834e335a…d3d3`, reproduce it, classify the final artifact, and close parent `.2.43`. Pending `.2.43a.ii` owns
  the separate Wishbone/CAN typed-qualifier precision audit.
- In-flight uncommitted: completed `.2.43a.i` code, measurement, book, live-doc, task, and Knowledge Map changes
  await their recording commit. Parent #43's same-volume rollback/fresh cascade remains owned. No background job.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
