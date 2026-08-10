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
- Active unit: `LIVE-DOC-STOP-RISK` — `.0` and `.0a` done; `.1` pending.
- Current state: the roadmap surface is fully repaired. `ROADMAP.md` was 364 of 384 ceiling lines, but 213 of
  those were one `Current strategic priorities` bullet grown a sentence per closed leaf — chronology its own
  `bounded_snapshot` lifecycle forbids. `.0` (ADR 0030) supplied the missing **exit**: the archive is now a
  bounded capsule series, so a rollover is copy-root-to-dated-capsule, append one contract record, add one
  index row, rewrite the root, all gate-proved; capsules are held to the current root's ceilings, and
  `rollover_policy` (16 max / 12 warning / named remedy) carries the signal that `archive_terminal` surfaces
  are exempt from. `.0a` (ADR 0031) closed the **entrance**: each H2 declares its own line bound and remedy,
  the declared set must equal the required H2 order, and the bounds' legal sum plus scaffold must fit the
  health target (243 of 256). Root is **156 lines / 40.6% of ceiling**, every section below its 80% warning.
  Replayed accretion fails at 180 total lines where the file bound stayed silent to 384. Self-test 9 → 49.
- Next action: `LIVE-DOC-STOP-RISK.1` — decide each of the eight remedy-less aggregate surfaces in Finding 1
  (name the remedy its aggregate triggers, or re-derive it as `files × per-file` under ADR 0029) and settle
  whether near-ceiling pressure needs its own signal. `CORPUS-COVERAGE.2.51` is the next product slice after.
  Note before appending: `CHANGES.md` is at 86.1% of its line health target, past warning, 90% rolls over.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
