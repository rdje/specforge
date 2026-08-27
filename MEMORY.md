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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.8c` is the frontier under active `.8`. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`, `CLAIM-VERIFICATION-ADOPTION.1a`, and
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.8a` froze the required-residual rule as an executable contract and `.8b` made the evaluator
  agree with it. The published ratio is now 4/16 with an affected population of twelve; the frozen first result
  re-derives as 0/82; no per-cell score or other global dimension moved. A missing canonical key still adds a
  required observation and is met only by an exact, provenanced, actionable residual for that key, and a
  residual duplicating a promoted key credits nothing. The published current result was re-summarized in place
  and the replay evidence records both digests. Production still owns only the non-applicable timing carrier.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: execute `.8c` — emit the typed, source-linked, actionable residual for captured
  `static_component_topology` figure regions at SemanticIR and IntentIR under closed structural grammar, then
  rebuild every proof-affected retained chain to zero stale with zero public field delta.
- In-flight uncommitted: none after the `.8b` commit; no background job is running.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
