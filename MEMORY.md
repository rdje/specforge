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
- Active unit: `SOURCE-IR-REPRODUCIBILITY.8`, then `.7`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`–`.8`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CLAIM-VERIFICATION-ADOPTION.1a`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`. The last six are tracking-only.
- Current state: `.9` gave provenance the coordinate it lacked — `source_batch` on the four
  `source_ref`-bearing records, written only for a batched run, so `(source_batch, source_ref)` addresses
  one converter item. Standing ambiguity measured corpus-wide: **14 of 78** artifacts, 83,262 records
  (74.4% of those 14) unaddressable by a bare ref; falsified against the retained converter bundles with
  **0 disagreements over 24**. A first pass claiming all 78 was wrong (it pooled `document_sections`,
  which legitimately shares a ref) and is corrected in the record. Earlier this session
  `STATUS-LEDGER-ROLLOVER` closed out: `.4a` re-derived the measurement `.3`/`.4` rested on (70 records,
  not the published 64 — that was the warning threshold), `.3` rolled the ledger (root 70 -> 44 records /
  102,748 -> 76,758 bytes), and `.4` derived and published the per-record budget plus the live record
  count no producer reported before, finding **two of four** declared record windows unreachable. The
  standing figure-interior ingest defect is unchanged and unfixed.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.8` — a typed carrier or an explicit residual for the 5,896
  figure-interior text items ingest discards, never promoted into `content_elements` as prose. It is the
  last prerequisite `.7`'s conservation gate waits on now that `.9` is done.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none. Owned, not fixed: `CLAIM-VERIFICATION-ADOPTION.7` (a published count does not
  re-derive), `.8` (census registry 109 of a declared 128, +1 per ledger-prepending slice),
  `SOURCE-IR-REPRODUCIBILITY.13` (frozen reviewed fixture not re-derivable), `docs/research/*.md` 63 of
  64 files and this pointer 44 of 50 lines (`LIVE-DOCUMENT-PRESSURE-HEADROOM.4`).
