# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SOURCE-IR-REPRODUCIBILITY.7`, then `.14`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`/`.14`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CLAIM-VERIFICATION-ADOPTION.1a`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`. The last six are tracking-only.
- Current state: `SIGNOFF-REMEDIATION.3` restored workspace formatting (whitespace + one inert trailing
  comma; all 10 pinned digests byte-identical, snapshot validator green). Before it, `.8` closed the
  figure-interior drop. `VisualAsset` carries `interior_texts`, filled from
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
- Next action: run `SOURCE-IR-REPRODUCIBILITY.7` — both its prerequisites are now discharged (`.8` the
  carrier, `.9` the exact join). Its gate must distinguish an artifact written with the carrier from one
  written before it, or it fails closed on all 24 persisted artifacts.
- In-flight uncommitted: none after this commit; no background job is running. This pointer's fixed prose
  is capped at a derived 12 lines (`MEMORY_ARCHITECTURE.md` §6); the gate prints the room left each run.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.14` (measured at HEAD `3833ad10` before this
  work: `specforge validate` verified 0/24, ruleset-stale 24/24; `check_chain_currency.sh` 0 current / 24
  stale at all four stages — a stale seal, not stale content), `.13` (frozen reviewed fixture not
  re-derivable), `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/`.9` (`.9`: the book census's noun list cannot see
  `items`/`elements`/`texts`/`bundles`/`figures`, so five new quantities raised no alarm — the tool
  declares itself a lexical alarm, so this is a blind spot, not an over-claim), and
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`.
- One thing blocks a push now: `run_ci.sh` runs `check_doctrines.sh --all` first under `set -euo
  pipefail`, and CHAIN-CURRENCY fails 0 current / 24 stale — `.14`'s seal debt. `SIGNOFF-REMEDIATION.3`
  cleared the formatting drift that blocked its third step.
