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
- Current state: `CLAIM-VERIFICATION-ADOPTION.7.0` is closed and the correcting sequence is deliberately over.
  Asked a third time, `.11a` was re-derived and does NOT fully hold either: its warning-line total was
  invalidated by its own commit's `CHANGES.md` prepend crossing another band, and its "exactly three surfaces"
  was a classification published without its predicate (five files carried the literal; two are dated records §1
  exempts). Both WITHDRAWN, not corrected. Everything else in `.11a` re-derives. Five consecutive rounds —
  `.6`/`.6a`/`.6b`, `.10`/`.10a`, `.11`/`.11a` — each invalidated by its own transaction, which settles the
  design question `.7` had left open: prose cannot close this class, so `.7` is split into `.7.0` freeze,
  `.7.1` gate, `.7.2` population. The frozen contract: execute the producer and compare the named field; four
  outcomes (`derived`/`gated`/`authored`/`dated`) with NO slot for "a trajectory shows it has held"; the
  governed population derived at check time and never stored; a declared `excludes_self` because `.11`'s
  classifier went green for the surfaces it named BECAUSE it named them; cross-surface disagreement with no
  producer run; membership compared as an enumeration rather than a size; mechanism claims out of scope with an
  `adjudicated_against` field.
- Next action: `CLAIM-VERIFICATION-ADOPTION.7.1` — implement `scripts/check_published_assertions.pl` plus a
  self-bounded `doctrine/claim_verification/published_assertions.jsonl`, registered through
  `scripts/check_doctrines.sh` with no duplicated hook or CI wiring. Argv-form producers only, never a shell
  string; every region, producer and input Git-tracked and digest-bound. `--self-test` must instantiate every
  outcome family positively and drive all seven `.7.0` faults RED on a disposable repository-local fixture: a
  drifted value; a value bound to the wrong field; a published value in a governed region no record lists; a
  `gated` record whose control has no known-bad case; a self-referential record without `excludes_self`; two
  records disagreeing on one producer+field; and a membership record whose enumeration drifted while its size
  held. The last is the one that separates this gate from a numeral scanner.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`; `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe, so the residue
  recurs. Never run the fixture suite concurrently with the locality gate:
  `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a fixture run
  deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover, so do not write a
  research record until `.4` releases that surface. Read every live-document percentage from
  `perl scripts/check_live_document_size.pl` rather than carrying it here. Also unowned by any gate, recorded in
  `.11`: `DOCTRINE_ENFORCEMENT.md` §10 is called the driver registry's lockstep mirror and nothing checks it
  (both 11 today). `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` owns the unassigned gate-level live-doc warnings.
