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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.7c.ii` owns the clean complete replay, comparable publication, and
  signoff after `.7c.i` implemented exact source-local constraint carry across the EvidenceIR-to-SemanticIR gate.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.7c.i` shares the closed inference-antecedent parser with a one-pass document-local grounding
  index and admits an otherwise undeclared constraint only when its complete semantic and provenance identity
  matches a uniquely supported same-clause appositive derivation after the existing polarity refinement. The exact
  APB-shaped `PSEL` record reaches SemanticIR and IntentIR; support- and source-altered impostors remain residual,
  no interface/catalog/alias authority is created, and distinct declared `PSELX` remains unchanged. Seven focused
  tests, 1,365 core passes / five ignores / zero failures, warning-denied Clippy, and all five genericity components
  pass. All 24 retained chains are current; 96/192 artifacts changed only in proof surfaces, with zero public or
  validation delta, and the exact rollback is absent. Tracked publication remains revision-pinned at 39/0/1.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: execute `.7c.ii` from clean production across all 12 reviewed sources and 48 isolated stages,
  compare it to the diagnostic replay, publish only a complete conserved result, prove reproducibility, and clean
  exact project-local scratch.
- In-flight uncommitted: none after the `.7c.i` commit; its diagnostic replay and external-source map remain
  isolated under `.project-data/tmp` for `.7c.ii` comparison, and no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
