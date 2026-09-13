# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`TASK-NODE-RETENTION.0` CLOSED `2026-09-13` (DOCTRINE)** — **every other memory layer was guarded; the CONTENTS of a task tree
  were not.** My own `.3k.2j` commit (`ab2c6ee0`) spliced a node replacement from its `- ID:` line to the next section header and **deleted 8
  nodes** — four closed leaves' full records — while all 14 doctrines passed; it surfaced only because a later edit lost its anchor. Restored at
  `ae064329`. Shipped `scripts/check_task_node_retention.py`, registered `TASK-NODE-RETENTION|gate` (driver now runs **15**). The rule admits the
  one legitimate operation, derived from history not invented: over 200 revisions a node id vanished twice — once a real split (`.7.2` →
  `.7.2.0`/`.7.2.1`) and once my accident — and the descendant-or-declared discriminator replays GREEN on the first and RED on the second.
- Director directive `2026-09-13`, owned: the table-row producer must see ALL documents. `LEGACY-SOURCE-RECLASSIFICATION.0` opened with the
  measurement that makes it possible — `classified_table_kind` is a pure function of the PERSISTED table record, so running it over the 51 legacy
  artifacts' own preserved structure recovers **363 signal-description tables across 23 documents** (AXI 89, LTI 24, CHI/HBM2 8 each), with no PDF
  and no model. The neutralization gate itself is CORRECT and stays: those labels came from a retired identity-keyed classifier, measured 3,484
  table-label changes different from what this code says. What was wrong was the inference that the 51 were unrecoverable.
- Current state: thirteen commits. **The statement path and the row path now apply the same rules**: same modal vocabulary (`.3k.2d`), same
  vocabulary-slot refusals (`.3k.2f`), same typed gateway (`.3k.2e`) — and `replay-constraints` judges all three deterministic producers
  (`.3k.2g`), publishing **26 documents row-judged / 51 not**. **Three leaves in a row corrected a population the one before published**, and the
  lesson is sharper than "measure first": measure with the PRODUCER. `.3k.2d` refuted a mechanism taken from a test string; `.3k.2g` refuted two
  counts taken from a persisted FIELD the producer no longer trusts. 306 fact cards; 15 doctrines, 13 at gate tier; core lib 1,468.
- Next action: **`EXTRACTION-QUALITY-GAUGE.3k.3`**, whose population is **already re-derived in its node**: 3 records, not the published 4
  (NVMe `sigcon_0007` no longer reproduces). **Read the trap recorded there before touching the span**: AHB `sigcon_0002` is correct today by
  accident, and narrowing the classifier's span as the node originally described would make `.3k.2a` refuse it — losing a record the document
  supports. The defect underneath is that `extract_signal_constraints` takes only the FIRST modal sentence and drops obligations 2..n; one record
  per clause (the row reader's shape) is what to size, and it is RECALL, so measure it as an addition before shipping it as a narrowing.
- In-flight uncommitted: none after this commit.
- Blockers: none. Push cadence is **400** commits per director directive `2026-09-13` and is FIXED there, so no push is due at 231; directive 16
  still gates it on full CI. `scripts/check_doctrines.sh --all` did not finish in 50 minutes — `CHAIN-CURRENCY` re-executes the real pipeline for every
  persisted artifact across four stages. Budget hours, not minutes, and run it detached. Standing hazards: **an evidence-stage change stales the proof of every
  current-schema artifact whose content moves and they
  then refuse to LOAD** — rebuild (`evidence → validate → semantic → validate → intent → validate → adapt`, each validated exactly once,
  upstream-first). Only APB/AHB/AXI-L have held-out bundles (`generated/preserved/WIRE-BASED-100.10/`): restore, rebuild, `diff -r`, remove,
  retention back to 24. **Run `replay-constraints` before sizing any extractor change** (`[[persisted-census-measures-published-not-current]]`)
  and read `row_stratum_unjudged_documents` with it (`[[legacy-source-classifications-are-neutralized-on-load]]`). **A census counts the
  population of the FUNCTION being changed** (`[[constraint-record-producer-strata]]`), **enumerate a phrase/modal set in full before publishing
  a zero**, and **never size from a mirror** (`[[one-modal-vocabulary-per-constraint-record]]`). **A Rust change moves `flow_census.json` and the
  re-derivation must ATTRIBUTE the delta to the owning leaf**; a new module moves `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`;
  a new raw-evidence reader `information_flow_boundary.tsv` — all fail closed. **Editing a live surface sets off a fixed bookkeeping chain** —
  `[[live-surface-edit-bookkeeping-chain]]`; deleting a governed book line also RETIRES its region record and lowers
  `expected_candidate_lines`. **A book section about an extraction rule goes in its concern's chapter, not the stage chapter** — the EvidenceIR
  chapter breached its ceiling twice in three days (`LIVE-DOCUMENT-PRESSURE-HEADROOM` `.3`/`.19`/`.20`). **The doctrine driver runs no cargo
  gate** (`[[doctrine-driver-runs-no-cargo-gate]]`). Never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`).
