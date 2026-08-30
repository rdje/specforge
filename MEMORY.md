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
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`/`.7`. The last ten are tracking-only.
- Current state: `CLAIM-VERIFICATION-ADOPTION.7.1a` is closed — **the gate's own population scanner had a blind
  spot, and it is measured and closed.** `check_published_assertions.pl` derives its governed population, so its
  numeral grammar decides which published values exist; a value the scanner cannot see is a silent hole, not an
  unlisted one, and `frozen` phase would have called the surface complete anyway. An independent tokenizer
  (the gate's own lookbehind kept, only the lookahead widened) found 19 dropped values across the four
  claim-annotated files, one of them live — `TOOLBOX.md`'s "The 27-case self-test". Repaired: a comma joins a
  numeral only before exactly three digits, and only a hyphen followed by a digit is excluded. Self-test 19/19,
  all three new cases proven grammar-dependent by revert-and-re-apply at 14/19. Real-tree unlisted 21 -> 23.
  The registry still ships in `inventory` phase.
- Next action: `CLAIM-VERIFICATION-ADOPTION.7.2` — it now has three parts, not one. (1) Replace
  `governed_globs: ["TOOLBOX.md"]`, which is the stored surface list `.7.0` element 3 forbids, with a derived
  rule; as shipped, freezing would make one surface fatal and leave the book chapter — instance 5's own
  surface — unwatched. (2) Give every value in the resulting population a closed outcome. (3) Flip `phase` to
  `frozen`. Note the whole-tree population does not fit `max_records: 128`, and raising the bound to make it
  fit is the move `.8` refuses. Read the current unlisted set from
  `perl scripts/check_published_assertions.pl --produce`; do not carry its size anywhere.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`; `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`/`.7`;
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
