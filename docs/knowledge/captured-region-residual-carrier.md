---
id: captured-region-residual-carrier
title: Captured visual regions that reach no carrier become typed SemanticIR/IntentIR residuals
answers:
  - "what happens to a figure that reaches no canonical carrier"
  - "where does SpecForge record a captured region that produced no fact"
  - "what is captured_region_residuals"
  - "what is CapturedRegionResidualRecord"
  - "what does no_canonical_carrier_for_captured_region mean"
  - "how is a captured visual region proven covered or uncovered"
  - "why are table regions excluded from captured-region residuals"
  - "why is a figure caption not coverage for the figure"
  - "does a figure that produced waveform contracts get a residual"
  - "what is figure_region_provenance_id for"
  - "how many production rules does the registry declare"
  - "how many public fields do SemanticIR and IntentIR declare"
  - "which claim family owns captured_region_residuals"
  - "is captured_region_residuals an exact IntentIR carry"
  - "how many captured figure regions does the retained corpus hold"
date: 2026-08-27
status: current
tags: [spec-to-intent-alignment, residual, semantic-ir, intent-ir, visual-evidence, proof-rules, genericity]
evidence: crates/specforge/src/ir/source.rs (CapturedRegionResidualRecord); crates/specforge/src/ir/semantic.rs (unexplained_captured_visual_regions, cited_provenance_ids); crates/specforge/src/ir/waveform.rs (figure_region_provenance_id); doctrine/production_genericity/claim_family_inventory.tsv; doctrine/spec_to_intent/residual_actionability_contract.json; docs/tasks/spec-to-intent-alignment/residual-actionability.md (.8c)
reverify: "Run `cargo test -p specforge-core --lib ir::semantic::tests::captured` and `ir::semantic::tests::a_captured`, `bash scripts/check_production_genericity.sh`, and `python3 -B scripts/validate_residual_actionability_contract.py --self-test`; then rebuild any retained chain with `specforge semantic` and confirm its `captured_region_residuals` count equals its captured non-table, non-unknown visual-evidence count."
---

**Established `2026-08-27` (`SPEC-TO-INTENT-ALIGNMENT.8c`).** `residual_decisions` explains a record the
pipeline *refused*. It cannot explain a source region the pipeline never turned into a record, which is the
harder silence to notice because nothing remains in the artifact. `SemanticIr.captured_region_residuals` closes
that: every captured visual region that no canonical `SemanticIR` record cites earns exactly one
`CapturedRegionResidualRecord`, and `IntentIr` carries the collection unchanged.

The record lives in `crates/specforge/src/ir/source.rs` beside `TimingIntentDisposition::NonApplicable`, the
only other typed residual carrier, and holds `region_id` (the visual asset id), the typed `region_kind`,
`supporting_evidence_ids` (the `EvidenceIR` id, never empty), the closed cause
`no_canonical_carrier_for_captured_region`, a `reason`, the boundary `evidence_to_semantic_ir`, and an operator
`replay`. It asserts only the *absence* of a carrier and never states what the region said, so it cannot become
a fabricated fact under another name.

Coverage is a membership test over concrete record collections, not a text scan. `SemanticIr::cited_provenance_ids`
gathers provenance from `timing_constraints`, `signal_constraints`, `conditional_rules`, `regular_states`,
`state_transitions`, `temporal_rules`, `temporal_conflicts`, and `actor_contracts` — exactly the surfaces a
visual region can reach. Two citation forms count, because two production paths lead from a figure to a carrier:
`extract_records_from_vlm_observations` threads the item's `evidence_id`, while `mine_verified_figure_contracts`
cites `figure:<asset_id>`. `crate::ir::waveform::figure_region_provenance_id` is the single construction site for
the second form, so producer and accounting cannot drift.

Statement-mediated links are deliberately **not** coverage. `EvidenceIR` relates a statement to a visual region
when that statement *is* the region's caption, and a caption reaching a canonical carrier says nothing about the
region's content. The measurement is decisive: admitting caption links would mark 466 of the retained corpus's
1,089 captured figure-kind regions as explained, including the reviewed `picture_0001` in
`den0068_2018_07_23_coresight_base_system_architecture`, whose only mediating statement is the literal caption
`Figure 1: Example 1, with a shared ETB`.

`VisualAssetKind::TableRegion` is excluded because a table region already reaches canonical carriers through the
register, signal, and timing paths, so a residual there would duplicate a promoted key — the self-contradiction
the residual contract forbids. `VisualAssetKind::Unknown` is excluded for the reason the table-side sibling
`completeness::unexplained_intent_bearing_tables` skips unclassified table kinds: capture never established the
region as intent-bearing. The match is exhaustive, so a new visual kind cannot join either side silently.

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
