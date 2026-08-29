# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `CLAIM-VERIFICATION-ADOPTION.6b`, then `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a`/`.2b`/`.2c`. Open:
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.7`/`.8`/`.1a`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `SPEC-TO-INTENT-ALIGNMENT.9`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`. The last seven are tracking-only.
- Current state: `.6a` measured the four drifted `TOOLBOX.md` census counters across 28 consecutive revisions
  (`e6f5012d` -> HEAD) plus `.6`'s `50775894` anchor, in a detached worktree, each with that revision's OWN
  checker, and the answer changes the diagnosis. `registered` went
  6 -> 5 and closure 86/51/35 -> 72/50/22 INSIDE `5fe81128` — the commit that published them as "confirmed
  unchanged" — then 4 and 69/49/20 at `1507adbf`. Same-transaction invalidation, not decay, so no maintenance
  pass could have caught it. All four are withdrawn from `TOOLBOX.md` and routed to `--report` with their
  producer fields named; the six stable at every measurement (derived 11, identity-gated 7, no incomplete, unresolved
  0, 39 surfaces, 5 views) are carried. `.6`'s own unit trajectory is corrected (59, not 60, at `5fe81128`) and
  its "one more excluded unit per rolling-ledger head" rule withdrawn: over the 27 transitions from `e6f5012d`
  the total rises 15 times, FALLS twice, and holds 10 times. Rig gotcha recorded as `[[worktree-doctrine-measurement-gitlink]]`:
  a worktree does not populate the `subs/fsmgen` gitlink, so the census exits before printing at every revision.
- Next action: `CLAIM-VERIFICATION-ADOPTION.6b`, because `.6`/`.6a` swept `TOOLBOX.md` only and two surfaces
  still publish the same counts, measured stale at `60a81db7`: `docs/book/src/reference/doctrine-enforcement.md`
  ("56 exact evidence units: 11/7/6/0/32" vs 70 and 11/7/4/0/48; "regions=307, registered=8, incomplete=78,
  excluded=221" vs 321/8/89/224) and `docs/knowledge/current-claim-census-freeze.md` (the same 56-unit
  sentence, "78 incomplete", and "56 exact authority units" in its own title). Withdraw what `.6a` proved
  moves, correct the stale constants, leave `.3c`/`.4`/`.5` boundary sentences dated. Editing the book moves
  the frozen region set, so regenerate `doctrine/claim_verification/book_quantitative_claims.jsonl` in the SAME
  commit and re-derive TOOLBOX's mdBook incomplete count AFTER the edit. Then `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a`:
  null `task_evidence.files` in both bands behind a DECLARED exemption the checker enforces, delete `$MAX_TASKS`
  at `scripts/check_task_tree_catalog.pl:18` in the same transaction, add one exact record to
  `ceiling_increase_authorities.jsonl` plus an ADR, and observe four RED refusals; `.2b` then retires that
  single-use authority or the next commit fails on `unused or banked ceiling-increase authority`.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`; `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture
  producer is still signal-unsafe, so the residue recurs. Never run the fixture suite concurrently with the
  locality gate: `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a
  fixture run deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover, so do
  not write a research record until `.4` releases that surface.
