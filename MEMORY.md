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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.7c` owns the complete reviewed replay and publication of canonical-
  recovery closure. `.7a` froze the design and `.7b` shipped production recovery plus exact retained-chain
  reconciliation.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: the `.7b` production sibling now preserves an inference antecedent's independently explicit
  asserted/deasserted state without widening the existing consequence extractor. It accepts only the frozen
  marker/state/declaration grammar, keeps source-local `PSEL` distinct from declared `PSELX`, and leaves unknown
  polarity symbolic; the complete 7-positive/13-refusal contract, exact full-build witness, 1,364-test core
  suite, warning-denied Clippy, and all five production-genericity components pass. All 24 retained chains were
  rebuilt through the adapter: every measurable stage is current with zero stale, and comparison found zero
  public JSON or emitted-ISF delta. The tracked 39/0/1 result remains the prior revision-bound publication until
  `.7c` independently replays all 12 reviewed sources and 48 stages.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: execute `.7c`: replay the complete reviewed population, attribute every canonical/provenance/
  conservation/residual/category/controller delta, and publish reproducible closure with exact cleanup evidence.
- In-flight uncommitted: `.7b` production, tests, retained-chain reconciliation, and synchronized live/book/
  retrieval records await the commit workflow; no background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
