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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6b.ii.b`; `.0` through `.6b.ii.a` and
  `FSMGEN-REFRESH-INTEGRATE-6.1` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2` and
  `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: clean revision `bb152dfb` has 12/12-source and 48/48-stage replay evidence. IntentIR changes
  7/22/33→19/10/21 TP/FP/FN, provenance 3/29→17/29, and conservation 21/54→57/78. Arm Debug closes 12/12;
  AMD IOMMU and GIC-400 gain provenance only. Controller v4 selects `.6c` from ten fabricated / twelve
  unprovenanced survivors. The successful replay's 3,913-file / 1,090,884-KiB root and runtime map are absent.
- Next action: commit the fully verified `.6b.ii.b` slice, prove the tree handoff-clean, then activate `.6b.iii`
  and diagnose the generic bounded-ingestion selection seam from its existing tests and Knowledge Map fact.
- In-flight uncommitted: completed `.6b.ii.b` replay/result/controller evidence and aligned durable surfaces are
  ready for their task-scoped commit; no scratch or background replay remains.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
