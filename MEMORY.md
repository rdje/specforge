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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.9` is the next root child; `.8` is complete with `.8a`–`.8d` all done.
  Tracking-only: `SOURCE-IR-REPRODUCIBILITY` and `PROVIDER-MODEL-STORE-LOCALITY` (both new, `.1` runnable),
  `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`, `CLAIM-VERIFICATION-ADOPTION.1a`,
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.8d` replayed all 12 reviewed sources through all 48 isolated stages from clean production at
  `483e525d` and published the comparable result. Residual actionability is 8/16, source-region disposition
  10/14, required-modality accounting 8/12, provenance closure 45/45; conservation stays 120/120, IntentIR stays
  40/0/0, fabrication and unexplained drops stay zero, and `platform-system-ip` is the third supported category.
  A control leg re-projecting the same replayed artifacts with the frozen pre-change builder reproduces 4/16
  exactly, so the whole delta is the fixture projection's and `.8c` moved no reviewed metric. The frozen `.8a`
  contract gained a closed `state` on its selected family (28/28 RED). The replay surfaced one unrelated
  regression — reviewed cells anchor on ordinal SourceIR element ids and ingest is not reproducible across time,
  so exact source regions are 13/14 — now owned by `SOURCE-IR-REPRODUCIBILITY`.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: start `SPEC-TO-INTENT-ALIGNMENT.9` (measure and resolve the five omitted production-capability
  islands the controller ranks second), or take `SOURCE-IR-REPRODUCIBILITY.1` first if ingest currency should
  gate before breadth.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none.
