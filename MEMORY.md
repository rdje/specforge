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
- Active unit: `SOURCE-IR-REPRODUCIBILITY.2` is the next child; `.0`/`.1`/`.5` are done and `.8`/`.9`/`.10` are
  new. Tracking-only: `SCRATCH-RESIDUE-CONTAINMENT.1`, `SPEC-TO-INTENT-ALIGNMENT.9`,
  `PROVIDER-MODEL-STORE-LOCALITY.1`, `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.5` withdrew `.1`'s three-paragraph loss finding. All three elements `.1` called absent are
  **retained** — every persisted token present in order, with 6 / 5 / 35 tokens *inserted* between them — so all
  31 dropped elements are re-segmentation and re-ingest content loss is **zero**. `.1`'s whole-string test
  cannot see a sentence the converter split or spliced into. The measurement that replaces it is larger and
  standing rather than drift: **8,648 of 18,870 converter text items (46%) reach no `SourceIR` record and earn
  no residual**, 5,896 of them because ingest never traverses inside a figure (`traverse_pictures=False`), and
  the persisted artifacts carry the same gap — the I2C specification reproduces byte-for-byte and still
  discards 1,372 figure-interior items including 39 captions. Two further defects are now owned: `source_ref`
  is ambiguous under batched ingest, and preservation has come apart from faithfulness (spliced figure text
  makes sentences the specification never wrote).
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.2` — make a reviewed source region resolve by content identity
  rather than ordinal position, changing how `source_record` *resolves* a region and never the recorded anchor,
  under the three-way digest lockstep already scoped in [[reviewed-fixture-projection-digest-lockstep]].
  `.8` (a carrier or residual for figure-interior text) and `.9` (batch-qualified `source_ref`) are the
  prerequisites for `.7`'s conservation gate and can be taken in either order.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none.
