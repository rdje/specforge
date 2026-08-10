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
- Active unit: `LIVE-DOC-STOP-RISK` — `.0` done; `.0a` and `.1` pending.
- Current state: `.0` removed the only finding with a deadline. `ROADMAP.md` was 364 of 384 ceiling lines, but
  213 of those lines were one `Current strategic priorities` bullet grown a sentence per closed leaf — the
  delivery chronology its own `bounded_snapshot` lifecycle forbids and the file itself disclaims. The real
  defect was that ADR 0010 gave the surface a one-time migration and no rollover, so the only exits were
  deleting direction into Git reachability or widening a ceiling. ADR 0030 makes the archive a **series**:
  copy the root to a dated capsule, append one contract record, add one index row, rewrite the root — all
  gate-proved. Capsules are checked against the *current root's* ceilings (so a capsule above them is a
  failure, not a rescue), and `rollover_policy` bounds the series at 16, warns at 12, and names its remedy,
  because `archive_terminal` surfaces are exempt from the milestone report. Root is now **156 lines /
  40.6% of ceiling**; no target, ceiling, or milestone moved. Contract self-test 9 → 31 cases.
- Next action: `LIVE-DOC-STOP-RISK.0a` — bound each current-root section so the accretion that filled the
  roadmap fails closed where it happens; `.0` gave the surface an exit but left the entrance open. Then `.1`
  (eight remedy-less aggregates + the near-ceiling reporting question), then `CORPUS-COVERAGE.2.51`.
  Note before appending: `CHANGES.md` is at 84.3% of its line health target, past warning, 90% rolls over.
- In-flight uncommitted: none; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
