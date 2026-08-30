# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.9` (`.9a` done, `.9b` next). Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`/`.7`. The last ten are tracking-only.
- Current state: **pivoted to the product lane.** Measured first: 6 of the last 60 commits touched `crates/` and
  28 consecutive commits had not, so governance had become the program rather than its foundation. Director's
  rule now standing — governance only when it is required for product progress. `SPEC-TO-INTENT-ALIGNMENT.9a`
  is closed: the remaining residual population is re-derived and `.8`'s description of it does not hold.
  8/16 reproduces exactly; the four required cells are `informational_disclaimer`, `software_guidance`,
  `table_of_contents`, `packed_page_table_entry`. `.8` said all four need `non_contract_region`; that is wrong
  for the last (it is intent-bearing, disposition `residual`), and no cell is waiting on the cause taxonomy at
  all because the reviewed dataset never checks `cause`.
- Next action: `SPEC-TO-INTENT-ALIGNMENT.9b` — ship the carrier. Generalise `.8c`'s structural rule by region
  kind (one residual per captured region no canonical record cites, for prose statements and table regions,
  not only figures) in `crates/specforge/src/ir/semantic.rs`; widen `project_captured_regions` in
  `crates/specforge/test_data/source_to_intent_vertical/build_fixture.py` past its `spec["region"][0] !=
  "figure"` gate; keep `non_contract_region` declared-unbuilt for prose in
  `doctrine/spec_to_intent/residual_actionability_contract.json`. Then replay the population with
  `scripts/replay_source_to_intent_population.py` — the eight external sources are retained under
  `.project-data/tmp/spec-to-intent-external-sources/`. Do not claim an evidence-capture move: two of the four
  cells also have empty `evidence_capture.matched_keys`, which is a separate leg.
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
