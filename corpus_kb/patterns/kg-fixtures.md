# Semantic And Truthfulness Fixture Patterns

This page records semantic arbitration, actor/connectivity, residual, caveat, negative-knowledge, and truthfulness-caution KG fixture coverage from the tracked truthfulness benchmark suite.
It is derived from tracked KG fixture outcomes and remains reviewable synthesis, not canonical document truth.

## Human Synthesis

Use this section for curated notes about semantic arbitration, graph/connectivity evidence, residual/caveat behavior, and false-positive control patterns.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed Fixture Projection

<!-- corpus_kb_kg_fixture_family:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- source: `kg-bench fixtures`
- selected_family_labels: `actor connectivity`, `semantic role arbitration`, `negative knowledge`, `truthfulness negatives and cautions`, `residuals and caveats`
- fixtures_total: `74`
- fixtures_passed: `74`
- fixtures_failed: `0`

| fixture | status | matched families | path |
| --- | --- | --- | --- |
| `actor_boundary_residual` | `pass` | `actor connectivity`, `residuals and caveats`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/actor_boundary_residual/fixture.json` |
| `actor_port_gap_surface_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/actor_port_gap_surface_negative/fixture.json` |
| `actor_ports_gold` | `pass` | `actor connectivity` | `crates/specforge/test_data/kg_quality/actor_ports_gold/fixture.json` |
| `actor_taxonomy_prior_guided_section_direction_gold` | `pass` | `actor connectivity` | `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json` |
| `actor_taxonomy_prior_guided_section_direction_without_prior_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_without_prior_negative/fixture.json` |
| `ahb_section_heading_direction_gold` | `pass` | `actor connectivity` | `crates/specforge/test_data/kg_quality/ahb_section_heading_direction_gold/fixture.json` |
| `alias_dependent_handshake_completion_caveat` | `pass` | `residuals and caveats`, `semantic role arbitration` | `crates/specforge/test_data/kg_quality/alias_dependent_handshake_completion_caveat/fixture.json` |
| `amba_destination_column_reads_gold` | `pass` | `actor connectivity` | `crates/specforge/test_data/kg_quality/amba_destination_column_reads_gold/fixture.json` |
| `amba_source_column_handshake_gold` | `pass` | `actor connectivity`, `semantic role arbitration` | `crates/specforge/test_data/kg_quality/amba_source_column_handshake_gold/fixture.json` |
| `apb_requester_completer_handshake_gold` | `pass` | `semantic role arbitration` | `crates/specforge/test_data/kg_quality/apb_requester_completer_handshake_gold/fixture.json` |
| `axi_width_only_prose_direction_gold` | `pass` | `actor connectivity` | `crates/specforge/test_data/kg_quality/axi_width_only_prose_direction_gold/fixture.json` |
| `clock_reset_contract_scope_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/clock_reset_contract_scope_negative/fixture.json` |
| `clock_reset_generic_advice_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/clock_reset_generic_advice_negative/fixture.json` |
| `compat_direction_hints_graph_conflict_incomplete_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/compat_direction_hints_graph_conflict_incomplete_negative/fixture.json` |
| `compat_direction_hints_incomplete_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/compat_direction_hints_incomplete_negative/fixture.json` |
| `compat_direction_hints_lag_graph_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/compat_direction_hints_lag_graph_negative/fixture.json` |
| `compat_direction_hints_mixed_lag_incomplete_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/compat_direction_hints_mixed_lag_incomplete_negative/fixture.json` |
| `connectivity_endpoint_gaps_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/connectivity_endpoint_gaps_negative/fixture.json` |
| `contested_handshake_name_fallback_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/contested_handshake_name_fallback_negative/fixture.json` |
| `control_polarity_conflict_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/control_polarity_conflict_negative/fixture.json` |
| `cross_modality_semantic_conflict_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/cross_modality_semantic_conflict_negative/fixture.json` |
| `cross_modality_semantic_grounding_gold` | `pass` | `semantic role arbitration` | `crates/specforge/test_data/kg_quality/cross_modality_semantic_grounding_gold/fixture.json` |
| `detached_mixed_control_polarity_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/detached_mixed_control_polarity_negative/fixture.json` |
| `evidence_missing_vlm_observations_surface_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/evidence_missing_vlm_observations_surface_negative/fixture.json` |
| `evidence_normative_residual_surface_negative` | `pass` | `residuals and caveats`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/evidence_normative_residual_surface_negative/fixture.json` |
| `evidence_structural_kg_missing_surface_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/evidence_structural_kg_missing_surface_negative/fixture.json` |
| `graph_direction_coverage_incomplete_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/graph_direction_coverage_incomplete_negative/fixture.json` |
| `graph_direction_same_actor_conflict_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/graph_direction_same_actor_conflict_negative/fixture.json` |
| `interface_signal_conflict_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/interface_signal_conflict_negative/fixture.json` |
| `multi_producer_conflict_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/multi_producer_conflict_negative/fixture.json` |
| `name_only_semantic_noise_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/name_only_semantic_noise_negative/fixture.json` |
| `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` | `pass` | `actor connectivity`, `negative knowledge`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_connectivity_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_interface_conflict_caution_gold` | `pass` | `negative knowledge`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_interface_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_polarity_conflict_caution_gold` | `pass` | `negative knowledge`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_polarity_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_residual_caution_gold` | `pass` | `negative knowledge`, `residuals and caveats`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_residual_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_semantic_conflict_caution_gold` | `pass` | `negative knowledge`, `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_semantic_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_temporal_conflict_caution_gold` | `pass` | `negative knowledge`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_temporal_conflict_caution_gold/fixture.json` |
| `relative_clause_actor_noise_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/relative_clause_actor_noise_negative/fixture.json` |
| `semantic_modality_reliability_prior_guided_conflict_gold` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_gold/fixture.json` |
| `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_without_prior_negative/fixture.json` |
| `semantic_prior_conflicting_roles_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/semantic_prior_conflicting_roles_negative/fixture.json` |
| `semantic_prior_guided_phrase_gold` | `pass` | `semantic role arbitration` | `crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_gold/fixture.json` |
| `semantic_prior_guided_phrase_without_prior_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_without_prior_negative/fixture.json` |
| `semantic_prior_phrase_mismatch_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/semantic_prior_phrase_mismatch_negative/fixture.json` |
| `semantic_prior_source_kind_mismatch_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/semantic_prior_source_kind_mismatch_negative/fixture.json` |
| `semantic_ready_sink_prior_guided_phrase_gold` | `pass` | `semantic role arbitration` | `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_gold/fixture.json` |
| `semantic_ready_sink_prior_guided_phrase_without_prior_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_without_prior_negative/fixture.json` |
| `semantic_valid_prior_guided_phrase_gold` | `pass` | `semantic role arbitration` | `crates/specforge/test_data/kg_quality/semantic_valid_prior_guided_phrase_gold/fixture.json` |
| `semantic_valid_prior_guided_phrase_without_prior_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/semantic_valid_prior_guided_phrase_without_prior_negative/fixture.json` |
| `signal_table_inventory_authority_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/signal_table_inventory_authority_negative/fixture.json` |
| `source_column_bogus_actor_attribution_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/source_column_bogus_actor_attribution_negative/fixture.json` |
| `source_vlm_enrichment_surface_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/source_vlm_enrichment_surface_negative/fixture.json` |
| `table_misclassification_field_table_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/table_misclassification_field_table_negative/fixture.json` |
| `table_shape_prior_guided_signal_table_without_prior_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_without_prior_negative/fixture.json` |
| `table_shape_prior_guided_timing_table_without_prior_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json` |
| `temporal_actor_grounding_surface_negative` | `pass` | `actor connectivity`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/temporal_actor_grounding_surface_negative/fixture.json` |
| `temporal_clock_grounding_surface_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/temporal_clock_grounding_surface_negative/fixture.json` |
| `temporal_conflict_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/temporal_conflict_negative/fixture.json` |
| `temporal_prior_guided_cycle_window_without_prior_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_without_prior_negative/fixture.json` |
| `visual_motif_prior_guided_diagram_classification_without_prior_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_without_prior_negative/fixture.json` |
| `visual_semantic_prior_guided_caption_gold` | `pass` | `semantic role arbitration` | `crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_gold/fixture.json` |
| `visual_semantic_prior_guided_caption_without_prior_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_without_prior_negative/fixture.json` |
| `visual_sources_semantic_conflict_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/visual_sources_semantic_conflict_negative/fixture.json` |
| `vlm_state_machine_label_noise_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_state_machine_label_noise_negative/fixture.json` |
| `vlm_state_machine_missing_initial_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_state_machine_missing_initial_negative/fixture.json` |
| `vlm_state_machine_multiple_initial_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_state_machine_multiple_initial_negative/fixture.json` |
| `vlm_state_machine_undeclared_transition_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_state_machine_undeclared_transition_negative/fixture.json` |
| `vlm_timing_cycle_qualified_signal_value_annotation_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_timing_cycle_qualified_signal_value_annotation_negative/fixture.json` |
| `vlm_timing_indexed_signal_value_annotation_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_timing_indexed_signal_value_annotation_negative/fixture.json` |
| `vlm_timing_motion_annotation_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_timing_motion_annotation_negative/fixture.json` |
| `vlm_timing_name_only_semantic_noise_negative` | `pass` | `semantic role arbitration`, `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_timing_name_only_semantic_noise_negative/fixture.json` |
| `vlm_timing_semantic_grounding_gold` | `pass` | `semantic role arbitration` | `crates/specforge/test_data/kg_quality/vlm_timing_semantic_grounding_gold/fixture.json` |
| `vlm_timing_spurious_annotation_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_timing_spurious_annotation_negative/fixture.json` |
| `vlm_timing_waveform_motion_negative` | `pass` | `truthfulness negatives and cautions` | `crates/specforge/test_data/kg_quality/vlm_timing_waveform_motion_negative/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
