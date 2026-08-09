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
- Active unit: `CORPUS-COVERAGE.2.38a`; contain copied-artifact validation backannotation before resuming #38.
- Current state: #38 source `08a37c35…162`, release binary, typed baseline, and six original artifact hashes were
  authenticated before mutation. Running `validate` on copied rollback artifacts exposed that each IR persistence
  helper follows the embedded canonical artifact layout for the JSON write, even though the sidecar/reporting path
  follows the caller's copy. The canonical #38 SourceIR→IntentIR chain was backannotated and two normalized summary
  files materialized; the copied rollback was also updated, so it is no longer a byte-exact recovery set.
- Next action: commit `.2.38a` ownership, add a hermetic all-stage reproducer, persist validation JSON beside the
  explicit CLI artifact, then recover #38 from its authenticated same-SSD source and resume the guarded refresh.
- In-flight uncommitted: ignored canonical/rollback #38 artifacts preserve the validator side effect; no tracked
  product code has changed yet. Original pre-side-effect hashes/counts are recorded in the active task evidence.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
