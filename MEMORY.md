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
- Active unit: `CORPUS-COVERAGE.2`; next corpus refresh selection (#47).
- Current state: refresh #46 is complete. Three guarded OpenCAPI Discovery ingests reproduce 40 pages / 54 visuals /
  49 tables / 50 sections / 172 elements with 47–53% memory free and 188 portable normalized paths. Current SourceIR
  181→172 and EvidenceIR 831→754 remove only flattened labels, synthetic table/front-matter enums, and false
  `BDF`/`DL`/`VPD` declarations. Final IntentIR retains ten behaviors / 55 constraints / six timings; adapter
  lowering blocks honestly on no grounded signals while validators expose the remaining capture frontier. Two
  repaired cascades reproduce eight downstream hashes; final committed-binary validation pins all 12 hashes.
  Corpus status is 46 done / ten remaining at 80 SourceIR / 18 normalized / 80 EvidenceIR / 79 downstream chains;
  all 61 emitted ISFs are FSMGen-strict clean.
- Next action: from a clean tracked tree, select the smallest/highest-value authenticated same-SSD candidate for
  refresh #47 and commit its owning task leaf before any generated mutation.
- In-flight uncommitted: the verified #46 parent documentation/fact bundle and its exact repository-local task
  evidence pending doctrine gates, exact cleanup, and recording commit. No background job.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
