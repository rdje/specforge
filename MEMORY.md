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
- Active unit: `STATUS-LEDGER-ROLLOVER.3` (blocks the next status-bearing commit), then
  `SOURCE-IR-REPRODUCIBILITY.8` or `.9`. In `SOURCE-IR-REPRODUCIBILITY`: `.0`/`.1`/`.2`/`.5`/`.11`/`.12` done;
  `.3`/`.4`/`.6`–`.10`/`.13` open. `CLAIM-VERIFICATION-ADOPTION` reopened (`.6` done, `.7` open);
  `CHANGES-LEDGER-ROLLOVER.2` done. Tracking-only: `SCRATCH-RESIDUE-CONTAINMENT.1`,
  `SPEC-TO-INTENT-ALIGNMENT.9`, `PROVIDER-MODEL-STORE-LOCALITY.1`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `.11` replaced the tree's one inferred mechanism with a control (`--oracle`: 24/24 retained
  converter documents, 22,127 yielded vs 22,127 predicted, **0 disagreements**). `.12` made the published
  headline the actionable one (**5,896 of 18,870 — 31% — dropped as a defect**; raw 8,648/46% as context).
  `.2` replaced ordinal reviewed anchors with content identity, taking exact source regions **13/14 → 14/14**
  with every other global metric identical and no reviewed fact moved; it publishes its own replay
  `source-ir-repro-2-population-r1` at revision `5fe81128`. `CLAIM-VERIFICATION-ADOPTION.6` re-derived all 11
  counts `TOOLBOX.md` publishes about the claim doctrines (8 confirmed, 3 stale). The standing ingest defect
  is unchanged and unfixed: figure-interior text reaches no record and no residual, `source_ref` is ambiguous
  under batched ingest, and preservation has come apart from faithfulness.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `STATUS-LEDGER-ROLLOVER.3` — `LIVE_ACHIEVEMENT_STATUS.md` is at 89.3% of its byte health
  target, so the next status record crosses the 90% signal; that leaf carries how to build the plan and why a
  hand-modelled cut fails. Then `SOURCE-IR-REPRODUCIBILITY.8` (typed carrier or residual for figure-interior
  text) or `.9` (batch-qualified `source_ref`); both gate `.7`'s conservation gate.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none. Defects found this session, owned, not yet fixed: `CLAIM-VERIFICATION-ADOPTION.7` (a
  claim-annotated prose count does not re-derive against its producer) and `SOURCE-IR-REPRODUCIBILITY.13`
  (the frozen reviewed fixture is not re-derivable: 12/12 chains drifted, 0/12 from EvidenceIR onward).
