---
id: swd-serial-frame-surface
title: SWD serial-frame fields are a distinct typed surface (SerialFrameField), double-gated to serial docs + frame phases
answers:
  - "how does specforge model the SWD serial frame / packet"
  - "what is SerialFrameField / serial_frame_fields / SerialFramePhase"
  - "how are ACK WDATA RDATA DATAIN bit-widths extracted"
  - "why don't parallel buses get serial_frame_fields"
  - "how are NAME[hi:lo] bit-ranges parsed into frame fields"
date: 2026-06-07
tags: [swd, adi, serial, serial-frame, extraction, evidence-ir, adr-0006]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.3); crates/specforge/src/ir/evidence.rs (extract_serial_frame_fields, parse_bit_range_fields, SerialFrameField)
reverify: python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([(f['name'],f.get('bit_width')) for f in e.get('serial_frame_fields',[])])"
---

The SWD transaction is a SEQUENCE protocol (start → 8-bit packet request → turnaround → 3-bit ACK →
data+parity → park), not a clocked-edge rule — so it gets its OWN typed surface rather than being forced
into `signal_constraints`/`temporal_rules` (owner decision (ii) on `SWD-SERIAL-EXTRACTION.3`).

`EvidenceIr.serial_frame_fields: Vec<SerialFrameField>` — each field carries `name`, `bit_width`,
`bit_range` `(high, low)`, a `SerialFramePhase` (Request / Acknowledge / Data), and supporting statements.
`extract_serial_frame_fields` mines `NAME[hi:lo]` bit-ranges (`parse_bit_range_fields`; width = |hi-lo|+1)
and is **double-gated**:

1. **Document-level** serial markers ("serial wire" / "packet request" / "shift-dr" / SWDIO / SWCLK) —
   so parallel-bus specs (which also use `WDATA`/`RDATA` and the phrase "data phase") produce **zero**.
2. **Per-statement frame phase** (request via APnDP/RnW/"packet request"; acknowledge via "ack["/"acknowledge";
   data via "data bits"/wdata/rdata/datain) — so unrelated bit-fields in the serial doc (register fields,
   bridged-bus `AxCACHE[3:0]`/`HMASTER[3:0]`) are dropped.

Protocol vocabulary, not chip names (ADR 0006). Result on real ADI evidence: **5 clean frame fields** —
`A`(2b, request), `ACK`(3b, acknowledge), `DATAIN`/`WDATA`/`RDATA`(32b, data) — zero noise; APB/AHB/AXI
emit 0 `serial_frame_fields` and stay 100% on all three aspects (the surface is additive). Builds on
`[[prose-pin-appositive-signal-capture]]` (`.2` captured SWCLK/SWDIO). Follow-on `.3b`: named single-bit
request fields (APnDP/RnW), ACK response VALUES (OK/WAIT/FAULT), and the ordered field sequence.
