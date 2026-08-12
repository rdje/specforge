# State-Machine Fixture Patterns

This page records VLM state-machine KG fixture coverage from the tracked truthfulness benchmark suite.
It is derived from tracked KG fixture outcomes and remains reviewable synthesis, not canonical document truth.

## Human Synthesis

Use this section for curated notes about state labels, transition endpoint grounding, duplicate initial markers, and initial-cardinality validation behavior.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed Fixture Projection

<!-- corpus_kb_kg_fixture_family:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- source: `kg-bench fixtures`
- selected_structural_capabilities: `state-machine semantics`
- fixtures_total: `5`
- fixtures_passed: `5`
- fixtures_failed: `0`

| fixture | status | matched capabilities | path |
| --- | --- | --- | --- |
| `vlm_state_machine_duplicate_initial_gold` | `pass` | `state-machine semantics` | `crates/specforge/test_data/kg_quality/vlm_state_machine_duplicate_initial_gold/fixture.json` |
| `vlm_state_machine_label_noise_negative` | `pass` | `state-machine semantics` | `crates/specforge/test_data/kg_quality/vlm_state_machine_label_noise_negative/fixture.json` |
| `vlm_state_machine_missing_initial_negative` | `pass` | `state-machine semantics` | `crates/specforge/test_data/kg_quality/vlm_state_machine_missing_initial_negative/fixture.json` |
| `vlm_state_machine_multiple_initial_negative` | `pass` | `state-machine semantics` | `crates/specforge/test_data/kg_quality/vlm_state_machine_multiple_initial_negative/fixture.json` |
| `vlm_state_machine_undeclared_transition_negative` | `pass` | `state-machine semantics` | `crates/specforge/test_data/kg_quality/vlm_state_machine_undeclared_transition_negative/fixture.json` |

<!-- corpus_kb_kg_fixture_family:end -->
