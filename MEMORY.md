# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
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
- Active unit: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.i` — complete and verified, pending its
  required per-leaf commit.
- Current state: `.0`–`.5g.i` are complete. The 544-line tracked validation snapshot is explicitly the
  last reviewed four-report projection; its exact root/live/producer/review contract and ten-case
  read-only oracle run unconditionally, and currency debt is closed. Full doctrines, Clippy, 1,724
  Rust tests, rustdoc, and mdBook are green; validation data and canonical artifacts are unchanged.
- Next action: commit `.5g.i`, clear/verify `git_message_brief.txt`, then activate `.5g.ii` from the clean
  tree and bind the tracked source-PDF registry to exact corpus membership and derived keys/paths.
- In-flight uncommitted: completed `.5g.i` files until the required commit; no background job.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
