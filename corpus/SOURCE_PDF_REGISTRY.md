# Source PDF registry

Durable, git-tracked source PDFs for the specs SpecForge actively works with, so a re-ingest is
always reproducible (artifact-cleanup reclaims the normalized source; without the PDF in-repo a
re-ingest is impossible — this previously blocked the `WIRE-BASED-100` cross-spec eval). Source PDFs
are **inputs**, not generated artifacts, so tracking them does not conflict with the artifact-cleanup
directive.

**Scope:** only the PDFs SpecForge intends to use (the WIRE-BASED-100 wire-based + serial-debug specs, the
`PDF-VARIANT-DIGESTION` register/prose-signal specs, and the `PDF-VARIANT-DIGESTION.9` new serial-protocol
class — CAN / SWP / SMBus / I2S). The owner keeps a larger host-local spec library and **provides its path on
request** (it is deliberately not recorded here or in any tracked file). Only the PDFs SpecForge needs are
copied in; more may be added when needed (owner: "you can use more if you need to").

**Re-ingest** a spec to refresh its `source_ir`/normalized before measuring (the eval scores persisted
evidence — see `docs/knowledge/eval-scores-persisted-evidence.md`):

```bash
DOCLING_DEVICE=cpu cargo run -p specforge -- ingest <repo PDF path>
# then: specforge evidence … ; specforge semantic … ; specforge eval-extraction …
```

| document_key | class | repo path | source (relative) |
|---|---|---|---|
| `ihi0024_e_2023_02_amba_5_apb_protocol_specification` | wire-bus (APB) | `corpus/arm/amba/core/apb/current/IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf` | `arm/amba/core/apb/current/` |
| `ihi0033_c_2021_09_amba_5_ahb_protocol_specification` | wire-bus (AHB) | `corpus/arm/amba/core/ahb/current/IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf` | `arm/amba/core/ahb/current/` |
| `ihi0022_l_2025_08_amba_axi_protocol_specification` | wire-bus (AXI) | `corpus/arm/amba/core/axi/current/IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf` | `arm/amba/core/axi/current/` |
| `ihi0051_b_2021_04_amba_axi_stream_protocol_specification` | wire-bus (AXI-Stream) | `corpus/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf` | `arm/amba/supporting/axi-stream/current/` |
| `ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification` | serial-debug (SWD/ADI) | `corpus/arm/debug/interfaces/adi/current/IHI0074_A_2017-03-09_Arm_Debug_Interface_v6_Architecture_Specification.pdf` | `arm/debug/interfaces/adi/current/` |
| `1_0_risc_v_debug_specification` | register/debug (RISC-V Debug) | `corpus/risc-v/debug/current/1.0_RISC_V_Debug_Specification.pdf` | `risc-v/debug/current/` |
| `nvme_base_specification_2_0a_2021_07_26` | register (NVMe) | `corpus/nvm-express/nvme/current/NVMe-Base-Specification-2.0a-2021.07.26.pdf` | `nvm-express/nvme/current/` |
| `um10204_rev7_0_2021_i2c_bus_specification` | prose-signal (I2C) | `corpus/nxp/i2c/current/UM10204_Rev7.0_2021_I2C-bus_Specification.pdf` | `nxp/i2c/current/` |
| `bosch_can_specification_2_0_1991` | serial (CAN) | `corpus/bosch/can/current/Bosch_CAN_Specification_2.0_1991.pdf` | `bosch/can/current/` |
| `etsi_ts_102613_v16_0_0_2021_10_smart_cards_uicc_clf_single_wire_protocol_swp` | serial / single-wire (SWP) | `corpus/etsi/swp/current/ETSI_TS_102613_V16.0.0_2021-10_Smart_Cards_UICC_CLF_Single_Wire_Protocol_SWP.pdf` | `etsi/swp/current/` |
| `smbus_3_3_1_2024_10_20_system_management_bus_specification` | serial (SMBus) | `corpus/smbus/current/SMBus_3.3.1_2024-10-20_System_Management_Bus_Specification.pdf` | `smbus/current/` |
| `um11732_v3_2022_02_17_i2s_bus_specification` | serial (I2S) | `corpus/nxp/i2s/current/UM11732_v3_2022-02-17_I2S_Bus_Specification.pdf` | `nxp/i2s/current/` |

The three register/prose-signal `PDF-VARIANT-DIGESTION` specs (RISC-V Debug + NVMe register-field recovery
`.2`/`.2c`, I2C prose-signal capture `.3a`) and the four `PDF-VARIANT-DIGESTION.9` serial-protocol specs
(CAN / SWP / SMBus / I2S — owner-downloaded serial chip-spec class, `2026-06-08`) were copied in to exercise
the corresponding SpecForge extraction features. The `document_key` is each spec's normalized filename
(lowercased, `-`/`.`/spaces → `_`); it is confirmed against `generated/source_ir/<key>/` on first ingest.

To add another spec: copy its PDF under `corpus/` mirroring the source library's relative structure, add
a row here, and git-track it (owner directive `2026-06-07`).
