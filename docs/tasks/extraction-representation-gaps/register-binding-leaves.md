# EXTRACTION-REPRESENTATION-GAPS — register binding leaves

- Part ID: `register-binding-leaves`
- State: `active`

New leaves outside the `.3` family continue here, from `.41.g` on, after `clause-and-register-leaves` reached its
mandatory rollover (`2026-09-30`): the register-field-to-signal binding readers and their residuals.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.41.g`
  Status: `done` (`2026-09-30`; CODE; found by `.41.e`; checklist below; opened `.41.h`)
  Goal: **the AHB appendix's HPROT bit map binds `CSW.Prot`.** ADIv6 *E1.4 AMBA AHB3* states *The CSW.Prot field
  drives the AHB HPROT signals*, then maps bits 27-24 to `HPROT[3:0]` in a table whose caption the converter lost
  (`table_0205`: `Bit | HPROT signal | Description | Description when not implemented …`), and binds bit 28 in prose
  under a sub-field heading (*HPROT[4], Allocate, bit[28]*: *Drives HPROT[4]*). **Reproduce:** `CSW@MEM-AP.Prot`
  has no `HPROT` binding. **Impact:** an AHB MEM-AP's protection bits reach no wire. Acceptance: decide whether the
  sentence naming the field (*The CSW.Prot field drives the AHB HPROT signals*) locates the field for the captionless
  table under it and for the sub-field headings of that section, and measure every chain.
  **Measured (`2026-09-30`), and it overturns a premise of `.41.e`.** The appendix states HPROT bindings in two
  prose shapes: AHB3 heads *HPROT[4], Allocate, bit[28]* and *HPROT[3:0], bits[27:24]*, each followed by *Drives
  HPROT[…]*; AHB5 lists *Bit[27] drives HPROT[6], HPROT[4], and HPROT[3].* So **HPROT[4] is driven by bit 28 in AHB3
  and by bit 27 in AHB5**: one signal, two bit maps, one merged `CSW@MEM-AP` record. `.41.e` reasoned that a bus
  variant is implied by its signals and merged twin tables without a scope; that holds for the AXI twins, whose
  content is identical, and fails here. SourceIR's heading levels are all 1 in this appendix, but the numbered
  titles carry the nesting (`E1.4.1 …` under `E1.4 AMBA AHB3`). APB4 writes *Drives PPROT[2:0].* far from its field
  heading once converted, so no adjacency grammar reads it.
  **Design:** a binding records the numbered heading path it is stated under (`scopes`), for tables (`.41.e`) and
  prose alike; *Bit[N] drives SIG[n], …* and a *SIG[h:l], bits[H:L]* heading followed by *Drives SIG[h:l]* are
  read under the nearest *<REG> register implementation* heading, into the one field of that register holding the
  bits. APB4's PPROT map stays a stated residual.
  **Measured after:** the first run bound AHB5's bit 27 only. `CSW@JTAG-AP` shares the name and holds bits 30:28
  (`WFIFOCNT`) and 26:24 (`RFIFOCNT`), so a single bit named two fields and the reader failed closed. The section's
  own field headings (*Prot, bits[30:24]*, *SDeviceEn, bit[23]*) name fields only `CSW@MEM-AP` has, so they pick
  the register when several share the name. EvidenceIR dry-run of the 31 chains with the HEAD and the new binary:
  30 identical; ADIv6 moves only `register_records`. `CSW@MEM-AP.Prot` gains 7 `HPROT` bindings (bit 28 ->
  `HPROT[4]` in AHB3; bits 27-24 -> `HPROT[3:0]` in both; bit 27 -> `HPROT[6]` and `HPROT[4]` in AHB5), and every
  binding in the document, prose or table, gains its scope (the 14 AXI bindings name both AXI sections; `.41`'s 10
  name their register-description sections).

- ID: `EXTRACTION-REPRESENTATION-GAPS.41.h`
  Status: `done` (`2026-10-20`; CODE; found by `.41.g`; checklist below)
  Goal: **APB4's PPROT map binds `CSW.Prot[2:0]`.** ADIv6 *E1.7 AMBA APB4* heads *Prot[2:0], bits[30:28]* and says
  *Drives PPROT[2:0].*, but the conversion puts the sentence 13 elements after its heading, as a section header,
  and puts field headings for later bits in between; *Bit[29], Non-secure, corresponds to Prot[1] … PPROT[1] is
  HIGH* follows it. **Reproduce:** `CSW@MEM-AP.Prot` has no `PPROT` binding. **Impact:** an APB4 MEM-AP's
  protection bits reach no wire. Acceptance: decide whether the heading's field selection (`Prot[2:0]`) with a
  later *Drives SIG[2:0]* of the same width in the same register section binds, without pairing a sentence with
  the wrong heading, and measure every chain.
  **Decided `2026-10-20`: it binds when the section holds exactly one selection heading naming the same bits.** In
  `fill_field_signal_prose_bit_bindings` a *Drives SIG[h:l]* that no selection heading stands right before (that case
  is `.41.g`'s, and a different selection there is refused) binds to the one selection heading of its register
  section, of any element kind, whose selection is `[h:l]`, when that heading names the driven signal or a field of
  the section's register (*Prot*); the heading's register bits map to the signal's bits in order, and the binding
  keeps its numbered heading path. Two such headings, one naming other bits, or one naming neither the signal nor a
  field bind nothing: position alone never pairs a sentence with a heading. **Measured over all 31 chains** (evidence
  dry-run against `f258b7cda`'s): 30 identical; ADIv6 moves only `register_records`, one record:
  `CSW@MEM-AP.Prot` gains `PPROT[2]`, `[1]`, `[0]` from bits 30, 29, 28, scoped `E1.7 AMBA APB4 / E1.7.1 CSW register
  implementation` (21 -> 24 bindings). Rebuilt stage by stage: only
  `register_records` moves at each stage, the adapter and every `.isf` text byte-identical; no chain re-seals
  (`check_chain_currency.sh` 31 of 31 current).

- ID: `EXTRACTION-REPRESENTATION-GAPS.45`
  Status: `done` (`2026-10-20`; CODE; found by `.3.d.ii.a`; checklist below)
  Goal: **a presence row whose converter fused two code cells keeps the header's variant labels.** APB5
  `table_0018` (*Table B-3*, header `Signal | Width | Property | APB5 | APB4 | APB3 | APB2`) has a row the converter
  rendered as `PWAKEUPCHK | 1 | Check_Type &Wakeup_Signal | C | N N |  | N` — two codes fused into one cell and the next
  cell empty. The capture's rotation remap then relabelled the whole fragment: `presence_0020`-`presence_0023`
  (`PWDATACHK`, `PSTRBCHK`, `PREADYCHK`, `PBUSERCHK`) carry variant labels `OC`/`N`/`N`/`N` and `PWDATACHK`'s
  condition reads `DATA_WIDTH/8 Check_Type`, while the fused row itself is refused. **Reproduce:** `python3 -c
  "import json; e=json.load(open('generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json')); print([(r['presence_id'], [v['variant_label'] for v in r['variant_presence']]) for r in e['signal_presence_records'] if r['table_id']=='table_0018'])"`.
  The same converter fault hits `table_0016`: `PRESETn | 1 | - | - | Y Y |  | Y | Y` and `PADDR | ADDR_WIDTH | - | - |
  Y | Y | Y Y | ` are refused whole, so APB5 has no presence row for `PADDR` or `PRESETn` and `.3.d.ii.a.i`'s class
  value reaches 15 of the 16 covered signals. **Impact:** four check signals state presence under labels that are
  codes, not versions; a consumer reading `APB5` finds nothing for them, and two always-present signals have no row. Acceptance: find why one fused row moves the fragment's header remap (the
  `.5h` rotation votes, or the fused-pair split), keep the header labels when the header row is a proper
  header, refuse only the fused row, and measure every chain.
  **Found `2026-10-20`: neither — a data row flagged as a header became the header.** The converter put every row of
  `table_0018` among its header rows, and three (`PSLVERRCHK`, `PAUSERCHK`, `PWUSERCHK`) with their second cell
  flagged a header; `is_row_header_data_row` (a row whose only flagged cell is its first) does not lift them, so the
  capture's header view kept them, and its last row gave the variant labels (`OC`/`N`/`N`/`N`). The fused row only
  shared the table. **Decided:** in `capture_signal_presence_rows` a header row after the first whose first cell
  opens with an identifier and two more of whose cells are presence codes (`states_presence`) is a data row of the
  matrix, whatever its flags, and leaves the header view; the shared `recovered_trapped_data_rows` is unchanged, so no
  other reader moves. **Measured over all 31 chains** (evidence dry-run against `b2e6027a4`'s): 30 identical; APB5
  moves only `signal_presence_records` (24 -> 26) and its extraction-manifest counts: `table_0018`'s 4 records keep
  their signals with labels `APB5`/`APB4`/`APB3`/`APB2`, and `PAUSERCHK` and `PWUSERCHK` gain rows; the 4 fused `N N`
  rows (`PRDATACHK`, `PSLVERRCHK`, `PRUSERCHK`, `PWAKEUPCHK`) stay refused. Rebuilt stage by stage: SemanticIR and IntentIR
  carry the 26, the adapter and the `.isf` text byte-identical; no chain re-seals. **Declared residuals:**
  `PWDATACHK`'s condition reads `DATA_WIDTH/8 Check_Type`, because the converter fused its width into the property
  cell; and `table_0016`'s `PRESETn` and `PADDR` rows, each with a fused `Y Y` beside an empty cell, stay refused
  whole, so APB5 still has no presence row for either: splitting a fused pair into the empty neighbour would guess
  which column it came from.

- ID: `EXTRACTION-REPRESENTATION-GAPS.46`
  Status: `open` (`2026-10-03`; DESIGN then CODE; found by `.3.d.ii.a.i`)
  Goal: **a parity check signal lowers from the signals it covers and its enable, or the gap is requested.** IntentIR
  now carries 153 `check_signal_records` in 3 documents (AXI 123, AHB5 16, APB5 14: a check signal, its covered
  signals, width, granularity and enable, all as written). `.isf` has `(^ A B)` and `(set …)` but no parity or
  reduction form, so a byte-granular check (`ceil(ADDR_WIDTH/8)` wide, granularity `1-8`) would be a chain of XOR
  rules per byte that the adapter does not emit; a 1-bit check over several signals (`PCTRLCHK` over `PPROT,
  PWRITE, PNSE`) is one XOR chain under its enable (`PSEL`). **Reproduce:** `python3 scripts/measure_intent_to_isf_reach.py
  residuals` lists no check signal; `validate` on AXI's IntentIR reports `check_signal_records: 123`. **Impact:** the
  parity structure every AMBA5 interface declares reaches `.isf` as plain ports with no drive. Acceptance: decide,
  per record shape, whether the adapter emits a combinational rule (and how a parameterised byte count is expressed
  in `.isf`'s width vocabulary) or whether a parity form is a `FSMGEN-REQUESTS` packet; then emit or request, and
  measure every emitting chain under FSMGen `--strict --check`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.47`
  Status: `open` (`2026-10-03`; CODE; found by `.3.d.ii.a.ii`)
  Goal: **a rule stated on a collective name reaches each member.** `.3.d.ii.a.ii` carries the collective names
  AXI defines (`collective_signal_names`) and resolves them for one reader, the absent-value binder. Every other
  reader still takes `AxVALID`, `AxADDR` or `AxMMUATST` as a name of its own: AXI uses a collective name in 245 of
  its statements (*When AxMMUATST is deasserted LOW, the Stall flow is used*; *AxBURST* encodings), so each such
  constraint, condition, enum and relation reaches neither `AW*` nor `AR*`. **Reproduce:** `python3 -c "import json,re;
  e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json'));
  print(sum(1 for s in e['extracted_statements'] if re.search(r'\\bAx[A-Z]', s['text'])))"`. **Impact:** the largest
  AMBA document's generic rules are typed on a signal no interface has. Acceptance: decide where resolution lives
  (a SemanticIR projection that duplicates a record per member, or each reader consulting the records), apply it to
  the readers that resolve a condition's or constraint's signal, keep enum definitions on the collective name where
  the document defines them there, and measure every chain and the 11 seeds.

- ID: `EXTRACTION-REPRESENTATION-GAPS.48`
  Status: `done` (`2026-10-19`; DATA + CODE; found by `.3.d.ii.b.iii`, whose three children took the root to its
  mandatory rollover; ADR 0057)
  Goal: **the bounded root stays under rollover as leaves are registered.** Registering `.3.d.ii.b.iii`'s three
  children took the root from 229 to 232 lines, 90.6% of its 256-line health target, and
  `check_active_task_evidence.pl` refused it. The pressure is structural: when a contract's input state is
  `complete`, the checker requires the root's `## Executable Owner Registry` to declare every leaf the route
  catalog routes, closed ones included, so the root gains a line per leaf the tree has ever had and none back when
  one closes. After `.38` partitioned the tree the root was 142 lines with 77 ids; on `2026-10-19` it holds 143 ids,
  of which 43 are open. The other sharded trees carry the same registry: LIVE-DOCUMENT-PRESSURE-HEADROOM 95 ids /
  187 lines, KG-ISF-COMPLETENESS 90 / 195, SPEC-TO-INTENT-ALIGNMENT 84 / 212, EXTRACTION-QUALITY-GAUGE 66 / 200,
  CLAIM-VERIFICATION-ADOPTION 54 / 157. **Landed in `.3.d.ii.b.iii`'s commit to let it register its children:** the
  frontier's list of closed leaves (21 lines), which restated the route catalog's lifecycles, moved verbatim to
  *Frontier notes moved from the root* at the end of this part; root 232 -> 213 lines (83.2%), above the warning
  milestone. **Reproduce:** `bash scripts/check_doctrines.sh --only LIVE-DOC-SIZE` after adding one `- ID:` line
  per new leaf. **Impact:** about twenty more leaves and the tree refuses a commit that registers one. Acceptance:
  bound the registry by what a session can act on rather than by history (the route catalog already carries every
  leaf with its lifecycle and part, and ADR 0056 bounds the leaf count by it), keep a stable-path lookup for every
  open owner, change the checker's rule for every contract that declares a registry in one transaction with a RED
  control, and leave each root below its warning milestone.
  **Decided `2026-10-19` (ADR 0057): under a sharded route catalog the registry declares exactly the leaves not
  yet closed, plus every closed leaf whose primary part records no status for it.** It is ADR 0046's lifecycle cut,
  which the index already takes, applied to the root, and it reads the same evidence: the leaf's own status line in
  its part, so the registry never drops the only owner declaration a leaf has (`SPEC-TO-INTENT-ALIGNMENT.9a`, whose
  record `LIVE-DOCUMENT-PRESSURE-HEADROOM.16` writes, is the one such leaf and stays). An inline catalog keeps the
  complete-route rule. `check_active_task_evidence.pl`: `root_owner_ids` reads the declared ids; `validate_contract`
  requires every route that is not closed; `validate_route_lifecycles` refuses a closed, part-recorded leaf the
  registry still declares and a closed, unrecorded leaf it no longer declares. The self-test's closed-append fixture
  no longer registers its closed leaf, and three RED cases join it (80 -> 83 declared).
  **Landed over the six sharded trees in one transaction**, each pruned exactly to what the checker reported:
  extraction representation gaps 214 -> 116 root lines (144 -> 44 ids), spec-to-intent alignment 212 -> 134 (84 ->
  4), extraction quality gauge 200 -> 147 (66 -> 11), KG-ISF completeness 195 -> 136 (90 -> 29), pressure headroom
  187 -> 114 (95 -> 20), claim verification 157 -> 107 (54 -> 2); each registry header now states the rule. Closing
  a leaf now removes its `- ID:` line and its `root_required_literals` entry in the same commit. **Found and fixed
  beside it:** the book's live-document suite counts were stale, 93 common lifecycle cases (120 run) and 44
  active-task cases (80 before this leaf), and both now read what the suites run.
  Verification: `self-test 83/83; RED: each of the four rule mutations (the closed-declared check, the
  closed-unrecorded check, the open-required check, the old every-route rule) fails the suite; all six sharded
  contracts --check clean; task-node-retention 1,576 nodes, none moved or retired; no other diagnostic`
  Commit: `EXTRACTION-REPRESENTATION-GAPS.48 — a root's owner registry declares the leaves not yet closed`

- ID: `EXTRACTION-REPRESENTATION-GAPS.48.a`
  Status: `done` (`2026-10-19`; CODE; found by `.3.d.ii.b.iii.a`'s workspace test run, a regression of `.48`)
  Goal: **the trajectory controller finds a closed owner where ADR 0057 put it.** `.48` pruned the closed owner ids
  from six roots and searched every script that reads `- ID:` lines, but not the Rust: `validate_task_owner`
  (`crates/specforge/src/ir/trajectory.rs`) accepts a gap's owner only when the named root file contains `- ID:
  \`<owner>\``. The persisted controller input names `SPEC-TO-INTENT-ALIGNMENT.8` and the tests name `.5a`, both
  closed and both now recorded only in `docs/tasks/spec-to-intent-alignment/`. **Reproduce:** `cargo test --offline
  -p specforge-conformance --lib trajectory` — 6 failures, each *gap '…' owner 'SPEC-TO-INTENT-ALIGNMENT.…' is not
  declared in docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md*. The doctrine gate passed `.48` because it runs no Rust tests.
  **Impact:** the controller refuses its own persisted input, so the trajectory report cannot be derived.
  Acceptance: resolve an owner in the root or, failing that, in the tree's task-evidence directory, refuse an owner
  declared in neither, and pass the six tests.
  **Landed.** `validate_task_owner` accepts an owner the named root declares or, through `task_evidence_declares`,
  one a Markdown part of the tree's task-evidence directory declares — the directory named after the root in
  lowercase beside it, which is every migrated contract's declared `collection_directory` (all eight checked) — and
  its message now says where it looked. ADR 0057's consequences name this consumer. The `.3.d.ii.b.iii.a` work in
  flight was set aside (`git stash`, with a patch backup) so this lands on its own.
  Verification: `cargo test -p specforge-conformance --lib trajectory 27 passed (the 6 failures and the new
  a_closed_owner_is_found_in_its_trees_task_evidence); RED: without the fallback the same 7 fail; cargo fmt --check
  clean; cargo clippy --workspace --all-targets -D warnings clean; cargo test --workspace --lib --exclude
  specforge-production-graph 474 / 171 / 1,833 passed; flow_census rolled +1 function, +4 decision sites, +2 helper
  edges, boundary unchanged`
  Commit: `EXTRACTION-REPRESENTATION-GAPS.48.a — the trajectory controller finds a closed owner in its tree's evidence`

- ID: `EXTRACTION-REPRESENTATION-GAPS.49`
  Status: `done` (`2026-10-19`; CODE; found by `.3.d.ii.b.iii.c`; checklist below)
  Goal: **a state the document enumerates and names is a protocol state.** AXI states *When using credit control, an
  interface can be in one of four states: STOP, ACTIV ATE, RUN, DEACTIVATE* (`statement_3926`), *When ACTIVATEACK is
  asserted, the interface moves into the RUN state* (`statement_3939`), *RUN and STOP are stable states*
  (`statement_3928`) and *in the RUN or DEACTIVATE states* (`statement_3952`), yet `protocol_states` holds DEACTIVATE,
  STOP and ACTIVATE only. `extract_transition_bound_states` keeps a name only when two statements bind it as *the X
  state*; RUN's second mention is plural, its third unbound, and the counted enumeration is read by no grammar.
  **Reproduce:** AXI's EvidenceIR `protocol_states`. **Impact:** the credited-transport state machine lacks the
  state it spends its time in, and `.3.d.ii.b.iii.c`'s default (*If credit control signals are not included, the
  interface is assumed to be running when not in reset*) has no state to name. Acceptance: count a state bound in a
  plural phrase (*the RUN or DEACTIVATE states*) and a state listed by an enumeration whose stated count matches its
  members, keep the two-statement guard's purpose (one incidental capital word is no state machine), measure every
  chain, and decide whether `statement_3903`'s default binds to the RUN state.
  **Decided `2026-10-19`: a plural binding counts, a counted enumeration is not read, and the default stays an
  assumption.** A probe of both readers over the 31 chains, before any code: the plural changes one chain, AXI, which
  gains RUN; the enumeration occurs twice, both in AXI, one with no members and one whose converter-split *ACTIV ATE*
  fails its own count, so it would read nothing. `transition_bound_state_names_in` now reads, before *states*, two or
  more bare names joined by *or*, *and* or commas, after the same trigger and article the singular form takes,
  locating the trigger from the leftmost name; one name before *states*, or a list after a word that is no trigger,
  binds none, and the two-statement and two-state guards are unchanged. `statement_3903`'s *the interface is assumed
  to be running when not in reset* stays an IntentIR assumption: no record carries a state an interface takes by
  default, and the document never writes that *running* is the RUN state.
  **Measured over all 31 chains** (evidence dry-run, `.3.d.ii.b.iii.b`'s binary against the new one): 30 identical; AXI
  moves in `protocol_states` (+RUN, `named_state_0002`, citing `statement_3939` and `statement_3952`) and in the
  extraction manifest's counts. Rebuilt stage by stage: SemanticIR and IntentIR carry the fourth state; the adapter
  gains one residual, `isf_protocol_state_named_state_0002`; `manager.isf` byte-identical.

- ID: `EXTRACTION-REPRESENTATION-GAPS.50`
  Status: `open` (`2026-10-20`; CODE; found by `.3.d.iii.b`)
  Goal: **a prose hint gives a signal the role its sentence gives it.** The prose hint reader
  (`synthesize_signal_semantic_hints_from_prose`, source facts only) tags a signal from the words near it. When
  `.3.d.iii.b` returned 534 misfiled timing statements to their own classes it read them for the first time, and of
  its 9 new hints 6 give a role the sentence does not: TMC's `ATREADYM`/`ATREADYS` -> `handshake_valid_like` from *If
  there is valid data, that is, ATVALID HIGH, the data was accepted this cycle* (the valid signal is another named
  one), AHB's `HREADY` -> `handshake_valid_like` from *provide valid data in the final cycle of the transfer, as
  indicated by HREADY HIGH*, Wishbone's `RTY_I`/`RTY_O` -> `handshake_ready_like` from *indicates that the interface is
  not ready to accept or send data* (a negated readiness: a retry), and Wishbone's `DAT_O` (already tagged from another
  sentence) from *presents valid data on its [DAT_O()] lines* (the data the validity qualifies). **Reproduce:**
  `signal_semantic_hints` of those three chains, `statement_1727`/`1751`, `statement_0733`, `statement_0398`/`0412`,
  `statement_1232`. **Impact:** interfaces' signal records carry a handshake role the document does not give (TMC's
  table tags were already wrong and are now `cross_modality`); no transaction, temporal record or `.isf` text moves
  today. Acceptance: tag a signal valid- or ready-like only when the sentence makes that signal the bearer of the
  validity or readiness (not the data it qualifies, not another named signal, not a negated readiness), and measure
  every chain's hints and interfaces.

- ID: `EXTRACTION-REPRESENTATION-GAPS.51`
  Status: `open` (`2026-10-20`; CODE; found by `.3.d.iii.d`)
  Goal: **a sentence defining the document's own notation is apparatus.** ADIv6's appendix E3 (*Pseudocode
  Definition*) explains the pseudocode language the document uses, and its sentences are read as protocol content:
  `.3.d.iii.d` read *The block of lines consisting of elsif and its indented statements is optional* as an optional
  feature (`option_0006`), and *This statement indicates a special case that replaces the behavior defined by the
  current pseudocode* is classed `normative_statement` five times and `derived_rule` once. **Reproduce:** statements
  on the 31 chains not classed `apparatus` that name *pseudocode*, *elsif*, *indented statements* or *indentation*:
  55, 46 of them ADIv6's (39 `source_fact`, 6 `normative_statement`, 1 `derived_rule`), AXI 6, ACE 2, CHI C2C 1.
  **Impact:** a notation rule becomes a requirement or an option the hardware is held to, and an obligation reader
  can take it. Acceptance: tell a section that defines the document's notation (pseudocode, typographic or
  numbering conventions) from one that uses it, by its heading and its own words rather than a document's name, file
  its sentences as apparatus, keep a pseudocode function that describes hardware behavior (*The pseudocode function
  EnterDormantState describes the function of the JTAG-to-DS sequence detector*) where it is, and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.52`
  Status: `done` (`2026-10-20`; DESIGN + CODE; found by `.3.d.i`; checklist below)
  Goal: **a software agent's statement mints no hardware rule.** `.3.d.i` types what the document's software agents
  do (`software_intent`, 342 records in 11 chains), but the readers that ran before it still read the same statements
  as hardware: 137 IntentIR records cite one, 102 `temporal_invariants` (the USB4 connection-manager guide 64, ADIv6
  16, the IOMMU 7, RISC-V debug 5; the GIC guide's *Software must enable the system register interface before using
  these registers*), 25 `conditional_rules`, and 10 multi-statement records (5 transactions,
  2 interfaces, 2 actor relations, a protocol state) for which the software statement is one citation among several.
  **Reproduce:** IntentIR records whose `supporting_statement_ids` meet a `software_intent` record's. **Impact:** the
  category contract's `false_executable_intent_facts` floor (0 for a methodology guide) is breached by every invariant
  and rule minted from a software procedure, and a hardware reader of IntentIR cannot tell them from the device's own.
  Acceptance: decide, per surface, whether a record minted only from software-intent statements is withheld, marked,
  or kept; keep a record where the software agent is the hardware's external actor (*the debugger drives TDI*) or one
  citation of several; and measure every chain's invariants, rules and adapters.
  **Decided `2026-10-20`: withhold what a software statement alone mints; keep what the hardware also states.** A
  statement `software_intent` reads mints no SemanticIR invariant (`build_invariants` skips it through
  `SemanticContext::software_statement_ids`), so no IntentIR temporal invariant either; and a conditional rule every
  cited statement of which is software intent is dropped in EvidenceIR (`withhold_software_rules`; a rule citing none
  is kept). Not marked: a marked invariant would still be an executable fact to every reader that does not look, and
  the statement is already carried, typed, as software intent. Kept on purpose: the 8 multi-statement records (5
  transactions, 2 interfaces, a protocol state), where the software statement is one citation of several, and ADIv6's
  2 actor relations (*the debugger* drives `TDI`, samples `TDO`), where the software agent is the hardware's external
  actor. **Measured over all 31 chains** (evidence dry-run against `404ebb4eb`'s): 25 identical in EvidenceIR; 6 lose
  only conditional rules, 25 in all (USB4 connection-manager guide 18 -> 6, ADIv6 110 -> 104, RISC-V debug 80 -> 77,
  IOMMU 49 -> 47, SMMU guide 8 -> 7, GIC guide 12 -> 11). Rebuilt stage by stage (the 11
  `software_intent` chains): SemanticIR invariants and IntentIR temporal invariants -102 in 10 (USB4
  connection-manager guide 166 -> 102, ADIv6 552 -> 536, IOMMU 165 -> 158, RISC-V debug 163 -> 158, GIC guide 35 -> 32,
  CoreSight TMC and USB 3.2 2 each, AXI, SMMU guide and external debug guide 1 each), the IntentIR constraints derived
  from them -86, conditional rules -25 at every stage; ADIv6's adapter only renumbers one rule
  (`isf_obligations_not_lowered`: `rule_79` -> `rule_74`); every `.isf` text byte-identical. The changed semantic rule
  re-sealed every chain (`rebuild_stage_cascade.sh --write`, 31 content-identical).

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.41.g` (CODE) — DONE `2026-09-30`

- [x] **REPRODUCE / MEASURE** — ADIv6's AHB3 and AHB5 sections map `CSW.Prot` bits 28-24 to `HPROT` in prose; no
  binding carried them, and nothing recorded which implementation stated a binding.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `fill_field_signal_bindings` reads whole
  bindings under a field's own heading only, and `RegisterFieldSignalBinding` had no scope. RED, with the file restored
  byte-identically after each (8 mutations): no scope, no parent chain, a numbered heading keeping the register, a
  heading bound without its *Drives* sentence, or no section narrowing fails
  `an_implementation_section_binds_field_bits_in_prose_with_its_scope`; no merge also fails the `.41.e` test; accepting
  an ordinary word as a register fails `a_prose_bit_map_is_read_by_its_grammar_only`; an unwired reader fails
  `the_register_surface_binds_a_field_a_bit_map_names`.
- [x] **ADDRESSED (verified)** — `fill_field_signal_prose_bit_bindings`, `merge_signal_binding`,
  `numbered_heading_path`, `heading_section_number`, `register_heading_name`, `field_bits_heading`,
  `bit_drives_selects`, `signal_bits_heading`, `drives_selection`; `RegisterFieldSignalBinding.scopes`. ADIv6 rebuilt
  stage by stage, each stage validated once, the adapter last: 21 bindings on `CSW@MEM-AP.Prot`, all scoped, in
  EvidenceIR, SemanticIR and IntentIR.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical.
  `check_chain_currency.sh` 31 of 31 current (no seal moved). WIRE-BASED-100: all 11 seeds identical to the reports of
  `02ee6421b`'s binary, unchanged in Rust since (temp paths aside); `kg-bench` 158/158. `cargo fmt --check` clean;
  `cargo clippy --offline --workspace --all-targets -- -D warnings` clean (a nested tuple became `SignalBitsHeading`);
  `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,810 passed, 0 failed.
  `flow_census.json` rolled forward (+10 functions, +68 decision sites, +114 helper edges, +3 macros).
- [x] **GENERICITY (ADR 0006)** — section numbering, heading and sentence shapes, field headings against the register
  records' own fields; no document, register or signal name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (*A field bound to a wire*); KM
  `a-register-field-binding-to-a-wire` (amended); `.41.e`'s record corrected; `.41.h` opened.

## Frontier notes moved from the root

Moved verbatim from the root's `## Current Frontier` on `2026-10-19` by `.48`, whose record is above: the closed
leaves the frontier listed, with the phrase each one closed on. The route catalog's lifecycles are the authority for
which leaves are closed.

> `.3.c`,
> `.3.e`, `.3.e.i`, `.3.c.iii`,
> `.39`, `.3.c.ii.a`, `.3.c.ii.b`, `.42`, `.3.c.ii`, `.3.c.vii`, `.41.c`, `.43.a`, `.3.b`, `.3.b.i`, `.3.b.ii`,
> `.3.b.iii`, `.41.f`, `.3.b.iii.a`, `.3.b.iii.b`, `.3.b.iii.e`, `.3.b.iii.f`, `.3.b.iii.c`, `.3.b.iii.d`, `.3.b.iii.g`,
> `.3.b.iii.h`, `.3.b.iii.i`, `.3.f.iv`, `.41.e`, `.3.d`, `.41.g`, `.3.d.ii`, `.3.d.ii.a`, `.3.d.ii.a.i`, `.3.d.ii.a.ii`, `.3.d.ii.b`, `.3.d.ii.b.i` and `.3.d.ii.b.iii` are done
> (every typed table of `2026-09-26` has a reason or an owner; a symbol cites the table it was read from; a word in a
> signal table's name column is not a signal; a field's value table fills the field's values; a condition marker that
> opens a sentence opens that sentence's condition; the object before *and then* is no subject; an actor phrase ends at
> its punctuation; a character reference's semicolon is no clause boundary; a signal named from one actor to another is
> declared; a width operator the converter dropped stays a residual; a layout table sizes the register its heading names;
> a descriptive consequent's signal is the one it describes; every untyped uncited table has a family, an owner or a
> reader; an untyped signal table with a direction column declares its signals; every untyped uncited table is
> non-contract or owned; a field table takes the register of the heading before it; a field's value table is read without
> a header row or beside an attribute column; a record read from a table's row cites the table; a keyed row's value is its
> first sentence or its one code; an untyped timeout table is read as timing; a naming table that names a component and
> its state machine feeds no signal alias; a timeout's next state is the transition its timer takes; a timer name's
> operation mode is its condition; a state a stated transition names is a protocol state; a typed timing value is read without a printed
> footnote mark; a table mapping a field's bits to signal bits binds them; the unread assumptions and behaviors fall into six
> families; a prose bit map binds a field, and every binding keeps its scope; an assumption's shapes are measured; a signal's
> stated absent value rides its presence record to IntentIR; a check-coverage table is a typed relation that binds the class of
> covered signals; a document-stated collective name stands for its members; the remaining assume-sentences fall into six shapes; an assume-sentence about the document is apparatus; the remaining stated defaults need three remedies).

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.49` (CODE) — DONE `2026-10-19`

- [x] **REPRODUCE / MEASURE** — AXI's EvidenceIR `protocol_states`: DEACTIVATE, STOP, ACTIVATE (3), against the four
  states `statement_3926` enumerates.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `transition_bound_state_names_in` read only *the X
  state*, singular, so RUN had one binding (`statement_3939`) where `extract_transition_bound_states` asks two; the
  second mention is plural (`statement_3952`). RED, the file restored byte-identically after each (3 mutations, each
  failing its named test): the plural disabled (`a_plural_phrase_binds_each_name_it_coordinates`,
  `a_plural_mention_is_a_supporting_statement`), one name accepted as a plural and the trigger taken from the last
  examined word instead of the leftmost name (`a_plural_needs_two_names_after_a_trigger`).
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 30 identical; AXI `protocol_states` 3 -> 4 (RUN, two
  statements). Rebuilt stage by stage: SemanticIR and IntentIR `protocol_states` 3 -> 4; adapter residuals 65 -> 66;
  `manager.isf` byte-identical.
- [x] **NO REGRESSION** — the state grammar is outside the EvidenceIR seal closure: `check_chain_currency.sh` 31 of 31
  current at every stage after the rebuild. WIRE-BASED-100: all 11 seeds byte-identical to `9fc463824`'s reports (temp
  paths masked), each side on its own binary. `kg-bench` 158/158 (`fixtures_failed: 0`). `cargo fmt --check` clean;
  `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 171 / 1,847 passed, 0 failed. `flow_census.json` rolled forward (+13 decision
  sites, +1 helper edge, no function added); boundary counts unchanged.
- [x] **GENERICITY (ADR 0006)** — the existing trigger and article words plus *or*, *and*, a comma and the plural *states*;
  no state, document or protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*A state bound in a plural phrase is supported by
  it*); KM `transition-bound-state-fsm` (amended). No production rule was deleted or replaced: the singular form is
  read exactly as before.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.52` (DESIGN + CODE) — DONE `2026-10-20`

- [x] **REPRODUCE / MEASURE** — IntentIR records whose `supporting_statement_ids` meet a `software_intent` record's:
  137, of which 129 cite only such statements (102 temporal invariants, 25 conditional rules, 2 actor relations) and 8
  cite hardware statements too (`.project-data/tmp/erg-52`, before any code).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs` `build_invariants` and
  `crates/specforge/src/ir/evidence.rs` `extract_conditional_rules` ran over every statement, so a software agent's
  procedure became hardware invariants and rules. Changed: `SemanticContext::software_statement_ids` (from EvidenceIR's
  `software_intent`) and the skip in `build_invariants`; `withhold_software_rules` after `extract_conditional_rules`.
  RED, both files restored byte-identically after each (4 mutations, each failing its named test): any citation enough,
  an uncited rule dropped and an empty software set (`a_conditional_rule_read_only_from_software_statements_is_withheld`);
  the invariant skip off (`a_statement_about_a_software_agent_mints_no_invariant`). The two call sites are wired as
  measured: the rebuild removes exactly the 102 invariants and 25 rules.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 25 identical, 6 lose only conditional rules (25). The 11
  chains rebuilt stage by stage, each artifact validated once, the adapter last: -102 invariants, -102 temporal
  invariants, -86 derived constraints, -25 conditional rules, nothing added; `.isf` byte-identical; then
  `rebuild_stage_cascade.sh --write` re-sealed all 31 content-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: 10 seed reports byte-identical to
  `404ebb4eb`'s (temp paths masked); `seed_swd` differs only in its context line `existing_conditional_rules` 109 ->
  103 (ADIv6's six withheld rules), its scores unchanged. `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 171 / 1,871 passed, 0 failed. `flow_census.json` rolled (+1 function, +4 decision
  sites, +7 helper edges); boundary unchanged. Census: unread records 138, unchanged (the withheld records were typed).
- [x] **GENERICITY (ADR 0006)** — no new words: the set is `software_intent`'s, whose grammar `.3.d.i` measured.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*A software agent's statement mints no hardware
  rule*); KM `a-software-agents-statement-is-software-intent` amended. No production rule deleted; two readers narrowed.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.41.h` (CODE) — DONE `2026-10-20`

- [x] **REPRODUCE / MEASURE** — ADIv6's SourceIR: *E1.7.1 CSW register implementation*, *Prot[2:0], bits[30:28]* as
  body text among the field headings, *Drives PPROT[2:0] .* thirteen elements later as a section header;
  `CSW@MEM-AP.Prot` has no `PPROT` binding.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `fill_field_signal_prose_bit_bindings`
  bound a selection heading only to the element right after it. New branch for a *Drives* element no selection heading
  stands right before, over the section's selection headings (`section_selections`). RED, the file restored
  byte-identically after each (4 mutations, each failing its named test): the adjacency exclusion off
  (`an_implementation_section_binds_field_bits_in_prose_with_its_scope`: AHB3's *Drives ARPROT[2:0]* would bind); two
  headings allowed, any heading name, any selection
  (`a_drives_sentence_far_from_its_selection_heading_binds_to_the_only_heading_naming_its_bits`). The last three first
  survived because each refusal case put its heading right before the sentence, where the adjacency exclusion refuses
  it first; separating them killed all three.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 30 identical, ADIv6's one register record gains
  the three bindings and nothing else moves. ADIv6 rebuilt stage by stage, each artifact validated once, the adapter last; only
  `register_records` moves; `.isf` byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports byte-identical
  to `f258b7cda`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  474 / 171 / 1,872 passed, 0 failed. `flow_census.json` rolled (+10 decision sites); boundary unchanged. Census:
  unread records 138, unchanged.
- [x] **GENERICITY (ADR 0006)** — the heading and sentence grammar `.41.g` already reads, and the register records' own
  field names and ranges; no document, register or signal name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (the prose bit-map section); KM
  `a-register-field-binding-to-a-wire` amended. No production rule deleted or replaced.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.45` (CODE) — DONE `2026-10-20`

- [x] **REPRODUCE / MEASURE** — the leaf's command: `table_0018`'s 4 records with labels `OC`/`N`/`N`/`N`; its
  SourceIR rows dumped: all 11 among the header rows, three with the second cell flagged a header.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `capture_signal_presence_rows` built its
  header view from every header row `is_row_header_data_row` refuses, so a flagged data row became the header and its
  codes the labels. New `states_presence` in the data rows and out of the header view. RED, the file restored
  byte-identically after each (3 mutations, each failing
  `a_header_row_that_states_presence_is_data_and_the_labels_stay_the_headers`): not read as data, kept as a header,
  the code threshold unreachable.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 30 identical, APB5 moves only its presence records
  and manifest counts. APB5 rebuilt stage by stage, each artifact validated once, the adapter last; only
  `signal_presence_records` moves (24 -> 26) at each stage; `.isf` byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports byte-identical
  to `b2e6027a4`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  474 / 171 / 1,873 passed, 0 failed. `flow_census.json` unchanged (the flow check passes as is). Census: unread
  records 138, unchanged.
- [x] **GENERICITY (ADR 0006)** — the capture's own identifier and presence-code tests; no document, table or signal
  name.
- [x] **LOCKSTEP** — book `pipeline/presence-and-assumptions.md` (*A data row a converter flagged as a header is still
  data*); KM `signal-presence-matrix-capture` amended. No production rule deleted or replaced.
