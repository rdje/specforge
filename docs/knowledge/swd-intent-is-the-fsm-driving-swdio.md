---
id: swd-intent-is-the-fsm-driving-swdio
title: SWD's full intent IS its FSM walking SWDIO — host drives commands / samples returned data per state
answers:
  - "where does SWD's intent live (FSM, not constraints/relations)"
  - "what must SpecForge derive to fully capture SWD"
  - "how does SWD drive commands and capture data on SWDIO"
  - "what are the SWD per-phase SWDIO directions (drive vs sample)"
  - "what SWD intent gaps remain in SpecForge"
date: 2026-06-07
tags: [swd, adi, fsm, swdio, intent, completeness, owner-guidance]
evidence: corpus/.../IHI0074_A (SWD operation); generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification (statement_1946/1948); docs/tasks/SWD-SERIAL-EXTRACTION.md
reverify: python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print(e['extracted_statements'][0] and [s['text'][:90] for s in e['extracted_statements'] if s['statement_id']=='statement_1948'])"
---

Owner (`2026-06-07`): "SWD full specification comes from its FSM description. An agent will use SWD to
drive commands onto SWDIO using the FSM and return captured data still using the FSM on SWDIO." So SWD's
intent does **not** live in the classic constraint/relation/temporal surfaces (those are sparse for the
architecture-style ADI spec — SWD constraints+relations measure 100% but on a tiny clean set, and the lone
temporal rule is noise). **SWD's intent IS the FSM that walks SWDIO.**

The SWD packet FSM (host perspective), each state with a SWDIO **direction**:
- **Request** — host **drives** SWDIO: start, APnDP, RnW, A[2:3], parity, stop, park (8-bit packet request).
- **Turnaround** — SWDIO tristates / changes direction.
- **Acknowledge** — target **drives** SWDIO; host **samples** ACK[2:0] (OK/WAIT/FAULT).
- **Turnaround** (write) — direction changes.
- **Data** — write: host **drives** SWDIO (WDATA[0:31]+parity); read: target **drives**, host **samples**
  (RDATA[0:31]+parity).
Edge timing (`statement_1948`): the target **samples SWDIO on the rising edge of SWCLK** and **drives /
stops driving on the falling edge**. Line states also exist (reset / operating / protocol-error).

**What SpecForge derives today:** SWD signals SWCLK/SWDIO (`[[prose-pin-appositive-signal-capture]]`); the
packet FIELDS with phase + widths (`[[swd-serial-frame-surface]]`: A/ACK/APnDP/RnW/WDATA/RDATA/DATAIN); and
the **JTAG** TAP states (`[[swd-protocol-fsm-surface]]`: DBGTAPSM — note this is the JTAG-DP FSM on
TDI/TDO/TMS, NOT the SWD-on-SWDIO packet FSM).

**Gaps to FULLY derive SWD's intent (the work):**
1. **Per-state SWDIO direction** — each frame field/phase needs `drive` vs `sample` and the actor
   (host/target). Today `SerialFrameField` has phase but no SWDIO direction. (SWD-SERIAL-EXTRACTION.4c)
2. **The SWD packet FSM** — the request→turnaround→ack→turnaround→data→park sequence as states with
   transitions (distinct from the JTAG TAP FSM already captured). (.4b/.4c)
3. **Line state machine** — reset / operating / protocol-error states. (.4d)
4. **Edge timing** binding (sample on rising SWCLK, drive on falling). (.4d)

No FSMGen/ISF gap is involved (ISF expresses FSMs — `[[isf-fsm-via-switch-select]]`); this is SpecForge-side
intent DERIVATION completeness.
