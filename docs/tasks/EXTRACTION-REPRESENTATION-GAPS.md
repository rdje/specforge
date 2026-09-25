# EXTRACTION-REPRESENTATION-GAPS: one ledger of what a specification states, what SpecForge extracts, and what `.isf` can say — each gap owned on its own side

## Metadata

- Tree ID: `EXTRACTION-REPRESENTATION-GAPS`
- Status: `active`
- Roadmap lane: the program objective (ADR 0033, amended by ADR 0053) and `R0` downstream contract hygiene
- Created: `2026-09-25`
- Last updated: `2026-09-25`
- Owner: repo-local workflow
- Decision: [ADR 0053](../decisions/0053-extraction-and-isf-requests-advance-in-parallel.md)

## Goal

Every piece of design intent a chip specification states is **extracted** by SpecForge into canonical IR and
is **representable** in `.isf`. Where either fails, the gap is named, attributed to the side that owns it, and
owned by an open leaf: a SpecForge extraction leaf, an FSMGen request (through `FSMGEN-REQUESTS`), or both.
The owner's directive (`2026-09-25`): *SpecForge does its part and FSMGen does its part.*

The ledger is a book chapter, [`reference/representation-gaps.md`](../book/src/reference/representation-gaps.md),
so the owner can see every gap and who holds it without reading code.

## Non-Goals

- Filing a request without a reproduced gap. ADR 0053 keeps `PROCESS.md`'s evidence bar: cited
  specification text, and FSMGen's refusal reproduced on the pin and on upstream.
- Forwarding requests. The owner forwards; `FSMGEN-REQUESTS` records it.
- Fixing a SpecForge extraction gap inside this tree when an existing tree already owns that category. The
  ledger names the owner; the work stays where it belongs.
- Inspecting or changing `subs/fsmgen`. FSMGen is read only through its published book and CLI.

## Acceptance Criteria

- The ledger lists every gap the two censuses (`.2`, `.3`) find, with evidence, the owner of each half and a
  status, and a check refuses a row whose owner is not an open work unit (`.1`).
- Every FSMGen-owned row has a request bundle signed off under `PROCESS.md`, or a written reason it does not
  need one (already in FSMGen's published contract at the pin, or withdrawn).
- Every SpecForge-owned row has an open leaf in the tree that owns its category.
- The ledger is re-checked on every FSMGen pin refresh; the refresh leaf records the re-check.

## Task Tree

- ID: `EXTRACTION-REPRESENTATION-GAPS`
  Status: `active` (`2026-09-25`)
  Children: `.0`-`.28`

- ID: `EXTRACTION-REPRESENTATION-GAPS.0`
  Status: `done` (`2026-09-25`; DECISION + DOC)
  Goal: **open the lane.** Record the owner's directive as ADR 0053 (amending ADR 0033 item 4: an FSMGen
  request may precede the IntentIR carrier when an open SpecForge leaf owns the extraction); publish the
  ledger chapter seeded only with rows verified in `KG-ISF-COMPLETENESS.2a.vii.a`-`.c.iii` and against FSMGen's
  published book at the pin (13a *Interface*, 13k feature matrix rows *Actor report metadata and params*,
  *Interface ports*, *Reusable ISF libraries*); state its update rule; route it from the book summary, the
  FSMGen requests chapter and the ISF adapter chapter; add the lane to the roadmap's owner region.

- ID: `EXTRACTION-REPRESENTATION-GAPS.1`
  Status: `done` (`2026-09-25`; CHECK)
  Goal: **a ledger row cannot cite a closed or missing owner.** Declare the ledger's owner column as an
  `OWNERSHIP-CITATIONS` region. First extend `scripts/check_ownership_citations.pl`: it resolves a leaf's
  status only from the two-line `- ID:` / `Status:` form in a tree's root file, so a leaf of a partitioned tree
  (`KG-ISF-COMPLETENESS.2a.vii.c.iv`, declared as one line `- ID: … · Status: …` in a part under
  `docs/tasks/<tree>/`) reads as nonexistent. Resolve the one-line form in the root and in the tree's parts,
  with a self-test case for each form and a refused case for a closed leaf.
  **Landed.** `resolve_leaf_status` reads both grammars over the root and every part file under
  `docs/tasks/<tree in lower case>/` (`tree_part_texts`), and fails closed when a leaf's declarations disagree;
  the self-test calls it directly instead of re-stating its regex (the old leaf-shape case tested a copy), and
  gains five cases (one-line in a root, in a part, a closed one-line leaf, disagreeing declarations, a longer id
  sharing the prefix): 16/16. The ledger's open-gaps table is the `gap_owners` region, with its 10 owners
  classified `current_owner`. The limit mattered beyond this ledger: the one-line form is how every partitioned
  tree and many roots declare leaves, so any such citation in a declared region would have been refused as
  nonexistent.

- ID: `EXTRACTION-REPRESENTATION-GAPS.2`
  Status: `done` (`2026-09-25`; MEASUREMENT)
  Goal: **the IntentIR-to-`.isf` census.** For every IntentIR family, over every live chain: how many records
  the IR carries, how many the adapter lowers, and, for the rest, why — the IR lacks an operand (SpecForge), or
  FSMGen's published contract cannot state it (FSMGen; cite the section), or both. The adapter's residual packets
  are the starting evidence. Each FSMGen-owned result becomes a request leaf here; each SpecForge-owned result a
  leaf in the tree that owns the family. Every row lands in the ledger with its evidence command.
  **Measured** (`python3 scripts/measure_intent_to_isf_reach.py`, read-only; result below under *`.2` — the
  IntentIR-to-`.isf` census*): the adapter reads 18 of IntentIR's families and never reads ~20; **every one of
  the 27 live adapters is `blocked`, all 27 for want of a clock/reset contract**, so no current chain hands FSMGen an
  `.isf` file; 150 residual packets fall into 8 attributable classes. New leaves `.7`-`.16`; `.3` re-anchored.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3`
  Status: `open` (`2026-09-25`; MEASUREMENT)
  Goal: **the PDF-to-IR census: extend the closed ontology.** `docs/research/intent-ontology-coverage.md`
  (`2026-05-31`) is the recall denominator, and it has no category for configuration parameters, presence
  properties or interface-class profiles, which is why the `.2a.vii.c` gaps were found by hand. Measure, per
  document category, which kinds of design intent the live corpus states that no IR carrier holds; extend the
  ontology with each, and give each an owning SpecForge leaf. The ledger gains a row per category.
  **Re-anchored by `.2`**: the per-category denominator already exists as
  `doctrine/spec_to_intent_category_contract.json` (six categories, each with semantic families mapped to IntentIR
  surfaces; `platform-system-ip`'s `component_topology_and_domains` family maps to none), so measure against it
  and extend it rather than the older ontology. It also owns the **7,477 untyped records** `.2` found
  (`constraints` 5,229, `behaviors` 2,091, `assumptions` 157: a free-text `statement` plus provenance): count how
  many state something no typed record holds, which is captured-but-not-understood intent.

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
  Status: `open` (`2026-09-25`; CODE; SpecForge)
  Goal: Wishbone's clock and reset come from prose, since it has no signal table: *The clock input [CLK_I]*,
  *The reset input [RST_I]*, *a purely synchronous protocol*. Its polarity is only stated as a rule over every
  interface signal (*All WISHBONE interface signals MUST use active high logic*). Reading that needs the rule's
  scope (which signals are *WISHBONE interface signals*) to be grounded, not assumed.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.d`
  Status: `open` (`2026-09-25`; DECISION; SpecForge, then FSMGen if a request is needed)
  Goal: APB-E states its reset's polarity but not its kind. FSMGen's clause has no *kind not stated*: a flat
  `(reset PRESETn)` means `sync`. Decide: an implementation choice recorded as a residual, like the parameter
  defaults of `KG-ISF-COMPLETENESS.2a.vii.c.iii`, or a request for an unstated kind.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.e`
  Status: `open` (`2026-09-25`; DECISION; SpecForge, then FSMGen if a request is needed)
  Goal: the documents with no system reset (I2C, I2S, ADIv6's SWD port) and those with one clock per port or
  mode (ADIv6, I2C). FSMGen gives an actor that omits `(reset ...)` the reset `rst_n async active_low`
  (13a *Actor Declaration*): SPECFORGE must never omit the clause and let that default stand for the document.
  13a also says *each domain owns zero or one reset* under `(clock-domains ...)`, which may state a reset-less
  clock without inventing one. Decide the route, and whether FSMGen needs a single-clock reset-less form.

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

- ID: `EXTRACTION-REPRESENTATION-GAPS.10`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **a normative constraint keeps its condition, and is lowered as what it is.** 38 rules are dropped as
  conflicting drives under an empty guard (`AWSNOOP` 0 vs 1, `AWBURST` INCR vs WRAP) and 7 more overlap a
  transaction's drive: the source states each value under a condition that the rule lost. Read, not assumed:
  `AWSNOOP` 0 (`dyn_sigcon_0068`) comes from *"A Manager that only uses Opcodes where AWSNOOP is LOW can set
  AWSNOOP_WIDTH to 0 which omits the AWSNOOP output … An attached Subordinate must have its AWSNOOP input tied
  LOW"*: a configuration condition and a tie-off for an absent signal (`.4`), not a drive. Classify every dropped
  rule's lost condition (configuration, transaction, phase) before designing the fix. Recover the conditions; and decide, from the lowered actor's perspective, whether a constraint on a
  signal it drives is a drive, and one on a signal it reads an `assume`, rather than lowering every constraint as
  a drive.

- ID: `EXTRACTION-REPRESENTATION-GAPS.11`
  Status: `open` (`2026-09-25`; CODE; SpecForge; found by `.2`)
  Goal: **stability and exclusion obligations lower as assertions.** 7 rules are residualized as *bare stability
  across tick phases has no supported .isf construct* (`HSIZE must not be changed`) and 3 same-cycle exclusions as a
  0-cycle window (`HEXOKAY must not be asserted in the same cycle as HRESP`). The first claim is stale: FSMGen's
  matrix (13k *Immediate verification checks*) has `(assert …)` with `(stable SIG)`, `(=> A B)` and plain boolean
  exclusion. Lower them as assertions; an actor with no transaction to host one needs `.15`. First make
  `classify_invariant_text` (`ir/intent.rs`) stop labelling unmatched text `MustRemainStable` ("default for chip
  invariants"): lowered as an assertion, that default would state a stability the document never did.

- ID: `EXTRACTION-REPRESENTATION-GAPS.12`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **a protocol state machine is typed.** 51 protocol records (45 states, 5 operations, 1 edge timing) are
  residualized because they carry no typed transitions, guards, initial state, encoding or actions, and the typed
  state-machine families (`regular_states`, `state_transitions`) are empty on all 27 live chains. FSMGen can state
  state machines and transactions; SpecForge must extract their parts. No open leaf owned this after
  `SWD-SERIAL-EXTRACTION.7d` introduced the residuals.

- ID: `EXTRACTION-REPRESENTATION-GAPS.13`
  Status: `open` (`2026-09-25`; MEASUREMENT; SpecForge, possibly FSMGen; found by `.2`)
  Goal: **every register field and reset value is accounted for.** 158 register fields are not lowered (no bit
  range, a shared name, or overlap) and 71 registers keep a documented reset that is not integer-lowerable
  (symbolic or implementation-defined, partial coverage, over-wide, overlapping). Break both down by cause; the
  capture causes are SpecForge's, and a symbolic or implementation-defined reset value that FSMGen's `(reset V)`
  cannot state becomes an FSMGen request here if the breakdown shows it.

- ID: `EXTRACTION-REPRESENTATION-GAPS.14`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **platform topology lowers to composition.** `signal_connectivity` (235 records in 6 documents) is never
  read by the adapter, which emits one flat actor; FSMGen composes actors (13f). Measure which documents state a
  multi-component topology and lower it to FSMGen composition where the IR carries it.

- ID: `EXTRACTION-REPRESENTATION-GAPS.15`
  Status: `open` (`2026-09-25`; REQUEST; found by `.2`)
  Goal: **FSMGen can state an always-on assertion outside a transaction.** `(assert …)`, `(assume …)` and `(cover …)`
  are transaction-level clauses (13d, 13k *Immediate verification checks*), although each lowers to an always-on
  clocked property. A protocol invariant (`HSIZE must not change during a burst`) belongs to no transaction. Draft,
  reproduce and sign off a `feature_request` for actor-level assert/assume/cover.
  **Corrected `2026-09-25`**: `.2` wrote that most SPECFORGE actors have no transaction to host one; measured, **1
  of the 7** signal-bearing live documents (I2S) has none, and the emitted property is always-on with no
  transaction-state gating, so elsewhere hosting is placement only. The need is that one actor class plus not
  misattributing an invariant to its host. **Bundle signed off `2026-09-25`**:
  [`sf-isf-actor-level-assertion`](../upstream/fsmgen/sf-isf-actor-level-assertion/README.md), forwarded
  by the owner `2026-09-25`; reproduced on `54ac1c22` and `354844fe1` (*unknown actor clause 'assert'*, also in an actor with no
  transaction). FSMGen's own guidance supports it: obligations belong in the verification family, not a
  transaction body. R1 actor-level placement, R2 documented placement semantics, R3 (optional) a clock domain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.16`
  Status: `open` (`2026-09-25`; REQUEST; found by `.2`)
  Goal: **FSMGen can say "X holds a known value when Y".** Specifications state validity obligations (`PADDR,
  PWRITE and PWDATA must be valid` when `PSEL` is asserted; 11 residualized records, and many more captured as
  constraints). FSMGen's property predicates are `stable`, `changed`, `rose`, `fell` and `past` (13k); there is no
  known-value (`$isunknown`-style) predicate. Draft, reproduce and sign off a `feature_request`. SpecForge's half,
  never emitting `VALID` as a value, is `KG-ISF-COMPLETENESS.2a.vii.e`.
  **Bundle signed off `2026-09-25`**: [`sf-isf-known-value-predicate`](../upstream/fsmgen/sf-isf-known-value-predicate/README.md),
  forwarded by the owner `2026-09-25`; `(known PADDR)` and `(! (isunknown PADDR))` refused on `54ac1c22` and
  `354844fe1`, `(stable PADDR)` accepted in the same position. R1 `(known SIG)`, R2 (optional) `(isunknown SIG)`, R3
  (optional) several signals. Measured need: 28 *must be valid* statements, 17 typed obligations, 11 held-back
  temporal rules.

- ID: `EXTRACTION-REPRESENTATION-GAPS.17`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.8`)
  Goal: **every signal's polarity the document states reaches the IR.** After `.8`, `asserted`/`deasserted` lower
  only through a stated polarity, and the 27 live chains carry 51 polarity records across 4 documents (5 of them
  active-low). Measure where the documents state polarity (an `active-LOW`/`active-HIGH` cell or phrase per
  signal; a document-wide statement such as *signals are active-HIGH unless stated*) and read it: a stated
  convention is document grammar, not spelling (ADR 0037 still forbids inferring it from a `…n` suffix).

- ID: `EXTRACTION-REPRESENTATION-GAPS.18`
  Status: `open` (`2026-09-25`; CODE; SpecForge integrity; found by `.8`)
  Goal: **an invariant id names one invariant.** 6 SemanticIR invariant ids occur twice in 4 live chains (AXI
  `invariant_completion_0b0_completion_not_required` from `statement_4186` and `statement_4289`; ADIv6 two, each
  a list item and its table cell restating one sentence), and IntentIR carries both records under one
  `tinv_…` id. An id that names two records breaks every consumer that keys by id (found when a comparison
  keyed by it compared the wrong pair). Mint distinct ids, or merge restatements into one record with both
  provenances, and gate uniqueness at validation.

- ID: `EXTRACTION-REPRESENTATION-GAPS.19`
  Status: `open` (`2026-09-25`; QUESTION; found by `.4`)
  Goal: **learn whether FSMGen's strict mode should reject an unknown interface-port clause.** Building `.4`,
  `(output AWPROT (width 3) (present PROT_Present))` and `(… (bogus-clause xyz))` both passed `--strict --check`
  with no diagnostic, and the generated SystemVerilog was identical to the port without the clause (pin
  `54ac1c22` and upstream `354844fe1`). FSMGen's book says strict mode narrows *"toward the supported intentional
  surface"* (10) and lists the port grammar (13k) but does not say an unknown clause is an error, so under
  `PROCESS.md` §1 this is a question, not a bug: file it in `docs/FSMGEN_FEEDBACK.md`. It matters to SPECFORGE
  because a strict pass cannot then prove that a clause SPECFORGE emits was understood.
  **Filed `2026-09-25`** as the open question `strict-unknown-port-clause` in `docs/FSMGEN_FEEDBACK.md`, forwarded
  by the owner `2026-09-25`. **Also here**: the ledger's contract capped open records at 8, which the five requests
  had just reached; raised to 12 (`doctrine/live_document_size/fsmgen_feedback.json`), because ADR 0053 expects
  more concurrent requests and the file sits at 173 of 256 lines and 9,674 of 32,768 bytes of its health target
  (an open record is about 9 lines, so 12 records stay under the line target). Records leave the open list when a
  request is integrated, rejected or withdrawn. This leaf closes when FSMGen answers and the answer is recorded.
  **Packet since `2026-09-25`** (ADR 0054): [`sf-isf-strict-unknown-port-clause`](../upstream/fsmgen/sf-isf-strict-unknown-port-clause/README.md),
  which adds the `bogus-clause` probe and shows each generated module equal to the clause-free one.

- ID: `EXTRACTION-REPRESENTATION-GAPS.20`
  Status: `open` (`2026-09-25`; REQUEST; found reading FSMGen's 2026-06-16 answer while building `.15`)
  Goal: **FSMGen can carry a transaction's phase membership.** SPECFORGE extracts, per transaction, which signals
  each protocol phase involves (`phase_membership`, `KG-ISF-TRANSACTIONS.2i`): 18 transactions, 14 phase groups, 105
  port memberships over five documents; only 2 of the 18 have grounded steps, and the adapter emits a transaction
  only with steps, so the membership of the other 16 reaches no `.isf`. `.2`'s census missed it because the
  `transactions` family is one the adapter reads. On `2026-06-16` FSMGen answered SPECFORGE that *"checked
  transaction phase-group metadata"* is *"the right future ISF shape"* *"if this becomes load-bearing for
  `.isf`"*; this measures that it is, and files the request.
  **Bundle signed off `2026-09-25`**: [`sf-isf-transaction-phase-groups`](../upstream/fsmgen/sf-isf-transaction-phase-groups/README.md),
  forwarded by the owner `2026-09-25`; `(phases …)` refused as an unsupported body clause and the actor-level spelling
  as an unknown actor clause on `54ac1c22` and `354844fe1`. R1-R3 are FSMGen's own first slice (checked metadata,
  validation, reporting), R4 (optional) a transaction whose only content is its phases. **Also here**: the
  `fsmgen_issue_packets` collection (`docs/upstream/fsmgen/**/*.md`) allowed 32 files and this bundle made 35 (each
  bundle adds a README, a SIGNOFF and expected notes); raised to 96 files with its byte and line totals raised in
  step (96 × the per-file limits, ADR 0032), because ADR 0053 expects many more requests and the collection used
  1,930 of 16,384 lines and 131 KB of 1 MiB, so only the file count bound. Noted for `.19`: unknown
  transaction and actor clauses ARE refused; only port clauses are silently ignored.

- ID: `EXTRACTION-REPRESENTATION-GAPS.21`
  Status: `open` (`2026-09-25`; fixed in FSMGen's unpublished development line; closes with `FSMGEN-REFRESH-INTEGRATE-10`)
  Goal: **an `.isf` reset keeps the polarity it states, whatever its name.** At the pin `54ac1c22` an `async`
  reset always comes out active low: `(rst_n async active_high)` passes `--strict` as active low, and without
  `--strict` so does `(rst async active_high)`. `--strict` also refuses `(ARST async active_low)`, `(nSRST async
  active_low)`, `(rst_b async active_low)`, `(rst async active_high)` and `(rst_n sync active_low)` as a *legacy
  or misleading* spelling. **Not filed**: building the bundle showed upstream HEAD `354844fe1` honours all eight
  probe cases under `--strict`, and exports place the fix at `f16130a4f` (`2026-09-25`, *preserve ISF reset policy
  through lowering*; `07f7e8333` alone did not fix the `.isf` path). A name cannot carry a polarity anyway: the
  owner knows `X_n`, `X_b`, `Xn`, `Xb`, and the corpus adds ADIv6's `nSRST`/`TRST*` and Wishbone's `[#RST_I]`,
  `[/RST_I]`, `[N_RST_I]` (`0381`). AXI's and AHB's resets are unaffected. Closes when the pin includes `f16130a4f`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.22`
  Status: `done` (`2026-09-25`; CODE; SpecForge defect; found when `.7.b` made AXI emit `manager.isf`; checklist below)
  Goal: **a rule keeps the condition its sentence states, or is not lowered.** `sanitize_rule_condition`
  (`ir/isf_ir.rs`) turns a multi-word condition into no guard, so the rule drives unconditionally. It turns a
  one-word condition into `(== <word lowercased> 1)` without checking that the word is a declared signal or that
  `1` is its stated level. Upstream, `synthesize_temporal_invariants` (`ir/intent.rs`) builds an invariant
  without its constraint's condition (24 of 81 constraint-derived invariants, in AXI, APB, AHB and I2C). Over the
  live chains 16 emitted rules drop a stated condition, 8 by each path (AXI `rule_70` *AWSNOOP is not CMO*). AXI
  *SYSCOREQ … must be deasserted when ARESETn is asserted* is emitted twice: guarded `(== ARESETn 0)` from its
  temporal rule, and unguarded from its invariant. 3 rules are guarded on an undeclared name. AXI *ACTIVATEREQ,
  ACTIVATEACK and ASKSTOP must be LOW during reset* became `(rule constraint_45 (== reset 1) (ACTIVATEREQ 0))`,
  and FSMGen then adds an input port `reset` (`.23`). A condition becomes a guard only on a declared signal at a
  stated level (`resolve_level`); *during reset* names the contract's reset (`ARESETn` at its active level);
  anything else is listed with the obligations not lowered.
  **Design (`2026-09-25`, measured)**: of 80 conditions on lowerable records, most are prose (*it is driving the
  final read transfer*, *PSEL and PWRITE are asserted*). One reader, `condition_guard`, turns a whole `SIGNAL is
  LEVEL` or *(in/during) reset* into a guard and refuses the rest (`ConditionNotLowered`). An invariant's
  `condition_signal` keeps its `condition_value`, and a constraint whose condition cannot be read yields no invariant
  (an invariant holds unconditionally). A temporal rule's guard candidates are a conjunction: all are kept, as
  FSMGen's `(& …)`, or the rule is refused. Today `guard_from_candidates` keeps the first usable one; none of the
  24 emitted temporal rules loses one yet, but 4 contracts carry 2-3 candidates.
  **Corrected `2026-09-25` (`.25.a`)**: the claim below missed the temporal path. A temporal rule whose condition
  gave no antecedent carried no trace of it, so 4 emitted rules still dropped theirs (AXI's three *… LOW during
  reset*, I2C's *the Acknowledge cycle*); `.25.a` lists them. The measurement had read the temporal rules' guard
  candidates, not their source constraints' conditions.
  **Landed**: no emitted rule drops or invents a condition (16 and 3 before); AXI 89 → 80 rules, 12 guarded; the
  conjunction exposed a fused contract with contradictory conditions, refused and owned by `.25`; APB's *PSEL is
  asserted* against the declared `PSELx` family is `.24`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.23`
  Status: `done` (`2026-09-25`; no addendum needed — fixed in FSMGen's development line)
  Goal: **the acknowledged `sf-isf-undeclared-rule-operand` fix covers a rule's guard too.** At the pin
  `54ac1c22`, `(rule r1 (== bogus_name 1) (flag 0))` passes `--strict --check` and FSMGen adds `input bogus_name`.
  Upstream HEAD `354844fe1` refuses it (*operand 'bogus_name' read by rule 'r1' guard*), and a binary search over
  exports puts the refusal of both the guard and the report's own value case at `c6b8bc783` (`2026-09-24`). The fix
  already covers guards, so no addendum is sent; it lands with `FSMGEN-REFRESH-INTEGRATE-10`, which also closes
  `FSMGEN-REQUESTS.2`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.24`
  Status: `open` (`2026-09-25`; DECISION `.24.a` done, CODE `.24.b` open; SpecForge; found by `.22`)
  Goal: **a signal family with one instance per target is read as one.** APB states *The Requester generates a
  PSELx signal for each Completer* (`0168`); AHB states *Each Subordinate has its own select signal HSELx* (`0277`)
  and *An address decoder provides a select signal, HSELx, for each Subordinate* (`0658`). Per the owner, `PSEL`
  is the generic name and every instance (`PSELA`, `PSELB`, …) follows `PSEL`'s contract, so APB's *PSEL is
  asserted* is a condition on each instance. Today SpecForge declares `PSELx` as one 1-bit port and leaves
  conditions naming `PSEL` unread (`.22`: APB's *PSEL is asserted* yields no invariant). Decide how the generic
  name is tied to its family: the document's own *for each* statement grounds the family, and the identity must
  not come from the trailing `x` (ADR 0037). Then read it, and decide whether FSMGen can state a per-target port
  family or needs a request. Closes when `.24.a` and `.24.b` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.24.a`
  Status: `done` (`2026-09-25`; MEASUREMENT + DECISION; SpecForge)
  Goal: what the live documents state about a per-target family and its generic name, what each stage reads
  today, and the route. **Measured** over the persisted IR of the 27 live chains
  (`.project-data/tmp/erg24/family_census.py`, `partial_conditions.py`, scratch):
  - Per-target statements: APB `0168` (*The Requester generates a PSELx signal for each Completer*), AHB `0277`
    (*Each Subordinate has its own select signal HSELx*) and `0658` (*… a select signal, HSELx, for each
    Subordinate*). No other live document states one (AXI `0715`, *one PENDING signal per channel*, names a set of
    channel signals, not one per target).
  - The generic name: APB writes `PSEL` in 26 statements and `PSELx` in 11; AHB `HSEL` in 5 and `HSELx` in 12.
  - The identity: APB states it, writing the same role phrase before both (*Select signal, PSELx*, *Select
    signal, PSEL*), which `document_signal_coreferences` has read since `WIRE-BASED-100.4a`. AHB never does: its
    `HSEL` sentences (*HSEL asserted if present*, *an HSEL output*, *HSEL to be tied HIGH on the Subordinates*)
    share no role phrase with `HSELx`, and no AHB constraint names `HSEL`.
  - What is read: all 4 APB constraints conditioned on `PSEL` (`sigcon_0007`, `0012`, `0015`, `0016`) reach IntentIR
    as contracts guarded on `PSELx`. `.22`'s *PSEL is asserted yields no invariant* is the invariant reader
    (`extract_condition_from_text`, exact or case-folded only); the contract carries the same condition, so no
    intent is lost, only the two readers disagree. `sigcon_0015` keeps 2 of its 4 predicates, a defect of the
    condition reader, not of the family (`.26`). All 4 state *must be valid*, so none lowers yet
    (`KG-ISF-COMPLETENESS.2a.vii.e`, `.16`).
  **Decision.** (1) A generic name is tied to its family only by what the document states, today a shared role
  phrase. A per-target sentence grounds that one declared signal has an instance per target, but it names one
  identifier, so it ties nothing. ADR 0037 stands: nothing is read from the trailing `x`, and AHB's `HSEL` stays
  unread. (2) The family lowers one instance at a time, each under the generic contract, as the owner stated. An
  actor that reads it (a Completer, a Subordinate) has one instance, under the declared name. An actor that drives
  it (the Requester, a decoder) drives one per target, in a number neither document states; a single-actor `.isf`
  states one, and must say so. (3) **No FSMGen request.** FSMGen's published contract already has the shape: a
  requester declares one `(output PSEL)` (13a's APB example), a multi-peripheral interconnect fans out a decoded
  `PSEL` to each completer child (14h), and AHB decoding emits one `HSEL_*` per mapped window (`HSEL_REGS`, 14i).
  The fan-out is composition, `.14`.
  **Found while measuring**: of the 16 temporal rules whose stated condition yields a guard, 5 keep only part of it
  (`.26`); both emitted `.isf` files declare ports whose direction is a default rather than grounded on the lowered
  actor (`.27`); the ownership-citation registry the ledger uses is at 60 of 64 records (`.28`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.24.b`
  Status: `open` (`2026-09-25`; CODE; SpecForge)
  Goal: **a per-target family is typed intent, and every reader resolves the generic name alike.** Record a
  per-target family from the document's own sentence (signal, target role, statements) through IntentIR; persist
  the co-reference once in SemanticIR so IntentIR's invariant reader resolves `PSEL` as the contract path does; and
  have the adapter list a family lowered as one instance on an actor that drives it. Generic: the grammar is *for
  each ROLE*, *each ROLE has its own … SIGNAL*, with ROLE a declared actor; no signal name is read.

- ID: `EXTRACTION-REPRESENTATION-GAPS.25`
  Status: `open` (`2026-09-25`; CODE; SpecForge defect; found by `.22`)
  Goal: **a contract keeps one sentence's conditions, and an *until* clause is not a condition.** AXI's contract
  `fused:contract_temporal_signal_constraint_sigcon_0043+…_0044` joins *When AWAKEUP is asserted with SYSCOREQ
  asserted and SYSCOACK deasserted, it must remain asserted until SYSCOACK is asserted* with its mirror sentence
  (*SYSCOREQ deasserted and SYSCOACK asserted*). Its guard candidates are `SYSCOREQ ASSERTED`, `SYSCOACK
  DEASSERTED` and `SYSCOACK ASSERTED`, the last being the *until* release, and it drives `AWAKEUP HIGH` although
  the sentence says *must remain asserted*, a stability obligation. Until `.22` the adapter kept the first candidate
  and emitted `(== SYSCOREQ 1)`; it now refuses the contradiction. Fix the fusion and the antecedent reading.
  Closes when `.25.a`-`.25.c` close. Measured (SemanticIR): `sigcon_0043`'s condition swallows its obligation
  (*…, it must remain asserted until SYSCOACK is asserted*) and its kind is `must_be_high`; its temporal rule's
  antecedents are `SYSCOREQ ASSERTED`, `SYSCOACK DEASSERTED`, `SYSCOACK ASSERTED`; `sigcon_0044`'s are `SYSCOREQ
  ASSERTED`, `SYSCOACK ASSERTED`, although its text says *SYSCOREQ deasserted*.

- ID: `EXTRACTION-REPRESENTATION-GAPS.25.a`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below; `.25.b` landed with it)
  Goal: **contracts with different stated conditions never fuse.** `merge_cluster` (`ir/fusion.rs`) compares kind,
  obligation and guard, and unions `guard_candidates` without comparing them; `fusion_key` omits the conditions.
  Put the sorted conditions in the key, so fusion merges only one obligation seen twice.
  **Landed**, with what splitting the fusion exposed: a temporal rule whose stated condition gave no antecedent is
  listed by the adapter instead of lowered unguarded (the IR keeps it: a phase condition has no signal predicate,
  as the WIRE-BASED-100 gold records); a clause naming several signals splits at each mention and each signal
  takes its own value (`.25.b`). AXI's AWAKEUP rules become holds, `(& (== AWAKEUP 1) (== SYSCOREQ 1) (== SYSCOACK
  0)) (AWAKEUP 1)` and its mirror; its three *during reset* rules and I2C's one are listed. Every emitted rule on the
  live chains keeps its stated condition as its guard (14) or states none (69).

- ID: `EXTRACTION-REPRESENTATION-GAPS.25.b`
  Status: `done` (`2026-09-25`; CODE; SpecForge; landed with `.25.a`, WIRE-BASED-100 6 of 6 identical)
  Goal: **an *until* clause is not a condition, and each signal takes its own value.**
  `parse_temporal_condition_predicates` (`ir/semantic.rs`) cuts *unless*/*except* but not *until*, and a clause
  naming several signals gives each the first value word (*with SYSCOREQ deasserted and SYSCOACK asserted* read
  SYSCOREQ as asserted).

- ID: `EXTRACTION-REPRESENTATION-GAPS.25.c`
  Status: `open` (`2026-09-25`; CODE; SpecForge; with `.11`)
  Goal: **an obligation does not leak into its condition.** *When AWAKEUP is asserted with …, it must remain
  asserted until SYSCOACK is asserted* is a persistence obligation (remain asserted, released by SYSCOACK). The
  constraint keeps it all as the condition, with kind `must_be_high`. Since `.25.a` the lowering is a faithful hold
  (the guard includes `(== AWAKEUP 1)`); the constraint's kind and condition text are still wrong in the IR.

- ID: `EXTRACTION-REPRESENTATION-GAPS.26`
  Status: `open` (`2026-09-25`; CODE; SpecForge defect; found by `.24.a`; before `.11`)
  Goal: **a temporal rule's condition is read whole, or the rule says it was not.** `parse_temporal_condition_predicates`
  (`ir/semantic.rs`) silently keeps part of a condition in 5 of the 16 temporal rules on the live chains whose stated
  condition yields a guard (`partial_conditions.py`). APB `sigcon_0015`, *PSEL, PENABLE, and PREADY are asserted, and PWRITE is
  deasserted*, keeps `PREADY` and `PWRITE`: the shared value is distributed only when the whole condition has one
  value, so the list members before *and PREADY are asserted* are dropped. AHB `sigcon_0007`, `0009`, `0010`, `0011`,
  *when HREADY is LOW, unless HRESP signal is ERROR*, keep `HREADY` alone: the exception is cut (`WIRE-BASED-100.5h`,
  right not to conjoin it as a positive condition, wrong to lose it). Each weakened guard applies the obligation in
  cycles the document exempts; `.11` would lower AHB's two stability obligations with it. None is emitted today.
  Fix: a value-less list member takes the value of the clause that ends its list; an exception becomes a negated
  predicate (FSMGen guards take `(! …)`, 13g) or the rule is marked as not read whole, so the adapter lists it.
  The AHB WIRE-BASED-100 gold labels the `HREADY` antecedent alone (*unless … is an EXCEPTION, not the
  antecedent*), so the gold's intent is read before its key is changed.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge defect; found by `.24.a`)
  Goal: **every port of an emitted actor has the direction the document grounds on that actor, or is listed.** The
  adapter lowers every interface signal into the initiator's actor; a signal the initiator neither drives nor reads
  takes `(output)` and a named drive, and no residual lists it (`initiator_perspective_directions`, `ir/isf_ir.rs`;
  `KG-ISF-COMPLETENESS.2a.ii` called the default an honest residual). It is not neutral: AHB `manager.isf` declares
  as Manager outputs `HSELx` (driven by the decoder, per IntentIR), `HRUSER`, `HBUSER` and `HREADYOUT` (the
  Subordinate). Measured on the two emitted files (`port_grounding.py`): AHB 11 of 39 ports (8 grounded only on
  other actors, 2 the Manager both drives and reads, 1 unrelated), AXI 172 of 294 (169 with no actor relation). Measure why (extraction recall versus a signal that belongs to another
  actor), then emit only grounded ports and list the rest.

- ID: `EXTRACTION-REPRESENTATION-GAPS.28`
  Status: `open` (`2026-09-25`; CHECK capacity; SpecForge; found by `.24.a`)
  Goal: **the ledger's owner citations have room to grow.** `doctrine/ownership_citations/citations.jsonl` holds one
  record per cited unit and allows 64; `.24.a` leaves it at 62, and the ledger holds most of them (ADR 0053 expects
  more rows). Raise the record bound under a recorded authority, keeping the byte bound able to fund it, as `.19`
  and `.20` did for their collections, and re-derive `claim-registry-capacity-is-coherent`, which reads this registry.

## `.2` — the IntentIR-to-`.isf` census (`2026-09-25`)

Re-derive: `python3 scripts/measure_intent_to_isf_reach.py` (read-only, over the 27 live chains).

**Emission.** All 27 live adapters are `blocked`: 27 have no clock/reset contract, 20 also have no interface
signals, 4 also no behaviour. The 7 with signals (AXI, APB 5, AHB, ADIv6, I2C, I2S, Wishbone) are blocked by the
contract alone (`.7`). Every number below is what the adapter *would* lower; none of it reaches FSMGen today.

**Families the adapter never reads**, and why that is or is not a gap:

| Family | Records (docs) | Verdict |
| --- | --- | --- |
| `constraints`, `behaviors`, `assumptions` | 5,229 (26), 2,091 (24), 157 (22) | untyped prose (`statement` + ids): unlowerable by construction; how many say what no typed record does is `.3` |
| `timing_constraints` | 237 (5) | physical and electrical parameters (ns, V, dB, Ω, mA, UI) from PHY/electrical tables: not an `.isf` gap by decision (below) |
| `signal_polarities` | 51 (4) | read nowhere, which makes the `asserted` lowering polarity-blind: `.8` |
| `signal_connectivity` | 235 (6) | topology; FSMGen composes actors, SpecForge emits one flat actor: `.14` |
| `actor_signal_relations`, `actor_drive_relations`, `actor_sample_relations` | 546, 273, 273 (6) | inputs to `actor_ports` directions, which the adapter reads: not a gap |
| `*_conflicts`, `fidelity_findings`, `captured_region_residuals`, proofs, identities | — | accounting and provenance, not design intent: not a gap |
| `regular_states`, `state_transitions`, `explicit_modules`, `explicit_tops`, `actor_trigger_relations` | 0 | empty on every live chain: an extraction gap where the document states them (`.12`, `.14`) |

**Residual packets** (150 in 28 families over the 27 adapters), attributed:

| Class | Packets | Owner | Why |
| --- | --- | --- | --- |
| protocol state / operation / edge timing | 51 | SpecForge `.12` | the records lack typed transitions, guards, initial state, encoding, actions |
| conflicting drives under an empty guard, rule/transaction overlap | 45 | SpecForge `.10` | the source's condition was lost; constraints lowered as drives |
| value `VALID`/`INVALID` | 11 | SpecForge `KG-ISF-COMPLETENESS.2a.vii.e`; FSMGen `.16` | validity is not a value; no known-value predicate |
| bare stability, 0-cycle exclusion | 10 | SpecForge `.11`; FSMGen `.15` for placement | `(stable …)` exists; the adapter's claim that it does not is stale |
| value words `ZERO`, `INCR`, `SHAREABLE` | 5 | SpecForge `.9` | FSMGen accepts enum members and numbers |
| register fields / resets not lowered | 5 (229 items) | SpecForge `.13`, FSMGen maybe | capture causes; symbolic resets to be measured |
| signal-width parameters | 9 | FSMGen `FSMGEN-REQUESTS.1`, `.4`-`.6`; SpecForge `KG-ISF-COMPLETENESS.2a.vii.c.iv`-`.c.vi` | the ledger's parameter rows |
| upstream canonicalization, grouping, role, unreadable width, guard disagreement, missing interface signal | 14 | SpecForge (their producing stages) | IR-side, before the adapter |

**Decision — physical and electrical parameters are not requested of FSMGen.** `timing_constraints` holds
tables of nanoseconds, volts, decibels, ohms and unit intervals, from documents the category contract classes
`physical-link` (`executable_target: honest_non_target_by_default`). ADR 0033 §6, kept by ADR 0053, keeps such
knowledge typed in IntentIR and forbids forcing it into executable constructs. Requesting an FSMGen carrier for
them would reverse that; it is the owner's call and is raised to them, not assumed.

## Current Frontier

1. `.28` — room in the owner-citation registry; `.26` — a condition read whole; `.27` — grounded port directions;
   `.24.b` — a per-target family as typed intent; `.25.c` — an obligation that leaks into its condition.
2. `.7.c`, `.7.d`, `.7.e` — the remaining clock/reset documents.
3. `.17`, `.11`, `.9`, `.10`, `.12`, `.13`, `.14`, `.18` — the SpecForge extraction and lowering gaps.
   (`.4`-`.6`, `.15`, `.16`, `.20` are requests and `.19` a question, forwarded by the owner `2026-09-25`; each
   closes when FSMGen answers and SPECFORGE integrates. `.21` lands with `FSMGEN-REFRESH-INTEGRATE-10` once FSMGen
   publishes its fix.)
4. `.3` — the PDF-to-IR census against the category contract.

## Decisions

- `2026-09-25` — `.24.a`: a generic name is tied to its family only by the document's own statement (APB's shared
  role phrase; AHB states none, so `HSEL` stays unread); a family lowers one instance at a time under the generic
  contract; FSMGen already states one select per target, so no request, and the fan-out is `.14`'s composition.
- `2026-09-25` — ADR 0053: extraction and FSMGen requests advance in parallel; the evidence bar is unchanged;
  one ledger in the book.
- `2026-09-25` — the owner, on reset semantics (`.7`): *an asynchronous reset, active high or active low (the
  default), asserts asynchronously and deasserts synchronously with respect to the domain clock.* So a document's
  *asserted asynchronously, deasserted synchronously* is FSMGen's `async`, and an asynchronous deassertion has no
  `.isf` form and is refused. Since FSMGen defaults the polarity, SPECFORGE always states it and never omits it.
- `2026-09-25` — the owner, on reset names: an active-low reset *usually* ends in `_n`, `_b` or a bare `n`
  (`ARESETn`), *just a convention* that helps a human read RTL. It stays a convention: no stage reads a polarity
  from a name (ADR 0037). FSMGen at the pin does not honour a stated polarity; it keys the polarity on the name
  (probe in `.21`).
- `2026-09-25` — the owner, on `PSELx`: *the `x` discriminates several selectable APB targets (target A has
  `PSELA`, target B `PSELB`, …); `PSEL` without it is the generic name, and any instance must behave according to
  `PSEL`'s contract.* Owned by `.24`.
- `2026-09-25` — the ledger is seeded only with rows re-verified today. The legacy-document reach was left
  out: chain currency reports 51 documents unmeasurable, while `CORPUS-COVERAGE`'s census counts refreshed
  documents differently; `.2` re-derives the reach before it is published.

## Open Questions

None.

## Blockers

None.

## Verification Log

- `2026-09-25` — `.0`: every seeded row re-derived in this session. FSMGen facts read at the pin from its
  published book (`subs/fsmgen/docs/book/src/13a-actor-interface.md` *Interface*;
  `13k-isf-feature-support-matrix.md` rows *Actor report metadata and params*, *Interface ports*, *Reusable ISF
  libraries*); SpecForge facts from the persisted IntentIR of the three live AMBA chains (36 `Param` symbols: AXI
  28, AHB 7, APB 5 1) and the `.2a.vii.c.iii` record.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.1` (CHECK) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — with the ledger declared as a region, `perl scripts/check_ownership_citations.pl --check` at `869a2b62` would read `KG-ISF-COMPLETENESS.2a.vii.c.iv` (declared one-line in `docs/tasks/kg-isf-completeness/isf-parameter-widths.md`) and `FSMGEN-REQUESTS.1` (declared one-line in its root) as nonexistent: `unit_status` matched only `- ID:` + newline + `Status:` in the root file.
- [x] **ROOT CAUSE (WHY + WHERE)** — `unit_status` (`scripts/check_ownership_citations.pl`) knew one of the two leaf grammars and never opened a partitioned tree's parts.
- [x] **ADDRESSED (verified)** — `resolve_leaf_status` + `tree_part_texts`; live check: 38 cited units across 3 regions resolve. End to end: pointing one ledger row at the closed `KG-ISF-COMPLETENESS.2a.vii.c.iii` is refused by name (`its own Status is 'done'`), then restored.
- [x] **NO REGRESSION** — self-test 16/16 (11 prior cases unchanged in intent); `ROADMAP.md` and the pressure-headroom region still resolve (28 units before the ledger region, 38 with it); RED: deleting the one-line grammar fails *a one-line leaf in a root did not resolve*, and resolving disagreeing declarations to the first fails *disagreeing declarations were resolved to one status*.
- [x] **GENERICITY** — the rule reads the task-tree grammars and the repository's `docs/tasks/<tree>/` layout; no tree is named.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md` `OWNERSHIP-CITATIONS` row; the ledger chapter's update rule states the check; this tree's frontier.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.8` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — persisted adapter text at `e2de9962`: AXI `(rule constraint_8 (AWLOCK 1))` from *AWLOCK must be deasserted* (`must_be_low`), `(AWSTASHLPIDEN 1)` from *… must be LOW*, APB `(PADDR 1)` from *must remain stable*, AXI `rule_70 (AWCMO 1)` from *must be 0b000*; 250 drive rules over 6 documents, 46 of 53 lowered conditional rules with action text `must` alone.
- [x] **ROOT CAUSE (WHY + WHERE)** — `IsfIr::from_intent_ir` (`ir/isf_ir.rs`): the signal-constraint loop ignored `constraint_kind` and did `target_value.unwrap_or("1")`, the invariant loop the same, the conditional-rule loop drove `1` unless the action contained `not`, and `isf_literal_value` mapped `asserted` to `1` regardless of polarity; upstream, `signal_constraint_to_temporal_invariant_kind` (`ir/intent.rs`) folded five level kinds into `MustBeAsserted` and `synthesize_temporal_invariants` copied neither the level nor `negated`.
- [x] **ADDRESSED (verified)** — rebuilt 6 chains: 250 → 95 rules, 44 AXI values corrected, 166 obligations listed in `isf_obligations_not_lowered`; spot check of emitted AXI constraints against their source sentences (`AWSTASHNIDEN … must be LOW` → `0`, `ARMMUATST Must be 0b0` → `0B0`, `SYSCOACK must be deasserted` → `0`). Tests `a_signal_constraint_states_its_level_with_negation_applied`, `a_level_resolves_only_through_the_signals_stated_polarity`, `an_action_states_a_level_only_when_it_names_one`, `an_obligation_drives_only_the_level_the_document_states`, `a_guard_on_an_unknown_polarity_refuses_the_rule_instead_of_vanishing`; three RED-verified (restoring the default `1`, resolving `asserted` without polarity, skipping an unknown-polarity guard). No test pinned the old values, which is how an inversion survived.
- [x] **NO REGRESSION** — `intent`/`adapt --dry-run` over the 27 live chains: 21 identical, the 6 changed only in `temporal_invariants.target_value` (IntentIR) and in their rules/residuals (adapter); FSMGen `--strict --check` 0 diagnostics on all 6 rebuilt texts; WIRE-BASED-100 11 of 11 seeds identical (HEAD and new binaries; EvidenceIR unchanged); `check_chain_currency.sh` 27 of 27 current at evidence, semantic, intent and isf-adapter; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,684); flow census +8 / +45 / +130 / 0, boundary counts unchanged.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — levels come from the constraint's typed kind, the action's own words and the signal's stated polarity; no signal name, suffix or protocol is consulted.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` *What value does a rule drive?* (the rule, the table, the dated correction) and step 10; ledger chapter (polarity row now `.17`, conflict count, *already flows* entry); KM card `isf-emission-blocked-by-clock-reset-contract`; new leaves `.17`, `.18`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.7.b` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `01ed8d80` 0 of 27 live chains carried a `system_contract`. `.7.a`: AXI and AHB state their reset's polarity and kind only in prose (`0549`, `0855`), APB-E its polarity only in a table cell (`0164`), and the table rule needed both in one cell. A prototype of the reader over all 78 EvidenceIR documents fired on exactly AXI, AHB and APB-E.
- [x] **ROOT CAUSE (WHY + WHERE)** — `parse_system_reset_descriptor` (`ir/semantic.rs`) required a kind in every `Reset` statement and `build_system_contract` merged whole declarations, so a kind and a polarity stated apart never met; nothing read either from prose (`synthesize_system_contract_from_table_descriptions`, `ir/evidence.rs`, reads one table cell).
- [x] **ADDRESSED (verified)** — `synthesize_system_reset_prose_statements`, a `Reset` grammar that takes the kind or the polarity alone, and a merge that refuses disagreement. AXI `ACLK`/`ARESETn` and AHB `HCLK`/`HRESETn`, asynchronous, active low, confidence high; both chains `renderable`; both `manager.isf` pass FSMGen `--strict --check` at `54ac1c22` with 0 diagnostics. Tests `a_reset_sentence_states_its_kind_and_polarity`, `a_reset_sentence_is_refused_when_it_is_ambiguous_or_unrepresentable`, `a_contract_merges_a_kind_and_a_polarity_stated_apart`, `a_contract_is_refused_without_a_kind_or_when_its_statements_disagree`. Found in verification and fixed here: the reset left the interface, so `guard_from_candidates` skipped *when ARESETn is asserted* and AXI SYSCOREQ/SYSCOACK drove `0` unconditionally. The adapter now tests the contract's reset at its stated level (`with_reset_polarity`, rule signals) and never drives it: `a_guard_on_the_contract_reset_survives_the_reset_leaving_the_interface`, `the_contract_states_the_reset_polarity_unless_a_record_disagrees`, both halves RED-verified.
- [x] **NO REGRESSION** — EvidenceIR replay over the 27 live chains: 24 identical; AXI, APB and AHB gain one appended statement each (earlier statements an exact prefix, source span carried). Rebuilt: APB's IntentIR and adapter identical; AXI and AHB gain the contract, 2 infrastructure signals, clock/reset input ports on every protocol actor (18/16, some on descriptor-noun actors owned by `KG-ISF-COMPLETENESS.1`), and AHB 2 direction conflicts (the table's driver-side `output` against the contract's `input`). Their clock and reset are no longer emitted as driven outputs. AXI residuals 49 → 47 (census 145/25 → 143/25). WIRE-BASED-100: 6 of 6 AMBA seeds identical between the HEAD and new binaries (constraints 6/6·6/6·3/3, relations 5/5·6/6·6/6, temporal 3/3·4/4·3/3). `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. `cargo fmt` and `cargo clippy -D warnings` clean; lib tests 473 / 168 / 1,690; flow census +8 / +50 / +30 / 0.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the reader keys on the words *reset signal*, *active low/high* and *asserted (a)synchronously*, and on the document's declared signals; no signal name, suffix or protocol is consulted (the renamed-signal case in the refusal test).
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` *Where do the clock and reset come from?*; ledger intro, *Clock and reset*, new rows *A reset's stated polarity* (`.21`) and *A rule's stated condition* (`.22`, `.23`), *already flows*; KM card `isf-emission-blocked-by-clock-reset-contract` (and its stale `.8` reverify line); leaves `.7.a`-`.7.e`, `.21`-`.23`; the owner's reset statements under Decisions.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.22` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `1746276a`, over the live chains: 16 emitted rules dropped a stated condition (AXI `rule_70` *AWSNOOP is not CMO*, 8 through invariants built without their constraint's condition) and 3 AXI rules were guarded `(== reset 1)` on a signal that does not exist, which FSMGen at the pin turns into an input port. Of 80 conditions on lowerable records most are prose; 24 of 81 constraint-derived invariants had lost their condition.
- [x] **ROOT CAUSE (WHY + WHERE)** — `sanitize_rule_condition` (`ir/isf_ir.rs`) returned no guard for a condition of several words and `(== <word lowercased> 1)` for one word; the invariant loop guarded on `condition_signal` alone at an invented `1`; `extract_condition_from_text` (`ir/intent.rs`) read only a condition still containing *when* and kept no value; `guard_from_candidates` kept the first usable candidate and skipped the rest.
- [x] **ADDRESSED (verified)** — `condition_guard` (whole `SIGNAL is LEVEL`, or *(in/during) reset* as the contract's reset), candidates joined by `(& …)` with undeclared, levelless or contradictory ones refusing the rule, `condition_value` carried and an unreadable condition yielding no invariant, refusal `ConditionNotLowered`. AXI 89 → 80 rules (12 guarded: nine `(== ARESETn 0)`, three `(== ACTIVATEACK 0)`), APB 5 → 3, I2C 3 → 1; AXI `manager.isf` strict-clean. Tests `a_condition_is_a_guard_on_a_declared_signal_or_it_is_refused`, `an_obligation_keeps_its_stated_condition_or_is_refused`, `a_constraint_condition_is_read_whole_or_the_invariant_is_not_made`, the updated `a_guard_on_an_unknown_polarity_refuses_the_rule_instead_of_vanishing`; three RED-verified (a condition dropped to no guard, first candidate only, contradiction accepted).
- [x] **NO REGRESSION** — IntentIR replay over 27 chains: 23 identical, AXI/APB/AHB/I2C change only in `temporal_invariants` (22 removed, all with an unread condition; 5 gain their condition); adapters: all 27 persisted equal the final binary (ADIv6 and Wishbone change only the listed-obligations wording). WIRE-BASED-100: 6 of 6 AMBA seeds identical (EvidenceIR unchanged). `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. `cargo fmt`, `cargo clippy -D warnings` clean; lib tests 473 / 168 / 1,692; flow census 0 / +5 / +4 / −2.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the grammar is the words *is*, the level vocabulary and *reset*; signals come from the declared set and levels from stated polarities; no name or protocol is consulted.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` *And under which condition?*; ledger row *A rule's stated condition*; KM card; leaves `.24`, `.25`; the owner's `PSELx` statement under Decisions.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.25.a` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `8f6c177c`, AXI's contract `fused:…sigcon_0043+…_0044` joined two mirror sentences with opposite conditions (candidates `SYSCOREQ ASSERTED`, `SYSCOACK DEASSERTED`, `SYSCOACK ASSERTED`); `sigcon_0044`'s antecedents read *SYSCOREQ deasserted* as `ASSERTED`; 4 emitted temporal rules dropped a stated condition (AXI `sigcon_0047`-`0049` *during reset*, I2C `dyn_sigcon_0004`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `fusion_key` (`ir/fusion.rs`) omitted the conditions and `merge_cluster` unioned them; `temporal_clause_value` (`ir/semantic.rs`) took the clause's first value word for whichever signal the clause yielded, and a clause yielded one signal; *until* was not cut; the adapter lowered a temporal rule with no antecedent as unconditional although its constraint stated a condition.
- [x] **ADDRESSED (verified)** — conditions in the fusion key; `split_clause_at_signal_mentions` plus `own_mention_tail` give each signal its own value; `until` cut with `unless`/`except`; `unread_stated_condition` makes the adapter list such a rule. AXI: the AWAKEUP rules become strict-clean holds, three *during reset* rules listed (the constraint loop keeps them guarded `(== ARESETn 0)`); I2C's one listed. Tests `contracts_with_different_stated_conditions_do_not_fuse`, `temporal_condition_reads_each_signals_own_value_and_stops_at_until`, `a_temporal_rule_whose_stated_condition_was_not_read_is_listed_not_emitted`, and `derives_actor_grounded_stability_event_from_stable_constraint` (now declares `HREADY` and asserts the antecedent); five RED-verified (key without conditions, no `until` cut, first value word, no per-mention split, unread condition lowered).
- [x] **NO REGRESSION** — first attempt, dropping such rules in SemanticIR, lost 2 of 4 AHB WIRE-BASED-100 temporal golds (*HAUSER must be valid during the address phase*: the gold keeps the rule with no antecedent), so the refusal moved to the adapter. Final: SemanticIR replay 26 of 27 identical, AXI changing only the AWAKEUP contracts and rules; AHB un-fuses one pair (its adapter text unchanged); every adapter equals the final binary; WIRE-BASED-100 6 of 6 identical; `check_chain_currency.sh` 27 of 27 current at every stage (2 emitted `.isf` files); lib tests 473 / 168 / 1,695; `cargo fmt`, `cargo clippy -D warnings` clean; flow census +6 / +26 / +89 / +2.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — mentions are the document's declared signals and co-references; values are the level words; no name or protocol is consulted.
- [x] **LOCKSTEP** — book *And under which condition?* (temporal path, dated correction of `.22`); ledger condition row; KM card; `.22`'s record corrected; `.25.b` closed with it; `.25.c` updated.

## Commit Log

- `.0` — `EXTRACTION-REPRESENTATION-GAPS.0 — one ledger of extraction and representation gaps, owned per side`.
- `.1` — `EXTRACTION-REPRESENTATION-GAPS.1 — the gap ledger's owners are checked, in either leaf grammar`.
- `.2` — `EXTRACTION-REPRESENTATION-GAPS.2 — census: every live adapter is blocked, and 150 residuals attributed`.
- `.8` — `EXTRACTION-REPRESENTATION-GAPS.8 — a rule drives only the level the document states`.
- `.7.b` — `EXTRACTION-REPRESENTATION-GAPS.7.b — a reset's kind and polarity come from the document's sentences`.
- `.22` — `EXTRACTION-REPRESENTATION-GAPS.22 — a rule keeps the condition it states, or is not emitted`.
- `.25.a` — `EXTRACTION-REPRESENTATION-GAPS.25.a — conditions fuse only when they agree, and each signal keeps its own`.
- `.24.a` — `EXTRACTION-REPRESENTATION-GAPS.24.a — the PSELx family: read through the document's own words, no FSMGen request`.

## Changelog

- `2026-09-25` — tree opened on the owner's directive; `.0` done (ADR 0053, ledger chapter); `.1`-`.6` open.
- `2026-09-25` — `.1` done: the ledger's owner column is an enforced region, and the checker reads one-line and
  partitioned-tree leaves.
- `2026-09-25` — `.2` done: all 27 live adapters blocked for want of a clock/reset contract; 150 residuals and
  ~20 unread families attributed; `.7`-`.16` opened (8 SpecForge, 2 FSMGen); `.3` re-anchored on the category
  contract; physical/electrical parameters not requested, by decision, and raised to the owner.
- `2026-09-25` — `.8` done: the adapter gave value-less obligations `1` and inverted some stated ones; rules 250 → 95
  on 6 chains, 44 AXI values corrected, 166 obligations listed; `.17` (polarity extraction) and `.18` (duplicate
  invariant ids) opened.
- `2026-09-25` — `.7.a` measured, `.7.b` done: AXI and AHB carry a clock/reset contract and emit strict-clean `.isf`;
  `.7.c`-`.7.e` (Wishbone, APB-E's unstated kind, reset-less documents), `.21` (FSMGen drops a stated reset polarity),
  `.22` (rules drop or invent conditions) and `.23` (guard addendum) opened.
- `2026-09-25` — `.21` and `.23`: both defects found fixed in FSMGen's development line (`f16130a4f`, `c6b8bc783`,
  unpublished) before a report
  went out; `.23` done, `.21` waits for `FSMGEN-REFRESH-INTEGRATE-10`, opened for it.
- `2026-09-25` — the owner forwarded `.4`, `.5`, `.6`, `.15`, `.16`, `.20` and question `.19` to FSMGen.
- `2026-09-25` — `.22` done: no emitted rule drops or invents a condition; `.24` (the `PSELx` family) and `.25` (a
  fused contract's mixed conditions) opened.
- `2026-09-25` — `.25.a`/`.25.b` done; `.22`'s claim corrected (4 temporal-path rules had still dropped a condition).
- `2026-09-25` — `.24.a` done (decision: no FSMGen request; `HSEL` unread); `.24.b`, `.26`, `.27`, `.28` opened.
