# EXTRACTION-REPRESENTATION-GAPS — FSMGen requests and the reset contract

- Part ID: `fsmgen-requests-and-reset`
- State: `legacy`

<!-- extraction-representation-gaps-task-source-region:fsmgen-requests-and-reset:start -->
- ID: `EXTRACTION-REPRESENTATION-GAPS.4`
  Status: `open` (`2026-09-25`; REQUEST)
  Goal: **FSMGen can state an optional signal.** AXI sets 14 width parameters to `0` by default (re-derived for the
  bundle; `.0` wrote 20, which counted parameters with no default too) (*the signal
  is absent unless enabled*) and gates signals on yes/no properties (`PROT_Present`). FSMGen's published
  contract refuses a zero-valued parameter as a width (13a, 13k *Interface ports*) and has no optional or
  conditional port. Draft, reproduce and sign off one `feature_request` covering a zero-width (absent) port, a
  port present only when a parameter enables it, and the value an absent input is tied to (AXI: *an attached
  Subordinate must have its AWSNOOP input tied LOW* when the Manager omits it; found by `.2`).
  **Bundle signed off `2026-09-25`**: [`sf-isf-optional-signal`](../upstream/fsmgen/sf-isf-optional-signal/README.md),
  forwarded by the owner `2026-09-25`. Reproduced on the pin `54ac1c22` and upstream `354844fe1` (FSMGen's feature
  matrix is byte-identical at both, so nothing has shipped since the pin). Requests R1 zero width = absent, R2
  `(present P)`, R3 `(absent-value V)`, R4 (optional) presence kept across instantiation. Measured need: AXI 14
  width parameters default to 0 (21 signals); 18 tie-off statements across AXI/AHB/APB. **Found while
  building it**: strict mode accepts an unknown port clause and ignores it (`(present PROT_Present)` and
  `(bogus-clause xyz)` both pass and generate the same SystemVerilog as no clause); owned as `.19`. This leaf
  closes when FSMGen ships and SPECFORGE integrates, or the request is rejected or withdrawn. SpecForge halves: `KG-ISF-COMPLETENESS.2a.vii.c.vi` (read the
  presence properties); the adapter's positive-width substitution is replaced once the feature ships.

- ID: `EXTRACTION-REPRESENTATION-GAPS.5`
  Status: `open` (`2026-09-25`; REQUEST)
  Goal: **FSMGen can carry a parameter's legal values and recommended range.** SpecForge now carries
  `legal_values` and `recommended_values` on 36 parameter symbols (`KG-ISF-COMPLETENESS.2a.vii.c.ii`/`.c.iii`);
  FSMGen's `(params (P default))` holds a default only (13k *Actor report metadata and params*), so the rest
  stops at IntentIR, and nothing checks that an override is legal. Draft, reproduce and sign off a
  `feature_request` for a declared value set / range on an actor parameter, checked on every default and
  override.
  **Bundle signed off `2026-09-25`**: [`sf-isf-parameter-value-set`](../upstream/fsmgen/sf-isf-parameter-value-set/README.md),
  forwarded by the owner `2026-09-25`. Reproduced on `54ac1c22` and `354844fe1`: every spelling is refused with
  *params entries require '(NAME value)'* (13k *Actor report metadata and params*). R1 `(values …)`/`(range …)`, R2
  checked on overrides, R3 reported in `actor_params[]`, R4 (optional) advisory `(recommended …)`, R5 (optional) an
  HDL elaboration check. Measured need: 36 parameters (AXI 28, AHB 7, APB 5 1).

- ID: `EXTRACTION-REPRESENTATION-GAPS.6`
  Status: `open` (`2026-09-25`; REQUEST)
  Goal: **a generated block stays generic in its width parameters.** FSMGen gives ports *resolved positive
  integer widths* and lists *parameter-driven shape elaboration* as backlog (13k *Interface ports*, *Reusable ISF
  libraries*), so the HDL is fixed at the declared default and a different width means regenerating.
  Specifications describe generic blocks (`HRDATA` is `DATA_WIDTH` bits). Draft, reproduce and sign off a
  `feature_request` for HDL whose port widths remain module parameters, citing FSMGen's own backlog entry and
  checking FSMGen's feedback records for an earlier request first.
  **Bundle signed off `2026-09-25`**: [`sf-isf-generic-port-widths`](../upstream/fsmgen/sf-isf-generic-port-widths/README.md),
  forwarded by the owner `2026-09-25`. Sharper than expected: the generated module DECLARES `parameter DATA_WIDTH =
  32` but sizes the port `output reg [31:0] WDATA` (register and reset too), and a default of 64 gives `[63:0]`;
  the width follows the default, never the parameter, so an instantiation override silently does nothing.
  R1 ports follow the parameter, R2 overrides re-specialize, R3 (if R1 is declined) no misleading parameter, R4
  (optional) `+size` records the dependency. The base case `sf-isf-derived-width-parameter` presumes. 62 ports.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **a real specification's clock and reset reach the system contract.** 0 of 27 live chains carry a
  `system_contract`, so `assess_isf_renderability` blocks all 27 and no current chain emits an `.isf` file; the
  carrier the adapter reads for clock/reset distribution (`infrastructure_signals`) is empty on all 27 too.
  `build_system_contract` (`ir/semantic.rs`) reads only canonical `Clock X.` / `Reset X …` statements and needs all
  seven fields (clock, reset, kind, polarity, assertion and release timing, target); no rule derives them from a
  signal table (`ACLK | Clock source | Global clock signal`, `ARESETn | … | active-LOW`). FSMGen can state it
  (13a *Clock and reset*: `(reset (rst_n async active_low))`). Measure what the 7 signal-bearing live documents
  state, decide which fields the adapter truly needs, and derive the contract from the document's own grammar.
  Closes when `.7.a`-`.7.e` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.a`
  Status: `done` (`2026-09-25`; MEASUREMENT; SpecForge)
  Goal: what each signal-bearing live document states about its clock and reset (persisted EvidenceIR, HEAD
  `01ed8d80`). The adapter needs four fields: clock, reset, kind (`sync`/`async`) and polarity. FSMGen's
  clause (13a *Reset*) has no fifth, and the other three contract fields follow from the kind.

  | Document | Clock | Reset | Polarity stated | Kind stated | Contract today |
  |---|---|---|---|---|---|
  | AXI | `ACLK` (table) | `ARESETn` (table, prose) | active-LOW (prose `0549`) | asserted asynchronously, deasserted synchronously (`0549`) | none: the table cell states no kind |
  | AHB | `HCLK` (table) | `HRESETn` (table, prose) | active LOW (table `0246`, prose `0855`) | the same (`0855`) | none, as AXI |
  | APB-E | `PCLK` (table) | `PRESETn` (table) | active-LOW (table `0164`) | **not stated** | none |
  | Wishbone | `CLK_I` (prose `0385`) | `RST_I` (prose `0388`) | only as a rule over every interface signal (`0380`) | "purely synchronous" (`0433`) | none: no signal table |
  | ADIv6 | `TCK` and `SWCLK`, one per port | none for the interface; *the SWD interface does not include a reset signal* (`1957`) | — | — | none |
  | I2C | `SCL`, `SCLH`, `USCL`, one per mode | none | — | — | none |
  | I2S | `SCK` | none | — | — | none |

  Only a table cell that states a kind yields a `Reset` statement, and the grammar wants the kind and the polarity
  in one statement. The four AMBA-style documents split them across a table row and a prose sentence.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.b`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **AXI and AHB reach a contract from their own sentences.** The reset grammar accepts a statement
  that gives the kind alone or the polarity alone, and `build_system_contract` merges them, refusing any
  disagreement. The table rule emits the polarity it reads. A prose reader emits the kind and polarity of the
  one declared signal the sentence calls *the reset signal*. It reads *asserted asynchronously* as `async` and
  *asserted synchronously* as `sync`. It refuses an asynchronous deassertion (the model releases on the clock),
  two candidate resets, or both polarities. APB then carries a polarity and no kind, so it is still refused.
  **Landed**: AXI and AHB emit `manager.isf` with `(reset (ARESETn async active_low))` and `(reset (HRESETn
  async active_low))`, strict-clean. Emitting them exposed `.22` (rules drop or invent their condition).

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.c`
  Status: `done` (`2026-09-26`; CODE; SpecForge; checklist below)
  Goal: Wishbone's clock and reset come from prose, since it has no signal table: *The clock input [CLK_I]*,
  *The reset input [RST_I]*, *a purely synchronous protocol*. Its polarity is only stated as a rule over every
  interface signal (*All WISHBONE interface signals MUST use active high logic*). Reading that needs the rule's
  scope (which signals are *WISHBONE interface signals*) to be grounded, not assumed.
  **Landed.** Three readers, each from a sentence the document states: the clock from *The clock input [CLK_I] …*
  (only a clock *input*: I2C's *serial clock signal SCL* is a line its initiator drives, `.7.e`); the kind from *the
  WISHBONE reset [RST_I] signal that uses a purely synchronous protocol*; the polarity from the class rule, which
  reaches `RST_I` only through *the SLAVE interface MUST include the following signals: … [RST_I]* and *WISHBONE
  MASTER and SLAVE interfaces*. A heading (*CLOCK GENERATION*) had also parsed as a second clock and refused the
  contract; a clock is now declared only by a sentence. Wishbone's contract is `CLK_I`, `RST_I` synchronous
  active-high, and it emits `slave.isf`, strict-clean. That file's port set is `.32`'s: *The SLAVE asserts either the
  [ACK_I], [ERR_I] or [RTY_I] signals* grounds the MASTER's port names on the SLAVE.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.d`
  Status: `done` (`2026-09-26`; DECISION + CODE; SpecForge; checklist below)
  Goal: APB-E states its reset's polarity but not its kind. FSMGen's clause has no *kind not stated*: a flat
  `(reset PRESETn)` means `sync`. Decide: an implementation choice recorded as a residual, like the parameter
  defaults of `KG-ISF-COMPLETENESS.2a.vii.c.iii`, or a request for an unstated kind.
  **Decision (`2026-09-26`): an implementation choice, listed**, as `.4` already does for a width the document
  leaves open. The contract records the kind as `not_stated` (its timing and target too); the adapter emits the kind
  that keeps the stated polarity at the pinned FSMGen, where `async` always comes out active-low and `sync`
  active-high (`.21`), so `async` for APB-E's active-LOW `PRESETn`, and lists `isf_reset_kind_not_stated`. A request
  for an unstated kind is not raised now: once `.21`'s fix is published either kind keeps the polarity, and the
  residual already says the kind is not the document's. **Landed.** APB-E emits `requester.isf` for the first time,
  strict-clean. Emitting it exposed one lowering defect, fixed here: a windowed event lowered as `(monitor (within
  SIGNAL N))` ignored its value, so APB's *PADDR, PWRITE, and PWDATA must be valid* became *PADDR is true within a
  cycle*; a windowed event now lowers only when its value resolves to `1`. The three defects behind those rules are
  `.29`-`.31`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.e`
  Status: `done` (`2026-09-26`; DECISION; SpecForge; implementation `.7.e.i`)
  Goal: the documents with no system reset (I2C, I2S, ADIv6's SWD port) and those with one clock per port or
  mode (ADIv6, I2C). FSMGen gives an actor that omits `(reset ...)` the reset `rst_n async active_low`
  (13a *Actor Declaration*): SPECFORGE must never omit the clause and let that default stand for the document.
  13a also says *each domain owns zero or one reset* under `(clock-domains ...)`, which may state a reset-less
  clock without inventing one. Decide the route, and whether FSMGen needs a single-clock reset-less form.
  **Decision (`2026-09-26`).** Measured on the persisted IR: I2C, I2S and ADIv6 yield no system contract, and their
  clock-like signals are protocol wires the initiator drives (I2C *the controller's SCL output*; ADIv6 *Signal SWCLK
  is output*; I2S relates no actor to `SCK`). (1) **No FSMGen request.** FSMGen's published contract already states a
  clock with no reset: a single-domain `(clock-domains (domain … (clock C) :default))` block, whose *clock-only no-reset
  domains* lower through the single-clock path (13a *Multi-Clock Domains*). SpecForge emits that block for a document
  that states no reset, and never omits `(reset …)` to let `rst_n` default in. (2) **The initiator's own clock is not
  the protocol clock it drives.** No document states it, so it is an implementation clock, chosen and listed, as `.7.d`
  lists a chosen reset kind; a target takes the protocol clock as its clock only where the document says it samples
  on it. (3) **Implementation waits on the actor graph.** Emitting these documents now would state their current actor
  graphs: I2C's initiator is the phrase *optional open-drain*, and I2S relates no actor to its clock
  (`KG-ISF-COMPLETENESS.1`, agent-surface fidelity). The route is `.7.e.i`, after their initiators are grounded.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.e.i`
  Status: `open` (`2026-09-26`; CODE; SpecForge; after `KG-ISF-COMPLETENESS.1` grounds these documents' initiators)
  Goal: **a document with no reset, and an initiator whose clock the document does not name, lower as `.7.e` decided.**
  Record in the system contract a clock with no reset (I2S `SCK`, SWD's `SWCLK`), and which actor drives the clock;
  emit the single-domain `(clock-domains …)` block with no reset; for an initiator that drives the protocol clock,
  choose an implementation clock and list it. One clock per port or mode (ADIv6's `TCK` and `SWCLK`, I2C's `SCL`,
  `SCLH`, `USCL`) is one actor per port or mode, each with its own domain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.8`
  Status: `done` (`2026-09-25`; CODE; SpecForge defect, ACTIVE; found by `.2`, widened the same day)
  Goal: **a drive is lowered only from a level the document states, resolved through the signal's polarity.**
  Found as a latent case: `isf_literal_value` (`ir/isf_ir.rs`) maps `asserted` to `1` for every signal and the
  adapter never reads `signal_polarities` (51 records), so an active-low rule would lower inverted (0 live rules
  reach it). **Widened on reading the lowering loops: the defect is active and broad.** Of the 250 drive rules in
  the live adapters' `.isf` text (6 documents), the signal-constraint loop ignores `constraint_kind` and defaults a
  missing value to `1`, the invariant loop does the same, and IntentIR's
  `signal_constraint_to_temporal_invariant_kind` folds `MustBeHigh`, `MustBeLow`, `MustBeAsserted`,
  `MustBeDeasserted` and `MustBeValue` into one `MustBeAsserted` without carrying the level or `negated`; the
  conditional-rule loop drives `1` unless the action text contains `not`. Measured: AXI *AWLOCK must be
  deasserted* and *AWSTASHLPIDEN … must be LOW* are emitted as `(AWLOCK 1)`, `(AWSTASHLPIDEN 1)`; APB *PADDR must
  remain stable* and *PAUSER must be valid* as `(PADDR 1)`, `(PAUSER 1)`; AXI *AWCMO must be 0b000* as `1`; 46 of 53
  lowered conditional rules have the action text `must` alone. FSMGen strict accepts all of it (valid syntax), so
  only reading the values shows it; no `.isf` file is emitted today (`.7`), which is why this must land before `.7`.
  Fix: IntentIR invariants carry the stated level and negation in `target_value`; every adapter path derives a
  drive's value from a stated level (`1`/`0`, `asserted`/`deasserted` through polarity, a stated literal) and
  otherwise records the obligation in one counted residual, never a default; an unresolvable polarity in a guard
  residualizes the rule rather than dropping the guard.
  **Landed.** IntentIR: `signal_constraint_level` (`ir/intent.rs`) puts the stated level, negation applied, in
  each signal-constraint invariant's `target_value` (`0`, `1`, `asserted`, `deasserted`, or the stated value).
  Adapter (`ir/isf_ir.rs`): the three rule loops became `lower_obligations`, whose value comes from
  `signal_constraint_level`, the invariant's `target_value`, or `action_stated_level` (the action's own `be <level>`),
  resolved by `resolve_level` through `signal_polarity_map`; `guard_from_candidates` and `drive_level` do the
  same for temporal contracts in both `classify_actor_contract` and its test oracle; `isf_literal_value` is gone.
  What is refused lands in `isf_obligations_not_lowered`. **Result** on the 6 chains whose IntentIR changed (their
  sealed IntentIR no longer verified under the new binary, so each was rebuilt: `intent` → validate → `adapt` →
  validate): drive rules **250 → 95**; AXI 149 → 82 with **44 values corrected** (`AWLOCK`, `AWSTASHLPIDEN`,
  `AWSTASHNIDEN`, `AWSTASHNID`, `AWIDUNQ` 1 → 0; `AWCMO` 1 → `2'd0`) and 18 rules that now lower because their
  stated value no longer collides with an invented one (`SYSCOACK 0`); APB 38 → 5 (`PSTRB` 1 → 0); AHB 32, ADIv6 5
  and Wishbone 10 → 0, every one of them invented; I2C 11 → 3. 166 obligations are listed instead (53 for an
  unstated polarity, 63 with no level, 61 naming a word); conflicting drives fell 38 → 20 and rule/transaction
  overlaps 7 → 3. EvidenceIR was already right: it refines `asserted` only through a resolved polarity
  (`apply_signal_polarity_to_constraints`); the adapter was the only stage that guessed. **Found while landing**:
  6 SemanticIR invariant ids name two records each (`.18`), and recovering the 53 polarity refusals needs polarity
  extraction (`.17`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.9`
  Status: `open` (`2026-09-25`; CODE; SpecForge; found by `.2`)
  Goal: **a value word lowers to the value it names.** Five rule values are residualized as *not an ISF literal*
  although FSMGen accepts them: `ZERO` (2; the number 0) and the enum members `INCR` (2) and `SHAREABLE` (1)
  (13k *Enum member values*: enum members are legal rule and drive values). Map number words to numbers and a
  value naming a member of the target signal's own enum to that member, declaring the enum. `ARDOMAIN` *Must be
  Shareable (0b01 or 0b10)* names a set of two encodings, so it is a membership assertion (`.11`), not a drive.

<!-- extraction-representation-gaps-task-source-region:fsmgen-requests-and-reset:end -->
