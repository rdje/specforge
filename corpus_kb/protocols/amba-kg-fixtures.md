# AMBA Family Fixture Patterns

This page records AMBA/APB/AHB/AXI-style KG fixture coverage from the tracked truthfulness benchmark suite.
It is derived from tracked KG fixture outcomes and remains reviewable synthesis, not canonical document truth.

## Human Synthesis

Use this section for curated notes about AMBA-family evidence idioms, protocol vocabulary, and future protocol-family benchmark gaps.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed Fixture Projection

<!-- corpus_kb_kg_fixture_family:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- source: `kg-bench fixtures`
- selected_family_labels: `protocol-family AMBA/APB/AHB/AXI`
- fixtures_total: `22`
- fixtures_passed: `22`
- fixtures_failed: `0`

| fixture | status | matched families | path |
| --- | --- | --- | --- |
| `ahb_control_stability_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/ahb_control_stability_gold/fixture.json` |
| `ahb_response_stability_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/ahb_response_stability_gold/fixture.json` |
| `ahb_section_heading_direction_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/ahb_section_heading_direction_gold/fixture.json` |
| `ahb_wait_state_timing_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/ahb_wait_state_timing_gold/fixture.json` |
| `ahb_write_data_stability_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/ahb_write_data_stability_gold/fixture.json` |
| `amba_destination_column_reads_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/amba_destination_column_reads_gold/fixture.json` |
| `amba_source_column_handshake_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/amba_source_column_handshake_gold/fixture.json` |
| `apb_requester_completer_handshake_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/apb_requester_completer_handshake_gold/fixture.json` |
| `apb_response_stability_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/apb_response_stability_gold/fixture.json` |
| `apb_setup_access_timing_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/apb_setup_access_timing_gold/fixture.json` |
| `apb_write_control_stability_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/apb_write_control_stability_gold/fixture.json` |
| `axi_next_cycle_timing_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_next_cycle_timing_gold/fixture.json` |
| `axi_read_address_timing_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_read_address_timing_gold/fixture.json` |
| `axi_read_data_last_stability_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_read_data_last_stability_gold/fixture.json` |
| `axi_read_data_timing_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_read_data_timing_gold/fixture.json` |
| `axi_sideband_stability_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_sideband_stability_gold/fixture.json` |
| `axi_width_only_prose_direction_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_width_only_prose_direction_gold/fixture.json` |
| `axi_write_address_sideband_stability_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_write_address_sideband_stability_gold/fixture.json` |
| `axi_write_data_last_stability_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_write_data_last_stability_gold/fixture.json` |
| `axi_write_data_timing_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_write_data_timing_gold/fixture.json` |
| `axi_write_response_timing_gold` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/axi_write_response_timing_gold/fixture.json` |
| `source_column_bogus_actor_attribution_negative` | `pass` | `protocol-family AMBA/APB/AHB/AXI` | `crates/specforge/test_data/kg_quality/source_column_bogus_actor_attribution_negative/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
