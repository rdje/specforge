# EXTRACTION-REPRESENTATION-GAPS — conditions and their lowering

- Part ID: `condition-lowering`
- State: `legacy`

<!-- extraction-representation-gaps-task-source-region:condition-lowering:start -->
- ID: `EXTRACTION-REPRESENTATION-GAPS.10`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`; `.10.a`-`.10.f` and `.10.h` done, `.10.g` open)
  Goal: **a normative constraint keeps its condition, and is lowered as what it is.** 38 rules are dropped as
  conflicting drives under an empty guard (`AWSNOOP` 0 vs 1, `AWBURST` INCR vs WRAP) and 7 more overlap a
  transaction's drive: the source states each value under a condition that the rule lost. Read, not assumed:
  `AWSNOOP` 0 (`dyn_sigcon_0068`) comes from *"A Manager that only uses Opcodes where AWSNOOP is LOW can set
  AWSNOOP_WIDTH to 0 which omits the AWSNOOP output … An attached Subordinate must have its AWSNOOP input tied
  LOW"*: a configuration condition and a tie-off for an absent signal (`.4`), not a drive. Classify every dropped
  rule's lost condition (configuration, transaction, phase) before designing the fix. Recover the conditions; and decide, from the lowered actor's perspective, whether a constraint on a
  signal it drives is a drive, and one on a signal it reads an `assume`, rather than lowering every constraint as
  a drive. **Measured `2026-09-25` (`.27.f`)**: 18 of the 80 rules in AXI's `manager.isf` drive a port the Manager
  declares as an input (`ACTIVATEACK`, `ACWAKEUP`, `ASKSTOP`, `BCOMP`, `SYSCOACK`: another actor's obligation, e.g.
  *ASKSTOP must be LOW when ACTIVATEACK is LOW*, which the Subordinate meets); 14 did before `.27.f` corrected
  `ASKSTOP`'s direction. FSMGen `--strict --check` accepts all of them, so only reading the ports against the rules
  shows it. Closes when `.10.a`-`.10.h` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.a`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **an obligation on a signal the actor reads is not lowered as its drive.** Such an obligation is met by the
  actor that drives the signal; from the lowered actor's side it is an assumption, which `.isf` states only inside a
  transaction until FSMGen answers `.15`. **Landed**: `lower_obligations` and the temporal-contract rule path refuse an
  obligation whose target the actor declares as an input (`DrivesAnInput`), listed with the other obligations not
  lowered. AXI's `manager.isf` drops exactly its 18 such rules (80 → 62), none of which the Manager meets; AHB's has no
  rules; APB-E, AHB and Wishbone only re-classify obligations already listed. FSMGen `--strict --check` 0 diagnostics.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.b`
  Status: `done` (`2026-09-25`; MEASUREMENT; SpecForge)
  Goal: **the rest of `.10`: conflicting drives and overlaps, and the conditions they lost.** Re-measure `.2`'s 38
  conflicting drives under an empty guard and 7 rule/transaction overlaps after `.8`, `.22`, `.25`, `.26` and `.10.a`;
  classify each remaining one's lost condition (configuration, transaction, phase), recover the conditions the
  document states, and list the rest.
  **Measured** (the persisted adapters at `c5292610`): only AXI has any, 18 rule conflicts and 3 rule/transaction
  overlaps (from 45). Every one pairs obligations the sentence conditions, read with no condition:

  | Class | Constraints | Example | Owner |
  |---|---:|---|---|
  | a transaction or presence condition not captured | 8 | *If present, AWIDUNQ must be asserted for Prefetch transactions* (6); per-transaction table rows *ARSNOOP \| Must be 0b1110* and *ARLOCK \| Must be 0b0* (2) | `.10.c` |
  | the condition's signal taken as the subject | 2 | *If AWIDUNQ is deasserted for an Atomic request, the corresponding RIDUNQ signal must be deasserted* — recorded on `AWIDUNQ` | `.10.d` |
  | a configuration tie-off read as a drive | 3 | *AWLOCK and ARLOCK are tied LOW* (a compatibility row); *an attached Subordinate must have its AWSNOOP input tied LOW* | `.10.e` (with `.4`) |

  Separately, `ARLOCK` `0B0` against `0` is counted as a conflict although both are the level LOW (`.10.f`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.c`
  Status: `done` (`2026-09-26`; CODE; SpecForge; checklist below)
  Goal: **a constraint keeps a transaction or presence condition its sentence states.** *… must be asserted for Prefetch
  transactions* and *If present, …* carry `condition_text: None`, so the obligation lowers unconditionally and collides
  with the ones for other transactions. Capture the stated condition (a transaction type, a presence clause, the
  transaction a table row belongs to) as the constraint's condition, so lowering lists it (`ConditionNotLowered`)
  instead of driving it always; reading it as a guard (the opcode a transaction type names) is later work (`.10.g`).
  **Landed.** `extract_condition_clause` reads a condition the sentence fronts whole (an inner *when* no longer takes
  the obligation with it), and `stated_condition_clauses` adds a fronted *For <operation>,*, an infix *if*, *for as
  long as* and a trailing *for* phrase that names an operation, never one that counts (*for one response transfer*);
  an infix condition ends at the comma before a later obligation. A two-cell keyed row takes the operation its
  table's caption names (*Table A15.23: DVM Complete request constraints*), whichever producer reads it. AXI's 16 rule
  conflicts fall to 0, and 30 drives true only of a DVM Complete request or a stated transaction leave its
  `manager.isf`; the 3 `AWSNOOP` overlaps are `.10.e`'s.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.d`
  Status: `done` (`2026-09-26`; CODE; SpecForge; `CONSTRAINT-SUBJECT-PRECISION` is closed; landed with `.10.c`)
  Goal: **in *If X is V …, the corresponding Y must be W*, the subject is Y and the condition X is V.** AXI's two
  `RIDUNQ` sentences are recorded as obligations on `AWIDUNQ` with no condition.
  **Landed.** The sentences are list items (*- -If AWIDUNQ is deasserted …*), and the markers hid the fronted *If*
  from `main_clause_of_fronted_conditional` and `fronted_condition_subject`, so the subject search fell back to the
  first signal. `without_list_markers` strips them; the two `AWIDUNQ` records leave, and the `RIDUNQ` ones keep the
  condition.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.e`
  Status: `done` (`2026-09-26`; CODE; SpecForge; with `.4` and `KG-ISF-COMPLETENESS.2a.vii.c.vi`; checklist below)
  Goal: **a stated tie-off is not a drive.** *AWLOCK and ARLOCK are tied LOW* (a row of a property-compatibility table)
  and *an attached Subordinate must have its AWSNOOP input tied LOW* state what an input is tied to under a
  configuration, not what an actor drives. Record them as tie-offs with their configuration, and never as unconditional
  drive obligations. **Measured `2026-09-26` (`.10.c`)**: with the DVM Complete rows conditioned, AXI's `manager.isf`
  now emits the tie-off readings `ARLOCK` 0 (*AWLOCK and ARLOCK are tied LOW*) and `ARSNOOP` 0 (the `ARSNOOP_WIDTH`
  omission) as unconditional drives, 2 rules each, and `AWSNOOP` 0 still overlaps `persistent_cmo_pcmo_transaction`
  (3 residuals).
  **Landed.** A level bound by *tied* in a clause that states no condition is retyped `TiedTo`, whichever producer
  read it (`type_tie_offs`); a *tied* level with a stated scope (*tied HIGH for the entire burst*) stays a hold. No
  stage lowers it: semantic gives it no rule, IntentIR no invariant, and the adapter lists it (*is a tie-off*). Its
  configuration is the signal's own absence on the driver, which the signal's presence condition states
  (`AWSNOOP_WIDTH` 0, `Exclusive_Accesses` False) and `KG-ISF-COMPLETENESS.2a.vii.c.vi` reads; `.isf` states the tied
  value once FSMGen ships `.4`'s absent-input value. AXI's 5 tie-offs (`ARLOCK`, `AWLOCK`, `ARSNOOP`, `AWSNOOP`,
  `WSTRB`) leave `manager.isf` (9 rules) and its 3 rule/transaction overlaps fall to 0. `AWLOCK` 0 is still driven (3
  rules, one constraint), from *AWLOCK must be deasserted, not exclusive access*, a list item whose condition is its
  lead-in's (`.10.h`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.f`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **the same level in two spellings is one drive.** The rule-conflict check compares drive values as text, so
  `ARLOCK` `0B0` and `0` are reported as conflicting; compare the resolved level.
  **Landed.** A first attempt emitted both spellings and FSMGen strict refused the pair
  (`isf_conflicting_rule_writes`: it compares writes as written). So `drive_agreement` separates the same text (both
  emitted, as before), the same number in another spelling (emitted once, the restatement listed as
  `isf_rule_same_level_<name>`), and a different value (a conflict). Both the same-guard and the unconditional-overlap
  passes use it. AXI's `manager.isf` is byte-identical and strict-clean; its two `ARLOCK` conflicts become same-level
  entries (16 real conflicts remain, owned by `.10.c`-`.10.e`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g`
  Status: `open` (`2026-09-26`; MEASUREMENT + DECISION done, CODE in `.10.g.ii`/`.10.g.iii`; SpecForge; found by `.10.c`)
  Goal: **a transaction or presence condition is read as the guard or scope it names.** `.10.c` keeps the condition, so
  the obligation is listed instead of driven always; it is still not lowered. AXI's `manager.isf` lists 20 obligations
  whose condition is not `SIGNAL is LEVEL` (3 before `.10.c`): *DVM Complete request* (a table's rows), *For
  non-stash transactions*, *present on the interface, and for Prefetch transactions*. A transaction condition belongs
  inside the transaction the document names (the opcode that identifies it), a presence condition to the configuration
  that includes the signal (with `.4`). Measure which conditions name a transaction the IntentIR already carries, then
  lower those and list the rest.
  **Measured `2026-09-26`** over the 27 live adapters: 19 obligations are listed for a condition that is not
  `SIGNAL is LEVEL`, AXI 17 and APB-E 2. None can be lowered inside a transaction: AXI's IntentIR carries 14 named
  transactions (from section headings), 4 of which the conditions name (Atomic, Prefetch, WriteZero,
  WriteDeferrable), but none carries the value that identifies it or a body, and `manager.isf` emits none. What
  identifies the transaction is the document's own statement of it, of two kinds:
  - **a signal's level states the transfer kind**: APB's *PWRITE indicates an APB write access when HIGH and an APB
    read access when LOW* (`0170`) makes *For read transfers* and *a read transfer* (`PSTRB` must be LOW,
    `constraint_15`, `_17`) the guard `PWRITE` LOW. Owned by `.10.g.ii`.
  - **an opcode table states it**: AXI's `AWSNOOP` table row `| 0b00111 | NSH, SH, SYS | WriteZero |
    WriteZero_Transaction | …` (`2227`) identifies WriteZero by `AWSNOOP` *and* `AWDOMAIN`, so one signal's value is
    not the key; `InvalidateHint`, `UnstashTranslation`, `Prefetch`, *non-stash*, Atomic (`AWATOP`) and *DVM
    Complete* (`ARSNOOP`) are the same kind. Owned by `.10.g.iii`. 7 of AXI's 17 also carry a presence clause (*If
    present*), which waits for `.4`; and *DVM Complete request*'s 7 rows state the request's own attributes, an
    implication between outputs the Manager chooses together (`.11` lowers it as an assertion, not a drive).
  **Decision.** A condition is lowered only through the document's statement of what identifies the transfer; the
  transaction's name alone lowers nothing. `.10.g` closes when `.10.g.ii` and `.10.g.iii` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g.ii`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.10.g`; checklist below)
  Goal: **a transfer kind a signal's level states is a guard.** Read *<SIGNAL> indicates a <kind> <operation> when
  <LEVEL> [and a <kind> <operation> when <LEVEL>]* (APB `PWRITE`, `0170`) as a typed record (kind → signal, level,
  statement), and read a condition that names that kind of operation (*For read transfers*, *a read transfer*) as
  the guard `SIGNAL is LEVEL`, alike in every reader (temporal rule, invariant, constraint lowering). APB-E's two
  `PSTRB` obligations then lower in `requester.isf`.
  **Landed.** SemanticIR's `operation_kinds` (`read` → `PWRITE` LOW, `write` → `PWRITE` HIGH, from `0170`), carried
  by IntentIR; `operation_kind_condition` rewrites a condition naming only that kind of operation before the temporal,
  invariant and constraint readers parse it. APB-E's `requester.isf` gains 5 rules `(== PWRITE 0)` → `(PSTRB 0)`
  (two constraints, two invariants, one temporal rule), strict-clean, and its 2 listed conditions go; the sentence
  states no transfer scope, so none is added. AHB's *When HIGH this signal indicates a write transferandwhenLOWa read
  transfer* (`0265`, fronted, *this signal*, glued text layer) is not read; no AHB obligation names a read or write
  transfer. Registered: inventory 173 → 175 rules, `rule_roots` 122 → 123.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g.iii`
  Status: `blocked` (`2026-09-26`; MEASUREMENT done; CODE waits for `.4`; SpecForge; found by `.10.g`)
  Goal: **a transaction an opcode table identifies is a guard.** Type the rows of a table that names each transaction
  by the values of declared signals (AXI's `AWSNOOP`/`AWDOMAIN` table `2227`, `ARSNOOP`, `AWATOP`), then read a
  condition naming one of those transactions as the conjunction the row states. Measure first which tables key a
  transaction by one signal and which by several; a presence clause still waits for `.4`.
  **Measured `2026-09-26`**: of AXI's 17 listed conditions, none lowers through opcode identification alone. The 6
  on `AWIDUNQ` (*If present, … for a WriteZero / WriteDeferrable / Prefetch transaction, an InvalidateHint /
  UnstashTranslation request, a write with tag operations …*) also need presence, which waits for `.4`; the 7 rows
  of *DVM Complete request constraints* state the request's own attributes, implications between outputs the Manager
  chooses together (`.11`, as assertions); *For Atomic transactions* (`AWLOCK`) and *For non-stash transactions*
  (`AWSTASHNIDEN`, `AWSTASHLPIDEN`) name a class the document gives only as its complement (*AWATOP is 0b000000
  (Non-atomic operation)*), which reading would take from a word's shape; and *When AWSNOOP is not CMO, WritePtlCMO or
  WriteFullCMO* (`rule_70`, `AWCMO`) is read only as far as its first comma (`.35`). The opcode table is typed
  when an obligation can use it: after `.4`, the 6 `AWIDUNQ` conditions.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.h`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.10.e`; checklist below)
  Goal: **a list item keeps the condition its lead-in states.** AXI's *For Atomic transactions, the following
  restrictions apply for request attributes:* introduces *- AWLOCK must be deasserted, not exclusive access.*; read
  alone, the item is an unconditional drive of `AWLOCK` 0 (3 rules in `manager.isf`). Measured `2026-09-26` over the
  live chains: 3 constraints whose list lead-in opens with *For*/*When*/*If* and ends in a colon (AXI `AWLOCK` and
  `WTAGUPDATE`, AHB `HSIZE` under *For all Device memory, … the required behavior is:*). Carry the lead-in's
  condition to its items, as `.10.c` carries a table caption's operation to its rows.
  **Landed.** `condition_list_items_on_their_lead_in` gives every list-marked statement after an introducing one the
  condition that introducing sentence fronts, joined to the item's own. Only the sentence ending in the colon counts
  (*It has the following rules:* conditions nothing, so `WTAGUPDATE`'s items, one of which states the opposite scope,
  stay as written), and only a *For* phrase naming an operation or quantifying over *all*/*each*/*every*, or a fronted
  *If*/*When* clause (*For example, …:* is none). Read, not assumed: AXI's two other lists, *When using a Valid-Ready
  transport, the following rules apply:* and *When transporting tags, …:*, condition 4 items that already had a
  condition. AXI's `manager.isf` loses its 3 `AWLOCK` 0 rules; AHB's `HSIZE` keeps *For all Device memory*.

- ID: `EXTRACTION-REPRESENTATION-GAPS.11`
  Status: `open` (`2026-09-25`; CODE; SpecForge; found by `.2`)
  Goal: **stability and exclusion obligations lower as assertions.** 7 rules are residualized as *bare stability
  across tick phases has no supported .isf construct* (`HSIZE must not be changed`) and 3 same-cycle exclusions as a
  0-cycle window (`HEXOKAY must not be asserted in the same cycle as HRESP`). The first claim is stale: FSMGen's
  matrix (13k *Immediate verification checks*) has `(assert …)` with `(stable SIG)`, `(=> A B)` and plain boolean
  exclusion. Lower them as assertions; an actor with no transaction to host one needs `.15`. First make
  `classify_invariant_text` (`ir/intent.rs`) stop labelling unmatched text `MustRemainStable` ("default for chip
  invariants"): lowered as an assertion, that default would state a stability the document never did.

- ID: `EXTRACTION-REPRESENTATION-GAPS.11.a`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.11`; checklist below)
  Goal: **an invariant whose text states no kind says so.** Measured `2026-09-26` over the 27 live chains' IntentIR:
  of 5,162 invariants lifted from SemanticIR's statement invariants, 5,089 match none of `classify_invariant_text`'s
  phrases and are labelled `must_remain_stable` by its default (*The PE cannot service interrupts in Debug state.*,
  *Each access port can drive a different memory space.*); 73 match a phrase. None has a subject signal, so none
  reaches an `.isf`, but each is a stability claim the document never made, and `.11` would lower it as one. Add a
  `NotStated` kind and make it the default.
  **Landed.** `TemporalInvariantKind::NotStated` (`not_stated`) is `classify_invariant_text`'s default; exactly the
  5,089 flip, the 73 that name a kind keep it, and no semantic record, adapter or `.isf` moves.

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

<!-- extraction-representation-gaps-task-source-region:condition-lowering:end -->
