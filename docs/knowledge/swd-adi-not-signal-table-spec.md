---
id: swd-adi-not-signal-table-spec
title: SWD/ADI (IHI0074) is an architecture/serial spec — the parallel-bus signal-table model doesn't fit
answers:
  - "why is SWD/ADI hard / different from APB AHB AXI"
  - "are SWCLK and SWDIO extracted / declared"
  - "why does the ADI spec produce so few signals and so much garbage"
  - "can WIRE-BASED-100 reach 100% on SWD the same way as the parallel buses"
  - "what extraction approach does SWD/ADI need"
date: 2026-06-07
tags: [swd, adi, jtag, wire-based-100, serial, extraction-limits]
evidence: corpus/arm/debug/interfaces/adi/current/IHI0074_A_2017-03-09_Arm_Debug_Interface_v6_Architecture_Specification.pdf; generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification
reverify: python3 -c "import json,re; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print(sorted({re.match(r'Signal (\w+)',s['text']).group(1) for s in e['extracted_statements'] if s['text'].startswith('Signal ')}))"
---

The Arm Debug Interface v6 spec (IHI0074, the SWD/ADI spec) is **structurally different** from the
parallel-bus specs (APB/AHB/AXI) and does NOT fit the signal-description-table extraction model that took
those to 100%:

- It is an **architecture/register/protocol spec** (~7032 statements: DAP, DP/AP registers, the SWD &
  JTAG-DP serial protocols), not a signal-table spec.
- The **core SWD wire contract — `SWCLK` and `SWDIO` — is described in prose/figures, never in a signal
  table**, so it is NOT declared (SWCLK appears 13×, SWDIO 28× in prose, 0 declarations). Only ~9 real
  signals get declared (JTAG `TCK`/`TDI`/`TDO`, `DBGTDI`/`DBGTDO`/`DBGTMS`, `NSRSTOUT`, `CSYSPWRUPACK`,
  `PORTCONNECTED`).
- The pattern/table extractor therefore produces mostly garbage on it (cleaned in `WIRE-BASED-100.5j`:
  `0B0`/`0B1` number-literals via the new leading-letter rule in `is_hardware_signal_token`; `IN`/`LEVEL`
  via `is_signal_synthesis_non_signal`).

**Conclusion (no-faking):** SWD/ADI cannot reach a real 100% via the parallel-bus playbook — it needs
**serial-protocol / architecture-specific extraction** (capture SWCLK/SWDIO from prose+figures, model the
serial frame request/ack/data sequence and the DP/AP register interface). That is a separate research/
design effort, a different problem class. APB/AHB/AXI (the three parallel buses) are at 100% on all three
aspects; SWD is honestly deferred to a dedicated serial-extraction tree rather than faked with a
cherry-picked gold. See `[[axi-channel-structure]]`, `[[axi-constraint-subject-must-be-declared]]`.
