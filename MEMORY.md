# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a`, then `.2b`/`.2c`. Open elsewhere:
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.7`/`.8`/`.1a`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `SPEC-TO-INTENT-ALIGNMENT.9`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`. The last seven are tracking-only.
- Current state: the director decided (`2026-08-29`) there is to be NO limit on the number of task-trees,
  and that a completed tree stays as project history. `.2` recorded that; splitting it into executable
  remedies showed it is not a one-line registry edit. The cap has TWO enforcers —
  `task_evidence.enforcement_ceilings.files = 160` in `doctrine/live_document_size/surfaces.jsonl`, and an
  independent `my $MAX_TASKS = 160;` at `scripts/check_task_tree_catalog.pl:18` — so a registry-only change
  would have read as delivered while the plane stayed capped. Measured at `c1609558`: `task_evidence` is
  151 files and the gate says `files ... rollover (94.4%) - 9 below its 160 ceiling`, while
  `task_tree_index` is 404 lines with `lines_each ... warning (84.2%) - 108 below its 512 ceiling` at one
  catalog row per tree. So `.2a` RELOCATES the nearest stop from 9 trees to about 108, and `.2c` is the
  half that removes it. Doctrine gate green at `c1609558`: 10/10 executed PASS, CHAIN-CURRENCY deferred.
- Next action: run `.2a`. Null `task_evidence.files` in both bands behind a DECLARED exemption the checker
  enforces (every resource axis stays numeric, both bands null together, and a bounded reader-facing route
  must name a different registered surface covering the index), delete `$MAX_TASKS` in the same
  transaction, add one exact record to `ceiling_increase_authorities.jsonl` plus an ADR, and observe the
  four refusals RED in `scripts/test_live_document_size.pl`. `.2b` must then retire that single-use
  authority, or the very next commit fails on `unused or banked ceiling-increase authority`.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`; `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture
  producer is still signal-unsafe, so the residue recurs. Never run the fixture suite concurrently with the
  locality gate: `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a
  fixture run deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover, so do
  not write a research record until `.4` releases that surface.
