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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.8` is the next eligible product leaf after canonical recovery closed.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.7c.ii` independently replayed all 12 reviewed sources and 48 isolated stages at production
  revision `a4a08cd4`. EvidenceIR, SemanticIR, and IntentIR each publish 40/0/0 TP/FP/FN, provenance is 43/43,
  conservation is 120/120, and fabrication plus unexplained drops are zero. Source/evidence capture are 14/14
  and 12/14; source disposition is 8/14, modality accounting 6/12, and residual actionability 4/24. Wire-protocol
  and physical-link are supported. Strict replay/result/controller artifacts are byte-current, zero hard gates
  fail, trajectory remains honestly unmeasurable for insufficient history, and the controller ranks `.8`.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: activate `.8` from its existing task-tree contract, reproduce the highest-value required
  promotion-loss residual family, and decompose the bounded typed/source-linked/actionable repair before code.
- In-flight uncommitted: none after the `.7c.ii` closure commit; both replay roots and the external-source map are
  absent, and no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
