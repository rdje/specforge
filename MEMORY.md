# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.9b` (declared; implementation next). Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`/`.7`. The last ten are tracking-only.
- Current state: product lane. `.9b` is declared and the bounded root is rolled (89.8% -> 81.2% of its line
  target), which was mandatory: the root could not take another line. `.9b` measured `.9a`'s design against the
  retained corpus first and split `.9` by region kind. The table leg is buildable now; the prose leg is blocked
  because `EvidenceIR` carries no `SourceIR` content-element identity (`elem_\d+` occurs zero times in a
  complete `evidence_ir.json`), which `.9c` owns; two reviewed cells expect a residual label
  (`table_0004|toc_non_contract`, `elem_00017|informational_non_contract`) that no non-circular projection can
  produce; and `informational_disclaimer` is unreachable by any structural rule because its statement carries
  the `source_fact` fallback class.
- Next action: implement `.9b` — widen `residual_accountable_region_kind` in `crates/specforge/src/ir/semantic.rs`
  to admit `VisualAssetKind::TableRegion`, add a `supporting_table_ids`-based coverage gatherer beside
  `cited_provenance_ids` (over `register_records`, `timing_constraints`, `conditional_rules`,
  `signal_polarities`, and the interface-signal records), carry both `evidence_id` and `asset_id` in a table
  region's `supporting_evidence_ids`, and widen `project_captured_regions` in
  `crates/specforge/test_data/source_to_intent_vertical/build_fixture.py` past its `spec["region"][0] !=
  "figure"` gate for `table` only. Every figure-kind residual `.8c` publishes must stay byte-unchanged. Then
  rebuild the retained chains and run the doctrines. The population replay is a later leaf; it requires
  unchanged `crates/specforge/src` and the eight external sources retained under
  `.project-data/tmp/spec-to-intent-external-sources/`.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/
  `.12` (per-slice region re-pin is hand work with a silent-wrong-line hazard) / `.13`;
  `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`/`.7`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe, so the residue
  recurs. Never run the fixture suite concurrently with the locality gate:
  `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a fixture run
  deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover, so do not write a
  research record until `.4` releases that surface. The alignment task index is now the pressured
  task-evidence surface at 87.5% of its line target, so `.9c` and later leaves should expect an index rollover
  before many more leaf routes. Read every live-document percentage from
  `perl scripts/check_live_document_size.pl` rather than carrying it here; several persisted `generated/`
  chains are still stale schema-1 artifacts (AMD IOMMU, GIC-400, APB, both RISC-V), so measure only against
  schema-2 chains until they rebuild. Also unowned by any gate, recorded in `.11`:
  `DOCTRINE_ENFORCEMENT.md` §10 is called the driver registry's lockstep mirror and nothing checks it.
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` owns the unassigned gate-level live-doc warnings.
