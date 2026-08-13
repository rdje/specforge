# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey. History lives in
> `git log`; work state lives in the task-trees (`docs/tasks/`); durable facts/decisions live in
> `docs/decisions/`. Do **not** append session narration — overwrite the "Current state" block.

## How to resume (any AI, any harness)
- Derive the current revision on read with `git rev-parse HEAD`; never store a latest-commit shadow.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (doctrines are mechanically
  gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`); follow `COMMIT.md`
  after every slice (unit id in the commit subject).
- Non-negotiable doctrine: `docs/decisions/0003-task-tree-and-commit-doctrine.md` (no code change without
  an owning task-tree first; signoff quality; zero ROADMAP↔code↔mdBook drift; push ~every 200 commits;
  artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh`; hooks + CI run it too. Retrieval starts at bounded
  `KNOWLEDGE_MAP.md`, then its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.0` is complete and awaiting commit from clean
  predecessor `112bc333`.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `.e.vii` committed at `112bc333`. Its next product leaf `.6d.ii.f` cannot safely append to the
  2,049-line / 278,178-byte active task file, only 350 bytes below its unchanged 278,528-byte ceiling. New
  containment `.0` pins that target byte-identically at SHA-256 e70892a5…a26c / Git blob 66ae9b6c…3632.
- Next action: commit containment `.0`, verify the clean handoff, then activate `.1.1` for the target-specific
  semantic-region, current-authority, route, reader, writer, and reconstruction census.
- In-flight uncommitted: new containment ownership/tree route, exact baseline, retrieval update, and resume
  pointer awaiting commit; the target task file remains untouched and no background job exists.
- Blockers: `.6d.ii.f` waits only for bounded task-evidence migration; product direction is unambiguous. The
  user-owned `.claude/settings.json` is untouched.
