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
- Active unit: `CORPUS-COVERAGE.2.33a` — USB 3.2 cascade-blocker probe completed, awaiting commit.
- Current state: current release `9cd700…` CPU-ingested the authorized external USB 3.2 PDF at 81–82% free RAM,
  producing 548 pages / 507 visual assets / 283 tables / 5,830 elements / zero residuals. Evidence then panicked
  before writing output: both signal-declaration collectors slice `lowered[idx - 2..idx]`, and the match byte
  offset places `idx - 2` inside a three-byte `U+F0B7` bullet. The failed symlink-relative launch was separately
  an expected portability refusal and made no change.
- Next action: commit `.2.33a`, then implement `.2.33b`'s shared UTF-8-safe sentence-boundary helper and tests;
  `.2.33c` will rebuild and resume evidence from the promoted SourceIR without another Docling ingest.
- In-flight uncommitted: fresh ignored USB normalized/SourceIR plus the 4.9 MiB byte-verified rollback under
  `.cache/task-workspaces/CORPUS-COVERAGE.2.33`; no EvidenceIR exists. User settings remain untouched.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
