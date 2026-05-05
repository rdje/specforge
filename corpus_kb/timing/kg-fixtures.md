# Timing Motif Fixture Patterns

This page records temporal and timing-motif KG fixture coverage from the tracked truthfulness benchmark suite.
It is derived from tracked KG fixture outcomes and remains reviewable synthesis, not canonical document truth.

## Human Synthesis

Use this section for curated notes about cycle windows, handshake completion, timing diagrams, and temporal conflict patterns.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed Fixture Projection

<!-- corpus_kb_kg_fixture_family:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- source: `kg-bench fixtures`
- selected_family_labels: `temporal semantics`, `VLM timing diagrams`
- fixtures_total: `70`
- fixtures_passed: `70`
- fixtures_failed: `0`

| fixture | status | matched families | path |
| --- | --- | --- | --- |
| `ahb_control_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_control_stability_gold/fixture.json` |
| `ahb_exclusive_security_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_exclusive_security_stability_gold/fixture.json` |
| `ahb_response_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_response_stability_gold/fixture.json` |
| `ahb_transfer_lock_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_transfer_lock_stability_gold/fixture.json` |
| `ahb_wait_state_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_wait_state_timing_gold/fixture.json` |
| `ahb_write_data_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_write_data_stability_gold/fixture.json` |
| `apb_address_protection_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/apb_address_protection_stability_gold/fixture.json` |
| `apb_response_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/apb_response_stability_gold/fixture.json` |
| `apb_setup_access_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/apb_setup_access_timing_gold/fixture.json` |
| `apb_write_control_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/apb_write_control_stability_gold/fixture.json` |
| `axi_address_qos_region_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_address_qos_region_sideband_stability_gold/fixture.json` |
| `axi_address_response_user_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_address_response_user_sideband_stability_gold/fixture.json` |
| `axi_data_user_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_data_user_sideband_stability_gold/fixture.json` |
| `axi_next_cycle_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_next_cycle_timing_gold/fixture.json` |
| `axi_read_address_control_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_address_control_sideband_stability_gold/fixture.json` |
| `axi_read_address_id_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_address_id_stability_gold/fixture.json` |
| `axi_read_address_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_address_sideband_stability_gold/fixture.json` |
| `axi_read_address_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_address_timing_gold/fixture.json` |
| `axi_read_data_id_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_data_id_stability_gold/fixture.json` |
| `axi_read_data_last_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_data_last_stability_gold/fixture.json` |
| `axi_read_data_response_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_data_response_stability_gold/fixture.json` |
| `axi_read_data_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_data_timing_gold/fixture.json` |
| `axi_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_sideband_stability_gold/fixture.json` |
| `axi_write_address_control_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_address_control_sideband_stability_gold/fixture.json` |
| `axi_write_address_id_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_address_id_stability_gold/fixture.json` |
| `axi_write_address_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_address_sideband_stability_gold/fixture.json` |
| `axi_write_data_last_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_data_last_stability_gold/fixture.json` |
| `axi_write_data_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_data_timing_gold/fixture.json` |
| `axi_write_response_id_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_response_id_stability_gold/fixture.json` |
| `axi_write_response_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_response_timing_gold/fixture.json` |
| `clock_edge_of_clock_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/clock_edge_of_clock_timing_gold/fixture.json` |
| `default_clock_explicit_next_edge_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/fixture.json` |
| `default_clock_later_edge_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/default_clock_later_edge_timing_gold/fixture.json` |
| `default_clock_quantified_edge_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/default_clock_quantified_edge_timing_gold/fixture.json` |
| `generic_bounded_cycle_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/generic_bounded_cycle_timing_gold/fixture.json` |
| `generic_clock_edge_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/generic_clock_edge_timing_gold/fixture.json` |
| `generic_exact_cycle_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/generic_exact_cycle_timing_gold/fixture.json` |
| `generic_next_cycle_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/generic_next_cycle_timing_gold/fixture.json` |
| `generic_range_cycle_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/generic_range_cycle_timing_gold/fixture.json` |
| `later_phrase_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/later_phrase_timing_gold/fixture.json` |
| `named_cycle_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/fixture.json` |
| `named_diagram_edge_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/named_diagram_edge_timing_gold/fixture.json` |
| `named_next_clock_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/fixture.json` |
| `named_quantified_edge_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/named_quantified_edge_timing_gold/fixture.json` |
| `negative_knowledge_prior_guided_temporal_conflict_caution_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_temporal_conflict_caution_gold/fixture.json` |
| `next_tick_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/next_tick_timing_gold/fixture.json` |
| `plural_edge_of_clock_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/plural_edge_of_clock_timing_gold/fixture.json` |
| `shorthand_next_edge_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/shorthand_next_edge_timing_gold/fixture.json` |
| `signal_leading_clock_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/signal_leading_clock_timing_gold/fixture.json` |
| `table_shape_prior_guided_timing_table_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_gold/fixture.json` |
| `table_shape_prior_guided_timing_table_without_prior_negative` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json` |
| `temporal_actor_grounding_surface_negative` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/temporal_actor_grounding_surface_negative/fixture.json` |
| `temporal_clock_grounding_surface_negative` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/temporal_clock_grounding_surface_negative/fixture.json` |
| `temporal_conflict_negative` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/temporal_conflict_negative/fixture.json` |
| `temporal_prior_guided_cycle_window_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_gold/fixture.json` |
| `temporal_prior_guided_cycle_window_without_prior_negative` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_without_prior_negative/fixture.json` |
| `temporal_prior_protocol_family_mismatch_negative` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/temporal_prior_protocol_family_mismatch_negative/fixture.json` |
| `tick_unit_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/tick_unit_timing_gold/fixture.json` |
| `trailing_shorthand_edge_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/fixture.json` |
| `unit_first_diagram_position_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/fixture.json` |
| `vlm_timing_active_low_assertion_equivalence_gold` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_assertion_equivalence_gold/fixture.json` |
| `vlm_timing_active_low_deassertion_equivalence_gold` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_deassertion_equivalence_gold/fixture.json` |
| `vlm_timing_cycle_qualified_signal_value_annotation_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_cycle_qualified_signal_value_annotation_negative/fixture.json` |
| `vlm_timing_indexed_signal_value_annotation_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_indexed_signal_value_annotation_negative/fixture.json` |
| `vlm_timing_motion_annotation_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_motion_annotation_negative/fixture.json` |
| `vlm_timing_name_only_semantic_noise_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_name_only_semantic_noise_negative/fixture.json` |
| `vlm_timing_semantic_grounding_gold` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_semantic_grounding_gold/fixture.json` |
| `vlm_timing_spurious_annotation_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_spurious_annotation_negative/fixture.json` |
| `vlm_timing_waveform_motion_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_waveform_motion_negative/fixture.json` |
| `zero_cycle_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
