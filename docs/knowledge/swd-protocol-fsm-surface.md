---
id: swd-protocol-fsm-surface
title: The SWD/JTAG protocol FSM is a typed surface (ProtocolStateRecord) — states extracted from "<State> state" grammar
answers:
  - "how does specforge model the JTAG TAP / SWD state machine (FSM)"
  - "what is ProtocolStateRecord / protocol_states / DBGTAPSM"
  - "how are TAP states (Shift-DR, Run-Test/Idle, Test-Logic-Reset) extracted"
  - "why is the FSM important for SWD/JTAG"
  - "how are per-state actions captured"
date: 2026-06-07
tags: [swd, jtag, adi, fsm, state-machine, extraction, evidence-ir, fsmgen, adr-0006]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.4); crates/specforge/src/ir/evidence.rs (extract_protocol_states, find_states_with_actions, looks_like_state_name, ProtocolStateRecord)
reverify: python3 -c "import json; e=json.load(open('generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json')); print([s['state_name'] for s in e.get('protocol_states',[])])"
---

SWD and JTAG are *defined* by a finite state machine — the JTAG **TAP controller** (16 states, TMS-driven)
and the SWD line protocol — and the owner stressed the FSM "is critical to the proper understanding and
implementation of SWD/JTAG". It is also the heart of SpecForge's purpose: IntentIR → `.isf` → **FSMGen
builds the `.fsm`** from it. There was no FSM/state surface in the IR before this (only the FSMGen `.isf`
consumer).

`EvidenceIr.protocol_states: Vec<ProtocolStateRecord>` — each record is `{state_id, machine_name,
state_name, action, supporting_statement_ids}`. `extract_protocol_states`:
- gates to documents that describe a state machine ("state machine" / `DBGTAPSM` / "TAP controller"), so
  non-FSM specs (the parallel buses) emit **0**;
- recognizes states by the **"`<StateName>` state"** grammar where the name is a hyphen/slash-joined
  capitalized token (`looks_like_state_name`: `Shift-DR`, `Run-Test/Idle`, `Test-Logic-Reset`) — grammar,
  not names (ADR 0006);
- captures the per-state **action** clause via `find_states_with_actions` ("In the Shift-DR state, *data is
  transferred from DBGTDI to DBGTDO*").

Result on real ADI evidence: the **DBGTAPSM** ("Debug TAP State Machine") with **9 named states + actions**
(Capture-/Shift-/Update-IR, Capture-/Shift-/Update-DR, Run-Test/Idle = "no special actions occur",
Test-Logic-Reset = "is the reset condition"). Parallel buses emit 0 protocol_states and stay 100%.

Known minor artifact: a `Test-Logic/Reset` separator variant of `Test-Logic-Reset` (docling rendered the
hyphenated JTAG name with a slash) — dedup is a `.4b` refinement. Follow-ons: `.4b` FSM **transitions**
(TMS-driven edges); `.6` ensure **ISF can model the FSM elegantly** (states + transitions) or raise an ISF
feature request (owner: no hacks). Builds on `[[swd-serial-frame-surface]]`, `[[prose-pin-appositive-signal-capture]]`.
