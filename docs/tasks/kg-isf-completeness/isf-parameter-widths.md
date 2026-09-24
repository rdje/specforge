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
  Children: `.c.i`, `.c.ii`, `.c.iii` (split `2026-09-24` after measuring the statements; see below).
  **Measured `2026-09-24`: the 39 live parameters are stated in four grammars, and one of them carries most.**
  (1) **A parameter table** — a header with a name column, a `Values` column and a `Default` column. It exists only
  in AXI-L among the 27 live chains: 22 tables, 33 rows, 23 with an integer default, 9 positive. It covers 26 of
  AXI-L's 27 width parameters (`MPAM_WIDTH` has no such row). Defaults are mostly `0` (the signal is absent by
  default: `MECID_WIDTH`, `USER_REQ_WIDTH`, …) or `-` (`DATA_WIDTH | 8, 16, … 1024 | -`, `ID_R_WIDTH | 0..32 |
  -`); positive ones include `ADDR_WIDTH 32`, `BRESP_WIDTH 2`, `Num_RP_AR 1`. The rows reach EvidenceIR
  today only as `source_fact` table-row text; the SourceIR table is `table_kind: unknown`, captioned `… property`.
  (2) **`P can be / must be a, b, or c`** — AHB `DATA_WIDTH` (8 … 1024), `HPROT_WIDTH` (0, 4, 7).
  (3) **`P is recommended to be between a and b`** — AHB `ADDR_WIDTH` (10, 64), `HMASTER_WIDTH` (0, 8); and a
  symbolic bound, `between 0 and DATA_WIDTH/2` (AHB user widths).
  (4) **The SIGNAL as subject** — APB-E never states its parameters: `PWDATA can be 8 bits, 16 bits, or 32 bits
  wide`, `PADDR can be up to 32 bits wide`, `PAUSER is recommended to have a maximum width of 128 bits`. The
  value reaches the parameter only through the signal's width cell.
  **The IR can carry a default but not a legal-value set.** SemanticIR already parses `Parameter P = D.` into a
  `Param` symbol (`parse_explicit_symbol_definition`), and `SymbolDefinitionRecord.value` holds `D`; nothing holds
  `8, 16, … 1024`. **And one trap is already armed**: the adapter renders every `Param` symbol in `(constants …)`,
  so the first `Param` minted for a width parameter would ALSO declare it as a constant, `.b`'s collision guard
  would then keep the signal at `(width 1)`, and the parameter would silently regress. `.c.i` must move
  `Param` symbols out of the constants path in the same change.

- ID: `KG-ISF-COMPLETENESS.2a.vii.c.i` · Status: `done` (`2026-09-24`; CODE) · Goal: **a parameter table's stated
  default becomes the parameter's default.** EvidenceIR mints `Parameter NAME = D.` for each row of a table whose
  header has a `Values` and a `Default` column, when the name is an identifier and `D` is a bare integer;
  SemanticIR's existing parser makes it a `Param` symbol. The adapter declares a parameter that sizes a signal
  with a positive `D` as `(P D)`; a `0` default keeps the placeholder, and the residual says the document's
  default is `0` (the signal is absent by default); `Param` symbols leave the `(constants …)` path. New statements
  are appended after every existing rule, so no statement id renumbers. Gates: `evidence --dry-run` over the 27
  live chains with both binaries (only AXI-L may move, and only by the new statements), every downstream stage
  rebuilt and diffed for AXI-L, FSMGen strict green, WIRE-BASED-100 unmoved.
  **Landed.** `synthesize_parameter_default_statements` / `parameter_default_statements_from_table`
  (`crates/specforge/src/ir/evidence.rs`) run after the last statement-reading surface, pre-incrementing the
  shared counter, so the new statements take the ids after the highest existing one. `stated_parameter_defaults`
  (`ir/isf_ir.rs`) keeps one integer default per `Param` and never chooses between two. The lowering pass moved
  ahead of the rules: with a real default the rule-value alignment must see the parameter's width, and before this
  it ran after them (harmless only while every default was `1`). **Result on AXI-L**: 23 `Parameter` statements
  (every existing id unchanged), 23 new `Param` symbols in SemanticIR and IntentIR (no existing symbol moved;
  IntentIR's identity summary counts them), and `.isf` `(params …)` gains `ADDR_WIDTH 32`, `ARSNOOP_WIDTH 4`,
  `AWSNOOP_WIDTH 4`, `BRESP_WIDTH 2`, `RRESP_WIDTH 2`; `Num_RP_AR`/`Num_RP_AWW` state `1`. That is 7 of 27 with a
  stated default; the other 20 are named in the residual with their reason (`0`: absent by default, or none).
  Nothing else in the `.isf` moved: no rule, no constant (`Max_Transaction_Bytes`, `Cache_Line_Size` are `Param`
  symbols that size no signal and are not emitted). **Found while verifying**: the first WIRE-BASED-100 run
  reported `seed_axi`/`seed_axi_temporal` as different; the HEAD binary had refused the freshly rebuilt chain
  (`topology is stale`) and scored nothing. Scored each side against its own chain (`--evidence-root` holding the
  pre-rebuild EvidenceIR for HEAD), both are identical at 1.000 — the resume pointer's standing rule.
  Commit: `KG-ISF-COMPLETENESS.2a.vii.c.i — a parameter table's default becomes the parameter's default`

- ID: `KG-ISF-COMPLETENESS.2a.vii.c.ii` · Status: `open` (`2026-09-24`; CODE) · Goal: **a parameter's legal
  values reach the IR**, so a parameter with no positive stated default takes its smallest positive legal value
  (`DATA_WIDTH` 8, `ID_R_WIDTH` 1, `ACT_R_WIDTH` 16). Needs a backward-compatible legal-value field on
  `SymbolDefinitionRecord` (absent when empty, so existing artifacts stay byte-identical) and a canonical
  statement for it; the choice of default is named in the residual as a choice, not a statement.

- ID: `KG-ISF-COMPLETENESS.2a.vii.c.iii` · Status: `open` (`2026-09-24`; CODE) · Goal: **the prose grammars**
  (2)-(4) above feed the same records: a parameter-subject clause (AHB), and a signal-subject width clause whose
  value reaches the parameter through that signal's width cell (APB). A symbolic bound (`DATA_WIDTH/2`) is not
  a legal value and stays out.

- ID: `KG-ISF-COMPLETENESS.2a.vii.d` · Status: `open` (`2026-09-24`; UPSTREAM) · Goal: **a derived width
  (`DATA_WIDTH/8`, `ceil(ADDR_WIDTH/8)`, 56 live signals) becomes statable.** FSMGen's published contract refuses
  every form `.a` probed. Prepare the reproducer bundle under `docs/fsmgen-issues/` for FSMGen to own; until it
  ships, each derived width is `.b`'s named residual.
  **Bundle filed `2026-09-24`** as `FSMGEN-REQUESTS.1`
  ([`sf-isf-derived-width-parameter`](../../fsmgen-issues/sf-isf-derived-width-parameter/README.md), signed off,
  awaiting the owner's forward); re-verified on FSMGen upstream HEAD `5a44ad49` as well as the pin, and it
  cites the `.fsm` `+size` width-expression surface as precedent. This leaf now owns only the SPECFORGE
  integration: once FSMGen ships R1-R3, lower each derived width to a derived `(params …)` entry.

- ID: `KG-ISF-COMPLETENESS.2a.vii.e` · Status: `open` (`2026-09-24`; MEASUREMENT) · Goal: **the two legacy
  findings are confirmed absent from the current producer, or fixed.** Replay today's readers over the legacy
  stratum (`corpus_chain_currency_11`'s method) and determine whether they still mint (i) a `VALID` rule value
  from *"must be valid"* and (ii) the non-parameter symbolic widths `.a` lists. A reader that still mints one is
  a defect fixed here; one that does not closes as evidence.
  **The finding has an FSMGen half, filed `2026-09-24`** as `FSMGEN-REQUESTS.2`
  ([`sf-isf-undeclared-rule-operand`](../../fsmgen-issues/sf-isf-undeclared-rule-operand/README.md)): FSMGen's
  strict mode accepts the undeclared `VALID` and invents an input port for it, against its own book 10. That
  half is FSMGen's to fix; this leaf keeps SPECFORGE's half (never emitting a prose word as a rule value).

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.vii.b` (CODE) — DONE `2026-09-24`

- [x] **REPRODUCE / MEASURE** — `.2a.vii.a`: `python3 scripts/measure_parameterised_signal_widths.py` gives 62 parameter + 56 derived widths over 3 live chains; HEAD `adapt --dry-run` renders every one as `(width 1)`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `IsfIr::from_intent_ir` (`crates/specforge/src/ir/isf_ir.rs`) parsed `render_isf_width_hint(w).parse::<u32>().unwrap_or(1)`, so every `WidthHint::Parametric` became `1`, and `IsfSignal` had no way to carry a parameter name.
- [x] **ADDRESSED (verified)** — `adapt --dry-run` with the new binary: AHB 7 params / 9 signals, APB-E 5 / 7, AXI-L 27 / 46 lower to `(width P)`; 9 / 7 / 40 derived widths named in `isf_signal_width_unlowered`. Five seam tests (`symbolic_signal_widths_keeps_only_a_single_symbolic_width`, `is_isf_identifier_accepts_only_a_bare_name`, `lower_parameter_widths_declares_parameters_and_residualizes_the_rest`, `lower_parameter_widths_changes_nothing_without_a_symbolic_width`, `render_declares_width_params_first_and_sizes_signals_by_name`), each RED-verified by neutralising its rule (numeric exclusion, disagreement, identifier gate, collision gate, render).
- [x] **NO REGRESSION** — `adapt --dry-run` over all 27 live chains: 24 byte-identical in `.isf` text and residuals, 3 changed as intended; FSMGen `--strict --check` 0 diagnostics on all three (unchanged from HEAD); WIRE-BASED-100: all 11 seeds (`eval-extraction --provider skip`) byte-identical between the HEAD and new binaries once the random temp-dir path is masked; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,667).
- [x] **GENERICITY (ADR 0006)** — the rule reads the IR's own width-hint type and FSMGen's identifier grammar; no parameter, signal, protocol or vendor name appears in it.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` gains *How wide is each signal?* and step 3 links to it; no book text described the replaced behaviour beyond step 3's `(width 1)` default, which now names the parameter case; KM card `isf-parameter-width-contract` (from `.a`) holds; `shipped_behavior` aggregate authority rebased; task root and `MEMORY.md` point at `.2a.vii.c`.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.vii.c.i` (CODE) — DONE `2026-09-24`

- [x] **REPRODUCE / MEASURE** — `.2a.vii.c`'s census: AXI-L's 22 `Name | Values | Default` tables state 23 integer defaults; HEAD `adapt --dry-run` declares every width parameter `(P 1)` and IntentIR holds 0 `Param` symbols.
- [x] **ROOT CAUSE (WHY + WHERE)** — no EvidenceIR rule read a parameter table (its rows reached `extracted_statements` only as `source_fact` table text), so SemanticIR's existing `Parameter P = D.` parser (`parse_explicit_symbol_definition`, `crates/specforge/src/ir/semantic.rs`) had nothing to parse; and `IsfIr::from_intent_ir` sent `Param` to `(constants …)`.
- [x] **ADDRESSED (verified)** — AXI-L rebuilt evidence -> validate -> semantic -> validate -> intent -> validate -> adapt -> validate: 23 statements, 23 `Param` symbols, 7 of 27 width parameters declared with the stated default. Tests `a_parameter_table_states_each_integer_default`, `a_table_without_values_and_default_columns_states_no_parameter`, `a_positive_stated_default_replaces_the_placeholder`, `stated_parameter_defaults_keeps_one_integer_value_per_parameter`.
- [x] **NO REGRESSION** — `evidence --dry-run` over the 27 live chains with the HEAD and new binaries: 26 byte-identical, AXI-L differs only by the 23 appended statements (no statement removed, no other field changed); `adapt --dry-run` over the 27: 24 byte-identical, AXI-L as intended, APB-E/AHB residual wording only; FSMGen `--strict --check` 0 diagnostics on all three; WIRE-BASED-100: 11 of 11 seeds identical (the two AXI seeds each scored against its own binary's chain); `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,671).
- [x] **GENERICITY (ADR 0006)** — the rule reads the table's own `Values` and `Default` header words and an identifier-shaped first cell; no parameter, protocol or vendor name.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` *How wide is each signal?*: the placeholder-only paragraph is replaced by the table rule, the two defaults that keep the placeholder, and a dated reach line; KM card `isf-parameter-width-contract` notes the stated defaults; `shipped_behavior` aggregate and `flow_census.json` rolled under this leaf; root and `MEMORY.md` point at `.c.ii`.
