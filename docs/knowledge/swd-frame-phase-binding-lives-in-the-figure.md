---
id: swd-frame-phase-binding-lives-in-the-figure
title: SWD's frame field-to-phase binding is drawn in Figure B4-1, not written in prose — so no statement-scope or section-scope rule can recover it
answers:
  - "can the retired SWD serial frame fields be recovered by binding a document-stated phase to nearby statements (NO — the nearest preceding phase-stating statement resolves for all 11 gold fields and is WRONG for ALL ELEVEN, correct on none; re-derive with python3 scripts/measure_swd_frame_phase_scope.py. APnDP/RnW/Start/Parity/Stop would inherit transfer and Park would inherit data, all against a gold phase of request; A, ACK and DATAIN sit 278, 303 and 324 statements after the nearest one, which says response)"
  - "can the owning section title supply the SWD frame phase (NO — 4 of 11 under a literal title-states-the-phase reading and 5 of 11 even counting any appearance of the phase word or its stem, so the ceiling is 5. Packet requests gives request for APnDP/RnW and Data transfers (WDATA and RDATA) gives data for WDATA/RDATA, but Start/Parity/Stop sit under B4.2 SWD protocol operation, Park under B4.2.5 Protocol error response, A under Attributes, ACK under a table caption, DATAIN under OK or FAULT response to a DPACC or APACC access. No title contains the word phase)"
  - "where is the SWD frame's field-to-phase membership actually stated (in Figure B4-1 SWD successful write operation and Figure B4-2 SWD successful read operation. Both are captured as visual assets picture_0038 and picture_0039, but their role is ambiguous and their only observation is the caption, so the diagram content was never read)"
  - "why did the retired SWD frame extractor score 11 of 11 if the document does not state the phases (because it did not read the frame: it keyed the phase off the FIELD NAME — wdata/rdata/datain/ack[ — so it carried SWD's field-to-phase table in the code. That is what ADR 0006 forbids, and it is why removing it lost the facts entirely)"
  - "what would make SWD frame recall buildable again (figure-content extraction reaching this class of diagram. The typed carrier already exists as VisualObservationKind::TimingDiagramExtraction and the assets are already captured, so the gap is the extraction pass rather than the schema)"
  - "why is SWD serial_frame_field left at 0/11 instead of being partially recovered (because every candidate rule that fires often enough to help also mis-assigns the phase on most fields, and a wrong phase is fabrication. A measured zero is the honest result; WIRE-BASED-100.8d is deferred with that consequence stated)"
date: 2026-09-01
status: current
tags: [swd, adi, wire-based-100, serial-frame, genericity, adr-0006, measurement, residual, vlm]
evidence: "generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json (extracted_statements, evidence_spans, section_anchors, visual_evidence picture_0038/picture_0039); crates/specforge/test_data/llm_eval/seed_swd_derivation.json; crates/specforge/src/ir/evidence.rs (extract_serial_frame_fields, stated_phase_name); git show 89d8dee7^:crates/specforge/src/ir/evidence.rs; docs/tasks/WIRE-BASED-100.md (.8d)"
reverify: "python3 scripts/measure_swd_frame_phase_scope.py"
---

# The phase is in the picture

`WIRE-BASED-100.8c` established that SWD's 11 gold frame fields went to 0/11 when a protocol-name-bound
extractor was retired. `.8d` proposed the obvious generic repair: stop requiring the phase name and the bit
range in the *same* statement, and bind a document-stated phase to the fields in its scope. Measured against
the gold, that repair is wrong.

**Statement scope fires everywhere and is right nowhere.** The nearest preceding phase-stating statement
exists for all 11 fields, so the rule would always produce something — and for **every one of the eleven** it
produces the wrong phase. `APnDP` and `RnW` would take `transfer` against a gold of `request`; `Start`, `Parity`
and `Stop` would take `transfer` from 28 to 44 statements back; `Park` would take `data`. `A`, `ACK` and
`DATAIN` are **278, 303 and 324** statements past the nearest phase-stating sentence, which says `response`.

**Section scope is honest but thin.** Resolving each gold statement's span to its section anchor recovers
`request` for `APnDP`/`RnW` (section `Packet requests`) and `data` for `WDATA`/`RDATA` (section `Data transfers
(WDATA and RDATA)`) — 4 of 11 — while `Start`/`Parity`/`Stop` sit under `B4.2 SWD protocol operation`, `Park`
under `B4.2.5 Protocol error response`, `A` under `Attributes`, and `DATAIN` under `OK or FAULT response to a
DPACC or APACC access`. Not one section title contains the word `phase`.

## Because the document draws it rather than writing it

For `Start`, `Parity`, `Stop`, `Park` and `A` the specification never states the phase in text. The frame's
field-to-phase membership is in **`Figure B4-1 SWD successful write operation`** and `Figure B4-2 SWD
successful read operation`. Both figures are captured — `picture_0038` and `picture_0039` — but each carries
role `ambiguous` and a single `caption` observation, so nothing has read what the diagram shows.

That also explains the retired extractor's perfect score. It never read the frame either: it assigned the phase
from the field name (`wdata`, `rdata`, `datain`, `ack[`), which is SWD's field-to-phase table transcribed into
production code. A prose rule cannot replace a lookup that was never a reading, and adding one back is exactly
what ADR 0006 forbids.

## The consequence, stated rather than papered over

SWD `serial_frame_field` stays at **0/11**. Nothing is minted, because a wrong phase is worse than a measured
zero. `WIRE-BASED-100.8d` is deferred on that basis, and its re-open trigger is figure-content extraction: the
typed carrier (`VisualObservationKind::TimingDiagramExtraction`) and the captured assets already exist, so what
is missing is the pass that reads them — not the schema.

Links: [[swd-serial-frame-score-retired-by-genericity]], [[swd-adi-not-signal-table-spec]],
[[swd-intent-is-the-fsm-driving-swdio]], [[production-genericity-boundary]].
