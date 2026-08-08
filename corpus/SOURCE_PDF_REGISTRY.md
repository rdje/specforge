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

| document_key | class | repo path | corpus directory |
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
| `ihi0022_h_c_2021_01_amba_axi_and_ace_protocol_specification` | wire-bus / coherency (ACE) | `corpus/arm/amba/core/axi/legacy/IHI0022_H.c_2021-01_AMBA_AXI_and_ACE_Protocol_Specification.pdf` | `arm/amba/core/axi/legacy/` |
| `ihi0024_d_2021_04_amba_apb_protocol_specification` | wire-bus (APB, legacy issue) | `corpus/arm/amba/core/apb/legacy/IHI0024_D_2021-04_AMBA_APB_Protocol_Specification.pdf` | `arm/amba/core/apb/legacy/` |
| `ihi0032_c_2021_12_amba_trace_bus_protocol_specification` | wire-bus / trace (ATB) | `corpus/arm/amba/supporting/atb/current/IHI0032_C_2021-12_AMBA_Trace_Bus_Protocol_Specification.pdf` | `arm/amba/supporting/atb/current/` |
| `ihi0089_d_2025_08_amba_lti_protocol_specification` | wire-bus / translation (LTI) | `corpus/arm/amba/specialized/lti/current/IHI0089_D_2025-08_AMBA_LTI_Protocol_Specification.pdf` | `arm/amba/specialized/lti/current/` |
| `ihi0050_g_2024_03_amba_chi_architecture_specification` | packet/flit coherency (CHI) | `corpus/arm/amba/core/chi/current/IHI0050_G_2024-03_AMBA_CHI_Architecture_Specification.pdf` | `arm/amba/core/chi/current/` |
| `ccix_base_specification_r1_0_v1_0` | packet / register-structure (CCIX r1.0) | `corpus/cxl/ccix/current/CCIX_Base_Specification_r1.0_v1.0.pdf` | `cxl/ccix/current/` |
| `ccix_base_specification_r1_0a_v1_0_for_evaluation` | packet / register-structure (CCIX r1.0a) | `corpus/cxl/ccix/current/CCIX_Base_Specification_r1.0a_v1.0_for-Evaluation.pdf` | `cxl/ccix/current/` |
| `ccix_base_specification_revision1_1_version1_0` | packet / register-structure (CCIX 1.1) | `corpus/cxl/ccix/current/CCIX_Base_Specification_Revision1.1_Version1.0.pdf` | `cxl/ccix/current/` |
| `ccix_base_specification_rev_2_0_v1_0` | packet / register-structure (CCIX 2.0) | `corpus/cxl/ccix/current/CCIX_Base_Specification-Rev-2.0-v1.0.pdf` | `cxl/ccix/current/` |
| `48882_pub_3_10_2025_02_amd_io_virtualization_technology_iommu_specification` | register / in-memory structure (AMD IOMMU) | `corpus/amd/system-ip/iommu/current/48882_PUB_3.10_2025-02_AMD_IO_Virtualization_Technology_IOMMU_Specification.pdf` | `amd/system-ip/iommu/current/` |

The three register/prose-signal `PDF-VARIANT-DIGESTION` specs (RISC-V Debug + NVMe register-field recovery
`.2`/`.2c`, I2C prose-signal capture `.3a`) and the four `PDF-VARIANT-DIGESTION.9` serial-protocol specs
(CAN / SWP / SMBus / I2S — owner-downloaded serial chip-spec class, `2026-06-08`) were copied in to exercise
the corresponding SpecForge extraction features. The ten `PDF-VARIANT-DIGESTION.13` re-ingest-sweep specs
(ACE / APB-legacy / ATB / LTI / CHI / CCIX ×4 / AMD IOMMU — owner-granted from the host-local library,
`2026-06-11`) were copied in so the recorded pending wins (the `.12a` ACE wires, the `.12b` presence
records, the `EXTRACTION-QUALITY-GAUGE.3c` CHI gauge, the `.10`-family message-field canonical rebuilds)
become reproducible re-ingests; their filenames are kept verbatim so each derived `document_key` matches
its persisted artifacts. `SourceIR` derives the key from the final stem: retain lowercase ASCII alphanumerics,
collapse each other-character run to one `_`, then trim leading/trailing `_` characters.

Currentness is executable through `scripts/check_source_pdf_registry_currentness.pl`: every Git-tracked PDF below `corpus/`
appears exactly once with its code-derived key, parent directory, and PDF signature; host-local libraries and git-ignored generated artifacts are outside this tracked membership authority.

To add another spec: copy its PDF under `corpus/` mirroring the source library's relative structure, add
a row here, git-track it, and run the currentness checker (owner directive `2026-06-07`).
