---
id: isf-no-explicit-fsm-abstraction
title: ISF (IAL1) has no construct to DECLARE a given explicit FSM — feature request raised for SWD/JTAG
answers:
  - "can ISF model an explicit state machine / given FSM"
  - "how to lower the JTAG TAP / SWD FSM to .isf"
  - "is there an ISF gap for SWD/JTAG and was a feature request raised"
  - "what is IAL0 vs IAL1 (.fsm vs .isf)"
  - "can the SWD serial frame be modeled in ISF"
date: 2026-06-07
tags: [isf, fsmgen, swd, jtag, fsm, feature-request, no-hacks]
evidence: docs/fsmgen-issues/sf-isf-explicit-fsm-declaration/README.md; subs/fsmgen/docs/ISF_PUBLIC_INTERFACE_CONTRACT.md; subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md (§11.4); docs/tasks/SWD-SERIAL-EXTRACTION.md (.6)
reverify: grep -nE "IAL0|IAL1|scheduling-intent" subs/fsmgen/docs/ISF_PUBLIC_INTERFACE_CONTRACT.md | head
---

ISF layering (from the FSMGen public contract): **`.fsm` = IAL0**, "the explicit cycle-authored" finite
state machine; **`.isf` = IAL1**, "scheduling-intent that lowers to reviewable IAL0 `.fsm`". SpecForge
emits `.isf` (IAL1). IAL1's vocabulary is transactions, ready/valid stages, timing, drives, rules/
priorities, and structured control flow *inside a transaction* (`when`/`while`/`until`/`repeat`,
`switch`). FSMGen **synthesizes** the `.fsm` from that intent.

**Gap (SWD-SERIAL-EXTRACTION.6):** IAL1 has **no first-class construct to DECLARE a given explicit FSM** —
named states with labeled transition edges. But the JTAG **TAP controller** is a *given* 16-state,
TMS-driven state machine (and SWD has its line/protocol FSM); SpecForge captures it
(`[[swd-protocol-fsm-surface]]`). Encoding that fixed state graph as an IAL1 scheduling transaction
(nested `while`/`switch` over a synthetic TMS input with synthetic "current state" storage) would be a
**hack** — it launders a declarative state graph through a procedural surface and loses the 1:1
state/transition identity. The owner forbids hacks (`feedback_isf_no_hacks`): the response is a **feature
request**, filed at `docs/fsmgen-issues/sf-isf-explicit-fsm-declaration/`, asking FSMGen for either an
IAL1 `(state-machine …)` declaration that lowers 1:1 to `.fsm`, a supported path to contribute a given
IAL0 `.fsm` directly, or guidance with a non-hack worked example.

**Serial frame is NOT a gap (likely):** ISF advertises SPI-like / I2C-like serial fixtures, explicit-width
**shift registers**, bit selection, read-data shifting, and completion pulses — so the SWD serial frame
(`[[swd-serial-frame-surface]]`) should lower with existing abstractions; to be confirmed (or escalated)
when the SWD `.isf` is produced at `SWD-SERIAL-EXTRACTION.5`.

Process note: per the contract, **parser acceptance ≠ support** ([[feedback_fsmgen_contract]]); the FR will
be substantiated with a concrete `.isf` attempt + `--strict --check` output at `.5`.
