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
  - "where are a parameter's legal values like 8, 16, 1024 or 0..32 stored"
  - "why is DATA_WIDTH declared as 8 in the AXI isf"
  - "why does APB-D requester.isf fail OperandContract on VALID when widths are parameterised"
  - "does IntentIR carry Param symbol definitions"
  - "does FSMGen strict mode accept an undeclared identifier in a rule value"
  - "has the derived-width feature request been filed with FSMGen"
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

**Shipped `2026-09-24` (`.2a.vii.b`).** The adapter now emits these parameter widths with a placeholder
default of `1` and the residual `isf_signal_width_parameter_defaults`; the derived widths stay `(width 1)` and
are listed in `isf_signal_width_unlowered` (`lower_parameter_widths` in `ir/isf_ir.rs`).

**Stated defaults `2026-09-24` (`.2a.vii.c.i`).** A `Name | Values | Default` table row becomes
`Parameter P = D.` in EvidenceIR and a `Param` symbol downstream; a positive `D` is the declared default
(AXI: 7 of 27). `Param` is never emitted in `(constants …)`, which would collide with the params block.

**Legal values `2026-09-24` (`.2a.vii.c.ii`).** The same row's `Values` cell becomes `Parameter P values 8,
16, 0..32.` and `SymbolDefinitionRecord.legal_values` (ranges; omitted from JSON when empty, so older
artifacts are byte-identical). With no positive stated default the adapter declares the smallest POSITIVE
legal value and names it in `isf_signal_width_parameter_chosen_defaults` as a choice: AXI 19 of the 20
placeholders (`DATA_WIDTH 8`, `ACT_*_WIDTH 16`, `MECID_WIDTH 16`, `AWCMO_WIDTH 2`, the rest 1); only
`MPAM_WIDTH` (no row) remains a placeholder. FSMGen strict: 0 diagnostics.

**A trap for the next person.** Legacy APB-D and AXI-H rules lower *"X must be valid"* to `(X VALID)`. FSMGen
does not reject the undeclared `VALID`: it invents a module **input port** for it (`input wire VALID` in the
generated SystemVerilog), sized from its first assignee, so the moment any driven signal is wider than 1 every
width-1 `(Y VALID)` fails the OperandContract. It has passed strict only because every width was 1. The
FSMGen half is reported upstream (`FSMGEN-REQUESTS.2`, `docs/fsmgen-issues/sf-isf-undeclared-rule-operand/`);
the SPECFORGE half (the current producer's APB chain mints none) is `.2a.vii.e`. Derived widths are requested
upstream as `FSMGEN-REQUESTS.1`.

Links: [[fsmgen-ignores-signal-direction]], [[isf-value-width-operand-contract]],
[[arithmetic-width-drops-the-declaration]], [[property-table-is-not-a-signal-inventory]].
