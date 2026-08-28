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
- Active unit: `SOURCE-IR-REPRODUCIBILITY.12` is the next child; `.0`/`.1`/`.5`/`.11` are done and `.2`–`.4`,
  `.6`–`.10` remain. Tracking-only: `SCRATCH-RESIDUE-CONTAINMENT.1`, `SPEC-TO-INTENT-ALIGNMENT.9`,
  `PROVIDER-MODEL-STORE-LOCALITY.1`, `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.11` closed the one inference the tree's largest number rested on. `.5` attributes 5,896
  dropped items to `iterate_items(traverse_pictures=False)` by reimplementing two docling-core predicates read
  from that library's source; `--oracle` now asks the library itself on **all 24** retained converter documents
  — 43,614 text items, **22,127 yielded, 22,127 predicted, 0 disagreements** in both directions, per document
  and per batch, every document round-tripping through its own serialization, and the residue closing on 39
  empty formulas plus exactly the 22,088 content elements the live population holds. `--self-test` 27/27 with
  six observed RED perturbations. The standing defect is unchanged and unfixed: figure-interior text reaches
  no record and no residual, `source_ref` is ambiguous under batched ingest, and preservation has come apart
  from faithfulness.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.12` — make the published conservation figure lead with the
  defect (5,896 / 31%) rather than the raw non-carry rate (8,648 / 46%), which bundles the intended
  furniture-layer and empty-formula exclusions with it, across the research report, the book chapter,
  `LIVE_ACHIEVEMENT_STATUS.md`, `CHANGES.md`, and [[ingest-drops-figure-interior-text]]. No measurement
  changes. Then `.2` — make a reviewed source region resolve by content identity rather than ordinal
  position, changing how `source_record` *resolves* a region and never the recorded anchor, under the
  three-way digest lockstep in [[reviewed-fixture-projection-digest-lockstep]].
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none.
