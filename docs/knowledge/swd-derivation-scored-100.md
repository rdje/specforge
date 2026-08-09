---
id: swd-derivation-scored-100
title: SWD protocol derivation is scored 100% across frame, operation, state, and interface-edge timing tasks
answers:
  - "how is the SWD FSM/frame derivation scored (not constraints/relations/temporal)"
  - "what eval-extraction tasks score the SWD surfaces"
  - "what is in seed_swd_derivation.json"
  - "is SWD at 100% and on what metric"
  - "how to re-score SWD derivation"
  - "how is the SWDIO sampling and drive-change edge scored"
date: 2026-06-07
tags: [swd, eval, scoring, wire-based-100, serial-frame, fsm, completeness]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.5); crates/specforge/src/eval.rs; crates/specforge/src/commands/eval_extraction.rs; crates/specforge/test_data/llm_eval/seed_swd_derivation.json
reverify: ./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip 2>/dev/null | sed -n '/source-tolerant/,/document-level/p'
---

SWD's intent lives in the FSM/frame, NOT constraints/relations/temporal
(`[[swd-intent-is-the-fsm-driving-swdio]]`) — but `eval-extraction` only scored those three. Owner
decision (a): build a real scorer for the SWD surfaces and drive it to per-fact 100%.

`eval`/`eval-extraction` has four tasks — **`serial_frame_field`**, **`swd_operation`**,
**`protocol_state`**, and **`interface_edge_timing`** — with typed `EvalTask`/`GoldFact` variants,
complete canonical keys, produced-record key + index functions, and deterministic readers over the
EvidenceIR surfaces (no LLM). Gold: `seed_swd_derivation.json` = **29 spec-verified facts** (11 frame
fields, 4 operations, 13 FSM states, 1 target/SWDIO/SWCLK rising-edge fact), each checked against spec
B4.2 / B4.3.1 / B3.2.3 rather than copied from extractor output.

Two precision fixes were needed to reach 100% (the scorer surfaced them): **DATAIN phase request→data**
(data-keyword precedence in `extract_serial_frame_fields` — its statement has both "RnW" and "DATAIN"),
and **separator dedup** of the docling `Test-Logic/Reset` variant into `Test-Logic-Reset` (normalize '/'→
'-', keep max-support; `Run-Test/Idle`'s legit '/' is preserved).

Result: **source-tolerant (the WIRE-BASED-100 metric) `P=R=F1=1.000` on all four** — `serial_frame_field`
(tp=11), `swd_operation` (tp=4), `protocol_state` (tp=13), `interface_edge_timing` (tp=1). The edge-timing
key includes actor, data signal, clock signal, edge, sampling, and drive-state change, so an incomplete
or wrong tuple cannot receive credit. The strict per-statement view is lower only
because some statements support multiple doc-level facts (e.g. `1679`→A/APnDP/RnW, `2094`→Dormant/Operating)
— the source-tolerant scorer is the bar used for every spec. Parallel buses + their evals unchanged.

**"100% on all fronts" reached:** APB/AHB/AXI 100% (constraints/relations/temporal) + SWD 100% (frame/
operations/FSM/edge timing). The fresh canonical chain now projects those exact records through SemanticIR and
IntentIR and gives every record an honest adapter disposition (`[[swd-protocol-surfaces-reach-intentir]]`).
