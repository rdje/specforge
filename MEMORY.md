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
- Active unit: `CORPUS-COVERAGE.2.42`; guarded current-binary refresh of the 15-page OpenCAPI 3.0 Certified
  Definition from the caller-authorized same-SSD chipdoc source.
- Current state: 41 refreshes are complete / 15 remain, with 80 SourceIR / ten normalized / 80 EvidenceIR / 79
  SemanticIR / 79 IntentIR / 79 adapters and all 66 current emitted ISFs strict-clean. OpenCAPI AFU Address Space
  Usage reproduces 14 pages / nine visuals / two tables / 23 sections / 118 elements at 20% and 19% peak memory.
  Evidence stays at 106 statements with no typed hardware surfaces. Current authority removes five stale acronym-
  group interfaces/five actors/eight outputs; `.2.41a` removes false legal semantics. The final adapter blocks with
  no signals or behavior. Twelve hashes, WIRE/I2C/SWD, KG 156/156, full CI, 66/66 strict, book/doctrines/locality,
  exact task-evidence cleanup, and zero residue pass.
- Next action: authenticate the source, release binary, and exact six-file / 364142-byte stale chain; create the
  same-volume rollback under `.cache/task-work/CORPUS-COVERAGE.2.42`, then run the guarded CPU ingest.
- In-flight uncommitted: `.2.42` ownership only. Candidate selection found a 15-page tie; Certified Definition
  wins on 155 elements / 234448 source bytes versus Ready Definition's 171 / 249595. No generated artifact has
  changed yet.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
