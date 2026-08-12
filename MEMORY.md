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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6b.ii.a` at signoff; `.0` through `.6b.i` and
  `FSMGEN-REFRESH-INTEGRATE-6.1` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: all 12 reviewed documents have hash-pinned current-binary replay evidence. Current IntentIR is
  7 TP / 22 FP / 33 FN with provenance closure 3/29; the largest bounded family is Arm Debug, where 12 correct
  register names lack reviewed access mode and become 12 FP + 12 FN + 12 provenance failures.
- Next action: commit the signed-off `.6b.ii.a` carrier and four-chain ADR 0025 reconciliation, then activate
  `.6b.ii.b` and replay the clean committed production revision over all 12 reviewed sources.
- In-flight uncommitted: completed, fully verified `.6b.ii.a`; frozen/current replay-result authorities remain
  unchanged until `.6b.ii.b`.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
