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
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.v`; checklist below)
  Goal: **a matrix reaches IntentIR, and the adapter names it.** Acceptance: carry `matrix_tables` through SemanticIR
  to IntentIR; one adapter residual per matrix naming its family's disposition; `.isf` texts unchanged; rebuild what
  moves.
  **Done `2026-10-22`.**
  - **Carried.** `matrix_tables` is a field of SemanticIR (`semantic.carried_evidence`) and IntentIR
    (`intent.carried_semantics`), cloned as it is; both claim-family inventories name it.
  - **Residual.** One `isf_protocol_matrix_table_*` packet per matrix, citing its tables: no rule, guard or drive
    names its keys; a legality matrix's lowering is `.3.c.v.iv`'s; a requirement, version or effect matrix states
    configuration the `.isf` does not carry (ADR 0033 §6).
  - **Rebuilt.** The 10 matrix chains rebuilt stage by stage: only `matrix_tables` moved in SemanticIR and IntentIR,
      and the adapters gained 48 residuals (AXI 87 -> 109, USB 3.2 95 -> 102, ADIv6 30 -> 35, APB5 42 -> 46, the
      Cortex-A76 guide 3 -> 6, OpenCAPI discovery 3 -> 5, I2C 7 -> 9, AHB 42 -> 43, OpenCAPI DLL 1 -> 2, USB4 CM 5 ->
      6); every `.isf` text byte-identical. The new fields moved the SemanticIR and IntentIR ruleset hashes, so
      `rebuild_stage_cascade.sh --write` re-sealed all 31 chains, content-identical at every stage.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.v.ii` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — after `.3.c.v.i` the 48 matrices stopped at EvidenceIR; the adapters' residual packets
  per chain.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs` and `intent.rs` declared no carrier, and
  `crates/specforge/src/ir/adapters.rs` stated no residual. RED 3/3, each file restored byte-identically: SemanticIR
  dropping the matrices, IntentIR dropping them, the adapter stating no residual; each fails its test
  (`projects_protocol_surfaces_exactly_across_all_three_stages`,
  `isf_adapter_residualizes_a_matrix_table_citing_its_tables`)..
- [x] **ADDRESSED (verified)** — the residuals on the rebuilt chains; `.isf` texts byte-identical, so their FSMGen
  strict verdicts are unchanged.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `d5a85fe5a`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,925 passed; the proof tests count 65 SemanticIR and 64 IntentIR rule
  fields (44 carried) and the inventory 211 rules. `flow_census.json` rolled (+2 decision sites); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — the residual reads the record only; no document or table name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A table of marks per pair of keys*: carriage and residual); KM card
  `a-two-way-table-of-marks-is-a-matrix`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v.iii`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.v`; checklist below)
  Goal: **a presence code reads its legend.** **Reproduce:** AXI's presence records keep `YM`, `YS`, `NS` as written;
  *B2.1* says `YM` is mandatory on a manager and optional on a subordinate. Acceptance: bind each presence code to the
  legend the document states for it (by its codes, never by a vocabulary list), per interface side; measure every
  chain's presence records.
  **Done `2026-10-22`.**
  - **Binding.** `bind_presence_legends` runs after `read_matrix_tables`. A legend is a matrix with one label column,
    modal cells and distinct row keys. A presence table binds to the one legend whose row keys include every code the
    table uses; with none or two, nothing binds. Each `VariantPresenceEntry` gains `legend` (`PresenceLegendMeaning`:
    the legend column's title and the row's cell, as written), and the `SignalPresenceRecord` `legend_matrix_id`.
  - **Measured.** Dry-run of the 31 chains: 30 identical. AXI's 297 *B2.2*/*B2.3* presence rows (11 tables) bind to
    *B2.1*; *A13.3*'s 9 use `C`, which *B2.1* does not define, and stay as written. APB5's and AHB's codes (`C`, `OC`,
    `OO`) have no legend matrix. No EvidenceIR field is new, so the ruleset hash is unchanged.
  - **Rebuilt.** AXI rebuilt stage by stage: only `signal_presence_records` moved in SemanticIR and IntentIR; its 109
      residuals and `.isf` text are unchanged. No cascade was needed: the ruleset hash did not move, and the other 30
      chains are current.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.v.iii` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — the codes each chain's presence tables use, and the legend matrices `.3.c.v.i` reads.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: presence records kept each code as
  written, and nothing joined them to the legend the document states. RED 3/3, the file restored byte-identically: a
  table binding when any code is a legend key, the first of two covering legends binding, no legend cells attached;
  each fails `a_presence_code_reads_its_legend`..
- [x] **ADDRESSED (verified)** — AXI's 297 bound rows above, in the dry-run and the rebuilt chain.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `4a5052ced`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,926 passed. `flow_census.json` rolled (+1 function, +36 decision sites,
  +34 helper edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — a legend is found by its shape and its codes, never by a code vocabulary.
- [x] **LOCKSTEP** — book `pipeline/semanticir.md` (the carried presence fields) and `pipeline/isf-enums.md` (a
  matrix can be a legend); KM card `a-two-way-table-of-marks-is-a-matrix`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v.iv`
  Status: `done` (`2026-10-22`; DESIGN; opened by `.3.c.v`; opened `.3.c.v.v`, `.62`)
  Goal: **a legality matrix's lowering is decided from a consumer.** AXI *A12.11*/*A12.13* say which tag operation an
  opcode may carry under each `MTE_Support`; *A9.20* which accesses a phase permits; TMC *3-52*/*3-53* which register
  accesses a state permits. Acceptance: bind row and column keys to the encodings and parameters they name, decide
  whether an assertion states the matrix faithfully (ADR 0033 §4, `sf-isf-actor-level-assertion`), and open the CODE
  leaf or record the residual.
  **Measured `2026-10-22`** (the legality matrices `.3.c.v.i` reads, against AXI's IntentIR and `manager.isf`).
  - **AXI *A12.11*/*A12.13*.** The tag-operation column keys (`Transfer`, `Fetch`, `Update`, `Match`) are members of
    `ARTAGOP`/`AWTAGOP`, which the `.isf` declares. `MTE_Support = Basic` names a property; the `.isf` has no
    parameter for it, and properties reach IR as `KG-ISF-COMPLETENESS.2a.vii.c.v`/`.c.vi` decide. The row keys are
    opcodes (`ReadNoSnoop`, `WriteUnique*`, `Write*CMO`): no enum or combination names them. AXI states their decode,
    *A7.4 ARSNOOP encodings* (`ARSNOOP | ARDOMAIN 1 | Opcode | Enable`): a value combination of two signals. No reader
    takes it, because its second title carries a footnote mark (`ARDOMAIN 1`) and so names no declared signal: `.62`.
    A `*` key names a family of opcodes.
  - **AXI *A9.20*.** Its rows are the numbered phases of a described sequence, with no signal to bind; it stays typed.
  - **TMC *3-52*/*3-53*.** Refused by `.3.c.v.i` as field-keyed: a value combination of `TraceCaptEn`, `TMCReady` and
    `MODE` that the combination reader refuses for its emptied cells (`.3.g.iii.b.i`).
  **Decided.**
  - **What a lowering is.** A legality matrix states which tag operation an opcode may carry under a property value.
    Its faithful `.isf` form is an always-on assertion per actor. Each data column (property value, tag operation) is
    guarded by that parameter's value and that tag operation's code. Its consequent is the set of opcodes the column
    marks, each decoded to its signals' codes. A `*` key expands to the opcodes the decode names under it.
  - **Not lowered yet, by three prerequisites.** (1) The opcode decode: `.62` reads *A7.4* and its write twin as value
    combinations. (2) The property as a parameter: KG-ISF `.2a.vii.c.v`/`.c.vi`. (3) An always-on assertion outside a
    transaction: `sf-isf-actor-level-assertion`, accepted by FSMGen and not yet shipped
    (`EXTRACTION-REPRESENTATION-GAPS.15`). `.3.c.v.v` lowers the matrices once all three hold. Until then each keeps
    its `isf_protocol_matrix_table_*` residual. No new FSMGen request is needed: the shape is the accepted actor-level
    assertion, and ADR 0033 §4 holds.
  - **The other families** (requirement, version, effect) stay typed, as `.3.c.v` decided.

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

- ID: `EXTRACTION-REPRESENTATION-GAPS.62`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.v.iv`; opened `.63`, `.64`; checklist below)
  Goal: **a column title with a footnote mark names its signal.** **Reproduce:** AXI *A7.4 ARSNOOP encodings* titles
  its second key `ARDOMAIN 1` (footnote 1), so `combination_key_name` names no signal, the table has one key, and the
  opcode decode (`ReadNoSnoop` = `ARSNOOP` 0b0000 with `ARDOMAIN` `NSH, SYS`) reaches no record. Acceptance: strip a
  footnote mark the document's own footnote defines (or a trailing digit after a declared name) before naming a key;
  measure every chain's combination, layout and matrix titles; keep every existing record.
  **Done `2026-10-22`.**
  - **Census.** Of every chain's header cells, 54 in 5 documents end in a mark that a footnote on the table's page or
    a neighbour opens with (ADIv6 19 letters; AXI 14, OpenCAPI 8, USB 3.2 7, USB4 CM 6 digits). Only AXI's
    `AWDOMAIN 1` and `ARDOMAIN 1` (*A7.3*, *A7.4*) put the mark after a declared name. AXI's other ten sit in matrix
    titles and cells (*A12.13* `Update 1`, `Y 2`, `Y 3`; *A8.8* `Must hit 2`), where the note narrows the cell:
    `.64`. No layout title carries one; OpenCAPI's `Lane 4` and USB's `Port 1` name no declared signal.
  - **Key names.** `combination_key_name` first drops a trailing mark (`without_note_number`) the table's notes use:
    the note numbers `note_numbers_by_page` reads and the footnote letters `footnote_marks_by_page` reads, on the
    table's page or beside it, or beside any fragment its caption names. A table carried over pages prints its notes
    after its last fragment: *A7.3*'s note 1 is two pages after its head. Without such a note a trailing word stays
    in the title, so `ARDOMAIN 2` names nothing. The key keeps its title as printed.
  - **Member.** A reserved row (`RESERVED`, `is_reserved_field_name`) names no member, so it may repeat in the
    member column; *A7.3*'s three and *A7.4*'s six would otherwise leave the opcodes as a meaning.
  - **Measured.** Dry-run of the 31 chains: 30 identical. AXI's `value_combinations` go from 7 to 9: *A7.3* (25 rows
    over `table_0102`-`table_0104`) and *A7.4* (19 rows over `table_0105`-`table_0106`), keyed by the snoop code and
    the domain, the member `Opcode`. The seven earlier records are kept, renumbered by table order, so *A15.26*'s
    four states cite `value_combination_0009`. No domain value joins an encoding: `NSH`, `SH`, `SYS` are footnote
    1's labels, not `AxDOMAIN`'s members (`.63`). AXI's `.isf` `ARSNOOP`/`AWSNOOP` members come from *A9.4*/*A9.11*
    and are unchanged.
  - **Rebuilt.** AXI rebuilt stage by stage: `value_combinations` and `protocol_states` moved in SemanticIR and
    IntentIR, the adapter states 111 residuals (109 before, the two new combinations), and `manager.isf` is
    byte-identical. No EvidenceIR field is new, so the ruleset hash is unchanged and no cascade was needed.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.62` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — the census above: every chain's header cells ending in a mark a nearby footnote
  opens with (`.project-data/tmp/erg-62/census.py`), and AXI *A7.3*/*A7.4* read as no combination before.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `combination_key_name` compared the
  whole title (`ARDOMAIN 1`) with the declared names, and the member column had to be distinct in every row, which
  repeated `RESERVED` rows defeat. RED 4/4, the file restored byte-identically: a key title keeping its mark, a
  reserved row required to name a distinct member, a fragment taking only its own pages' marks, a mark stripped with
  no note printing it; each fails `a_key_title_with_a_footnote_mark_names_its_signal`.
- [x] **ADDRESSED (verified)** — AXI's two decodes above, in the dry-run and the rebuilt chain.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `9c48f1974`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,927 passed. `flow_census.json` rolled (+6 decision sites, +6 helper
  edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — a mark is stripped only when the document prints its note; no document, table or
  signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (keys, member, residual count, *A7.3*/*A7.4*, the legality
  matrices' prerequisites); KM card `a-jointly-keyed-table-is-a-value-combination`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.63`
  Status: `done` (`2026-10-22`; CODE; opened by `.62`; checklist below)
  Goal: **a key value of labels a note defines carries their codes.** **Reproduce:** AXI *A7.3*/*A7.4* key each
  opcode by a domain list (`NSH, SYS`) whose labels footnote 1 defines (*NSH is Non-shareable (0b00), SH is
  Shareable (0b01 or 0b10), SYS is System (0b11)*); `bind_combination_value_encodings` joins nothing, so the decode
  has no domain codes. Acceptance: read a note that defines a title's labels with their codes, bind each label of a
  list to its codes, and measure every chain's combinations.
  **Done `2026-10-22`.**
  - **The note.** A key whose title carries a mark keeps the note it refers to (`CombinationKey.note`): the one text
    that `footnote` elements with that mark print beside the table's fragments (`footnote_texts_by_page`). Two
    different notes with the mark give none.
  - **Definitions.** `note_label_definitions` reads a note that is wholly a list, split at commas outside
    parentheses, of `<label> is <name>` items, each optionally closed by its codes in parentheses. A label is one
    word with a letter, and the codes are code literals. A note stating anything else defines nothing.
  - **Binding.** A value that no encoding names directly may list defined labels. `defined_labels_encoding`
    resolves each label by its name through the key's encodings, as a label value is. All the labels must sit in one
    table, and a code the note prints must equal the code that table gives. The value then carries every label's
    codes, in order. The note's codes are a check, not the source: the encoding table stays the provenance.
  - **Measured.** Dry-run of the 31 chains: 30 identical. AXI's joined values go from 34 to 69: all 35 domain values
    of *A7.3* (22) and *A7.4* (13), each resolved through *A8.5* `AxDOMAIN encodings` (`table_0111`). `NSH, SYS` is
    `0b00`, `0b11`, and `NSH, SH, SYS` all four codes. No `-` value joins. No EvidenceIR field is new
    (`CombinationKey.note` is a nested field), so the ruleset hash is unchanged.
  - **Rebuilt.** AXI rebuilt stage by stage: only `value_combinations` moved in SemanticIR and IntentIR; its 111
    residuals and `manager.isf` are unchanged, and `check_chain_currency.sh` confirms the ruleset did not move.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.63` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — `.62`'s two decodes, whose 35 domain values joined nothing, and every chain's
  combination keys (only AXI's two carry a mark).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: a key kept no note, and
  `bind_combination_value_encodings` joined only a value that is one label of the encoding, never a list of the
  note's abbreviations. RED 5/5, the file restored byte-identically: a key recording no note, a note of any mark
  taken as the key's, an undefined label skipped, a printed code not checked, any sentence read as a definition;
  each fails `a_key_title_with_a_footnote_mark_names_its_signal` or
  `a_key_value_of_labels_a_note_defines_carries_their_codes`.
- [x] **ADDRESSED (verified)** — AXI's 35 joined domain values above, in the dry-run and the rebuilt chain.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `1b9fb1905`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,928 passed. `flow_census.json` rolled (+3 functions, +26 decision
  sites, +44 helper edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — a note's grammar (`<label> is <name> ( codes )`) and the document's own encoding;
  no label, signal or document name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (joined values 69; a note's labels); KM card
  `a-jointly-keyed-table-is-a-value-combination`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.64`
  Status: `done` (`2026-10-22`; CODE; opened by `.62`; opened `.65`; checklist below)
  Goal: **a matrix cell's note narrows it.** **Reproduce:** AXI *A12.13* marks `WriteNoSnoop`'s Standard `Match` `Y
  2` (*Not Exclusive write*) and `Write*CMO`'s Transfer `Y 3` (*Domain must be Non-shareable*), and titles a column
  `Update 1` (*Partial tag updates are not supported*); the matrix keeps `Y 2` as written and the note is read by
  no record. Acceptance: bind a matrix cell's or column title's mark to its note's text, kept on the matrix, and
  measure every chain's matrices.
  **Done `2026-10-22`.**
  - **Marks.** `trailing_note_marks` reads the words after a title's or cell's last other word, each a note number,
    a bracketed one or a lowercase letter (`Y 2`, `O [2]`, `State 1, 2`). A cell of marks alone (`2, 3`) has none.
  - **Notes.** `bind_matrix_notes` runs after the matrix reader. Each mark cites the one note with that mark that
    `footnote` elements print on the pages beside the matrix's tables (`footnote_texts_by_page`); with none, or two
    different ones, it cites nothing. `MatrixTableRecord.notes` holds one `MatrixNote` per note: its mark, its text,
    the data columns whose title cites it and the cells (`MatrixCellPosition`) that do. Titles and cells stay as
    written.
  - **Measured.** Dry-run of the 31 chains: 29 identical. AXI *A12.13* (`matrix_table_0018`) cites three notes:
    note 1 on column 4 (`Update 1`), note 2 on one cell, note 3 on two. USB 3.2 *9-2* cites notes 1 and 2 on both
    columns (*'Yes' indicates a parameter that shall be preserved*). ADIv6's `64 bits d` and USB's `Set Address 0`
    have no such note and cite nothing. I2C *Table 3* prints its three notes as list items whose numbers the
    converter drops, so `O [2]` and `Target [1]` reach no note: `.65`. No EvidenceIR field is new
    (`MatrixTableRecord.notes` is a nested field).
  - **Rebuilt.** AXI and USB 3.2 rebuilt stage by stage: only `matrix_tables` moved in SemanticIR and IntentIR;
    their residuals (111, 102) and AXI's `manager.isf` are unchanged, and `check_chain_currency.sh` confirms the
    ruleset did not move.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.64` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — every chain's matrix titles and cells closing with a mark, and the notes printed
  beside each matrix: AXI *A12.13*, USB 3.2 *9-2*/*9-10*, I2C *Table 3*, ADIv6, Cortex-A76.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `matrix_cell_kind` drops a trailing mark
  to classify a cell, and nothing read the note it cites. RED 4/4, the file restored byte-identically: a cell of
  marks alone citing its marks, a mark no note explains kept, a title citing no column, a note on any page taken as
  the matrix's; each fails `a_matrix_cell_or_title_cites_the_note_printed_beside_it`.
- [x] **ADDRESSED (verified)** — AXI's three and USB 3.2's two notes above, in the dry-run and the rebuilt chains.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `d7336f49f`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,929 passed. `flow_census.json` rolled (+2 functions, +14 decision
  sites, +68 helper edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — a mark's shape and the document's own printed notes; no document, table or mark
  list.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (a matrix keeps its notes); KM card
  `a-two-way-table-of-marks-is-a-matrix`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.65`
  Status: `open` (`2026-10-22`; CODE; opened by `.64`)
  Goal: **a note printed as a numbered list keeps its number.** **Reproduce:** I2C *Table 3* marks `Target [1]`, `O
  [2]` and `O [3]`, and prints the notes after its continuation as three `list_item` elements on `page_0008` (*Also
  refers to a controller acting as a target.*, *Clock stretching is a feature of some targets …*, *'Bit banging' …
  should consider a START byte.*) with no number, so no mark reaches its note. Acceptance: find where the number is
  lost (the conversion or SourceIR's list reading), keep it so the note is read as note 1-3, never bind by position
  alone, and measure every chain's numbered notes.
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.vi.a`
  Status: `open` (`2026-10-22`; CODE; opened by `.3.c.vi`)
  Goal: **a table a carried record reads is cited.** **Reproduce:** `SemanticIr::cited_table_ids` gathers register,
  timing, polarity, interface-signal and symbol records only, though its comment says *every SemanticIr surface that
  declares supporting_table_ids*. Tables SemanticIR carries in `value_combinations`, `matrix_tables`,
  `signal_layouts`, `signal_presence_records` and `protocol_states` stay `no_canonical_carrier_for_captured_region`:
  82 of the 1,219 table residuals (10 typed), AXI 48 of them. Acceptance: gather every carried surface that names its
  tables (`table_ids` or `supporting_table_ids`), with a test that fails when a carried table field is not gathered,
  and measure every chain's residuals.
- ID: `EXTRACTION-REPRESENTATION-GAPS.66`
  Status: `open` (`2026-10-22`; CODE; opened by `.3.c.vi`)
  Goal: **a one-column code key is a legend.** **Reproduce:** AHB *Table A-1 Key for Table A-2 and Table A-3* (`Code |
  Meaning`: `Y` *Mandatory for inputs and outputs*, `N`, `O`, `C`, `OC`) defines every code AHB's 80 presence rows
  use, but it has one data column, so `read_matrix_tables` does not take it and `bind_presence_legends` finds no
  legend; `.3.c.v.iii` recorded AHB's codes as having none. Acceptance: read a code key whose codes cover a presence
  table's (by codes, never by a vocabulary), bind it as `.3.c.v.iii` binds *B2.1*, and measure every chain.
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
