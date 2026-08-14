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
- Active unit: `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.4` is complete and verified from clean migration commit
  `10ee4a49`. Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `DECISION-RECORD-CAPACITY-HEADROOM.1`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: an independent no-hardlink same-volume clone of `10ee4a49` passed strict fsck, 21-region exact
  reconstruction, 58-owner/58-route navigation, and a clone-only future transaction. The proven five-file route
  release is installed: root 127/5,707, index 86/5,937, behavior 22/974, eight parts 2,162/282,891; capsule
  SHA-256 `e70892a5…a26c` is unchanged. Full CI and cleanup pass; `.6d.ii.f` is pending and eligible.
- Next action: activate `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f` through the bounded root-plus-behavioral-part
  transaction and execute population behavioral qualification.
- In-flight uncommitted: none after the `.4` closure commit; no background job.
- Blockers: none. Product direction is unambiguous. The user-owned `.claude/settings.json` is untouched.
