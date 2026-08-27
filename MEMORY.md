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
- Active unit: `SOURCE-IR-REPRODUCIBILITY.5` is the gating next child; `.0`/`.1` are done and `.6`/`.7` are
  new. Tracking-only: `SCRATCH-RESIDUE-CONTAINMENT.1`, `SPEC-TO-INTENT-ALIGNMENT.9`,
  `PROVIDER-MODEL-STORE-LOCALITY.1`, `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.1` censused all 78 persisted `generated/source_ir/*` artifacts — 24 live (exactly the
  chain-currency retained set) and 54 legacy — and re-ingested the whole live stratum: 11 reproduce, 13 do not.
  Ingest must not drop information present in the source PDF, so `.1`'s re-ingest-versus-retain framing is
  **withdrawn**: both branches are defective, and the persisted corpus is itself missing the 1,804
  figure-interior elements a current ingest recovers, so neither capture is complete. The caption loss is
  measured and recoverable — both runs label the same seven texts `caption`; only the figure-to-caption
  reference disappears, and items carrying one fall seven to five as `texts` rises 373 to 469. The added
  elements and the lost bindings are one upstream cause, not two. What is still unknown is the three absent
  paragraphs, and nothing today measures PDF-to-SourceIR conservation at all.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.5` — for each of the three content elements the census found
  absent from a re-ingest (USB4 Connection Manager guide, USB 3.2, Wishbone), state whether the text is absent
  from Docling's own document or present there and dropped by SpecForge's element construction. That single
  measurement decides whether losslessness is reachable in our code, and it gates `.6` (rebuild the unattached
  caption binding deterministically, fail closed on ambiguity) and `.7` (the missing ingest conservation gate).
  `.2` content-addressed anchors stays runnable in parallel; its digest lockstep is already scoped.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none.
