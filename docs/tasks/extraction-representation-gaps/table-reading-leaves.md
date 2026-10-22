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
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.v`)
  Goal: **an encoding member enabled by a property says so.** **Reproduce:** AXI's untitled `AWCMO` table gives
  `CleanSharedPersist` the enabling property `Persist_CMO` and `CleanInvalidPoPA` `RME_Support`; the members are read
  (`Enum AWCMO …`) and the property is not. Acceptance: census encoding tables with such a column, decide the carrier
  (a member's enabling property, joined to the property's own record), and measure every chain.
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
  Status: `open` (`2026-10-22`; CODE; opened by `.3.c.iv.c`)
  Goal: **a per-message layout reaches IntentIR, and the adapter names it.** Acceptance: carry
  `message_signal_layouts` through SemanticIR and IntentIR unchanged, state one `isf_protocol_message_signal_layout_*`
  residual per record naming its message kinds, and measure every chain's adapter.
- ID: `EXTRACTION-REPRESENTATION-GAPS.56.c`
  Status: `open` (`2026-10-22`; DOC; opened by `.3.c.iv.c`'s part rollover)
  Goal: **`layout-and-binding-leaves`, every leaf closed, is archived.** Its open leaves moved whole to
  `table-reading-leaves` on `2026-10-22`. Acceptance: archive it byte-exact as `.56.a` did (sealed at the rollover
  commit, registry and census entries, repointed card evidence paths), and measure the live totals.
- ID: `EXTRACTION-REPRESENTATION-GAPS.60`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.iv.b`)
  Goal: **a register field's sub-field table types the field's sub-fields.** **Reproduce:** ADIv6 *B3-10* lays
  `IDCODE`'s `DESIGNER` (`bits[11:1]`) out as *Continuation code* `Bits[11:8]` and *Identity code* `Bits[7:1]`, with
  ARM's values; `RegisterFieldRecord` has no sub-fields. Acceptance: decide the carrier (as `MessageSubField` did for
  message fields, `.55.b.iv.b.i`), how the register's bit positions map onto the field's, and measure every chain.
