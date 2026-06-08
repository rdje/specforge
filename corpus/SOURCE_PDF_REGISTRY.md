# Source PDF registry

Durable, git-tracked source PDFs for the specs SpecForge actively works with, so a re-ingest is
always reproducible (artifact-cleanup reclaims the normalized source; without the PDF in-repo a
re-ingest is impossible — this previously blocked the `WIRE-BASED-100` cross-spec eval). Source PDFs
are **inputs**, not generated artifacts, so tracking them does not conflict with the artifact-cleanup
directive.

**Scope:** only the PDFs SpecForge intends to use (the WIRE-BASED-100 wire-based + serial-debug specs).
The owner keeps a larger host-local spec library and **provides its path on request** (it is deliberately
not recorded here or in any tracked file). Only the PDFs SpecForge needs are copied in; more may be added
when needed (owner: "you can use more if you need to").

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

The three `PDF-VARIANT-DIGESTION` specs above were copied in when SpecForge gained a feature exercising
them: RISC-V Debug + NVMe (register-field recovery `.2`/`.2c`), I2C (prose-signal capture `.3a`).

To add another spec: copy its PDF under `corpus/` mirroring the source library's relative structure, add
a row here, and git-track it (owner directive `2026-06-07`).
