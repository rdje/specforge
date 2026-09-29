# EXTRACTION-REPRESENTATION-GAPS — table census leaves

- Part ID: `table-census-leaves`
- State: `active`

New records of the `.3` family continue here, from `.3.b`'s census of the untyped tables no record cites, after
`census-leaves` reached its size warning (`2026-09-29`).

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b`
  Status: `done` (`2026-09-29`; MEASUREMENT; continued and closed in this part; children `.3.b.i`, `.3.b.ii`)
  Goal: unchanged (see `program-and-census`). **Answered.** The population re-derives at 1,165 untyped tables no
  canonical record cites (not 1,161): 515 header shapes, 274 with no header row. `scripts/measure_untyped_table_families.py`
  places each by its header's words, or with no header row by its body's shape, into 15 families: 451 tables (38.7%)
  state nothing the contract lowers (document apparatus 180, figures read as tables 106, instruction timings and
  programming examples 73, line coding and electrical parameters 70, empty 15, waveform narrations 7); 389 (33.4%) are
  contract content with an owner (structure and message layouts 207 `DOC-INTENT-TAXONOMY.4b`, parameter definitions 78
  `KG-ISF-COMPLETENESS.2a.vii.c`, presence and compliance matrices 71 `.3.c.v`, encodings keyed by several fields or
  signals 27 `.3.g.iii`, TMC's untyped signal tables 6 `.3.b.i`); 325 (27.9%) are read table by table in `.3.b.ii`.
  Seven random members of each family were read: parameter definitions 7 of 7, software and performance 6 of 6,
  structure layouts about 5 of 6, presence matrices about 6 of 7; the two register families were mixed, so they go
  to `.3.b.ii` rather than to an owner. The family table is in
  [the census record](../../research/category-family-census.md). No code change, so no acceptance checklist applies.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.i`
  Status: `done` (`2026-09-29`; CODE; found by `.3.b`; checklist below)
  Goal: **a signal table SourceIR left untyped declares its signals.** CoreSight TMC r0p1's appendix tables for the ATB
  slave, APB debug, memory and AXI master interfaces (`table_0075`-`table_0080`, `Signal | Type | Description` and
  `Signal | Type | Source | Description`, 55 rows: `ATVALIDS`, `PADDRDBG[11:2]`, `MEMADDR`, `AWADDRM[39:0]`, …) are typed
  `unknown` at ingest, so no declaration reader visits them and none of their signals reaches the catalog, while the
  same header on typed tables declares every row (`SIGNAL-DECLARATION-ROW-DROP.6`). Acceptance: find why the ingest
  classifier leaves them untyped (caption, footnote marks in the name cells, the fused `MEMADDR a Output` cell),
  decide whether the fix belongs at ingest or in the evidence stage's reading of an untyped table, and measure every
  chain.
  **Measured (`2026-09-29`).** The ingest classifier (`classify_table_kind`, `ir/source/docling_backend.rs`) types a
  signal table from a caption naming signals, a `Direction` header, or a signal column plus a width column. These six
  have no caption and title their direction column `Type`, so they stay `unknown`, and `effective_table_kind` offers a
  declaration reader nothing else. The typed tables `SIGNAL-DECLARATION-ROW-DROP.6` fixed carry captions.
  **Decision:** the evidence stage, as `synthesize_register_field_tables` already reads an untyped field table. An
  untyped table whose header names a signal column (`Signal`, `Name`, `Port`, `Pin`, with or without `name`) and one of
  whose other columns holds direction words in most of at least two cells is a signal description. That is the
  ingest's own rule, a signal column with direction authority, with the authority read from the cells. Across all 78
  SourceIRs it admits exactly these six tables on the current chains, and 4 in GIC-600 and 1 in ATB, both legacy.
  **Shipped, with two readers the new rows exposed.** The first replay read all six tables. It also made `PCLKENDBG`
  the TMC's system clock, because the table clock reader takes a description starting *clock* and `PCLKENDBG` reads
  *Clock enable for APB.*. And the table relation reader's section-heading fallback gave the heading's actor
  (*Table A-3 ATB slave interface*) every row as a drive, so `ATDATAS`, an input, became an output. Both are fixed
  here: a *clock enable* is not a clock, and the fallback takes the row's direction when the table states one (an
  `Input` row is read, an `Output` or `Inout` row driven, a row stating none has no relation). The memory table
  (`table_0077`) is shifted by the converter (`MEMADDR a Output`, `MEMD b Input`, and `MEMQ`, read data, on an
  `Output` cell), so a table with a direction word inside a name cell stays untyped. That leaves five tables and 30
  declarations. The 20 bracketed names (`AWADDRM[39:0]`) stay refused (`SIGNAL-DECLARATION-ROW-DROP.2f`).
  TMC's `.isf` now lowers `master` (the `Source` column's name for the TMC's AXI side) rather than the caption actor,
  and every port it emits carries the document's direction. Two ATB ports had carried the wrong one. The TMC is split
  across three actor names, `KG-ISF-COMPLETENESS.1i`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.ii`
  Status: `done` (`2026-09-29`; MEASUREMENT; found by `.3.b`)
  Goal: **read the untyped tables no family places.** `.3.b` placed 840 of the 1,165 untyped uncited tables; 325 are
  read here, table by table: 56 register indexes and register-figure pieces (ADIv6 39: `Offset |`, `Component |
  Offset`, `LAR | LSR`), 12 register field tables the field-table reader missed (RISC-V Debug's `Field | Description |
  Access | Reset` with its header row in the body, ADIv6's `DLCR.TURNROUND` one-field value table among them), and a
  residue of 182 header shapes and 48 headerless tables (USB 3.2 43, ADIv6 39, AXI 32 the largest). Acceptance: each
  one a contract family with an owner, a non-contract disposition, or a named recall miss; refine the family script
  where a whole shape moves, and re-measure.
  **Answered.** `.3.b.i` took five TMC tables, so the population is 1,160. The family script gained rules for
  register-figure pieces, response and requirement matrices, captions saying *permitted*, *legal* or *example*,
  acronym glossaries and letter-spaced figure text, which leaves 125 tables no rule places, all read one by one: 56
  non-contract, 18 response or requirement matrices (`.3.c.v`), 19 keyed by several values (`.3.g.iii`), 17 layouts
  (`DOC-INTENT-TAXONOMY.4b`), 15 single-key value, mapping or constraint tables the ingest left untyped (`.3.b.iii`,
  opened, with 28 more the script places there). Five ADIv6 tables mapping `CSW.Prot`'s bits to `ARPROT`, `AWPROT`,
  `ARCACHE`, `AWCACHE` and `HPROT` bits go to `.41.e` (opened). The census, whole: 616 tables (53.1%) state nothing the
  contract lowers, 544 (46.9%) are contract content with an owner. Two RISC-V Debug `Field | Description | Access |
  Reset` tables are not missed fields: one is the format legend, the other one field's values. The dispositions are in
  [the census record](../../research/category-family-census.md). No product code change, so no acceptance checklist
  applies.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii`
  Status: `done` (`2026-09-29`; MEASUREMENT; found by `.3.b.ii`; children `.3.b.iii.a`-`.3.b.iii.d`, and `.41.f`)
  Goal: **a value, timer or constraint table the ingest left untyped is read by the reader its header names.** 43
  tables on the 31 chains state single-key values or limits and reach no record because SourceIR typed them
  `unknown`: value and meaning tables (GIC INTID ranges, USB4 directory keys and TMU modes, USB request codes, ADIv6's
  AP identification and `DbgSwEnable` values, RISC-V Debug's CSR number groups), USB 3.2's timers and LTSSM timeouts,
  and AXI's *DVM Complete* table (`Signal | Constraint`, *ARSNOOP must be 0b1110*), whose constraints no reader visits.
  `.3.b.i` read an untyped signal table from its own cells; measure which of these the encoding, timing and constraint
  readers would read under the same kind of structural proof, decide per reader, and measure every chain.
  **Measured (`2026-09-29`).** The encoding reader already accepts an untyped table by its header when the enum has
  an anchor (`table_looks_like_encoding`), so a type is not what these lack. Each table was read against the register
  records and the declared signals of its document: 23 encode a structure, message or protocol field with no signal
  or register to name the enum after (USB descriptor types, request codes and feature selectors, USB4 directory keys
  and TMU modes, CHI C2C `ActivationOp` and credit types, IOMMU `TTYP`, ADIv6 activation codes, AXI atomic operators),
  which `KG-ISF-COMPLETENESS.5.i`'s gate refuses by design, so they are `DOC-INTENT-TAXONOMY.4b`'s. Six state a
  register field's values: ADIv6 `DbgSwEnable` already carries both values from the field's own text, so its table is
  only uncited; IOMMU `eventID` and RISC-V Debug's `tdata1.type` list are in header shapes `.3.c.iii` does not read;
  ADIv6's AP identification tables key two fields (`TYPE`, `CLASS`) and `TYPE` names a field in two registers
  (`.3.b.iii.a`); RISC-V Debug's `cmdtype` table cannot be joined because `cmdtype` sits in the wrong register:
  RISC-V Debug's field tables are named after the last heading on their page, so `command` holds `abstractcs`'s
  `cmderr` and `datacount` and `confstrptr0` holds `command`'s `cmdtype` and `control` (`.41.f`). One is a constraint
  table (`.3.b.iii.b`), four are USB 3.2 timer tables (`.3.b.iii.c`), one maps IEEE 1149.1 signal names to JTAG-DP
  names (`.3.b.iii.d`), and eight are the family rule's misfits (conventions, electrical and replay-buffer parameters,
  a key legend, two response matrices), each of a family already owned.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.a`
  Status: `done` (`2026-09-29`; CODE; found by `.3.b.iii`; checklist below)
  Goal: **a field's value table is read in the header shapes documents use.** IOMMU *Table 19. Standard Events list*
  heads its code column with the bare field name (`eventID | Event counted | …`), RISC-V Debug lists `tdata1.type`'s
  values in a `Field | Description | Access | Reset` table of one field, and ADIv6's AP identification tables key two
  fields (`TYPE`, `CLASS`), `TYPE` being a field name two registers share. `.3.c.iii` reads a header that names one field
  (`AddrInc value`, `Value of X`). Acceptance: decide each shape (a bare field name heading a code column; a one-field
  table; two code columns whose fields both resolve in one register), and measure every chain.
  **Measured (`2026-09-29`).** `.3.c.iii` already takes a bare field name (`eventID`, and since `.41.f` `cmdtype` in
  `command`). The `eventID` table is refused because its third column (*IDT settings supported*: `0`, `0/1`) is mostly
  codes; the guard is there for a table keyed by two fields, whose *meaning* column is codes (`Value of CSW.ERRNPASS |
  Value of CSW.ERRSTOP | Functionality`). The `cmdtype` table has no header row: its header, `cmdtype | Command`, is its
  first body row. The AP identification tables stay refused, because `TYPE` names a field of two registers (`IDR`,
  `CSW@MEM-AP`). The `tdata1.type` table states no bit positions; it is a field description, not a value table.
  **Decision:** only the meaning column decides that a table is keyed by several fields; a code column after it is an
  attribute of each value. A table without a header row takes its first body row as the header, the field-table
  reader's own fallback.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.b`
  Status: `done` (`2026-09-29`; PROBE; found by `.3.b.iii`; children `.3.b.iii.e`, `.3.b.iii.f`)
  Goal: **a `Signal | Constraint` table states its signals' constraints.** AXI *Table A15.23: DVM Complete request
  constraints* lists `ARSNOOP | Must be 0b1110 .`, `ARADDR | Must be zero`, and so on for 14 signals. The table is
  untyped and no constraint reader visits it. Acceptance: read a table whose columns are a declared signal and a
  constraint sentence as that sentence stated of that signal, under the transaction the caption names, and measure.
  **Refuted (`2026-09-29`).** The constraints are read. The table's rows reach EvidenceIR as Markdown-row statements
  (`| ARSNOOP | Must be 0b1110 . |`), and the keyed-row reader turns 10 of the 14 into `must_be_value` constraints
  under the condition *DVM Complete request* (`ARSNOOP`, `ARADDR`, `ARBURST`, `ARLEN`, `ARDOMAIN`, `ARCHUNKEN`,
  `ARMMUATST`, `ARMMUFLOW`, `ARTAGOP`, `ARLOCK`). `ARID` and `ARSIZE` state relations, not values. Two values are
  missed: `ARCACHE` (*Must be Modifiable, Non-cacheable ( 0b0010 )*) and `ARMMUVALID` (*Must be 0b0 . If not present,
  …*), and `ARBURST` is read twice (`.3.b.iii.f`). The table is listed uncarried only because a Markdown-row statement
  cites no table and SemanticIR counts a table as cited only through a record's own `supporting_table_ids`, which
  signal constraints and conditional rules do not have. On the 31 chains **45 tables listed uncarried have 75 rows**
  reaching typed records this way (USB 3.2 21, AXI 11) (`.3.b.iii.e`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.e`
  Status: `open` (`2026-09-29`; DESIGN + CODE; found by `.3.b.iii.b`)
  Goal: **a record read from a table's row cites the table.** A statement EvidenceIR takes from a table's Markdown row
  (`| ARSNOOP | Must be 0b1110 . |`) carries an evidence span but no table, so the signal constraint or rule it yields
  cites none, and SemanticIR, which counts a table as cited only through records' `supporting_table_ids`, lists the
  table as reaching no carrier. 45 tables on the 31 chains are false residuals this way. Acceptance: decide where the
  citation lives (the statement's `supporting_table_ids`, as `.3.e` did for symbols, when its text is exactly one
  rendered row of one table in the document; and SemanticIR counting a table cited when a canonical record cites such
  a statement), keep an ambiguous row uncited, and measure every chain's residuals.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.f`
  Status: `open` (`2026-09-29`; CODE; found by `.3.b.iii.b`)
  Goal: **a keyed row's stated value is read wherever the cell states it.** AXI *Table A15.23* rows read
  `must_be_value` for 10 signals, but not `ARCACHE` (*Must be Modifiable, Non-cacheable ( 0b0010 )*: the code follows
  words) or `ARMMUVALID` (*Must be 0b0 . If not present, ARMMUVALID is assumed to …*: a second sentence follows), and
  `ARBURST` yields two identical constraints. Acceptance: read a code stated in parentheses after the words that name
  it, and a value whose sentence another follows, keep one record per row, and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.c`
  Status: `open` (`2026-09-29`; CODE; found by `.3.b.iii`)
  Goal: **a timer table the ingest left untyped is read as timing.** USB 3.2 *Table 7-7. Transmitter Timers Summary*
  (`Timers | Timeout Value (μs)`), *Table 7-8* and *Table 7-12. LTSSM State Transition Timeouts* state named timeouts
  (`PENDING_HP_TIMER | 10`), and SourceIR typed none of them `timing_parameter`. Acceptance: decide whether a
  `Timeout` column with a stated unit is timing authority, read the four tables if so, and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.d`
  Status: `open` (`2026-09-29`; CODE; found by `.3.b.iii`)
  Goal: **a table of a signal's other names feeds the alias map.** ADIv6 *Table B3-1 Comparison of IEEE 1149.1 and
  JTAG-DP signal names* pairs each IEEE name with the JTAG-DP name (its header row sits in the body), and EvidenceIR's
  `signal_alias_map` takes none of them. Acceptance: read a two-name table whose both columns name the same wire into
  aliases when one side is a declared signal, and measure.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.b.i` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — CoreSight TMC `table_0075`-`table_0080` (`Signal | Type | Description`, 55 rows) are
  SourceIR kind `unknown`; none of their signals is declared or in the catalog.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `effective_table_kind` gives an untyped table
  only corpus-memory guidance; the ingest (`classify_table_kind`) needs a caption, a `Direction` header or a width
  column. Exposed: `synthesize_system_contract_from_table_descriptions` takes *clock enable* for a clock, and
  `extract_relations_from_signal_tables_with_prior_guidance` drives every row from a section heading's actor. RED,
  with the file restored byte-identically after each: removing the promotion, the shifted-row guard, the `Input` read
  or the *clock enable* exclusion fails `an_untyped_signal_table_with_a_direction_column_is_read_as_one`.
- [x] **ADDRESSED (verified)** — TMC rebuilt from its HEAD snapshot stage by stage, each stage validated, the adapter
  last: +30 declarations (`table_signal_declaration_provenance` 19 -> 49), clock `CLK`, interfaces 8 -> 11, relations
  13 -> 27 each with the direction its row states; `.isf` renderable, 18 ports added, 6 moved to
  `isf_ports_not_on_the_lowered_actor`, FSMGen `--strict --check` 0 diagnostics.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical. `check_chain_currency.sh` 31 of 31
  current. WIRE-BASED-100: all 11 seeds identical; `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 170 / 1,794 passed, 0 failed. `flow_census.json` rolled forward.
- [x] **GENERICITY (ADR 0006)** — header role words, direction words in cells, and *clock enable*; no document or signal
  name.
- [x] **LOCKSTEP** — book `pipeline/evidence-failure-modes.md`; KM `an-untyped-signal-table-is-read-by-its-direction-cells`;
  `.3.c.viii` widened; `KG-ISF-COMPLETENESS.1i` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.a` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — IOMMU *Table 19. Standard Events list* (`eventID | Event counted | IDT settings
  supported`) and RISC-V Debug *Table 3. Meaning of cmdtype* (no header row) state a field's values and fill nothing.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `fill_field_values_from_value_tables`
  refused any table with a second code column anywhere, and skipped a table with no header row. RED, with the file
  restored byte-identically after each: guarding every column again, dropping the headerless fallback, or dropping the
  repeated-value refusal fails `a_field_value_table_fills_the_one_field_it_names`.
- [x] **ADDRESSED (verified)** — IOMMU, RISC-V Debug and ADIv6 rebuilt stage by stage, each stage validated, the
  adapter last: `eventID` 9 values, `command.cmdtype` 3, `IDR.TYPE` 6; the first replay also filled IOMMU `DMASK`
  from a programming example (`0, 1, 1, 1`), which the repeated-value refusal now stops. The IOMMU and ADIv6 `.isf`
  carry the new field enums; FSMGen `--strict --check` 0 diagnostics on both.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 28 identical, three move only these three fields and the
  tables they cite. `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seeds identical (the rebuilt
  documents scored by the HEAD binary on its own snapshot); `kg-bench` 158/158. `cargo fmt --check` clean; `cargo
  clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 170 / 1,795 passed, 0 failed. `flow_census.json` rolled forward (+3 decision
  sites).
- [x] **GENERICITY (ADR 0006)** — table structure only: which column holds codes, a header row's place, repeated
  values; no document, register or field name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (*A field's value table*); KM
  `field-value-table-fills-register-field`.
