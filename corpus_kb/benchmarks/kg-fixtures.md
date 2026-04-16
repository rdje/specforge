# KG Benchmark Fixture Results

This page is the benchmark-result projection family in the `R15g` corpus knowledge base.
It records KG fixture outcomes from the tracked truthfulness benchmark suite without promoting benchmark behavior into canonical document truth.

## Human Synthesis

Use this section for curated notes about recurring benchmark patterns, fixture families, and candidate follow-up work.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed KG Benchmark Projection

<!-- corpus_kb_kg_fixtures:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- fixtures_total: `81`
- fixtures_passed: `81`
- fixtures_failed: `0`

### Fixture Family Summary
Fixtures can appear in more than one family because protocol semantics, modality, and expected behavior are orthogonal.

| family | fixtures | passed | failed |
| --- | ---: | ---: | ---: |
| VLM state machines | `5` | `5` | `0` |
| VLM timing diagrams | `7` | `7` | `0` |
| actor connectivity | `11` | `11` | `0` |
| infrastructure semantics | `3` | `3` | `0` |
| multimodal visual grounding | `19` | `19` | `0` |
| negative knowledge | `5` | `5` | `0` |
| polarity semantics | `7` | `7` | `0` |
| protocol-family AMBA/APB/AHB/AXI | `32` | `32` | `0` |
| residuals and caveats | `3` | `3` | `0` |
| semantic role arbitration | `17` | `17` | `0` |
| table extraction and hygiene | `8` | `8` | `0` |
| temporal semantics | `38` | `38` | `0` |
| truthfulness negatives and cautions | `32` | `32` | `0` |
| typed prior memory | `21` | `21` | `0` |

### actor_boundary_residual
- fixture_path: `crates/specforge/test_data/kg_quality/actor_boundary_residual/fixture.json`
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

### clock_reset_topology_gold
- fixture_path: `crates/specforge/test_data/kg_quality/clock_reset_topology_gold/fixture.json`
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

### detached_mixed_control_polarity_negative
- fixture_path: `crates/specforge/test_data/kg_quality/detached_mixed_control_polarity_negative/fixture.json`
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

### non_reset_control_polarity_gold
- fixture_path: `crates/specforge/test_data/kg_quality/non_reset_control_polarity_gold/fixture.json`
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

### source_column_bogus_actor_attribution_negative
- fixture_path: `crates/specforge/test_data/kg_quality/source_column_bogus_actor_attribution_negative/fixture.json`
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

<!-- corpus_kb_kg_fixtures:end -->
