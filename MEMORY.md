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
- Current state: `CLAIM-VERIFICATION-ADOPTION.11a` is closed. `.11` enumerated the claim-surface population three
  earlier leaves swept from memory and withdrew six stale publications plus three carried-but-correct constants as
  one class. Asked whether those findings held, `.11a` re-derived them and **five did not**. Two mechanism claims
  read off co-occurrence rather than off what a commit changed: only ONE of three commits republished
  `identity_gated 7`, and `d23e8bae` withdrew `current_surfaces` from TWO publishers, not three — quoted from that
  commit's own body instead of re-derived. Two set claims with the wrong denominator: "18 lines / 13 surfaces" is
  `check_live_document_size.pl`'s output where the doctrine driver emits 35 lines across FOUR producers; and the
  ownership grep scored `done` trees as owners (hiding `corpus_task_evidence_parts`) and turned green for the
  three surfaces it named, because writing the finding is what made `docs/tasks/` mention them. One reported
  defect withdrawn entirely: the `Opening Pressure Boundary (92e59c97)` table is a dated snapshot §1 exempts.
  Both corrections are STRONGER than what they replace. Everything else re-derives unchanged. `.11`'s own
  prepends moved the warned surface count 13 -> 15; both new ones are owned.
- Next action: `CLAIM-VERIFICATION-ADOPTION.7`, unchanged in scope by `.11` and sharpened in evidence. Build the
  bounded declared map binding each published count in a claim-annotated prose region to its producer command and
  the exact field of that producer's report, re-deriving and comparing rather than pattern-matching numbers out of
  prose; an unlisted count in a governed region is reported, not ignored. Keep the two residues `.10` identified
  (cross-surface disagreement; an optional field naming the prior adjudication a mechanism claim was checked
  against) and add the one `.11` proves is load-bearing: the map's POPULATION must be derived by a command, since
  every miss so far came from a remembered surface list rather than a wrong number, and `.11a` adds that a
  screen the finding itself can satisfy is not a classifier at all. RED controls must prove a
  drifted count, a count bound to the wrong field, and an unmapped count in a governed region.
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
