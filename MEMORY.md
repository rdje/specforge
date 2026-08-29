# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2b`, then `.2c`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`/`.1a`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`. The last eight are tracking-only.
- Current state: `.2a` removed the 160-file cap on `docs/tasks/` as a DECLARED exemption, not a null.
  `surfaces.jsonl` carries a `cardinality_exemption` on `task_evidence` (authority ADR 0045, work unit, route
  `task_tree_index`, rationale); `check_live_document_size.pl` gained `validate_cardinality_exemption`, which
  refuses a bare null, a one-band null, an exemption that also unbounds a resource axis, a route that is
  itself / unregistered / not covering the declared index / unbounded, and an untracked authority. The second
  enforcer moved in the same transaction: `$MAX_TASKS` is gone from `check_task_tree_catalog.pl`. Suite 84 ->
  92, all pass; the published count was corrected on both surfaces carrying it. This RELOCATES the stop to
  `docs/TASK_TREE.md` (404/512 lines, ~108 further trees), which `.2c` must shard.
- Next action: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2b` FIRST and immediately — delete the single-use
  `task_evidence` record from `doctrine/live_document_size/ceiling_increase_authorities.jsonl` now that HEAD
  carries the new bands, and confirm the generic gate's `unused or banked ceiling-increase authority` refusal
  is what makes the retirement mandatory rather than cosmetic (run the gate before and after; the "before"
  observation is the control). Leaving it banked fails the very next commit. Then `.2c`: shard
  `docs/TASK_TREE.md` into a bounded landing plus derived catalog parts, following the two remedies this repo
  already proves — the Knowledge Map's landing plus 15 question shards, and `fact_card_titles`' 5 title parts —
  keeping it derive-and-diff generated with no hand-edited member list and every existing route resolving.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`; `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`; `SCRATCH-RESIDUE-CONTAINMENT.4` —
  the `generated/` fixture producer is still signal-unsafe, so the residue recurs. Never run the fixture suite
  concurrently with the locality gate: `check_persisted_artifact_paths.pl` walks every `*.json` under
  `generated/` and FAILS if a fixture run deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling
  with no rollover, so do not write a research record until `.4` releases that surface (`.jsonl` rollover plans
  there do not count). `CHANGES.md` is 66% — roughly ten records of headroom.
