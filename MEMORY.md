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
- Active unit: `CORPUS-COVERAGE.2.39`; complete the verified OpenCAPI Ready-note signoff and commit workflow.
- Current state: two guarded ingests reproduce 10 pages / six visuals / five tables / 105 elements at 18% peak
  used. Evidence 113→112 removes only synthetic `Signal DL is width 1.` from a glossary row; current authority
  removes the stale one-interface/one-output surface. The guide preserves 12 behaviors/five constraints, blocks
  on no signals plus no behavior, and leaves only `adapter.json` plus its report. Two cascades reproduce all 12
  hashes; WIRE/I2C/SWD, KG 156/156, 66/66 FSMGen strict, persisted-path, and locality gates pass.
- Next action: stage the completed live-doc/mdBook/fact alignment, run final doctrines, commit `.2.39`, then
  select and own refresh #40 from the 17-document tail.
- In-flight uncommitted: generated #39 artifacts and tracked signoff alignment are complete; the exact ignored
  29-file / 804-KiB rollback/task evidence is deleted with zero task-id residue, and only the final commit workflow
  remains.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
