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
- fixtures_total: `20`
- fixtures_passed: `20`
- fixtures_failed: `0`

| fixture | status | matched families | path |
| --- | --- | --- | --- |
| `clock_edge_of_clock_timing_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/clock_edge_of_clock_timing_gold/fixture.json` |
| `clock_reset_contract_scope_negative` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/clock_reset_contract_scope_negative/fixture.json` |
| `clock_reset_generic_advice_negative` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/clock_reset_generic_advice_negative/fixture.json` |
| `clock_reset_topology_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/clock_reset_topology_gold/fixture.json` |
| `control_polarity_conflict_negative` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/control_polarity_conflict_negative/fixture.json` |
| `default_clock_explicit_next_edge_timing_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/fixture.json` |
| `default_clock_later_edge_timing_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/default_clock_later_edge_timing_gold/fixture.json` |
| `default_clock_quantified_edge_timing_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/default_clock_quantified_edge_timing_gold/fixture.json` |
| `detached_mixed_control_polarity_negative` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/detached_mixed_control_polarity_negative/fixture.json` |
| `generic_clock_edge_timing_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/generic_clock_edge_timing_gold/fixture.json` |
| `mixed_control_polarity_gold` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/mixed_control_polarity_gold/fixture.json` |
| `multi_control_polarity_gold` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/multi_control_polarity_gold/fixture.json` |
| `named_next_clock_timing_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/fixture.json` |
| `negative_knowledge_prior_guided_polarity_conflict_caution_gold` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_polarity_conflict_caution_gold/fixture.json` |
| `non_reset_control_polarity_gold` | `pass` | `polarity semantics` | `crates/specforge/test_data/kg_quality/non_reset_control_polarity_gold/fixture.json` |
| `plural_edge_of_clock_timing_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/plural_edge_of_clock_timing_gold/fixture.json` |
| `signal_leading_clock_timing_gold` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/signal_leading_clock_timing_gold/fixture.json` |
| `temporal_clock_grounding_surface_negative` | `pass` | `infrastructure semantics` | `crates/specforge/test_data/kg_quality/temporal_clock_grounding_surface_negative/fixture.json` |
| `vlm_timing_active_low_assertion_equivalence_gold` | `pass` | `infrastructure semantics`, `polarity semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_assertion_equivalence_gold/fixture.json` |
| `vlm_timing_active_low_deassertion_equivalence_gold` | `pass` | `infrastructure semantics`, `polarity semantics` | `crates/specforge/test_data/kg_quality/vlm_timing_active_low_deassertion_equivalence_gold/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
