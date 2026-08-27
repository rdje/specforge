#!/usr/bin/env python3
"""Build the review-locked source-to-IntentIR vertical fixture.

This is fixture construction, not production extraction. It refuses artifact drift at the
selection boundary, projects only the reviewed records into a bounded normalized snapshot, and
never writes an external host path to the tracked dataset.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[4]
FIXTURE = Path("crates/specforge/test_data/source_to_intent_vertical/reviewed_dataset.json")
SELECTION_COMMIT = "a3e9757d63ca5499a2393864fb503d6537de0035"
MISSING = "<missing>"
ACTIONABILITY_FIELDS = ("reason", "first_failing_stage", "replay")


def external(portable_id: str, sha256: str, necessity: str) -> dict:
    return {
        "location": "external_read_only",
        "portable_id": portable_id,
        "sha256": sha256,
        "necessity": necessity,
    }


def canonical_table_cell(
    family: str, gold: list[str], oracle: str, scope: str
) -> dict:
    return {
        "family": family,
        "disposition": "canonical",
        "gold": gold,
        "residual_gold": gold,
        "oracle": oracle,
        "scope": scope,
    }


def residual_cell(
    family: str, residual_gold: list[str], oracle: str, scope: str
) -> dict:
    return {
        "family": family,
        "disposition": "residual",
        "gold": [],
        "residual_gold": residual_gold,
        "oracle": oracle,
        "scope": scope,
    }


def non_applicable_cell(
    family: str, residual_gold: list[str], oracle: str, scope: str
) -> dict:
    cell = residual_cell(family, residual_gold, oracle, scope)
    cell["disposition"] = "non_applicable"
    return cell


def physical_cells(digital: list[str], analog: list[str], scope: str) -> list[dict]:
    return [
        {
            "family": "digital_lane_skew",
            "disposition": "canonical",
            "gold": digital,
            "residual_gold": [],
            "oracle": "Lane-to-lane skew bounds in UI are digital boundary timing facts.",
            "scope": scope,
        },
        non_applicable_cell(
            "analog_channel_loss",
            analog,
            "Insertion-loss and insertion-loss-deviation values in dB/dB_RMS are analog channel facts, not executable digital intent.",
            scope,
        ),
    ]


DOCS = [
    {
        "key": "ihi0024_e_2023_02_amba_5_apb_protocol_specification",
        "category": "wire-protocol",
        "source": {
            "location": "repository",
            "relative_path": "corpus/arm/amba/core/apb/current/IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf",
            "sha256": "c1e0fbc73d7bc61f1fdb63de75e78607ea989c1bffc8da0476bca3a84e40f204",
        },
        "hashes": [
            "0eb30dfe48d44f3ca6e2e5d945717c706673276c4ffadfe42715120126009fc3",
            "0fe31edf71f9c51f7122ab3b1967f564af592c50438103ac702fee5d570b7cc1",
            "33eb31ba9ce43319e87178ac5bc7920ea0a806a8ec2b6e7186df55cb5268142e",
            "f044ec43a9fe2bcc13891e7d965791d077df56babbc250f01d5bc7e958f215f1",
        ],
        "region": ["prose", "elem_00205", "statement_0203"],
        "source_assert": "The Setup phase of the write transfer occurs at T1",
        "projection": "apb_signal",
        "present_modalities": ["prose"],
        "cells": [
            {
                "family": "setup_signal_state",
                "disposition": "canonical",
                "gold": [
                    "PADDR|must_be_value|VALID",
                    "PWDATA|must_be_value|VALID",
                    "PSEL|must_be_asserted|<missing>",
                    "PWRITE|must_be_value|VALID",
                ],
                "residual_gold": ["PSEL|must_be_asserted|<missing>"],
                "oracle": "The prose states PSEL asserted and PADDR/PWRITE/PWDATA valid at setup T1.",
                "scope": "Complete review of SourceIR elem_00205 and its four explicit signal-state obligations.",
            }
        ],
    },
    {
        "key": "um11732_v3_2022_02_17_i2s_bus_specification",
        "category": "wire-protocol",
        "source": {
            "location": "repository",
            "relative_path": "corpus/nxp/i2s/current/UM11732_v3_2022-02-17_I2S_Bus_Specification.pdf",
            "sha256": "905d20f3cd7c38f8118a5fab0aa7f7a65d0fb8608681af6d8b6156842e603f53",
        },
        "hashes": [
            "905e97c88051caa3e30697846662328b801c014a28b742b1b7ec7db826da1c81",
            "574b771cbe974ed99a76f3ee3844a73f24c3ef4f1f82b1dee4bd69b3773b838f",
            "a8d474684dadca12e3543e5dbb8e6efce892d032577cbb8f04af95818c7c5e83",
            "8aa3938506131c91c696067a6d1ecd86412d6b8b8c36c799241a472cee9a44e7",
        ],
        "region": ["table", "table_0004", None],
        "source_assert": "all values in ns",
        "projection": "timing_full",
        "present_modalities": ["table"],
        "cells": [
            {
                "family": "receiver_timing",
                "disposition": "canonical",
                "gold": [
                    "clock HIGH t HC|min=110|typ=<missing>|max=<missing>|unit=ns",
                    "clock LOW t LC|min=110|typ=<missing>|max=<missing>|unit=ns",
                    "clock period T|min=360|typ=400|max=440|unit=ns",
                    "hold time t htr|min=0|typ=<missing>|max=<missing>|unit=ns",
                    "set-up time t sr|min=60|typ=<missing>|max=<missing>|unit=ns",
                ],
                "residual_gold": [
                    "clock HIGH t HC|min=110|typ=<missing>|max=<missing>|unit=ns",
                    "clock LOW t LC|min=110|typ=<missing>|max=<missing>|unit=ns",
                    "clock period T|min=360|typ=400|max=440|unit=ns",
                    "hold time t htr|min=0|typ=<missing>|max=<missing>|unit=ns",
                    "set-up time t sr|min=60|typ=<missing>|max=<missing>|unit=ns",
                ],
                "oracle": "All five receiver timing rows are canonical timing facts and inherit nanoseconds from the caption.",
                "scope": "Complete review of all five data rows in SourceIR table_0004 and its caption-level unit.",
            }
        ],
    },
    {
        "key": "ddi0471_a_2011_06_23_gic_400_technical_reference_manual",
        "category": "register-ip",
        "source": external(
            "DDI0471_A_2011-06-23_GIC_400_Technical_Reference_Manual.pdf",
            "afcac68f733304fb2a236a5667d502783a3965d4609cc97d2115fc5d911fd7ec",
            "Second register-IP document is necessary because only one selected register-IP source is repository-tracked.",
        ),
        "hashes": [
            "89d9d73772b4f8931d38c4c13fb842c7247ac9054ee68f81cd107492a314f54c",
            "4a69d49bd936e8c25fed29fc0b2a3015cc02ebe7d933a64b10ce57d9c5a2b787",
            "9255f4b42831415a8ce3eceffb716a8a9b2280528d1fbe86fde068a19d4f1cf5",
            "5bf8705f8a4390b8c29f8311c705b83e12ad9a7c3e87e277230f30e9ae6e2edd",
        ],
        "region": ["table", "table_0014", None],
        "source_assert": "CPU interface register summary",
        "projection": "register_access_offset",
        "present_modalities": ["table"],
        "cells": [
            canonical_table_cell(
                "register_summary",
                [
                    "GICC_ABPR|access=RW|offset=0x001C",
                    "GICC_AEOIR|access=WO|offset=0x0024",
                    "GICC_AHPPIR|access=RO|offset=0x0028",
                    "GICC_AIAR|access=RO|offset=0x0020",
                    "GICC_APR0|access=RW|offset=0x00D0",
                    "GICC_BPR|access=RW|offset=0x0008",
                    "GICC_CTLR|access=RW|offset=0x0000",
                    "GICC_DIR|access=WO|offset=0x1000",
                    "GICC_EOIR|access=WO|offset=0x0010",
                    "GICC_HPPIR|access=RO|offset=0x0018",
                    "GICC_IAR|access=RO|offset=0x000C",
                    "GICC_IIDR|access=RO|offset=0x00FC",
                    "GICC_NSAPR0|access=RW|offset=0x00E0",
                    "GICC_PMR|access=RW|offset=0x0004",
                    "GICC_RPR|access=RO|offset=0x0014",
                ],
                "The 15 table body rows are 15 registers with name, access, offset, reset, and description.",
                "Complete review of all 15 body rows in SourceIR table_0014.",
            )
        ],
    },
    {
        "key": "48882_pub_3_10_2025_02_amd_io_virtualization_technology_iommu_specification",
        "category": "register-ip",
        "source": {
            "location": "repository",
            "relative_path": "corpus/amd/system-ip/iommu/current/48882_PUB_3.10_2025-02_AMD_IO_Virtualization_Technology_IOMMU_Specification.pdf",
            "sha256": "cd7559334b3f46a72d56e9f2fac786fcbf5d6a2c41e7770a46fcacc080bc8a12",
        },
        "hashes": [
            "c34bb5d7dd786e7a686437fb6e4dc2006da6c06bd532d44ed105e0663af4cf4e",
            "e0431096295d9eb9852a40e2bb4c09e9daf519bfa8ec891e09261c9c589da51f",
            "6e405451c02fec729ac8b8a9db8788ccf641ce147416a9271efa280f0ba6f1b8",
            "18554337e70cf5304b9a4b6019111285120e11b16396662208a11a905ad46f65",
        ],
        "region": ["table", "table_0067", None],
        "source_assert": "Guest-Physical Page-Table Base Address",
        "projection": "register_name",
        "present_modalities": ["table"],
        "cells": [
            residual_cell(
                "packed_page_table_entry",
                ["table_0067|packed_page_table_entry"],
                "The bit-layout is a packed page-table entry with address and NX/PAT/AVL/G/D/A/PCD/PWT/U-S/R-W/P fields, not a register named for one address slice.",
                "Complete review of the single 64-bit packed layout represented by SourceIR table_0067.",
            )
        ],
    },
    {
        "key": "den0068_2018_07_23_coresight_base_system_architecture",
        "category": "platform-system-ip",
        "source": external(
            "DEN0068_2018-07-23_CoreSight_Base_System_Architecture.pdf",
            "c4a5f342afbae7ef5229f5f8344dbd63ec6b5473cd319830dd148d9dc732a66d",
            "A visually reviewed component-topology figure is required for platform/system-IP coverage.",
        ),
        "hashes": [
            "adcfcb07786211eae17b0e492a7ecba8ee1fac0ba37ccaa8368dc9ba70c67da3",
            "e2a7455d61bf8bded41378d80ed352685847afb7da82209886285eb51f72ea16",
            "bd1fe1ad57006c4f7121e17887636f4611246dc43a53ac735bc6694927087ef6",
            "459e48c4f721e40c908e4c5e2a90bce714b32988ce37225e42e779b5161857ac",
        ],
        "region": ["figure", "picture_0001", "visual_0008"],
        "source_assert": "shared ETB",
        "projection": "none",
        "present_modalities": ["figure"],
        "cells": [
            residual_cell(
                "static_component_topology",
                ["picture_0001|static_component_topology"],
                "The reviewed figure shows four PE/Trace Unit pairs feeding a Trace Funnel, then an ETB containing SRAM.",
                "Complete visual review of every labeled component and directed edge in Figure 1 picture_0001.",
            )
        ],
    },
    {
        "key": "1_0_1_2026_02_22_risc_v_iommu_architecture_specification",
        "category": "platform-system-ip",
        "source": external(
            "1.0.1_2026-02-22_RISC_V_IOMMU_Architecture_Specification.pdf",
            "be2134b407a1a3ce5a9da1779cbdc4b4c641ee50ab32676e0a858e5a04ff2b8e",
            "A second visually reviewed platform integration figure is required for the category floor.",
        ),
        "hashes": [
            "ed61e3a1276b965eb4e0eac52c3aede6feea0771567e019c1664fe90ca6043f3",
            "686db01abf627f6eac62cab06547a507ee55c00a474610bd12add99dfd179157",
            "8ae929873b7b8162bd7cef827309eee648e73bfbd3b4a5263c51b94a36804e68",
            "46061f5d551e615dcc7308f379b014da873a8114a9e253a3b643e6008e6fa107",
        ],
        "region": ["figure", "picture_0008", "visual_0018"],
        "source_assert": "IOMMUs integration in SoC",
        "projection": "none",
        "present_modalities": ["figure"],
        "cells": [
            residual_cell(
                "static_component_topology",
                ["picture_0008|static_component_topology"],
                "The reviewed figure shows two IOMMU/IO-Bridge placements, system interconnect/memory, root-port/switch/endpoints, ATS, host/data-structure/translation interfaces, and inbound/outbound flows.",
                "Complete visual review of every labeled component, interface, and directed traffic edge in Figure 5 picture_0008.",
            )
        ],
    },
    {
        "key": "1_0_2025_03_12_risc_v_advanced_interrupt_architecture",
        "category": "cpu-isa",
        "source": external(
            "1.0_2025-03-12_RISC_V_Advanced_Interrupt_Architecture.pdf",
            "2d359579dcb84c6d00a1b284db3a7f8ec8c87764d96406a45cbaa9052f04c7c8",
            "A CPU/ISA negative table case is necessary to measure TOC-shape overclassification.",
        ),
        "hashes": [
            "fa59dc4db30db381eb594ea93fe27f4297b61b99984d2cba6e89c0b1e3be1f07",
            "3a952401ffc6bcc6ea614010d262c71247a1b3ffe4bee1772ebb460ba3225550",
            "c68b70a7580a7b55d8ddad9a6767d34db62027c8ba9834317c866ee248b88ce8",
            "182b0759e79f343fa3ae3ee56719c2c14db51364b8cf8054514b1a664e695830",
        ],
        "region": ["table", "table_0004", None],
        "source_assert": "Virtual supervisor top interrupt CSR",
        "projection": "timing_id",
        "present_modalities": ["table"],
        "cells": [
            non_applicable_cell(
                "table_of_contents",
                ["table_0004|toc_non_contract"],
                "All 20 rows are table-of-contents titles plus page numbers; none is a timing parameter or executable fact.",
                "Complete review of all 20 TOC rows in SourceIR table_0004.",
            )
        ],
    },
    {
        "key": "ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification",
        "category": "cpu-isa",
        "source": {
            "location": "repository",
            "relative_path": "corpus/arm/debug/interfaces/adi/current/IHI0074_A_2017-03-09_Arm_Debug_Interface_v6_Architecture_Specification.pdf",
            "sha256": "0e370b75bf96d0e2625ba71474bec8ae8718b21d687f8e0424dcfe3740969c01",
        },
        "hashes": [
            "fddbb31d84bb141814b02990394172dcff6b3e7baf7a7082497255ca0e8e5257",
            "b8c7b401cee2fac508bf1a1c01001c95fe80d813c43efe1be453b5aa0378c5fb",
            "4d6cebf8fdb3b9f7764919d572be5ef28b55226699000de07e5c0e46d7a5f010",
            "82a0d27831e83b1656bcbecc7ea4cc341de00f2cdf4fc255081190e6bf1fa662",
        ],
        "region": ["table", "table_0044", None],
        "source_assert": "JTAG-DP Register access summary",
        "projection": "register_access",
        "present_modalities": ["table"],
        "cells": [
            canonical_table_cell(
                "debug_register_summary",
                [
                    "BASEPTR0|access=RO",
                    "BASEPTR1|access=RO",
                    "CTRL/STAT|access=RW",
                    "DLCR|access=RW",
                    "DLPIDR|access=RO",
                    "DPIDR1|access=RO",
                    "DPIDR|access=RO",
                    "EVENTSTAT|access=RO",
                    "RDBUFF|access=RO",
                    "SELECT1|access=WO b",
                    "SELECT|access=WO b",
                    "TARGETID|access=RO",
                ],
                "The 12 table body rows are JTAG-DP registers and the Access column is part of each reviewed fact.",
                "Complete review of all 12 body rows in SourceIR table_0044.",
            )
        ],
    },
    {
        "key": "opencapi_25gbps_phy_signaling_spec_1_0",
        "category": "physical-link",
        "source": external(
            "OpenCAPI-25Gbps_PHY_Signaling_Spec_1.0.pdf",
            "0e0c8afc1d89189f610dd8340ebb2206270958faa247c6b345c64ee5c8dece12",
            "Physical-link channel requirements are not available from a repository-tracked selected PDF.",
        ),
        "hashes": [
            "b15b4203a28e3faecbcdbf2bf45a7b0a095ffb2baa923d452b9adc6410b91638",
            "928184282b9f0582d5ad403df7dfed938aebefd03f81f00173eb98a5b0740f92",
            "7335772a11925997eb2f35ae5b7178d307ccf95b006b8cce52d2b32fe4cf0ff7",
            "d1f65b764beee49ee26319c62f3663534ab83c28be5f77be3dadd03dd23a7050",
        ],
        "region": ["table", "table_0008", None],
        "source_assert": "Channel requirements",
        "projection": "physical_timing",
        "families": {"T CH_SKEW": "digital_lane_skew", "IL": "analog_channel_loss"},
        "present_modalities": ["table"],
        "cells": physical_cells(
            [
                "T CH_SKEW _Host_RX|max=132|unit=UI",
                "T CH_SKEW _Host_TX|max=10|unit=UI",
            ],
            ["ILD(f)|max=0.45|unit=dB_RMS", "IL(f)|max=21|unit=dB"],
            "Complete review of all four body rows in SourceIR table_0008.",
        ),
    },
    {
        "key": "opencapi_4_0_32g_phy_signal_spec_1_0_16nov2020",
        "category": "physical-link",
        "source": external(
            "OpenCAPI-4.0-32G_PHY_Signal_Spec_1.0_16NOV2020.pdf",
            "d3eb19fc39dd5d1a3925e381bdc9c2f22683d2485d26a447c7ccdeb0080e38be",
            "A second independent physical-link rate/specification is required for the category floor.",
        ),
        "hashes": [
            "ce4cefecde07969f89e29fa86188f5181c3df4c28b161f10620c92e744b1edac",
            "256b453f94712f713fc879553da03baea629331166598df17910384bdfccf3f5",
            "6e43e8cd25acf7aa99b8c3bcf764b9119619afb9c7ec5e0570927a099b2c9059",
            "08a362677545057b928ab6351ae149baa443defdc129e86f64fb60a5d9371707",
        ],
        "region": ["table", "table_0009", None],
        "source_assert": "Channel requirements",
        "projection": "physical_timing",
        "families": {"T CH_SKEW": "digital_lane_skew", "IL": "analog_channel_loss"},
        "present_modalities": ["table"],
        "cells": physical_cells(
            [
                "T CH_SKEW _Host_RX|max=132|unit=UI",
                "T CH_SKEW _Host_TX|max=12.5|unit=UI",
            ],
            ["IL(f)|max=30|unit=dB"],
            "Complete review of all three body rows in SourceIR table_0009.",
        ),
    },
    {
        "key": "pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide",
        "category": "methodology-guide",
        "source": external(
            "PJDOC-466751330-7215_10.0_Cortex_A76_Software_Optimization_Guide.pdf",
            "8358c5ae0d4b9fbc713b40dfec840d76e9ab3ce57cdf66badbdb41e0e60b3a22",
            "A genuine software-optimization recommendation is required for guide non-contract coverage.",
        ),
        "hashes": [
            "7a9e81cd878a759ad5b1585451b23c977516f6a5eca32c63ef66446a31bb0387",
            "201984f779d5a5b830a9c57587b4b75af530ae0d048450d2054911d0cf06cc03",
            "e8755034f360db40313f30de4814f8a7bd323fa323f524c96b01b75599efd4c5",
            "6fa2f0f6629a8aeafb4e8dde864f84565ec30265ba24c012f5d825d8a0e3fe0b",
        ],
        "region": ["prose", "elem_00219", "statement_0843"],
        "source_assert": "it is recommended that GPR registers be filled/spilled to the VPR",
        "projection": "none",
        "present_modalities": ["prose"],
        "cells": [
            non_applicable_cell(
                "software_guidance",
                ["elem_00219|software_guidance"],
                "This is performance advice to keep GPR/VPR transfers out of memory, not an executable hardware contract.",
                "Complete review of the one recommendation in SourceIR elem_00219.",
            )
        ],
    },
    {
        "key": "198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide",
        "category": "methodology-guide",
        "source": external(
            "198123_0302_03_2025-04-22_Generic_Interrupt_Controller_Overview_Guide.pdf",
            "5358701e080664f2db98fb9d066a0c90a4deb2ba34ca35c81597332534707e97",
            "A second guide source is required to test explicit informational-only disposition.",
        ),
        "hashes": [
            "421166d5f13c914229462e034cc0311e90b0891e9cc783fa08f5f9d364181dd5",
            "6468c0a74ddcad621ff79214a92c339aa8853db5a6998886df9238c49ab286c9",
            "8611c60789f3d70bc6c285e9a863087adff754695d7a049675ff7beb9aa1bde3",
            "b6142cc482722dc5dec95fba0ed55cc53adfe559af1ce58b987b80b579a0389b",
        ],
        "region": ["prose", "elem_00017", "statement_0013"],
        "source_assert": "The content of this document is informational only",
        "projection": "none",
        "present_modalities": ["prose"],
        "cells": [
            non_applicable_cell(
                "informational_disclaimer",
                ["elem_00017|informational_non_contract"],
                "The region explicitly says the document is informational and disclaims normative obligation.",
                "Complete review of the informational-only disclaimer in SourceIR elem_00017.",
            )
        ],
    },
]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def stage_paths(key: str, replay_root: Path | None = None) -> list[Path]:
    if replay_root is not None:
        base = ROOT / replay_root / "replays" / key
        return [
            base / "source_ir" / key / "source_ir.json",
            base / "evidence_ir" / key / "evidence_ir.json",
            base / "semantic_ir" / key / "semantic_ir.json",
            base / "intent_ir" / key / "intent_ir.json",
        ]
    return [
        ROOT / "generated/source_ir" / key / "source_ir.json",
        ROOT / "generated/evidence_ir" / key / "evidence_ir.json",
        ROOT / "generated/semantic_ir" / key / "semantic_ir.json",
        ROOT / "generated/intent_ir" / key / "intent_ir.json",
    ]


def read_json(path: Path) -> dict:
    with path.open(encoding="utf-8") as stream:
        return json.load(stream)


def source_record(source: dict, spec: dict, require_reviewed_region: bool) -> dict | None:
    kind, region_id, _ = spec["region"]
    if kind == "prose":
        records = [
            item for item in source["content_elements"] if item["element_id"] == region_id
        ]
        if len(records) != 1 or spec["source_assert"] not in records[0]["text"]:
            assert not require_reviewed_region, f"{spec['key']} reviewed source region disappeared"
            return None
        excerpt = records[0]["text"]
    elif kind == "table":
        records = [item for item in source["structured_tables"] if item["table_id"] == region_id]
        if len(records) != 1 or spec["source_assert"] not in json.dumps(records[0]):
            assert not require_reviewed_region, f"{spec['key']} reviewed source region disappeared"
            return None
        excerpt = {
            "caption": records[0].get("caption_text"),
            "row_count": records[0]["row_count"],
            "col_count": records[0]["col_count"],
        }
    else:
        records = [item for item in source["visual_assets"] if item["asset_id"] == region_id]
        if len(records) != 1 or spec["source_assert"] not in records[0]["caption_text"]:
            assert not require_reviewed_region, f"{spec['key']} reviewed source region disappeared"
            return None
        excerpt = records[0]["caption_text"]
    return {
        "region_id": region_id,
        "modality": spec["present_modalities"][0],
        "source_excerpt": excerpt,
        "reviewed_source_facts": sorted(
            {fact for cell in spec["cells"] for fact in cell["gold"] + cell["residual_gold"]}
        ),
    }


def source_ids(record: dict) -> list[str]:
    return sorted(
        set(record.get("supporting_statement_ids", []))
        | set(record.get("supporting_table_ids", []))
    )


def timing_key(record: dict, full: bool) -> str:
    if full:
        return (
            f'{record["parameter_name"]}|min={record.get("min_value", MISSING)}'
            f'|typ={record.get("typ_value", MISSING)}|max={record.get("max_value", MISSING)}'
            f'|unit={record.get("unit", MISSING)}'
        )
    return (
        f'{record["parameter_name"]}|max={record.get("max_value", MISSING)}'
        f'|unit={record.get("unit", MISSING)}'
    )


def timing_is_canonical(record: dict) -> bool:
    return record.get("intent_disposition", {}).get("status", "canonical") == "canonical"


def physical_timing_family(record: dict, spec: dict) -> str:
    return next(
        value
        for prefix, value in spec["families"].items()
        if record["parameter_name"].startswith(prefix)
    )


def project_canonical(stage: dict, spec: dict) -> list[dict]:
    region_id = spec["region"][1]
    projection = spec["projection"]
    projected = []
    if projection == "apb_signal":
        records = [
            item
            for item in stage["signal_constraints"]
            if spec["region"][2] in item.get("supporting_statement_ids", [])
        ]
        for item in records:
            kind = item["constraint_kind"]
            projected.append(
                fact(
                    region_id,
                    "setup_signal_state",
                    f'{item["subject_signal"]}|{kind["kind"]}|{kind.get("value", MISSING)}',
                    source_ids(item),
                )
            )
    elif projection.startswith("timing") or projection == "physical_timing":
        records = [
            item
            for item in stage["timing_constraints"]
            if item["constraint_id"].startswith(f"timing_{region_id}_")
            and timing_is_canonical(item)
        ]
        for item in records:
            if projection == "timing_full":
                family, key = "receiver_timing", timing_key(item, True)
            elif projection == "timing_id":
                family, key = "table_of_contents", item["constraint_id"]
            else:
                family = physical_timing_family(item, spec)
                key = timing_key(item, False)
            projected.append(fact(region_id, family, key, source_ids(item)))
    elif projection.startswith("register"):
        records = [
            item
            for item in stage["register_records"]
            if item["register_id"].startswith((f"reg_{region_id}_", f"regfld_{region_id}"))
        ]
        family = spec["cells"][0]["family"]
        for item in records:
            if projection == "register_access_offset":
                key = (
                    f'{item["register_name"]}|access={item.get("access_type", MISSING)}'
                    f'|offset={item.get("offset_address", MISSING)}'
                )
            elif projection == "register_access":
                key = f'{item["register_name"]}|access={item.get("access_type", MISSING)}'
            else:
                key = item["register_name"]
            projected.append(fact(region_id, family, key, source_ids(item)))
    elif projection != "none":
        raise AssertionError(f"unknown projection {projection}")
    return sorted(projected, key=lambda item: (item["family"], item["fact_key"]))


def project_residuals(stage: dict, spec: dict) -> list[dict]:
    """Project every production residual carrier that can explain this reviewed region.

    Each carrier owns its own applicability test instead of sharing one priority chain, so a region
    two carriers could explain is projected by both rather than silently by whichever branch is
    written first.
    """
    projected = project_non_applicable_timing(stage, spec) + project_captured_regions(stage, spec)
    return sorted(projected, key=lambda item: (item["family"], item["fact_key"]))


def project_non_applicable_timing(stage: dict, spec: dict) -> list[dict]:
    """Project the scalar timing carrier: rows production placed outside digital intent."""
    if spec["projection"] != "physical_timing":
        return []
    region_id = spec["region"][1]
    projected = []
    for item in stage["timing_constraints"]:
        if not item["constraint_id"].startswith(f"timing_{region_id}_"):
            continue
        disposition = item.get("intent_disposition", {})
        if disposition.get("status", "canonical") != "non_applicable":
            continue
        record = fact(
            region_id,
            physical_timing_family(item, spec),
            timing_key(item, False),
            source_ids(item),
        )
        for field in ACTIONABILITY_FIELDS:
            record[field] = disposition.get(field, "")
        projected.append(record)
    return projected


def project_captured_regions(stage: dict, spec: dict) -> list[dict]:
    """Project the captured-region carrier: a captured visual region no canonical record cites.

    The carrier is region-scoped rather than fact-scoped, which is exactly the question a reviewed
    figure cell asks — was this region's disposition accounted for. The reviewed family names the
    key; region identity, `EvidenceIR` provenance, and the three actionability fields are carried
    verbatim from the production record. A region a canonical record does cite earns no carrier and
    therefore projects nothing, so an explained region can never read as an accounted one.
    """
    if spec["region"][0] != "figure":
        return []
    region_id = spec["region"][1]
    projected = []
    for item in stage.get("captured_region_residuals", []):
        if item["region_id"] != region_id:
            continue
        for cell in spec["cells"]:
            record = fact(
                region_id,
                cell["family"],
                f'{region_id}|{cell["family"]}',
                sorted(set(item["supporting_evidence_ids"])),
            )
            for field in ACTIONABILITY_FIELDS:
                record[field] = item.get(field, "")
            projected.append(record)
    return projected


def fact(region_id: str, family: str, key: str, provenance: list[str]) -> dict:
    return {
        "region_id": region_id,
        "family": family,
        "fact_key": key,
        "source_ids": provenance,
    }


def capture_records(evidence: dict, spec: dict, require_reviewed_capture: bool) -> list[dict]:
    kind, region_id, evidence_id = spec["region"]
    if kind == "prose":
        found = any(
            item["statement_id"] == evidence_id for item in evidence["extracted_statements"]
        )
    elif kind == "figure":
        found = any(item["evidence_id"] == evidence_id for item in evidence["visual_evidence"])
    else:
        found = bool(project_canonical(evidence, spec))
    if not found:
        assert not require_reviewed_capture, f"{spec['key']} reviewed evidence capture disappeared"
        return []
    return [
        {
            "region_id": region_id,
            "family": cell["family"],
            "capture_key": cell["family"],
            "source_ids": [evidence_id or region_id],
        }
        for cell in spec["cells"]
    ]


def predicates(region_id: str, family: str) -> list[dict]:
    return [
        {"operator": "equals", "field": "/region_id", "value": region_id},
        {"operator": "equals", "field": "/family", "value": family},
    ]


def query(collection: str, region_id: str, family: str, key: str, expected: list[str], provenance: list[str] | None = None) -> dict:
    result = {
        "collection_pointer": collection,
        "predicates": predicates(region_id, family),
        "key_fields": [key],
        "expected_keys": sorted(expected),
    }
    if provenance:
        result["provenance_fields"] = ["/source_ids"]
        result["required_provenance_ids"] = provenance
    return result


def build_cell(spec: dict, index: int, cell: dict) -> dict:
    region_id, evidence_id = spec["region"][1], spec["region"][2]
    provenance_id = evidence_id or region_id
    result = {
        "cell_id": f'{spec["key"]}__{index:02d}_{cell["family"]}',
        "semantic_family": cell["family"],
        "modality": spec["present_modalities"][0],
        "oracle": cell["oracle"],
        "review_scope": cell["scope"],
        "complete_gold": True,
        "expected_disposition": cell["disposition"],
        "source_region": {
            "collection_pointer": "/regions",
            "predicates": [
                {"operator": "equals", "field": "/region_id", "value": region_id}
            ],
            "key_fields": ["/region_id"],
            "expected_keys": [region_id],
        },
        "evidence_capture": query(
            "/captures", region_id, cell["family"], "/capture_key", [cell["family"]]
        ),
        "canonical": {
            stage: query(
                "/canonical",
                region_id,
                cell["family"],
                "/fact_key",
                cell["gold"],
                [provenance_id],
            )
            for stage in ("evidence_ir", "semantic_ir", "intent_ir")
        },
    }
    if cell["residual_gold"]:
        result["residual"] = {
            stage: query(
                "/residuals",
                region_id,
                cell["family"],
                "/fact_key",
                cell["residual_gold"],
                [provenance_id],
            )
            for stage in ("semantic_ir", "intent_ir")
        }
        result["residual"]["required_actionability_fields"] = [
            "/reason",
            "/first_failing_stage",
            "/replay",
        ]
    return result


def build_document(spec: dict, replay_root: Path | None = None) -> dict:
    paths = stage_paths(spec["key"], replay_root)
    actual_hashes = [sha256(path) for path in paths]
    frozen = replay_root is None
    if frozen:
        assert actual_hashes == spec["hashes"], f"{spec['key']} stage identity drift"
    stages = [read_json(path) for path in paths]

    if frozen and spec["source"]["location"] == "repository":
        source_path = ROOT / spec["source"]["relative_path"]
        assert sha256(source_path) == spec["source"]["sha256"]
        assert stages[0]["source"]["canonical_path"] == spec["source"]["relative_path"]
    elif frozen:
        source_path = Path(stages[0]["source"]["canonical_path"])
        assert source_path.name == spec["source"]["portable_id"]
        assert sha256(source_path) == spec["source"]["sha256"]
    else:
        persisted_source = Path(stages[0]["source"]["canonical_path"])
        assert not persisted_source.is_absolute(), f"{spec['key']} replay persisted an absolute source"
        source_path = (ROOT / persisted_source).resolve()
        source_path.relative_to(ROOT)
        assert source_path.name == Path(
            spec["source"].get("relative_path", spec["source"].get("portable_id"))
        ).name
        assert sha256(source_path) == spec["source"]["sha256"]

    region = source_record(stages[0], spec, frozen)
    evidence_captures = capture_records(stages[1], spec, frozen)
    snapshots = [
        {
            "snapshot_format": "reviewed_bounded_projection_v1",
            "regions": [] if region is None else [region],
        },
        {
            "snapshot_format": "reviewed_bounded_projection_v1",
            "captures": evidence_captures,
            "canonical": project_canonical(stages[1], spec),
            "residuals": [],
        },
        {
            "snapshot_format": "reviewed_bounded_projection_v1",
            "canonical": project_canonical(stages[2], spec),
            "residuals": project_residuals(stages[2], spec),
        },
        {
            "snapshot_format": "reviewed_bounded_projection_v1",
            "canonical": project_canonical(stages[3], spec),
            "residuals": project_residuals(stages[3], spec),
        },
    ]
    return {
        "document_key": spec["key"],
        "category": spec["category"],
        "source": spec["source"],
        "review_scope_complete": True,
        "present_modalities": spec["present_modalities"],
        "stages": {
            name: {"original_sha256": digest, "snapshot": snapshot}
            for name, digest, snapshot in zip(
                ("source_ir", "evidence_ir", "semantic_ir", "intent_ir"),
                spec["hashes"],
                snapshots,
                strict=True,
            )
        },
        "cells": [build_cell(spec, index, cell) for index, cell in enumerate(spec["cells"], 1)],
    }


def replay_authority_identifier(value: str | None, label: str) -> str:
    if (
        value is None
        or not value
        or len(value) > 128
        or any(
            not character.isascii()
            or not (character.isalnum() or character in "._-")
            for character in value
        )
    ):
        raise ValueError(f"{label} must be a non-empty portable identifier")
    return value


def build(
    replay_root: Path | None = None,
    replay_dataset_id: str | None = None,
    replay_owner: str | None = None,
) -> bytes:
    current_replay = replay_root is not None
    if current_replay:
        replay_dataset_id = replay_authority_identifier(replay_dataset_id, "replay dataset id")
        replay_owner = replay_authority_identifier(replay_owner, "replay owner")
    dataset = {
        "schema_version": 1,
        "dataset_id": (
            replay_dataset_id
            if current_replay
            else "source-to-intent-vertical-reviewed-v1"
        ),
        "owner": (
            replay_owner
            if current_replay
            else "SPEC-TO-INTENT-ALIGNMENT.4b"
        ),
        "selection_boundary_commit": SELECTION_COMMIT,
        "selection_claim": (
            "Current-binary replay of the unchanged review-locked population; source bytes and all "
            "four replayed stage identities are re-verified without mutating the frozen baseline."
            if current_replay
            else "Retrospective corpus baseline frozen before gold construction and held out from extractor "
            "changes at or after the selection boundary; it is not claimed historically untouched."
        ),
        "minimum_documents_per_category": 2,
        "documents": [build_document(spec, replay_root) for spec in DOCS],
    }
    return (json.dumps(dataset, indent=2, ensure_ascii=False) + "\n").encode()


def main() -> int:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--write-replay", action="store_true")
    parser.add_argument("--replay-root", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--replay-dataset-id")
    parser.add_argument("--replay-owner")
    args = parser.parse_args()
    if args.write_replay:
        if (
            args.replay_root is None
            or args.output is None
            or args.replay_dataset_id is None
            or args.replay_owner is None
        ):
            parser.error(
                "--write-replay requires --replay-root, --output, --replay-dataset-id, and --replay-owner"
            )
        if (
            args.replay_root.is_absolute()
            or args.output.is_absolute()
            or ".." in args.replay_root.parts
            or ".." in args.output.parts
            or args.replay_root.parts[:2] != (".project-data", "tmp")
        ):
            parser.error("replay root and output must be safe repository-relative project scratch paths")
        replay_absolute = (ROOT / args.replay_root).resolve(strict=True)
        replay_absolute.relative_to((ROOT / ".project-data/tmp").resolve(strict=True))
        destination = (ROOT / args.output).resolve()
        destination.relative_to(replay_absolute)
        rendered = build(args.replay_root, args.replay_dataset_id, args.replay_owner)
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(rendered)
        print(f"wrote {args.output} ({len(rendered)} bytes)")
        return 0
    if (
        args.replay_root is not None
        or args.output is not None
        or args.replay_dataset_id is not None
        or args.replay_owner is not None
    ):
        parser.error("replay-only arguments are valid only with --write-replay")
    rendered = build()
    destination = ROOT / FIXTURE
    if args.write:
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(rendered)
        print(f"wrote {FIXTURE} ({len(rendered)} bytes)")
        return 0
    if not destination.is_file() or destination.read_bytes() != rendered:
        print(f"{FIXTURE} is stale; run {Path(__file__).relative_to(ROOT)} --write", file=sys.stderr)
        return 1
    print(f"{FIXTURE} matches the 12-document reviewed selection boundary")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
