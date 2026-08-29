# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a`, then `.2b`/`.2c`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`/`.1a`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`. The last eight are tracking-only.
- Current state: the census-count sweep is finished. `.6a` measured the four `TOOLBOX.md` counters over 28
  consecutive revisions with each commit's own checker and found all four already false INSIDE `5fe81128`, the
  commit that published them as "confirmed unchanged" — same-transaction invalidation, not decay. `.6b` then
  found the same drift live on the mdBook doctrine chapter and the census fact card, attributed it from the
  31 tracked revisions of `book_quantitative_claims.jsonl` (307/8/78/221 was RIGHT at `be3b12e6` and stale for
  12 days across 7 registry changes), and routed all of it to `--report`. Carried everywhere now: `derived` 11,
  `identity_gated` 7, no `incomplete`, `unresolved` 0, 39 surfaces, 5 views. `.6b`'s own commit proved the
  point again — its rollover took `evidence_units` 71 -> 59 while every carried field held — and it also
  CORRECTED `.6a`, which had published an unmeasured mechanism ("sealed 18 while adding 2"): `fdda3c53` and
  `5fe81128` both hold 59 evidence rows and differ by one row removed and one added. `.6a`'s numbers all
  re-derive; only its cause was wrong. Recorded as `.7`'s seventh instance — a count-only gate passes a false
  mechanism, so `.7` must decide explicitly whether causal accounts are in scope.
- Next action: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a` — null `task_evidence.files` in BOTH bands behind a
  DECLARED exemption the checker enforces (every resource axis stays numeric, both bands null together, a
  bounded reader-facing route names a different registered surface covering the index), delete `$MAX_TASKS` at
  `scripts/check_task_tree_catalog.pl:18` in the SAME transaction — it is a second, independent enforcer of the
  same 160 cap, so a registry-only change reads as delivered and is not — add one exact record to
  `doctrine/live_document_size/ceiling_increase_authorities.jsonl` plus an ADR, and observe four RED refusals
  in `scripts/test_live_document_size.pl` (undeclared null, half-declared null, exemption that also unbounds a
  resource axis, exemption whose route is unregistered or unbounded). `.2b` must then retire that single-use
  authority or the next commit fails on `unused or banked ceiling-increase authority`; `.2c` shards
  `docs/TASK_TREE.md`, which is where the stop actually relocates (about 108 trees).
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`; `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`; `SCRATCH-RESIDUE-CONTAINMENT.4` —
  the `generated/` fixture producer is still signal-unsafe, so the residue recurs. Never run the fixture suite
  concurrently with the locality gate: `check_persisted_artifact_paths.pl` walks every `*.json` under
  `generated/` and FAILS if a fixture run deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling
  with no rollover, so do not write a research record until `.4` releases that surface (`.jsonl` rollover plans
  there do not count). `CHANGES.md` is 64% after `.3`'s rollover — roughly eleven records of headroom.
