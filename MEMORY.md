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
- Active unit: `CORPUS-COVERAGE.2.40` complete; `.2.41` is the next unowned refresh slice.
- Current state: 40 refreshes are complete / 16 remain, with 80 SourceIR / 9 normalized / 80 EvidenceIR / 79
  SemanticIR / 79 IntentIR / 79 adapters and all 66 current emitted ISFs strict-clean. Two guarded 13-page
  ingests and complete cascades reproduce all 12 hashes at 21% and 22% peak memory used. Evidence 173→172 removes
  only stale synthetic `Signal DL is width 1.` from the terms glossary; the one-interface/one-output surface
  disappears, lowering blocks honestly on no signals plus no behavior, and only `adapter.json` plus its report
  remain. All 13 image and 13 layout paths are repository-relative.
- Next action: select the next smallest roadmap-aligned real chip-spec document, authenticate its same-SSD source
  and retained chain, then own `.2.41` before changing generated artifacts.
- In-flight uncommitted: none after the `.2.40` signoff commit. The exact 29-file / 932010-byte / 968-KiB task
  bundle is deleted with zero task-id residue.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
