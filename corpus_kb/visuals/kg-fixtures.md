# Visual Evidence Fixture Patterns

This page records visual and VLM-related KG fixture coverage from the tracked truthfulness benchmark suite.
It is derived from tracked KG fixture outcomes and remains reviewable synthesis, not canonical document truth.

## Human Synthesis

Use this section for curated notes about visual grounding, VLM timing/state-machine extraction, and multimodal conflict patterns.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed Fixture Projection

<!-- corpus_kb_kg_fixture_family:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- source: `kg-bench fixtures`
- selected_structural_capabilities: `multimodal visual grounding`
- fixtures_total: `26`
- fixtures_passed: `26`
- fixtures_failed: `0`

| fixture | status | matched capabilities | path |
| --- | --- | --- | --- |
| `cross_modality_semantic_conflict_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/cross_modality_semantic_conflict_negative/fixture.json` |
| `cross_modality_semantic_grounding_gold` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/cross_modality_semantic_grounding_gold/fixture.json` |
| `evidence_missing_vlm_observations_surface_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/evidence_missing_vlm_observations_surface_negative/fixture.json` |
| `negative_knowledge_prior_guided_semantic_conflict_caution_gold` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_semantic_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_pattern_mismatch_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_pattern_mismatch_negative/fixture.json` |
| `source_vlm_enrichment_surface_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/source_vlm_enrichment_surface_negative/fixture.json` |
| `visual_motif_prior_caption_mismatch_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/visual_motif_prior_caption_mismatch_negative/fixture.json` |
| `visual_motif_prior_guided_diagram_classification_gold` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_gold/fixture.json` |
| `visual_motif_prior_guided_diagram_classification_without_prior_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_without_prior_negative/fixture.json` |
| `visual_semantic_prior_guided_caption_gold` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_gold/fixture.json` |
| `visual_semantic_prior_guided_caption_without_prior_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_without_prior_negative/fixture.json` |
| `visual_sources_semantic_conflict_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/visual_sources_semantic_conflict_negative/fixture.json` |
| `vlm_state_machine_duplicate_initial_gold` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_state_machine_duplicate_initial_gold/fixture.json` |
| `vlm_state_machine_label_noise_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_state_machine_label_noise_negative/fixture.json` |
| `vlm_state_machine_missing_initial_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_state_machine_missing_initial_negative/fixture.json` |
| `vlm_state_machine_multiple_initial_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_state_machine_multiple_initial_negative/fixture.json` |
| `vlm_state_machine_undeclared_transition_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_state_machine_undeclared_transition_negative/fixture.json` |
| `vlm_timing_active_low_assertion_equivalence_gold` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_assertion_equivalence_gold/fixture.json` |
| `vlm_timing_active_low_deassertion_equivalence_gold` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_deassertion_equivalence_gold/fixture.json` |
| `vlm_timing_cycle_qualified_signal_value_annotation_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_timing_cycle_qualified_signal_value_annotation_negative/fixture.json` |
| `vlm_timing_indexed_signal_value_annotation_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_timing_indexed_signal_value_annotation_negative/fixture.json` |
| `vlm_timing_motion_annotation_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_timing_motion_annotation_negative/fixture.json` |
| `vlm_timing_name_only_semantic_noise_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_timing_name_only_semantic_noise_negative/fixture.json` |
| `vlm_timing_semantic_grounding_gold` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_timing_semantic_grounding_gold/fixture.json` |
| `vlm_timing_spurious_annotation_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_timing_spurious_annotation_negative/fixture.json` |
| `vlm_timing_waveform_motion_negative` | `pass` | `multimodal visual grounding` | `crates/specforge/test_data/kg_quality/vlm_timing_waveform_motion_negative/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
