# EXTRACTION-REPRESENTATION-GAPS — layout and binding leaves

- Part ID: `layout-and-binding-leaves`
- State: `active`

Open ERG leaves continue here, after `encoding-and-state-leaves` reached its rollover (`2026-10-22`): a signal's
sub-field layouts, a signal laid out per message kind, a register field's sub-fields, and every later ERG leaf.
`.3.c.iv.b.i`, `.3.c.iv.b.ii`, `.3.c.iv.c` and `.60` moved here whole.

## Post-migration records

- **`.3.c.v`'s measurement and decision** (`2026-10-22`; the leaf's record is in `census-leaves`).
  **Measured `2026-10-22`** (31 chains; `.project-data/tmp/erg-3cv/census2.py`). 51 tables in 13 documents hold two or
  more key columns of marks or modal words (`Y`, `-`, `X`, `Yes`, `N/A`, `M`, `O`, *Required*, *Prohibited*,
  *Allowed*, *Recommended*, *Permitted*, *On*, *May be off*). Two readers already take 17: `signal_presence_records`
  15 presence tables (AXI *B2.2*/*B2.3* 11, APB5 3, AHB 1), and the register reader 2 RISC-V debug access tables. The
  other 34 fall into these families (the fifteen-families census of `.3.b` counted 135 such tables among the untyped
  tables of all 78 stored documents; this one reads the 31 current chains' tables by shape):
  - **Legality matrices: 6.** An operation per row, a mode or state per column, the cell whether it is permitted: AXI
    *A12.11*/*A12.13* (*Legal tag operations*, opcode x `MTE_Support` x tag operation), *A9.20* (*Permitted accesses*,
    phase x domain x read/write), TMC *3-52*/*3-53* (*Permitted register writes/reads*, register x
    `TraceCaptEn`/`TMCReady`/`MODE`, 3 tables).
  - **Requirement matrices: 5.** A feature per row, a configuration per column, a modal word per cell: I2C *Table 3*
    (its caption defines `M`, `O` and `n/a`), OpenCAPI *1-5* (*Reset Types*) and its DVSEC applicability table, USB
    3.2 *10-2* (*VBUS Requirements*), ADIv6 *E2-1* (*Recommended ADI implementations*).
  - **Version and variant matrices: 6, and one legend.** An item per row, a version, issue or interface class per
    column: AXI *A15.2* (*DVM message versions*), *B2.4*'s last page, APB5's property-by-issue table, ADIv6 *B2-2* (DP
    registers by DP version, 2 tables), OpenCAPI *8-2* (version interoperability). AXI *B2.1* (*Key to signals table*)
    is the legend of the presence matrices: `YM` is mandatory on a manager and optional on a subordinate. The presence
    records keep each code as written and no reader binds it to the legend.
  - **Effect matrices: 11.** A parameter, register access or code per row, an event, state or property per column: USB
    3.2 *9-2* (*Preserved USB Suspend State Parameters*), *9-10* (*Device Parameters and Events*, 2 tables), *7-11*
    (*Error Types and Recovery*, 3 tables), USB4 CM *7-3* (*TMU Mode Parameters*), the Cortex-A76 guide's register
    read and write properties (3 tables), ADIv6 *B5-2* (*Activation codes*).
  - **Not matrices: 5.** AXI's untitled `AWCMO` table is an encoding whose *Enable property* column names the property
    that enables a member (`Persist_CMO`, `RME_Support`); its members are read, and the property is not: `.61`. AHB
    *6-1*/*6-2* (active byte lanes) and ADIv6 *C3-2* (command-byte bits) are layouts: `DOC-INTENT-TAXONOMY.4b`, as
    ADIv6 *C2-4* was routed. OpenCAPI *1-1* (*Changes to product which require re-testing*) is certification process,
    a non-contract.
  `.57.a` routed four more with the same two-way shape but other cells: USB 3.2 *7-14* (*Start-up PHY Capability
  Match*, the negotiated generation per pair), *8-8* (*Port Type Selection Matrix*, a sentence per pair), OpenCAPI
  *8-1* and *8-3* (version numbers, version compliance).
  **Decided.**
  - **One record for a two-way table.** An EvidenceIR `matrix_tables` record per table and its continuations: the row
    key columns' titles, each data column's key path (its header cells from the top header row down, as written, a
    spanned title repeated), and per row the key cells and every cell as written. The cell class is recorded: *mark*,
    *modal*, *value* or *prose*, from the cells alone. A legend the document states is attached as written: a caption
    that defines its codes (I2C *Table 3*), or a legend table whose code column names the matrix's codes (AXI *B2.1*).
  - **Precedence.** A table another reader takes keeps that reader: presence matrices, register tables, value
    combinations (columns titled by signals), signal layouts and channel code tables. The encoding readers take a code
    column; a matrix has none.
  - **Dispositions.** Legality matrices constrain which operations a mode permits, so they bear on the executable
    contract: their lowering is decided apart, from a consumer (`.3.c.v.iv`). Requirement, version and effect matrices
    describe configuration, variants and software-visible state. They stay typed in IR and are stated as residuals
    (ADR 0033 §6).
  - **Legends and presence codes.** A presence record's code (`YM`) gains the legend's meaning per interface side when
    the document states a legend whose codes cover the presence codes (`.3.c.v.iii`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.i`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.iv.b`; checklist below)
  Goal: **a table naming a signal's bit sub-fields is typed.** **Reproduce:** AHB *3-6*, APB-E `table_0007`, AXI-L
  *A12.3*-*A12.6* reach EvidenceIR as raw rows only. Acceptance: the `signal_layouts` surface, its shape and level
  reading as `.3.c.iv.b` decided; every other family of the 23 tables untouched; measure the 31 chains; amend the
  book's *captured region* sentence.
  **Done `2026-10-22`.**
  - **Reader.** `read_signal_layouts` reads `.3.c.iv.b`'s shape (`parse_name_slice`: a slice of one name; the name
    column the first other column of distinct cells of at most five words, no number). A number column must restate
    each slice's width; another column holding a slice or a binary literal refuses the table. One `SignalLayoutRecord`
    per table and its continuations, with `SignalSubField` rows (slice, high and low bit, name, other cells as
    written) and, for a one-bit sub-field, `SubFieldLevel`s (`sub_field_levels`, `sentence_level`).
  - **Manifest.** `signal_layout_surface` records the `signal_layouts` entry only when a table had the slice column,
    so a document without one keeps its manifest (as `.3.b.iii.i` did for transition states).
  - **Measured.** Dry-run of the 31 chains: 27 identical. AHB gains 1 layout (*3-6*: 7 sub-fields, 12 levels;
    `HPROT[2]` none, under its condition), APB5 1 (3 sub-fields, 6 levels from the `LOWindicates`/`HIGH indicates`
    bullets), AXI 4 (*A12.3*-*A12.6*: `MPAM_NS`/`MPAM_SP`, `PARTID`, `PMG` onto `AxMPAM`), and ADIv6 counts its
    caption-less `HPROT` map as a refused candidate (manifest 1 produced, 0 kept). No statement moved.
  - **Rebuilt.** AHB, APB5, AXI and ADIv6 rebuilt stage by stage: every later stage and `.isf` text byte-identical,
    the adapter's residuals unchanged (AHB 41, APB5 41, AXI 83, ADIv6 30). The new field moved the EvidenceIR ruleset
    hash, so `rebuild_stage_cascade.sh --write` re-sealed all 31 chains, content-identical at every stage.
  - **Part rollover.** With this record `encoding-and-state-leaves` passed its 90% rollover (93.3% of its 65,536-byte
    target): new part `layout-and-binding-leaves` (contract and manifest part, leaf routes, index, route catalog,
    parts surface, routing paragraph, `MEMORY.md`); `.3.c.iv.b.i`, `.3.c.iv.b.ii`, `.3.c.iv.c` and `.60` moved whole,
    and the old part is at 84.5%.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.i` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — `.3.c.iv.b`'s census (`scan2.py`, 23 tables, five families); the persisted EvidenceIR
  held AHB *3-6*, APB5 `table_0007` and AXI *A12.3*-*A12.6* as raw row statements only.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` had no reader for a table keyed by one
  signal's slices; the value-combination and single-column readers key on titles or codes. RED 4/4, the file restored
  byte-identically: levels read under a condition, a number column not checked against the widths, overlapping slices
  kept, a level word needing its space; each fails `a_table_naming_a_signals_sub_fields_is_its_layout`.
- [x] **ADDRESSED (verified)** — the six layouts above in the dry-run and the rebuilt chains.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports (the three
  temporal ones among them; first written "and the temporal one", corrected by `.3.c.iv.b.ii`) byte-identical to
  `f5ceee7cb`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean (an `if let` collapsed). `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,922 passed; the proof test counts 50 EvidenceIR rule fields and the
  inventory 206 rules (`signal_layouts` added to `evidence.protocol_structure` in both inventories).
  `flow_census.json` rolled (+6 functions, +57 decision sites, +94 helper edges, +2 macros) and its boundary moved for
  the new producer (rule roots 134, grammar declassifiers 75).
- [x] **GENERICITY (ADR 0006)** — the shape is read from cells (a slice of a declared name, a number restating a
  width, a binary literal, a level sentence's opening words); no document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A signal's bit sub-fields*; the *captured region* sentence
  amended, as `.3.c.iv.b` named); KM card `a-table-naming-a-signals-bits-is-its-layout`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.iv.b`; checklist below)
  Goal: **a signal's sub-field layout reaches IntentIR, and the adapter names it.** Acceptance: carry `signal_layouts`
  through SemanticIR to IntentIR; an adapter residual per layout, naming any sub-field past its width parameter's
  default; `.isf` texts unchanged and FSMGen-strict-clean; rebuild what moves. Then re-evaluate ADR 0033 §4 for
  `docs/gaps/fsmgen-ladder.md` F29: file the drafted request (`.project-data/tmp/req-port-fields/packet`) only if an
  executable consumer needs the layout.
  **Done `2026-10-22`.**
  - **Carried.** `signal_layouts` is a field of SemanticIR (`semantic.carried_evidence`) and IntentIR
    (`intent.carried_semantics`), cloned as it is; both claim-family inventories name it.
  - **Residual.** `signal_layout_residuals` states one `isf_protocol_signal_layout_*` packet per layout: no `.isf`
    port form (ladder F29), and per port the layout names (the signal, or each member of its collective) every
    sub-field whose high bit reaches the port's elaborated width (`IsfIr::signal_width`), with the parameter's
    default. A layout whose signal is no port of the actor says so.
  - **Measured.** AHB's residual names `Lookup [4]`, `Allocate [5]` and `Shareable [6]` past `HPROT_WIDTH`'s default
    4. APB5's `PPROT` fits its width 3. AXI's four name each layout's fields past `MPAM_WIDTH`'s default 1 (a
    placeholder default, `KG-ISF-COMPLETENESS.2a.vii`) on `ARMPAM` and `AWMPAM`. Residuals: AHB 41 -> 42, APB5 41 ->
    42, AXI 83 -> 87; only `signal_layouts` moved in SemanticIR and IntentIR, and every `.isf` text is byte-identical.
  - **ADR 0033 §4, re-evaluated.** No rule, guard or drive names a sub-field, and a field changes no generated HDL, so
    no executable consumer needs the layout: no request is filed, and F29 stays *anticipated*, now owned by
    `.3.c.iv.b.iii` with its drafted bundle and trigger.
  - **Rebuilt.** AHB, APB5, AXI and ADIv6 rebuilt stage by stage (ADIv6 unchanged at every stage). The new fields
    moved the SemanticIR and IntentIR ruleset hashes, so `rebuild_stage_cascade.sh --write` re-sealed all 31 chains,
    content-identical at every stage.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — after `.3.c.iv.b.i` the six layouts stopped at EvidenceIR; the adapters' residual
  packets per chain; the ports the layouts name and their widths in each `.isf`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs` and `intent.rs` declared no carrier, and
  `crates/specforge/src/ir/adapters.rs` stated no residual. New carried fields, `signal_layout_residuals` and
  `IsfIr::signal_width` (`isf_ir.rs`). RED 5/5, each file restored byte-identically: SemanticIR dropping the layouts,
  IntentIR dropping them, the adapter stating no residual, no field listed past the width, a collective not expanded;
  each fails its test (`projects_protocol_surfaces_exactly_across_all_three_stages`,
  `isf_adapter_residualizes_a_signal_layout_naming_fields_past_the_width`).
- [x] **ADDRESSED (verified)** — the residuals above on the rebuilt chains; `.isf` texts byte-identical, so their
  FSMGen strict verdicts are unchanged.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `86ea02e05`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,923 passed; the proof tests count 64 SemanticIR and 63 IntentIR rule
  fields (43 carried) and the inventory 208 rules. `flow_census.json` rolled (+2 functions, +8 decision sites, +19
  helper edges, +1 macro); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — the residual reads the record, the collective's members and the port's own width;
  no document, signal or parameter name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A signal's bit sub-fields*: carriage and residual); KM card
  `a-table-naming-a-signals-bits-is-its-layout`; ladder F29 re-evaluated, unchanged.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.iii`
  Status: `open` (`2026-10-22`; REQUEST, waits for a consumer; opened by `.3.c.iv.b.ii`)
  Goal: **a port's sub-field layout reaches FSMGen once something executable names a sub-field.** FSMGen ladder F29 is
  *anticipated*: no `.isf` port takes a field, and ADR 0033 §4 files a request only when executable intent needs one.
  Trigger: a rule, guard or drive SPECFORGE lowers names a sub-field (`HPROT` *Privileged*, `AxMPAM` *PARTID*).
  Acceptance: on the trigger, refresh the drafted bundle (`.project-data/tmp/req-port-fields/packet`, reproduced on
  the pin `54ac1c22` and upstream `c51e22712`) against the current pin and upstream, sign it off as an
  `FSMGEN-REQUESTS` leaf and its ledger record, and move F29 to *requested*.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v.i`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.v`; checklist below)
  Goal: **a two-way table is a matrix record.** **Reproduce:** AXI *A12.11*, I2C *Table 3*, USB 3.2 *10-2* and *7-14*
  reach EvidenceIR as raw rows. Acceptance: the `matrix_tables` surface as `.3.c.v` decided (row keys, column key
  paths, cells as written, cell class, a caption's or a legend table's codes); precedence for the readers named there;
  measure the 31 chains, every family of the census accounted for.
  **Done `2026-10-22`.**
  - **Reader.** `read_matrix_tables` runs after the convergent loop. Label columns are every leading column before the
    first whose filled cells are mostly marks or modal words (`matrix_cell_kind`: a closed class, cells opening with a
    modal word included, a footnote mark dropped). The header is the leading run of non-data rows, at most the header
    rows EvidenceIR keeps, and seven in ten filled data cells are marks or modal words. Refused and counted: a
    slice-titled data column, all-number column titles, code-literal label columns (`is_matrix_code_literal`), two or
    more register-field titles. Presence, register, value-combination and layout tables are left to their readers.
    Records keep row key titles, column header paths, rows as written, the cell class and a caption's legend
    (`matrix_caption_legend`); a continuation page joins a head it repeats.
  - **Manifest.** `matrix_table_surface` records the `matrix_tables` entry only when a table had the shape.
  - **Measured.** Dry-run of the 31 chains: 20 identical; 48 matrices in 10 documents (AXI 22, USB 3.2 7, ADIv6 5,
    APB5 4, the Cortex-A76 guide 3, OpenCAPI 3, I2C 2, AHB 1, USB4 CM 1), and TMC's three *Permitted register* tables
    refused as field-keyed. Against `.3.c.v`'s census: the 34 less the 5 non-matrices and TMC's 3, that leaves 26, all
    read, and 22 more: 17 AXI and AHB property compatibility and interoperability tables whose cells open with
    *Compatible* (the census had counted only whole-cell words), 3 APB5 *presence and compatibility* tables, ADIv6's
    untitled `table_0098` and I2C's `table_0021`. I2C *Table 3*'s second page (no *continued* caption, one data row)
    stays unread. No statement or other record moved.
  - **Rebuilt.** The 11 chains whose EvidenceIR moved (10 with matrices, TMC with its refused candidates) rebuilt
      stage by stage: SemanticIR, IntentIR, every residual and every `.isf` text unchanged. The new field moved the
      EvidenceIR ruleset hash, so `rebuild_stage_cascade.sh --write` re-sealed all 31 chains, content-identical at
      every stage.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.v.i` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — `.3.c.v`'s census and the Python prototype of the rule
  (`.project-data/tmp/erg-3cvi/proto2.py`) over the 31 chains, each table classified.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` had no reader for a table keyed by rows and
  columns at once; the encoding, combination and presence readers key on codes, titled signals or presence rows. RED
  5/5, the file restored byte-identically: a code-keyed table read as a matrix, slice-titled columns read,
  field-titled columns read, the caption legend dropped, a cell opening with a modal word not modal; each fails
  `a_two_way_table_of_marks_is_a_matrix`..
- [x] **ADDRESSED (verified)** — the 48 matrices above in the dry-run and the rebuilt chains.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `a5eefe9fe`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,924 passed; the proof test counts 51 EvidenceIR rule fields and the
  inventory 209 rules (`matrix_tables` in both production-genericity inventories). `flow_census.json` rolled (+5
  functions, +71 decision sites, +108 helper edges, +2 macros) and its boundary moved for the new producer (rule roots
  135, grammar declassifiers 76).
- [x] **GENERICITY (ADR 0006)** — closed grammatical classes (marks, modal words), code-literal and slice shapes, and
  the document's own register field names; no document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A table of marks per pair of keys*); KM card
  `a-two-way-table-of-marks-is-a-matrix`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v.ii`
  Status: `open` (`2026-10-22`; CODE; opened by `.3.c.v`)
  Goal: **a matrix reaches IntentIR, and the adapter names it.** Acceptance: carry `matrix_tables` through SemanticIR
  to IntentIR; one adapter residual per matrix naming its family's disposition; `.isf` texts unchanged; rebuild what
  moves.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v.iii`
  Status: `open` (`2026-10-22`; CODE; opened by `.3.c.v`)
  Goal: **a presence code reads its legend.** **Reproduce:** AXI's presence records keep `YM`, `YS`, `NS` as written;
  *B2.1* says `YM` is mandatory on a manager and optional on a subordinate. Acceptance: bind each presence code to the
  legend the document states for it (by its codes, never by a vocabulary list), per interface side; measure every
  chain's presence records.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v.iv`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.v`)
  Goal: **a legality matrix's lowering is decided from a consumer.** AXI *A12.11*/*A12.13* say which tag operation an
  opcode may carry under each `MTE_Support`; *A9.20* which accesses a phase permits; TMC *3-52*/*3-53* which register
  accesses a state permits. Acceptance: bind row and column keys to the encodings and parameters they name, decide
  whether an assertion states the matrix faithfully (ADR 0033 §4, `sf-isf-actor-level-assertion`), and open the CODE
  leaf or record the residual.

- ID: `EXTRACTION-REPRESENTATION-GAPS.61`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.v`)
  Goal: **an encoding member enabled by a property says so.** **Reproduce:** AXI's untitled `AWCMO` table gives
  `CleanSharedPersist` the enabling property `Persist_CMO` and `CleanInvalidPoPA` `RME_Support`; the members are read
  (`Enum AWCMO …`) and the property is not. Acceptance: census encoding tables with such a column, decide the carrier
  (a member's enabling property, joined to the property's own record), and measure every chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.c`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.iv.b`)
  Goal: **a signal laid out per message kind is read.** **Reproduce:** AXI-L *A15.20*-*A15.22* and their continuations
  give each `ACADDR` bit a constant or a source field (`VA[56]`) per TLBI/BPI/VICI/PICI message part, and no reader
  takes them. Acceptance: decide the record (a layout per message kind, with constant and source-field bits), how it
  joins the message kinds' encodings and `DOC-INTENT-TAXONOMY.4b`'s layouts, and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.60`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.iv.b`)
  Goal: **a register field's sub-field table types the field's sub-fields.** **Reproduce:** ADIv6 *B3-10* lays
  `IDCODE`'s `DESIGNER` (`bits[11:1]`) out as *Continuation code* `Bits[11:8]` and *Identity code* `Bits[7:1]`, with
  ARM's values; `RegisterFieldRecord` has no sub-fields. Acceptance: decide the carrier (as `MessageSubField` did for
  message fields, `.55.b.iv.b.i`), how the register's bit positions map onto the field's, and measure every chain.
