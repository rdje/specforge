# KG-ISF-COMPLETENESS — isf parameter widths

- Part ID: `isf-parameter-widths`
- State: `active`
## Post-migration record

- ID: `KG-ISF-COMPLETENESS.2a.vii` · Status: `open` (`2026-09-24`; MEASUREMENT then CODE; opened on the owner's
  explanation of parameterised blocks) · Goal: **a parameterised signal width lowers to a parameter, not to
  `(width 1)`.** A specification that writes a width as a parameter (`HRDATA | DATA_WIDTH`, `HADDR | ADDR_WIDTH`)
  describes a generic block: one design that a user specialises when they instantiate it, by giving the
  parameter a value. The lowered `.isf` must keep that genericity. Today it doesn't: `.2a.i` emits only a
  concrete grounded width, and every symbolic width falls back to `(width 1)`. That is wrong for AHB
  `HRDATA`/`HWDATA`/`HADDR` and every other `*_WIDTH` signal.
  **No owner decision is pending.** The `2026-09-24` report said this waited on the owner, which misread `.2a`.
  Its header still reads `deferred … OWNER decision`, but its body records it resolved on `2026-06-18` (width
  `.2a.i`, direction `.2a.ii`). The header sits in a sealed legacy part and cannot be edited; this record
  supersedes it. The owner has since confirmed the design intent: parameterised sizes are decided by the
  designer at instantiation, and the toolchain must model generic blocks.
  **The downstream contract already allows it.** FSMGen's published book
  (`subs/fsmgen/docs/book/src/13a-actor-interface.md`, *Interface*) accepts `(params (DATA_W 32))` and `(width
  DATA_W)`: the width stays a named, authored parameter, and the default only makes the block elaborable.
  An undeclared name, or an arbitrary expression such as `DATA_WIDTH/8`, fails closed
  (`[[fsmgen-ignores-signal-direction]]`). So the adapter must declare every parameter it uses.
  **Design, to be measured before it is coded:**
  1. Emit `(width P)` for a signal whose grounded width is exactly one parameter `P`, together with a `(params
     (P <default>))` declaration.
  2. The default comes from the document: a stated default or recommended value; failing that, the smallest
     width in a stated list of legal widths (AHB lists `DATA_WIDTH` as 8, 16, 32, …, 1024). If the document
     states none, the default is an explicit, reviewable placeholder, flagged as a residual in the adapter output
     and never presented as a fact.
  3. A derived width (`DATA_WIDTH/8`, `ceil(ADDR_WIDTH/8)`) cannot be a bare parameter. Measure how FSMGen's
     published contract lets it be stated (a declared constant or a derived parameter) before choosing. If it
     cannot be stated, it stays a named residual rather than becoming a wrong literal.
  **Measure first:** over every lowered adapter, count the signals whose grounded width is a single parameter, a
  derived expression, or concrete; list each parameter and what its document states about values; confirm with
  FSMGen `--strict --check` (its published CLI) that the emitted `.isf` elaborates. Hard gates: WIRE-BASED-100
  unmoved, the gold `.isf` differences read line by line, and no new FSMGen `--strict` diagnostics.
