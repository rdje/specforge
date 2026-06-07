# Feature request: ISF (IAL1) abstraction to declare a GIVEN explicit FSM (states + transitions)

- Type: **feature request** (design gap), not a bug repro bundle.
- Raised by: `SWD-SERIAL-EXTRACTION.6` (SpecForge), `2026-06-07`.
- Owner directive: "SPECFORGE need to make sure ISF has all the necessary abstractions to accurately
  model all these bus interface protocols. If not, a feature request shall be raised. Modelling … shall
  be elegant, seamless and shall by no means resort to hacks." (No-hack: `feedback_isf_no_hacks`.)

## Context

SpecForge now captures the SWD/JTAG protocol FSM as a first-class evidence surface
(`EvidenceIr.protocol_states`, `ProtocolStateRecord`): the **DBGTAPSM** ("Debug TAP State Machine") with
its named TAP states (Test-Logic-Reset, Run-Test/Idle, Capture-/Shift-/Update-IR, Capture-/Shift-/Update-DR)
and per-state actions. JTAG's TAP controller is a **given, fixed 16-state machine** whose transitions are
driven by TMS; SWD has an analogous line/protocol state machine. This FSM "is critical to the proper
understanding and implementation of SWD/JTAG" (owner).

To reach FSMGen, SpecForge lowers IntentIR → **`.isf` (IAL1)** → FSMGen → **`.fsm` (IAL0)**.

## The gap

Per the ISF public contract: **`.fsm` (IAL0) is "the explicit cycle-authored" FSM**, and **`.isf` (IAL1)
is "scheduling-intent that lowers to reviewable IAL0 `.fsm`"**. IAL1's vocabulary is transactions, stages
(ready/valid), timing, drives, rules/priorities, and *structured control flow within a transaction*
(`when`/`while`/`until`/`repeat`, `switch`). FSMGen **synthesizes** the `.fsm` from that scheduling intent.

There is **no first-class IAL1 construct to DECLARE a given explicit state machine** — a set of named
states with labeled transition edges (e.g. `Shift-DR --TMS=1--> Exit1-DR`, `--TMS=0--> Shift-DR`). The
explicit FSM is an IAL0 (`.fsm`) concept, but SpecForge emits IAL1 (`.isf`).

For a protocol whose FSM is **given by the spec** (the JTAG TAP), the IAL1 paradigm is a poor fit:
encoding a fixed 16-state TMS-driven graph as a scheduling-intent transaction (nested `while`/`switch`
over a synthetic `tms` input, with synthetic storage tracking the "current state") would be a **hack** —
it would launder a declarative state graph through a procedural scheduling surface, losing the 1:1
state/transition identity the spec defines. The owner explicitly forbids that.

## The request

An elegant IAL1 path to express a **given explicit FSM** that lowers identity-preservingly to the IAL0
`.fsm`. Candidate shapes (for FSMGen owner to choose):

1. A first-class ISF `(state-machine NAME (state S …) (transition S -> T (on COND)) …)` construct that
   FSMGen lowers to the corresponding `.fsm` states/edges 1:1; or
2. A documented, supported path for SpecForge to contribute a given `.fsm` (IAL0) directly for
   given-FSM protocols, bypassing IAL1 synthesis; or
3. Guidance that the TAP FSM SHOULD be expressed as IAL1 scheduling intent — with a worked, non-hack
   example — if FSMGen considers that the intended modelling.

## Serial frame — likely already covered (confirm at `.5`)

The companion SWD serial-frame surface (`SerialFrameField`: ordered request/ack/data bit-fields with
widths + ACK OK/WAIT/FAULT) appears representable with EXISTING ISF abstractions — the contract advertises
SPI-like and I2C-like serial fixtures, explicit-width **shift registers**, bit selection, read-data
shifting, and completion pulses. No feature request is raised for the serial frame yet; its clean
lowering will be **confirmed (or escalated) at `SWD-SERIAL-EXTRACTION.5`** when the SWD `.isf` is produced.

## Substantiation plan (per "parser acceptance ≠ support")

At `.5`, attempt to lower the DBGTAPSM to `.isf`. If no non-hack IAL1 expression exists, attach a concrete
`.isf` attempt + FSMGen `--strict --check` output here (env.txt / commands.sh / observed/, matching the
other `sf-isf-*` bundles) to make this request exact and regression-anchorable. Until then this is the
design-level gap statement.

See KM `[[isf-no-explicit-fsm-abstraction]]`, `[[swd-protocol-fsm-surface]]`; contract
`subs/fsmgen/docs/ISF_PUBLIC_INTERFACE_CONTRACT.md`.
