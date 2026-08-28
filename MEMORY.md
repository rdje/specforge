# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SOURCE-IR-REPRODUCIBILITY.15`, then `.16`, then `.7`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`/`.15`/`.16`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CLAIM-VERIFICATION-ADOPTION.1a`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`. The last six are tracking-only.
- Current state: `.14` re-sealed the SourceIR corpus — **24 proof-only, 0 public content changed** against
  an 89 MB pre-write snapshot; `validate` 24/24; chain-currency `evidence` **0 -> 24 current**. Downstream
  `semantic`/`intent`/`adapter` still 0/24 on a different error (persisted EvidenceIR's cumulative seal),
  which needs a stage-rebuild cascade (`.15`) because only SourceIr has a proof-only re-seal path.
  Detection costs 0.18 s / 7.2 s versus ~20 min via chain currency, after 13 days / 54 commits of latency
  — so the check belongs at gate tier (`.16`). Earlier: `SIGNOFF-REMEDIATION.3` restored workspace
  formatting; `.8` closed the figure-interior drop. `VisualAsset` carries `interior_texts`, filled from
  the library's own traversal differenced against itself (`iterate_items(traverse_pictures=True)` minus the
  call production already makes), attributed up the parent chain, raising rather than dropping when a figure
  cannot be named. Population 13,506 over all 24 retained bundles, 0 orphans; on a real I2S re-ingest
  `picture_interior_not_traversed` goes **255 -> 0** with `content_elements` **115 -> 115**, so the labels are
  carried without entering prose. Landable by construction and by measurement: an empty carrier is
  byte-indistinguishable from no carrier, and `source_proof_migrate` re-derives 24/24 live artifacts as
  `verified`. Producer self-test 33 -> 37, four observed RED, two rewritten after first running GREEN.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.15` — rebuild evidence (already proven 24/24 content-current,
  so the safe starting point), then semantic/intent/adapter, diffing content per stage and attributing any
  delta per ADR 0025. That is the last thing blocking a push.
- In-flight uncommitted: none after this commit; no background job is running. This pointer's fixed prose
  is capped at a derived 12 lines (`MEMORY_ARCHITECTURE.md` §6); the gate prints the room left each run.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.14` (measured at HEAD `3833ad10` before this
  work: `specforge validate` verified 0/24, ruleset-stale 24/24; `check_chain_currency.sh` 0 current / 24
  stale at all four stages — a stale seal, not stale content), `.13` (frozen reviewed fixture not
  re-derivable), `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/`.9` (`.9`: the book census's noun list cannot see
  `items`/`elements`/`texts`/`bundles`/`figures`, so five new quantities raised no alarm — the tool
  declares itself a lexical alarm, so this is a blind spot, not an over-claim), and
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`.
- One thing still blocks a push: CHAIN-CURRENCY, now only at `semantic`/`intent`/`isf-adapter` (evidence
  is current). `.15` owns the downstream cascade; formatting and the SourceIR seal are both cleared.
