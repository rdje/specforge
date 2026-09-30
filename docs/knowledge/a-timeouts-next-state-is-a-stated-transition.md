---
id: a-timeouts-next-state-is-a-stated-transition
title: A timeout table's next state is a typed protocol transition, carried to IntentIR and not lowered
answers:
  - "where does SpecForge carry a state transition a table states"
  - "what is protocol_transitions / ProtocolTransitionRecord / ProtocolTransitionTrigger"
  - "how is USB 3.2 Table 7-12 LTSSM State Transition Timeouts read"
  - "why is a timer's expiry not a temporal rule or a StateTransitionRecord"
  - "what does a transition with several target_states mean"
  - "when does a slash in a state cell separate two states (Run-Test/Idle)"
  - "how is a footnote mark read when the converter lost its note (Polling.Active 2)"
  - "how does a timeout table's caption bind a state machine (LTSSM)"
  - "why does the isf adapter keep an isf_protocol_transition residual"
  - "why does adding an EvidenceIR top-level field re-seal every chain"
  - "how is a timer name with a parenthesized operation mode read (SuperSpeedPlus operation)"
  - "what is timer_name_and_mode / ProtocolTransitionRecord conditions"
date: 2026-09-30
status: current
tags: [evidence-ir, timing, protocol-states, transitions, table, usb, adr-0006, adr-0033, fsmgen]
evidence: crates/specforge/src/ir/evidence.rs (ProtocolTransitionRecord; protocol_transition_surface; timeout_tables; timeout_table_note_marks; timeout_table_states); crates/specforge/src/ir/adapters.rs (protocol_residual_decisions); docs/tasks/extraction-representation-gaps/table-census-leaves.md (.3.b.iii.g); docs/upstream/fsmgen/sf-isf-named-timers/README.md
reverify: "cargo test --offline -p specforge-core --lib -- timeout_rows_next_state lost_note_mark state_cell_is_split && python3 -c \"import json; e=json.load(open('generated/evidence_ir/usb_3_2_revision_1_0_2017_09/evidence_ir.json')); t=e['protocol_transitions']; print(len(t), {r.get('machine_name') for r in t})\"   # expect 28 {'LTSSM'}"
---

A column headed `… Next State` in a timeout table (USB 3.2 *Table 7-12. LTSSM State Transition Timeouts*:
`Name | Initial State | Timeout to Next State | Timeout Values`) states a transition: the timer running in the
initial state leads, on expiry, to the next state. EvidenceIR carries it as `protocol_transitions`
(`ProtocolTransitionRecord`) beside `protocol_states`, in the `evidence.protocol_structure` family; SemanticIR and
IntentIR carry it losslessly (`semantic.carried_evidence`, `intent.carried_semantics`).

**Why a record of its own.** A temporal rule needs a clocked antecedent and a signal consequent, and a timer's
expiry is neither; one rule per listed next state would assert every alternative. `StateTransitionRecord` belongs
to the explicit design notation (`transition A -> B when …`): one target, a signal guard. ADR 0033 §6 keeps
non-executable knowledge typed and queryable rather than forcing it into an executable construct.

**What a record says.** `source_state`, every `target_states` entry as written, a `trigger` of kind `timer_expiry`
naming the timer and citing the same row's `timing_constraint_ids`, and `supporting_table_ids`. Several targets
are alternatives the prose chooses among; the record does not choose. A row whose timer runs in two states
(`Polling.LFPS or Polling.LFPSPlus`) gives one record per state. A row whose value is not a duration still states
its transition and cites no timing record (`PORT_U2_TIMEOUT`, U1 -> U2). The caption binds `machine_name` when it
names exactly one machine a statement introduces (`state_machine_introductions`); two bind none. A name cell's
mode phrase (`tPollingSCDLFPSTimeout (SuperSpeedPlus operation)`: one identifier, then two or more words ending in
`operation` or `mode`) is split off by `timer_name_and_mode` (`.3.b.iii.h`) into the transition's `conditions` and
the timing records' first `row_conditions` entry; any other parenthesis stays in the name.

**Splitting a state cell.** A comma and the word `or` always separate states. A slash separates two only when one
side is a name the table writes whole elsewhere (`Rx.Detect` is a row's entire next state), because a slash can be
part of one name (`Run-Test/Idle`). The rule is per boundary, so `A/B/C` with only `A` proven gives `A`, `B/C`.

**Lost footnote marks.** The converter kept *Table 7-12*'s marks and truncated its notes after note 1. When the
page prints note 1 and the table's name and state cells end in exactly 1..*n* (*n* ≥ 2), all *n* are marks; one
text ending in two different numbers is an enumeration (`Phase 1`, `Phase 2`) and nothing is dropped. The same
marks come off the timing records' `Initial State` conditions.

**Measured `2026-09-30`.** On the 31 chains only USB 3.2 moves: 28 transitions from 25 rows, all bound to `LTSSM`;
six timing records lose a lost mark (`Initial State: Polling.Active 2` -> `Polling.Active`). 23 of the 27 states the
transitions name are not `protocol_states` (`EXTRACTION-REPRESENTATION-GAPS.3.b.iii.i`).

**Downstream.** `.isf` has no timer whose expiry is a condition (`FSMGEN-REQUESTS.8`, packet `sf-isf-named-timers`),
so the adapter keeps one `isf_protocol_transition_*` residual per record, citing its table.

**Cost of the carrier.** A new top-level field enters `EVIDENCE_RULE_FIELDS`, which is inside the EvidenceIR seal
closure ([[stage-seal-closure-membership]]), so every chain's EvidenceIR and everything below it re-seals; the 30
content-identical chains are re-sealed by `scripts/rebuild_stage_cascade.sh --write` after the one chain with a
content change is rebuilt stage by stage. The claim-family inventory and the rule counts in `ir/mod.rs` move by one
rule per stage.

Links: [[timing-table-structural-authority]], [[protocol-state-machine-binding]],
[[untyped-uncited-tables-fall-into-fifteen-families]].
