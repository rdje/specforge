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
- Current state: `CLAIM-VERIFICATION-ADOPTION.7.2.0` is closed — **the gate's scope is derived and fails
  closed.** It was about to freeze on `governed_globs: ["TOOLBOX.md"]`, a stored list of one that would have
  called the map complete while the mdBook doctrine chapter sat outside it. Membership is now discovered by
  scanning every tracked Markdown file for a claim tag and resolved through
  `doctrine/live_document_size/surfaces.jsonl` (digest-bound); an undeclared surface, or a file no surface
  owns, is an error. Only a surface's disposition is authored. Two exemptions state their reasons (dated task
  evidence; sealed archive segments) and each is proven load-bearing by flipping it to `governed` and watching
  the same value turn fatal. Self-test 25/25; 2 governed / 2 exempt files, 6 governed regions, 27 unlisted,
  still `inventory` phase.
- Next action: `CLAIM-VERIFICATION-ADOPTION.7.2.1` — repair, then populate, then freeze. Two TOOLBOX
  sentences take no closed outcome as written: "the 27 transitions measured from `e6f5012d`" names an
  open-ended window whose boundary has moved (39 commits now), and "The 27-case self-test" is a live count
  whose producer reports it only as prose on stderr. Repair those, give every remaining governed value a
  closed outcome, then flip `phase` to `frozen`. Read the unlisted set from
  `perl scripts/check_published_assertions.pl --produce`; do not carry its size anywhere.
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
