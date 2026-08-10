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
- Active unit: `CORPUS-COVERAGE.2.47`; Cortex-A76 Software Optimization Guide current-binary refresh.
- Current state: refresh #46 is committed and its exact task bundle is removed with zero residue. Read-only queue
  selection identifies the 46-page / 260-element / 637434-byte Cortex-A76 optimization guide as the smallest of
  ten remaining real candidates. Its portable same-SSD source hashes to `8358c5ae…3a22`; release
  `c4072c33…a1b05` and a seven-file / 2208588-byte stale chain are authenticated. The stale chain has 971 Evidence
  statements but zero relations, while mixed-vintage grouping emits 228 interfaces and a 537-output
  instruction-name `consumer.isf`; this is a baseline to investigate, not accepted topology.
- Next action: commit `.2.47` ownership, copy and byte-verify its exact stale chain under repository-local task
  storage, then run guarded CPU ingest and the deterministic SourceIR-through-adapter cascade.
- In-flight uncommitted: `.2.47` ownership/task-frontier documentation only. No generated artifact changed and no
  background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
