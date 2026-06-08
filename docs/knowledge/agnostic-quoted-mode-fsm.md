---
id: agnostic-quoted-mode-fsm
title: A second agnostic FSM grammar — quoted operational modes of an actor (CAN error states) vs SWD's "<State> state"
answers:
  - "how does specforge extract a CAN-style error-state FSM (error active / error passive / bus off)"
  - "what is extract_quoted_mode_states / quoted_mode_states_in / mode_state_ ids"
  - "how are FSM states recovered when the protocol quotes them as node modes instead of <Name> state"
  - "why does CAN yield 0 from extract_protocol_states but 3 from the quoted-mode path"
  - "how is the quoted-mode FSM extractor kept agnostic and false-positive-free (ADR 0006)"
date: 2026-06-08
tags: [can, fsm, state-machine, serial, extraction, evidence-ir, pdf-variant-digestion, adr-0006]
evidence: docs/tasks/PDF-VARIANT-DIGESTION.md (.9.3a); crates/specforge/src/ir/evidence.rs (extract_quoted_mode_states, quoted_mode_states_in, is_quoted_mode_state_name)
reverify: python3 -c "import json; e=json.load(open('generated/evidence_ir/bosch_can_specification_2_0_1991/evidence_ir.json')); print([s['state_name'] for s in e.get('protocol_states',[])])"
---

Protocols name their FSM states **two structurally different ways**, and SpecForge now has an extractor for
each (both grammar-only, ADR 0006 — never the literal names):

1. **SWD/JTAG shape** — an unquoted Capitalized-Hyphen identifier followed by the word *state*
   (`Shift-DR state`, `Test-Logic-Reset state`), behind a TAP/scan-chain doc-gate →
   `find_states_with_actions` / `extract_protocol_states` (see `[[swd-protocol-fsm-surface]]`).
2. **Quoted-mode shape** — the state is a **single-quoted operational MODE of a generic actor**:
   CAN's fault-confinement FSM says *"a unit may be in one of three states: 'error active' / 'error passive'
   / 'bus off'"* and *"A node is 'error passive' when the TRANSMIT ERROR COUNT ≥ 128"*. The SWD path yields
   **0** here (no `<Name> state` grammar, no TAP gate), so CAN extracted no FSM until `.9.3a`.

`extract_quoted_mode_states` (additive; deduped by name after the two SWD paths) recovers shape #2. The
grammar that makes it agnostic AND false-positive-free is the **actor-noun binding**: a state is a quoted
name (1–3 alphabetic words) bound to a generic actor-noun (`node`/`unit`/`station`/`device`) in either the
**adjective** form (`'error active' unit`) or the **predicate** form (`a node is 'error passive'` — where the
word right before the verb is the actor subject). That binding is exactly what separates a STATE (a *node* is
'error passive') from a quoted **bit value** (a *bit* is 'dominant'/'recessive') or a **bus condition** (the
*bus* is 'idle'). Two more gates kill incidental quotes: a name must **recur in ≥2 statements**, and a doc
must yield **≥2 distinct states** (one mode is not an FSM). The trailing `when …` clause is captured as the
state's `action`; records use a distinct `mode_state_NNNN` id so they never collide with `protocol_state_` /
`swd_line_state_`.

Result on real CAN evidence: **3 states** — `error active` (8 supports), `error passive` (18), `bus off` (4) —
**zero fabrication, zero spurious** (no `dominant`/`recessive`/`bus idle`). Regression: NVMe / I2C / RISC-V
Debug stay **0** protocol_states; SWD/ADI gains **0** `mode_state_*` records (its actors are DP/target/host,
not node/unit/station, and its states are unquoted) so its FSM surface is unchanged. CAN is a typical
`document_class: guide` + front-matter `specification` ⚠ under-extracted spec (`[[document-class-from-structure]]`);
this is the first prose lever closing that gap on the new serial class. Frame-field recovery (SOF/Arbitration/
…/EOF) is the sibling `.9.3b`.
