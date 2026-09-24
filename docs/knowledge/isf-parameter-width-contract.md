---
id: isf-parameter-width-contract
title: FSMGen elaborates a signal width named by a declared actor parameter, but refuses every published spelling of a DERIVED width (DATA_WIDTH/8) and every zero default — so 62 live parameter widths are emittable and 56 derived ones are not
answers:
  - "can the isf adapter emit (width DATA_WIDTH) with a params declaration"
  - "does FSMGen accept (params (DATA_WIDTH 32)) and (width DATA_WIDTH) in an actor"
  - "can FSMGen state a derived width like DATA_WIDTH/8 or ceil(ADDR_WIDTH/8)"
  - "does an actor param or constant accept an expression such as (/ DATA_WIDTH 8)"
  - "can a width parameter default be 0 in FSMGen"
  - "how many signals have a parameterised width and how many a derived width"
  - "which chains would a parameterised-width adapter change reach"
  - "where do parameter defaults like ADDR_WIDTH 32 come from in the IR"
  - "why does APB-D requester.isf fail OperandContract on VALID when widths are parameterised"
  - "does IntentIR carry Param symbol definitions"
date: 2026-09-24
status: current
tags: [fsmgen, isf, isf-adapter, contract, width, params, kg-isf-completeness, measured, empirical]
evidence: "docs/tasks/kg-isf-completeness/isf-parameter-widths.md (KG-ISF-COMPLETENESS.2a.vii.a); scripts/measure_parameterised_signal_widths.py; subs/fsmgen/docs/book/src/13a-actor-interface.md (Interface: width may be an actor scalar parameter default or declared constant; arbitrary expressions fail closed); subs/fsmgen/docs/book/src/07-packages-and-sharing.md (arbitrary expressions are not type-width symbols); crates/specforge/src/ir/isf_ir.rs (signal width lowering in IsfModule::from_intent_ir; render_isf_width_hint)"
reverify: "python3 scripts/measure_parameterised_signal_widths.py — expect TOTAL live (schema 2): parameter=62 derived=56 conflict=0 over exactly AXI-L, APB-E and AHB. FSMGen probe: take generated/adapters/isf/ihi0024_d_2021_04_amba_apb_protocol_specification/requester.isf, add `(params (DATA_WIDTH 32))` after the actor line and change `(output PSTRB (width 1))` to `(width PSTRB_WIDTH)` with `(PSTRB_WIDTH (/ DATA_WIDTH 8))` in the params — `perl subs/fsmgen/bin/fsmgen --strict --check --json` must report `unsupported aggregate/list parameter leaf '/'`; a `(DATA_WIDTH 0)` default must report `must resolve to a positive integer`."
---

**Measured `2026-09-24` (`KG-ISF-COMPLETENESS.2a.vii.a`), by probing FSMGen's published CLI.**

**What elaborates.** An actor `(params (P N))` with `N > 0`, and an interface port `(width P)`. An undeclared
`P` fails closed (*is not a declared actor scalar parameter, actor constant, or imported package scalar
constant*); so does `N = 0` (*must resolve to a positive integer*). Rewriting every single-parameter signal of
the three live wire chains (APB-E, AHB, AXI-L) this way stays strict-green.

**What does not.** A derived width has no published spelling:

| Spelling | FSMGen says |
| --- | --- |
| `(params (PSTRB_WIDTH (/ DATA_WIDTH 8)))` | unsupported aggregate/list parameter leaf `/` |
| `(constants (PSTRB_WIDTH (/ DATA_WIDTH 8)))` | requires a non-negative integer literal value or enum member reference |
| `(output PSTRB (width (/ DATA_WIDTH 8)))` | width must be a positive integer, actor constant, actor scalar parameter, or qualified package scalar constant |
| package scalar constant | book 07: arbitrary expressions are not type-width symbols |

The `.fsm` `+params` surface does accept `(+ WIDTH 1)`-style expressions (book 04), but the `.isf` actor
params do not. The exact identity `ceil(x/8) = (x+7)/8` does not help, because `/` itself is refused.

**Zero is a legal width in the specifications** (AXI `MECID_WIDTH` default 0, AHB `HPROT_WIDTH` 0, 4 or 7) and
means the signal is absent. FSMGen cannot take it as a default, so a document default of 0 is not an emittable
width.

**The population.** Only schema-2 chains can be re-adapted (`adapt` refuses a legacy IntentIR). Three carry
symbolic widths: **62 single-parameter signals and 56 derived ones**, with no conflicts. The documents state
the values: AXI in `Name | Values | Default` parameter tables, AHB and APB in signal-row prose. **None of it reaches
the IR**: IntentIR `symbol_definitions` holds only enums, and `SymbolDefinitionKind::Param` is never minted.

**A trap for the next person.** Legacy APB-D and AXI-H rules lower *"X must be valid"* to `(X VALID)`. FSMGen
sizes the undeclared `VALID` from its first assignment, so the moment any driven signal is wider than 1, every
width-1 `(Y VALID)` fails the OperandContract. It has passed strict only because every width was 1. The
current producer's APB chain (APB-E) mints none; `.2a.vii.e` owns confirming it.

Links: [[fsmgen-ignores-signal-direction]], [[isf-value-width-operand-contract]],
[[arithmetic-width-drops-the-declaration]], [[property-table-is-not-a-signal-inventory]].
