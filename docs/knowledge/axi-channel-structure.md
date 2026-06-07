---
id: axi-channel-structure
title: AXI is channel-organized — each channel (AW/W/B/AR/R/AC) has its own VALID/READY + payload signals
answers:
  - "how is AXI organized / what are the AXI channels"
  - "what signals belong to which AXI channel"
  - "how should an AXI gold or extraction be structured (per channel)"
  - "what is the AXI signal naming convention (channel prefix)"
  - "how many signals does each AXI channel have"
date: 2026-06-07
tags: [axi, domain, wire-based-100, channels]
evidence: corpus/arm/amba/core/axi/current/IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf; generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification (310 declared signals)
reverify: python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); d={re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}; print(sorted(x for x in d if x.startswith('AW'))[:10])"
---

AXI is **channel-organized** (owner: "AXi is made of channels and each channel has its set of signals").
Every signal belongs to a channel identified by its **name prefix**, and each channel is a
VALID/READY-handshaked group with a payload:

- **AW** — Write Address (~55 signals): `AWVALID`/`AWREADY` + `AWADDR`/`AWLEN`/`AWSIZE`/`AWBURST`/`AWLOCK`/`AWCACHE`/`AWPROT`/`AWID`/`AWQOS`/`AWUSER`/… (AXI5 adds many: `AWATOP`/`AWCMO`/`AWMMU*`/`AWSTASH*`/…).
- **W** — Write Data (~15): `WVALID`/`WREADY` + `WDATA`/`WSTRB`/`WLAST`/`WUSER`/`WPOISON`/…
- **B** — Write Response (~20): `BVALID`/`BREADY` + `BRESP`/`BID`/`BUSER`/`BCOMP`/…
- **AR** — Read Address (~50): `ARVALID`/`ARREADY` + `ARADDR`/`ARLEN`/`ARSIZE`/`ARBURST`/`ARID`/`ARCHUNKEN`/…
- **R** — Read Data (~18): `RVALID`/`RREADY` + `RDATA`/`RRESP`/`RID`/`RLAST`/`RCHUNKV`/`RCHUNKNUM`/…
- **AC** / system — Snoop/DVM + activation (~13): `ACVALID`/`ACREADY`/`ACADDR`, `ACTIVATEREQ`/`ACTIVATEACK`/`ASKSTOP`, `SYSCOREQ`/`SYSCOACK`, `ACLK`.

Implications for WIRE-BASED-100 AXI work: an AXI gold should cover signals **per channel** (the
handshake VALID/READY + representative payload per AW/W/B/AR/R), and AXI extraction noise is dominated by
**property/config prose** (`RME_Support`, `MPAM_WIDTH`, `AXI_Transport`, `LICENSEE`, …) whose "subjects"
are NOT declared signals — see `[[axi-constraint-subject-must-be-declared]]`. The AXI doc has **310**
declared signals (vs APB 35 / AHB 39) — much larger, so precision (rejecting non-signal subjects) matters
more than for the smaller buses.
