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
- Active unit: `CORPUS-TASK-EVIDENCE-CONTAINMENT`; `.3a` completed the neutral checker prerequisites.
- Current state: `CORPUS-COVERAGE.2` refresh #48 is complete at 48 done / eight remaining, 80 SourceIR / 20
  normalized / 80 EvidenceIR / 79 downstream chains, and 59/59 current emitted ISFs FSMGen-strict clean. Its task
  source is pinned at commit `d78d842e`, blob `d7ac9aa2…`, SHA-256 `5d7acb0c…`, 2,308 lines / 277,636 bytes /
  4,746 maximum content-line bytes, with 892 bytes below the registered ceiling and no target mutation. Seven exact
  source regions / 48 formal routes select a bounded active root, seven semantic parts, and an exact capsule. The
  neutral checker now permits exact part lines only through the existing 6,400-byte task-evidence cap and binds
  formal container ids through source-backed `structural` routes; 39/39 focused cases pass.
- Next action: run `CORPUS-TASK-EVIDENCE-CONTAINMENT.3b` to commit the complete source-locked corpus contract,
  unconditional verifier invocation, 41 legacy plus seven structural routes, and absent-destination proof.
- In-flight uncommitted: none after the `.3a` checker commit. No background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
