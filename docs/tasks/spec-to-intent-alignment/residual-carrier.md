# SPEC-TO-INTENT-ALIGNMENT — residual carrier

- Part ID: `residual-carrier`
- State: `active`

This part owns the production carrier and the population replay — `SPEC-TO-INTENT-ALIGNMENT.8c` and `.8d`. The
`.8` localization, the frozen `.8a` rule, and the corrected `.8b` accounting that route here stay in
[residual actionability](residual-actionability.md); this part was split from it at that task boundary when the
combined part reached its declared line-count rollover milestone. `.9`'s generalisation of the same carrier to
the remaining captured region kinds was split out of *this* part at the `.8`/`.9` boundary for the same reason
and lives in [region-kind generalisation](region-kind-generalisation.md).

## Owned leaves

- ID: `SPEC-TO-INTENT-ALIGNMENT.8c`
  State: `done`
  Goal: emit the bounded typed, source-linked, actionable residual family across SemanticIR and IntentIR
  Acceptance: a structurally gated producer emits one typed residual for a captured source region of the
  selected family that reaches no canonical carrier, carrying its exact region and evidence provenance, typed
  cause, first failing stage, and operator replay route; authority is document structure and closed grammar
  only, never document, vendor, protocol, or review-label identity; a region with a canonical carrier emits no
  residual, and no residual duplicates a promoted canonical key; focused positive/refusal tests, the full core
  suite, warning-denied Clippy, and all production-genericity components pass; every proof-affected retained
  chain is rebuilt to zero stale and every public field change is exactly an intended new residual record, with
  no other field moving
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.8b`
  Verification: `the producer emits one typed residual per captured figure-kind region no canonical record
  cites; the reviewed CoreSight chain carries region picture_0001 with provenance visual_0008, cause
  no_canonical_carrier_for_captured_region, boundary evidence_to_semantic_ir, and an operator replay route.
  Two independent pre-change chains compared field-by-field show captured_region_residuals as the only
  differing public field at both SemanticIR and IntentIR (I2S 20 records, I2C 103 records); all 24 retained
  chains rebuild to 24 current / zero stale at all four stages. Four focused positive/refusal tests, the
  50/50-field proof coverage tests, the 170-rule inventory join, the workspace suite, warning-denied Clippy,
  all five genericity components, and the 24/24 contract self-test pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.8c — emit the captured-region residual carrier`

- ID: `SPEC-TO-INTENT-ALIGNMENT.8d`
  State: `done`
  Goal: replay the complete reviewed population and publish comparable residual-actionability closure
  Acceptance: all 12 reviewed sources and all 48 isolated stages replay from clean production under the frozen
  oracle; exact canonical, provenance, conservation, residual, disposition, category, and controller deltas are
  attributed; no reviewed canonical true positive is lost and no fabrication or unexplained drop appears;
  tracked replay, result, controller-input, and report authorities reproduce byte-for-byte; selected CI, mdBook,
  retrieval truth, task parents, cleanup, and residue census agree
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.8c`
  Prerequisite-input: `the owner supplied all eight authorized external read-only sources on 2026-08-27; every
  one is digest-identical to the reviewed lock and the orchestrator's map, basename, volume, and coverage
  preconditions pass`
  Verification: `all 12 reviewed sources replayed through all 48 isolated stages from clean production at
  483e525d; residual actionability moves 4/16 to 8/16, disposition 8/14 to 10/14, modality accounting 6/12 to
  8/12, and provenance closure 43/43 to 45/45, while conservation stays 120/120, IntentIR stays 40/0/0, and
  fabrication and unexplained drops stay zero. platform-system-ip becomes the third supported category. A
  control leg re-projecting the same replayed artifacts with the frozen pre-change builder (edf0a871)
  reproduces the published 4/16 baseline exactly, so the whole measured delta is attributable to the projection
  and .8c disturbed no reviewed metric. The replay surfaced one unrelated regression: the Cortex-A76 reviewed
  prose region is no longer at elem_00219, so exact source regions are 13/14 (routed to
  SOURCE-IR-REPRODUCIBILITY). Contract self-test 28/28, workspace suite 470/168/1,369/4, Clippy clean, all nine
  gate-tier doctrines pass, and 3,095 files / 1,158,476 KiB of population scratch plus 4,192 files /
  1,903,200 KiB of diagnostic scratch are removed with an empty residue census`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.8d — publish the reviewed population residual closure`

## Reviewed-source availability (`.8d`)

`crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json` records the
provenance of every reviewed source. Four are repository sources under `corpus/`: APB, I2S, AMD IOMMU, and Arm
Debug. The other eight are `external_read_only` inputs whose only tracked identity is a portable id and digest;
their working copies lived under the `.7c.ii` replay root and were removed with it.

The owner supplied all eight on `2026-08-27` from the sibling `chipdoc` repository on the same filesystem
volume. Each was located by exact SHA-256 rather than by filename, copied into the repository-derived,
git-ignored path `.project-data/tmp/spec-to-intent-external-sources/`, and re-verified after the copy. All eight
digests and byte counts match the reviewed lock exactly:

| Portable id | Bytes | SHA-256 (reviewed = copied) |
| --- | ---: | --- |
| `DDI0471_A_2011-06-23_GIC_400_Technical_Reference_Manual.pdf` | 557,071 | `afcac68f…d7ec` |
| `DEN0068_2018-07-23_CoreSight_Base_System_Architecture.pdf` | 165,891 | `c4a5f342…a66d` |
| `1.0.1_2026-02-22_RISC_V_IOMMU_Architecture_Specification.pdf` | 1,051,915 | `be2134b4…2b8e` |
| `1.0_2025-03-12_RISC_V_Advanced_Interrupt_Architecture.pdf` | 827,669 | `2d359579…c7a8` |
| `OpenCAPI-25Gbps_PHY_Signaling_Spec_1.0.pdf` | 748,724 | `0e0c8afc…ce12` |
| `OpenCAPI-4.0-32G_PHY_Signal_Spec_1.0_16NOV2020.pdf` | 855,025 | `d3eb19fc…38be` |
| `PJDOC-466751330-7215_10.0_Cortex_A76_Software_Optimization_Guide.pdf` | 637,434 | `8358c5ae…3a22` |
| `198123_0302_03_2025-04-22_Generic_Interrupt_Controller_Overview_Guide.pdf` | 1,571,128 | `5358701e…7e97` |

The runtime map is `.project-data/tmp/spec-to-intent-8d-external-source-map.json`. Executing the orchestrator's
own predicates against it — repository-relative map path below `.project-data/tmp`, absolute source paths,
basename equal to the portable id, same device as the repository root, and exact coverage of the required
external set — passed for all eight with no missing and no extra entry. `.8d` consumed exactly that map; the
map itself is removed with the population scratch, and regenerating it from the retained sources is the only
step a re-run needs.

The sources stay git-ignored rather than tracked under `corpus/`. The reviewed dataset is review-locked and
classifies these eight as `external_read_only`; promoting them to repository sources would change frozen
selection authority, the source-PDF registry, and every digest pinned to that dataset. The table above plus the
sibling repository keep the copy reproducible, so the artifact-cleanup doctrine can still reclaim the bytes
without losing the ability to replay.

The `.8c` chain-currency baseline is green at this boundary: 24 replayed / 24 current / zero stale at
EvidenceIR, SemanticIR, IntentIR, and the ISF adapter, with 54 explicitly unmeasurable legacy chains, 24
blocked/no-file adapter states, and exactly the declared retained bundle set on disk.

## Frozen carrier design (`.8c`, before implementation)

The structural gate is the figure-side sibling of the existing table-side region accounting in
`crates/specforge/src/ir/completeness.rs#unexplained_intent_bearing_tables`: coverage is resolved through
existing provenance and nothing is fabricated. A captured `VisualEvidenceItem` whose `asset_kind` is a
figure-kind region, and whose `evidence_id` no SemanticIR record cites in its provenance, reaches no canonical
carrier and earns exactly one typed residual. Table-kind visual assets are excluded because they already have
canonical carriers through the register, signal, and timing paths. `extract_records_from_vlm_observations`
threads `visual_item.evidence_id` into every record it projects, so coverage is a provenance membership test
over concrete collections rather than a text scan.

The record is a new shared type carrying `region_id` (the item's `asset_id`), `region_kind`,
`supporting_evidence_ids` (the item's `evidence_id`), the typed cause
`no_canonical_carrier_for_captured_region`, a `reason`, the boundary `evidence_to_semantic_ir`, and an operator
`replay` route. It lands on `SemanticIr` and is carried unchanged to `IntentIr`, matching the frozen `.8a`
record grammar.

Registration is data-driven and already has a home. `semantic.residual` and `intent.residual` exist in
`doctrine/production_genericity/claim_family_inventory.tsv` with `symbol_capability: residual` and
`alpha_obligation: residual_topology_invariant`. Adding the new field to both families' `top_level_fields` and
to the `SEMANTIC_RULE_FIELDS` / intent field tables expands the registry from 168 to 170 field rules; the
inventory-bound qualification test in `crates/specforge/src/ir/mod.rs` and the proof-context `insert_field!`
sites move with it.

A real reviewed document already witnesses the family without any external source. `den0068_2018_07_23_coresight_base_system_architecture`
is one of the 24 retained measurable chains, and its persisted EvidenceIR holds 19 visual items: 12 table
regions, all of whose asset ids SemanticIR cites, and seven figure regions, none of whose evidence ids SemanticIR
cites anywhere. The first of those seven is `visual_0008` on `picture_0001` — exactly the region, evidence id,
and fact key the review expects for the selected family. The RISC-V IOMMU document is not in the retained set,
so its chain stays explicitly unmeasurable until the external source returns.

## Shipped captured-region carrier (`.8c`)

The frozen design is implemented as designed, with one correction found while reading the producers it depends
on. `CapturedRegionResidualRecord` lives in `crates/specforge/src/ir/source.rs` beside
`TimingIntentDisposition::NonApplicable` and carries `region_id` (the visual asset id), the typed
`region_kind`, `supporting_evidence_ids` (the `EvidenceIR` id), the closed cause
`no_canonical_carrier_for_captured_region`, a `reason`, the boundary `evidence_to_semantic_ir`, and an operator
`replay`. `SemanticIr::captured_region_residuals` is produced last, over the assembled artifact, and
`IntentIr` carries it unchanged.

The correction is to *coverage*, and it only removes false positives. The frozen design tested membership of the
item's `evidence_id` alone, but `mine_verified_figure_contracts` is a second production path from a figure to a
canonical carrier, and it cites the region as `figure:<asset_id>` rather than by evidence id. A figure that
produced verified waveform contracts would therefore have been given a residual asserting it reached no carrier
— a false statement, and exactly what the contract's `no_fabrication_rule` forbids. Coverage now accepts either
citation form, and `crate::ir::waveform::figure_region_provenance_id` is the single construction site both the
producer and the accounting read, so the two cannot drift.

Coverage is a membership test over the artifact's own record collections — `timing_constraints`,
`signal_constraints`, `conditional_rules`, `regular_states`, `state_transitions`, `temporal_rules`,
`temporal_conflicts`, and `actor_contracts` — never a text scan. Statement-mediated links are deliberately
excluded: `EvidenceIR` relates a statement to a visual region when that statement *is* the region's caption, and
a caption reaching a canonical carrier says nothing about the region's content. The measurement is decisive
rather than stylistic, and its size depends on how far the mediation is taken. Over the retained corpus's 1,089
captured figure-kind regions, admitting statement-mediated coverage explains 110 of them when the mediation is
applied only to the collections above, and 466 when it is applied to every SemanticIR collection carrying
`supporting_statement_ids` — the shape a naive implementation takes. The reviewed `picture_0001` is in the
second set and not the first: its only mediating statement is the literal caption
`Figure 1: Example 1, with a shared ETB`, cited through the statement-lift collections rather than the
grounded-projection ones. The looser reading is therefore the one that would have reported success on a cell
the review requires to fail, which is why neither reading is coverage.

`TableRegion` is excluded because it already reaches canonical carriers through the register, signal, and timing
paths; a residual there would duplicate a promoted key. `Unknown` is excluded for the reason the table-side
sibling skips unclassified table kinds: capture never established the region as intent-bearing. The match over
`VisualAssetKind` is exhaustive, so a new visual kind cannot join either side silently.

Registration landed where the design placed it. `semantic.residual` and `intent.residual` each gained
`captured_region_residuals` in `top_level_fields`, the `SEMANTIC_RULE_FIELDS` / `INTENT_RULE_FIELDS` tables and
both `insert_field!` projections gained the field, and the registry expanded from 168 to 170 field rules — 50
public fields at SemanticIR and 50 at IntentIR. The field is deliberately *not* in `INTENT_CARRIED_FIELDS`:
`intent.residual` declares `symbol_capability: residual`, so its rule is `current_only` and must be rebuilt and
compared rather than matched against an upstream claim. That is the stricter of the two obligations.

The frozen contract was updated in one line, because leaving it unchanged would have made it false.
`no_canonical_carrier_for_captured_region` now declares `existing_carrier: CapturedRegionResidualRecord` instead
of `null`. The checker no longer trusts that declaration: a named carrier must resolve to an actual production
declaration, so claiming coverage that does not exist and deleting a carrier the contract still cites both fail
closed. Three new RED cases cover claiming a carrier for a still-unbuilt cause, renaming the shipped carrier,
and disowning it; the self-test is now 24/24.

The public delta is exactly the intended record and nothing else. Two pre-change chains were snapshotted before
the rebuild and compared field-by-field against their rebuilds: I2S is 37 SemanticIR / 40 IntentIR public fields
with `captured_region_residuals(absent->20)` as the only difference, and I2C is 40 / 42 with
`captured_region_residuals(absent->103)` as the only difference. No other field moved at either stage.

Across the whole retained population the carrier emits 1,089 records at SemanticIR and the same 1,089 at
IntentIR — exactly the 1,089 captured `figure` and `diagram` regions the 24 chains hold, and none of the 906
`table_region` ones. The 24 final adapter ledgers still share one ruleset, now carry 150,942 cumulative claims
across the same 120 artifacts, and reference all 170 rule ids; `residual_decisions` stays 0 / 3 / 8 / 62 across
Source, Semantic, Intent, and adapter, and all 24 adapters remain blocked with zero emitted `.isf` files.

The claim total moved from the `2026-08-14` structural-qualification figure of 148,708 by +2,234, and the
attribution is exact rather than assumed. This leaf contributes 24 field-root plus 1,089 record claims at each
of SemanticIR and IntentIR, so +2,226. The residual +8 net is pre-existing drift the intervening `.7b`/`.7c.i`
slices left unrepublished: +8 at EvidenceIR, carried as +8 through SemanticIR and +8 through IntentIR, against
−16 at the adapter. Stage-local totals are therefore 31,382 / 89,766 / 13,520 / 15,409 / 865.

## Published population closure (`.8d`)

The complete reviewed population replayed from clean production at `483e525d` — all 12 sources, all 48 isolated
stages — under unchanged reviewed authority: the reviewed dataset (`c743bcda`), prior memory (`a416cc8b`), and
orchestrator (`55c81520`) digests are identical to the `.7c.ii` run, and every source is digest-equal to the
reviewed lock. The published result is the replay's own evaluator output, byte for byte; nothing was
re-summarized.

| Dimension | `.7c.ii` / `.8b` | `.8d` |
| --- | --- | --- |
| residual actionability | 4 / 16 | **8 / 16** |
| source-region disposition | 8 / 14 | **10 / 14** |
| required-modality accounting | 6 / 12 | **8 / 12** |
| canonical provenance closure | 43 / 43 | **45 / 45** |
| stage conservation or residual | 120 / 120 | 120 / 120 |
| IntentIR canonical TP/FP/FN | 40 / 0 / 0 | 40 / 0 / 0 |
| fabricated facts, unexplained drops | 0, 0 | 0, 0 |
| supported categories | 2 / 6 | **3 / 6** |
| exact source regions | 14 / 14 | **13 / 14** |

`platform-system-ip` becomes the third supported category, exactly as the `.8c` family selection predicted: both
its documents lose every hard failure once their captured figures earn an actionable residual.

The delta is attributed rather than assumed. A control leg re-projected the *same* replayed stage artifacts with
the frozen pre-change builder (`edf0a871`, the digest `.7c.ii` pinned) and reproduced the published baseline
exactly — 4/16, 8/14, 6/12, 43/43, 120/120, two supported categories. So the `.8c` production carrier moved no
reviewed metric, and every difference above is attributable to the `.8d` projection. Cell-level comparison
confirms it: exactly two cells changed, both `static_component_topology`, each from 0/1 with no actionable
record to 1/1 exact, provenanced, and actionable at SemanticIR and IntentIR. No canonical score, boundary score,
category other than `platform-system-ip`, or other document moved.

### Reviewed anchor drift found by this replay (`.8d`)

The control leg also surfaced one regression that has nothing to do with this leaf's change. The Cortex-A76
guide's reviewed prose region is anchored on the *ordinal* id `elem_00219`, and the current binary places that
prose at `elem_00230`: ingest now emits 260 content elements where the persisted chain holds 249, an
11-element shift with identical table (68), visual (71), and page (46) counts. The reviewed cell therefore
gains `source_region_missing_or_ambiguous` and exact source regions fall to 13/14. It is reported, not
absorbed: no hard gate regressed, and the fixture failed closed rather than matching the wrong element.

The cause is not this repository's code. The same PDF bytes (`8358c5ae`, unchanged since April), the same
Docling 2.84.0, and the same model blobs produce 260 elements at both `483e525d` and `3d04bde0` — the commit
*before* the only revision in range that touched `docling_backend.rs` — batched and unbatched alike, and two
back-to-back runs agree byte for byte after normalizing the replay root. The persisted 249-element chain was
built from a normalized bundle cached before that, so SourceIR ingest is not reproducible across time in this
environment. Nothing gates it: `scripts/check_chain_currency.sh` replays evidence through the adapter from the
*persisted* `source_ir.json` and never re-runs ingest, so a 24/24 current chain says nothing about SourceIR.
[`SOURCE-IR-REPRODUCIBILITY`](../SOURCE-IR-REPRODUCIBILITY.md) owns both the reproducibility gap and the
ordinal-anchor fragility that turns it into a reviewed-cell failure.

### Scratch reclaimed (`.8d`)

The population root and its runtime source map are removed with an empty residue census: 3,095 files /
1,158,476 KiB. The diagnostic roots this leaf created to attribute the delta — the control projection, five
ingest-determinism replays, and one detached worktree at `3d04bde0` — are removed too: 4,192 files /
1,903,200 KiB, also residue-free. The eight owner-supplied external sources are deliberately retained under
`.project-data/tmp/spec-to-intent-external-sources/`: they are 6.3 MiB, git-ignored, digest-pinned in the table
above, and they are what makes this leaf's own measurement re-runnable.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.8d`

- [x] **REPRODUCE / MEASURE** — the tracked `.7c.ii`/`.8b` authority published residual actionability 4/16,
  disposition 8/14, modality accounting 6/12, and provenance 43/43, with `platform-system-ip` incomplete on
  four hard failures. Both `static_component_topology` cells scored 0/1 at SemanticIR and IntentIR with no
  actionable record.
- [x] **ROOT CAUSE (WHY + WHERE)** — `project_residuals` in
  `crates/specforge/test_data/source_to_intent_vertical/build_fixture.py` returned `[]` for every projection
  except `physical_timing`, so `.8c`'s shipped `captured_region_residuals` reached the reviewed `/residuals`
  collection for no cell. The production carrier existed; nothing projected it into the measurement.
- [x] **ADDRESSED (verified)** — the projection now dispatches to one projector per production carrier, and
  `project_captured_regions` carries region identity, `EvidenceIR` provenance, and the three actionability
  fields verbatim. Measured over the fresh 12-source replay: 4/16 → 8/16, 8/14 → 10/14, 6/12 → 8/12,
  43/43 → 45/45, and `platform-system-ip` incomplete → supported.
- [x] **NO REGRESSION** — conservation stays 120/120, IntentIR stays 40/0/0, fabrication and unexplained drops
  stay zero, and the control leg proves no non-projected metric moved. `cargo test --workspace --lib` is
  470 / 168 / 1,369 / 4 with zero failures; `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  is clean; the `.8a` contract self-test rejects 28/28 mutations; all nine gate-tier doctrines pass.
- [x] **GENERICITY (ADR 0006 / ADR 0037)** — the change is fixture-side only. The reviewed family label
  `static_component_topology` names the projected key inside the conformance fixture, exactly where `.8c`
  placed it; the projector gates on the reviewed region's structural kind and reads only `region_id`,
  `supporting_evidence_ids`, and the three actionability fields from the production record. No production
  source changed.
- [x] **LOCKSTEP** — the published result, replay evidence, controller input and report, the frozen `.8a`
  contract and its checker, the mdBook trajectory and contract chapters, this part, the bounded root, the
  resume pointer, the live status, and the Knowledge Map agree on 8/16 and on the three supported categories.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.8c`

- [x] **REPRODUCE / MEASURE** — before the change, the reviewed `den0068_2018_07_23_coresight_base_system_architecture`
  chain captured 19 visual items (12 `table_region`, seven `figure`) and its `SemanticIR` cited no visual
  evidence id anywhere in a public collection, so all seven figure regions reached no canonical carrier and left
  no record. Across the 24 retained chains, 1,089 captured figure-kind regions were in that state.
- [x] **ROOT CAUSE (WHY + WHERE)** — `SemanticIr::from_evidence_ir` in `crates/specforge/src/ir/semantic.rs`
  projected `EvidenceIR` visual evidence only through `extract_records_from_vlm_observations` and
  `mine_verified_figure_contracts`. A region those two paths produce nothing from simply stopped, because no
  collection on `SemanticIr` could hold "this region was captured and no carrier accepted it".
- [x] **ADDRESSED (verified)** — the shared `CapturedRegionResidualRecord`, the closed
  `CapturedRegionResidualCause` / `CapturedRegionBoundary` grammars, the structurally gated
  `unexplained_captured_visual_regions` producer (renamed to `unexplained_captured_regions` by `.9b`), the
  `cited_provenance_ids` membership test, the shared
  `figure_region_provenance_id`, and the new field on both `SemanticIr` and `IntentIr` ship together with their
  rule registrations. The rebuilt CoreSight chain carries `picture_0001` / `visual_0008` /
  `no_canonical_carrier_for_captured_region` / `evidence_to_semantic_ir` with a non-empty reason and replay.
- [x] **NO REGRESSION** — `cargo test --workspace --lib` is 470 / 168 / 1,373 / 4 passing with zero failures;
  `cargo clippy --workspace --all-targets --all-features -- -D warnings` is clean; all five
  production-genericity components pass; the inventory join is exactly 170 runtime rules; the `.8a` contract
  self-test rejects 24/24 mutations; the derived flow oracle is refreshed to 2,370 functions / 14,681 helper
  edges / 12,656 decision sites / 1,464 semantic macros with 120 rule roots.
- [x] **GENERICITY (ADR 0006 / ADR 0037)** — the gate reads only `VisualAssetKind`, `asset_id`, `evidence_id`,
  and record-level provenance lists. No document, vendor, protocol, family, or review label reaches production:
  the reviewed family name `static_component_topology` appears only in the conformance fixture that projects the
  record, never in the producer.
- [x] **LOCKSTEP** — the producer, the two claim-family inventories, the frozen contract and its checker, the
  mdBook SemanticIR/IntentIR/EvidenceIR/trajectory/contract chapters, this part, the bounded root, the resume
  pointer, and the live status agree that the carrier ships and that `.8d` is next.

## Update protocol

`.8c` and `.8d` are complete and closed, and `.9`'s leaves have moved to
[region-kind generalisation](region-kind-generalisation.md), so this part is closed to new leaves. A correction
to a published metric or route here updates the containment contract, index, and manifest in the same commit.
Decisions, verification, and commit records for the `.8` program stay in
[residual actionability](residual-actionability.md) so that program keeps one chronology.
