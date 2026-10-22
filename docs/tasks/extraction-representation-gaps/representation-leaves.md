# EXTRACTION-REPRESENTATION-GAPS — polarity, windows, families and fronted conditions

- Part ID: `representation-leaves`
- State: `legacy`

<!-- extraction-representation-gaps-task-source-region:representation-leaves:start -->
- ID: `EXTRACTION-REPRESENTATION-GAPS.17`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.8`; checklist below)
  Goal: **every signal's polarity the document states reaches the IR.** After `.8`, `asserted`/`deasserted` lower
  only through a stated polarity, and the 27 live chains carry 51 polarity records across 4 documents (5 of them
  active-low). Measure where the documents state polarity (an `active-LOW`/`active-HIGH` cell or phrase per
  signal; a document-wide statement such as *signals are active-HIGH unless stated*) and read it: a stated
  convention is document grammar, not spelling (ADR 0037 still forbids inferring it from a `…n` suffix).
  Found by `.7.c` (`2026-09-26`): Wishbone's *All WISHBONE interface signals MUST use active high logic*, grounded
  through its role sections, states every interface signal's polarity; only the reset reads it today, so
  `master.isf` still lists *MASTER interfaces MUST assert [CYC_O]* as a polarity unknown.
  **Measured `2026-09-26`** over the 27 live adapters: held back because *the document never states the signal's
  polarity* are 19 obligations (AXI 17, Wishbone 2), 16 temporal rules (AXI 14, Wishbone 1, AHB 1) and 1 AXI guard.
  Over all 78 persisted evidence chains, five documents state polarity for a scope of signals, not one:
  Wishbone's class rule above; AHB *The reset signal, HRESETn, is the only active LOW signal in the protocol*
  (an exclusive statement: every other signal of the protocol is active-HIGH); ADIv6 *All signals that are
  described in this section are active-high* (B2.3.2; a section rule); Avalon *In the following table, all
  signal roles are active high*; GIC-400 *All PPI signals are active-LOW level-sensitive* (GIC-600 states it
  only as a default that parameters invert). The last three are not among the 27 live chains. Wishbone's class membership cannot come from its minimum lists
  alone: *As a minimum, the MASTER interface MUST include … [CYC_O] …* is split across two statements, so the
  list reads only `ACK_I`; the role sections `.32` reads place every MASTER and SLAVE port. AXI states no scope
  rule: its only convention is *Lowercase n … denotes an active-LOW signal*, a spelling rule ADR 0037 forbids
  reading, so its 17 obligations, 14 temporal rules and guard stay held back (honest residuals). This slice reads the class rule (members from the
  minimum lists and the role sections), the exclusive statement (over the declared catalog) and the section
  rule (over the signals the section's statements name). Avalon's table rule and GIC's PPI class name members
  this reader does not ground, and are owned by `.17.a`.
  **Landed.** `scope_rule_polarity_observations` joins the prose polarity strategy: the class rule reaches 23
  Wishbone signals (members from the minimum lists and the MASTER and SLAVE role sections, not SYSCON's), the
  exclusive statement the other 40 AHB signals, the section rule ADIv6's `CSYSPWRUPACK` (B2.3.2's other three wires
  are rows the catalog does not declare, `SIGNAL-DECLARATION-ROW-DROP.1e`). Live records 51 → 115 in 5 documents,
  active-low 5 → 5, no conflict. The reset's class-rule statement now follows the resolved record, so a contested
  reset gets none. AHB's `HEXOKAY` and Wishbone's polarity refusals clear; the 17 AXI obligations, 14 temporal rules
  and 1 guard stay. **Found here and fixed**: Wishbone's `CYC_O` constraint was *[CYC_O] MUST be asserted no later
  than the rising [CLK_I] edge that qualifies the assertion of [STB_O]*, an order between events read as a level;
  with its polarity read it lowered as three always-HIGH `(CYC_O 1)` rules. A level bounded *no later/earlier
  than* an event now mints no constraint (`states_a_level_bounded_by_an_event`); `master.isf` is byte-identical to
  before, and reading the order is `.33`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.17.a`
  Status: `open` (`2026-09-26`; MEASUREMENT then CODE; SpecForge; found by `.17`)
  Goal: **a polarity stated over a table's rows or a named class of signals reaches each member.** Avalon's *In
  the following table, all signal roles are active high* scopes the rows of the table after it, and GIC-400's
  *All PPI signals are active-LOW level-sensitive* scopes a class the document lists in *Table 2-3*. Neither is
  among the 27 live chains, and their persisted artifacts predate the current readers (Avalon's holds a polarity
  record for a catalog entry `ROLES`, read from that very sentence), so first rebuild each from its source, then
  measure which declared signals each scope names and read the scope only where a table or list names its members.
  GIC-600's *PPI signals are active-LOW by default* is a configurable default, not a fixed polarity: it stays
  unread.

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
  Status: `done` (`2026-09-26`; DECISION `.24.a` and CODE `.24.b` done; SpecForge; found by `.22`)
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
  Status: `done` (`2026-09-26`; CODE; SpecForge; checklist below)
  Goal: **a per-target family is typed intent, and every reader resolves the generic name alike.** Record a
  per-target family from the document's own sentence (signal, target role, statements) through IntentIR; persist
  the co-reference once in SemanticIR so IntentIR's invariant reader resolves `PSEL` as the contract path does; and
  have the adapter list a family lowered as one instance on an actor that drives it. Generic: the grammar is *for
  each ROLE*, *each ROLE has its own … SIGNAL*, with ROLE a declared actor; no signal name is read.
  **Design `2026-09-26`.** SemanticIR persists `signal_coreferences` (name, declared signal, the shared role
  phrase, statements) and `signal_families` (signal, the role it has one instance per, statements), computed once;
  `build_temporal_rules` takes the co-references instead of recomputing them. IntentIR carries `signal_families`,
  and its invariant reader resolves a condition's signal through the co-references (APB `sigcon_0007`, *PSEL is
  asserted*, is the one live condition it changes). The adapter lists a family whose signal the lowered actor
  declares as an output. Both fields are skipped when empty, so only APB-E and AHB carry them.
  **Landed.** APB-E: `PSEL` → `PSELx` (role phrase *select signal*, statements 0203, 0211, 0243), family `PSELx`
  per `Completer` (0168), a new invariant `tinv_sc_sigcon_0007` (*PNSE must be valid when PSEL is asserted*), and
  `requester.isf` lists `isf_family_driven_once`; AHB: family `HSELx` per `Subordinate` (0277, 0658), no
  co-reference, no residual (its Manager does not drive `HSELx`). No `.isf` changes. The two SemanticIR fields and
  the IntentIR carry are registered rules (inventory 170 → 173; `rule_roots` 120 → 122), so every live chain's
  proof digest moved and the 27 were regenerated.

- ID: `EXTRACTION-REPRESENTATION-GAPS.25`
  Status: `done` (`2026-09-26`; CODE; SpecForge defect; found by `.22`; `.25.a`-`.25.c` done)
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
  Status: `done` (`2026-09-26`; CODE; SpecForge; with `.11`; checklist below)
  Goal: **an obligation does not leak into its condition.** *When AWAKEUP is asserted with …, it must remain
  asserted until SYSCOACK is asserted* is a persistence obligation (remain asserted, released by SYSCOACK). The
  constraint keeps it all as the condition, with kind `must_be_high`. Since `.25.a` the lowering is a faithful hold
  (the guard includes `(== AWAKEUP 1)`); the constraint's kind and condition text are still wrong in the IR.
  **Landed.** The condition half landed with `.10.c` (*AWAKEUP is asserted with SYSCOREQ asserted and SYSCOACK
  deasserted*). The kind: 9 constraints on the live chains read *must remain LEVEL until RELEASE* as a constant level
  (8 AXI, 1 APB), and in 5 the release was nowhere in the IR, so lowered they would never release. `type_holds`
  retypes them `MustRemainUntil { value, release }`; semantic reads a release that states a value into the rule's
  exceptions; IntentIR gives a hold no invariant; the adapter refuses it as a constant level, lists a hold whose
  release was not read, and drops an exception the guard already excludes, so AXI's four `AWAKEUP` holds and its
  `manager.isf` are unchanged. The APB WIRE-BASED-100 gold for *PWAKEUP must remain asserted until PREADY is
  asserted …* moves from `must_be_asserted` to `must_remain_until`, as the sentence states.

- ID: `EXTRACTION-REPRESENTATION-GAPS.26`
  Status: `done` (`2026-09-25`; CODE; SpecForge defect; found by `.24.a`; checklist below)
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
  **Landed.** `parse_temporal_condition` reads a condition whole or returns nothing: a list member takes the value
  of the clause that closes its list; an `unless`/`except` clause becomes `TemporalRuleRecord::exceptions` (still
  never an antecedent); a signal left without a value, or an exception naming none, empties the condition, and the
  adapter lists such a rule, now for conditional rules too (`unread_stated_condition`). An exception reaches the
  contract as `Condition::Ne` (so fusion keeps it apart) and the adapter's guard as `(! (== S L))`; an exception at
  the required level is a contradiction and one whose value is no level refuses the rule. The eval key and the LTL
  rendering carry exceptions; the two AHB temporal golds were labelled with theirs, as their notes already said.
  **Corrected `2026-09-25` (`.27.b`)**: this commit left test code that fails `cargo fmt --check` (an edit in
  `adapters.rs` after the last format) and `cargo clippy -D warnings` (a tuple return type in a `semantic.rs` test
  helper, added after the last clippy run). The pre-commit gate runs neither, so only push CI would have caught them;
  `.27.b` fixes both. The checklist's *`cargo fmt` and `cargo clippy -D warnings` clean* was true of the code before
  the last test edits, not of the commit.
  **Result**: all 16 guarded temporal rules keep their whole condition (APB `sigcon_0015` gains `PSELx`, `PENABLE`;
  AHB's four gain `HRESP ERROR`). Nothing emitted changes, because those obligations are refused for their own
  reasons (`VALID`, `.16`; stability, `.11`), and `ERROR` becomes lowerable only with `.9`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.26.b`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.25.c`; checklist below)
  Goal: **a condition's symbolic value is one the clause states.** `temporal_clause_value` (`ir/semantic.rs`) falls
  back to any capitalized word after the signal, so *the associated ACVALID / ACREADY handshake … of the DVM
  transaction* reads as `ACVALID` = `DVM`. `.25.c` found it when reading a hold's release, and reads a release only
  when it states a value (a copula). Measured `2026-09-26` on the live SemanticIR: of the 6 non-level predicates, the
  4 AHB `HRESP` = `ERROR` exceptions are stated (*HRESP signal is ERROR*) and none other remains, so no condition is
  misread today; the fallback stays able to invent one. Take a symbolic value only where the clause states it of the
  signal (a copula or an equality), and re-measure.
  **Landed.** `symbolic_value_is_stated` keeps a symbolic value only after a copula or an equality. SemanticIR
  replay: 27 of 27 identical, as measured; the reader can no longer invent a value.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27`
  Status: `done` (`2026-09-25`; MEASUREMENT then CODE; SpecForge defect; found by `.24.a`; `.27.a`-`.27.f` done)
  Goal: **every port of an emitted actor has the direction the document grounds on that actor, or is listed.** The
  adapter lowers every interface signal into the initiator's actor; a signal the initiator neither drives nor reads
  takes `(output)` and a named drive, and no residual lists it (`initiator_perspective_directions`, `ir/isf_ir.rs`;
  `KG-ISF-COMPLETENESS.2a.ii` called the default an honest residual). It is not neutral: AHB `manager.isf` declares
  as Manager outputs `HSELx` (driven by the decoder, per IntentIR), `HRUSER`, `HBUSER` and `HREADYOUT` (the
  Subordinate). Measured on the two emitted files (`port_grounding.py`): AHB 11 of 39 ports (8 grounded only on
  other actors, 2 the Manager both drives and reads, 1 unrelated), AXI 172 of 294 (169 with no actor relation).
  Measure why (extraction recall versus a signal that belongs to another actor), then emit only grounded ports and
  list the rest. Closes when `.27.a`-`.27.e` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.a`
  Status: `done` (`2026-09-25`; MEASUREMENT; SpecForge)
  Goal: why each of the 183 emitted ports lacks a direction grounded on the Manager, from the row that declares it
  (`.project-data/tmp/erg27/classify.py`, scratch: the declaring table's kind, header and `Source` cell, and the
  signal's actor relations in IntentIR).

  | Class | AXI | AHB | What the row states | Owner |
  |---|---:|---:|---|---|
  | a check signal whose check table has no `Source` column | 122 | 0 | the signals it covers (AXI Table A16.4) | `.27.d` |
  | a `Source: Manager` row on a `Continued from previous page` fragment | 42 | 0 | `Manager`, never read | `.27.b` |
  | a `Source: Tie-off` row | 5 | 0 | a tie-off, not an actor | `.27.c` |
  | a relation only on another actor | 2 | 8 | e.g. AHB `HSELx` (address decoder), `HRUSER`, `HBUSER`, `HREADYOUT` (Subordinate) | `.27.e` |
  | a Manager relation that is both input and output | 1 | 2 | AXI `ASKSTOP`, AHB `HTRANS`, `HTRANSCHK` | `.27.e` |
  | no `Source` cell and no relation | 0 | 1 | AHB `HRESET`, a Manager/Subordinate presence-matrix row | `.27.e` |

  The 42 are the clearest: AXI's B1.1 and B1.4 continue on a second page (`table_0251`, `table_0255`), whose header
  is the head's exactly and whose rows state `Source: Manager` (`AWSTASHLPID | 5 | Manager | Stash Logical Processor
  ID`). `continuation_inherited_table_heads` already gives such a fragment its head's kind for the declaration
  gap-fill and the table accounting (card `header-trapped-signal-table-recovery`), but the actor-relation reader does
  not consult it, so the rows declare their signals and relate them to no actor.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.b`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **a continuation page states its source like the page it continues.** The table-source actor-relation reader
  consumes `continuation_inherited_table_heads`, so a fragment with its head's header signature grounds its rows'
  `Source` cells (AXI 42). Measure the corpus-wide reach first; the emitted direction does not change for these
  (they already default to `(output)`), their grounding does.
  **Landed.** Reach (`continuation_reach.py`): only AXI on the live chains, 2 fragments with a relation column
  (`table_0251` 24 rows, `table_0255` 19); the other inheriting fragments are encodings or header-trapped presence
  matrices with no body rows. `extract_relations_from_signal_tables_with_prior_guidance` reads such a fragment in its
  head's context (kind gate, caption, section), citing the fragment. AXI gains 43 `Manager` drives and 43
  complementary `Subordinate` reads, as its head pages already had; the Manager's grounded ports rise 122 → 165 of 294.
  Downstream the rules and contracts on those signals gain their actor (`Manager` drives `AWIDUNQ`; 10 contracts,
  some now `lowerable`), and one real document conflict surfaces: `ARCHUNKEN` has two producers, the Manager (Table
  B1.4) and an interconnect (`1854`). The emitted `.isf` is byte-identical.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.c`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **a signal stated as an actor's input is read as that actor's input.** AXI B1.13's `BROADCAST*` rows state
  `Source: Tie-off`, and each signal's own table says whose input it is (A8.12 *BROADCASTSHAREABLE | 1 | 0b1 | Manager
  tie-off input, used to control the issuing of Shareable transactions*). **Corrected `2026-09-25`**: `.27.a` framed
  these as signals the Manager's interface should stop declaring; the document says the opposite, that they are
  Manager inputs tied at integration, which no actor drives. The emitted `.isf` got four of them right only through
  the flat `input` hint, and declared `BROADCASTCACHEMAINT` and `BROADCASTSHAREABLE` as driven Manager outputs (the
  second on a noise actor `Shareable`, read from *an optional Manager input signal, BROADCASTSHAREABLE*).
  **Landed.** `actor_relation_stated_by_description`: when a signal row's relation column names no actor, a
  description that opens by naming the signal an input or output of an actor the document already names
  (`<actor> [tie-off] input|output`) states that relation, before the section-heading fallback. Reach (AXI only,
  `tieoff_census.py`): 5 `Manager` reads; A14.1's *Manager output, asserted HIGH …* now also states `AWAKEUP` and
  `ACWAKEUP`, which B1.10's `Source` column already gave, so only their citation moves to the earlier page. The
  Manager's grounded ports rise 286 → 291 of 294; `manager.isf` makes `BROADCASTCACHEMAINT` and `BROADCASTSHAREABLE`
  inputs and drops their drives. `BROADCASTATOMIC`'s own table (A6.11) is classed `unknown`, so it keeps only its
  flat hint (`.27.e`). The tie-off's stated value (`Default 0b1`) is `.4`'s concern.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.d`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **a check table is read by its covered-signals column, whatever it calls its name column.** AXI Table A16.4
  lists, for each check signal, the signals it covers, and the document says check signals *are added to groups of
  signals on the interface and driven such that there is always an odd number of asserted bits in that group*
  (`4766`). **Corrected `2026-09-25`**: `.27.a` framed this as a decision still to make; it was made long ago.
  `augment_check_signal_relations_from_tables` (`ir/evidence.rs`, test
  `check_signal_tables_inherit_relations_from_covered_signals`) gives a check signal the relations of the signals it
  covers. It never fires on AXI because it finds the check column by a header containing *check signal*, and A16.4
  heads it `Name` (`Name | Signals covered | Width | Check enable`, on its uncaptioned head `table_0245` and four
  continuation pages, 122 rows). Recognise a check table by its covered-signals column, with the row's subject in the
  name column; the direction then comes from the stated coverage, never from the `CHK` spelling (ADR 0037).
  **Landed.** A table with a covered-signals column and no *check signal* header takes the row's signal from its first
  column. Reach: AXI only (123 rows; APB's and AHB's check tables already head the column *Check signal*). AXI gains
  254 inherited relations (Manager drives 82, Subordinate reads 81, Manager reads 42, Subordinate drives 42, and 7 on
  other actors the covered signals were already related to: `interconnect` 3, and descriptor noise such as `Note
  reads` and `exclusive reads`, owned by actor precision in `KG-ISF-COMPLETENESS.1`) and 123 direction declarations; the Manager's grounded ports rise 165 → 286 of 294. The
  emitted `manager.isf` changes as it should: 40 checks of Subordinate-driven signals (`WREADYCHK`, `RVALIDCHK`, …)
  become inputs and lose the drive the default gave them; FSMGen `--strict --check` accepts it with 0 diagnostics.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.f`
  Status: `done` (`2026-09-25`; CODE; SpecForge defect; found by `.27.e`'s re-measurement; checklist below)
  Goal: **an active relation verb's object stops where another agent's clause begins.** The in/out relations `.27.a`
  left were prose misreadings. AHB *The two-cycle response provides sufficient time for the Manager to cancel this next
  access and drive HTRANS[1:0] to IDLE* gave `two-cycle response drives HTRANS` (`provides`'s object ran into the
  Manager's infinitive) and `Manager reads HTRANS` (`access`, a noun after *this next*, read as a verb). AXI *The
  Manager controls the move from RUN to DEACTIVATE, but the Subordinate can use the ASKSTOP signal …* gave `Manager
  drives ASKSTOP`. `two-cycle response` is the Class-A noise `KG-ISF-COMPLETENESS.1` catalogued; no open leaf held
  these instances. **Landed**: the object ends at `but` and at `for <noun phrase of at most three words> to`; a verb
  form after a determiner and a closed-class sequence modifier (`next`, `same`, `subsequent`, …) is a noun. Over the
  27 live chains exactly 8 relations leave, all false (AXI `Manager drives ASKSTOP` and its inherited check; AHB the
  two above and their checks; ADIv6 `following drives In`, `previous reads In`), none is added. The Manager's `HTRANS`
  is a grounded output and AXI's `ASKSTOP`/`ASKSTOPCHK` become inputs.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.e`
  Status: `done` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; checklist below)
  Goal: **the adapter declares only ports grounded on the lowered actor, and lists the rest.** After `.27.b`-`.27.d`,
  re-measure what remains (AHB's 8 other-actor ports; the in/out relations, where AHB's Manager both drives and reads
  `HTRANS` although its own transaction drives it, so the relation is wrong upstream; the unrelated `HRESET`), then
  emit only grounded ports, list the rest, and keep every transaction and rule that drives a port consistent with it.
  **Re-measured `2026-09-25`** after `.27.b`-`.27.d` and `.27.f`: AXI 293 of 294 ports grounded (`BROADCASTATOMIC`
  keeps only its flat hint: its own table, A6.11, is classed `unknown`); AHB 30 of 39 (8 grounded only on the
  Subordinate or the address decoder: `HSELx`, `HRUSER`, `HBUSER`, `HREADYOUT` and their checks; `HRESET`, a
  presence-matrix row, unrelated). The in/out class is gone. Dropping an ungrounded port can lose one the Manager does
  have but the document never relates to it (AHB states only that the Subordinate is `HRUSER`'s source, not who reads
  it), so the listing must say which ports the document gives to another actor and which it relates to none; a rule
  that drives an input is `.10`'s question.
  **Landed.** With an initiator, the adapter declares a port only with a direction grounded on it; a signal no actor is
  related to keeps its declared direction, read across every interface record that states one
  (`declared_signal_directions`: the first record alone often states none); every other signal is left out and
  listed in `isf_ports_not_on_the_lowered_actor` by reason. AXI's `manager.isf` is unchanged (its one ungrounded port,
  `BROADCASTATOMIC`, keeps its declared input). AHB's loses 9 ports, their 9 drives and `USER_RESP_WIDTH`, which sized
  only `HBUSER`. The blocked chains' adapter text changes too: APB-E leaves out `PCLK`/`PRESETn` (related only to their
  source actors until `.7.d` gives APB a clock/reset contract), and ADIv6's `TCK`/`DBGTMS` and Wishbone's six `…_I`
  signals flip from a defaulted output to their declared input. Their listings expose the actor graph: I2C's
  initiator is the phrase `optional open-drain` and Wishbone's is `SLAVE`, both `KG-ISF-COMPLETENESS.1`'s actor
  precision.

<!-- extraction-representation-gaps-task-source-region:representation-leaves:end -->

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.19`
  Status: `done` (`2026-10-22`; QUESTION; supersedes the sealed record above; closing record in
  `encoding-and-state-leaves`)
  Goal: unchanged. FSMGen answered on `2026-09-26` that an unrecognised port clause is a validation defect, and
  upstream `c51e22712` refuses one; the ledger record now belongs to `FSMGEN-REFRESH-INTEGRATE-10.1`.
