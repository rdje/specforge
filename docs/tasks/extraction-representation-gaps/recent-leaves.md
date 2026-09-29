# EXTRACTION-REPRESENTATION-GAPS — recent leaves

- Part ID: `recent-leaves`
- State: `active`

<!-- extraction-representation-gaps-task-source-region:recent-leaves:start -->
- ID: `EXTRACTION-REPRESENTATION-GAPS.29`
  Status: `done` (`2026-09-26`; MEASUREMENT + DECISION + CODE; SpecForge; found by `.7.d`; checklist below)
  Goal: **a timing-diagram position is a window only against the event it counts from.** All 6 windowed temporal
  rules on the live chains are APB-E's, and each takes its window from a diagram time label: *The Setup phase of the
  write transfer occurs at T1 in Figure 3-1* gives `[1, 1]`, *… shown at T2 in Figure 3-1* gives `[2, 2]`, with no
  antecedent to count from, so lowered they would count from the start of the run. The reader does this on purpose
  (`parse_named_generic_edge_diagram_position`, kg-bench `unit_first_diagram_position_timing_gold`). Decide what a
  position counts from, and keep a window only where the document grounds that event.
  **Decision (`2026-09-26`).** The IR keeps a position as the document states it (the kg-bench fixtures pin `[3, 3]`
  for *at tick T3 of HCLK*); what a window may not do is count from an event no sentence names. FSMGen's book (13,
  *Intent scheduling*) arms `(monitor (within S N))` when its transaction's `(on start)` fires, so the adapter's
  unguarded monitor counted every such window from the start of the run. A windowed rule now lowers only with an
  antecedent, as `(=> g (within s [min] max))`; without one it is listed. **Landed.** No live artifact moves: APB-E's
  six windowed rules were already refused for other reasons (*must be valid*, *remain stable*, `PSEL` undeclared).

- ID: `EXTRACTION-REPRESENTATION-GAPS.30`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.7.d`; checklist below)
  Goal: **contracts fuse only when their obligations agree.** `.25.a` fuses contracts whose conditions agree, but APB-E
  `fused:contract_temporal_signal_constraint_sigcon_0002+…_0005` joins *PADDR … must be valid* (window 1) with
  *PADDR must remain stable* (window 2) and keeps only the first obligation; `0003`+`0006` likewise. Measure every
  fused contract on the live chains and fuse only equal obligations. Since `.31` those two no longer fuse (their
  conditions differ); the rule that fused them stands.
  **Measured, and the goal corrected (`2026-09-26`).** Fusing disagreeing obligations is by design: `merge_cluster`
  marks the fused contract `Residual { disagreement: … }` (R16-MULTIMODAL-CONTRACT-FUSION's *no source silently
  overwrites another*). The defect is downstream: `classify_actor_contract` lowers a `Drive` or windowed contract
  through its obligation arm, reading `lowering` only in its fallback, so a disagreeing fusion became its first
  member's drive. On the live chains 9 contracts are fused, 3 of them disagreements, all AXI's (`AWBURST` INCR and
  WRAP, `BCOMP` HIGH and LOW, `WTAG` ZERO and VALID), none emitted only because each value was refused for another
  reason. **Landed.** A fused contract marked residual is listed as a disagreement. Only a fusion: the converter
  also marks a handshake barrier residual, which R16-CONTRACT-IR.4 lowers as a stage on purpose.

- ID: `EXTRACTION-REPRESENTATION-GAPS.31`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.7.d`; checklist below)
  Goal: ***X is asserted, which means that Y must …* keeps X as Y's condition.** APB-E *The select signal, PSEL, is
  asserted, which means that PADDR, PWRITE, and PWDATA must be valid* yields three constraints with no condition,
  and a fourth that `PSEL` is asserted; the obligation holds when `PSEL` is. Read the *which means that* clause as
  the consequence of the one before it.
  **Landed.** The pattern path gives an inferred obligation its antecedent clause as condition (`inference_antecedent`),
  unless the antecedent names the subject itself, which restates the fact the antecedent producer records; the
  semantic reader closes an appositive member with the bare predicate after it (*The select signal, PSEL , is
  asserted* reads `PSELx = ASSERTED`). With the three rules guarded, `.30`'s APB fusions stop, and the two stability
  rules they had hidden (`0005`, `0006`, *must remain stable* with a two-cycle window) surfaced as `(monitor (within
  PADDR 2))`, *PADDR becomes true*, because a windowed consequent with no value took the level `1`. Such a consequent
  is now a `Stable` or `Observe` obligation, listed, in the conversion and the classifier alike. APB-E's
  `requester.isf` is unchanged.

- ID: `EXTRACTION-REPRESENTATION-GAPS.32`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.7.c`; checklist below)
  Goal: **a port is named from its own actor's side of the wire.** Wishbone names each interface's ports by direction
  (`ACK_O` on the SLAVE, `ACK_I` on the MASTER, joined by the interconnect), and its MASTER section says *The SLAVE
  asserts either the [ACK_I], [ERR_I] or [RTY_I] signals in response to every assertion of the [STB_O] signal*
  (statement `0401`). Read as *SLAVE drives ACK_I*, it puts the MASTER's inputs on the SLAVE: Wishbone's `slave.isf`
  declares `ACK_I`, `ERR_I`, `RTY_I` as outputs beside `ACK_O`, `ERR_O`, `RTY_O`, while the SLAVE's own `STB_I`,
  which *indicates that the SLAVE is selected*, is related to no actor and left out. Decide from the document how a
  wire's two port names relate (the interconnect *connects the [CLK_O] output to the [CLK_I] input*), and ground each
  port on the actor whose interface names it.
  **Measured and landed.** The document assigns every port to a role in its own sections: *2.2.3 MASTER Signals*
  (inputs `ACK_I`, `ERR_I`, `RTY_I`, `STALL_I`; outputs `ADR_O`, `CYC_O`, `LOCK_O`, `SEL_O`, `STB_O`, `WE_O`), *2.2.4
  SLAVE Signals* (the mirror), *2.2.2 Signals Common to MASTER and SLAVE Interfaces* (`CLK_I`, `RST_I`, `DAT_I`,
  `DAT_O`), *2.2.1 SYSCON Module Signals*. `extract_relations_from_role_signal_sections` reads an *input*/*output*
  naming a bracketed signal under such a heading; `supersede_relations_by_role_sections` drops a prose relation that
  puts a signal on a role whose section does not name it (the four other-side relations, including *MASTER reads
  STB_I*). With the SLAVE no longer holding the MASTER's names, no actor was a net producer: the contract's clock and
  reset, inputs of every actor, tied the MASTER at 7 and 7. `select_initiator_for` counts without them, and Wishbone
  emits `master.isf` with exactly the MASTER's 12 ports, strict-clean. The section's role is named as written, so
  *SYSCON Module* stands beside the prose's `SYSCON` (`KG-ISF-COMPLETENESS.1`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.33`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.17`; checklist below)
  Goal: **an obligation bounded in time keeps its bound.** Wishbone's statement `0438` states three: *MASTER
  interfaces MUST assert [CYC_O] for the duration of SINGLE READ / WRITE, BLOCK and RMW cycles* (a scope over
  operations, in the active voice no constraint reader takes), *[CYC_O] MUST be asserted no later than the rising
  [CLK_I] edge that qualifies the assertion of [STB_O]* and *… MUST be negated no earlier than the rising [CLK_I] edge
  that qualifies the negation of [STB_O]* (an order between events; since `.17` minted as nothing, not as a level).
  Its `RST_I` *MUST be asserted for at least one complete clock cycle* is still read as `must_be_high` with the
  minimum duration lost (an input and the reset, so nothing lowers it). Measure these shapes across the live chains
  (*no later than*, *no earlier than*, *for the duration of*, *for at least*) and type each with its bound, or list it.
  **Measured `2026-09-26`** over the 27 live chains: 20 normative statements carry one of the shapes (*no later
  than* 7, *no earlier than* 1, *for the duration of* 5, *for at least* 5, *at least N cycles* 3; USB 3.2 holds 8,
  Wishbone 6). Only one yields a signal record: Wishbone's `RST_I` `must_be_high`. The USB rows are system
  behaviour (*U0 entry shall be initiated no later than tDownLinkStateChange*) with no signal, and the rest mint
  nothing. **Found here**: *nothing* is not *counted* — Wishbone's `0438` is classed `signal_value_constraint`, and
  the completeness summary counts only uncited `normative_statement`s, so it is listed nowhere (`.34`). **The bound is not lost at SemanticIR**: the rule
  semantic derives from `RST_I`'s constraint carries `cycle_window {min_cycles: 1}`, and the adapter lists it as an
  unguarded window (`.29`). **It is lost at IntentIR**: `synthesize_temporal_invariants` turns the same
  constraint into `tinv_sc_sigcon_0002`, *RST_I is 1* at every cycle. The same holds for every constraint whose
  own rule is windowed, 5 on the live chains: `RST_I`; APB-E's `PADDR`, `PWDATA` and `PSEL`, placed at a figure's
  *T1*/*T2*; AHB's `HEXOKAY` *in the same cycle as HRESP is asserted*, whose exclusion `.11` owns. None reaches an
  emitted `.isf` (each is an input's or already listed), so the defect is five false IntentIR facts. The code:
  an invariant holds at every cycle, so a constraint whose own rule states a window is not one.
  **Landed.** `synthesize_temporal_invariants` skips a constraint whose rule, found by `signal_constraint_rule_id`
  (the one link SemanticIR mints), carries a cycle window; the rule keeps the bound. IntentIR loses exactly the 5
  invariants, and no semantic record and no emitted `.isf` moves. `CYC_O`'s three clauses stay unread: an order
  between events and a scope over Wishbone's bus cycles have no IR shape, and one statement on the live chains
  states them. Counting that statement is `.34`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.34`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.33`; checklist below)
  Goal: **an obligation no typed record cites is counted, whatever its class.** `validate`'s completeness summary
  counts the uncited statements of class `normative_statement` only (`uncaptured_normative_statement_ids`,
  `WIRE-BASED-100.3b`). On the 27 live chains, 96 of 165 `signal_value_constraint` statements and 34 of 1,007
  `conditional_rule` statements are cited by no signal constraint or conditional rule, and none is counted: Wishbone's
  `0438` (*[CYC_O] MUST be asserted no later than …*), which `.17` stopped reading as a level, is listed nowhere.
  Several refusals say their statement *stays counted as an uncaptured normative statement*
  (`ir/evidence.rs`, EXTRACTION-QUALITY-GAUGE.3k.2a and siblings; `pipeline/obligation-reading.md`); that holds only
  for a statement of that class. Measure which of the uncited statements another family cites (temporal rules,
  timing constraints, protocol records, relations), then count the rest as candidate misses, and correct the claims.
  **Measured `2026-09-26`** over the 27 live chains' EvidenceIR, counting a statement as cited when any other
  record family names it: of the statements no constraint or rule cites, 7 of 96 `signal_value_constraint` and 0 of
  34 `conditional_rule` are cited elsewhere (protocol states, relations), and 107 of 3,008 `normative_statement`
  (93 by protocol states, whose records carry no typed obligation, `.12`). The refusal comments never held: the
  passive constraint path reads only `signal_value_constraint` statements. `timing_constraint` (705 cited by none)
  and `derived_rule` (181) are topic classes, not obligation classes (*… typically has a data width of 32 bits*,
  *Figure 4-6 shows a configuration …*), so they are not counted. The finding's statement list was also not its
  count: `evidence_normative_residual_statement_ids` listed every normative statement, cited or not (3,026 against
  3,008). The code: an uncited `signal_value_constraint` or `conditional_rule` statement is a candidate miss, with
  its own metric and finding, and the normative finding lists exactly the statements it counts. Cited keeps its
  meaning (a constraint or a rule names it), so no count falls.
  **Landed.** `uncaptured_classified_obligation_statement_ids` (`ir/completeness.rs`) and one citation set in
  `validate` (`evidence_captured_statement_ids`) give `classified_obligation_residuals`, which joins
  `completeness_candidate_misses`, and `evidence_classified_obligation_residuals_remaining`, which lists them: 130 on
  the live chains (AXI 30, USB 3.2 22, ADIv6 17, Wishbone 15 with `0438`, …), 17 chains' totals rise by that much and
  none falls. The refusal comments in `ir/evidence.rs` and the book's *stays counted* sentences now name the count
  that holds them.

- ID: `EXTRACTION-REPRESENTATION-GAPS.35`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.10.g.iii`; checklist below)
  Goal: **a fronted condition keeps every member of the list it names.** AXI's *When AWSNOOP is not CMO, WritePtlCMO
  or WriteFullCMO, AWCMO must be 0b000* is `condrule_0152` with the antecedent *AWSNOOP is not CMO*: the fronted
  split (`split_conditional_sentence`) stops at the first comma, and the constraint path's
  `main_clause_of_fronted_conditional` recognizes a list only by an Oxford comma (*A, B, and C,*). Read, a guard would
  exempt only `CMO`. Measured over the 27 live chains: 3 conditional rules end their antecedent before a *, X or Y,*
  list (this one; USB 3.2 and USB4's *If there is a conflict between text, figures, and tables, …*), and no signal
  constraint does. End a fronted condition at the comma after the list's last member.
  **Landed.** `fronted_condition_end` serves both readers. It keeps the documented Oxford rule (a comma followed by an
  *and*/*or* segment is a list comma, and the condition ends at the comma after the list or else at its verb). It
  closes a list without the Oxford comma at the comma after its coordinating member, only where the main clause keeps
  a subject before the modal. The conditional-rule reader uses it only with a *must*/*shall* in the same sentence,
  else the first comma. A verbless all-list condition ends at the first comma. On the 27 live chains, 16 conditional
  rules change, every one a fix: AXI `condrule_0152` *AWSNOOP is not CMO, WritePtlCMO or WriteFullCMO*, ADIv6's
  *…, and the power controller receives a conflicting request …*, USB's *text, figures, and tables*. 8 AXI
  constraints regain *present* (*If present, AWSTASHNID, AWSTASHNIDEN, AWSTASHLPID, and AWSTASHLPIDEN must be
  LOW*), so `manager.isf` loses 18 rules that drove them LOW in every configuration; they are now listed under a
  presence condition (AXI's listed conditions 17 → 25, waiting for `.4`), strict-clean. **Found in replay and
  fixed before landing**: a first cut read lists over whole multi-sentence statements (USB 3.2's *…, it may
  receive …* swallowed its main clause), and broke the Oxford rule's `ACADDR, ACPROT, and ACSNOOP` subject; the
  three existing boundary tests caught the second.

- ID: `EXTRACTION-REPRESENTATION-GAPS.36`
  Status: `done` (`2026-09-26`; DOCUMENTATION + CHECK; SpecForge; the owner's request)
  Goal: **every missing piece from PDF to `.isf` is indexed in one place, with its owner.** The owner, `2026-09-26`:
  *catalog, categorize, list, describe, specify everything not addressed yet … explicitly indexed, task-tree owned,
  tracked … for FSMGen's items, an incremental list of all the things SPECFORGE needs FSMGen to support to go from
  PDF to ISF.* Gap knowledge is spread over this tree's leaves, the book's ledger, `docs/FSMGEN_FEEDBACK.md`, the 13
  packets under `docs/upstream/fsmgen/`, other trees' leaves and the category contract's carrier-less families.
  Build `docs/gaps/`: `INDEX.md` (every item, one row: side, road, stage, status, owner), `fsmgen-ladder.md` (the
  incremental list along the PDF-to-`.isf` path, each rung classified from FSMGen's published book at the pin as
  supported, fixed upstream and unpublished, requested, anticipated or a question) and `specforge.md` (SpecForge's
  items by road and stage). The index's owner column is an ownership region, so every cited unit must be classified
  and every current owner open (`OWNERSHIP-CITATIONS`); the collection is a registered live-document surface. The
  ladder is re-checked on every FSMGen pin refresh. An anticipated rung is not a request: `PROCESS.md`'s evidence
  bar and ADR 0033 still decide what is filed.
  **Landed.** [`docs/gaps/`](../gaps/INDEX.md): 25 SpecForge items (S1–S25) by road and stage, and FSMGen's ladder
  of 28 rungs classified from its book at `54ac1c22` (15 supported, 1 fixed upstream, 9 requested, 1 question, 2
  anticipated, F26's multi-bit crossings anticipated too); the index lists the 25 and the 14 rungs FSMGen does not
  yet provide, each with an open owner. The index table is the `gap_catalog_owners` ownership region (44 owners
  classified; the registry holds 104 of 128 records), and `docs/gaps/*.md` is the registered `gap_catalog` surface.
  Opened `.37` for the one need with no owner (temporal properties beyond *eventually*). **Found building it**: the
  handshake-to-stage lowering first listed as missing was delivered in May by `R16-CONTRACT-IR.4`, so it is not an
  item; the SpecForge list was drawn from the census, the category page and the owner leaves checked, not from every
  open leaf, so `.36.a` triages the rest.

- ID: `EXTRACTION-REPRESENTATION-GAPS.36.a`
  Status: `open` (`2026-09-26`; TRIAGE; SpecForge; found by `.36`)
  Goal: **every non-closed leaf is either a catalogued missing piece, process work, or closed.** On `2026-09-26`, 244
  leaves in 64 trees are not closed; many are process or repository work, and several trees are `active` with no
  open child. Classify each: a missing product piece gets a row in `docs/gaps/` under its owner; process work is
  named as such; a root whose children are all closed is closed or given its next leaf. Record the counts.

- ID: `EXTRACTION-REPRESENTATION-GAPS.37`
  Status: `open` (`2026-09-26`; MEASUREMENT then CODE, then a request if FSMGen lacks it; SpecForge + FSMGen; found by `.36`)
  Goal: **a temporal obligation beyond *eventually within N* is typed, and lowered or requested.** FSMGen's matrix
  (13k, *Explicit Non-Claims*) does not ship *runtime-signal or expression windows, global implication forms,
  dynamic bounds, expression operands, and multiple outstanding obligations*. The live chains state them: a window
  counted from an event (`.29` lists unguarded ones), a minimum duration (Wishbone *[RST_I] MUST be asserted for at
  least one complete clock cycle*, kept only as a SemanticIR `cycle_window`, `.33`), an order between two events
  (Wishbone *[CYC_O] MUST be asserted no later than the rising [CLK_I] edge that qualifies the assertion of [STB_O]*,
  counted, not read, `.34`). Measure how many live obligations take each form, type them in IntentIR, lower what
  `(assert …)` inside a transaction can state, and file a request for the rest under `PROCESS.md`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.38`
  Status: `open` (`2026-09-26`; DOC; found by `.3.c`'s commit)
  Goal: **partition this tree before its root warns.** The root file is 79.6% of the `task_evidence` 278,528-byte
  ceiling (warning 80%, rollover 90%) after 38 leaves in two days. Migrate it to a partitioned tree with
  `scripts/check_active_task_evidence.pl --migrate`, parts by reader concern (the census and roads, the FSMGen
  requests, the lowering leaves, the checklists), as the other large trees are; every leaf keeps its id and text.

- ID: `EXTRACTION-REPRESENTATION-GAPS.28`
  Status: `done` (`2026-09-25`; CHECK capacity; SpecForge; found by `.24.a`)
  Goal: **the ledger's owner citations have room to grow.** `doctrine/ownership_citations/citations.jsonl` holds one
  record per cited unit and allows 64; `.24.a` leaves it at 62, and the ledger holds most of them (ADR 0053 expects
  more rows). Raise the record bound under a recorded authority, keeping the byte bound able to fund it, as `.19`
  and `.20` did for their collections, and re-derive `claim-registry-capacity-is-coherent`, which reads this registry.
  **Landed.** `max_records` 64 → 128 under a registry authority in `ceiling_increase_authorities.jsonl` (retired by
  the next commit, as the protocol requires); `max_bytes` stays 32,768, which funds about 200 records at the real
  mean of 160 bytes, so the census reports no byte bound biting. The registry stays incoherent in the class sense
  (`max_records` × `max_record_bytes` exceeds `max_bytes`), which is `LIVE-DOCUMENT-PRESSURE-HEADROOM.36d`'s question,
  so the census's class shape (2 coherent, 8 not) is unchanged. **Found here and fixed**: that claim's result says
  *no registry is at or above a rollover milestone*, which was false while this registry sat at 93.8% and 96.9%,
  and nothing it ran tested the sentence. The census's summary line now carries `at rollover N` and the claim checks
  `… incoherent 8   at rollover 0`: restoring 64 prints `at rollover 1` and the claim's command fails (RED).

<!-- extraction-representation-gaps-task-source-region:recent-leaves:end -->

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.38`
  Status: `done` (`2026-09-26`; DOC; found by `.3.c`'s commit)
  Goal: **partition this tree before its root warns.** The root file was 79.8% of the `task_evidence` 278,528-byte
  ceiling (warning 80%, rollover 90%) after 38 leaves in two days. Migrate it to a partitioned tree with
  `scripts/check_active_task_evidence.pl --migrate`, parts by reader concern; every leaf keeps its id and text.
  **Done.** Pinned source `e7fd7402` (sha256 `36e0fe74…c71e769`, 222,294 bytes / 1,796 lines), migrated under
  `doctrine/live_document_size/extraction_representation_gaps_task_evidence.json`. Nine contiguous regions on the
  file's own section and node boundaries form eight parts: `program-and-census` (the tree's program, `.0`-`.3.f`,
  and the `.2` census with the decisions; active), `fsmgen-requests-and-reset` (`.4`-`.9`), `condition-lowering`
  (`.10`-`.16`), `representation-leaves` (`.17`-`.27`), `recent-leaves` (`.28`-`.38`; active), two halves of the
  acceptance checklists, and `changelog`. The root is 142 lines / 7,046 bytes (2.5% of the ceiling) with an owner
  registry of all 77 ids; the largest part is 42,254 bytes (64.5% of the 65,536-byte part target). The archived
  capsule is byte-identical to the pinned source. 77 routes: 50 `legacy` (every id the file's commit subjects
  name), 27 `structural`. Four surfaces registered (`erg_task_evidence_index`, `_parts`, `_route_parts`,
  `_archive`), the contract added to `scripts/check_task_evidence_contracts.sh` and its encoder declared.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.38 — the tree is partitioned before its root warns: 222,294 bytes to 7,046`

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.f`
  Status: `done` (`2026-09-26`; CODE; found by `SIGNAL-DECLARATION-ROW-DROP.6`; checklist below)
  Goal: **CoreSight TMC's clock sentence declares its clock.** TMC r0p1 writes *All configurations of the TMC use a
  single clock domain with input, CLK* and *The TMC provides a single reset input, RESETn, that is active LOW*. Since
  `SIGNAL-DECLARATION-ROW-DROP.6` both are declared inputs and the reset's polarity is read, but
  `build_system_contract` takes a clock only from a `Clock X.` statement and no reader produces one from this
  phrasing, so TMC's adapter still blocks on `no source-grounded system clock/reset contract`. Acceptance: read the
  clock a sentence names as the input of the block's single clock domain, structurally and for a declared signal
  only; TMC reaches a contract (`RESETn` active-low, kind not stated per `.7.d`); measure every current chain's
  contract delta; if TMC then emits, its `.isf` passes FSMGen `--strict --check`.
  **Done.** `synthesize_system_clock_prose_statements` (`ir/evidence.rs`) also reads the input a sentence gives a
  `clock domain`: the input must follow `clock domain` within the same clause, at most two words between, and be a
  declared signal; the reader's existing guard still adds nothing when the prose names two such clocks. Measured over
  the 78 chains' statements first: TMC's is the only sentence of that shape. `check_chain_currency.sh` with the new
  binary moves TMC alone (+1 statement, *Clock CLK.*; 30 chains current at every stage). Rebuilt with each stage
  validated, **TMC emits `table_a_3_atb_slave_interface.isf`**: `(clock CLK)`, `(reset (RESETn async active_low))`
  with the unstated kind listed (`.7.d`), 15 ports and 50 registers, and FSMGen `--strict --check --json` returns
  success with 0 diagnostics. Five chains now emit. Two fidelity limits stay visible and owned: the actor is named
  after the caption *Table A-3 ATB slave interface* and the ports face it (`KG-ISF-COMPLETENESS.1`, gap S3), and
  widths without a grounded value stay the honest default 1.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.7.f — TMC's clock sentence is read, and TMC emits a strict-clean .isf`

- ID: `EXTRACTION-REPRESENTATION-GAPS.39`
  Status: `done` (`2026-09-29`; CODE; SpecForge defect; found by `.3.c.ii`; checklist below)
  Goal: **a condition marker that opens a sentence opens that sentence's condition.** `split_conditional_sentence`
  reads a statement that does not itself open with a condition as *Y when Z*, split at its first ` when `. In a
  statement of several sentences that ` when ` can open a later sentence. ADIv6's *CSYSPWRUPREQ is the signal from the
  debug interface to the power controller. … When CSYSPWRUPREQ is asserted, the power controller … then asserts
  CSYSPWRUPACK …* gives the earlier sentences as the consequent and the whole *When* sentence as the antecedent. So
  the rule's consequent signal is the first known signal of the wrong sentence: today `CSYSPWRUPACK`, right only
  because it is the one known signal there, and `CSYSPWRUPREQ` once `.3.c.ii` makes that wire known. Measured on the
  persisted IR of the 31 current chains: 108 of 1,130 conditional rules, in 12 documents (USB 3.2 41, RISC-V IOMMU 11,
  RISC-V Debug 10, ADIv6 8, AXI 5, Wishbone 5, …). Acceptance: a marker that follows a sentence end is read as its
  sentence's leading condition; replay every chain and read every rule that moves, and what it lowers to.
  **Done.** In `split_conditional_sentence`'s embedded path, a marker whose preceding text ends in `.`, `:` or `;`
  opens a sentence: that sentence is split as a statement-initial one would be. A sentence the leading path cannot
  split is refused, as it is at the start of a statement. **Measured:** EvidenceIR dry-run of the 31 chains with the
  HEAD and new binaries: 10 identical, 21 move and only in `conditional_rules`, 105 rule sources. 93 keep their signal
  and action and their antecedent narrows to a strict prefix, the condition as written. 8 take their consequent
  signal from their own sentence or have none (AXI `BROADCASTATOMIC`, `AWMMUVALID` and Wishbone `RST_O` had borrowed
  one; ADIv6 `nSRST`, I2C `SCLH`, Wishbone `CLK_I`, AXI `RCHUNKCHK` gain one). 1 takes its action from its own sentence
  (RISC-V IOMMU *must be 0*). 6 are refused (TMC's two *When asserted:* list heads, Wishbone's two glossary
  sentences, USB 3.2's two comma-less conditions). The 21 chains rebuilt stage by stage: SemanticIR and IntentIR move
  only in the carried `conditional_rules`; no `.isf` text or lowering status moves; five adapters' listed
  obligations follow the rules (TMC's junk `CSYSACK`/`CSYSREQ` gone, AXI 120 → 118, ADIv6 and I2C one more listed,
  Wishbone renumbered). A consequent still runs to the end of its statement, so ADIv6's `nSRST` comes from a later
  sentence: `.39.a`.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.39 — a condition marker that opens a sentence opens that sentence's condition`

- ID: `EXTRACTION-REPRESENTATION-GAPS.39.a`
  Status: `open` (`2026-09-29`; CODE; SpecForge defect; found by `.39`)
  Goal: **a rule's consequent ends at its sentence.** `split_conditional_sentence` gives a fronted condition's
  consequent as the rest of the statement, and the consequent signal is the first known signal in it. ADIv6's *When
  nSRST is asserted, the entire system must be reset, including the debug domain. However, … Only the non-debug domain
  is held in reset while nSRST is asserted* takes `nSRST` from the third sentence. Measured on the 31 chains after
  `.39`: 254 of 1,124 rules have a consequent that runs past its sentence, and about 9 would change consequent signal
  if it were bounded (AHB `HTRANS` 3, AXI `BCOMP`, AHB `HRDATA`, I2C `SCL`, `SDA`, ADIv6 `nSRST`, Wishbone `STALL_I`).
  Acceptance: bound the consequent at its sentence end, replay every chain, read every rule that moves, and score the
  AHB gold.

- ID: `EXTRACTION-REPRESENTATION-GAPS.40`
  Status: `open` (`2026-09-29`; MEASUREMENT + DECISION; found by `.3.c.ii.a`)
  Goal: **`seed_swd`'s one gold constraint scores 0 where it once scored 1.000.** The gold labels ADIv6's
  *CSYSPWRUPACK must be asserted for as long as CSYSPWRUPREQ is asserted* `must_be_asserted`. SpecForge records
  `must_be_high`, because the section states *All signals that are described in this section are active-high, so assert
  denotes taking the signal HIGH* and polarity refinement applies it. So at `be58b45b` the seed scores `signal_constraint`
  tp=0 fp=1 fn=1, where `WIRE-BASED-100` and `LLM-PRIMARY-PROMOTION` recorded 1.000. `.3.c.ii.a` removes the prediction
  (fp 1 → 0) until `.3.c.ii` restores it. Acceptance: find the commit that moved it, and decide between refining
  the gold to the level the document states (the `.25.c` precedent) and scoring a refined kind as its unrefined form;
  record which, with the seed's score on each side.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.39` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — ADIv6 `statement_1242` (*CSYSPWRUPREQ is the signal from … . When CSYSPWRUPREQ is
  asserted, the power controller … asserts CSYSPWRUPACK …*) holds a rule whose antecedent is the whole *When* sentence
  and whose consequent is the sentences before it; on the persisted IR of the 31 current chains 108 of 1,130 rules are
  split at a marker that follows a sentence end.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `split_conditional_sentence` tries a fronted
  marker only at the start of the statement, and its embedded search (` when `, ` while `, ` during `, ` unless `)
  also matches a marker that opens a later sentence. RED, with the file restored byte-identically: dropping the new
  clause fails `a_condition_marker_after_a_sentence_end_opens_its_own_sentence`.
- [x] **ADDRESSED (verified)** — the 21 moved chains rebuilt stage by stage, each stage validated, the adapter last;
  ADIv6's rule reads antecedent *CSYSPWRUPREQ is asserted*, consequent signal `CSYSPWRUPACK`.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains with the HEAD and new binaries: 10 identical, 21 move
  only in `conditional_rules` (105 sources: 93 antecedents narrowed to a strict prefix, checked mechanically; 8
  consequent signals, 1 action and 6 refusals, each read). SemanticIR and IntentIR move only in the carried rules; no
  `.isf` text or lowering status moves; the five adapters whose listed obligations move were read.
  `check_chain_currency.sh` 31 of 31 current at every stage after the rebuilds. WIRE-BASED-100: all 11 seeds
  identical; `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D
  warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,778 passed, 0
  failed. `flow_census.json` rolled forward under this leaf (+4 decision sites, +1 helper edge).
- [x] **GENERICITY (ADR 0006)** — sentence punctuation only (`.`, `:`, `;` before the marker); no document, signal or
  phrase list.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` (*Corrected `2026-09-29`*); KM
  `a-condition-marker-after-a-sentence-end-is-fronted` (new); gap ledger S7 routes `.39.a`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.7.f` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — TMC's adapter after `SIGNAL-DECLARATION-ROW-DROP.6`: `blocking_reason` *no
  source-grounded system clock/reset contract*; its EvidenceIR carries *Reset RESETn is active low.* and no `Clock X.`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs` `build_system_contract` takes a clock only from
  `Clock X.`; `crates/specforge/src/ir/evidence.rs` `synthesize_system_clock_prose_statements` emitted it only for the
  window `clock input X`, and TMC writes *a single clock domain with input, CLK*.
- [x] **ADDRESSED (verified)** — TMC rebuilt: +1 statement (*Clock CLK.*); adapter `blocked` -> emits
  `table_a_3_atb_slave_interface.isf`; FSMGen `--strict --check --json` success, 0 diagnostics.
- [x] **NO REGRESSION** — `check_chain_currency.sh` (new binary): TMC alone moves; 30 chains current at every stage
  with the same 4 emitted `.isf`; TMC rebuilt, validated per stage, current with 1 emitted file. `kg-bench` 158/158.
  `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test
  --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,746 passed. `flow_census.json` +9 decision sites.
- [x] **GENERICITY (ADR 0006)** — a word grammar (`clock domain` … `input X`, X declared), the reader's uniqueness guard
  unchanged; no document, vendor or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (the clock/reset table gains TMC; the retained frontier says five
  emit), `architecture-rationale.md` and `reference/representation-gaps.md` say five; `docs/gaps/` S5 is 5 of 31.

- `2026-09-26` — `.38` done: the tree is partitioned (root 79.8% -> 2.5% of its ceiling; 8 parts, 77 routes).
- `2026-09-26` — `.7.f` opened by `SIGNAL-DECLARATION-ROW-DROP.6`: TMC's clock sentence is not read, so its adapter still blocks.
- `2026-09-26` — `.7.f` done: TMC's clock sentence is read; TMC emits a strict-clean `.isf` (five chains emit).
