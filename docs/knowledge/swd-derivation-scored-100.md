---
id: swd-derivation-scored-100
title: SWD intent derivation is scored 100% — eval-extraction gained serial_frame_field / swd_operation / protocol_state tasks
answers:
  - "how is the SWD FSM/frame derivation scored (not constraints/relations/temporal)"
  - "what eval-extraction tasks score the SWD surfaces"
  - "what is in seed_swd_derivation.json"
  - "is SWD at 100% and on what metric"
  - "how to re-score SWD derivation"
date: 2026-06-07
tags: [swd, eval, scoring, wire-based-100, serial-frame, fsm, completeness]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.5); crates/specforge/src/eval.rs; crates/specforge/src/commands/eval_extraction.rs; crates/specforge/test_data/llm_eval/seed_swd_derivation.json
reverify: ./target/debug/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip 2>/dev/null | sed -n '/source-tolerant/,/document-level/p'
---

SWD's intent lives in the FSM/frame, NOT constraints/relations/temporal
(`[[swd-intent-is-the-fsm-driving-swdio]]`) — but `eval-extraction` only scored those three. Owner
decision (a): build a real scorer for the SWD surfaces and drive it to per-fact 100%.

`eval`/`eval-extraction` gained three tasks — **`serial_frame_field`**, **`swd_operation`**,
**`protocol_state`** — with new `EvalTask`/`GoldFact` variants, canonical keys (`frame_field_key`,
`swd_operation_key`, `protocol_state_key`), produced-record key + index functions, and a deterministic
extractor that reads the EvidenceIR surfaces (no LLM). Gold: `seed_swd_derivation.json` = **28
spec-verified facts** (11 frame fields, 4 operations, 13 FSM states), each checked against spec B4.2 /
B3.2.3 (not copied blindly — verified).

Two precision fixes were needed to reach 100% (the scorer surfaced them): **DATAIN phase request→data**
(data-keyword precedence in `extract_serial_frame_fields` — its statement has both "RnW" and "DATAIN"),
and **separator dedup** of the docling `Test-Logic/Reset` variant into `Test-Logic-Reset` (normalize '/'→
'-', keep max-support; `Run-Test/Idle`'s legit '/' is preserved).

Result: **source-tolerant (the WIRE-BASED-100 metric) `P=R=F1=1.000` on all three** — `serial_frame_field`
(tp=11), `swd_operation` (tp=4), `protocol_state` (tp=13). The strict per-statement view is lower only
because some statements support multiple doc-level facts (e.g. `1679`→A/APnDP/RnW, `2094`→Dormant/Operating)
— the source-tolerant scorer is the bar used for every spec. Parallel buses + their evals unchanged.

**"100% on all fronts" reached:** APB/AHB/AXI 100% (constraints/relations/temporal) + SWD 100% (frame/
operations/FSM). This unblocks the owner's `.isf` lowering gate. Lower via `[[isf-fsm-via-switch-select]]`.
