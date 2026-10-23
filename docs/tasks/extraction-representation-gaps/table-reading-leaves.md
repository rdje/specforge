# EXTRACTION-REPRESENTATION-GAPS — table reading leaves

- Part ID: `table-reading-leaves`
- State: `active`

Open ERG leaves continue here, after `layout-and-binding-leaves` reached its size warning (`2026-10-22`): the ways a
specification's tables are read (layouts per message kind, sub-fields, notes, cross-tabs, swapped columns), and every
later ERG leaf. `.3.c.iv.b.iii`, `.3.c.v.v`, `.65`, `.67`, `.68`, `.3.c.viii.a`, `.61`, `.3.c.iv.c` and `.60` moved
here whole.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.iii`
  Status: `open` (`2026-10-22`; REQUEST, waits for a consumer; opened by `.3.c.iv.b.ii`)
  Goal: **a port's sub-field layout reaches FSMGen once something executable names a sub-field.** FSMGen ladder F29 is
  *anticipated*: no `.isf` port takes a field, and ADR 0033 §4 files a request only when executable intent needs one.
  Trigger: a rule, guard or drive SPECFORGE lowers names a sub-field (`HPROT` *Privileged*, `AxMPAM` *PARTID*).
  Acceptance: on the trigger, refresh the drafted bundle (`.project-data/tmp/req-port-fields/packet`, reproduced on
  the pin `54ac1c22` and upstream `c51e22712`) against the current pin and upstream, sign it off as an
  `FSMGEN-REQUESTS` leaf and its ledger record, and move F29 to *requested*.
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v.v`
  Status: `open` (`2026-10-22`; CODE, waits for `KG-ISF-COMPLETENESS.2a.vii.c.vi` and an FSMGen pin with
  actor-level assertions; opened by `.3.c.v.iv`; `.62`, `.63` and `.64` done)
  Goal: **a legality matrix lowers to an always-on assertion.** Acceptance: once the opcode decode, the property
  parameter and actor-level assertions exist, lower AXI *A12.11*/*A12.13* as `.3.c.v.iv` decided (column guard: the
  property value and the tag operation's code; consequent: the marked opcodes, decoded, `*` keys expanded), FSMGen
  strict-clean, and measure every chain's legality matrices; repoint the adapter's `isf_protocol_matrix_table_*`
  residual text, which names `.3.c.v.iv`, to the lowering. Since `.62` the decode is a record whose domain values
  are labels (`.63` binds them), and a marked cell may carry a note that narrows it (`Y 2`: *Not Exclusive write*,
  `.64`); the lowering states both.
- ID: `EXTRACTION-REPRESENTATION-GAPS.65`
  Status: `open` (`2026-10-22`; CODE; opened by `.64`)
  Goal: **a note printed as a numbered list keeps its number.** **Reproduce:** I2C *Table 3* marks `Target [1]`, `O
  [2]` and `O [3]`, and prints the notes after its continuation as three `list_item` elements on `page_0008` (*Also
  refers to a controller acting as a target.*, *Clock stretching is a feature of some targets …*, *'Bit banging' …
  should consider a START byte.*) with no number, so no mark reaches its note. Acceptance: find where the number is
  lost (the conversion or SourceIR's list reading), keep it so the note is read as note 1-3, never bind by position
  alone, and measure every chain's numbered notes.
- ID: `EXTRACTION-REPRESENTATION-GAPS.67`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.vi`)
  Goal: **a table listing the signals a rule names is the rule's operands.** **Reproduce:** AXI *Table A6.5 Signals
  that should be the same in an exclusive sequence* lists 25 signals (`AxID`, `AxADDR`, … `AxMMUSSIDV`) in a grid of
  five columns; the rule (an exclusive read and its write *should* carry equal values) is in the caption and prose,
  and `.3.c` disposed of the table as navigation. Acceptance: census tables whose cells are all declared names under
  a caption stating a rule, decide the carrier (the rule's operand set) and measure every chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.68`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.vi`)
  Goal: **a cross-tab whose cells are prose is read.** **Reproduce:** USB 3.2 *Table 8-8 Port Type Selection Matrix*
  answers, per pair of port capabilities (`Upstream Only`, `Downstream Only`, `Both`), which port is downstream in a
  sentence (*Port 2 is the downstream port*); `.3.c.v` set such cross-tabs aside as no matrix of marks (*7-14* too).
  Acceptance: census cross-tabs of prose or values, decide the record (keys and a statement per cell) and measure
  every chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.viii.a`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.viii`)
  Goal: **a table whose columns the converter swapped is read or refused as such.** **Reproduce:** CoreSight TMC
  *Table 2-1 Memory interface signals* puts each description under *Signal* and each name under *Description*
  (`Chip enable, active LOW. | MEMCEN`), its first and last rows garbled (`CLK input clock for the design | The`);
  ADIv6 *Table B2-7* puts each register bit under *Signal* and each wire under *Programmers' model*, its last row
  rotated. Acceptance: census tables whose header-named name column holds no names while another column holds only
  names, decide whether the pairs are read (TMC's rows state no direction or width; *B2-7* is a wire-to-register-bit
  map like `.41.e`'s), and measure every chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.69`
  Status: `open` (`2026-10-22`; DESIGN; opened by `FSMGEN-REQUESTS.9.a`)
  Goal: **a protocol whose only clock is a bus signal grounds its actor's clock, or says why not.** **Reproduce:**
  RISC-V debug, ADIv6, I2C and I2S are blocked by nothing but *no source-grounded system clock/reset contract*; each
  documents a bus clock (`TCK`, `SWCLK`, `SCL`, `SCK`) but no system clock, so the adapter writes
  `(clock __specforge_unresolved_clock)`. A bus clock is an input on one side and a generated output on the other (an
  I2C controller drives `SCL` from a clock the specification does not name). Acceptance: census the 26 blocked chains'
  clock evidence, decide per role whether a bus clock is the actor's `.isf` clock (and what reset then applies, given
  `FSMGEN-REQUESTS.9.a`'s refusal of an invented reset), and measure every chain's adapter.
- ID: `EXTRACTION-REPRESENTATION-GAPS.61`
  Status: `done` (`2026-10-23`; DESIGN; opened by `.3.c.v`; opened `.61.a`, `.61.b`, `.61.c`, `.70`)
  Goal: **an encoding member enabled by a property says so.** **Reproduce:** AXI's untitled `AWCMO` table gives
  `CleanSharedPersist` the enabling property `Persist_CMO` and `CleanInvalidPoPA` `RME_Support`; the members are read
  (`Enum AWCMO …`) and the property is not. Acceptance: census encoding tables with such a column, decide the carrier
  (a member's enabling property, joined to the property's own record), and measure every chain.
  **Measured `2026-10-23`** (31 chains; `.project-data/tmp/erg-61/census.py`, `heads.py`, `count.py`). Of the 48
  tables that mint an enum symbol or a value combination, five AXI tables (eight fragments) give each row an enabling
  property in a column headed `Enable property`, `Enable` or `Property`. Each cell is `-`, one property, or properties
  joined by one connective:
  - two enum tables: *A9.4* `AWSNOOP` (3 members, each gated: `CMO_On_Write`, `Write_Plus_CMO` twice) and the untitled
    `AWCMO` table (4 of its 6 members: `Persist_CMO` twice, `RME_Support`, `Storage_CMO`);
  - three combination tables, whose cells `value_combinations` already keeps as written among `meanings`: *A7.3*
    (`AWSNOOP` x `AWDOMAIN`, 25 rows: 15 one property, 5 `and`, 1 `or`, 4 `-`), *A7.4* (`ARSNOOP` x `ARDOMAIN`, 19
    rows: 7 one, 5 `and`, 7 `-`) and *A4.7* (physical address spaces, 6 rows: 4 one, 2 `-`).
  The cells name 22 properties, each with its own *`<Name>` property* table, and every one boolean, so none is a record
  yet: SemanticIR mints `param` symbols for integer properties only, and a boolean property is
  `KG-ISF-COMPLETENESS.2a.vii.c.vi`'s to read. The other matches are other shapes, each owned: a signal's presence
  property (APB, AHB; `signal_presence_records.presence_condition`, joined by `.2a.vii.c.vi`), a parity check's enable
  (`check_signal_records.check_enable`) and CHI C2C's register property bits. ADIv6 *C2-3*'s `Support required?` on
  `CSW.Size`'s members is a conformance requirement, not a property, and nothing reads it (`.70`).
  **Decided.**
  - **The column.** In a table that mints an enum or a value combination, a column other than the code and name
    columns whose title has the word *enable*, *enabled* or *property* (document grammar, as the presence matrix's
    *property* column, ADR 0006), and whose every cell is `-`, empty, one identifier, or identifiers joined by one
    connective (`and` or `or`). Any other cell refuses the column.
  - **The carrier.** One enabling condition, kept as the cell writes it, with its property names and its connective
    (`all` for `and` or a single name, `any` for `or`). `-` or an empty cell enables unconditionally, so it states
    nothing. An enum member exists in EvidenceIR only as the reader's statement, so the reader states the condition
    the same way (`Enum AWCMO CLEANSHAREDPERSIST is enabled by Persist_CMO.`, citing its table), as it states `also
    takes` and `shares a code between members`. SemanticIR reads it into `SymbolEnumMemberRecord.enabled_by`. A
    combination row gains `ValueCombinationRow.enabled_by` beside its `meanings`, which stay as written. IntentIR
    carries both with the symbols and combinations (`.61.a`).
  - **Not lowered.** An `.isf` enum declares its members unconditionally and has no boolean property; an optional
    port or member is FSMGen's half of `.2a.vii.c.vi`. The adapter keeps each gated member declared, since the full set
    is what the interface carries when every property is True, and states one residual per gated enum naming its
    members and conditions (`.61.b`). ADR 0033 §4 files no FSMGen request until a rule needs a gated member.
  - **The join.** Each named property joins the property's own record once boolean properties are records (`.61.c`,
    after `.2a.vii.c.vi`). Until then the names stay as written, never resolved by spelling alone.
- ID: `EXTRACTION-REPRESENTATION-GAPS.61.a`
  Status: `open` (`2026-10-23`; CODE; opened by `.61`)
  Goal: **an encoding member's enabling property is read.** Acceptance: read `.61`'s column into the enum reader's
  `is enabled by` statements and `SymbolEnumMemberRecord.enabled_by` (*A9.4* `AWSNOOP` 3 members, `AWCMO` 4) and into
  `ValueCombinationRow.enabled_by` (*A7.3* 21 rows, *A7.4* 12, *A4.7* 4), carry both to IntentIR, and measure every
  chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.61.b`
  Status: `open` (`2026-10-23`; CODE; opened by `.61`)
  Goal: **the adapter names each enum member a property enables.** Acceptance: state one residual per enum with a
  gated member, naming each member and its condition; the `.isf` enum keeps every member; measure every chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.61.c`
  Status: `open` (`2026-10-23`; CODE; opened by `.61`; waits for `KG-ISF-COMPLETENESS.2a.vii.c.vi`)
  Goal: **a member's enabling property joins the property's own record.** Acceptance: once boolean properties are
  records, join each name in an `enabled_by` to its record, leave a name no record declares unjoined, and measure AXI's
  22 properties.
- ID: `EXTRACTION-REPRESENTATION-GAPS.70`
  Status: `open` (`2026-10-23`; DESIGN; opened by `.61`)
  Goal: **an encoding member's support requirement is read.** **Reproduce:** ADIv6 *C2-3* (`Size value, CSW.Size |
  Memory access size | Support required?`) states `Yes a` for `Word` and `No` for the other sizes; the `Size` enum
  keeps the codes and drops the column. Acceptance: census encoding tables with a support-requirement column, decide
  the carrier (and how the footnote mark joins its note), and measure every chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.c`
  Status: `done` (`2026-10-22`; DESIGN; opened by `.3.c.iv.b`; opened `.3.c.iv.c.i`, `.3.c.iv.c.ii`)
  Goal: **a signal laid out per message kind is read.** **Reproduce:** AXI-L *A15.20*-*A15.22* and their continuations
  give each `ACADDR` bit a constant or a source field (`VA[56]`) per TLBI/BPI/VICI/PICI message part, and no reader
  takes them. Acceptance: decide the record (a layout per message kind, with constant and source-field bits), how it
  joins the message kinds' encodings and `DOC-INTENT-TAXONOMY.4b`'s layouts, and measure every chain.
  **Measured `2026-10-22`** (31 chains; `.project-data/tmp/erg-3civc/census.py`): 3 tables and their 3 continuations,
  all AXI (*A15.20* TLBI, *A15.21* BPI and Sync, *A15.22* VICI and PICI), have a first column of signal slices
  (`ACADDR[51]` down, then `ACVMIDEXT[3]` … `ACVMIDEXT[0]` in the continuations), one column per message part, and
  cells that are a code literal (`0b0`) or a slice of a source field (`VA[56]`, `ASID[4]`, `PA[51]`, `VI[17]`): 145
  and 284 in the three head tables. No other chain has the shape.
  **Decided.**
  - **The record.** A new EvidenceIR surface, `message_signal_layouts`: one record per table and its continuations,
    with the caption, the message columns as written (`TLBI 1st of 2-part`), and per row the signal slice and one
    source per message: a constant (`0b0`), a source field's slice (`VA[56]`) or anything else as written. It is the
    shape `signal_layouts` refuses (another column holding slices or literals), so the two never take one table.
  - **The joins.** A message column names its message kind by its leading words (`TLBI`, `BPI`, `VICI`, `PICI`,
    `Sync`); the kind's fixed fields are *A15.7*-*A15.16* (`Fixed field values for a TLBI message`), which
    `DOC-INTENT-TAXONOMY.4b` owns as message layouts. A source field (`VA`, `ASID`, `VMID`, `PA`, `VI`) is a message
    field the record names as written; resolving it waits for `.4b`'s message fields.
  - **Not lowered.** Like `signal_layouts`: carried to IntentIR typed, one adapter residual per layout, since an `.isf`
    port takes no per-message bit mapping (FSMGen ladder F29, anticipated).
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.c.i`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.iv.c`; checklist below)
  Goal: **a table giving a signal's bits per message kind is EvidenceIR's `message_signal_layouts`.** Acceptance: read
  the record `.3.c.iv.c` decided (a first column of declared signals' slices, two or more message columns, every other
  cell a literal, a slice or empty; continuations joined by caption), keep every cell as written, register the field
  (rule fields, inventories), re-seal what the ruleset hash moves, and measure every chain.
  **Done `2026-10-22`.**
  - **Record.** EvidenceIR `message_signal_layouts` (`MessageSignalLayoutRecord`: the tables, caption and message
    titles; `MessageSignalBit` per row: the signal, its slice, and one `MessageBitSource` per message). A source is a
    `constant` (`0b0`), with the field it fixes when the cell names one (`0b0 (DVMType[2])`, `0b1 (Addr)`); a `field`,
    sliced (`VA[56]`) or a one-bit field named alone (`VMIDV`, `Completion`); or `unstated` (empty, `-`).
  - **Reader.** `read_message_signal_layouts` runs in each convergence pass beside `read_signal_layouts`. Every body
    row starts with a declared signal's slice and every other cell takes one of those forms, at least one a field
    slice; a continuation joins by caption and header. Any other cell, or a slice of no declared signal, refuses the
    table. The manifest entry is recorded only for a document with a candidate.
  - **Measured.** Dry-run of the 31 chains: 30 identical. AXI's three records each hold 56 rows (52 `ACADDR`, 4
    `ACVMIDEXT`) across a head and its continuation; the continuations' `0b0 (DVMType[2])` and `VMIDV` cells were the
    forms the first reading refused.
  - **Re-sealed.** The new field moved the EvidenceIR ruleset hash. AXI rebuilt stage by stage: only EvidenceIR moved
    (`message_signal_layouts` and its manifest entry); 111 residuals and `manager.isf` unchanged.
    `rebuild_stage_cascade.sh --write` then re-sealed all 31 chains, content-identical at every stage.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.c.i` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — `.3.c.iv.c`'s census and a per-cell reading of the continuations (18 non-fitting
  forms, all a constant naming its field or a one-bit field's name).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` had no reader for a table keyed by signal
  slices with a column per message; `read_signal_layouts` refuses it by design. RED 4/4, the file restored
  byte-identically: a continuation not joined, prose taken as a field, an undeclared signal read, a constant naming
  its field refused; each fails `a_table_giving_signal_bits_per_message_is_a_message_signal_layout`.
- [x] **ADDRESSED (verified)** — AXI's three records above, in the dry-run and the rebuilt chain.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current after the cascade. WIRE-BASED-100: all 11 seed
  reports byte-identical to `0c0a4533f`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo
  clippy --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,935 passed; the proof test counts 52 EvidenceIR rule fields and the
  inventory 212 rules (`message_signal_layouts` in both production-genericity inventories). `flow_census.json` rolled
  (+2 functions, +19 decision sites, +54 helper edges, +2 macros) and its boundary moved for the new producer (rule
  roots 136, grammar declassifiers 77).
- [x] **GENERICITY (ADR 0006)** — cell shapes (literal, slice, identifier) and the document's own declared signals;
  no message or field vocabulary.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A signal laid out per message kind*); KM card
  `a-table-naming-a-signals-bits-is-its-layout`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.c.ii`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.iv.c`; checklist below)
  Goal: **a per-message layout reaches IntentIR, and the adapter names it.** Acceptance: carry
  `message_signal_layouts` through SemanticIR and IntentIR unchanged, state one `isf_protocol_message_signal_layout_*`
  residual per record naming its message kinds, and measure every chain's adapter.
  **Done `2026-10-22`.**
  - **Carried.** SemanticIR (`semantic.carried_evidence`) and IntentIR (`intent.carried_semantics`) carry
    `message_signal_layouts` unchanged. `cited_table_ids` gathers its tables, as `.3.c.vi.a` requires of every carried
    record, so its fragments are no longer captured-table residuals.
  - **Residual.** The adapter states one `isf_protocol_message_signal_layout_*` residual per record, naming its signals
    and message kinds (`ACADDR, ACVMIDEXT over TLBI 1-part; TLBI 1st of 2-part; …`) and citing its tables: no `.isf`
    port takes a per-message bit mapping (FSMGen ladder F29, anticipated).
  - **Measured.** AXI rebuilt stage by stage: `message_signal_layouts` and `captured_region_residuals` moved in
    SemanticIR and IntentIR; A15.20-A15.22's six fragments leave the table residuals (140 -> 134); the adapter states
    114 residuals (111, plus the three); `manager.isf` is byte-identical. The rule fields moved the SemanticIR and
    IntentIR ruleset hashes, so `rebuild_stage_cascade.sh --write` re-sealed all 31 chains, content-identical.
  - **Lockstep, and an earlier gap.** The book's carried-collection lists (`pipeline/semanticir.md`,
    `pipeline/intentir.md`) still named six collections, missing `signal_layouts` and `matrix_tables` that
    `.3.c.iv.b.ii` and `.3.c.v.ii` carried; they now name all nine.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.c.ii` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — `.3.c.iv.c.i`'s three AXI records stopped at EvidenceIR, and their six fragments were
  table residuals.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs` and `intent.rs` did not carry the field,
  and `crates/specforge/src/ir/adapters.rs` stated nothing for it. RED 4/4, each file restored byte-identically:
  SemanticIR dropping the layouts, IntentIR dropping them, the adapter stating no residual, the record citing no
  table; each fails `projects_protocol_surfaces_exactly_across_all_three_stages`,
  `isf_adapter_residualizes_a_message_signal_layout_naming_its_messages` or
  `cited_table_ids_gathers_every_declared_table_provenance_surface`.
- [x] **ADDRESSED (verified)** — AXI's carried records, three residuals and six cited fragments above.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current after the cascade. WIRE-BASED-100: all 11 seed
  reports byte-identical to `4d48e580e`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo
  clippy --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,936 passed once the field joined both carried rows of
  `claim_family_inventory.tsv` (the first run failed the inventory bound, 212 against 214); proof fields SemanticIR 66,
  IntentIR 65 (45 carried), inventory 214 rules. `flow_census.json` rolled (+61 decision sites, +45 helper edges, +1
  macro); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — carriage and a residual per record; no document, table or message vocabulary.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md`, `pipeline/semanticir.md`, `pipeline/intentir.md`; KM card
  `a-table-naming-a-signals-bits-is-its-layout`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.56.c`
  Status: `done` (`2026-10-22`; DOC; opened by `.3.c.iv.c`'s part rollover)
  Goal: **`layout-and-binding-leaves`, every leaf closed, is archived.** Its open leaves moved whole to
  `table-reading-leaves` on `2026-10-22`. Acceptance: archive it byte-exact as `.56.a` did (sealed at the rollover
  commit, registry and census entries, repointed card evidence paths), and measure the live totals.
  **Done `2026-10-22`.** Moved copy -> verify -> use -> delete to
  `docs/archive/tasks/extraction-representation-gaps/layout-and-binding-leaves.md`, sealed at `e3d023d43` (the rollover
  commit, its last write; blob `159bb866e`, sha256 `3c339d34…` at the seal, the copy and the live file alike). Both
  contracts mark it `archived` with `archived_from`, `sealed_commit` and `git_blob`; `erg_task_evidence_parts` drops
  it (11 targets) and `erg_task_evidence_archived_parts` takes it (9 files; per-file bounds unchanged, totals 9x). Two
  KM cards' evidence paths repointed. Its leaves stay routed to it, every one closed.
- ID: `EXTRACTION-REPRESENTATION-GAPS.60`
  Status: `done` (`2026-10-22`; DESIGN; opened by `.3.c.iv.b`; opened `.60.a`)
  Goal: **a register field's sub-field table types the field's sub-fields.** **Reproduce:** ADIv6 *B3-10* lays
  `IDCODE`'s `DESIGNER` (`bits[11:1]`) out as *Continuation code* `Bits[11:8]` and *Identity code* `Bits[7:1]`, with
  ARM's values; `RegisterFieldRecord` has no sub-fields. Acceptance: decide the carrier (as `MessageSubField` did for
  message fields, `.55.b.iv.b.i`), how the register's bit positions map onto the field's, and measure every chain.
  **Measured `2026-10-22`** (31 chains; `.project-data/tmp/erg-60/census.py`: a column of bit ranges titled by, or
  captioned with, a declared register, every range inside one field and none the whole field). One table qualifies:
  ADIv6 *B3-10* (`JEP106 field | Width (bits) | Bits in IDCODE | ARM registered value`), `Continuation code`
  `Bits[11:8]` and `Identity code` `Bits[7:1]` inside `IDCODE.DESIGNER` (`bits[11:1]`).
  **Decided.**
  - **The carrier.** `RegisterFieldRecord.sub_fields`, one `RegisterSubField` per row: its name, its bits as the
    register numbers them (the table states *Bits in IDCODE*; field-relative positions follow from the field's low
    bit), and every other cell as written (`0b0100 ( 0x4 )`), with the table cited. It parallels `MessageSubField`
    (`.55.b.iv.b.i`), whose rows state field-relative bits.
  - **The shape.** A bit-range column whose title or caption names a declared register; every range strictly inside
    one field of it; a name column of distinct cells; a width column, when present, restating each range's width, as
    `signal_layouts` requires. A range outside every field, or covering a whole field, refuses the table.
  - **Not lowered.** Register records already cross SemanticIR and IntentIR whole, so the sub-fields travel with
    them; `.isf` storage fields are one level, and no rule names a sub-field, so they stay typed (ADR 0033 §6).
- ID: `EXTRACTION-REPRESENTATION-GAPS.60.a`
  Status: `done` (`2026-10-23`; CODE; opened by `.60`; checklist below)
  Goal: **a register field's sub-field table is read into its sub-fields.** Acceptance: read `.60`'s shape into
  `RegisterFieldRecord.sub_fields` (ADIv6 *B3-10* into `IDCODE.DESIGNER`), keep every cell as written, and measure
  every chain's register records.
  **Done `2026-10-23`.**
  - **The reader.** `fill_field_sub_fields` runs last in the register pipeline (after the signal-binding readers). A
    table with at least two rows has a column of bit ranges (`sub_field_bit_range`: `Bits[11:8]`, `Bit[3]`, `[7:1]`,
    `11:8`; a bare number is a width, not a range) whose title, or else the table's caption, names a declared register
    as a word. The ranges are disjoint and all sit inside one field of that register, so none is the whole field; a
    column of numbers restates each range's width, or the table is refused. The first other non-numeric column of
    distinct, non-empty cells names each sub-field, and every other cell is kept with its column title
    (`RegisterSubField`, `RegisterSubFieldCell`). A field already laid out keeps its first table.
  - **Two rules `.60` left implicit.** Disjointness, as `signal_layouts` requires: overlapping ranges are no layout,
    and with two or more disjoint ranges none can be the whole field. And the caption: `.60`'s census matched the
    title or the caption, so the reader does too, title first.
  - **Measured.** A dry run of all 31 chains: ADIv6's `IDCODE.DESIGNER` (`bits[11:1]`) gains *Continuation code*
    `11:8` and *Identity code* `7:1` from `table_0050`, with `Width (bits)` and `ARM registered value` (`0b0100 ( 0x4
    )`, `0b0111011 ( 0x3B )`); the other 30 chains' EvidenceIR is identical (`sub_fields` is skipped when empty).
    ADIv6 rebuilt stage by stage: `register_records` moved in EvidenceIR, SemanticIR and IntentIR (records cross
    whole); the adapter's content is identical (only its proof fields moved), so `.isf` does not change.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.60.a` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — `.60`'s census: one qualifying table, ADIv6 *B3-10*, and `RegisterFieldRecord` had no
  sub-fields.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source.rs` had no sub-field carrier and
  `crates/specforge/src/ir/evidence.rs` no reader. RED 5/5, the file restored byte-identically: a bare number read as
  a range, overlapping ranges allowed, the caption ignored, a range's low bound ignored, the width column unchecked;
  each fails `a_register_fields_sub_field_table_lays_it_out`. The pipeline call is measured by the ADIv6 rebuild.
- [x] **ADDRESSED (verified)** — `IDCODE.DESIGNER`'s two sub-fields above, carried to IntentIR.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current (ADIv6 rebuilt; no new top-level field, so no
  cascade). WIRE-BASED-100: all 11 seed reports byte-identical to `.3.c.iv.c.ii`'s (temp paths masked). `kg-bench`
  158/158. `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings` clean. `cargo test
  --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,937 passed. `flow_census.json` rolled (+2
  functions, +36 decision sites, +12 helper edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — bit ranges, a register named by a column title or caption, field containment and
  width restatement; no document, register or vendor vocabulary.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (*A field laid out in sub-fields*), `pipeline/isf-adapter.md`
  (sub-fields are not lowered); KM card `a-register-fields-sub-field-table-is-its-layout`.
