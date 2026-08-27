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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.8d` is the frontier under active `.8`; `.8a`, `.8b`, and `.8c` are done.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.8c` shipped the captured-region residual carrier. Every captured visual region no canonical
  `SemanticIR` record cites earns one typed `CapturedRegionResidualRecord` (region id, typed kind, EvidenceIR
  provenance, cause `no_canonical_carrier_for_captured_region`, boundary `evidence_to_semantic_ir`, replay), and
  `IntentIR` carries it unchanged. Coverage is a provenance membership test over concrete record collections and
  accepts both the evidence id and the `figure:<asset_id>` form the figure-contract producer emits; caption-mediated
  links are refused. The registry is 170 rules over 50 public fields per stage, the frozen `.8a` contract names
  the shipped carrier and its checker now resolves a named carrier to real production source (24/24 RED), and all
  24 retained chains are rebuilt to 24 current / zero stale at all four stages.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `.8d` — replay all 12 reviewed sources through all 48 isolated stages from clean production
  using the verified external source map, project the new captured-region records into the reviewed `/residuals`
  shape in `crates/specforge/test_data/source_to_intent_vertical/build_fixture.py`, attribute every canonical,
  provenance, conservation, residual, disposition, category, and controller delta, and republish the result,
  replay-evidence, and controller authorities byte-exact.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none.
