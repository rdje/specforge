# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
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
- Active unit: `CORPUS-COVERAGE.3` — docs/audit reconciliation of refresh progress versus normalized-cache
  retention, completed and awaiting commit.
- Current state: the documented `2026-07-05` cleanup—not artifact loss—explains why only the fresh SWD normalized
  bundle remains. All 32 `.2` refresh documents retain complete SourceIR→adapter chains (160/160 files); progress
  is 32 completed refreshes / 24 real chip-spec documents unrefreshed, separate from today's 80/1/79/78 stage
  and cache census. Task, live docs, Knowledge Map source, and book reflect the lifecycle distinction.
- Next action: stage and commit `.3`, then add and execute
  `CORPUS-COVERAGE.2.33` for USB 3.2 (the only SourceIR currently lacking EvidenceIR) under the RAM-guarded CPU
  Docling workflow.
- In-flight uncommitted: verified `.3` task/live/book/fact-card corrections awaiting commit; generated artifacts
  are untouched. The user-owned `.claude/settings.json` remains untouched.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
