# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.9d` (next). Open in `.9`: `.9c` (prose leg), `.9d` (contract re-pin
  and gate), and the population replay. Also open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`/`.7`. The last ten are tracking-only.
- Current state: product lane. `.9b` shipped: the captured-region residual carrier now covers captured table
  regions, with coverage asked per kind — a visual region by `evidence_id`/`figure:<asset_id>`, a table region by
  its table id in the new `SemanticIr::cited_table_ids` over all seven declaring surfaces. Neither vocabulary can
  match the other's ids, so every figure residual `.8c` published is unchanged by construction. The prose leg is
  blocked upstream (`EvidenceIR` carries no `SourceIR` content-element identity) and is `.9c`.
- Next action: `SPEC-TO-INTENT-ALIGNMENT.9d`. Two halves, both measured in `.9b`. (1) Re-pin the frozen
  `doctrine/spec_to_intent/residual_actionability_contract.json` `witness`: `current_result_sha256` must become
  the tracked snapshot's digest, and the `software_guidance` row of `required_and_absent_cells` must drop
  `source_region_missing_or_ambiguous`, which `SOURCE-IR-REPRODUCIBILITY.2` (`245b3b60`) genuinely repaired after
  `.8d` (`893c2fba`) froze the witness. No published count moves — the derivation is still 4/4/4 cells and
  24/8/8/8/16/8 observations. (2) Register `scripts/validate_residual_actionability_contract.py` in
  `scripts/check_doctrines.sh`: nothing in the repository executes it today, which is why it went red for
  several commits unnoticed. Reproduce both with
  `python3 -B scripts/validate_residual_actionability_contract.py --check`.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/
  `.12` (per-slice region re-pin is hand work with a silent-wrong-line hazard) / `.13`;
  `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`/`.7`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe, so the residue
  recurs. Never run the fixture suite concurrently with the locality gate:
  `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a fixture run
  deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover, so do not write a
  research record until `.4` releases that surface. CONTAINMENT, BLOCKING FOR THE NEXT LEAF: `.9b` had to split
  `residual-carrier.md` at the `.8`/`.9` boundary into the new `region-kind-generalisation.md` part because the
  semantic-part collection crossed its mandatory `lines_each` rollover mid-slice. That returned the collection to
  83.8% but cost one active-task-index row, and the index is now at **89.1%** — one further leaf route crosses
  its mandatory rollover, so `.9c`/`.9d` must roll the index BEFORE declaring a leaf. The bounded root is at
  85.2%, and a root Verification Log row must stay under about 437 bytes or it trips `line_bytes` on its own. Read every live-document percentage from
  `perl scripts/check_live_document_size.pl` rather than carrying it here. Several persisted `generated/`
  chains are legacy proofless schema-1 artifacts (AMD IOMMU, GIC-400, APB, both RISC-V) outside the 24
  proof-carrying stratum, so measure only against schema-2 chains. Also unowned by any gate, recorded in `.11`:
  `DOCTRINE_ENFORCEMENT.md` §10 is called the driver registry's lockstep mirror and nothing checks it.
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` owns the unassigned gate-level live-doc warnings.
