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
  **Children:** `.a` (measured, done), `.b` (done), `.c`, `.d`, `.e`.

- ID: `KG-ISF-COMPLETENESS.2a.vii.a` · Status: `done` (`2026-09-24`; MEASUREMENT, no product change) · Goal:
  **measure the design before it is coded.** Instrument: `python3 scripts/measure_parameterised_signal_widths.py
  [--signals]` over every persisted IntentIR, classifying each signal the adapter emits (first-seen name, every
  interface record's and actor port's width hint) as concrete / parameter / derived / conflict / none.
  **Reach: 3 live chains.** `adapt` refuses a schema-1 IntentIR (`legacy/proofless and inspection-only`), so the
  change reaches schema-2 chains only, and exactly three carry a symbolic width: **AXI-L 46 parameter + 40
  derived, APB-E 7 + 7, AHB 9 + 9 — 62 parameter and 56 derived signals, 0 conflicts, 39 parameter names.** No
  live parameter names a declared signal. HEAD's `adapt --dry-run` reproduces each persisted `.isf` text byte for
  byte (residuals 12 / 16 / 50), so these are HEAD baselines, and all three texts pass FSMGen `--strict --check`
  today, although each adapter is `blocked` on its clock/reset contract.
  **FSMGen's published contract, probed** (`perl subs/fsmgen/bin/fsmgen --strict --check --json`, APB-D base):
  `(params (P N))` + `(width P)` is accepted; an undeclared `P` fails closed, and so does a default of `0` (*must
  resolve to a positive integer*). **A derived width cannot be stated in any published form**: a param expression
  (`(PSTRB_WIDTH (/ DATA_WIDTH 8))`: *unsupported aggregate/list parameter leaf `/`*), a constant expression
  (*requires a non-negative integer literal*), an inline `(width (/ DATA_WIDTH 8))`, and a package constant
  (book 07: *arbitrary expressions are not type-width symbols*) are all refused. The exact identity
  `ceil(x/8) = (x+7)/8` does not help, because `/` itself is refused. **Simulated on the three live texts**: every
  parameter signal rewritten to `(width P)` with a declared default elaborates strict-green (APB-E 5 params, AHB
  7, AXI-L 27).
  **What the documents state.** AXI-L publishes parameter tables headed `Name | Values | Default | Description`
  (`ADDR_WIDTH | 1..64 | 32`, `DATA_WIDTH | 8, 16, … 1024 | -`, `BRESP_WIDTH | 0, 2, 3 | 2`, `MECID_WIDTH | 0, 16
  | 0`, `ID_W_WIDTH | 0..32 | -`, `Num_RP_AR | 1-8 | 1`). AHB and APB state values in the signal row's prose
  (`DATA_WIDTH can be 8, 16, … 1024`, `ADDR_WIDTH is recommended to be between 10 and 64`, `HPROT_WIDTH must be 0,
  4, or 7`). **None of it reaches the IR**: IntentIR `symbol_definitions` holds 515 enums and 0 `Param`, and the
  AHB statements survive only in normalized Markdown or inside a table-row statement. **Zero is a legal width in
  both families** (`MECID_WIDTH` default 0, `HPROT_WIDTH` 0): it means the signal is absent, and FSMGen refuses it
  as a default, so a zero default is not a width the adapter can emit.
  **Decisions.** (1) Code the adapter first (`.b`) with a placeholder default of `1`: the elaborated widths are
  then exactly today's, so the change cannot raise a strict diagnostic, and every placeholder is a named residual.
  (2) Document-stated defaults need a reader (`.c`); the IR's unused `SymbolDefinitionKind::Param` is the carrier,
  and the adapter's constant path already renders it. (3) Derived widths are an upstream contract gap (`.d`).
  (4) Two findings are legacy-only and are owned as `.e`.
  **Legacy findings (frozen chains, evidence about themselves per `CORPUS-CHAIN-CURRENCY.11`).** APB-D (14 rules)
  and AXI-H (2) lower *"X must be valid"* to `(X VALID)`, an undeclared operand that FSMGen sizes from its first
  assignment: with `PADDR` at a parameter width, APB-D fails the OperandContract on `PAUSER`/`PBUSER`/`PRUSER`.
  APB-E, the current producer's APB chain, mints none. And 6 legacy chains carry symbolic widths that are not
  parameters: `LAVALID`/`LRVALID` (LTI, signal names), `V` (AXI-H), `C`/`D` (TileLink), `Preamble` (GIC),
  `Context`/`Translation` (LTI), `reset_req` (Avalon); `abits` (RISC-V debug) is a real parameter.
  Commit: `KG-ISF-COMPLETENESS.2a.vii.a — measured: 62 parameter widths are emittable, 56 derived ones are not`

- ID: `KG-ISF-COMPLETENESS.2a.vii.b` · Status: `done` (`2026-09-24`; CODE) · Goal: **the adapter lowers a
  single-parameter width to `(width P)` and declares `(params (P 1))`, flagging each placeholder.** For a signal
  whose width hints are exactly one identifier (`.a`'s `parameter` class): emit `(width P)`; declare every used
  `P` once in a `(params …)` block; record one residual per placeholder default naming the parameter and the
  signals it sizes. A name that is also a declared signal stays `(width 1)` with a residual (a width is not a
  wire). A derived width stays `(width 1)` with a residual naming its expression (owned by `.d`). Gates: the
  three live `.isf` stay FSMGen strict-green; WIRE-BASED-100 unmoved; every other chain's `.isf` byte-identical.
  **Landed.** `IsfSignal.width_param` and `IsfIr.width_params` (`crates/specforge/src/ir/isf_ir.rs`):
  `symbolic_signal_widths` reads every width hint of a signal (interface records and actor ports) and keeps a
  signal only when its hints are one symbolic width and no number; `lower_parameter_widths` runs last in
  `from_intent_ir`, so a parameter name is checked against every declared actor name; `render` emits the
  `(params …)` block first and `(width P)` per signal. Two refinements of the design above, both deliberate:
  **one summary packet per kind** (`isf_signal_width_parameter_defaults`, `isf_signal_width_unlowered`) rather
  than one per parameter, which is the proportionate form the storage residuals already use; and the collision
  guard covers every declared name (signals, clock/reset, constants, enum types and members, storage), not
  only signals. **Result**: AHB 7 params / 9 signals, APB-E 5 / 7, AXI-L 27 / 46; unlowered 9 / 7 / 40, all
  expressions and no collision. Each changed `.isf` differs from HEAD only by the `(params …)` block and the
  re-sized lines (read line by line), and each passes FSMGen `--strict --check` with 0 diagnostics. The other
  24 live chains are byte-identical in text and residuals. The three chains were re-adapted with the final
  binary (AXI-L also re-validated, since its persisted artifact embedded a report: same 2 findings).
  **Found while landing**: `pipeline/isf-adapter.md` is the chapter now driving the `shipped_behavior` byte
  warning (84.1% -> 86.8% with this section, 113,711 bytes at `ee78ecdb`) and no open leaf owned it; owned as
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.34b`.
  Commit: `KG-ISF-COMPLETENESS.2a.vii.b — a parameterised width lowers to (width P) with a declared parameter`

- ID: `KG-ISF-COMPLETENESS.2a.vii.c` · Status: `open` (`2026-09-24`; CODE) · Goal: **a parameter's default comes
  from the document.** Read parameter declarations (a `Name | Values | Default` table row; a signal row's
  `P can be …` / `must be …` / `is recommended to be between …` clause) into IntentIR `Param` symbols
  carrying the stated default and legal values; the adapter takes the stated default, else the smallest
  POSITIVE legal value, else keeps `.b`'s flagged placeholder. Universal grammar only (ADR 0006).

- ID: `KG-ISF-COMPLETENESS.2a.vii.d` · Status: `open` (`2026-09-24`; UPSTREAM) · Goal: **a derived width
  (`DATA_WIDTH/8`, `ceil(ADDR_WIDTH/8)`, 56 live signals) becomes statable.** FSMGen's published contract refuses
  every form `.a` probed. Prepare the reproducer bundle under `docs/fsmgen-issues/` for FSMGen to own; until it
  ships, each derived width is `.b`'s named residual.

- ID: `KG-ISF-COMPLETENESS.2a.vii.e` · Status: `open` (`2026-09-24`; MEASUREMENT) · Goal: **the two legacy
  findings are confirmed absent from the current producer, or fixed.** Replay today's readers over the legacy
  stratum (`corpus_chain_currency_11`'s method) and determine whether they still mint (i) a `VALID` rule value
  from *"must be valid"* and (ii) the non-parameter symbolic widths `.a` lists. A reader that still mints one is
  a defect fixed here; one that does not closes as evidence.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.vii.b` (CODE) — DONE `2026-09-24`

- [x] **REPRODUCE / MEASURE** — `.2a.vii.a`: `python3 scripts/measure_parameterised_signal_widths.py` gives 62 parameter + 56 derived widths over 3 live chains; HEAD `adapt --dry-run` renders every one as `(width 1)`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `IsfIr::from_intent_ir` (`crates/specforge/src/ir/isf_ir.rs`) parsed `render_isf_width_hint(w).parse::<u32>().unwrap_or(1)`, so every `WidthHint::Parametric` became `1`, and `IsfSignal` had no way to carry a parameter name.
- [x] **ADDRESSED (verified)** — `adapt --dry-run` with the new binary: AHB 7 params / 9 signals, APB-E 5 / 7, AXI-L 27 / 46 lower to `(width P)`; 9 / 7 / 40 derived widths named in `isf_signal_width_unlowered`. Five seam tests (`symbolic_signal_widths_keeps_only_a_single_symbolic_width`, `is_isf_identifier_accepts_only_a_bare_name`, `lower_parameter_widths_declares_parameters_and_residualizes_the_rest`, `lower_parameter_widths_changes_nothing_without_a_symbolic_width`, `render_declares_width_params_first_and_sizes_signals_by_name`), each RED-verified by neutralising its rule (numeric exclusion, disagreement, identifier gate, collision gate, render).
- [x] **NO REGRESSION** — `adapt --dry-run` over all 27 live chains: 24 byte-identical in `.isf` text and residuals, 3 changed as intended; FSMGen `--strict --check` 0 diagnostics on all three (unchanged from HEAD); WIRE-BASED-100: all 11 seeds (`eval-extraction --provider skip`) byte-identical between the HEAD and new binaries once the random temp-dir path is masked; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,667).
- [x] **GENERICITY (ADR 0006)** — the rule reads the IR's own width-hint type and FSMGen's identifier grammar; no parameter, signal, protocol or vendor name appears in it.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` gains *How wide is each signal?* and step 3 links to it; no book text described the replaced behaviour beyond step 3's `(width 1)` default, which now names the parameter case; KM card `isf-parameter-width-contract` (from `.a`) holds; `shipped_behavior` aggregate authority rebased; task root and `MEMORY.md` point at `.2a.vii.c`.
