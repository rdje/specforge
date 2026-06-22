#!/usr/bin/env python3
"""DOC-INTENT-TAXONOMY.2 — per-category ISF-lowering completeness gauge.

READ-ONLY measurement. Reads the persisted `generated/{evidence_ir,intent_ir}/<key>/*.json`
plus the `generated/adapters/isf/<key>/adapter.json` lowering artifacts and reports, per
purpose category (DOC-INTENT-TAXONOMY.0), how much of each document's typed intent reaches
the emitted `.isf` — and classifies every non-lowered surface as an HONEST RESIDUAL (the
document carries nothing there, or the surface is config/non-synthesizable metadata) vs a
TRUE GAP (the intent exists but no Evidence->Intent carrier and/or no ISF construct lowers it).

It mutates nothing (no `validate`, no `adapt` write). For the two docs whose adapter.json was
never materialized, pass their read-only `adapt --dry-run` JSON via --extra-adapter KEY=PATH.

The per-document category labels below are a one-off MEASUREMENT LABEL set (reconstructing the
DOC-INTENT-TAXONOMY.1 ground truth, which was not persisted), NOT shipped code: the runtime
recognizer (.3) must be structural per ADR 0006. Distribution matches .1: 36/7/15/2/4/14.
"""
from __future__ import annotations
import json, sys, argparse
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GEN = ROOT / "generated"

# --- category labels (measurement-only; see docstring). 1..6 per DOC-INTENT-TAXONOMY.0 ---
CATEGORY = {
    # cat 1 — wire-level bus / interconnect protocol (36)
    "683091_18_1_2021_12_25_avalon_interface_specifications": 1,
    "bosch_can_specification_2_0_1991": 1,
    "ccix_base_specification_r1_0_v1_0": 1,
    "ccix_base_specification_r1_0a_v1_0_for_evaluation": 1,
    "ccix_base_specification_rev_2_0_v1_0": 1,
    "ccix_base_specification_revision1_1_version1_0": 1,
    "etsi_ts_102613_v16_0_0_2021_10_smart_cards_uicc_clf_single_wire_protocol_swp": 1,
    "ihi0022_h_c_2021_01_amba_axi_and_ace_protocol_specification": 1,
    "ihi0022_l_2025_08_amba_axi_protocol_specification": 1,
    "ihi0024_d_2021_04_amba_apb_protocol_specification": 1,
    "ihi0024_e_2023_02_amba_5_apb_protocol_specification": 1,
    "ihi0032_c_2021_12_amba_trace_bus_protocol_specification": 1,
    "ihi0033_c_2021_09_amba_5_ahb_protocol_specification": 1,
    "ihi0050_g_2024_03_amba_chi_architecture_specification": 1,
    "ihi0051_b_2021_04_amba_axi_stream_protocol_specification": 1,
    "ihi0068_d_2021_10_amba_low_power_interface_specification": 1,
    "ihi0076_a_2018_05_02_advanced_communications_channel_architecture_specification": 1,
    "ihi0079_b_2020_12_amba_cxs_protocol_specification": 1,
    "ihi0082_a_2019_03_15_amba_adaptive_traffic_profiles_specification": 1,
    "ihi0083_a_b_2022_05_amba_5_generic_flash_bus_interface_specification": 1,
    "ihi0088_g_2024_06_amba_dti_protocol_specification": 1,
    "ihi0089_d_2025_08_amba_lti_protocol_specification": 1,
    "ihi0098_a_2024_02_07_amba_chi_chip_to_chip_c2c_architecture_specification": 1,
    "ihi0098_a_b_2026_02_03_amba_chi_chip_to_chip_c2c_architecture_specification": 1,
    "ihi0098_b_2026_03_23_amba_chi_chip_to_chip_c2c_architecture_specification": 1,
    "jesd84_b50_2013_09_emmc_5_0": 1,
    "opencapi_3_0_transaction_layer_28jan2020": 1,
    "opencapi_3_1_transaction_layer_28jan2020": 1,
    "opencapi_4_0_transactionlayer_arch_16jun2020": 1,
    "opencapi_data_link_layer_v20_09jul2020": 1,
    "smbus_3_3_1_2024_10_20_system_management_bus_specification": 1,
    "tilelink_1_7_1_specification": 1,
    "tilelink_1_8_0_specification": 1,
    "um10204_rev7_0_2021_i2c_bus_specification": 1,
    "um11732_v3_2022_02_17_i2s_bus_specification": 1,
    "wbspec_b4_wishbone_b4_specification": 1,
    # cat 2 — programmable register / memory-mapped IP (7)
    "1_0_1_2026_02_22_risc_v_iommu_architecture_specification": 2,
    "48882_pub_3_10_2025_02_amd_io_virtualization_technology_iommu_specification": 2,
    "5_0_2024_08_intel_virtualization_technology_for_directed_io_specification": 2,
    "jesd235_2013_10_hbm_dram": 2,
    "jesd235a_2015_11_hbm2_dram": 2,
    "nvme_base_specification_2_0a_2021_07_26": 2,
    "opencapi_discovery_configuration_v201": 2,
    # cat 3 — platform / system-IP topology & integration (15)
    "100336_0106_00_2019_02_08_gic_600_technical_reference_manual": 3,
    "100798_0401_00_2020_07_31_cortex_a76_technical_reference_manual": 3,
    "100806_0100_00_2017_08_01_coresight_soc_600_technical_reference_manual": 3,
    "100806_0200_00_2017_12_08_coresight_soc_600_technical_reference_manual": 3,
    "100806_0701_17_2025_06_30_coresight_soc_600_technical_reference_manual": 3,
    "101130_0002_02_2018_11_30_coresight_sdc_600_technical_reference_manual": 3,
    "101542_0102_08_2023_07_05_arm_mmu_700_technical_reference_manual": 3,
    "ddi0461_b_2010_12_10_coresight_trace_memory_controller_technical_reference_manual": 3,
    "ddi0471_a_2011_06_23_gic_400_technical_reference_manual": 3,
    "den0068_2018_07_23_coresight_base_system_architecture": 3,
    "ihi0029_e_2017_02_27_coresight_architecture_specification": 3,
    "ihi0069_g_2021_02_arm_generic_interrupt_controller_architecture_specification": 3,
    "ihi0070_e_a_2023_01_31_arm_system_memory_management_unit_architecture_specification": 3,
    "ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification": 3,
    "lpc_memory_agent_reference_design_guide_17jul2020": 3,
    # cat 4 — CPU ISA / privileged architecture (2)
    "1_0_2025_03_12_risc_v_advanced_interrupt_architecture": 4,
    "1_0_risc_v_debug_specification": 4,
    # cat 5 — physical / electrical / link layer (4)
    "opencapi_25gbps_phy_mechanical_spec_v10": 5,
    "opencapi_25gbps_phy_signaling_spec_1_0": 5,
    "opencapi_4_0_32g_phy_signal_spec_1_0_16nov2020": 5,
    "opencapi_4_0_32gbps_phy_mech_spec_v10_17mar2021": 5,
    # cat 6 — methodology / language / EDA standard / guide (14)
    "102196_0100_01_2022_05_05_aarch64_external_debug_guide": 6,
    "102520_0101_01_2025_09_15_introducing_coresight_debug_and_trace": 6,
    "109242_0100_01_2023_09_04_arm_smmu_software_guide": 6,
    "198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide": 6,
    "den0034_a_2013_09_13_debug_and_trace_configuration_and_usage_models": 6,
    "opencapi_3_0_certified_definition_v1_1": 6,
    "opencapi_3_0_certified_test_resources_engineering_note_v1_0": 6,
    "opencapi_3_0_ready_definition_v1_1": 6,
    "opencapi_3_0_ready_test_resources_engineering_note_v1_0": 6,
    "opencapi_afu_address_space_usage": 6,
    "pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide": 6,
    "readme": 6,
    "usb4_connection_manager_guide_v2_0_2025_11": 6,
    "usb4_inter_domain_service_specification_v2_0_2025_11": 6,
}

CAT_NAME = {
    1: "wire-protocol", 2: "register-IP", 3: "platform-system-IP",
    4: "CPU-ISA", 5: "PHY", 6: "methodology-guide",
}
NON_TARGET = {5, 6}


def load(p: Path):
    try:
        return json.loads(p.read_text())
    except Exception:
        return None


def alen(d, k):
    v = (d or {}).get(k)
    return len(v) if isinstance(v, list) else 0


def measure(key: str, extra_adapters: dict):
    ev = load(GEN / "evidence_ir" / key / "evidence_ir.json")
    it = load(GEN / "intent_ir" / key / "intent_ir.json")
    ad = None
    ap = GEN / "adapters" / "isf" / key / "adapter.json"
    if ap.exists():
        ad = load(ap)
    elif key in extra_adapters:
        ad = load(Path(extra_adapters[key]))
    isf = (ad or {}).get("isf", {}) if ad else {}

    reg_fields = 0
    for r in (it or {}).get("register_records", []) or []:
        reg_fields += len(r.get("fields", []) or [])

    return {
        "key": key,
        "cat": CATEGORY.get(key, 0),
        # --- intent-bearing surfaces (the denominator) ---
        "ev_msgfld": alen(ev, "message_field_records"),
        "ev_presence": alen(ev, "signal_presence_records"),
        "in_interfaces": alen(it, "interfaces"),
        "in_actor_ports": alen(it, "actor_ports"),
        "in_reg": alen(it, "register_records"),
        "in_reg_fields": reg_fields,
        "in_txn": alen(it, "transactions"),
        "in_rel": alen(it, "actor_signal_relations"),
        "in_sigc": alen(it, "signal_constraints"),
        "in_temporal": alen(it, "temporal_rules"),
        "in_cond": alen(it, "conditional_rules"),
        "in_symdef": alen(it, "symbol_definitions"),
        # --- ISF lowered (the numerator) ---
        "have_adapter": ad is not None,
        "isf_status": (ad or {}).get("lowering_status", "MISSING"),
        "isf_render": bool(isf.get("is_renderable", False)),
        "isf_sig": isf.get("signal_count", 0),
        "isf_storage": isf.get("storage_count", 0),
        "isf_enum": isf.get("enum_count", 0),
        "isf_rule": isf.get("rule_count", 0),
        "isf_txn": isf.get("transaction_count", 0),
    }


# Surface lowering ledger: for each typed surface, classify presence vs lowering.
# Returns list of (surface, present_count, lowered_count, verdict) where verdict in
# {"lowered","partial","true_gap","honest_residual","absent"}.
def ledger(m):
    rows = []

    def add(name, present, lowered, gap_kind):
        if present == 0:
            rows.append((name, 0, 0, "absent"))
        elif lowered >= present:
            rows.append((name, present, lowered, "lowered"))
        elif lowered > 0:
            rows.append((name, present, lowered, "partial"))
        else:
            rows.append((name, present, 0, gap_kind))

    signals_present = max(m["in_interfaces"], m["in_actor_ports"])
    add("signals", signals_present, m["isf_sig"], "true_gap")
    add("registers->storage", m["in_reg"], m["isf_storage"], "true_gap")
    # register fields: NO ISF field/bitfield construct (registers lower as opaque width-only var)
    rows.append(("register-fields", m["in_reg_fields"], 0,
                 "true_gap" if m["in_reg_fields"] else "absent"))
    add("enums", m["in_symdef"], m["isf_enum"], "true_gap")
    # constraints + temporal + conditional all lower into the single rule surface
    rule_src = m["in_sigc"] + m["in_temporal"] + m["in_cond"]
    add("constraints+temporal+conditional->rules", rule_src, m["isf_rule"], "true_gap")
    # transactions: only those with composed steps lower; the recognition/signal-set/channel
    # metadata is a structured residual (orthogonal to .isf, by design until bodies compose)
    add("transactions->steps", m["in_txn"], m["isf_txn"], "true_gap")
    # message-field structures: present at EvidenceIR, NO Intent carrier, NO ISF construct
    rows.append(("message-field-structures", m["ev_msgfld"], 0,
                 "true_gap" if m["ev_msgfld"] else "absent"))
    # signal-presence matrices: config/presence metadata, not synthesizable wire intent
    rows.append(("signal-presence(config)", m["ev_presence"], 0,
                 "honest_residual" if m["ev_presence"] else "absent"))
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--extra-adapter", action="append", default=[],
                    help="KEY=PATH for a read-only dry-run adapter.json (unmaterialized docs)")
    args = ap.parse_args()
    extra = {}
    for spec in args.extra_adapter:
        k, _, p = spec.partition("=")
        extra[k] = p

    keys = sorted(d.name for d in (GEN / "intent_ir").iterdir() if d.is_dir())
    rows = [measure(k, extra) for k in keys]

    # distribution checksum
    dist = {c: 0 for c in range(1, 7)}
    for m in rows:
        dist[m["cat"]] = dist.get(m["cat"], 0) + 1
    print("== distribution (checksum vs .1 = 36/7/15/2/4/14) ==")
    for c in range(1, 7):
        print(f"  cat{c} {CAT_NAME[c]:<20} {dist[c]}")
    print(f"  total {sum(dist.values())}")
    unlabeled = [m["key"] for m in rows if m["cat"] == 0]
    if unlabeled:
        print("  !! UNLABELED:", unlabeled)

    # per-category rollup of the surface ledger
    print("\n== per-category surface lowering rollup ==")
    print("   (present = docs/units with that intent; lowered/partial/gap = verdict tally)")
    for c in range(1, 7):
        cat_rows = [m for m in rows if m["cat"] == c]
        if not cat_rows:
            continue
        print(f"\n--- cat{c} {CAT_NAME[c]} ({len(cat_rows)} docs)"
              + ("  [NON-TARGET]" if c in NON_TARGET else "") + " ---")
        # aggregate present-unit and lowered-unit counts per surface
        agg = {}
        verdict_docs = {}
        for m in cat_rows:
            for name, present, lowered, verdict in ledger(m):
                a = agg.setdefault(name, [0, 0])
                a[0] += present
                a[1] += lowered
                if present > 0:
                    vd = verdict_docs.setdefault(name, {})
                    vd[verdict] = vd.get(verdict, 0) + 1
        order = ["signals", "registers->storage", "register-fields", "enums",
                 "constraints+temporal+conditional->rules", "transactions->steps",
                 "message-field-structures", "signal-presence(config)"]
        for name in order:
            if name not in agg:
                continue
            present, lowered = agg[name]
            vds = verdict_docs.get(name, {})
            vd_str = " ".join(f"{k}:{v}" for k, v in sorted(vds.items()))
            print(f"  {name:<42} units present={present:<6} lowered={lowered:<6} | docs[{vd_str}]")

    # per-doc table for buildable categories (1-4) — per-item demonstration
    print("\n== per-document detail (buildable cats 1-4) ==")
    hdr = ("cat", "key", "sig", "reg", "rfld", "txn", "msgfld", "rule_src",
           "| isf:", "sig", "stor", "enum", "rule", "txn", "status")
    for c in (1, 2, 3, 4):
        print(f"\n--- cat{c} {CAT_NAME[c]} ---")
        for m in sorted((x for x in rows if x["cat"] == c), key=lambda z: z["key"]):
            rule_src = m["in_sigc"] + m["in_temporal"] + m["in_cond"]
            print(f"  {m['key'][:52]:<52} "
                  f"sig={max(m['in_interfaces'], m['in_actor_ports']):<5} "
                  f"reg={m['in_reg']:<4} rfld={m['in_reg_fields']:<5} txn={m['in_txn']:<4} "
                  f"msgfld={m['ev_msgfld']:<5} rule_src={rule_src:<5} "
                  f"|| isf sig={m['isf_sig']:<5} stor={m['isf_storage']:<5} "
                  f"enum={m['isf_enum']:<4} rule={m['isf_rule']:<5} txn={m['isf_txn']:<4} "
                  f"{m['isf_status']}{'' if m['have_adapter'] else '(dry-run)'}")

    # non-target sanity: are cat5/6 .isf correctly near-empty? flag over-extraction.
    print("\n== non-target (cat5/6) .isf surface — should be near-empty ==")
    for c in (5, 6):
        for m in sorted((x for x in rows if x["cat"] == c), key=lambda z: -(z["isf_sig"] + z["isf_txn"])):
            flag = " <-- OVER-EXTRACTION" if (m["isf_sig"] + m["isf_txn"]) >= 20 else ""
            print(f"  cat{c} {m['key'][:50]:<50} isf sig={m['isf_sig']:<4} txn={m['isf_txn']:<3} "
                  f"rule={m['isf_rule']:<3}{flag}")


if __name__ == "__main__":
    main()
