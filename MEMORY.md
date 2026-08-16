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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.7c.i` owns exact source-local constraint grounding across the
  EvidenceIR-to-SemanticIR gate. `.7a` froze the design and `.7b` shipped production recovery plus exact
  retained-chain reconciliation; `.7c.ii` owns the clean replay and publication retry.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: the first complete 12-source/48-stage replay from `.7b` closes APB at EvidenceIR: the exact
  provenance-bearing `PSEL|must_be_asserted|<missing>` fact raises that stage to 40/0/0 with zero fabrication.
  SemanticIR then demotes only that record because source-local `PSEL` is intentionally absent from the global
  interface catalog, leaving SemanticIR and IntentIR at 39/0/1 and moving the sole unexplained loss to
  EvidenceIR-to-SemanticIR. Layer D remains correct for ordinary undeclared records; `.7c.i` must revalidate the
  recovered record's exact source, statement, subject, and closed appositive derivation without creating an
  interface, declaration, alias, or subject-wide exemption. The first replay is diagnostic, not publication.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: implement `.7c.i`'s record-scoped grounding revalidation, prove exact positive/refusal behavior,
  run selected core/genericity checks, and reconcile every proof-affected retained chain.
- In-flight uncommitted: none; the failed-publication replay and external-source map remain isolated under
  `.project-data/tmp` for repair comparison; production repair has not started and no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
