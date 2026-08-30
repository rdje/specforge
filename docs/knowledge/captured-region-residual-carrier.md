---
id: captured-region-residual-carrier
title: Captured regions that reach no carrier become typed SemanticIR/IntentIR residuals
answers:
  - "what happens to a figure that reaches no canonical carrier"
  - "where does SpecForge record a captured region that produced no fact"
  - "what is captured_region_residuals"
  - "what is CapturedRegionResidualRecord"
  - "what does no_canonical_carrier_for_captured_region mean"
  - "how is a captured visual region proven covered or uncovered"
  - "why are table regions no longer excluded from captured-region residuals"
  - "how is a captured table region proven covered or uncovered"
  - "what is cited_table_ids"
  - "which SemanticIR fields carry supporting_table_ids"
  - "why does a table-region residual carry two supporting evidence ids"
  - "why is a figure caption not coverage for the figure"
  - "does a figure that produced waveform contracts get a residual"
  - "what is figure_region_provenance_id for"
  - "how many production rules does the registry declare"
  - "how many public fields do SemanticIR and IntentIR declare"
  - "which claim family owns captured_region_residuals"
  - "is captured_region_residuals an exact IntentIR carry"
  - "how many captured figure regions does the retained corpus hold"
date: 2026-08-30
status: current
tags: [spec-to-intent-alignment, residual, semantic-ir, intent-ir, visual-evidence, proof-rules, genericity]
evidence: crates/specforge/src/ir/source.rs (CapturedRegionResidualRecord); crates/specforge/src/ir/semantic.rs (unexplained_captured_regions, captured_region_is_explained, captured_region_evidence_ids, cited_provenance_ids, cited_table_ids); crates/specforge/src/ir/waveform.rs (figure_region_provenance_id); doctrine/production_genericity/claim_family_inventory.tsv; doctrine/spec_to_intent/residual_actionability_contract.json; docs/tasks/spec-to-intent-alignment/residual-actionability.md (.8c); docs/tasks/spec-to-intent-alignment/region-kind-generalisation.md (.9b)
reverify: "Run `cargo test -p specforge-core --lib -- ir::semantic::tests::captured ir::semantic::tests::a_captured ir::semantic::tests::cited_table`, `bash scripts/check_production_genericity.sh`, and `python3 -B scripts/validate_residual_actionability_contract.py --self-test`; then rebuild any retained chain with `specforge semantic` and confirm its `captured_region_residuals` count equals its captured non-unknown visual-evidence count minus the table regions any record cites through `supporting_table_ids`. For the 110/466 counterfactual, run `python3 scripts/measure_caption_mediated_coverage.py`, which re-derives both readings and the CoreSight membership over the retained chains."
---

**Established `2026-08-27` (`SPEC-TO-INTENT-ALIGNMENT.8c`); widened to table regions `2026-08-30` (`.9b`).**
`residual_decisions` explains a record the pipeline *refused*. It cannot explain a source region the pipeline
never turned into a record, which is the harder silence to notice because nothing remains in the artifact.
`SemanticIr.captured_region_residuals` closes that: every captured region that no canonical `SemanticIR` record
cites earns exactly one `CapturedRegionResidualRecord`, and `IntentIr` carries the collection unchanged.

The record lives in `crates/specforge/src/ir/source.rs` beside `TimingIntentDisposition::NonApplicable`, the
only other typed residual carrier, and holds `region_id` (the visual asset id), the typed `region_kind`,
`supporting_evidence_ids` (never empty), the closed cause `no_canonical_carrier_for_captured_region`, a
`reason`, the boundary `evidence_to_semantic_ir`, and an operator `replay`. It asserts only the *absence* of a
carrier and never states what the region said, so it cannot become a fabricated fact under another name.

Coverage is a membership test over concrete record collections, not a text scan, and each region kind is asked
in the provenance vocabulary its own records use. `SemanticIr::cited_provenance_ids` answers for a **visual**
region, gathering provenance from `timing_constraints`, `signal_constraints`, `conditional_rules`,
`regular_states`, `state_transitions`, `temporal_rules`, `temporal_conflicts`, and `actor_contracts` — exactly
the surfaces a visual region can reach. Two citation forms count, because two production paths lead from a
figure to a carrier: `extract_records_from_vlm_observations` threads the item's `evidence_id`, while
`mine_verified_figure_contracts` cites `figure:<asset_id>`. `crate::ir::waveform::figure_region_provenance_id`
is the single construction site for the second form, so producer and accounting cannot drift.

`SemanticIr::cited_table_ids` answers for a **table** region, and it had to exist before one could join the
accounting at all. A table-derived record never names the region's `evidence_id` or its `figure:` form; it cites
the table through `supporting_table_ids`. Measured against the persisted corpus, the visual vocabulary reports
*every* captured table region as unexplained — 354 of 354 in the AMD IOMMU chain, 210 of 210 in the Arm Debug
chain — including the tables that produced those documents' registers and timing, so reusing it would have
emitted residuals duplicating promoted keys. The gatherer reads every `SemanticIr` surface that declares
`supporting_table_ids`: `register_records`, `timing_constraints`, `signal_polarities`, the observations of
`signal_polarity_conflicts` and `signal_semantic_conflicts`, and `interfaces[].signal_records` with their
`semantic_observations`. (`MessageFieldRecord` and `SignalSemanticHintRecord` declare the field too but are
`EvidenceIR`-only.) Over-inclusion is the safe direction — an extra surface can only remove a residual — and
`cited_table_ids_gathers_every_declared_table_provenance_surface` cross-checks the typed gatherer against the
artifact's own serialized table provenance so a surface added later cannot silently miss it. Asked this way the
current Arm Debug chain resolves to 16 cited and 194 unexplained of 210.

A table region's `supporting_evidence_ids` carries **both** identities under which the artifact refers to it —
`visual_NNNN`, the visual-evidence record that captured the rendered region, and `table_NNNN`, the id every
table-derived record uses. Both are real; a residual naming only one could not be joined to the other half of
the artifact.

Statement-mediated links are deliberately **not** coverage. `EvidenceIR` relates a statement to a visual region
when that statement *is* the region's caption, and a caption reaching a canonical carrier says nothing about the
region's content. How much that would explain depends on how far the mediation is taken. Over the retained
corpus's 1,089 captured figure-kind regions it explains **110** when applied only to the collections coverage
already reads, and **466** when applied to every SemanticIR collection carrying `supporting_statement_ids` —
the shape a naive implementation takes. The reviewed `picture_0001` in
`den0068_2018_07_23_coresight_base_system_architecture` appears **only in the second set**: its one mediating
statement is the literal caption `Figure 1: Example 1, with a shared ETB`, reached through the statement-lift
collections rather than the grounded-projection ones. The looser reading is therefore the one that would report
success on a cell the review requires to fail, which is why neither reading is coverage.

`VisualAssetKind::Unknown` is the only excluded kind, for the reason the validation-time sibling
`completeness::unexplained_intent_bearing_tables` skips unclassified table kinds: capture never established the
region as intent-bearing. The match is exhaustive, so a new visual kind cannot join either side silently.
`.8c` had also excluded `VisualAssetKind::TableRegion`, reasoning that a table region always reaches a register,
signal, or timing carrier; the measurement above refutes that, and the tables that *do* reach one are now
excluded by the table-provenance coverage test rather than by their kind.

Registration reused the existing residual families. `semantic.residual` and `intent.residual` each gained
`captured_region_residuals` in `top_level_fields`, taking the runtime registry from **168 to 170 rules** over
**50 public fields at SemanticIR and 50 at IntentIR**. The field is intentionally absent from
`INTENT_CARRIED_FIELDS`: `intent.residual` declares `symbol_capability: residual`, so its rule is `current_only`
and the current implementation must rebuild and compare the field rather than match an upstream claim — the
stricter obligation, even though the value is a byte-identical clone.

The frozen `.8a` contract records the shipped carrier and its checker proves it. Each typed cause's
`existing_carrier` must equal the closed expected value *and*, when non-null, resolve to a real declaration in
production source, so claiming coverage that was never built and deleting a carrier the contract still cites
both fail closed. The self-test is 24/24.
