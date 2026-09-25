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
  Children: `.c.i`, `.c.ii`, `.c.iii` (done; split `2026-09-24` after measuring the statements), `.c.iv`,
  `.c.v`, `.c.vi` (open).
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

- ID: `KG-ISF-COMPLETENESS.2a.vii.c.ii` · Status: `done` (`2026-09-25`; CODE) · Goal: **a parameter's legal
  values reach the IR**, so a parameter with no positive stated default takes its smallest positive legal value
  (`DATA_WIDTH` 8, `ID_R_WIDTH` 1, `ACT_R_WIDTH` 16). Needs a backward-compatible legal-value field on
  `SymbolDefinitionRecord` (absent when empty, so existing artifacts stay byte-identical) and a canonical
  statement for it; the choice of default is named in the residual as a choice, not a statement.
  **Landed.** EvidenceIR mints `Parameter NAME values 8, 16, 0..32.` from each parameter-table row whose
  `Values` cell reads in full (integers; ranges `a..b`, `a-b`, `a–b`; ascending only), after every `.c.i`
  default statement, so no statement id moves (`synthesize_parameter_values_statements`, sharing
  `parameter_table_rows` with `.c.i`). SemanticIR parses it into `SymbolDefinitionRecord.legal_values`
  (`ParameterValueRange { low, high }`, `skip_serializing_if` empty) on the parameter's `Param` symbol, and
  mints a value-less `Param` when the row states no default; two different lists for one parameter drop the
  lists, not the symbol. The adapter (`smallest_positive_legal_values`) takes the smallest positive entry when no
  positive default is stated, and names each in the new residual `isf_signal_width_parameter_chosen_defaults`
  (value, signals, why the stated default was unusable). **Result on AXI-L**: 28 statements (the 5 `True,
  False` rows state none), 23 `Param` symbols gain lists and 5 value-less ones are minted (`DATA_WIDTH`,
  `ID_W/R_WIDTH`, `LOOP_W/R_WIDTH`); nothing else in SemanticIR or IntentIR moved. 19 of the 20 placeholders
  become chosen defaults; the `.isf` changes in exactly 5 `(params …)` lines (`ACT_R/W_WIDTH 16`, `AWCMO_WIDTH
  2`, `DATA_WIDTH 8`, `MECID_WIDTH 16`; the other 14 choose 1, the placeholder's value); `MPAM_WIDTH` (no row)
  stays a placeholder. **Correction `2026-09-25` (`.c.iii`): "no row" is false** — AXI states `MPAM_WIDTH | K |
  0, 11, 12, 14, 15 | …` in a `Property | Issue | Values | AXI5 | ACE5-Lite | …` profile table with no `Default`
  column, which `.c.i`'s reader does not match, and in prose (`MPAM_WIDTH must be 11 …`). The claim restated
  `.c`'s census instead of re-deriving it. Owned as `.c.v`. **Found while landing**: the private constructor was first called `new`, and
  `helper_edges` moved +429 where +84 was the slice's own — `local_edges`' bare-name fallback attached 345
  std `…::new()` call sites to it. Renamed `named`; the tool defect is owned as `PRODUCTION-GRAPH-CENSUS-PIN.4`.
  Commit: `KG-ISF-COMPLETENESS.2a.vii.c.ii — a parameter's legal values reach the IR and choose its default`

- ID: `KG-ISF-COMPLETENESS.2a.vii.c.iii` · Status: `done` (`2026-09-25`; CODE) · Goal: **the prose grammars**
  (2)-(4) above feed the same records: a parameter-subject clause (AHB), and a signal-subject width clause whose
  value reaches the parameter through that signal's width cell (APB). A symbolic bound (`DATA_WIDTH/2`) is not
  a legal value and stays out.
  **Measured first** (all 27 live chains, every table with a `Width` column whose cell is an identifier and
  whose row mentions `can be`/`must be`/`recommended`/`up to`/`maximum width`): 24 rows, all in AXI-L, APB-E and
  AHB. The prose makes TWO kinds of statement, and the leaf's goal conflated them: a legal set (`can be`, `must
  be` + numbers) and a RECOMMENDATION (`is recommended to be between 10 and 64`). **Decision: a recommendation
  is not the legal set and gets its own field**, `SymbolDefinitionRecord.recommended_values`; the default is the
  smallest positive legal value inside a recommended range, else the smallest positive legal value, else the
  smallest positive recommended value, and the residual names which (`ChosenDefaultBasis`). **Decision:
  upper-bound-only clauses state nothing** (`PADDR can be up to 32 bits wide`, `a maximum width of 128 bits`):
  the lower bound is unstated, and the default they would give is the placeholder anyway. False friends
  (`must be valid when …`, `must be reflected …`) carry no number and state nothing.
  **Landed.** `synthesize_parameter_prose_statements` (`ir/evidence.rs`) reads a table with a `Width` and a
  `Description` column after the name, on a row whose width cell is an identifier; `parameter_value_clauses`
  splits the description at full stops and bullets and accepts a sentence whose subject is the parameter, the
  signal, or (a recommendation only) nothing, whole or not at all (`legal_value_clause`,
  `recommended_value_clause`). It mints `Parameter P values …` and `Parameter P recommended values A..B.` after
  every `.c.ii` statement. SemanticIR carries the second as `recommended_values` (not serialised when empty),
  never merged into `legal_values`; the adapter's `chosen_parameter_defaults` replaces `.c.ii`'s
  `smallest_positive_legal_values`. **Result**: AHB 8 statements, 7 new `Param` symbols; `.isf` `(ADDR_WIDTH 10)`
  (recommended), `(DATA_WIDTH 8)`, `(HPROT_WIDTH 4)` (legal), and 1 for `HMASTER_WIDTH`, `USER_REQ_WIDTH`,
  `USER_RESP_WIDTH` (recommended), so only `USER_DATA_WIDTH` (`DATA_WIDTH/2`) is a placeholder. APB-E 2
  statements, 1 `Param`; `(DATA_WIDTH 8)`; its four upper-bound-only parameters stay placeholders. AXI-L:
  evidence identical (its `must be valid` rows state nothing), residual wording only. **Found while
  verifying**: AHB states *"any value smaller than 32 or larger than 256 is not recommended"* for `DATA_WIDTH`,
  so the chosen 8 is legal but explicitly not recommended; owned as `.c.iv`. **Also corrected here**: the
  adapter's residuals and the book said a designer overrides a default "when instantiating the block"; FSMGen's
  published matrix (13k) gives ports *resolved positive integer widths* and lists parameter-driven shape
  elaboration as backlog, so both now say another width means regenerating. The three chains were re-adapted.
  Commit: `KG-ISF-COMPLETENESS.2a.vii.c.iii — prose states a parameter's legal and recommended values`

- ID: `KG-ISF-COMPLETENESS.2a.vii.c.iv` · Status: `open` (`2026-09-25`; CODE; found by `.c.iii`) · Goal: **a
  negative recommendation narrows the choice.** AHB's `DATA_WIDTH` rows add *"However, any value smaller than 32
  or larger than 256 is not recommended."* `.c.iii` reads only the positive `recommended to be between A and B`
  form, so AHB declares `(DATA_WIDTH 8)` — a legal value the document advises against. Read `any value smaller
  than A or larger than B is not recommended` (and its one-sided forms, if measured present) as the recommended
  range `A..B` on the row's parameter; the chooser then takes the smallest legal value inside it (AHB: 32,
  `LegalAndRecommended`). Measure the grammar's reach over the 27 live chains first; note the source's
  `smaller than32` (no space) in the persisted text. Gates as `.c.iii`. **Scope widened `2026-09-25`
  (`EXTRACTION-REPRESENTATION-GAPS.0`)**: the one-sided bounds `.c.iii` deliberately left unread (APB `PADDR
  can be up to 32 bits wide`, `a maximum width of 128 bits`) are the same missing capability, a range with one
  stated bound, so they are read here too; the IR needs a range whose lower bound is unstated, not an invented 0.

- ID: `KG-ISF-COMPLETENESS.2a.vii.c.v` · Status: `open` (`2026-09-25`; CODE; found by `.c.iii`) · Goal: **a
  per-interface-class property table states its parameters' values.** AXI's `Property | Issue | Values | AXI5 |
  ACE5-Lite | ACE5-LiteDVM | ACE5-LiteACP | AXI5-Lite` tables give each property's legal values and, per interface
  class, a fixed value or `-` (e.g. `MPAM_WIDTH | K | 0, 11, 12, 14, 15 | … | 0`; `DATA_WIDTH | H | 8 … 1024 | … |
  128 | -`). `.c.i`/`.c.ii` need a `Default` column, so none of it is read: `MPAM_WIDTH` stays a placeholder and
  every per-class constraint is dropped. Read the `Values` column into `.c.ii`'s records; carry each class
  column as a per-profile value only once the IR has a place for an interface class (measure, then decide).
  Measure the table family's reach over all 27 live chains first.

- ID: `KG-ISF-COMPLETENESS.2a.vii.c.vi` · Status: `open` (`2026-09-25`; MEASUREMENT then CODE; found by `.c.iii`)
  · Goal: **a yes/no configuration parameter, and the signals it enables, reach the IR.** AXI's parameter tables
  state `PROT_Present | True, False | True`, `INSTPRIV_Present`, `Shared_Credits_AW/W/AR`; `.c.i`/`.c.ii` read
  integers only, so the 5 rows state nothing, and nothing relates a `*_Present` property to the signals it adds
  or removes. SpecForge's half: read boolean parameters (value set and default) and, where the document states
  it, the property -> signal presence relation. The `.isf` half (an optional port) is FSMGen's; it is filed
  through the FSMGen requests lane, not here.

- ID: `KG-ISF-COMPLETENESS.2a.vii.f` · Status: `open` (`2026-09-25`; MEASUREMENT; found by `.c.iii`) · Goal:
  **WIRE-BASED-100 can see parameter extraction.** `.c.i`-`.c.iii` took IntentIR from 0 to 36 `Param` symbols and
  every seed scored identically before and after, because no gold label names a parameter. Add parameter gold
  (name, default, legal and recommended values as the document states them) to the AXI, AHB and APB seeds, so a
  regression in any of these readers fails a scored gate instead of passing unseen.

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

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.vii.c.ii` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — AXI-L's 33 parameter-table rows state a `Values` cell; at HEAD none reaches the IR (`SymbolDefinitionRecord` had no field for it), and `adapt --dry-run` declares 20 width parameters at the placeholder `1`, `DATA_WIDTH` among them.
- [x] **ROOT CAUSE (WHY + WHERE)** — `.c.i` read only the `Default` column (`parameter_default_statements_from_table`, `crates/specforge/src/ir/evidence.rs`); SemanticIR's `Param` grammar (`parse_explicit_symbol_definition`, `ir/semantic.rs`) had no value-set form and its accumulator required a value; `lower_parameter_widths` (`ir/isf_ir.rs`) had only the stated default to use.
- [x] **ADDRESSED (verified)** — AXI-L rebuilt evidence -> validate -> semantic -> validate -> intent -> validate -> adapt -> validate: 28 statements, 23 symbols gain `legal_values`, 5 value-less `Param` symbols minted, 19 chosen defaults named in `isf_signal_width_parameter_chosen_defaults`, 5 `(params …)` lines changed. Tests `a_parameter_table_states_each_legal_value_list`, `a_values_cell_reads_in_full_or_not_at_all`, `a_parameter_values_statement_carries_legal_values_on_its_param_symbol`, `legal_values_are_not_serialised_when_empty`, `the_smallest_positive_legal_value_is_chosen_and_named_as_a_choice`, `smallest_positive_legal_values_skips_zero_and_never_chooses_between_lists`; four RED-verified by neutralising their rule (the chosen default, the descending-range refusal, the conflicting-list drop, the zero exclusion).
- [x] **NO REGRESSION** — `evidence --dry-run` over the 27 live chains with the HEAD and new binaries: 26 identical, AXI-L differs only by the 28 appended statements (none removed or changed); SemanticIR and IntentIR move only in `symbol_definitions` (plus IntentIR's identity summary); `adapt --dry-run` over the other 26: 24 identical, APB-E/AHB placeholder-residual wording only (`.isf` text identical; both re-adapted and validated); FSMGen `--strict --check` 0 diagnostics on AXI-L before and after; WIRE-BASED-100: 11 of 11 seeds identical (the AXI seeds each against its own binary's chain); `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,677); flow census rolled (+11 / +32 / +84 / +4), boundary counts unchanged.
- [x] **GENERICITY (ADR 0006)** — the reader keys on the table's own `Values`/`Default` header words and on number and range syntax; the adapter rule is arithmetic over the IR's ranges; no parameter, protocol or vendor name appears in either.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` *How wide is each signal?* gains the legal-value rule with a worked table, the choice residual, and the dated AXI reach; KM card `isf-parameter-width-contract` gains the legal-value paragraph and two questions; `shipped_behavior` aggregate and `flow_census.json` rolled under this leaf; root and `MEMORY.md` point at `.c.iii`; the graph-tool defect found here is owned as `PRODUCTION-GRAPH-CENSUS-PIN.4`.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.vii.c.iii` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — census over the 27 live chains (tables with `Width` + `Description` whose width cell is an identifier and whose prose says `can be`/`must be`/`recommended`/`up to`/`maximum width`): 24 rows in AXI-L, APB-E and AHB; at HEAD none reaches the IR (AHB and APB-E hold 0 `Param` symbols).
- [x] **ROOT CAUSE (WHY + WHERE)** — only the `Name | Values | Default` table form was read (`parameter_table_rows`, `crates/specforge/src/ir/evidence.rs`); nothing bound a signal row's width parameter to its description prose, and `SymbolDefinitionRecord` had no place for a recommendation distinct from the legal set.
- [x] **ADDRESSED (verified)** — AHB and APB-E rebuilt evidence -> validate -> semantic -> validate -> intent -> validate -> adapt -> validate: AHB 8 statements / 7 `Param` symbols, `.isf` `ADDR_WIDTH 10`, `DATA_WIDTH 8`, `HPROT_WIDTH 4`; APB-E 2 / 1, `DATA_WIDTH 8`. Tests `a_signal_table_states_its_width_parameters_values_in_prose`, `a_value_clause_reads_only_a_complete_numeric_list`, `chosen_parameter_defaults_prefers_a_recommended_legal_value_and_skips_zero`, and the extended `a_parameter_values_statement_carries_legal_values_on_its_param_symbol`; four RED-verified by neutralising their rule (recommendation kept apart, whole-sentence recommendation, subject gate, legal-within-recommended preference).
- [x] **NO REGRESSION** — `evidence --dry-run` over the 27 live chains with the HEAD and new binaries: 25 identical, AHB +8 and APB-E +2 appended statements (none removed or changed); SemanticIR/IntentIR move only in `symbol_definitions` (+ IntentIR identity summary), no existing symbol changed; `adapt --dry-run` over the other 25: 24 identical, AXI-L residual wording only (`.isf` text identical); the three chains re-adapted and validated with the final binary; FSMGen `--strict --check` 0 diagnostics on AHB and APB-E before and after; WIRE-BASED-100: 11 of 11 seeds identical (AHB/APB seeds each against its own binary's chain); `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,679); flow census rolled (+8 / +28 / +57 / 0), boundary counts unchanged.
- [x] **GENERICITY (ADR 0006)** — the reader keys on the table's own `Width`/`Description` header words, the row's own names and an English value-clause grammar; no parameter, signal, protocol or vendor name.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` gains the prose rule (worked table, the three-step default order, what states nothing), the dated AHB/APB reach, the known weak choice (`.c.iv`), and two corrections (FSMGen genericity; `MPAM_WIDTH` does have values); KM card `isf-parameter-width-contract` updated with both corrections; `shipped_behavior` and `flow_census.json` rolled under this leaf; root and `MEMORY.md` point at `.c.iv`; new leaves `.c.iv`, `.c.v`, `.c.vi`, `.2a.vii.f` own what this slice found.
