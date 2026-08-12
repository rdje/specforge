# Table Extraction Fixture Patterns

This page records table-related KG fixture coverage from the tracked truthfulness benchmark suite.
It is derived from tracked KG fixture outcomes and remains reviewable synthesis, not canonical document truth.

## Human Synthesis

Use this section for curated notes about table-shape recovery, table-misclassification risks, and future table-prior candidates.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed Fixture Projection

<!-- corpus_kb_kg_fixture_family:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- source: `kg-bench fixtures`
- selected_structural_capabilities: `table extraction and hygiene`
- fixtures_total: `65`
- fixtures_passed: `65`
- fixtures_failed: `0`

| fixture | status | matched capabilities | path |
| --- | --- | --- | --- |
| `actor_taxonomy_prior_guided_section_direction_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json` |
| `actor_taxonomy_prior_guided_section_direction_without_prior_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_without_prior_negative/fixture.json` |
| `actor_taxonomy_prior_term_mismatch_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/actor_taxonomy_prior_term_mismatch_negative/fixture.json` |
| `ahb_control_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/ahb_control_stability_gold/fixture.json` |
| `ahb_exclusive_security_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/ahb_exclusive_security_stability_gold/fixture.json` |
| `ahb_response_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/ahb_response_stability_gold/fixture.json` |
| `ahb_section_heading_direction_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/ahb_section_heading_direction_gold/fixture.json` |
| `ahb_transfer_lock_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/ahb_transfer_lock_stability_gold/fixture.json` |
| `ahb_wait_state_timing_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/ahb_wait_state_timing_gold/fixture.json` |
| `ahb_write_data_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/ahb_write_data_stability_gold/fixture.json` |
| `amba_destination_column_reads_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/amba_destination_column_reads_gold/fixture.json` |
| `amba_source_column_handshake_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/amba_source_column_handshake_gold/fixture.json` |
| `apb_address_protection_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/apb_address_protection_stability_gold/fixture.json` |
| `apb_requester_completer_handshake_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/apb_requester_completer_handshake_gold/fixture.json` |
| `apb_response_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/apb_response_stability_gold/fixture.json` |
| `apb_setup_access_timing_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/apb_setup_access_timing_gold/fixture.json` |
| `apb_write_control_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/apb_write_control_stability_gold/fixture.json` |
| `axi_address_qos_region_sideband_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_address_qos_region_sideband_stability_gold/fixture.json` |
| `axi_address_response_user_sideband_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_address_response_user_sideband_stability_gold/fixture.json` |
| `axi_data_user_sideband_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_data_user_sideband_stability_gold/fixture.json` |
| `axi_next_cycle_timing_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_next_cycle_timing_gold/fixture.json` |
| `axi_read_address_control_sideband_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_read_address_control_sideband_stability_gold/fixture.json` |
| `axi_read_address_id_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_read_address_id_stability_gold/fixture.json` |
| `axi_read_address_sideband_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_read_address_sideband_stability_gold/fixture.json` |
| `axi_read_address_timing_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_read_address_timing_gold/fixture.json` |
| `axi_read_data_id_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_read_data_id_stability_gold/fixture.json` |
| `axi_read_data_last_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_read_data_last_stability_gold/fixture.json` |
| `axi_read_data_response_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_read_data_response_stability_gold/fixture.json` |
| `axi_read_data_timing_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_read_data_timing_gold/fixture.json` |
| `axi_sideband_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_sideband_stability_gold/fixture.json` |
| `axi_width_only_prose_direction_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_width_only_prose_direction_gold/fixture.json` |
| `axi_write_address_control_sideband_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_write_address_control_sideband_stability_gold/fixture.json` |
| `axi_write_address_id_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_write_address_id_stability_gold/fixture.json` |
| `axi_write_address_sideband_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_write_address_sideband_stability_gold/fixture.json` |
| `axi_write_address_timing_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_write_address_timing_gold/fixture.json` |
| `axi_write_data_last_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_write_data_last_stability_gold/fixture.json` |
| `axi_write_data_timing_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_write_data_timing_gold/fixture.json` |
| `axi_write_response_id_stability_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_write_response_id_stability_gold/fixture.json` |
| `axi_write_response_timing_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/axi_write_response_timing_gold/fixture.json` |
| `contested_handshake_name_fallback_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/contested_handshake_name_fallback_negative/fixture.json` |
| `control_polarity_conflict_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/control_polarity_conflict_negative/fixture.json` |
| `cross_modality_semantic_conflict_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/cross_modality_semantic_conflict_negative/fixture.json` |
| `cross_modality_semantic_grounding_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/cross_modality_semantic_grounding_gold/fixture.json` |
| `message_field_register_table_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/message_field_register_table_negative/fixture.json` |
| `message_field_table_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/message_field_table_gold/fixture.json` |
| `negative_knowledge_prior_guided_polarity_conflict_caution_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_polarity_conflict_caution_gold/fixture.json` |
| `semantic_modality_reliability_global_prior_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_global_prior_gold/fixture.json` |
| `semantic_modality_reliability_prior_guided_conflict_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_gold/fixture.json` |
| `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_without_prior_negative/fixture.json` |
| `semantic_modality_reliability_role_mismatch_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_role_mismatch_negative/fixture.json` |
| `semantic_modality_reliability_source_kind_mismatch_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_source_kind_mismatch_negative/fixture.json` |
| `semantic_modality_reliability_weak_prior_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/semantic_modality_reliability_weak_prior_negative/fixture.json` |
| `signal_presence_malformed_refusal_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/signal_presence_malformed_refusal_negative/fixture.json` |
| `signal_presence_matrix_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/signal_presence_matrix_gold/fixture.json` |
| `signal_table_inventory_authority_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/signal_table_inventory_authority_negative/fixture.json` |
| `source_column_bogus_actor_attribution_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/source_column_bogus_actor_attribution_negative/fixture.json` |
| `table_misclassification_field_table_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_misclassification_field_table_negative/fixture.json` |
| `table_shape_prior_guided_signal_table_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_gold/fixture.json` |
| `table_shape_prior_guided_signal_table_without_prior_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_without_prior_negative/fixture.json` |
| `table_shape_prior_guided_timing_table_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_gold/fixture.json` |
| `table_shape_prior_guided_timing_table_without_prior_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json` |
| `table_shape_prior_header_mismatch_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_header_mismatch_negative/fixture.json` |
| `visual_motif_prior_caption_mismatch_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/visual_motif_prior_caption_mismatch_negative/fixture.json` |
| `visual_motif_prior_guided_diagram_classification_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_gold/fixture.json` |
| `visual_motif_prior_guided_diagram_classification_without_prior_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_without_prior_negative/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
