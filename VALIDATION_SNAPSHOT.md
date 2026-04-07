# VALIDATION_SNAPSHOT
This file is auto-refreshed by `specforge project-validation <artifact>...`.
It summarizes the latest persisted validation reports projected from IR artifacts into the tracked live-doc surface.

## Snapshot Summary
- Artifacts projected: 4
- Highest severity observed: warning
- Score-bearing artifacts:
  - `IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf` (`intent_ir`): `94/100 EXCELLENT`
  - `IHI0024_D_2021-04_AMBA_APB_Protocol_Specification.pdf` (`intent_ir`): `95/100 EXCELLENT`
  - `IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf` (`intent_ir`): `95/100 EXCELLENT`
  - `IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf` (`intent_ir`): `80/100 GOOD`

## Projected Artifacts
### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0022_l_2025_08_amba_axi_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- artifact_fingerprint: `bd5a19cded996605`
- score: `94/100 EXCELLENT`
- summary: IntentIR validation for IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf with 5 finding(s)
- findings:
  - [warning:residual_decisions] IntentIR still carries 2 residual decision packet(s)
  - [warning:signal_connectivity] 181 signal(s) in IntentIR connectivity have no resolved consumer actor
  - [warning:signal_connectivity] 1 signal(s) in IntentIR connectivity have no resolved producer actor
  - [info:knowledge_graph] 4 declared signal record(s) still lack graph-derived direction coverage
  - [info:temporal_grounding] intent evidence includes timing/constraint records but no typed temporal rules were carried forward

### IHI0024_D_2021-04_AMBA_APB_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0024_d_2021_04_amba_apb_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json`
- artifact_fingerprint: `72381fb741880cb7`
- score: `95/100 EXCELLENT`
- summary: IntentIR validation for IHI0024_D_2021-04_AMBA_APB_Protocol_Specification.pdf with 2 finding(s)
- findings:
  - [warning:residual_decisions] IntentIR still carries 2 residual decision packet(s)
  - [info:temporal_grounding] intent evidence includes timing/constraint records but no typed temporal rules were carried forward

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0033_c_2021_09_amba_5_ahb_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- artifact_fingerprint: `2f46f0567c10e64b`
- score: `95/100 EXCELLENT`
- summary: IntentIR validation for IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf with 2 finding(s)
- findings:
  - [warning:residual_decisions] IntentIR still carries 2 residual decision packet(s)
  - [info:temporal_grounding] intent evidence includes timing/constraint records but no typed temporal rules were carried forward

### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0051_b_2021_04_amba_axi_stream_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- artifact_fingerprint: `9de985d1f7c445dc`
- score: `80/100 GOOD`
- summary: IntentIR validation for IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf with 7 finding(s)
- findings:
  - [warning:quality_score] IntentIR quality score is 80/100 (GOOD)
  - [warning:residual_decisions] IntentIR still carries 1 residual decision packet(s)
  - [warning:signal_connectivity] 10 signal(s) in IntentIR connectivity have no resolved consumer actor
  - [warning:temporal_conflicts] 2 typed temporal conflict(s) detected across contradictory value obligations
  - [info:compatibility_surface] 8 declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist
  - [info:knowledge_graph] 10 declared signal record(s) still lack graph-derived direction coverage
  - [info:temporal_grounding] typed temporal rules exist, but none currently carry explicit cycle-window bounds
