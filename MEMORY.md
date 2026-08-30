# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `CLAIM-VERIFICATION-ADOPTION.7`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`/`.9`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`. The last nine are tracking-only.
- Current state: `CLAIM-VERIFICATION-ADOPTION.7.1` is closed — **the gate exists and executes**.
  `scripts/check_published_assertions.pl` binds a published value in a governed region to a field of a named
  producer's JSON report, runs that producer, and compares. That is the leg ten recorded instances went stale
  for want of: a digest proves a region has not changed, never that its numbers still re-derive. Registered as
  the twelfth doctrine through the existing driver. Four closed outcomes (`derived`/`gated`/`authored`/`dated`)
  and a fifth is REFUSED, so a value can no longer be carried on a trajectory. Cross-surface disagreement fails
  with no producer run; a set is compared as an enumeration, never a size; `excludes_self` is required the
  moment an enumerator returns the record's own publishing surface. Self-test 16/16 on a disposable
  repository-local fixture, plus drift observed RED on REAL shipped prose by revert-and-re-apply — and an
  UNPROMPTED live catch inside its own commit, when the new fact card moved `card_count` 249 -> 250 while the
  seeded record still published 249. Ships in
  `inventory` phase with four seeded assertions; unlisted values are reported, not fatal, until `.7.2`.
- Next action: `CLAIM-VERIFICATION-ADOPTION.7.2` — populate `published_assertions.jsonl` from the governed
  regions the gate reports as unlisted, give each a closed outcome, and flip `phase` to `frozen` so an unlisted
  value becomes fatal rather than reported. Read the current unlisted set from
  `perl scripts/check_published_assertions.pl --produce`; do not carry its size anywhere.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`; `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe, so the residue
  recurs. Never run the fixture suite concurrently with the locality gate:
  `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a fixture run
  deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover, so do not write a
  research record until `.4` releases that surface. Read every live-document percentage from
  `perl scripts/check_live_document_size.pl` rather than carrying it here. Both root ledgers are climbing
  toward their mandatory 90% rollover — read the exact figures from
  `perl scripts/check_rolling_ledger_protocol.pl --report` and expect a rollover before many more slices
  (`CHANGES-LEDGER-ROLLOVER.4`, `STATUS-LEDGER-ROLLOVER.2` own them). Also unowned by any gate, recorded in
  `.11`: `DOCTRINE_ENFORCEMENT.md` §10 is called the driver registry's lockstep mirror and nothing checks it
  (both 11 today). `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` owns the unassigned gate-level live-doc warnings.
