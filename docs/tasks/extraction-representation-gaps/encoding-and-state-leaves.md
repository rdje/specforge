# EXTRACTION-REPRESENTATION-GAPS — encoding and state leaves

- Part ID: `encoding-and-state-leaves`
- State: `active`

Open ERG leaves continue here, after `packet-and-memory-leaves` reached its rollover (`2026-10-21`): tables keyed by
several signals' values, code columns that repeat, tables whose every row is a header row, and every later ERG leaf.
`.3.g.iii.b`, `.3.g.iii.c` and `.57` moved here whole.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b`
  Status: `done` (`2026-10-21`; DESIGN; split from `.3.g.iii`; opened `.3.g.iii.b.i`, `.3.g.iii.b.ii`)
  Goal: **a row binding several signals' or fields' values to a member or a meaning is typed.** **Reproduce:** APB's
  `PNSE | PPROT[1] | Physical address space`, AXI *A15.26* `State | SYSCOREQ | SYSCOACK`, and AXI *A13.14*'s legal MMU
  combinations are residuals. Acceptance: decide the record (the keys and values a row binds, and the member or
  meaning it names), its carrier to IntentIR, and whether a joint key over states reaches `protocol_states`. Measure
  the 9 tables of this shape on the 31 chains, and the untyped-table census's `signal_value_matrix` (22) and
  `field_value_encoding` (14, keyed by register fields) families it now owns. Then open the CODE leaf.
  **Measured `2026-10-21`** (31 chains). A table is jointly keyed when two or more of its columns are titled by
  different declared names, at least one column is not, and two or more rows follow. A declared name is a signal, a
  bit or slice of one (`PPROT[1]`, `ARBAR[0]`), a collective (`AxCACHE`), a register field (`CSW.ERRSTOP`), or a
  capitalized field name a register record declares (`TYPE`, `IDT`). Every column must be titled, and none only by a
  number; the address-alignment bit matrix and packet layouts are titled that way. This reading finds 55 tables with
  516 rows in 13 documents:
  - **Rows naming a member** (24 tables): a column whose cells are distinct names. Examples: APB `PNSE | PPROT[1] |
    Physical address space`, AXI-Stream `TKEEP | TSTRB | Data Type`, ACE's `ARBAR[0] | ARDOMAIN | ARSNOOP |
    Transaction type`, and ADIv6's `TYPE | CLASS` AP identification types. The 24 include 8 CoreSight SoC-600 TMC
    access-condition tables that name one register per row. They also include A4.4's `ARCACHE | AWCACHE` in both AXI
    revisions, which is one key written per channel, not a joint key.
  - **States** (3 tables, 12 states counting A15.26's continuation). AXI *A15.26* (issue L) and *E1-12* (issue H.c)
    key `Disabled`, `Connect`, `Enabled` and `Disconnect` by `SYSCOREQ | SYSCOACK`. LTI *B8.2* keys `ST_CLOSED` through
    `ST_CLOSING` by `LMOPENREQ | LMOPENACK`. None of the three documents has these states today.
  - **Rows stating a meaning** (28 tables): AXI *A12.15* `BTAGMATCH | BCOMP | Description` and *A13.10*, SMMU's
    `SMMU_CR0.SMMUEN | SMMU_S_CR0.SMMUEN | Traffic`, ADIv6's `CSW.ERRNPASS | CSW.ERRSTOP`, ATB's `ATVALID | ATREADY |
    AFREADY next cycle`, and 8 more TMC access-condition tables.
  Values are written as codes (`0b10`), levels (`HIGH`), labels of a member (`Non-secure`), don't-care marks (`-`,
  `x`, `0bX`), alternatives (`0b01 0b10`) and words (`Read-only`). In four more tables the converter emptied a key
  cell under a lost row span: three TMC tables and the continuation of LTI *B5.3*. The converter records `row_span` 1
  for every cell. **Found:** the single-column encoding reader reads two of these tables as one key's enum. AXI-L's
  A12.15 becomes `BTAGMATCH` with 7 members on 4 codes (`NOT_LEGAL = 1`,
  `COMPLETION_RESPONSE__PART_OF_A_TWO_PART_RESPONSE = 1`). A15.26 becomes `SYSCOACK` with `DISABLED = 0`,
  `CONNECT = 0`, `ENABLED = 1` and `DISCONNECT = 1`. These 11 statements are false. SemanticIR drops them all, so no
  `.isf` carries them. `.3.g.iii.b.i` withdraws them.
  **Decided.**
  - **Record.** EvidenceIR `value_combinations` holds one record per table, and a continuation joins the table it
    continues. The record lists the key columns (each title as written, the name it declares, and its bit or slice
    as written), the member column when there is one, and the meaning columns. Each row keeps its own table and
    every cell as written. A value is never interpreted, widened or resolved against an encoding.
  - **Members.** The member column is the first non-key column whose cells are all present, distinct and names. A
    cell is a name when `encoding_member_identifier` gives it an identifier and it neither ends a sentence nor holds
    a bullet.
  - **Precedence.** A table read here is not also read as one key's encoding, which withdraws the 11 false
    statements. Two key columns that name members of one collective are one key written per channel (A4.4). That
    table stays with `.3.g.iii.c`.
  - **Refusals.** A table with an emptied key cell is refused and counted in the surface's manifest. The converter
    states no span, so reading the cell above would be a guess. A column titled by a name no document declares is a
    meaning column, not a key: PCIe's `XT | T`, RISC-V's `GV | AV | PSC V`, ADIv6's `DP Offset`. A table left with
    fewer than two keys is not read.
  - **States.** A member column whose title ends in the word *state* names states. Each member becomes a
    `protocol_states` record, matched by name case-insensitively as `merge_transition_states` matches. It cites the
    table and its combination (`supporting_combination_ids`). A caption names the states but no machine, so the
    machine stays unbound.
  - **Carrier.** The record is carried verbatim through SemanticIR and IntentIR, as `protocol_states` is. The ISF
    adapter states each record as a residual packet until it can emit it. ISF can already state a decode (`switch` and
    `select` over guards), but each key's width and encoding and a member type must be joined first, so no FSMGen
    request is warranted. `.3.g.iii.b.i` does the reading. `.3.g.iii.b.ii` does the carriage, joins a label value to
    its key's encoding codes, and decides emission.
  - **Census.** The untyped-table census counts a table that `value_combinations` cites as typed. The
    `signal_value_matrix` and `field_value_encoding` families move to `.3.g.iii.b.i`. Their single-key members are
    not joint keys and stay in the census: `BRESP[2:0] | Response | Indication`, `Event | MODE | Outcome`,
    `DLCR.TURNROUND`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.i`
  Status: `done` (`2026-10-21`; CODE; split from `.3.g.iii.b`; checklist below)
  Goal: **a jointly keyed table is a value combination, and the states it keys are protocol states.**
  **Reproduce:** APB `table_0010`, AXI-L `table_0178` and `table_0239` + `table_0240`, and LTI `table_0040` have no
  record, and AXI-L's `BTAGMATCH` and `SYSCOACK` enum statements repeat codes. Acceptance:
  - add EvidenceIR `value_combinations` (family `evidence.protocol_structure`) as `.3.g.iii.b` decides, with its
    manifest surface;
  - refuse the single-column reading of a table read here;
  - merge the keyed states into `protocol_states` with `supporting_combination_ids`;
  - make the census script count a cited table as typed;
  - measure the 55 tables, the 11 withdrawn statements and the 12 states on every chain, then rebuild and re-seal.

  **Done `2026-10-21`.** `read_value_combinations` runs in every convergence pass, after the pass's collectives
  (`combination_key_columns`, `combination_key_name`, `is_combination_member_name`). Its table ids are the
  `combination_tables` that `scan_encoding_tables_by_signal_anchor` skips, and the last pass's records become
  `value_combinations` with a `jointly_keyed_tables` manifest entry (`value_combination_surface`).
  `merge_combination_states` runs after `merge_transition_states`, and `ProtocolStateRecord` gains
  `supporting_combination_ids`. On the 31 chains this gives 13 records in 4 documents, none refused. AXI-L has 7:
  A4.7, A9.1, A12.15, A13.10, A13.11, A13.14, and A15.26 with its continuation. ADIv6 has 3 (`TYPE | CLASS` twice,
  `CSW.ERRNPASS | CSW.ERRSTOP`), RISC-V IOMMU 2 (`IDT | DV_GSCV | PV_PSCV`) and APB 1 (`PNSE | PPROT[1]`). AXI-L gains
  `Disabled`, `Connect`, `Enabled` and `Disconnect` as protocol states and loses 15 false enum statements (`AxPROT` 4,
  `BTAGMATCH` 7, `SYSCOACK` 4). That frees its real `BTAGMATCH` encoding (*A12.14*, `table_0177`), which SemanticIR had
  dropped over the conflicting codes. `manager.isf` gains `(type BTAGMATCH (bits 2))` and `(BTAGMATCH (NONE 0)
  (SEPARATE 1) (FAIL 2) (PASS 3))`, FSMGen strict-clean with 0 diagnostics, and 4 state residual packets (72 -> 76).
  AXI-L's SemanticIR captured-region residuals fall from 236 to 235 (`table_0177`). RISC-V IOMMU, APB and ADIv6
  change only in EvidenceIR (their records and the manifest entry), and the other 27 chains only by the manifest
  entry. The 30 chains other than AXI-L keep every later stage and `.isf` text byte-identical.

  **Deviation from `.3.g.iii.b`.** `.3.g.iii.b` decided that a table read here is not also read as one key's
  encoding. Applied to both encoding readers, that withdrew A4.7's true `AxPAS` enum, because `AxPAS` alone
  determines A4.7's member. Only the single-column reader yields: it never asks whether a code determines its member.
  `.3.g.iii.a`'s column reader mints only a column that does, so it is not restricted.

  **Corrected `2026-10-21`** (`.3.g.iii.b`'s measurement, recorded in this part). Its script read all 78 persisted
  EvidenceIR directories, not the 31 retained chains, and it read SourceIR before `source_ir_with_row_header_rows_in_body`
  lifts row-header data rows. On the 31 chains the shape is the 13 tables above, in 4 documents. One keys states
  (A15.26: 4 states), and none is refused. The single-column reader's false statements number 15, not 11: A4.7 also
  gave `AxPROT` four members on two codes. The other 42 tables are in persisted chains outside the retained 31: SoC-600
  TMC access tables, AXI H.c, LTI, AXI-Stream, ATB, SMMU and GIC. They take effect when those documents are re-ingested.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.i` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — the jointly keyed tables of the 31 chains (13 in 4 documents) and of the 78 persisted
  directories (55); the `Enum` statements the single-column reader emits for A4.7, A12.15 and A15.26; AXI-L's
  protocol states and `BTAGMATCH` symbol before and after.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: no reader recorded a row keyed by
  several declared names. The single-column reader (`scan_encoding_tables_by_signal_anchor`) took one value column
  of such a table without asking whether its codes determine the member, and so published enums with repeated codes.
  SemanticIR then dropped those and AXI's true `BTAGMATCH` with them. New `ValueCombinationRecord`, `CombinationKey`
  and `CombinationRow`, plus `read_value_combinations`, `combination_key_columns`, `combination_key_name`,
  `is_combination_member_name`, `value_combination_surface` and `merge_combination_states`. RED 6/6, the file
  restored byte-identically after each mutation, and each fails its test: one collective's members read as a joint
  key, an emptied key cell read, a sentence naming a member, any member column naming states, a continuation standing
  alone, and the single-column reader taking a combination.
- [x] **ADDRESSED (verified)** — dry-run of the 31 chains as measured. All 31 rebuilt stage by stage, and
  `rebuild_stage_cascade.sh --write` re-sealed every chain content-identically. FSMGen `--strict --check --json` on
  AXI's `manager.isf`: success, 0 diagnostics.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `3900d932a`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,913 passed. The proof test counts 49 EvidenceIR fields and the rule
  inventory 203 rules. `flow_census.json` rolled (+6 functions, +59 decision sites, +117 helper edges, +3 semantic
  macros), and its boundary moved for the new producer (rule roots 133, grammar declassifiers 74). The census script
  counts a combination's tables as typed, and the family inventories name `value_combinations` and its producer.
- [x] **GENERICITY (ADR 0006)** — declared signal, collective, register and field names, column-title shape, the
  word *state* closing a member column's title, and a caption's table reference; no document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A row keyed by several signals together*),
  `source-to-intent-contract.md`, `quality/validation.md` and `reference/representation-gaps.md` (the state-machine
  row, whose stale count of 45 states is now 78 in 7 documents); KM card
  `a-jointly-keyed-table-is-a-value-combination` (new), and `a-signal-titled-code-column-determining-its-member-is-an-enum`
  amended; ROADMAP states nothing this falsifies. No production rule deleted or replaced.

- **Correction to `.54.a.i`, `.54.a.i.a` and `.56.b`** (`2026-10-21`; found during `.3.g.iii.b.i`). Each recorded
  that no decision record was written because `decision_records` was past its rollover or at 59 of 74 files. That
  reason was wrong. The per-file pressure (`bytes_each` 97.1%, `lines_each` 92.6%) is ADR 0038's alone, an accepted
  record that cannot grow. A new ADR is a new small file, and the collection holds 59 of 74 files. The file-count
  axis has the population and peak-day re-derivation of ADR 0041 (`DECISION-RECORD-CAPACITY-HEADROOM`). Each rule
  stays where those records put it. ADR 0038's permanent per-file warning is `LIVE-DOCUMENT-PRESSURE-HEADROOM.8`'s.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.ii`
  Status: `done` (`2026-10-22`; CODE; split from `.3.g.iii.b`; checklist below; opened `.3.g.iii.b.iii`)
  Goal: **a value combination reaches IntentIR and the ISF adapter.** **Reproduce:** after `.3.g.iii.b.i`,
  `value_combinations` stops at EvidenceIR. Acceptance:
  - carry the record verbatim through SemanticIR and IntentIR;
  - join each label value to its key's encoding codes where the key's enum states the label (`StatedValueEncoding`);
  - state each combination as an ISF adapter residual packet that names what it lacks;
  - run an FSMGen strict probe to decide whether a combination that names a member lowers to a decode, and open
    that leaf.

  **Done `2026-10-22`.**
  - **Carried.** SemanticIR (`semantic.carried_evidence`) and IntentIR (`intent.carried_semantics`) carry
    `value_combinations` verbatim, and the converge snapshots count them as they count `protocol_transitions`.
  - **Label values.** `bind_combination_value_encodings` runs after the convergent loop, beside
    `bind_stated_absent_values`. It joins each key value that is a label of its key's encoding to that encoding's codes
    (`CombinationRow.value_encodings`, `CombinationValueEncoding`), through the key's enum, then its collectives, then,
    for a collective key, its members' enums. A code literal, or a value with no letter, is no label.
  - **Measured.** AXI has 31 joined values in four records (`AxMMUSECSID` `Non-secure` `0b00`, `Realm` `0b10`;
    `AxDOMAIN` `Shareable` `0b01`/`0b10`; `AxMMUFLOW` `ATST` `0b01`). 17 label values join nothing: lists of
    alternatives (`Stall, NoStall, PRI` 6, `NoStall, PRI` 1), classes of codes (`AxCACHE` `Cacheable` 2,
    `Non-cacheable` 2, `Device` 1), and `AxPAS` `Non-secure` 3, whose member is written `Non-secure (NS)`.
  - **Adapter.** The ISF adapter states one `isf_protocol_value_combination_*` residual per record, citing its tables:
    13 (AXI 7, ADIv6 3, RISC-V IOMMU 2, APB 1).
  - **Rebuilt.** All 31 chains were rebuilt stage by stage. Only AXI's EvidenceIR moved (its value encodings), the four
    documents' SemanticIR and IntentIR gain the field, and every `.isf` text is byte-identical. The cascade re-sealed
    content-identically.
  - **Probe** (`.project-data/tmp`, FSMGen pin `54ac1c22`): APB's `PNSE | PPROT[1] | Physical address space` as an enum
    `PAS` plus one rule per row guarded by `(& (== PNSE v) (== PPROT[1] w))`. It was strict-clean, 0 diagnostics,
    writing an output port, writing actor storage declared `(width 2)`, and as a transaction of four `select`s. The
    first attempt used `(bits PPROT 1 1)`, which is no expression operator: the transaction form failed on the
    operator, and the rules form on rule writes FSMGen could not prove disjoint. `PPROT[1]` fixed both.
  - **Found.** A storage variable declared `(type PAS)` with no width passes strict but prints `Use of uninitialized
    value $width … Parser.pm line 8750`. `FSMGEN-REQUESTS.13` owns it, opened next.
  - **Decided.** A member-naming combination with literal key values can lower. How it should lower is a design
    question (which actor's storage, a flopped rule for a relation the document states combinationally, the
    unresolved labels, don't-care marks and alternatives as guards), so `.3.g.iii.b.iii` is opened as a DESIGN leaf.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.ii` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — the 13 records' stages after `.3.g.iii.b.i` (EvidenceIR only); their key values by
  kind (code, bit, mark, label, list, class); the adapter's residual packets per chain.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs` and `intent.rs` declared no carrier, so the
  record stopped at EvidenceIR, and `crates/specforge/src/ir/adapters.rs` stated no residual for it. A label value
  had no codes (`crates/specforge/src/ir/evidence.rs`). New `bind_combination_value_encodings` and
  `CombinationValueEncoding`; carried fields in both stages; the adapter's `value_combination` residual. RED 5/5, each
  file restored byte-identically and each mutation failing its test: a collective key reaching no member's encoding,
  every value skipped as no label, SemanticIR dropping the record, IntentIR dropping it, the adapter stating no residual.
- [x] **ADDRESSED (verified)** — 31 chains rebuilt stage by stage, the cascade re-sealed content-identically;
  `.isf` texts byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `ce8443075`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,915 passed; the proof tests count 63 SemanticIR and 62 IntentIR rule
  fields (42 carried) and the inventory 205 rules. `flow_census.json` rolled (+1 function, +7 decision sites, +36
  helper edges, +2 semantic macros); boundary unchanged. The claim family inventory names `value_combinations` in
  both carried families.
- [x] **GENERICITY (ADR 0006)** — the key's own name, its collectives and the document's `Enum` statements; no
  document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md`, `pipeline/semanticir.md`, `pipeline/intentir.md`,
  `source-to-intent-contract.md`, `quality/validation.md`; KM card `a-jointly-keyed-table-is-a-value-combination`
  amended. No production rule deleted or replaced.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.iii`
  Status: `done` (`2026-10-22`; DESIGN; opened by `.3.g.iii.b.ii`; opened `.3.g.iii.b.iv`)
  Goal: **decide how a value combination that names a member lowers to `.isf`.** **Reproduce:** the 13
  `isf_protocol_value_combination_*` residuals, and `.3.g.iii.b.ii`'s strict-clean probe of APB's decode. Acceptance:
  decide the target (an actor's storage, no invented port) and the owning actor. Decide whether a flopped rule
  honestly states a relation the document states combinationally. Decide how guards read a don't-care mark (`-`), a
  list of alternatives (`0b01 0b10`, `Stall, NoStall, PRI`), a class of codes (`Cacheable`) and a label written
  without its member's abbreviation (`Non-secure` for `Non-secure (NS)`). Decide what a combination that names only a
  meaning lowers to, if anything, and whether its states' keys become the states' encodings. Measure every chain,
  then open the CODE leaf.
  **Measured `2026-10-22`** (31 chains, the 13 records):
  - **Shape.** 8 name a member (APB's PAS; AXI's A4.7, A9.1, A13.14 and A15.26; ADIv6's two `TYPE | CLASS`) and 5 a
    meaning only. Key values: literal codes and bits, 31 labels joined to codes, 17 labels with no code, and
    don't-care marks (30 cells in 10 rows: A13.14 6 rows, A4.7 2, ADIv6 1 each).
  - **Consumers.** No condition on the chains names a combination's member and needs it resolved to keys. AXI's
    rules about a physical address space (*If AxMMUSECSID is Realm, the physical address space must be Non-secure or
    Realm*, `condrule_0197`-`0199`) name the space that `AxPAS`'s own enum encodes. APB's conditions name none of
    its four spaces. A15.26's state rules are read from the table's own rows (`condrule_0233`).
  - **FSMGen.** Every `.isf` assignment is clocked. A rule and a drive assign with `<-` and update on the next clock
    (book `13g`, `13c`), and a transaction is a state sequence. `.isf` has no combinational assignment.

  **Decided.**
  - **No lowering of the table itself.** A decode lowered to storage or a port adds a register and a one-cycle lag
    the document does not state, and it invents a storage target, which ADR 0016 forbids. The probe showing FSMGen
    accepts that form decides nothing about whether it is faithful. A meaning-only combination has nothing to lower.
  - **A guard vocabulary, built when a consumer exists.** A condition that names a member ("if the physical address
    space is Root") would resolve to the conjunction of its keys' values. A don't-care key drops out, alternatives
    become a disjunction, and an unresolved label or a class of codes refuses the guard. With no such condition on
    the chains today, nothing is built; the condition readers meeting one is the trigger. No FSMGen request is
    warranted (ADR 0033: fidelity before speculative `.isf` expansion).
  - **States keep their encoding as stated.** A15.26's four states are observable from `SYSCOREQ` and `SYSCOACK`, so
    no state register is implied. The adapter's state residual already says the encoding is missing; for a state a
    combination keys, that is no longer true, and the residual must say which combination states it.
  - **Residual text.** Each value-combination residual names why it stays: no consumer, and no combinational
    target in `.isf`.
  - **Label near-miss.** `AxPAS` `Non-secure` against the member `Non-secure (NS)`: a label equal to a member's
    name before a parenthesized abbreviation is that member. The same resolver serves stated absent values, so the
    change is measured there too.
  - CODE: `.3.g.iii.b.iv` (the two residual texts and the abbreviation match).

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.iv`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.g.iii.b.iii`; checklist below)
  Goal: **a value combination's residual says why it stays, a state a combination keys says its encoding is
  stated, and a label matches a member written with a parenthesized abbreviation.** **Reproduce:** the 13
  `isf_protocol_value_combination_*` residuals ask for a decode; AXI's four `combination_state_*` residuals list
  "encoding" as missing; `AxPAS` `Non-secure` joins nothing against `Non-secure (NS)`. Acceptance: implement
  `.3.g.iii.b.iii`'s three decisions; measure the residual texts, the joined labels and the stated absent values
  on every chain; rebuild the chains that move.
  **Done `2026-10-22`.**
  - **Abbreviation.** `label_encoding` also compares a cell's name before a closing parenthesized one-word
    abbreviation (`name_before_abbreviation`: *Non-secure (NS)* -> `non-secure`; *N/A (not legal for CMOs)* is a
    remark, not an abbreviation).
  - **State residual.** A protocol state with `supporting_combination_ids` drops "encoding" from what its residual
    lists as missing and names the combination that states it.
  - **Combination residual.** It now says it stays for want of a consumer, and that `.isf` has no combinational
    assignment that states the table without an invented storage target.
  - **Measured.** Dry-run of the 31 chains: 30 identical. AXI joins 3 more label values, `AxPAS` `Non-secure` ->
    `0b001` in A13.10 and A13.11, so 34 in all. Stated absent values are unchanged on every chain.
  - **Rebuilt.** AXI, APB, ADIv6 and RISC-V IOMMU were rebuilt stage by stage. AXI's value encodings move through
    IntentIR, and the 13 combination residuals and AXI's 4 keyed-state residuals change text. Every `.isf` text is
    byte-identical.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.iv` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — the residual texts of the 13 combinations and the 4 keyed states; the 17 unjoined
  label values by cause; the stated absent values of every chain before and after.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/adapters.rs` listed "encoding" as missing for every
  state and asked a combination for a decode that `.3.g.iii.b.iii` decided not to build; `crates/specforge/src/ir/
  evidence.rs` `label_encoding` compared whole cells only. New `name_before_abbreviation`. RED 4/4, each file
  restored byte-identically: a remark read as an abbreviation, the abbreviation form never compared, a keyed state
  still listing its encoding as missing, the combination residual still asking for a decode.
- [x] **ADDRESSED (verified)** — 4 chains rebuilt stage by stage; `.isf` texts byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `b365fc43a`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,916 passed. `flow_census.json` rolled (+1 function, +7 decision sites,
  +5 helper edges, +1 semantic macro); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — cell shape (a closing parenthesized single word) and the record's own fields; no
  document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md`; KM card `a-jointly-keyed-table-is-a-value-combination` amended.
  No production rule deleted or replaced.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.c`
  Status: `done` (`2026-10-22`; DESIGN; split from `.3.g.iii`; opened `.3.g.iii.c.i`, `.59`)
  Goal: **a code column that maps several members to one code is carried as what it is.** **Reproduce:** AXI *A4.4*:
  `ARCACHE` `0b1010` is Write-Through No-Allocate and Write-Allocate on reads; today's `ARCACHE` enum keeps 6 of 12
  rows and `AWCACHE` none. Acceptance: decide whether a repeating column is an aliasing encoding (code -> members) or
  stays a residual. Decide whether today's 6-member enum is kept, re-read or withdrawn, and how the parenthesized
  alternates (*0b1110 ( 0b0110 )*) read. Measure AXI and the wire golds.
  **Measured `2026-10-22`.** A4.4 (`table_0058`) gives each of 12 memory types an `ARCACHE` and an `AWCACHE` code.
  Per column a member has one preferred code, but each column repeats four codes. On reads `0b1010` is both
  Write-Through No-Allocate and Write-Allocate, because a read cannot carry the write-allocate hint. On writes `0b0110`
  is both Write-Through No-Allocate and Read-Allocate.
  The document states the rest: *Values in brackets are permitted but not preferred. Values that are not shown in the
  table are reserved* (`statement_1272`). Today EvidenceIR has 10 `Enum ARCACHE` statements; the two bracketed rows
  parse no code. SemanticIR then drops both members of each repeated code (`KG-ISF-COMPLETENESS.5.xiv.a`: *no encoding
  assigns one code to two members*), so `manager.isf` keeps 6 members and `AWCACHE` has none. The rule's premise is
  false here. Its default still holds for the misreads it was written against (9 enums, 68 members to 20, all wrong
  columns or one-bit reads). FSMGen strict accepts an enum with two members on one value (probe on the pin). No
  kg-bench gold or seed reads `ARCACHE`'s members; the two AXI fixtures that name it are sideband-stability golds.
  **Found:** the `.isf` enum type's width comes from the member count, not the largest code
  (`isf_ir.rs`: `members.len().next_power_of_two()`). 5 of the 22 enums in the 31 chains' emitted `.isf` overflow
  their type, all in AXI: `AWATOP` 4 bits for 49, `AxDOMAIN` 1 for 3, `AWSNOOP` 2 for 11, `ARSNOOP` 2 for 13, `ARCACHE`
  3 for 15. FSMGen's contract makes an enum's values and its co-declared type alias distinct roles, so it rightly
  accepts this; the defect is SPECFORGE's. `.59` owns it.
  **Decided.**
  - **An aliasing encoding.** A channel's code column that gives every member a code, where two members may share
    one, is that signal's aliasing enum: member -> preferred code, kept whole, with shared codes. It is minted only by
    an explicit reader for the A4.4 shape: an encoding table whose signal-titled code columns name members of one
    collective (one key per channel, the shape `.3.g.iii.b.i` routed here), with distinct member names. The collision
    rule stays the default for every other reader, and the aliasing enum is exempt only because its reader says so.
  - **Brackets.** The unbracketed code is the member's code. The bracketed one is a permitted alternative
    (`statement_1272`), carried with the member in EvidenceIR and not emitted. An `.isf` member has one value, and a
    second name for it would be invented.
  - **Today's enum** is re-read, not withdrawn. `ARCACHE` keeps its 6 members unchanged and gains the other 6;
    `AWCACHE` gains its 12.
  - **Order.** `.59` (an enum type as wide as its largest code) lands first, because the re-read `ARCACHE` codes reach
    15. Then `.3.g.iii.c.i`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.57.a`
  Status: `done` (`2026-10-22`; CODE; split from `.57`; checklist below)
  Goal: **a header row of leading row labels over data is data.** **Reproduce:** OpenCAPI DLL *Table 8-1* rows
  (*Transmission order | 8 consecutive bytes | P | P | P*) stay header rows, so no reader sees them; I2S's timing table,
  USB 3.2's *7-14* and its packet figure likewise. Acceptance: generalize `source_ir_with_row_header_rows_in_body`
  to `.57`'s rule; measure the 6 tables, the 131 already lifted (unchanged) and every chain's EvidenceIR; route the
  lifted tables that keep more than one header row to the leaves owning their families' readers; rebuild what moves.
  **Done `2026-10-22`.** `is_row_header_data_row` (the one rule shared by the lifting, `recovered_trapped_data_rows`,
  the timing and signal-table readers and the completeness coverage) now lifts a row whose flagged cells are a
  leading run of row labels over unflagged data. A row with every cell flagged, a mid-row flag or an empty first cell
  stays a header row. **Measured:** dry-run of the 31 chains with every EvidenceIR identical, and chain currency
  31/31 with no chain rebuilt. No current reader turns the newly lifted rows of OpenCAPI's *8-1*/*8-2*, I2S's timing
  table and USB's *7-14* into a record, so the rows now wait in the body for their families' readers. **Routed:** 24
  lifted tables keep a second header row whose qualifiers a reader taking the last header row loses:
  - `.3.f.iii`, the grouped timing tables: USB 3.2 *6-16*, *6-20*, *6-31*, *6-33*, the two `tBurst` tables, *7-8*
    and an untitled one, and I2S's timing table (*TRANSMITTER*/*RECEIVER*, *LOWER*/*UPPER LIMIT*, *MIN*/*MAX*);
  - `.3.c.v`, the requirement and legality matrices: AXI *A12.11*, *A12.13* (*MTE_Support = Basic* …), I2C *Table
    3*, OpenCAPI *8-1*, *8-2*, *8-3*, and USB 3.2 *7-14*, *8-8*, *9-2*, *9-10* and an untitled event table;
  - `DOC-INTENT-TAXONOMY.4b`, layouts: OpenCAPI *2-2* (training sets) and a USB packet figure;
  - non-contract: TMC's revision table and ADIv6's component-ID offset grid.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.57.a` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — the 137 header-only tables by lifting outcome before and after; the 24 that keep two
  or more header rows, by family.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `is_row_header_data_row` required the
  row label to be the only flagged cell, so a row labelled by two columns stayed a header row. RED 2/2, the file
  restored byte-identically: one label only, a fully flagged row lifted.
- [x] **ADDRESSED (verified)** — the rule lifts the measured rows; no artifact moves on any chain.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `93de7a29c`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,920 passed. `flow_census.json` rolled (+2 decision sites); boundary
  unchanged.
- [x] **GENERICITY (ADR 0006)** — the converter's header flags and cell emptiness; no document or table name.
- [x] **LOCKSTEP** — no book page states the old rule; KM card `a-signal-titled-code-column-determining-its-member-is-an-enum`
  already names `.57.a`. No production rule deleted or replaced.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.a`
  Status: `done` (`2026-10-22`; CODE; split from `.3.c.iv`; checklist below)
  Goal: **a table with a column per bit of one signal is a value combination.** **Reproduce:** AHB *3-5*
  (`HPROT[3] Modifiable` … with `-` marks) and *3-7* (`HPROT[6:2]` with `0 or 1`) reach no record. Acceptance: widen
  `combination_key_columns` to distinct bits of one signal when a bit column holds a mark or an alternative, and read a
  title's bit name after the slice; keep `.3.g.iii.a`'s concatenated codes untouched; measure AHB and every chain;
  rebuild what moves.
  **Done `2026-10-22`.**
  - **Titles.** `combination_key_name` reads a signal title that names its bit after the slice (`HPROT[3]
    Modifiable`); a register field or a bare field with trailing words is still no key.
  - **One signal's bits.** `combination_key_columns` accepts two or more distinct bits of one signal.
    `read_value_combinations` keeps such a table only when a bit cell is no plain `0` or `1`.
  - **Measured.** Dry-run of the 31 chains: 30 identical. AHB gains *3-5* (4 bit keys, member column *Description*,
    8 rows) and *3-7* (5 bit keys, member column *Memory Type*, 8 rows). It also loses 6 false `Enum HPROT`
    statements: the single-column scan had read *3-7*'s `HPROT[6]` column as `HPROT`'s code (`WRITE_BACK__SHAREABLE =
    1` …), and the precedence now leaves the table alone. SemanticIR had dropped all 6 over their repeated codes.
  - **Rebuilt.** AHB was rebuilt stage by stage. The two records reach IntentIR, the adapter's residuals go from 39
    to 41, and every `.isf` text is byte-identical.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.a` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — the 31 chains' tables with two or more bit columns of one signal (AHB *3-2*, *3-5*,
  *3-7*; ADIv6's layout grids) and the statements each reader emits for them.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `combination_key_columns` needed two
  different names and `combination_key_name` refused a title naming its bit, so a per-bit table reached no reader but
  the single-column scan, which misread *3-7*. RED 3/3, the file restored byte-identically: a bit name after the slice
  refused, one signal's bits no keys, all-binary bits read as a combination — each fails
  `a_column_per_bit_with_marks_is_a_value_combination`.
- [x] **ADDRESSED (verified)** — AHB rebuilt stage by stage; `.isf` texts byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `fbbbcdcab`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,921 passed. `flow_census.json` rolled (+6 decision sites, owner this
  leaf).
- [x] **GENERICITY (ADR 0006)** — title shape (a slice and the bit's name), cell shape (a plain bit or not); no
  document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (the keys bullet and the *Table 3-5* note); KM card
  `a-jointly-keyed-table-is-a-value-combination` amended. No production rule deleted or replaced.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b`
  Status: `open` (`2026-10-22`; DESIGN; split from `.3.c.iv`)
  Goal: **a signal's bit described as a named sub-field is typed.** **Reproduce:** AHB *3-6* (*Bit | Name |
  Description*) and APB's `PPROT` table (*PPROT[0] | Normal or Privileged | … | LOW indicates normal access …*) reach
  no record. Acceptance: decide the record (bit, name, description, the meaning per level as written), its carrier to
  IntentIR, and how it relates to the signal's own record; measure every chain's row-per-bit tables; open the CODE leaf.

- ID: `EXTRACTION-REPRESENTATION-GAPS.59`
  Status: `done` (`2026-10-22`; CODE; found by `.3.g.iii.c`; checklist below)
  Goal: **an `.isf` enum type is as wide as its largest code.** **Reproduce:** AXI's `manager.isf` declares `(type
  AWATOP (bits 4))` for members up to 49, and `AxDOMAIN` 1 bit for 3, `AWSNOOP` 2 for 11, `ARSNOOP` 2 for 13, `ARCACHE`
  3 for 15; `isf_ir.rs` sizes the type from the member count. Acceptance: size each enum type from its largest member
  code (at least 1 bit); measure every chain's `.isf` and FSMGen strict; no member changes.
  **Done `2026-10-22`.** `enum_type_bits` sizes the co-declared type from the largest integer member code: decimal,
  `0x` or `0b`, at least 1 bit. An enum with a non-literal member keeps the count-based width. With distinct codes
  the largest one needs at least as many bits as the count, so a type only widens. AXI's `manager.isf` was rebuilt:
  `AWATOP` 4 -> 6, `AxDOMAIN` 1 -> 2, `AWSNOOP` 2 -> 4, `ARSNOOP` 2 -> 4, `ARCACHE` 3 -> 4. Its members are unchanged,
  and FSMGen strict passes with 0 diagnostics and no stderr. The other four emitted `.isf` files are unchanged.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.59` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — every enum of the 31 chains' 5 emitted `.isf` files against its type: 22 enums, 5
  overflowing, all AXI's.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/isf_ir.rs` sized the type
  `members.len().next_power_of_two()`, a count, so a sparse code set (`AWATOP` 48, 49) outgrew it, and FSMGen's
  contract checks enum values against no type. New `enum_type_bits`. RED 2/2, the file restored byte-identically:
  the width from the member count, no one-bit floor.
- [x] **ADDRESSED (verified)** — AXI rebuilt stage by stage; only its `manager.isf` type lines move; FSMGen strict
  clean.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `6096ebd16`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,917 passed. `flow_census.json` rolled (+1 function, +1 decision site,
  +5 helper edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — the members' own codes; no enum, document or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (the type's width). No production rule deleted or replaced.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.c.i`
  Status: `done` (`2026-10-22`; CODE; split from `.3.g.iii.c`; checklist below)
  Goal: **a channel's code column that gives every member a code is that signal's aliasing enum, and a bracketed code
  is the member's permitted alternative.** **Reproduce:** AXI *A4.4*: `ARCACHE` keeps 6 of 12 members, `AWCACHE` none,
  and the bracketed rows parse no code. Acceptance: implement `.3.g.iii.c`'s decisions. The reader and its carrier
  must mark the aliasing enum so SemanticIR's collision rule exempts it alone. Measure AXI and every chain, then
  rebuild and FSMGen strict.
  **Done `2026-10-22`.**
  - **Reader.** `read_channel_code_tables` runs in each convergence pass after the collectives. It reads an encoding
    table whose two or more signal-titled code columns (a signal or a slice of it, never one bit) name members of
    one collective, with a member column of distinct names (`is_combination_member_name`) and a code in every channel
    cell (`channel_code_and_alternative`).
  - **Statements.** Each channel gets `Enum SIGNAL MEMBER = CODE.` per row with the preferred code, then `Enum SIGNAL
    MEMBER also takes CODE.` for a bracketed alternative. If its codes repeat, `Enum SIGNAL shares a code between
    members.` follows. The single-column scan and `.3.g.iii.a`'s column reader skip the table.
  - **Exemption.** SemanticIR exempts from `.5.xiv.a`'s collision rule exactly the enums named by a marking sentence
    that cites a table; the same words in prose exempt nothing.
  - **Measured.** Dry-run of the 31 chains: 30 identical. AXI adds 20 statements, the 10 earlier `ARCACHE`
    statements unchanged: `ARCACHE`'s two bracketed members, `AWCACHE`'s 12, 4 alternatives and 2 marks. After a
    stage-by-stage rebuild, `manager.isf` carries `ARCACHE` with 12 members (it had 6) and `AWCACHE` with 12, both
    typed 4 bits. FSMGen strict reports 0 diagnostics and no stderr. AXI's symbol count goes from 46 to 47.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.c.i` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — A4.4's statements per reader before and after; AXI's `ARCACHE`/`AWCACHE` in
  SemanticIR and `manager.isf`; every chain's EvidenceIR.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: no reader took a channel code table
  whole, and the bracketed cells parsed no code; `crates/specforge/src/ir/semantic.rs` `build_symbol_definitions`
  dropped every shared code. New `read_channel_code_tables`, `channel_code_and_alternative` and `shared_code_enums`.
  RED 5/5, each file restored byte-identically: signals of no one collective read as channels, no shared-code mark,
  the bracketed alternative dropped, the mark exempting nothing, a mark with no table exempting.
- [x] **ADDRESSED (verified)** — AXI rebuilt stage by stage; FSMGen strict clean.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `5e0650255`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean once the channel member tuple got a `type` alias.
  `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,919 passed. `flow_census.json`
  rolled (+2 functions, +23 decision sites, +26 helper edges, +2 semantic macros); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — the document's own collectives, column-title shape and bracket notation; no
  document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (the collision rule's one exemption); KM card
  `a-channel-code-column-is-an-aliasing-enum` (new) and `a-signal-titled-code-column-determining-its-member-is-an-enum`
  amended. No production rule deleted or replaced: `.5.xiv.a` keeps its default.

- ID: `EXTRACTION-REPRESENTATION-GAPS.57`
  Status: `done` (`2026-10-22`; DESIGN; found by `.3.g.iii`; opened `.57.a`)
  Goal: **a table whose every row the converter put in its header has one header row.** 137 tables in 16 documents
  have more than one header row and no body row (the Cortex-A76 guide 37, AXI 26, USB 3.2 23, ADIv6 14), so every
  table reader sees no data. AXI *A4.7* is one. **Reproduce:** `generated/source_ir/ihi0022…/source_ir.json`
  `table_0061` (7 header rows, 0 body rows). Acceptance: census the 137 by shape (front matter, data tables, true
  multi-row headers), decide whether SourceIR normalizes them or each reader does, from what the converter states,
  and measure every chain.
  **Measured `2026-10-22`** (31 chains). The 137 tables are all on the retained chains, but the premise that every
  table reader sees no data is wrong. EvidenceIR reads every table through `source_ir_with_row_header_rows_in_body`
  (`.3.g`), which moves into the body each header row after the first whose only flagged cell is its first.
  - **Fully lifted (109).** Every data row is moved, A4.7 among them: `.3.g.iii.a` and `.3.g.iii.b.i` read it.
  - **Partly lifted (22).** These have a genuine two- or three-row header over data rows. The data rows are lifted;
    the rows kept are header rows (*Opcode | Transfer | Fetch* under *MTE_Support = Basic*, *MIN | MAX* under
    *LOWER LIMIT* under *TRANSMITTER*).
  - **Not lifted (6).** OpenCAPI DLL 2, I2S 1 and USB 3.2 3. Their data rows carry two flagged row-label cells
    (*Transmission order | 8 consecutive bytes | P | P*), or a converter flag mid-row (USB *b6~4*).

  **Decided.**
  - **SourceIR keeps what the converter states.** The lifting stays the evidence stage's local reading, as it is
    today: every evidence reader sees the lifted view, and the converter's header flags are untouched.
  - **A leading run of row labels.** A header row after the first lifts when its flagged cells are a leading run of
    row labels with a non-empty first cell, followed by at least one unflagged cell and no flagged cell after it.
    This lifts OpenCAPI's *8-1* (10 rows) and *8-2* (1), I2S's timing table (9) and USB's *7-14* (4) and packet
    figure (2). A row under a lost span (empty first cell) and a row with a flag mid-row stay header rows. The
    measurement must show the 131 lifted tables unchanged.
  - **Upper header rows.** A reader takes a table's last header row as its columns, so an upper row's qualifier is
    lost (*TRANSMITTER* or *RECEIVER*, *MTE_Support = Basic*). `.57.a` measures how many lifted tables keep more than
    one header row and routes each to the leaf owning its family's reader.

- **`.19` closes** (`2026-10-22`; the leaf's record is in `representation-leaves`).
  FSMGen answered on `2026-09-26`: an interface-port clause it does not recognise is a validation defect, to be
  refused through its shared parser. The packet recorded the answer then, but the leaf stayed open. **Verified
  `2026-10-22`** on a read-only export of upstream `c51e22712` (known-good-checked first: AHB's `manager.isf` checks
  with no diagnostic and no stderr on both installations). `(output HPROT (width HPROT_WIDTH) (frobnicate 7))` is
  refused with *interface port 'HPROT' has unsupported 'frobnicate' option*, and so is a `(fields …)` clause on a
  port. The pin `54ac1c22` still accepts both with no diagnostic, and its semantic JSON equals the clause-free port's.
  The fix reaches SPECFORGE with `FSMGEN-REFRESH-INTEGRATE-10`, whose acceptance now re-checks the probe and whose
  `.1` now owns the ledger record. SPECFORGE still emits only `(width …)` on a port: none of the 19 written `.isf`
  files or the 31 live `source_text`s draws that refusal from the tip.

- **`.3.c.iv`'s measurement and decision** (`2026-10-22`; the leaf's record is in `census-leaves`).
  **Measured `2026-10-22`** (31 chains). Per-bit tables take two shapes.
  - **A column per bit.** AHB *3-5* gives `HPROT[3] Modifiable` … `HPROT[0] Data/Opcode` a column each. Each row sets
    one bit, marks the others `-` and names the meaning (`HPROT[0]` `1`: *Data access*). AHB *3-7* keys a memory
    type by `HPROT[6:2]`, with `0 or 1` alternatives. AHB *3-2* is not this shape: its bit columns are all `0`/`1`,
    and `.3.g.iii.a` already reads them as `HSIZE`'s concatenated code. ADIv6's `TDI`/`TDO` grids are layouts.
  - **A row per bit.** AHB *3-6* (*Bit | Name | Description*, 7 bits) and APB's `PPROT` table (3 bits, each with a
    *LOW indicates … HIGH indicates …* comment) describe each bit as a named sub-field of its signal.

  Neither reaches a record today. `.3.g.iii.b.i`'s reader needs two different names, and SemanticIR's collision rule
  drops a per-bit enum whose values repeat across bits.
  **Decided.**
  - **A column per bit is a value combination.** Its keys are distinct bits of one signal, and a title may name the
    bit after its slice (`HPROT[3] Modifiable`; the title is kept as written). The reader takes it only when a bit
    column holds a don't-care mark or an alternative, so the code does not concatenate and `.3.g.iii.a`'s determining
    reading is not doubled. Carriage to IntentIR and the ISF residual follow from `.3.g.iii.b.ii`. Nothing is lowered:
    a per-bit meaning has no consumer, and `.3.g.iii.b.iii`'s reasons hold. CODE: `.3.c.iv.a`.
  - **A row per bit is a signal's bit sub-field.** It has a name, a bit, a description and, where stated, a meaning
    per level. No record holds one: `message_field_records` are packet fields, and register fields belong to
    registers. Choosing that carrier is `.3.c.iv.b`'s.
