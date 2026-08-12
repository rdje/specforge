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
- selected_family_labels: `table extraction and hygiene`
- fixtures_total: `12`
- fixtures_passed: `12`
- fixtures_failed: `0`

| fixture | status | matched families | path |
| --- | --- | --- | --- |
| `amba_destination_column_reads_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/amba_destination_column_reads_gold/fixture.json` |
| `amba_source_column_handshake_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/amba_source_column_handshake_gold/fixture.json` |
| `message_field_register_table_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/message_field_register_table_negative/fixture.json` |
| `message_field_table_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/message_field_table_gold/fixture.json` |
| `signal_table_inventory_authority_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/signal_table_inventory_authority_negative/fixture.json` |
| `source_column_bogus_actor_attribution_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/source_column_bogus_actor_attribution_negative/fixture.json` |
| `table_misclassification_field_table_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_misclassification_field_table_negative/fixture.json` |
| `table_shape_prior_guided_signal_table_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_gold/fixture.json` |
| `table_shape_prior_guided_signal_table_without_prior_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_without_prior_negative/fixture.json` |
| `table_shape_prior_guided_timing_table_gold` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_gold/fixture.json` |
| `table_shape_prior_guided_timing_table_without_prior_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json` |
| `table_shape_prior_header_mismatch_negative` | `pass` | `table extraction and hygiene` | `crates/specforge/test_data/kg_quality/table_shape_prior_header_mismatch_negative/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
