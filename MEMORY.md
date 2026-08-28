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
- Active unit: `CLAIM-VERIFICATION-ADOPTION.7`. `SOURCE-IR-REPRODUCIBILITY` `.0`/`.1`/`.5`/`.11`/`.12` are done
  with `.2`/`.3`/`.4`/`.6`–`.10` open; `CLAIM-VERIFICATION-ADOPTION` reopened at `.6` done / `.7` open;
  `CHANGES-LEDGER-ROLLOVER` reopened at `.2` done (ledger back to 77.2% of its line health target).
  Tracking-only: `SCRATCH-RESIDUE-CONTAINMENT.1`, `SPEC-TO-INTENT-ALIGNMENT.9`,
  `PROVIDER-MODEL-STORE-LOCALITY.1`, `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: `SOURCE-IR-REPRODUCIBILITY.11` replaced the tree's one inferred mechanism with a control —
  `--oracle` asks docling-core itself, and on all 24 retained converter documents the drop model agrees exactly
  (43,614 text items, 22,127 yielded, 22,127 predicted, **0 disagreements**, every document round-tripping,
  residue closing on 39 empty formulas plus exactly the 22,088 content elements the live population holds).
  `.12` made the published headline the actionable one: **5,896 of 18,870 (31%) dropped as a defect**, raw 8,648
  (46%) as decomposed context. `CLAIM-VERIFICATION-ADOPTION.6` then re-derived all 11 counts `TOOLBOX.md`
  publishes about the claim doctrines — 8 confirmed, **3 stale** (56→59, 32→35, 75→89) — and named why the gate
  could not see them: an annotated region closes on the annotation's *presence*, and the digest leg proves only
  that a file is unchanged. Neither reads a number. `.6`'s own record crossed the ledger's 90% line signal, so
  `CHANGES-LEDGER-ROLLOVER.2` sealed 18 records into `segment-0013-2026-08-28.md` in the same commit, as the
  protocol requires; one census region went with them and was retired rather than re-anchored.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `CLAIM-VERIFICATION-ADOPTION.7` — bind each published count in a claim-annotated prose region
  to its producer command and the exact field of that producer's report, re-derive and compare rather than
  pattern-matching numbers out of prose, and report an unmapped count in a governed region rather than ignoring
  it; RED controls for a drifted count, a count bound to the wrong field, and an unmapped count. Then return to
  `SOURCE-IR-REPRODUCIBILITY.2` (resolve a reviewed source region by content identity, never by ordinal
  position, under the lockstep in [[reviewed-fixture-projection-digest-lockstep]]).
- In-flight uncommitted: none after this commit; no background job is running.
- Blockers: none.
