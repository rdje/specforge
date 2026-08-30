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
- Current state: `CLAIM-VERIFICATION-ADOPTION.7.2.1` is closed, and with it **`.7`** — **the map is closed and
  the registry is `frozen`.** Every published value in a governed region resolves to a record with a closed
  outcome (30 assertions: 1 derived, 2 gated, 1 authored, 26 dated), so an unmapped numeral now fails the
  build. Three probes on real shipped prose, all restored byte-exact: an unlisted value, a drifted dated value
  (three legs at once), and a claim-annotated file on an undeclared surface. Two sentences were repaired
  rather than recorded — an open-ended window now names its closing revision, and a live count whose producer
  publishes it only as prose lost its numeral. Read every figure from
  `perl scripts/check_published_assertions.pl --report`; do not carry it here.
- Next action: **`STATUS-LEDGER-ROLLOVER.2` is now mandatory and must go first.**
  `LIVE_ACHIEVEMENT_STATUS.md` reached 90.08% of its 115000-byte health target on this slice and was only
  brought back under (89.9%) by tightening this slice's own entry — the next ordinary append crosses it again.
  Follow `COMMIT.md`'s rolling-ledger rollover: write a task-owned JSONL plan, dry-run
  `perl scripts/check_rolling_ledger_protocol.pl --rollover-plan <plan>`, then `--apply-rollover` on the exact
  green plan. Read the live figures from `--report`; do not carry them here.
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
