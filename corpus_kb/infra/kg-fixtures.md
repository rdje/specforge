# Infrastructure Semantics Fixture Patterns

This page records infrastructure-adjacent KG fixture coverage from the tracked truthfulness benchmark suite.
It is derived from tracked KG fixture outcomes and remains reviewable synthesis, not canonical document truth.

## Human Synthesis

Use this section for curated notes about clock/reset handling, active-level polarity, and infrastructure/control boundaries.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed Fixture Projection

<!-- corpus_kb_kg_fixture_family:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- source: `kg-bench fixtures`
- selected_family_labels: `infrastructure semantics`, `polarity semantics`
- fixtures_total: `8`
- fixtures_passed: `8`
- fixtures_failed: `0`

| fixture | status | matched families | path |
| --- | --- | --- | --- |
| `clock_reset_topology_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/clock_reset_topology_gold/fixture.json` |
| `control_polarity_conflict_negative` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/control_polarity_conflict_negative/fixture.json` |
| `detached_mixed_control_polarity_negative` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/detached_mixed_control_polarity_negative/fixture.json` |
| `mixed_control_polarity_gold` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/mixed_control_polarity_gold/fixture.json` |
| `multi_control_polarity_gold` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/multi_control_polarity_gold/fixture.json` |
| `non_reset_control_polarity_gold` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/non_reset_control_polarity_gold/fixture.json` |
| `vlm_timing_active_low_assertion_equivalence_gold` | `pass` | `infrastructure semantics`, `polarity semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_assertion_equivalence_gold/fixture.json` |
| `vlm_timing_active_low_deassertion_equivalence_gold` | `pass` | `infrastructure semantics`, `polarity semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_deassertion_equivalence_gold/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
