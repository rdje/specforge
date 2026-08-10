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
- Active unit: `CORPUS-COVERAGE.2.49`; Generic Interrupt Controller Overview Guide refresh.
- Current state: `CORPUS-COVERAGE.2` refresh #48 is complete at 48 done / eight remaining, 80 SourceIR / 20
  normalized / 80 EvidenceIR / 79 downstream chains, and 59/59 current emitted ISFs FSMGen-strict clean. Its task
  stable root is now 65 lines / 2,819 bytes over a 75-line index, seven semantic parts, and an exact 2,308-line /
  277,636-byte / SHA-256 `5d7acb0c…` capsule. The migrated contract authenticates all seven regions / 41 legacy +
  seven structural routes. Containment is closed. `.2.49` selects the smallest remaining document: the 45-page /
  430-element GIC Overview Guide. Its same-SSD PDF SHA and exact 1,400,092-byte stale seven-file chain are pinned;
  normalized is absent and no generated artifact has changed.
- Next action: commit the `.2.49` ownership boundary, create authenticated repository-local rollback evidence,
  then run the guarded CPU ingest and complete deterministic downstream validation.
- In-flight uncommitted: `.2.49` ownership/root/index/manifest/contract/book alignment only; no background job.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
