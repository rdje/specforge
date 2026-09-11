---
id: protocol-state-machine-binding
title: A protocol state binds to its machine only when the document names both in one statement, and two machines fail closed
answers:
  - "why is ProtocolStateRecord machine_name always None"
  - "how does SpecForge bind a protocol state to its state machine"
  - "why did SWD protocol_state score 0/13 when the state names were right"
  - "what is bind_protocol_state_machines"
  - "how is a state machine identifier introduced generically"
  - "why does a state naming two machines stay unbound"
  - "how many protocol states carry a machine name corpus-wide"
  - "did WIRE-BASED-100.8g restore the retired SWD 29/29"
  - "why is scope binding not used for protocol states"
date: 2026-09-11
status: current
tags: [evidence-ir, protocol-states, swd, adr-0006, adr-0037, wire-based-100, fail-closed]
evidence: crates/specforge/src/ir/evidence.rs (bind_protocol_state_machines; state_machine_introductions; a_state_binds_to_the_machine_its_own_statement_names; two_machines_in_one_statement_leave_the_state_unbound; a_machine_introduction_must_be_closed_and_whole); docs/tasks/WIRE-BASED-100.md (.8c/.8d/.8g); crates/specforge/test_data/llm_eval/seed_swd_derivation.json
reverify: "cargo test --offline -p specforge-core --lib machine && ./target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip   # expect protocol_state 8/13"
---

`89d8dee7` retired the protocol-named carriers on ADR 0006 grounds and nothing replaced the one that set
`ProtocolStateRecord::machine_name`. The field survived — `ir/evidence.rs` still declares
`machine_name: Option<String>` — but has been `None` ever since, so a gold keyed `machine|state` could not
resolve **even where every state name was extracted correctly**. That is exactly what SWD looked like:
`protocol_state` scored `0/13` while the extractor produced 8 of the 13 names right.

## The binding the document already states

ADIv6 supplies both halves in its own words:

- it **introduces** the identifier through a role phrase — *"The Debug TAP **State Machine** (DBGTAPSM)
  controls the operation of a JTAG-DP"* — the same role-phrase appositive shape
  [[document-stated-identifier-coreference]] reads for signals, with a parenthesis instead of a comma;
- it then **names that identifier in the same statement as the state** — *"When the DBGTAPSM goes through
  the Capture-IR state"*, *"While the DBGTAPSM is in the Shift-IR state"*, and so on for all eight.

`state_machine_introductions` accepts `<…> state machine` followed by `(IDENT)` or `, IDENT`, closed the way
`SPEC-TO-INTENT-ALIGNMENT.7a`'s appositive punctuation requires; `state machine` must be a whole phrase, so
`substate machine (X)` introduces nothing. `bind_protocol_state_machines` then binds a state **only** from
its own supporting statement.

## Same statement only, because scope binding was already refuted

`WIRE-BASED-100.8d` measured scope binding for the sibling problem — serial frame fields — and disproved it:
the nearest preceding phase-stating statement resolves for every field and is **wrong on 11 of 11**, and the
owning section title tops out at 5 of 11. A rule that fires everywhere and is right nowhere is fabrication
with a structural alibi. Same-statement co-occurrence is therefore the only binding accepted here, and
**two machines in one statement fail closed** — an ambiguous binding is worse than an absent one, which is
`.8d`'s own conclusion about minting a wrong phase.

## Measured, before and after

Across the 27 proof-carrying chains, **12 of 45 states bind**: 8 in ADIv6 (`DBGTAPSM`) and 4 in USB 3.2
(`LTSSM`, `SPSM`). USB 3.2 also contributes the refusals the rule must make — 4 states name two machines in
one sentence and stay unbound, 22 name none. AXI (3 states), APB (2) and the USB4 connection-manager guide
(2) introduce no machine and are untouched. Nothing else in the corpus moves: ports, actors, declared
inventory, provenance, constraints and emitted ISF counts are identical across all 27.

## What it does and does not claim

SWD `protocol_state` goes `0/13 → 8/13` and document-level recall `5/29 → 13/29`. It does **not** restore
the retired `29/29`: that score measured a protocol recogniser which keyed phases off field names
(`.8c`), and this rule names no protocol. `serial_frame_field` stays honestly at `0/11` behind `.8d`'s
figure-extraction re-open trigger, and the five *SWD line state machine* states are not extracted at all —
a separate recall gap, not a binding one.

Links: [[document-stated-identifier-coreference]], [[indexed-signal-family-canonicalization]],
[[base-name-template-table-is-not-a-catalogue]].
