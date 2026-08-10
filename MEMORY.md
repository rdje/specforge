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
- Active unit: `CORPUS-COVERAGE.2.47a`; timing-table structural-authority repair exposed by parent #47.
- Current state: parent #47 owns an exact seven-file / 2208588-byte rollback. Two guarded ingests reproduce 46
  pages / 71 visuals / 68 tables / 68 sections / 249 elements and one SourceIR hash with 52–64% sampled memory
  free. The first cascade correctly removes the stale 537-output instruction adapter, but exposes a distinct
  shared defect: 11 instruction-performance tables create 151 apparent timing constraints with no min/typ/max;
  operation names are duplicated into `unit`. Source classification flattens trapped data rows into header
  vocabulary, and substring `ns` matches `Instruction group`; EvidenceIR then emits without a typed value.
- Next action: commit `.2.47a` ownership, census the full retained corpus for true/false timing populations, then
  implement and replay a generic column-header/token/value-authority repair before parent #47 resumes.
- In-flight uncommitted: `.2.47a` ownership/task-frontier documentation plus ignored parent generated/rollback
  evidence. No background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
