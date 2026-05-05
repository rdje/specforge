# KG Benchmark Fixture Results

This page is the benchmark-result projection family in the `R15g` corpus knowledge base.
It records KG fixture outcomes from the tracked truthfulness benchmark suite without promoting benchmark behavior into canonical document truth.

## Human Synthesis

Use this section for curated notes about recurring benchmark patterns, fixture families, and candidate follow-up work.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed KG Benchmark Projection

<!-- corpus_kb_kg_fixtures:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- fixtures_total: `147`
- fixtures_passed: `147`
- fixtures_failed: `0`

### Fixture Family Summary
Fixtures can appear in more than one family because protocol semantics, modality, and expected behavior are orthogonal.

| family | fixtures | passed | failed |
| --- | ---: | ---: | ---: |
| VLM state machines | `5` | `5` | `0` |
| VLM timing diagrams | `9` | `9` | `0` |
| actor connectivity | `21` | `21` | `0` |
| infrastructure semantics | `14` | `14` | `0` |
| multimodal visual grounding | `24` | `24` | `0` |
| negative knowledge | `7` | `7` | `0` |
| polarity semantics | `8` | `8` | `0` |
| protocol-family AMBA/APB/AHB/AXI | `36` | `36` | `0` |
| residuals and caveats | `4` | `4` | `0` |
| semantic role arbitration | `30` | `30` | `0` |
| table extraction and hygiene | `10` | `10` | `0` |
| temporal semantics | `70` | `70` | `0` |
| truthfulness negatives and cautions | `69` | `69` | `0` |
| typed prior memory | `31` | `31` | `0` |

### actor_boundary_residual
- fixture_path: `crates/specforge/test_data/kg_quality/actor_boundary_residual/fixture.json`
- status: `pass`
- failures:
  - none

### actor_port_gap_surface_negative
- fixture_path: `crates/specforge/test_data/kg_quality/actor_port_gap_surface_negative/fixture.json`
- status: `pass`
- failures:
  - none

### actor_ports_gold
- fixture_path: `crates/specforge/test_data/kg_quality/actor_ports_gold/fixture.json`
- status: `pass`
- failures:
  - none

### actor_taxonomy_prior_guided_section_direction_gold
- fixture_path: `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json`
- status: `pass`
- failures:
  - none

### actor_taxonomy_prior_guided_section_direction_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### ahb_control_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/ahb_control_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### ahb_exclusive_security_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/ahb_exclusive_security_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### ahb_response_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/ahb_response_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### ahb_section_heading_direction_gold
- fixture_path: `crates/specforge/test_data/kg_quality/ahb_section_heading_direction_gold/fixture.json`
- status: `pass`
- failures:
  - none

### ahb_transfer_lock_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/ahb_transfer_lock_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### ahb_wait_state_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/ahb_wait_state_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### ahb_write_data_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/ahb_write_data_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### alias_dependent_handshake_completion_caveat
- fixture_path: `crates/specforge/test_data/kg_quality/alias_dependent_handshake_completion_caveat/fixture.json`
- status: `pass`
- failures:
  - none

### amba_destination_column_reads_gold
- fixture_path: `crates/specforge/test_data/kg_quality/amba_destination_column_reads_gold/fixture.json`
- status: `pass`
- failures:
  - none

### amba_source_column_handshake_gold
- fixture_path: `crates/specforge/test_data/kg_quality/amba_source_column_handshake_gold/fixture.json`
- status: `pass`
- failures:
  - none

### apb_address_protection_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/apb_address_protection_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### apb_requester_completer_handshake_gold
- fixture_path: `crates/specforge/test_data/kg_quality/apb_requester_completer_handshake_gold/fixture.json`
- status: `pass`
- failures:
  - none

### apb_response_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/apb_response_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### apb_setup_access_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/apb_setup_access_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### apb_write_control_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/apb_write_control_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_address_qos_region_sideband_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_address_qos_region_sideband_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_address_response_user_sideband_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_address_response_user_sideband_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_data_user_sideband_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_data_user_sideband_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_next_cycle_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_next_cycle_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_read_address_control_sideband_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_read_address_control_sideband_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_read_address_id_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_read_address_id_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_read_address_sideband_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_read_address_sideband_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_read_address_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_read_address_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_read_data_id_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_read_data_id_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_read_data_last_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_read_data_last_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_read_data_response_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_read_data_response_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_read_data_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_read_data_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_sideband_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_sideband_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_width_only_prose_direction_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_width_only_prose_direction_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_write_address_control_sideband_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_write_address_control_sideband_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_write_address_id_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_write_address_id_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_write_address_sideband_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_write_address_sideband_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_write_data_last_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_write_data_last_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_write_data_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_write_data_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_write_response_id_stability_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_write_response_id_stability_gold/fixture.json`
- status: `pass`
- failures:
  - none

### axi_write_response_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/axi_write_response_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### clock_edge_of_clock_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/clock_edge_of_clock_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### clock_reset_contract_scope_negative
- fixture_path: `crates/specforge/test_data/kg_quality/clock_reset_contract_scope_negative/fixture.json`
- status: `pass`
- failures:
  - none

### clock_reset_generic_advice_negative
- fixture_path: `crates/specforge/test_data/kg_quality/clock_reset_generic_advice_negative/fixture.json`
- status: `pass`
- failures:
  - none

### clock_reset_topology_gold
- fixture_path: `crates/specforge/test_data/kg_quality/clock_reset_topology_gold/fixture.json`
- status: `pass`
- failures:
  - none

### compat_direction_hints_graph_conflict_incomplete_negative
- fixture_path: `crates/specforge/test_data/kg_quality/compat_direction_hints_graph_conflict_incomplete_negative/fixture.json`
- status: `pass`
- failures:
  - none

### compat_direction_hints_incomplete_negative
- fixture_path: `crates/specforge/test_data/kg_quality/compat_direction_hints_incomplete_negative/fixture.json`
- status: `pass`
- failures:
  - none

### compat_direction_hints_lag_graph_negative
- fixture_path: `crates/specforge/test_data/kg_quality/compat_direction_hints_lag_graph_negative/fixture.json`
- status: `pass`
- failures:
  - none

### compat_direction_hints_mixed_lag_incomplete_negative
- fixture_path: `crates/specforge/test_data/kg_quality/compat_direction_hints_mixed_lag_incomplete_negative/fixture.json`
- status: `pass`
- failures:
  - none

### connectivity_endpoint_gaps_negative
- fixture_path: `crates/specforge/test_data/kg_quality/connectivity_endpoint_gaps_negative/fixture.json`
- status: `pass`
- failures:
  - none

### contested_handshake_name_fallback_negative
- fixture_path: `crates/specforge/test_data/kg_quality/contested_handshake_name_fallback_negative/fixture.json`
- status: `pass`
- failures:
  - none

### control_polarity_conflict_negative
- fixture_path: `crates/specforge/test_data/kg_quality/control_polarity_conflict_negative/fixture.json`
- status: `pass`
- failures:
  - none

### cross_modality_semantic_conflict_negative
- fixture_path: `crates/specforge/test_data/kg_quality/cross_modality_semantic_conflict_negative/fixture.json`
- status: `pass`
- failures:
  - none

### cross_modality_semantic_grounding_gold
- fixture_path: `crates/specforge/test_data/kg_quality/cross_modality_semantic_grounding_gold/fixture.json`
- status: `pass`
- failures:
  - none

### default_clock_explicit_next_edge_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### default_clock_later_edge_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/default_clock_later_edge_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### default_clock_quantified_edge_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/default_clock_quantified_edge_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### detached_mixed_control_polarity_negative
- fixture_path: `crates/specforge/test_data/kg_quality/detached_mixed_control_polarity_negative/fixture.json`
- status: `pass`
- failures:
  - none

### evidence_missing_vlm_observations_surface_negative
- fixture_path: `crates/specforge/test_data/kg_quality/evidence_missing_vlm_observations_surface_negative/fixture.json`
- status: `pass`
- failures:
  - none

### evidence_normative_residual_surface_negative
- fixture_path: `crates/specforge/test_data/kg_quality/evidence_normative_residual_surface_negative/fixture.json`
- status: `pass`
- failures:
  - none

### evidence_structural_kg_missing_surface_negative
- fixture_path: `crates/specforge/test_data/kg_quality/evidence_structural_kg_missing_surface_negative/fixture.json`
- status: `pass`
- failures:
  - none

### generic_bounded_cycle_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/generic_bounded_cycle_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### generic_clock_edge_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/generic_clock_edge_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### generic_exact_cycle_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/generic_exact_cycle_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### generic_next_cycle_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/generic_next_cycle_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### generic_range_cycle_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/generic_range_cycle_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### graph_direction_coverage_incomplete_negative
- fixture_path: `crates/specforge/test_data/kg_quality/graph_direction_coverage_incomplete_negative/fixture.json`
- status: `pass`
- failures:
  - none

### graph_direction_same_actor_conflict_negative
- fixture_path: `crates/specforge/test_data/kg_quality/graph_direction_same_actor_conflict_negative/fixture.json`
- status: `pass`
- failures:
  - none

### interface_signal_conflict_negative
- fixture_path: `crates/specforge/test_data/kg_quality/interface_signal_conflict_negative/fixture.json`
- status: `pass`
- failures:
  - none

### later_phrase_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/later_phrase_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### mixed_control_polarity_gold
- fixture_path: `crates/specforge/test_data/kg_quality/mixed_control_polarity_gold/fixture.json`
- status: `pass`
- failures:
  - none

### multi_control_polarity_gold
- fixture_path: `crates/specforge/test_data/kg_quality/multi_control_polarity_gold/fixture.json`
- status: `pass`
- failures:
  - none

### multi_producer_conflict_negative
- fixture_path: `crates/specforge/test_data/kg_quality/multi_producer_conflict_negative/fixture.json`
- status: `pass`
- failures:
  - none

### name_only_semantic_noise_negative
- fixture_path: `crates/specforge/test_data/kg_quality/name_only_semantic_noise_negative/fixture.json`
- status: `pass`
- failures:
  - none

### named_cycle_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### named_diagram_edge_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/named_diagram_edge_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### named_next_clock_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### named_quantified_edge_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/named_quantified_edge_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### negative_knowledge_prior_guided_connectivity_conflict_caution_gold
- fixture_path: `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_connectivity_conflict_caution_gold/fixture.json`
- status: `pass`
- failures:
  - none

### negative_knowledge_prior_guided_interface_conflict_caution_gold
- fixture_path: `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_interface_conflict_caution_gold/fixture.json`
- status: `pass`
- failures:
  - none

### negative_knowledge_prior_guided_polarity_conflict_caution_gold
- fixture_path: `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_polarity_conflict_caution_gold/fixture.json`
- status: `pass`
- failures:
  - none

### negative_knowledge_prior_guided_residual_caution_gold
- fixture_path: `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_residual_caution_gold/fixture.json`
- status: `pass`
- failures:
  - none

### negative_knowledge_prior_guided_semantic_conflict_caution_gold
- fixture_path: `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_semantic_conflict_caution_gold/fixture.json`
- status: `pass`
- failures:
  - none

### negative_knowledge_prior_guided_temporal_conflict_caution_gold
- fixture_path: `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_temporal_conflict_caution_gold/fixture.json`
- status: `pass`
- failures:
  - none

### negative_knowledge_prior_protocol_family_mismatch_negative
- fixture_path: `crates/specforge/test_data/kg_quality/negative_knowledge_prior_protocol_family_mismatch_negative/fixture.json`
- status: `pass`
- failures:
  - none

### next_tick_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/next_tick_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### non_reset_control_polarity_gold
- fixture_path: `crates/specforge/test_data/kg_quality/non_reset_control_polarity_gold/fixture.json`
- status: `pass`
- failures:
  - none

### plural_edge_of_clock_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/plural_edge_of_clock_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### relative_clause_actor_noise_negative
- fixture_path: `crates/specforge/test_data/kg_quality/relative_clause_actor_noise_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_modality_reliability_amba_generic_fallback_gold
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_modality_reliability_amba_generic_fallback_gold/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_modality_reliability_prior_guided_conflict_gold
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_gold/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_modality_reliability_prior_guided_conflict_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_modality_reliability_protocol_family_mismatch_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_modality_reliability_protocol_family_mismatch_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_modality_reliability_source_kind_mismatch_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_modality_reliability_source_kind_mismatch_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_modality_reliability_weak_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_modality_reliability_weak_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_prior_broad_phrase_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_prior_broad_phrase_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_prior_conflicting_roles_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_prior_conflicting_roles_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_prior_guided_phrase_gold
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_gold/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_prior_guided_phrase_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_prior_phrase_mismatch_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_prior_phrase_mismatch_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_prior_protocol_family_mismatch_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_prior_protocol_family_mismatch_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_prior_source_kind_mismatch_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_prior_source_kind_mismatch_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_ready_sink_prior_guided_phrase_gold
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_gold/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_ready_sink_prior_guided_phrase_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_ready_sink_prior_guided_phrase_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_valid_prior_guided_phrase_gold
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_valid_prior_guided_phrase_gold/fixture.json`
- status: `pass`
- failures:
  - none

### semantic_valid_prior_guided_phrase_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/semantic_valid_prior_guided_phrase_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### shorthand_next_edge_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/shorthand_next_edge_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### signal_leading_clock_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/signal_leading_clock_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### signal_table_inventory_authority_negative
- fixture_path: `crates/specforge/test_data/kg_quality/signal_table_inventory_authority_negative/fixture.json`
- status: `pass`
- failures:
  - none

### source_column_bogus_actor_attribution_negative
- fixture_path: `crates/specforge/test_data/kg_quality/source_column_bogus_actor_attribution_negative/fixture.json`
- status: `pass`
- failures:
  - none

### source_vlm_enrichment_surface_negative
- fixture_path: `crates/specforge/test_data/kg_quality/source_vlm_enrichment_surface_negative/fixture.json`
- status: `pass`
- failures:
  - none

### table_misclassification_field_table_negative
- fixture_path: `crates/specforge/test_data/kg_quality/table_misclassification_field_table_negative/fixture.json`
- status: `pass`
- failures:
  - none

### table_shape_prior_guided_signal_table_gold
- fixture_path: `crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_gold/fixture.json`
- status: `pass`
- failures:
  - none

### table_shape_prior_guided_signal_table_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### table_shape_prior_guided_timing_table_gold
- fixture_path: `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_gold/fixture.json`
- status: `pass`
- failures:
  - none

### table_shape_prior_guided_timing_table_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### table_shape_prior_protocol_family_mismatch_negative
- fixture_path: `crates/specforge/test_data/kg_quality/table_shape_prior_protocol_family_mismatch_negative/fixture.json`
- status: `pass`
- failures:
  - none

### temporal_actor_grounding_surface_negative
- fixture_path: `crates/specforge/test_data/kg_quality/temporal_actor_grounding_surface_negative/fixture.json`
- status: `pass`
- failures:
  - none

### temporal_clock_grounding_surface_negative
- fixture_path: `crates/specforge/test_data/kg_quality/temporal_clock_grounding_surface_negative/fixture.json`
- status: `pass`
- failures:
  - none

### temporal_conflict_negative
- fixture_path: `crates/specforge/test_data/kg_quality/temporal_conflict_negative/fixture.json`
- status: `pass`
- failures:
  - none

### temporal_prior_guided_cycle_window_gold
- fixture_path: `crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_gold/fixture.json`
- status: `pass`
- failures:
  - none

### temporal_prior_guided_cycle_window_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### temporal_prior_protocol_family_mismatch_negative
- fixture_path: `crates/specforge/test_data/kg_quality/temporal_prior_protocol_family_mismatch_negative/fixture.json`
- status: `pass`
- failures:
  - none

### tick_unit_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/tick_unit_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### trailing_shorthand_edge_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### unit_first_diagram_position_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

### visual_motif_prior_guided_diagram_classification_gold
- fixture_path: `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_gold/fixture.json`
- status: `pass`
- failures:
  - none

### visual_motif_prior_guided_diagram_classification_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### visual_motif_prior_protocol_family_mismatch_negative
- fixture_path: `crates/specforge/test_data/kg_quality/visual_motif_prior_protocol_family_mismatch_negative/fixture.json`
- status: `pass`
- failures:
  - none

### visual_semantic_prior_guided_caption_gold
- fixture_path: `crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_gold/fixture.json`
- status: `pass`
- failures:
  - none

### visual_semantic_prior_guided_caption_without_prior_negative
- fixture_path: `crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_without_prior_negative/fixture.json`
- status: `pass`
- failures:
  - none

### visual_sources_semantic_conflict_negative
- fixture_path: `crates/specforge/test_data/kg_quality/visual_sources_semantic_conflict_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_state_machine_duplicate_initial_gold
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_state_machine_duplicate_initial_gold/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_state_machine_label_noise_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_state_machine_label_noise_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_state_machine_missing_initial_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_state_machine_missing_initial_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_state_machine_multiple_initial_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_state_machine_multiple_initial_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_state_machine_undeclared_transition_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_state_machine_undeclared_transition_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_timing_active_low_assertion_equivalence_gold
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_timing_active_low_assertion_equivalence_gold/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_timing_active_low_deassertion_equivalence_gold
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_timing_active_low_deassertion_equivalence_gold/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_timing_cycle_qualified_signal_value_annotation_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_timing_cycle_qualified_signal_value_annotation_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_timing_indexed_signal_value_annotation_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_timing_indexed_signal_value_annotation_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_timing_motion_annotation_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_timing_motion_annotation_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_timing_name_only_semantic_noise_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_timing_name_only_semantic_noise_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_timing_semantic_grounding_gold
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_timing_semantic_grounding_gold/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_timing_spurious_annotation_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_timing_spurious_annotation_negative/fixture.json`
- status: `pass`
- failures:
  - none

### vlm_timing_waveform_motion_negative
- fixture_path: `crates/specforge/test_data/kg_quality/vlm_timing_waveform_motion_negative/fixture.json`
- status: `pass`
- failures:
  - none

### zero_cycle_timing_gold
- fixture_path: `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/fixture.json`
- status: `pass`
- failures:
  - none

<!-- corpus_kb_kg_fixtures:end -->
