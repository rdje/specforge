---
id: swd-intent-is-the-fsm-driving-swdio
title: SWD's intent = its packet protocol + line state machine on SWDIO (read from spec Chapter B4)
answers:
  - "what is SWD's actual intent / protocol (from the spec)"
  - "what are the SWD packet phases and per-phase SWDIO direction"
  - "what is the SWD line state machine (reset/operating/protocol-error/lockout)"
  - "what must SpecForge derive to fully capture SWD; what are the gaps"
  - "is the SWD FSM the same as the JTAG TAP DBGTAPSM (no)"
date: 2026-06-07
tags: [swd, adi, fsm, swdio, protocol, intent, completeness]
evidence: corpus/.../IHI0074_A Chapter B4 (read directly via docling content_elements, pages 110-128); generated/source_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification
reverify: "python3 — dump source_ir content_elements for page_id page_0110..page_0128 (Chapter B4); the PDF itself is password-protected so the Read tool cannot open it — use the docling content_elements text."
---

Read directly from spec **Chapter B4 "The Serial Wire Debug Port (SW-DP)"** (the PDF is password-protected;
docling's `content_elements` carry the faithful text). SWD's full intent is its protocol, which has TWO FSM
levels — the host drives/samples SWDIO by walking them. **Two corrections to earlier loose framing:** (a)
the SWD FSM is NOT the JTAG `DBGTAPSM` (that is the JTAG-DP TAP machine on TDI/TDO/TMS, B3.2.3 — captured in
`.4` but a *different* thing); (b) per B4.3.1 the target both **samples and drives SWDIO on the RISING edge
of SWCLK** (an earlier extraction said "falling" — the spec says rising).

**1. Packet-operation micro-sequence (B4.1.1, B4.2)** — 2 or 3 phases, each with a SWDIO direction:
- **Packet request** (host drives, 8 bits): `Start`(0b1) · `APnDP` · `RnW` · `A[2:3]` · `Parity` · `Stop`(0b0) · `Park`(0b1). All LSB-first.
- **Turnaround (Trn)** — neither drives; length = `DLCR.TURNROUND` (default 1 cycle).
- **Acknowledge** (target drives, 3 bits `ACK[0:2]`): `OK`=0b001, `WAIT`=0b010, `FAULT`=0b100 (not parity-covered).
- **Data** (only on OK+data, or ORUNDETECT): write → `Trn` then host drives `WDATA[0:31]`+parity (then host keeps driving, no Trn); read → NO Trn, target drives `RDATA[0:31]`+parity, then `Trn` (host resumes).
- Parity: even, over `APnDP+RnW+A[2:3]` (request) and over the 32 data bits.
- Response branching: OK → 3-phase; WAIT/FAULT → 2-phase (data phase only if ORUNDETECT).

**2. Line state machine (B4.2.5, B4.3.3):** `reset` (entered by line reset = ≥50 SWCLK with SWDIO HIGH +
≥2 idle; only DPIDR read [exits], switch sequences, or TARGETSEL write[v2] are valid) → `operating` →
`protocol-error` (on Parity mismatch / Stop≠0 / Park≠1 / bad TURNROUND; target stops driving; exits on
line reset or DPIDR read) → `lockout` (further errors; exits only on line reset); plus `dormant`(v2 powerup)
and multi-drop `deselected`.

**What SpecForge derives today:** SWCLK/SWDIO signals (`.2`); SOME packet bit-fields (`.3`: A, ACK, APnDP,
RnW, WDATA, RDATA) — but MISSING Start/Parity/Stop/Park and with no direction/sequence/turnaround/branch;
the JTAG TAP `DBGTAPSM` (`.4`, not the SWD FSM). **So SpecForge does NOT yet fully derive SWD's intent.**

**Gaps (all derivable from the B4 prose docling extracted):** the SWD packet **phase sequence** + per-phase
**SWDIO direction** (host vs target) + turnarounds + **response branching** (OK/WAIT/FAULT) + missing fields
(Start/Parity/Stop/Park) + the **line state machine** (reset/operating/protocol-error/lockout/dormant) +
edge timing + line-reset/parity rules. This is a new protocol-FSM derivation capability, beyond the current
signal-table / constraint / relation / bit-field extractors. ISF can express it (`[[isf-fsm-via-switch-select]]`);
SpecForge emits intent, FSMGen lowers. See `[[swd-serial-frame-surface]]`, `[[swd-protocol-fsm-surface]]`.
