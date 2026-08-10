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
- Active unit: `CORPUS-COVERAGE.2`; refresh #48 is complete, but its task file has reached the size boundary.
- Current state: OpenCAPI 4.0 32 Gbps PHY Signaling now reproduces 43 pages / 39 visuals / 27 tables / 73 sections /
  318 elements, 130 normalized files, 547 statements, 60 timings, no declared signal graph, and no emitted target.
  SourceIR, normalized bundle, and eight downstream hashes reproduce. Corpus is 48 done / eight remaining at
  80 SourceIR / 20 normalized / 80 EvidenceIR / 79 downstream chains; all 59 current ISFs are FSMGen-strict clean.
- Next action: after the clean `.2.48` commit, open the smallest owning containment leaf and partition the
  277636/278528-byte corpus task tree; then select and own refresh #49.
- In-flight uncommitted: `.2.48` live-doc/book/task recording and its exact ignored rollback pending commit-time
  authentication and deletion. No background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
