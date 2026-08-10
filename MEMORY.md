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
- Active unit: `CORPUS-COVERAGE.2.47`; final Cortex-A76 Software Optimization Guide refresh recording.
- Current state: the committed release reproduces 46 pages / 71 visuals / 68 tables / 68 sections / 249 elements,
  SourceIR `a396a074…d478`, normalized manifest `872e7dba…1bae`, and downstream `b4b50237…9ade` twice. Current
  authority removes the stale 537-output instruction topology; IntentIR retains 15 constraints / two assumptions,
  and adapter lowering blocks only on no signals. Five-stage validation, nine provider-free evals, KG 156/156,
  and 60/60 FSMGen strict pass; child full CI passed at the same code revision.
- Next action: finish the parent task/live-doc/mdBook/Knowledge Map record, run doctrine/path/locality/book gates,
  commit `.2.47`, clear `git_message_brief.txt`, remove the exact `.2.47`/`.2.47a` task bundles, then select and
  own refresh #48 (43-page OpenCAPI 4.0 32 Gbps PHY Signaling).
- In-flight uncommitted: parent `.2.47` task/live-doc/book/fact recording plus the two exact ignored task bundles.
  No background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
