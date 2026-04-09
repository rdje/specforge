# VALIDATION_SNAPSHOT
This file is auto-refreshed by `specforge project-validation <artifact>...`.
It summarizes the latest persisted validation reports projected from IR artifacts into the tracked live-doc surface.

## Snapshot Summary
- Artifacts projected: 4
- Highest severity observed: warning
- Score-bearing artifacts:
  - `IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf` (`intent_ir`): `85/100 GOOD`
  - `IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf` (`intent_ir`): `90/100 EXCELLENT`
  - `IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf` (`intent_ir`): `94/100 EXCELLENT`
  - `IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf` (`intent_ir`): `90/100 EXCELLENT`

## Projected Artifacts
### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0022_l_2025_08_amba_axi_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- artifact_fingerprint: `51ff25441921fa7b`
- score: `85/100 GOOD`
- summary: IntentIR validation for IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf with 7 finding(s)
- findings:
  - [warning:quality_score] IntentIR quality score is 85/100 (GOOD)
  - [warning:signal_connectivity] 1 signal connectivity conflict(s) detected; the carried structural KG still has unresolved producer ambiguity
  - [warning:temporal_conflicts] 15 typed temporal conflict(s) detected across contradictory value obligations
  - [info:compatibility_surface] 118 declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist
  - [info:knowledge_graph] 124 declared signal record(s) still lack graph-derived direction coverage
  - [info:system_contract] 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing remains in the system-contract surface
  - [info:temporal_grounding] typed temporal rules exist, but none currently carry explicit cycle-window bounds

### IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0024_e_2023_02_amba_5_apb_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`
- artifact_fingerprint: `3dce7e0d613c9b2c`
- score: `90/100 EXCELLENT`
- summary: IntentIR validation for IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf with 2 finding(s)
- findings:
  - [warning:residual_decisions] IntentIR still carries 2 residual decision packet(s)
  - [info:temporal_grounding] intent evidence includes timing/constraint records but no typed temporal rules were carried forward

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0033_c_2021_09_amba_5_ahb_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- artifact_fingerprint: `674ae0823b38a6c2`
- score: `94/100 EXCELLENT`
- summary: IntentIR validation for IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf with 8 finding(s)
- findings:
  - [warning:semantic_role_conflicts] 1 semantic-role conflict(s) remain unresolved in the carried canonical intent surface
  - [warning:signal_connectivity] 8 signal(s) in IntentIR connectivity have no resolved consumer actor
  - [warning:signal_connectivity] 22 signal(s) in IntentIR connectivity have no resolved producer actor
  - [warning:signal_connectivity] 2 signal connectivity conflict(s) detected; the carried structural KG still has unresolved producer ambiguity
  - [info:compatibility_surface] 9 declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist
  - [info:knowledge_graph] 1 declared signal record(s) still lack graph-derived direction coverage
  - [info:semantic_role_arbitration] 7 declared signal(s) still have competing semantic role candidates; the canonical surface preserves the current lead, runner-up, and evidence margin without forcing an unsafe winner
  - [info:system_contract] 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing remains in the system-contract surface

### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0051_b_2021_04_amba_axi_stream_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- artifact_fingerprint: `43e63fc2e564398d`
- score: `90/100 EXCELLENT`
- summary: IntentIR validation for IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf with 1 finding(s)
- findings:
  - [info:system_contract] 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing remains in the system-contract surface
