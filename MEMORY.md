# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: none in flight; `CLAIM-VERIFICATION-ADOPTION.7` is closed. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`/`.7`. The last ten are tracking-only.
- Current state: `CLAIM-VERIFICATION-ADOPTION.7.2.1a` is closed — **`.7.2.1`'s findings were re-derived on a
  third reading and three of five did not hold.** The sharpest is this tree's eleventh instance and it is
  mine: `.7.2.1` published a commit count ending at `HEAD` inside the sentence explaining that an open-ended
  window silently grows. Withdrawn on three surfaces, with the command replacing the count. A run-counter
  figure and a share restated across two ledgers against a different budget were also withdrawn; the coverage
  blind spot (19/16/2/1) and the `governed_globs` scope finding re-derive exactly. `.7` is closed **over two
  governed files**, which is now stated wherever it is described.
- Next action: `CLAIM-VERIFICATION-ADOPTION.13` is the finding this exposed and the natural next unit — a
  published value on a surface carrying no `[claim: <id>]` tag is reached by no governed population, so the
  assertion gate's population is gated on an author remembering to write a tag. Decide whether the population
  extends from tag-bearing files to whole governed surfaces, or whether the bound is accepted explicitly.
  It is **not** `.9` (that is the book census's noun vocabulary, a different producer). Otherwise:
  `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`, or the `SOURCE-IR-REPRODUCIBILITY` lane
  (`.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`), which is the largest untouched body of work and wants a full
  roadmap and mdBook read first.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/
  `.12` (per-slice region re-pin is hand work with a silent-wrong-line hazard) / `.13`;
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
