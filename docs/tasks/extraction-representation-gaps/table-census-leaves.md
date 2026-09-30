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
  table (`.3.b.iii.b`), four are USB 3.2 timer tables (`.3.b.iii.c`), one maps IEEE 1149.1 names to JTAG-DP
  names (`.3.b.iii.d`, which found them to name the TAP and its state machine, not signals), and eight are the family rule's misfits (conventions, electrical and replay-buffer parameters,
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
  Status: `done` (`2026-09-29`; DESIGN + CODE; found by `.3.b.iii.b`; checklist below)
  Goal: **a record read from a table's row cites the table.** A statement EvidenceIR takes from a table's Markdown row
  (`| ARSNOOP | Must be 0b1110 . |`) carries an evidence span but no table, so the signal constraint or rule it yields
  cites none, and SemanticIR, which counts a table as cited only through records' `supporting_table_ids`, lists the
  table as reaching no carrier. 45 tables on the 31 chains are false residuals this way. Acceptance: decide where the
  citation lives (the statement's `supporting_table_ids`, as `.3.e` did for symbols, when its text is exactly one
  rendered row of one table in the document; and SemanticIR counting a table cited when a canonical record cites such
  a statement), keep an ambiguous row uncited, and measure every chain's residuals.
  **Decision (`2026-09-29`).** In EvidenceIR, right after statement assembly, a statement whose text is a Markdown row
  (`| … |`) cites the one structured table that has that row, header or body, with cell text compared after collapsing
  whitespace; a row two tables share cites neither. In SemanticIR, `unexplained_captured_regions` counts a table as
  cited when a canonical record cites a statement that cites it. That is statement-mediated, which the visual
  accounting refuses for a caption; a row statement is not a caption but the table's own content, so the refusal's
  reason does not apply. No new field: `ExtractedStatement.supporting_table_ids` exists since `.3.e`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.f`
  Status: `done` (`2026-09-29`; CODE; found by `.3.b.iii.b`; checklist below)
  Goal: **a keyed row's stated value is read wherever the cell states it.** AXI *Table A15.23* rows read
  `must_be_value` for 10 signals, but not `ARCACHE` (*Must be Modifiable, Non-cacheable ( 0b0010 )*: the code follows
  words) or `ARMMUVALID` (*Must be 0b0 . If not present, ARMMUVALID is assumed to …*: a second sentence follows), and
  `ARBURST` yields two identical constraints. Acceptance: read a code stated in parentheses after the words that name
  it, and a value whose sentence another follows, keep one record per row, and measure every chain.
  **Measured (`2026-09-29`).** `EXTRACTION-GAP-FIX.5b` refuses a value holding a comma, so that *Modifiable* alone is
  never published for `ARCACHE`; the same reader already takes a trailing `( … )` as the value's encoding and drops
  it (`Shareable ( 0b01 or 0b10 )` → *Shareable*). `ARMMUVALID` is refused by the same comma, which sits in its second
  sentence. `ARBURST`'s pair comes from two producers, `keyed_row_sigcon` and `dyn_sigcon`, both of the "established"
  surface that `dedup_appended_signal_constraints` preserves as it is, because one signal may carry one obligation
  from several different sentences.
  **Decision.** The value is read from the predicate's first sentence. When the words before a parenthesized code are
  compound and the parenthesis holds exactly one code literal, that code is the value (`Must be Modifiable,
  Non-cacheable ( 0b0010 )` → `0b0010`); a compound value without such a code stays refused. Among the established
  records, one that repeats another exactly, same fact read from the same statement, is dropped; the same obligation
  from a different sentence still stands.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.c`
  Status: `done` (`2026-09-29`; CODE; found by `.3.b.iii`; checklist below)
  Goal: **a timer table the ingest left untyped is read as timing.** USB 3.2 *Table 7-7. Transmitter Timers Summary*
  (`Timers | Timeout Value (μs)`), *Table 7-8* and *Table 7-12. LTSSM State Transition Timeouts* state named timeouts
  (`PENDING_HP_TIMER | 10`), and SourceIR typed none of them `timing_parameter`. Acceptance: decide whether a
  `Timeout` column with a stated unit is timing authority, read the four tables if so, and measure every chain.
  **Decided `2026-09-29`: a column headed `Timeout Value(s)` is timing authority, and each value is the nominal
  duration its timer is set to.** A timeout bounds nothing else, so the table states neither a `Min` nor a `Max`
  unless its cell does. SourceIR's typing is not touched (its timing classifier is inside the SOURCE seal
  closure). EvidenceIR reads an untyped table whose structural header rows head a column `Timeout Value` or
  `Timeout Values`, optionally with one parenthesized duration unit, and a name column (`Name`, `Timer(s)`,
  `Parameter`, `Symbol`). The timing records are built as follows:
  - an unlabeled duration is `typ`;
  - `12 ms (min) 120 ms (max)` gives those roles;
  - `12 ms (x1) 24 ms (x2)` gives one record per label as its `operating_condition`;
  - value columns a header row labels apart (`x1 operation`, `x2 operation`) give one record per column, and one
    cell spanning them all gives one record under no label;
  - an `Initial State` column is the row's condition, because `tNoLFPSResponseTimeout` is 2 ms in U1 and U2 but
    10 ms in U3;
  - a trailing footnote number is dropped only when the page prints that note (`Note 1:` in text, `Notes: 1.` in a
    table).
  Measured over all 31 chains, only USB 3.2 moves: 74 -> 108 timing records. That is 2 from *Table 7-7*, 6 from
  *Table 7-8* (two rows per width, two spanning both) and 26 from *Table 7-12*; `PORT_U2_TIMEOUT`, whose value
  refers to an LMP field, stays unread. No other record changes. The records carry no description, so they
  derive no temporal rule, and the `.isf` does not read timing. Three captured-region residuals go (475 -> 472),
  and the untyped uncited tables number 1,119. The fourth table, *Table 7-12*'s notes on the next page, has no
  timer row. Its text stops after note 1, so the state cells' marks `2` and `3` (`Polling.Active 2`) stay
  unproven and are kept as written. `.3.b.iii.g` owns the state columns.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.d`
  Status: `done` (`2026-09-29`; MEASUREMENT; found by `.3.b.iii`)
  Goal: **a table of a signal's other names feeds the alias map.** ADIv6 *Table B3-1 Comparison of IEEE 1149.1 and
  JTAG-DP signal names* pairs each IEEE name with the JTAG-DP name (its header row sits in the body), and EvidenceIR's
  `signal_alias_map` takes none of them. Acceptance: read a two-name table whose both columns name the same wire into
  aliases when one side is a declared signal, and measure.
  **Measured `2026-09-29`: the table names no signal, so the alias map is the wrong carrier, and no code follows.**
  *Table B3-1 Comparison of IEEE 1149.1 and JTAG-DP naming* (`table_0040`) has two rows under a header row the
  converter left in the body: `TAP | DBGTAP | Debug Test Access Port.` and `TAPSM | DBGTAPSM | Debug Test Access Port
  State Machine.`. These are a component and its state machine, and this leaf's goal took the caption's "naming"
  for signal names. What each name reaches today:
  - the eight TAP states in ADIv6's EvidenceIR already carry `machine_name` `DBGTAPSM`, the JTAG-DP name;
  - no SemanticIR actor is a TAP;
  - the prose uses both families (`TAP` 51 times, `DBGTAP` 8, `DBGTAPSM` 21, `TAPSM` once), but no record keys on
    `TAP` or `TAPSM`.
  So carrying the alias changes no record. The table is the document's naming concordance, apparatus in the
  census's terms. It becomes alias evidence only if a later reader carries the TAP as an actor or names a state
  machine from the IEEE side; that reader would read this table then.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.g`
  Status: `done` (`2026-09-30`; CODE; found by `.3.b.iii.c`; checklist below; opened `.3.b.iii.h`, `.3.b.iii.i`,
  `.3.f.iv`, `.44` and `TEXT-LAYER-IDENTIFIER-SPLIT.4`)
  Goal: **a timeout row's next state is the transition its timer takes.** USB 3.2 *Table 7-12. LTSSM State
  Transition Timeouts* states, per timer, the state it runs in and the state its expiry leads to
  (`tPollingActiveTimeout | Polling.Active | Rx.Detect/Polling.PortMatch/eSS.Disabled/eSS.Inactive | 12 ms`).
  `.3.b.iii.c` reads the duration and keeps the initial state as the row's condition. The transition itself is
  not carried, and the state cells keep footnote marks whose notes the converter dropped (`Polling.Active 2`).
  Acceptance: decide the carrier for a timeout-triggered state transition (a state and transition record, or a
  temporal rule from the initial state to the next on expiry), and whether a mark whose note is lost is read from
  the table's own mark sequence. Then measure every chain.
  **Decided `2026-09-30`: a typed transition record, carried to IntentIR and not lowered.** A temporal rule needs
  a clocked antecedent and a consequent on a signal; a timer's expiry is neither, and a rule per next state would
  assert each alternative. `StateTransitionRecord` is the explicit design notation's (`transition A -> B when …`):
  one target and a signal guard. So EvidenceIR gains `protocol_transitions` (`ProtocolTransitionRecord`) beside
  `protocol_states`, in `evidence.protocol_structure`, carried losslessly by SemanticIR and IntentIR (ADR 0033 §6:
  typed, queryable, not forced into an executable construct). A record has the source state, every listed next
  state, a `timer_expiry` trigger naming the timer and citing the row's own timing records, and the table. The
  caption binds a machine when it names exactly one a statement introduces. Several next states are alternatives
  the prose chooses among; the record does not choose. The `.isf` adapter keeps one residual per record
  (`isf_protocol_transition_*`): `.isf` has no timer whose expiry is a condition (`FSMGEN-REQUESTS.8`).
  **A slash separates two states only beside a name the table writes whole elsewhere**, since `Run-Test/Idle` is one
  name; a comma and the word `or` always separate. **Lost marks are read from the table's own run:** when the page
  prints note 1 and the name and state cells end in exactly 1 to *n* (*n* ≥ 2), all are marks; one text carrying two
  numbers is an enumeration and drops nothing.
  **Measured over all 31 chains:** 30 EvidenceIRs identical. USB 3.2 moves only `protocol_transitions` (0 -> 28, from
  25 rows: three rows run in two states; all 28 bound to `LTSSM`) and six timing records whose state condition loses
  its lost mark (`Initial State: Polling.Active 2` -> `Polling.Active`, likewise `Polling.Configuration`,
  `Polling.Idle` and `Polling.LFPS /Polling.LFPSPlus`); timing ids unchanged. `PORT_U2_TIMEOUT` (U1 -> U2) cites no
  timing record, since its value is not a duration. Four rows are self-loops as stated (`tU0LTimeout` U0 -> U0).
  **Issues uncovered, each owned:** 23 of the 27 states the transitions name are not `protocol_states` (`.3.b.iii.i`);
  two timer names keep a parenthesized operation mode (`tPollingSCDLFPSTimeout (SuperSpeedPlus operation)`,
  `.3.b.iii.h`); USB 3.2 `table_0144`'s typed timing values keep footnote marks (`4.0690104 1`, `7.6 2`, `.3.f.iv`);
  Symbol-font glyphs stay in the Private Use Area, so four timing units read `\uf06d s` for µs (`.44`); three of the
  same table's name cells are split mid-word (`BusIntervalAdjustmentGranula rity`, `TEXT-LAYER-IDENTIFIER-SPLIT.4`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.h`
  Status: `done` (`2026-09-30`; CODE; found by `.3.b.iii.g`; checklist below)
  Goal: **a timer name's parenthesized operation mode is the condition it holds under.** USB 3.2 *Table 7-12* names
  two timers `tPollingSCDLFPSTimeout (SuperSpeedPlus operation)` and `tPollingLBPMLFPSTimeout (SuperSpeedPlus
  operation)`; the timing records' `parameter_name` and the transitions' `timer_name` keep the parenthesis, while
  *Table 7-8*'s `x1 operation` column label becomes an `operating_condition`. **Reproduce:** timing records of
  `table_0094` whose name holds `(`. **Impact:** a timer that `.isf` could one day declare is named with a mode the
  document states as a condition. Acceptance: decide by grammar which trailing parenthetical in a timeout table's name
  cell is a mode (`(… operation)`) rather than part of the name (`Port (downstream connector)`, `(Deprecated)` in typed
  tables stay as written), read it as the records' condition, and measure every chain.
  **Decided `2026-09-30`: a mode phrase after one identifier is a row condition.** `timer_name_and_mode` splits a
  timeout table's name cell when it ends in a parenthesized phrase of two or more words whose last is `operation` or
  `mode` (the grammar the value columns label a mode with, `x1 operation`) and the words before it are one
  identifier. The mode is row-level, not per reading, so it is the timing records' first `row_conditions` entry
  (before the `Initial State` condition), not their `operating_condition`, which a value label may already hold. A
  transition gains `conditions` (a struct field, so no seal moves). The census of every timing name with a
  parenthesis on the 31 chains (19 names in 3 documents) shows why the grammar is narrow: `IL(f)`, `Loop bandwidth
  (BW)`, `RJ jitter (absolute)`, `Port (downstream connector)` and `(Deprecated)` are all part of a name or a status,
  and only the two `(SuperSpeedPlus operation)` cells name a mode.
  **Measured:** EvidenceIR dry-run of the 31 chains with the HEAD and the new binary: 30 identical; USB 3.2 moves
  only `timing_table_0094_004`/`_005` (name without the phrase, the phrase first in `row_conditions`) and the four
  transitions of those rows (`timer_name` without it, `conditions: ["SuperSpeedPlus operation"]`). Rebuilt stage by
  stage: SemanticIR and IntentIR move the same records; the adapter is unchanged.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.i`
  Status: `done` (`2026-09-30`; CODE; found by `.3.b.iii.g`; checklist below)
  Goal: **a state a stated transition names is a protocol state.** USB 3.2's 28 transitions name 27 distinct states;
  23 are not in `protocol_states` (`Polling.Active`, `Rx.Detect.Quiet`, `eSS.Inactive`, `Hot Reset.Exit`, …), which
  holds 30 states found in prose (`Polling`, `Hot Reset`, `U0`-`U3`, …). **Reproduce:** compare the names in
  `protocol_transitions` with `protocol_states[].state_name`. **Impact:** a transition names a state no state record
  declares, so a reader of `protocol_states` misses most of the LTSSM. Acceptance: decide whether a timeout table's
  state columns declare states (a `ProtocolStateRecord` citing the table, its machine from the caption), how a dotted
  name relates to a prose state (`Polling.Active` and `Polling`) without inventing a hierarchy, and measure every chain.
  **Decided `2026-09-30`: every state a transition names is a protocol state, as written.** The table states them as
  states (its `Initial State` and `… Next State` columns) of the machine its caption names, which is the same-statement
  evidence `bind_protocol_state_machines` accepts for prose. `merge_transition_states` runs after the transition
  surface: a name matching a prose state, compared case-insensitively as `protocol_state_surface` deduplicates, adds
  the table to the record's new `supporting_table_ids` and, only where the record has no machine, the machine every
  transition naming it agrees on; any other name is a `table_state_NNNN` record in order of first mention. A dotted
  name is its own state: reading `Polling.Active` as a child of `Polling` would read the document's notation, which
  ADR 0006 forbids, and the prose state `Polling` stays as it was. The contribution is the `transition_table_states`
  entry of the `protocol_states` manifest, recorded only for a document with transitions, so no other chain's
  manifest moves. The adapter's state residuals cite the table.
  **Measured:** EvidenceIR dry-run of the 31 chains with the HEAD and the new binary: 30 identical. USB 3.2 moves only
  `protocol_states` (30 -> 53: 23 `table_state_*`, all `LTSSM`; `U0`-`U3` gain `table_0094` and the `LTSSM` binding
  they lacked) and its manifest (`transition_table_states` produced 27, kept 23). Rebuilt stage by stage: SemanticIR
  and IntentIR carry the 53; adapter residuals 58 -> 81, the four `U0`-`U3` packets now cite the table too; lowering
  status and ISF tree unchanged.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.iv`
  Status: `open` (`2026-09-30`; CODE; found by `.3.b.iii.g`)
  Goal: **a typed timing table's value is read without its footnote mark.** USB 3.2 `table_0144` (`Name | Description |
  Min | Max | Units`) gives `BusIntervalAdjustmentGranularity` the maximum `4.0690104 1` and `tHostTransactionTimeout`
  the minimum `7.6 2`; the typed reader keeps the whole cell. **Reproduce:** timing records on the 31 chains whose
  value matches `<number> <number>` (2, both in `table_0144`). **Impact:** a limit that is not a number. Acceptance:
  read the mark by the rule the timeout reader uses (a printed note, or the table's own run), keep a value whose
  mark is unproven refused rather than guessed, and measure every chain.

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

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.e` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — on the 31 chains 45 tables listed as reaching no carrier have 75 rows that reached
  typed records through Markdown-row statements (AXI *Table A15.23*: 10 `must_be_value` constraints).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: a Markdown-row statement cited no table;
  `crates/specforge/src/ir/semantic.rs` `unexplained_captured_regions` counted a table cited only through records'
  own `supporting_table_ids`. RED, with each file restored byte-identically after each: overwriting a prior citation
  or comparing the statement uncollapsed fails `a_statement_that_is_a_tables_row_cites_that_table`; dropping the
  statement-mediated citation fails `a_table_is_cited_through_a_cited_statement_that_is_one_of_its_rows`.
- [x] **ADDRESSED (verified)** — all 31 chains rebuilt stage by stage, each stage validated, the adapter last: 12,114 of
  15,885 row statements cite a table (the rest are separator rows, rows two tables share, or rows rendered
  differently); 43 captured-region residuals go on 11 chains (USB 3.2 21, AXI 9, CHI C2C 3, RISC-V Debug 2, USB4 2,
  one each in IOMMU, AHB, ADIv6 and three OpenCAPI documents); no other SemanticIR, IntentIR or `.isf` change.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains moves only statements' `supporting_table_ids`.
  `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seeds identical (the HEAD binary scoring its own
  snapshot); `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D
  warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,797 passed, 0
  failed. `flow_census.json` rolled forward (+1 function, +8 decision sites, +64 helper edges, +1 macro).
- [x] **GENERICITY (ADR 0006)** — a statement's text against the document's own table rows; no document, table or
  signal name.
- [x] **LOCKSTEP** — book `pipeline/semanticir.md`; KM `captured-region-residual-carrier`,
  `untyped-uncited-tables-fall-into-fifteen-families`; the census record.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.f` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — AXI *Table A15.23*: `ARCACHE` (`Must be Modifiable, Non-cacheable ( 0b0010 )`) and
  `ARMMUVALID` (`Must be 0b0 . If not present, …`) yield no constraint; `ARBURST` yields two identical ones
  (`keyed_row_sigcon`, `dyn_sigcon`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `signal_keyed_obligation_value_is_readable`
  judged the whole cell, so a second sentence's comma refused it and a compound encoded by one code was refused
  outright; `dedup_appended_signal_constraints` kept every established record, and the two producers' records differ
  only in `target_value`. RED, with the file restored byte-identically after each: dropping the first-sentence cut or
  the code reading fails `the_three_guards_each_refuse_their_own_row`; dropping the exact-repeat merge or its target
  transfer fails `an_established_repeat_of_the_same_statement_is_one_record`.
- [x] **ADDRESSED (verified)** — AXI rebuilt stage by stage, each stage validated, the adapter last: `ARCACHE`
  `0b0010` and `ARMMUVALID` `0b0` under *DVM Complete request*, one `ARBURST` (target `INCR`); signal constraints 80
  -> 81, two more temporal rules and actor contracts; `.isf` text unchanged (the condition is not a guard, so the two
  obligations join `isf_obligations_not_lowered`, 112 -> 113).
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical; AXI moves only these three constraints
  (rule ids renumber by two, content identical). `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11
  seeds score identically (`seed_axi`'s preamble counts 75 constraints, was 74); `kg-bench` 158/158. `cargo fmt
  --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace
  --lib --exclude specforge-production-graph` 474 / 170 / 1,798 passed, 0 failed. `flow_census.json` rolled forward.
- [x] **GENERICITY (ADR 0006)** — sentence boundaries, a parenthesized code literal, record identity; no document or
  signal name.
- [x] **LOCKSTEP** — book `pipeline/evidence-failure-modes.md`; KM `a-keyed-rows-value-is-its-first-sentence-or-its-code`
  (new).

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.c` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — USB 3.2 *Tables 7-7, 7-8* and *7-12* (`table_0087`, `table_0088`, `table_0094`) are
  typed `unknown` and read by nothing: 0 timing records, 4 untyped uncited tables in the timing family.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source.rs` types a timing table only from
  `Min`/`Typ`/`Max` roles (`timing_table_has_structural_authority`), and `synthesize_timing_constraints` in
  `crates/specforge/src/ir/evidence.rs` reads only typed tables; a timeout column states neither. RED, with the
  file restored byte-identically after each: removing the reader, the note-number proof, the state condition, the
  `min` role or the spanning-cell reading, or loosening the header grammar, fails
  `an_untyped_timeout_table_is_read_as_timing` and/or `a_timeout_header_and_cell_are_read_by_their_grammar_only`.
- [x] **ADDRESSED (verified)** — `synthesize_timeout_table_constraints` and its grammar helpers. USB 3.2 rebuilt
  stage by stage, each stage validated, the adapter last: 74 -> 108 timing records in EvidenceIR, SemanticIR and
  IntentIR; captured-region residuals 475 -> 472; `.isf` text unchanged.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical, USB 3.2 moves only
  `timing_constraints` and no existing record changes. WIRE-BASED-100: all 11 seeds identical (temp paths aside);
  `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,800 passed, 0 failed.
  `flow_census.json` rolled forward (+9 functions, +52 decision sites, +110 helper edges, +6 macros).
- [x] **GENERICITY (ADR 0006)** — header words, a closed duration-unit list, cell grammar and the page's printed
  note numbers; no document, table or timer name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md`, `pipeline/semanticir.md`; research
  `category-family-census.md`; KM `timing-table-structural-authority` and `untyped-uncited-tables-fall-into-fifteen-families`
  (amended); `.3.b.iii.g` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.g` (CODE) — DONE `2026-09-30`

- [x] **REPRODUCE / MEASURE** — USB 3.2 *Table 7-12* (`table_0094`, `Name | Initial State | Timeout to Next State |
  Timeout Values`, 25 rows): 0 records carry a next state; six timing records keep lost marks (`Initial State:
  Polling.Active 2`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `timeout_table_columns` skipped every header
  holding `next` or `to`, no IR type could hold a timer-fired transition, and `without_note_number` knew only the
  printed notes. RED, with the file restored byte-identically after each (9 mutations): ignoring the next-state
  column, the caption's ambiguity, or the row's timing ids fails `a_timeout_rows_next_state_is_the_transition_its_timer_takes`;
  dropping the mark run, the printed-1 proof, the contiguity or the enumeration guard fails
  `a_lost_note_mark_is_read_from_the_tables_own_run`; splitting every slash or no `or` fails
  `a_state_cell_is_split_only_where_the_table_proves_a_boundary`.
- [x] **ADDRESSED (verified)** — `ProtocolTransitionRecord` in EvidenceIR, SemanticIR and IntentIR, built by
  `protocol_transition_surface` from the shared row reader `timeout_tables`. USB 3.2 rebuilt stage by stage, each
  stage validated once, the adapter last: 0 -> 28 transitions in each IR, all bound to `LTSSM`; six timing
  conditions lose their mark, ids unchanged; adapter residuals 30 -> 58 (`isf_protocol_transition_*`, citing
  `table_0094`), lowering status and ISF tree unchanged.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains with the HEAD and the new binary: 30 identical, USB 3.2
  moves only `protocol_transitions` and `timing_constraints`. The new rule fields re-seal every chain:
  `rebuild_stage_cascade.sh --write` rebuilt all 31 content-identical at every stage; `check_chain_currency.sh` 31 of
  31 current. WIRE-BASED-100: all 11 seeds identical to the reports of `63824f579`'s binary (no Rust change since;
  temp paths aside), since an older binary cannot read re-sealed artifacts; `kg-bench` 158/158. `cargo fmt --check`
  clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib
  --exclude specforge-production-graph` 474 / 170 / 1,803 passed, 0 failed. Claim-family inventory 175 -> 178 rules;
  `flow_census.json` rolled forward (+8 functions, +29 decision sites, +104 helper edges, +6 macros), and two boundary
  counts move by one for one cause: `protocol_transition_surface` is registered as a producer of a `grammar_introduces`
  family, so it is a rule root (123 -> 124) and a grammar root (`grammar_declassifiers` 64 -> 65).
- [x] **GENERICITY (ADR 0006)** — header words (`Next`, `State`), list punctuation, the table's own state names and
  note numbers, an introduced machine identifier; no document, table, timer or state name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (the `.3.b.iii.c` footnote sentence corrected, since
  the printed note is no longer the only proof), `pipeline/semanticir.md` (its stale "stop at SemanticIR" sentence
  corrected), `pipeline/intentir.md`, `pipeline/isf-adapter.md`, `reference/intentir-contract.md`,
  `source-to-intent-contract.md`, `reference/representation-gaps.md`, `reference/upstream-requests.md` (the four
  USB 3.2 packets' rows, missed at their sign-off); KM `a-timeouts-next-state-is-a-stated-transition` (new),
  `timing-table-structural-authority` (amended); category contract and census script carry the surface.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.h` (CODE) — DONE `2026-09-30`

- [x] **REPRODUCE / MEASURE** — USB 3.2 `table_0094`: two timing records and four transitions name their timer
  `tPollingSCDLFPSTimeout (SuperSpeedPlus operation)` / `tPollingLBPMLFPSTimeout (SuperSpeedPlus operation)`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `timeout_tables` took the whole name cell
  as the timer's name. RED, with the file restored byte-identically after each (5 mutations): no split, or dropping
  the mode from the timing conditions or from the transition, fails
  `a_timeout_rows_next_state_is_the_transition_its_timer_takes`; accepting any parenthetical or several words before
  it fails `a_timer_names_operation_mode_is_split_off_by_its_grammar_only`.
- [x] **ADDRESSED (verified)** — `timer_name_and_mode`, `TimeoutTableRow.mode`, `ProtocolTransitionRecord.conditions`.
  USB 3.2 rebuilt stage by stage, each stage validated once, the adapter last: the two timing records and four
  transitions carry the bare identifier and the mode as a condition in EvidenceIR, SemanticIR and IntentIR.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical.
  `check_chain_currency.sh` 31 of 31 current (no seal moved). WIRE-BASED-100: all 11 seeds identical to the reports
  of `c4c04f22a`'s binary (temp paths aside); `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 170 / 1,804 passed, 0 failed. `flow_census.json` rolled forward (+1 function, +1
  decision site, +4 helper edges); no boundary count moved.
- [x] **GENERICITY (ADR 0006)** — a closed pair of mode nouns (`operation`, `mode`), parenthesis and identifier shape;
  no document, table or timer name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md`; KM `a-timeouts-next-state-is-a-stated-transition`
  (amended). No book text described the old behaviour.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.i` (CODE) — DONE `2026-09-30`

- [x] **REPRODUCE / MEASURE** — USB 3.2: the 28 transitions name 27 distinct states; 23 are in no `protocol_states`
  record, and `U0`-`U3` carry no machine.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `protocol_state_surface` reads states from
  statements only, and `ProtocolStateRecord` could cite no table. RED, with the file restored byte-identically after
  each (5 mutations): no merge, overwriting a bound machine, binding the first of two machines, a case-sensitive
  match, or recording the manifest entry without transitions fails `every_state_a_transition_names_is_a_protocol_state`.
- [x] **ADDRESSED (verified)** — `merge_transition_states`, `ProtocolStateRecord.supporting_table_ids`, table
  provenance on the adapter's state residuals. USB 3.2 rebuilt stage by stage, each stage validated once, the adapter
  last: 53 protocol states in EvidenceIR, SemanticIR and IntentIR; adapter residuals 58 -> 81.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical.
  `check_chain_currency.sh` 31 of 31 current (no seal moved: the new record field and function sit outside the
  closure, and no `use` item changed). WIRE-BASED-100: all 11 seeds identical to the reports of `b349b1610`'s binary
  (temp paths aside); `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets
  -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,805
  passed, 0 failed. `flow_census.json` rolled forward (+1 function, +12 decision sites, +17 helper edges).
- [x] **GENERICITY (ADR 0006)** — state names as the transitions carry them, case-folded equality, the caption's
  machine binding; no document, machine or state name, and no reading of a name's punctuation.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md`, `reference/representation-gaps.md` (the state-machine
  row, and its `.3.b.iii.i` owner citation withdrawn); KM `a-timeouts-next-state-is-a-stated-transition` (amended).
