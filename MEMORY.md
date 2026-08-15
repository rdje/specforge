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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.7a` owns the frozen contract for the current first-boundary canonical
  recovery. `.7` is active with `.7a`–`.7c` as design, implementation/currency, and replay/signoff slices.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: current replay authority reports one missing APB canonical fact at the SourceIR-to-EvidenceIR
  boundary. The compound sentence explicitly says `PSEL` is asserted and then infers that three other signals
  must be valid. The existing extractor correctly prevents `PSEL` from borrowing the consequence's `VALID`
  value, but no independent producer preserves the antecedent's own asserted state. The bounded root, index,
  manifest, containment contract, and new canonical-recovery part route `.7a` as the sole frontier; the completed
  behavioral part and source capsule are unchanged.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: execute `.7a`: freeze the exact APB witness, declaration-grounded inference-antecedent grammar,
  polarity/ambiguity controls, affected-chain inventory, and complete replay obligations before production code.
- In-flight uncommitted: `.7` activation task/live/book/retrieval changes await focused gates and commit; no
  production code or background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
