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
  - `IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf` (`intent_ir`): `90/100 EXCELLENT`

## Projected Artifacts
### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0022_l_2025_08_amba_axi_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- artifact_fingerprint: `a6bb40cb09cd4cf4`
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
- artifact_fingerprint: `ffc63e79fd0e3dec`
- score: `95/100 EXCELLENT`
- summary: IntentIR validation for IHI0024_D_2021-04_AMBA_APB_Protocol_Specification.pdf with 2 finding(s)
- findings:
  - [warning:residual_decisions] IntentIR still carries 2 residual decision packet(s)
  - [info:temporal_grounding] intent evidence includes timing/constraint records but no typed temporal rules were carried forward

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0033_c_2021_09_amba_5_ahb_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- artifact_fingerprint: `2c5a91d5146f1d20`
- score: `95/100 EXCELLENT`
- summary: IntentIR validation for IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf with 2 finding(s)
- findings:
  - [warning:residual_decisions] IntentIR still carries 2 residual decision packet(s)
  - [info:temporal_grounding] intent evidence includes timing/constraint records but no typed temporal rules were carried forward

### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0051_b_2021_04_amba_axi_stream_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- artifact_fingerprint: `f92438fe179b647b`
- score: `90/100 EXCELLENT`
- summary: IntentIR validation for IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf with 2 finding(s)
- findings:
  - [warning:residual_decisions] IntentIR still carries 1 residual decision packet(s)
  - [info:system_contract] 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing remains in the system-contract surface
