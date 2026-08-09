---
id: swd-adi-not-signal-table-spec
title: SWD/ADI (IHI0074) is an architecture/serial spec — the parallel-bus signal-table model doesn't fit
answers:
  - "why is SWD/ADI hard / different from APB AHB AXI"
  - "are SWCLK and SWDIO extracted / declared"
  - "why does the ADI spec produce so few signals and so much garbage"
  - "can WIRE-BASED-100 reach 100% on SWD the same way as the parallel buses"
  - "what extraction approach does SWD/ADI need"
date: 2026-08-09
status: current
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
  table**. At the original audit they appeared 13×/28× in prose but had zero declarations; the dedicated
  serial path now captures both from source-backed pin-appositive grammar.
- The original parallel-bus pattern/table path also produced substantial garbage (cleaned in
  `WIRE-BASED-100.5j`:
  `0B0`/`0B1` number-literals via the new leading-letter rule in `is_hardware_signal_token`; `IN`/`LEVEL`
  via `is_signal_synthesis_non_signal`).

**Conclusion (no-faking):** SWD/ADI cannot reach a real 100% via the parallel-bus playbook; it requires
serial-protocol/architecture-specific extraction. The dedicated `SWD-SERIAL-EXTRACTION` tree has now completed
that path: SWCLK/SWDIO, 11 frame fields, four response-branched operations, 13 protocol states, and the complete
interface-edge timing tuple score 29/29 at 1.000 and reach canonical IntentIR exactly. APB/AHB/AXI remain 100%
on constraints/relations/temporal, while SWD uses its faithful serial metric rather than a cherry-picked bus
gold. See `[[swd-canonical-protocol-artifact-is-current]]` and `[[axi-channel-structure]]`.
