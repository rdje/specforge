# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `CLAIM-VERIFICATION-ADOPTION.7`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`/`.9`/`.12`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`/`.7`. The last ten are tracking-only.
- Current state: `STATUS-LEDGER-ROLLOVER.5` and `CHANGES-LEDGER-ROLLOVER.5` are closed — **both root ledgers
  are rolled in one transaction**, because the status rollover's own ledger record is what crossed
  `CHANGES.md`'s 90% line signal and could not be dodged by shortening it. Status root 56 -> 43 records /
  71,626 bytes (62.3%); change root 93 -> 80 records / 1,179 lines / 182,298 bytes (71.5% bytes, 65.5%
  lines). Segments `segment-0012-2026-08-30.md` and `segment-0016-2026-08-30.md`; older segments and both
  source capsules byte-identical. Before them, `CLAIM-VERIFICATION-ADOPTION.7` closed: the published-assertion
  registry is `frozen` at 0 unlisted values with a discovered, fail-closed scope.
- Next action: pick the next open unit. The largest untouched body of work is the `SOURCE-IR-REPRODUCIBILITY`
  lane (`.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`). Nearer: `CLAIM-VERIFICATION-ADOPTION` `.8` (census registry
  lifecycle), `.9` (candidate vocabulary blind spot), `.12` (the per-slice region re-pin is hand work with a
  silent-wrong-line hazard), `CHANGES-LEDGER-ROLLOVER.4` and `STATUS-LEDGER-ROLLOVER.2` (both own the same
  standing limit: a pinned migration suffix that consumes two thirds of the budget before any current record
  exists, which is why every rollover buys only about a dozen slices).
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`/`.12` (the per-slice region re-pin is hand work with a silent-wrong-line hazard);
  `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`/`.7`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe, so the residue
  recurs. Never run the fixture suite concurrently with the locality gate:
  `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a fixture run
  deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover, so do not write a
  research record until `.4` releases that surface. Read every live-document percentage from
  `perl scripts/check_live_document_size.pl` rather than carrying it here; `docs/book/src/pipeline/evidenceir.md`
  is already in the rollover band and no slice caused it. Both root ledgers are climbing toward their mandatory
  90% rollover — read the exact figures from `perl scripts/check_rolling_ledger_protocol.pl --report` and expect
  a rollover before many more slices (`CHANGES-LEDGER-ROLLOVER.4`, `STATUS-LEDGER-ROLLOVER.2` own them). Also
  unowned by any gate, recorded in `.11`: `DOCTRINE_ENFORCEMENT.md` §10 is called the driver registry's lockstep
  mirror and nothing checks it. `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` owns the unassigned gate-level live-doc
  warnings.
