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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iii` is complete pending its commit.
  `.6d.ii.e.ii`, `.6d.ii.e.i`, `.6d.ii.d.iv`, `.6d.ii.d.iii`, `.6d.ii.d.ii`, `.6d.ii.d.i`,
  `.6d.ii.c`, `FSMGEN-REFRESH-INTEGRATE-8.1`, `.6d.ii.b`, `.6d.ii.a`, and
  `.6d.i` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2`
  and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: sealed opaque document/symbol identities, purpose-specific capabilities,
  deterministic rule registration, typed current-evidence capture, a versioned proof
  ledger, and kernel-only `VerifiedProofLedger` authority are implemented. Inventory is
  77 modules / 38 families / 168 fields; 16 unit and five compile-fail controls pass.
  Current producers and persisted artifacts remain proofless until `.e.iv` migration.
- Next action: stage the exact `.e.iii` change, run doctrines and full CI, commit with the
  declared work-unit subject, clear the brief, and verify a clean tree. Only then activate
  `.e.iv` to register and migrate production inference families.
- In-flight uncommitted: `.e.iii` implementation, adversarial controls, dependency lock,
  inventory, task closure, ADR/audit/book/live-doc/retrieval alignment, and no generated
  canonical artifact. No background job is running.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
