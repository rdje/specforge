# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

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
- In-flight uncommitted: none after this commit; no background job is running. The pointer's fixed prose
  is now capped at a derived 12 lines (`MEMORY_ARCHITECTURE.md` §6), leaving 42 for this block.
- Blockers: none. Owned, not fixed: `CLAIM-VERIFICATION-ADOPTION.7` (a published count does not
  re-derive), `.8` (census registry 109 of a declared 128, +1 per ledger-prepending slice),
  `SOURCE-IR-REPRODUCIBILITY.13` (frozen reviewed fixture not re-derivable), `docs/research/*.md` 63 of
  64 files and this pointer 44 of 50 lines (`LIVE-DOCUMENT-PRESSURE-HEADROOM.4`).
