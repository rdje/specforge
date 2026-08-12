# KG Benchmark Fixture Results

This page is the benchmark-result projection family in the `R15g` corpus knowledge base.
It records KG fixture outcomes from the tracked truthfulness benchmark suite without promoting benchmark behavior into canonical document truth.

## Human Synthesis

Use this section for curated notes about recurring benchmark patterns, fixture families, and candidate follow-up work.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed KG Benchmark Projection

<!-- corpus_kb_kg_fixtures:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- fixtures_total: `156`
- fixtures_passed: `156`
- fixtures_failed: `0`

### Structural Capability Summary
Fixtures can exercise more than one capability because typed evidence, modality, and expected behavior are orthogonal. Capabilities come from populated fixture-schema fields, never fixture names.

| capability | fixtures | passed | failed |
| --- | ---: | ---: | ---: |
| actor connectivity | `54` | `54` | `0` |
| infrastructure semantics | `11` | `11` | `0` |
| multimodal visual grounding | `26` | `26` | `0` |
| negative knowledge | `7` | `7` | `0` |
| polarity semantics | `8` | `8` | `0` |
| residuals and caveats | `4` | `4` | `0` |
| semantic role arbitration | `64` | `64` | `0` |
| state-machine semantics | `5` | `5` | `0` |
| table extraction and hygiene | `65` | `65` | `0` |
| temporal semantics | `79` | `79` | `0` |
| truthfulness negatives and cautions | `83` | `83` | `0` |
| typed prior memory | `30` | `30` | `0` |
| uncategorized | `2` | `2` | `0` |

### Fixture Results
| fixture | status | path |
| --- | --- | --- |
| `actor_boundary_residual` | `pass` | `crates/specforge/test_data/kg_quality/actor_boundary_residual/fixture.json` |
| `actor_port_gap_surface_negative` | `pass` | `crates/specforge/test_data/kg_quality/actor_port_gap_surface_negative/fixture.json` |
| `actor_ports_gold` | `pass` | `crates/specforge/test_data/kg_quality/actor_ports_gold/fixture.json` |
| `actor_taxonomy_prior_guided_section_direction_gold` | `pass` | `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json` |
| `actor_taxonomy_prior_guided_section_direction_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_without_prior_negative/fixture.json` |
| `actor_taxonomy_prior_term_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_term_mismatch_negative/fixture.json` |
| `ahb_control_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/ahb_control_stability_gold/fixture.json` |
| `ahb_exclusive_security_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/ahb_exclusive_security_stability_gold/fixture.json` |
| `ahb_response_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/ahb_response_stability_gold/fixture.json` |
| `ahb_section_heading_direction_gold` | `pass` | `crates/specforge/test_data/kg_quality/ahb_section_heading_direction_gold/fixture.json` |
| `ahb_transfer_lock_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/ahb_transfer_lock_stability_gold/fixture.json` |
| `ahb_wait_state_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/ahb_wait_state_timing_gold/fixture.json` |
| `ahb_write_data_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/ahb_write_data_stability_gold/fixture.json` |
| `alias_dependent_handshake_completion_caveat` | `pass` | `crates/specforge/test_data/kg_quality/alias_dependent_handshake_completion_caveat/fixture.json` |
| `amba_destination_column_reads_gold` | `pass` | `crates/specforge/test_data/kg_quality/amba_destination_column_reads_gold/fixture.json` |
| `amba_source_column_handshake_gold` | `pass` | `crates/specforge/test_data/kg_quality/amba_source_column_handshake_gold/fixture.json` |
| `apb_address_protection_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/apb_address_protection_stability_gold/fixture.json` |
| `apb_requester_completer_handshake_gold` | `pass` | `crates/specforge/test_data/kg_quality/apb_requester_completer_handshake_gold/fixture.json` |
| `apb_response_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/apb_response_stability_gold/fixture.json` |
| `apb_setup_access_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/apb_setup_access_timing_gold/fixture.json` |
| `apb_write_control_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/apb_write_control_stability_gold/fixture.json` |
| `axi_address_qos_region_sideband_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_address_qos_region_sideband_stability_gold/fixture.json` |
| `axi_address_response_user_sideband_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_address_response_user_sideband_stability_gold/fixture.json` |
| `axi_data_user_sideband_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_data_user_sideband_stability_gold/fixture.json` |
| `axi_next_cycle_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_next_cycle_timing_gold/fixture.json` |
| `axi_read_address_control_sideband_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_read_address_control_sideband_stability_gold/fixture.json` |
| `axi_read_address_id_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_read_address_id_stability_gold/fixture.json` |
| `axi_read_address_sideband_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_read_address_sideband_stability_gold/fixture.json` |
| `axi_read_address_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_read_address_timing_gold/fixture.json` |
| `axi_read_data_id_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_read_data_id_stability_gold/fixture.json` |
| `axi_read_data_last_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_read_data_last_stability_gold/fixture.json` |
| `axi_read_data_response_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_read_data_response_stability_gold/fixture.json` |
| `axi_read_data_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_read_data_timing_gold/fixture.json` |
| `axi_sideband_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_sideband_stability_gold/fixture.json` |
| `axi_width_only_prose_direction_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_width_only_prose_direction_gold/fixture.json` |
| `axi_write_address_control_sideband_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_write_address_control_sideband_stability_gold/fixture.json` |
| `axi_write_address_id_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_write_address_id_stability_gold/fixture.json` |
| `axi_write_address_sideband_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_write_address_sideband_stability_gold/fixture.json` |
| `axi_write_address_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_write_address_timing_gold/fixture.json` |
| `axi_write_data_last_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_write_data_last_stability_gold/fixture.json` |
| `axi_write_data_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_write_data_timing_gold/fixture.json` |
| `axi_write_response_id_stability_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_write_response_id_stability_gold/fixture.json` |
| `axi_write_response_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/axi_write_response_timing_gold/fixture.json` |
| `clock_edge_of_clock_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/clock_edge_of_clock_timing_gold/fixture.json` |
| `clock_reset_contract_scope_negative` | `pass` | `crates/specforge/test_data/kg_quality/clock_reset_contract_scope_negative/fixture.json` |
| `clock_reset_generic_advice_negative` | `pass` | `crates/specforge/test_data/kg_quality/clock_reset_generic_advice_negative/fixture.json` |
| `clock_reset_topology_gold` | `pass` | `crates/specforge/test_data/kg_quality/clock_reset_topology_gold/fixture.json` |
| `compat_direction_hints_graph_conflict_incomplete_negative` | `pass` | `crates/specforge/test_data/kg_quality/compat_direction_hints_graph_conflict_incomplete_negative/fixture.json` |
| `compat_direction_hints_incomplete_negative` | `pass` | `crates/specforge/test_data/kg_quality/compat_direction_hints_incomplete_negative/fixture.json` |
| `compat_direction_hints_lag_graph_negative` | `pass` | `crates/specforge/test_data/kg_quality/compat_direction_hints_lag_graph_negative/fixture.json` |
| `compat_direction_hints_mixed_lag_incomplete_negative` | `pass` | `crates/specforge/test_data/kg_quality/compat_direction_hints_mixed_lag_incomplete_negative/fixture.json` |
| `connectivity_endpoint_gaps_negative` | `pass` | `crates/specforge/test_data/kg_quality/connectivity_endpoint_gaps_negative/fixture.json` |
| `contested_handshake_name_fallback_negative` | `pass` | `crates/specforge/test_data/kg_quality/contested_handshake_name_fallback_negative/fixture.json` |
| `control_polarity_conflict_negative` | `pass` | `crates/specforge/test_data/kg_quality/control_polarity_conflict_negative/fixture.json` |
| `cross_modality_semantic_conflict_negative` | `pass` | `crates/specforge/test_data/kg_quality/cross_modality_semantic_conflict_negative/fixture.json` |
| `cross_modality_semantic_grounding_gold` | `pass` | `crates/specforge/test_data/kg_quality/cross_modality_semantic_grounding_gold/fixture.json` |
| `default_clock_explicit_next_edge_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/fixture.json` |
| `default_clock_later_edge_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/default_clock_later_edge_timing_gold/fixture.json` |
| `default_clock_quantified_edge_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/default_clock_quantified_edge_timing_gold/fixture.json` |
| `detached_mixed_control_polarity_negative` | `pass` | `crates/specforge/test_data/kg_quality/detached_mixed_control_polarity_negative/fixture.json` |
| `evidence_missing_vlm_observations_surface_negative` | `pass` | `crates/specforge/test_data/kg_quality/evidence_missing_vlm_observations_surface_negative/fixture.json` |
| `evidence_normative_residual_surface_negative` | `pass` | `crates/specforge/test_data/kg_quality/evidence_normative_residual_surface_negative/fixture.json` |
| `evidence_structural_kg_missing_surface_negative` | `pass` | `crates/specforge/test_data/kg_quality/evidence_structural_kg_missing_surface_negative/fixture.json` |
| `extraction_quality_gauge_persisted_gold` | `pass` | `crates/specforge/test_data/kg_quality/extraction_quality_gauge_persisted_gold/fixture.json` |
| `generic_bounded_cycle_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/generic_bounded_cycle_timing_gold/fixture.json` |
| `generic_clock_edge_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/generic_clock_edge_timing_gold/fixture.json` |
| `generic_exact_cycle_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/generic_exact_cycle_timing_gold/fixture.json` |
| `generic_next_cycle_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/generic_next_cycle_timing_gold/fixture.json` |
| `generic_range_cycle_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/generic_range_cycle_timing_gold/fixture.json` |
| `graph_direction_coverage_incomplete_negative` | `pass` | `crates/specforge/test_data/kg_quality/graph_direction_coverage_incomplete_negative/fixture.json` |
| `graph_direction_same_actor_conflict_negative` | `pass` | `crates/specforge/test_data/kg_quality/graph_direction_same_actor_conflict_negative/fixture.json` |
| `interface_signal_conflict_negative` | `pass` | `crates/specforge/test_data/kg_quality/interface_signal_conflict_negative/fixture.json` |
| `later_phrase_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/later_phrase_timing_gold/fixture.json` |
| `message_field_register_table_negative` | `pass` | `crates/specforge/test_data/kg_quality/message_field_register_table_negative/fixture.json` |
| `message_field_table_gold` | `pass` | `crates/specforge/test_data/kg_quality/message_field_table_gold/fixture.json` |
| `mixed_control_polarity_gold` | `pass` | `crates/specforge/test_data/kg_quality/mixed_control_polarity_gold/fixture.json` |
| `multi_control_polarity_gold` | `pass` | `crates/specforge/test_data/kg_quality/multi_control_polarity_gold/fixture.json` |
| `multi_producer_conflict_negative` | `pass` | `crates/specforge/test_data/kg_quality/multi_producer_conflict_negative/fixture.json` |
| `name_only_semantic_noise_negative` | `pass` | `crates/specforge/test_data/kg_quality/name_only_semantic_noise_negative/fixture.json` |
| `named_cycle_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/fixture.json` |
| `named_diagram_edge_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/named_diagram_edge_timing_gold/fixture.json` |
| `named_next_clock_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/fixture.json` |
| `named_quantified_edge_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/named_quantified_edge_timing_gold/fixture.json` |
| `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` | `pass` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_connectivity_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_interface_conflict_caution_gold` | `pass` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_interface_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_polarity_conflict_caution_gold` | `pass` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_polarity_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_residual_caution_gold` | `pass` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_residual_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_semantic_conflict_caution_gold` | `pass` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_semantic_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_guided_temporal_conflict_caution_gold` | `pass` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_temporal_conflict_caution_gold/fixture.json` |
| `negative_knowledge_prior_pattern_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_pattern_mismatch_negative/fixture.json` |
| `next_tick_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/next_tick_timing_gold/fixture.json` |
| `non_reset_control_polarity_gold` | `pass` | `crates/specforge/test_data/kg_quality/non_reset_control_polarity_gold/fixture.json` |
| `plural_edge_of_clock_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/plural_edge_of_clock_timing_gold/fixture.json` |
| `relative_clause_actor_noise_negative` | `pass` | `crates/specforge/test_data/kg_quality/relative_clause_actor_noise_negative/fixture.json` |
| `semantic_modality_reliability_global_prior_gold` | `pass` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_global_prior_gold/fixture.json` |
| `semantic_modality_reliability_prior_guided_conflict_gold` | `pass` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_gold/fixture.json` |
| `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_without_prior_negative/fixture.json` |
| `semantic_modality_reliability_role_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_role_mismatch_negative/fixture.json` |
| `semantic_modality_reliability_source_kind_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_source_kind_mismatch_negative/fixture.json` |
| `semantic_modality_reliability_weak_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_weak_prior_negative/fixture.json` |
| `semantic_prior_broad_phrase_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_prior_broad_phrase_negative/fixture.json` |
| `semantic_prior_conflicting_roles_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_prior_conflicting_roles_negative/fixture.json` |
| `semantic_prior_guided_phrase_gold` | `pass` | `crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_gold/fixture.json` |
| `semantic_prior_guided_phrase_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_without_prior_negative/fixture.json` |
| `semantic_prior_normalized_phrase_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_prior_normalized_phrase_mismatch_negative/fixture.json` |
| `semantic_prior_phrase_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_prior_phrase_mismatch_negative/fixture.json` |
| `semantic_prior_source_kind_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_prior_source_kind_mismatch_negative/fixture.json` |
| `semantic_ready_sink_prior_guided_phrase_gold` | `pass` | `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_gold/fixture.json` |
| `semantic_ready_sink_prior_guided_phrase_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_without_prior_negative/fixture.json` |
| `semantic_valid_prior_guided_phrase_gold` | `pass` | `crates/specforge/test_data/kg_quality/semantic_valid_prior_guided_phrase_gold/fixture.json` |
| `semantic_valid_prior_guided_phrase_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/semantic_valid_prior_guided_phrase_without_prior_negative/fixture.json` |
| `shorthand_next_edge_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/shorthand_next_edge_timing_gold/fixture.json` |
| `signal_leading_clock_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/signal_leading_clock_timing_gold/fixture.json` |
| `signal_presence_malformed_refusal_negative` | `pass` | `crates/specforge/test_data/kg_quality/signal_presence_malformed_refusal_negative/fixture.json` |
| `signal_presence_matrix_gold` | `pass` | `crates/specforge/test_data/kg_quality/signal_presence_matrix_gold/fixture.json` |
| `signal_table_inventory_authority_negative` | `pass` | `crates/specforge/test_data/kg_quality/signal_table_inventory_authority_negative/fixture.json` |
| `source_column_bogus_actor_attribution_negative` | `pass` | `crates/specforge/test_data/kg_quality/source_column_bogus_actor_attribution_negative/fixture.json` |
| `source_vlm_enrichment_surface_negative` | `pass` | `crates/specforge/test_data/kg_quality/source_vlm_enrichment_surface_negative/fixture.json` |
| `table_misclassification_field_table_negative` | `pass` | `crates/specforge/test_data/kg_quality/table_misclassification_field_table_negative/fixture.json` |
| `table_shape_prior_guided_signal_table_gold` | `pass` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_gold/fixture.json` |
| `table_shape_prior_guided_signal_table_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_without_prior_negative/fixture.json` |
| `table_shape_prior_guided_timing_table_gold` | `pass` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_gold/fixture.json` |
| `table_shape_prior_guided_timing_table_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json` |
| `table_shape_prior_header_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/table_shape_prior_header_mismatch_negative/fixture.json` |
| `temporal_actor_grounding_surface_negative` | `pass` | `crates/specforge/test_data/kg_quality/temporal_actor_grounding_surface_negative/fixture.json` |
| `temporal_clock_grounding_surface_negative` | `pass` | `crates/specforge/test_data/kg_quality/temporal_clock_grounding_surface_negative/fixture.json` |
| `temporal_conflict_negative` | `pass` | `crates/specforge/test_data/kg_quality/temporal_conflict_negative/fixture.json` |
| `temporal_cycle_window_grounded_gold` | `pass` | `crates/specforge/test_data/kg_quality/temporal_cycle_window_grounded_gold/fixture.json` |
| `temporal_cycle_window_surface_negative` | `pass` | `crates/specforge/test_data/kg_quality/temporal_cycle_window_surface_negative/fixture.json` |
| `temporal_prior_guided_cycle_window_gold` | `pass` | `crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_gold/fixture.json` |
| `temporal_prior_guided_cycle_window_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_without_prior_negative/fixture.json` |
| `temporal_prior_phrase_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/temporal_prior_phrase_mismatch_negative/fixture.json` |
| `tick_unit_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/tick_unit_timing_gold/fixture.json` |
| `trailing_shorthand_edge_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/fixture.json` |
| `unit_first_diagram_position_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/fixture.json` |
| `visual_motif_prior_caption_mismatch_negative` | `pass` | `crates/specforge/test_data/kg_quality/visual_motif_prior_caption_mismatch_negative/fixture.json` |
| `visual_motif_prior_guided_diagram_classification_gold` | `pass` | `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_gold/fixture.json` |
| `visual_motif_prior_guided_diagram_classification_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_without_prior_negative/fixture.json` |
| `visual_semantic_prior_guided_caption_gold` | `pass` | `crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_gold/fixture.json` |
| `visual_semantic_prior_guided_caption_without_prior_negative` | `pass` | `crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_without_prior_negative/fixture.json` |
| `visual_sources_semantic_conflict_negative` | `pass` | `crates/specforge/test_data/kg_quality/visual_sources_semantic_conflict_negative/fixture.json` |
| `vlm_state_machine_duplicate_initial_gold` | `pass` | `crates/specforge/test_data/kg_quality/vlm_state_machine_duplicate_initial_gold/fixture.json` |
| `vlm_state_machine_label_noise_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_state_machine_label_noise_negative/fixture.json` |
| `vlm_state_machine_missing_initial_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_state_machine_missing_initial_negative/fixture.json` |
| `vlm_state_machine_multiple_initial_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_state_machine_multiple_initial_negative/fixture.json` |
| `vlm_state_machine_undeclared_transition_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_state_machine_undeclared_transition_negative/fixture.json` |
| `vlm_timing_active_low_assertion_equivalence_gold` | `pass` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_assertion_equivalence_gold/fixture.json` |
| `vlm_timing_active_low_deassertion_equivalence_gold` | `pass` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_deassertion_equivalence_gold/fixture.json` |
| `vlm_timing_cycle_qualified_signal_value_annotation_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_timing_cycle_qualified_signal_value_annotation_negative/fixture.json` |
| `vlm_timing_indexed_signal_value_annotation_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_timing_indexed_signal_value_annotation_negative/fixture.json` |
| `vlm_timing_motion_annotation_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_timing_motion_annotation_negative/fixture.json` |
| `vlm_timing_name_only_semantic_noise_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_timing_name_only_semantic_noise_negative/fixture.json` |
| `vlm_timing_semantic_grounding_gold` | `pass` | `crates/specforge/test_data/kg_quality/vlm_timing_semantic_grounding_gold/fixture.json` |
| `vlm_timing_spurious_annotation_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_timing_spurious_annotation_negative/fixture.json` |
| `vlm_timing_waveform_motion_negative` | `pass` | `crates/specforge/test_data/kg_quality/vlm_timing_waveform_motion_negative/fixture.json` |
| `zero_cycle_timing_gold` | `pass` | `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/fixture.json` |

<!-- corpus_kb_kg_fixtures:end -->
