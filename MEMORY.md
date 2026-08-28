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
- Active unit: `SOURCE-IR-REPRODUCIBILITY.8` or `.9`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`–`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`; `SCRATCH-RESIDUE-CONTAINMENT.1`.
  `STATUS-LEDGER-ROLLOVER` is down to tracking-only `.2`. Tracking-only:
  `SCRATCH-RESIDUE-CONTAINMENT.1`, `SPEC-TO-INTENT-ALIGNMENT.9`, `PROVIDER-MODEL-STORE-LOCALITY.1`,
  `TASK-PART-SEAL-REACHABILITY.0`, `CLAIM-VERIFICATION-ADOPTION.1a`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `STATUS-LEDGER-ROLLOVER` closed out. `.4a` re-derived the measurement `.3`/`.4` rested on
  (70 records, not the published 64 — `64` is the record warning threshold); `.3` rolled the ledger (26
  records sealed as `segment-0011-2026-08-28.md`, root **70 -> 44 records / 102,748 -> 76,758 bytes**, older
  members byte-identical); `.4` derived and published the per-record budget and the live record count, which
  no producer reported before. The finding generalized: **two of four declared record windows are
  unreachable** — `changes` 109 of 128, `live-achievement-status` 68 of 80 — reported as non-fatal pressure
  because sealed records cannot be shrunk. Self-test 35 -> 41 with seven observed RED perturbations. `.4a`
  also repaired a stale assertion inside the claim registry itself. The standing ingest defect is unchanged
  and unfixed.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.8` (a typed carrier or an explicit residual for the 5,896
  figure-interior text items ingest discards) or `.9` (a batch-qualified `source_ref`, since `self_ref`
  restarts per batch and makes the join ambiguous). Both gate `.7`'s conservation gate.
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none. Owned, not fixed: `CLAIM-VERIFICATION-ADOPTION.7` (a published count does not re-derive),
  `.8` (census registry 109 of a declared 128, +1 per ledger-prepending slice),
  `SOURCE-IR-REPRODUCIBILITY.13` (frozen reviewed fixture not re-derivable), `docs/research/*.md` 63 of 64
  files and this pointer 44 of 50 lines (`LIVE-DOCUMENT-PRESSURE-HEADROOM.4`).
