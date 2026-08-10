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
- Active unit: `CORPUS-COVERAGE.2.46`; current-binary refresh of OpenCAPI Discovery Configuration.
- Current state: the smallest of 11 remaining candidates is authenticated read-only at 40 pages / 181 retained
  elements / 403646 source bytes. Same-SSD source hashes `bc767d6e…fe103`; current release hashes
  `463a79e3…b06510`; the stale chain is seven files / 1951530 bytes. Its adapter currently fabricates width-one
  `BDF`/`DL`/`VPD` outputs from a chain with one actor relation, one port, four conditionals, one register, and six
  timing constraints; the refresh must preserve grounded intent while reapplying current shared authority.
- Next action: commit the `.2.46` owning leaf, copy and byte-verify its exact rollback on the repository volume,
  then run guarded CPU ingest and the deterministic EvidenceIR→SemanticIR→IntentIR→adapter cascade.
- In-flight uncommitted: `.2.46` task ownership, roadmap frontier, and this resume pointer await their recording
  commit; no generated artifact has changed and no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
