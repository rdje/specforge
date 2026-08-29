# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SOURCE-IR-REPRODUCIBILITY.7`, then `.3`/`.4`/`.6`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CLAIM-VERIFICATION-ADOPTION.1a`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`. The last six are tracking-only.
- Current state: `.16` closed the seal-latency hole `.14` measured at 13 days / 54 commits.
  `PROOF-SEAL-CURRENCY` (`scripts/check_proof_seal_currency.sh`) is registered **gate** tier, so every
  commit now reads the seal of every persisted artifact at all five chain stages and asks the current
  build's own loader whether it still accepts it: **24/24 sealed, 1 distinct seal per stage, 4 probes
  accepted, exit 0 in 14.1 s** (the gate measured 4m21s without it, 3m02s with it — variance
  dominates that comparison). The census is TOTAL and the probe is
  REPRESENTATIVE (one per distinct seal), so representativeness is measured, not assumed. Read-only is
  proved — all 120 in-scope artifacts byte-identical across a run — because the probe is the CONSUMING
  stage in `--dry-run`, never `specforge validate`. RED control is the real loader, not a stub: zeroing a
  copied artifact's ledger digest reproduces `.14`'s exact `proof ledger ruleset hash is stale`, exit 1,
  classified with its remedy. Seal reading is now shared with the remedy in
  `scripts/lib/proof_seal_scan.sh` (`.11`); sabotaging it drives the gate 16/16 -> 10/16 and the cascade
  14/14 -> 11/14. Predicate rewrite held to an agreement census, not review: **390 artifacts, 0
  mismatches**, 5.1x faster; prefilter soundness **120 candidates / 120 exact positives / 0 unsound**.
  Reported, not papered over: the terminal `isf-adapter` stage has NO read-only canonical probe (and
  CHAIN-CURRENCY does not close it — its comparison excludes the proof surface), and a current seal is
  not content currency.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.7` — the ingest-conservation gate. Its two prerequisites are
  discharged (`.8` carrier, `.9` exact join key); it must distinguish an artifact written with the carrier
  from one written before it, or it fails closed everywhere at the persisted corpus's numbers.
- In-flight uncommitted: none after this commit. This pointer's fixed prose is capped at a derived 12
  lines (`MEMORY_ARCHITECTURE.md` §6).
- Blockers: none. Owned, not fixed: `.13` (frozen reviewed fixture not re-derivable),
  `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/`.9`, `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`,
  `SCRATCH-RESIDUE-CONTAINMENT.4`. The `generated/` fixture residue is no longer unowned: `.3` narrowed
  that tree's `generated/` Non-Goal on a measured falsification (`specforge clean`'s three scopes reach
  it only via `--scope all-generated`, which discards the corpus) and reclaimed **317 roots / 7,630
  files / 32 MB, 0 remaining**. Mechanism measured, not inferred: killing the producer leaks every live
  fixture under SIGKILL and SIGTERM (15/15, 15/15, 20/20 each); killing only the wrapper and orphaning
  the producer leaks **0**; SIGINT stays honestly unmeasured. `.4` owns prevention — the producer is
  still signal-unsafe, so the residue recurs. Watch the exposure: `check_persisted_artifact_paths.pl`
  (locality gate) walks every `*.json` under `generated/` and FAILS if a fixture run deletes one
  mid-walk, so never run the suite concurrently with the gate.
