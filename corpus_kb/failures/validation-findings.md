# Validation Finding Patterns

This page is the first auto-refreshable page family in the `R15g` corpus knowledge base.
It records validation-finding patterns from reviewable validation reports without promoting them into canonical document truth.

## Human Synthesis

Use this section for curated notes that explain recurring validation patterns across documents.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed Validation Projection

<!-- corpus_kb_validation_findings:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

### ihi0022_l_2025_08_amba_axi_protocol_specification
- report_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/validation_report.json`
- stage: `intent_ir`
- artifact_fingerprint: `e7c46a81c53c55aa`
- score: `85/100 GOOD`
- summary: IntentIR validation for IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf with 7 finding(s)
- findings:
  - [info:system_contract] `intent_infrastructure_connectivity_missing_producer` 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing status remains explicit in the infrastructure/system-contract surface
  - [warning:signal_connectivity] `intent_signal_connectivity_conflicts_present` 1 signal connectivity conflict(s) detected; the carried structural KG still has unresolved producer ambiguity
  - [info:knowledge_graph] `intent_graph_direction_coverage_incomplete` 124 declared signal record(s) still lack graph-derived direction coverage
  - [info:compatibility_surface] `intent_compat_direction_hints_lag_graph` 118 declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist
  - [info:temporal_grounding] `intent_temporal_rules_missing_cycle_windows` typed temporal rules exist, but none currently carry explicit cycle-window bounds
  - [warning:temporal_conflicts] `intent_temporal_conflicts_present` 15 typed temporal conflict(s) detected across contradictory value obligations
  - [warning:quality_score] `intent_quality_below_excellent_threshold` IntentIR quality score is 85/100 (GOOD)

### ihi0024_e_2023_02_amba_5_apb_protocol_specification
- report_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/validation_report.json`
- stage: `intent_ir`
- artifact_fingerprint: `59890c4af5c8ebc9`
- score: `90/100 EXCELLENT`
- summary: IntentIR validation for IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf with 2 finding(s)
- findings:
  - [info:temporal_grounding] `intent_temporal_rule_surface_missing` intent evidence includes timing/constraint records but no typed temporal rules were carried forward
  - [warning:residual_decisions] `intent_residual_decisions_present` IntentIR still carries 2 residual decision packet(s)

### ihi0033_c_2021_09_amba_5_ahb_protocol_specification
- report_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/validation_report.json`
- stage: `intent_ir`
- artifact_fingerprint: `d3eb361d5eab0ea3`
- score: `94/100 EXCELLENT`
- summary: IntentIR validation for IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf with 8 finding(s)
- findings:
  - [warning:signal_connectivity] `intent_connectivity_missing_producer` 22 signal(s) in IntentIR connectivity have no resolved producer actor
  - [info:system_contract] `intent_infrastructure_connectivity_missing_producer` 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing status remains explicit in the infrastructure/system-contract surface
  - [warning:signal_connectivity] `intent_connectivity_missing_consumer` 8 signal(s) in IntentIR connectivity have no resolved consumer actor
  - [warning:signal_connectivity] `intent_signal_connectivity_conflicts_present` 2 signal connectivity conflict(s) detected; the carried structural KG still has unresolved producer ambiguity
  - [warning:semantic_role_conflicts] `intent_signal_semantic_conflicts_present` 1 semantic-role conflict(s) remain unresolved in the carried canonical intent surface
  - [info:semantic_role_arbitration] `intent_non_decisive_semantic_arbitration_present` 7 declared signal(s) still have competing semantic role candidates; the canonical surface preserves the current lead, runner-up, and evidence margin without forcing an unsafe winner
  - [info:knowledge_graph] `intent_graph_direction_coverage_incomplete` 1 declared signal record(s) still lack graph-derived direction coverage
  - [info:compatibility_surface] `intent_compat_direction_hints_lag_graph` 9 declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist

### ihi0051_b_2021_04_amba_axi_stream_protocol_specification
- report_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/validation_report.json`
- stage: `intent_ir`
- artifact_fingerprint: `26dc733965c17b02`
- score: `90/100 EXCELLENT`
- summary: IntentIR validation for IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf with 1 finding(s)
- findings:
  - [info:system_contract] `intent_infrastructure_connectivity_missing_producer` 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing status remains explicit in the infrastructure/system-contract surface

<!-- corpus_kb_validation_findings:end -->
