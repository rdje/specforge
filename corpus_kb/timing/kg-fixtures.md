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
- fixtures_total: `27`
- fixtures_passed: `27`
- fixtures_failed: `0`

| fixture | status | matched families | path |
| --- | --- | --- | --- |
| `ahb_control_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_control_stability_gold/fixture.json` |
| `ahb_response_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_response_stability_gold/fixture.json` |
| `ahb_wait_state_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_wait_state_timing_gold/fixture.json` |
| `ahb_write_data_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/ahb_write_data_stability_gold/fixture.json` |
| `apb_response_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/apb_response_stability_gold/fixture.json` |
| `apb_setup_access_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/apb_setup_access_timing_gold/fixture.json` |
| `apb_write_control_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/apb_write_control_stability_gold/fixture.json` |
| `axi_next_cycle_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_next_cycle_timing_gold/fixture.json` |
| `axi_read_address_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_address_timing_gold/fixture.json` |
| `axi_read_data_last_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_data_last_stability_gold/fixture.json` |
| `axi_read_data_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_read_data_timing_gold/fixture.json` |
| `axi_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_sideband_stability_gold/fixture.json` |
| `axi_write_address_sideband_stability_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_address_sideband_stability_gold/fixture.json` |
| `axi_write_data_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_data_timing_gold/fixture.json` |
| `axi_write_response_timing_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/axi_write_response_timing_gold/fixture.json` |
| `negative_knowledge_prior_guided_temporal_conflict_caution_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_temporal_conflict_caution_gold/fixture.json` |
| `table_shape_prior_guided_timing_table_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_gold/fixture.json` |
| `table_shape_prior_guided_timing_table_without_prior_negative` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json` |
| `temporal_prior_guided_cycle_window_gold` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_gold/fixture.json` |
| `temporal_prior_guided_cycle_window_without_prior_negative` | `pass` | `temporal semantics` | `crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_without_prior_negative/fixture.json` |
| `vlm_timing_active_low_assertion_equivalence_gold` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_assertion_equivalence_gold/fixture.json` |
| `vlm_timing_active_low_deassertion_equivalence_gold` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_deassertion_equivalence_gold/fixture.json` |
| `vlm_timing_motion_annotation_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_motion_annotation_negative/fixture.json` |
| `vlm_timing_name_only_semantic_noise_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_name_only_semantic_noise_negative/fixture.json` |
| `vlm_timing_semantic_grounding_gold` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_semantic_grounding_gold/fixture.json` |
| `vlm_timing_spurious_annotation_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_spurious_annotation_negative/fixture.json` |
| `vlm_timing_waveform_motion_negative` | `pass` | `VLM timing diagrams`, `temporal semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_waveform_motion_negative/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
