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
- Active unit: `CORPUS-COVERAGE.2.41`; complete final signoff of the OpenCAPI AFU Address Space Usage refresh now
  that child `.2.41a` has repaired legal/administrative semantic authority.
- Current state: 40 refreshes are complete / 16 remain, with 80 SourceIR / nine normalized / 80 EvidenceIR / 79
  SemanticIR / 79 IntentIR / 79 adapters and all 66 current emitted ISFs strict-clean. Child `.2.41a` measured 32
  legal/administrative gates across 21 retained documents and added a compound word-bounded SemanticContext
  boundary. Real #41 preserves 106 EvidenceIR statements while phases/gates fall 1→0, invariants 17→15, Intent
  behaviors 2→0, and the adapter stays honestly blocked with no signals or behavior. Two repaired cascades
  reproduce six hashes. Nine WIRE/I2C/SWD gates, KG 156/156, full CI 1,790/five ignored, and FSMGen 66/66 pass.
- Next action: reverify #41's complete 12-artifact hash/path/count set, update the corpus row and live counts,
  delete only the authenticated rollback/task evidence with zero residue, then commit parent `.2.41` signoff.
- In-flight uncommitted: #41 retains its exact six-file rollback and SSD-local task evidence until parent final
  gates close. Two guarded ingests peak at 20% and 19% system memory used; refreshed SourceIR/EvidenceIR are
  content-exact to the first current cascade, and the repaired downstream chain is deterministic.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
