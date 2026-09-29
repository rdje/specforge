# EXTRACTION-REPRESENTATION-GAPS — PDF-to-IR census leaves

- Part ID: `census-leaves`
- State: `active`

New records of the `.3` family (the PDF-to-IR census and the leaves it found) continue here, after
`program-and-census` reached its size warning (`.3.f`, `2026-09-28`).

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.i`
  Status: `done` (`2026-09-29`; CODE; found by `.3.f`; checklist below)
  Goal: **a timing limit keeps the row condition it is stated under.** Timing tables head a `Conditions` column (16
  tables in 4 current chains: I2C 6, both OpenCAPI PHY signalling specs 4 each, I2S 2) and the reader ignores it, so
  one parameter gives several records with different limits and nothing saying which applies when: in I2C 8
  (table, parameter, condition) keys hold 16 records, e.g. `I OL` at `V OL = 0.4 V` (min 3) and `V OL = 0.6 V` (min
  6), `t HD;DAT` for *CBUS compatible controllers* (5.0) and *I2C-bus devices* (0), `t rCL` by load. Some rows are
  condition headings with no value (`V OL`: *(open-drain) at 3 mA sink current*, refined by the next rows). Acceptance:
  read a column the table heads `Condition(s)` into the record, carried; census every chain; the same temporal-rule
  exclusion; I2C's 8 keys resolved.
  **Done.** `TimingConstraintRecord.row_conditions` (a list, omitted when empty) carries the row's cell verbatim from
  a column a header names `Condition`, `Conditions`, `Test condition` or `Test conditions` and nothing else
  (`timing_condition_column`, shared by every layout). A heading row (a condition, no value in any layout, the same
  parameter name as the next row) is inherited, first, by the following rows of that name (`timing_row_conditions`):
  I2C *Table 12*'s `V OL`, `t rCL`, `t fCL`. `is_temporal_rule_input` now also requires no row condition.
  **Measured:** exactly the four chains with such a column move, only in `timing_constraints` and with no record added
  or removed: 93 records gain a condition (I2C 49, I2S 11, OpenCAPI 25G signalling 17, OpenCAPI 4.0 32G PHY 16), every
  one the verbatim cell, read against its row. I2C's ambiguous (table, parameter, mode) keys go 8 -> 0. The column holds
  what the document put there: conditions, note references (OpenCAPI `Note 1`), a remark (I2C `t HD;STA`, *After this
  period, the first clock pulse is generated.*) and I2S derivations (`min > 0.35T = 140 (at typical data rate)`), so
  the carrier promises the cell, not a parsed condition. Consequence: I2S's info finding
  `semantic_temporal_rule_surface_missing` cites 1 input instead of 8, its 11 limits being conditioned.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.f.i — a timing limit keeps the row condition it is stated under`

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.ii`
  Status: `open` (`2026-09-28`; DESIGN + CODE; found by `.3.f`)
  Goal: **a single-condition timing table keeps its condition.** I2C's Hs-mode *Table 12* and Ultra Fast-mode *Tables
  14, 15* head their one `Min | Max` pair with the mode, exactly as USB 3.2 heads a pair with `Limits`, so `.3.f` reads
  all four as scalar: 37 I2C records carry no condition and read as unconditional beside Table 11's per-mode ones.
  Decide from structure whether one spanning label is a condition (a candidate: the caption restates it, *… for
  Hs-mode I2C-bus devices*), and measure.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.iii`
  Status: `open` (`2026-09-28`; DESIGN + CODE; found by `.3.f`)
  Goal: **a grouped timing table that names no parameter column.** USB 3.2 gives per-condition limits with rows as
  parameters in *Table 6-20* (`5GT/s | 10GT/s`), *6-31* (`U1 Exit | U2/Loopback Exit | U3 Wakeup`) and *6-33*
  (`Transmit | Receive`), but its *Table 6-30* fragments put the parameters in the groups (`tBurst | tRepeat`) and
  the conditions in the rows (`Polling.LFPS`, `Ping.LFPS`). None heads its row-label column `Symbol`, `Parameter` or
  `Name`, so `.3.f` reads none (a residual each). Decide from the table's structure which dimension is the parameter,
  or state why it cannot be, and measure.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.i`
  Status: `done` (`2026-09-29`; TOOLING — no product change; found by `.3.c`)
  Goal: **the encoding replay mirror reads a member's value the way the producer does.**
  `scripts/adjudicate_positional_enum_values.py` `replay_table_with_provenance`, the replay every enum instrument
  attributes with (`located_tables`, `measure_uncarried_encoding_tables.py`,
  `adjudicate_decimal_before_binary_encoding_values.py`), parses a value cell with the pre-`KG-ISF-COMPLETENESS.5.viii`
  reader: no quoted literal (`'1010'`, `x'1F'`) and no `.5.xi.a` radix witness. The producer
  (`parse_encoding_member_value`) reads CHI C2C *Table B4.8* `Opcode2[1:0]` rows `00 01 10 11` as 0-3 and mints
  `Opcode2` into IntentIR; the mirror reads `10`, `11` as ten and eleven, so attribution fails and the census script
  reports the table as *minted nothing* (likewise `ActTrigger`, *Table B8.3*). **Impact:** a manufactured measurement:
  `.3.c`'s record calls the two tables withdrawn, and the radix script's "0 replayable member slots" cannot see them.
  Acceptance: the mirror's current-producer value parse is the producer's, single-sourced with the radix script; the
  census and radix numbers re-derived and every published use of them corrected.
  **Done.** `replay_table_with_provenance` reads a current-producer value cell with `parse_encoding_member_value`, a
  line-for-line mirror of the producer's (the `.5.xi.a` radix witness, then `.5.viii`'s quoted literal, then the shared
  reader); a legacy artifact keeps the old reader, which is what its binary used. The radix helpers now live in the
  mirror module and `adjudicate_decimal_before_binary_encoding_values.py` imports them, so one Python copy exists.
  Checked on the cells: C2C `table_0020`/`table_0035` replay `10`, `11` as 2 and 3 and are located; the quoted literals
  give the values the producer's unit tests pin (`'1010'` 10, `'0010'` 2, `'11'` 3). **Re-derived, before -> after,
  every other line of each report identical:** uncarried encoding tables (91) *minted nothing* 73 -> 70, *reaches
  IntentIR* 15 -> 17 (`Opcode2`, `ActTrigger`), *not in IntentIR* 3 -> 4 (`Message credit` mints `Message` with numeric
  member names, withdrawn); radix census replayable member slots 0 -> 10 (all C2C, all binary and already read so by
  the product), total 299 -> 309; positional census attributed enums 511 -> 514, members 17,982 -> 17,998;
  member-name census minting pairs 3,330 -> 3,333; conflated-identity census unchanged. **Published uses corrected:**
  the `.3.c` and `.3.e` records (continuations below); KM `decimal-before-binary-needs-a-notational-witness` (its "0
  replayable" was a manufactured zero since `.3.a` made C2C replayable on `2026-09-26`); the stale `reverify`
  expectations of KM `minted-enum-values-are-mostly-row-indices`, `minted-enums-are-mostly-cross-table-conflations`
  and `enum-member-name-column-needs-adjudication-not-a-score` (stale before this leaf too: 555 enums then, 522 now);
  the book's `pipeline/isf-enums.md` stratum sentence (51 legacy documents holding 530 of 555 enums, and the current
  producer's 25 enums / 136 members, are now 47, 490 of 522, and 32 / 179); and the script's own hard-coded "51 of 78".
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.c.i — the enum replay reads a value the way the producer does`

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c`
  Status: `done` (`2026-09-29`; MEASUREMENT; continued and closed in this part)
  Goal: unchanged (see `program-and-census`). **Corrected by `.3.c.i`:** of the "3 CHI C2C encodings minted and
  withdrawn later", `Opcode2` (*Table B4.8*) and `ActTrigger` (*Table B8.3*) reach IntentIR and are listed only because a
  symbol definition cites no table (`.3.e`); `Message credit` (*Table B5.1*) mints `Message` with numeric member names,
  withdrawn before IntentIR. The earlier reading came from a replay that read `10`, `11` as decimal. So **11 are left
  here**, not 13.
  **Done: the 11, each read against its extractor (`2026-09-29`).**

  | table | what it states | disposition |
  | --- | --- | --- |
  | CHI C2C *B5.1 Message credit field encoding* | `Message credit[2:0]` code -> number of credits (0, 1, 2, 4, 8, 16) | a code-to-quantity map of a message field, not an enumeration (its `Message` enum, numeric member names, is withdrawn before IntentIR): `DOC-INTENT-TAXONOMY.4b` |
  | USB 3.2 *6-14 Compliance Pattern Sequences* | `CP0`-`CP15` -> 8b/10b symbol patterns (`D10.2`, `K28.5`) | refused correctly: a value cell that states no code mints no member; physical-layer test content, no owner |
  | AXI *A2.3 Credited channel signals* | `VALID`, `PENDING`, `RP`, `SHAREDCRD`, `CRDT`, `CRDTSH` as a per-channel pattern | refused by `WIRE-BASED-100.10b` (a base-name template; the instantiations are declared); expanding its prose is `WIRE-BASED-100.10d` |
  | AXI *A13.14 Legal combinations of MMU signals and PAS* | five signals jointly (with don't-cares) determine the meaning | a multi-key relation: `.3.g.iii` |
  | ADIv6 *B2-7 Debug Port programmers' model* | `CDBGPWRUPREQ`, `CDBGPWRUPACK`, `CSYSPWRUPREQ`, `CSYSPWRUPACK` -> CTRL/STAT bits 28-31 (one row rotated) | a recall miss: three of the four wires are declared nowhere: `.3.c.ii` |
  | ADIv6 *C2-5 Using DbgSwEnable* | `Value of DbgSwEnable`: `0b0` Disabled, `0b1` Enabled | a register field's value table (`CSW.DbgSwEnable`, whose record has no enumerated values): `.3.c.iii` |
  | AHB *3-5 Protection signal encoding* | one column per `HPROT` bit, each row one bit's value and meaning | a per-bit encoding: `.3.c.iv` |
  | USB 3.2 *10-12 Port Status Type Codes* | a request's `wValue` codes (`PORT_STATUS` 00H …), no signal | a software request encoding: `DOC-INTENT-TAXONOMY.4b` |
  | USB 3.2 *10-1 Downstream Facing Hub Port State Machine Diagram Legend* | the hub-port state machine's transition events (`connect.detected`, `clear.port.power`, timeouts) | guard vocabulary of a state machine: `.12` |
  | USB 3.2 *10-2 Downstream USB Standard-A Port VBUS Requirements* | (upstream connection x downstream port type) -> VBUS `On` / `May be off` | a requirement matrix: `.3.c.v` |
  | OpenCAPI discovery DVSEC applicability (`table_0030`) | (capability x AFU configuration x function) -> `Required` / `Prohibited` / `Allowed` | a requirement matrix: `.3.c.v` |

  With the encoding adjudication (`program-and-census`, corrected by `.3.c.i`) and the first read of the other 43
  (`docs/research/category-family-census.md`), every one of the 134 typed tables `.3` found on `2026-09-26` is a refusal
  with a reason or a miss with an owner. The population has moved since; routing today's is `.3.c.vi`.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.c — every typed table no record cites has a reason or an owner`

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.ii`
  Status: `done` (`2026-09-29`; DESIGN + CODE; found by `.3.c`; checklist below)
  Goal: **a wire the document names only in its programmers'-model table is declared.** ADIv6 *Table B2-7* maps the
  power-up handshake wires `CDBGPWRUPREQ`, `CDBGPWRUPACK`, `CSYSPWRUPREQ`, `CSYSPWRUPACK` to CTRL/STAT bits 28-31. It
  states no direction and no width, so the declaration reader drops each row (`no_direction_and_no_width`,
  `SIGNAL-DECLARATION-ROW-DROP.1`'s accounting), and three of the four are declared nowhere in ADIv6's IR
  (`CSYSPWRUPACK` is declared from another source). **Impact:** real wires of a wire-bearing specification missing.
  Acceptance: decide how a wire with no stated direction or width is carried (a declaration of existence, or a relation
  to the register bit it mirrors), census every chain's `no_direction_and_no_width` rows that name real wires, and
  measure ADIv6 and the wire golds.
  **Measured `2026-09-29` (MEASUREMENT; the CODE half is below).** The declaration contract refuses a declaration
  with neither direction nor width at both stages on purpose (the EvidenceIR row drop and SemanticIR's
  `NoDirectionAndNoWidth` refusal, `SIGNAL-DECLARATION-ROW-DROP`), so the answer is not a direction-less declaration: a
  wire is declared when the document states its direction or width somewhere, and the question is where. Over the 31
  current chains the row accounting records **382** such dropped rows: **303** name a signal declared from another
  source, **31** name a token declared nowhere and mentioned at least three times. The real wires among the 31, by
  where the document states what the table does not:
  - ADIv6 `CDBGPWRUPREQ`, `CDBGPWRUPACK`, `CSYSPWRUPREQ`: in prose, *"CDBGPWRUPREQ is the signal from the debug
    interface to the power controller"*. No relation reads that form (only `CSYSPWRUPACK` has one, `system power`
    drives it, from another sentence), and a declaration's direction comes from a driving relation
    (`synthesize_directions_from_relations`). **Stays here:** read *X is the signal from A to B* as A drives X and B
    samples X, through the actor-identity gate (`KG-ISF-COMPLETENESS.1d`, a phrase is not an actor), with the relation
    replay over every chain.
  - AXI `RUSERCHK`: its check-signal row states a width, but the cell reads `ceil((USER_DATA_WIDTH USER_RESP_WIDTH)/8)`,
    the `+` lost by the converter at a line wrap: `.3.c.vii` (answered, no code; the residual carries it).
  - ADIv6 `RTCK`: *Table C3-1 JTAG Access Port JTAG port signals* is shifted by the converter (`RTCK In`, `TDI Out`, a
    name in the Notes column), the table `SIGNAL-DECLARATION-ROW-DROP.1b` adjudicated as a garbled body: `.3.c.viii`.
  - ADIv6 `SWDIOTMS` (SWJ-DP pin routing) and RISC-V Debug `TMS`, `TCK`, `TDO`, `TDI` (MIPI connector diagrams): pin
    tables, physical content outside the digital boundary, as `.3.c` disposed of their tables; no owner.
  The rest are parameters (AXI `LOOP_W_WIDTH`, `USER_REQ_WIDTH`, …, correctly not wires), AHB's `HBURST` encoding
  values, AXI's `NSP` (a PAS value), TMC's `MEMD` and `GND`.
  **CODE half, first attempt (`2026-09-29`).** The reader was written and replayed over the 31 chains; ADIv6 alone
  moves. Its 8 from/to relations are right, but the wires, once known, expose three defects in other readers: `.39`
  (a rule's consequent taken from the previous sentence), `.3.c.ii.a` (`debug power`), `.3.c.ii.b` (`debugger
  CDBGPWRUPREQ`), and `KG-ISF-COMPLETENESS.1g` (*This bit controls the CSYSPWRUPREQ signal* makes `bit` an actor).
  `.3.c.ii` lands after `.39`, `.3.c.ii.a` and `.3.c.ii.b` (all done `2026-09-29`) and `KG-ISF-COMPLETENESS.1g`:
  replayed with both fixes, the reader's only junk is the `bit` actor (2 relations), but through them `bit` takes 32
  of ADIv6's behaviours (every statement that mentions a bit), two connectivity conflicts and two temporal
  dependencies. The reader is saved as `.project-data/tmp/erg3cii/erg3cii-wip.patch`.
  **Done.** `from_to_signal_relations`, inside `extract_actor_signal_relations`, reads *X is the [long name] signal
  from A to B* as A drives X and B samples X. The sentence names X itself, so X need not be a known signal yet. X must
  be the sentence's whole subject and signal-shaped (one identifier, not an ordinary or all-lower-case word). Each end
  is at most four words with no *and*/*or*, and passes the relation actor gate. The driving relation then declares X
  through `synthesize_directions_from_relations`, as any drive does. **Measured** after `.39`, `.3.c.ii.a`, `.3.c.ii.b`,
  `.42` and `KG-ISF-COMPLETENESS.1g`: 30 of 31 chains identical. ADIv6 gains only additions, all read: 8 from/to
  relations and `debugger drives CSYSPWRUPREQ`; the four wires declared; the rules' consequent signals from their own
  sentences; two constraints (each acknowledge must be HIGH for as long as its request is asserted), which become two
  conditional temporal rules with the power controller as driver; the section's active-high polarity on three more
  wires. Downstream: actors `debug interface` and `power controller`, one connectivity conflict (`debug interface` and
  `debugger` both drive `CSYSPWRUPREQ`, as the document says), and the blocked adapter declares `CSYSPWRUPREQ` a
  `debugger` output, listing the rest. 7 behaviours join `debug interface` because statements name the *ARM Debug
  Interface*: `KG-ISF-COMPLETENESS.1h`. Over the 78 persisted chains the reader finds 14 relations: ADIv6's 8 and the
  CoreSight Architecture's 6 (`TDI`, `TMS`, `RTCK`, on re-ingest), with `TCK / SWCLK` refused as an alias pair.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.c.ii — a signal named from one actor to another is declared`

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.ii.a`
  Status: `done` (`2026-09-29`; CODE; found by `.3.c.ii`; checklist below)
  Goal: **a relation verb after *and then* takes no subject from the clause before it.** ADIv6's *the power controller
  powers up the system power domain and then asserts CSYSPWRUPACK* publishes `system power drives CSYSPWRUPACK`.
  `active_verb_subject` shares a subject only across a bare *and*/*or*, and only with a governing relation verb. With
  *then* between, and *powers up* no relation verb, the plain reading takes the object before *and*. The same reading
  gives `debug power drives CDBGPWRUPACK` once `.3.c.ii` makes that wire known. Persisted: 1 such relation in 78
  chains. Acceptance: a finite relation verb opened by a conjunction (an adverb such as *then* may stand between) whose
  governing verb is not a relation verb is refused, never read from the object before it; replay every chain.
  **Done, narrowed by the replay.** Refusing every such verb removed a right relation: I2C's *when the target is
  ready for another byte of data and releases clock line SCL* shares its subject with a copula, and the plain reading
  finds `target`. So `active_verb_subject` looks through *then*, and in that fallback `extract_subject_phrase` keeps a
  phrase only when a verb-shaped word follows it before the relation verb (`is_verb_shaped`: a closed-class copula,
  auxiliary or modal, or a lower-case `-s` word; the phrase's own swallowed second word counts). **Measured:** 30 of 31
  chains identical; ADIv6 loses `system power drives CSYSPWRUPACK` and what hung off it: the declaration it made, a
  rule's consequent signal, the constraint and interface record of a wire no longer declared, an unconditional temporal
  rule on that actor, 3 behaviours, and 2 adapter residual lines. No `.isf` text moves. `.3.c.ii` restores the
  constraint and rule with the power controller as driver. Over the 47 legacy chains, a persisted-statement
  approximation finds the rule would remove two more objects read as actors on re-ingest (AXI H.c *acknowledgements*,
  HBM2 *Read data*).

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.ii.b`
  Status: `done` (`2026-09-29`; CODE; found by `.3.c.ii`; checklist below)
  Goal: **an actor phrase ends at its punctuation.** `extract_actor_phrase` strips a word's trailing delimiter and
  keeps reading, so *When CSYSPWRUPREQ is asserted by the debugger, CDBGPWRUPREQ must also be asserted* gives the actor
  `debugger CDBGPWRUPREQ` once `.3.c.ii` makes `CSYSPWRUPREQ` known. The persisted IR of the 31 chains holds 2 more
  actors that span a delimiter (I2C `tFS which`, `HIGH but`); the replay names their path. Acceptance: a word that ends
  in a delimiter ends the phrase; replay every chain.
  **Done.** `extract_actor_phrase` ends the phrase at the word whose delimiter it stripped. **Measured:** no chain
  moves (the I2C actors come from another path and are unchanged; no persisted actor over the 78 chains spans a
  passive phrase's delimiter). With `.3.c.ii`'s reader the ADIv6 sentence reads `debugger drives CSYSPWRUPREQ`.
  Both leaves land in one commit: `EXTRACTION-REPRESENTATION-GAPS.3.c.ii.a — the object before "and then" is no subject`

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
  Status: `open` (`2026-09-29`; CODE; found by `.3.b`)
  Goal: **a signal table SourceIR left untyped declares its signals.** CoreSight TMC r0p1's appendix tables for the ATB
  slave, APB debug, memory and AXI master interfaces (`table_0075`-`table_0080`, `Signal | Type | Description` and
  `Signal | Type | Source | Description`, 55 rows: `ATVALIDS`, `PADDRDBG[11:2]`, `MEMADDR`, `AWADDRM[39:0]`, …) are typed
  `unknown` at ingest, so no declaration reader visits them and none of their signals reaches the catalog, while the
  same header on typed tables declares every row (`SIGNAL-DECLARATION-ROW-DROP.6`). Acceptance: find why the ingest
  classifier leaves them untyped (caption, footnote marks in the name cells, the fused `MEMADDR a Output` cell),
  decide whether the fix belongs at ingest or in the evidence stage's reading of an untyped table, and measure every
  chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.ii`
  Status: `open` (`2026-09-29`; MEASUREMENT; found by `.3.b`)
  Goal: **read the untyped tables no family places.** `.3.b` placed 840 of the 1,165 untyped uncited tables; 325 are
  read here, table by table: 56 register indexes and register-figure pieces (ADIv6 39: `Offset |`, `Component |
  Offset`, `LAR | LSR`), 12 register field tables the field-table reader missed (RISC-V Debug's `Field | Description |
  Access | Reset` with its header row in the body, ADIv6's `DLCR.TURNROUND` one-field value table among them), and a
  residue of 182 header shapes and 48 headerless tables (USB 3.2 43, ADIv6 39, AXI 32 the largest). Acceptance: each
  one a contract family with an owner, a non-contract disposition, or a named recall miss; refine the family script
  where a whole shape moves, and re-measure.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.vii`
  Status: `done` (`2026-09-29`, PROBE/DOC; found by `.3.c.ii`)
  Goal: **a width cell keeps its operator.** AXI's check-signal table gives `RUSERCHK` the width
  `ceil((USER_DATA_WIDTH + USER_RESP_WIDTH)/8)`, and the captured cell reads `ceil((USER_DATA_WIDTH USER_RESP_WIDTH)/8)`;
  the unreadable width leaves the row with neither direction nor width, so `RUSERCHK`, a real AXI wire whose siblings
  (`ARUSERCHK`, `WUSERCHK`, …) are declared, is declared nowhere. Acceptance: find where the `+` is lost (the converter's
  cell text, normalization, or the width reader), census every captured cell with two operands and no operator between
  them, and recover only what the source states.
  **Measured (`2026-09-29`).** The converter drops it. The PDF's text layer (`pdftotext -layout`) wraps the cell as
  `ceil((USER_DATA_WIDTH +` / `USER_RESP_WIDTH)/8)`, and the converter's own output (`backend.json` and the Markdown)
  already reads `ceil((USER_DATA_WIDTH USER_RESP_WIDTH)/8)`: it joined the two lines and lost the `+` at the break.
  SourceIR keeps no text layer to repair it from, so the width is not recoverable without guessing. The leaf's
  premise is also stale. EvidenceIR declares `RUSERCHK` (`… is width …` and `… is output width …`, the Subordinate
  driving it through `.27.d`'s check-signal relation), and SemanticIR refuses both declarations as
  `width_text_unread`. That is the one such refusal on the 31 chains, and it is listed as
  `semantic_unreadable_declaration_width`, so `RUSERCHK` reaches no interface record while its siblings do.
  **The population is one cell.** Across the captured cells of all 78 SourceIR artifacts, an arithmetic group holding two
  parameter operands with no operator between them occurs once, and it is this one; the only other shape match is
  MMU-700's `TCUCFG_ PARTID_WIDTH`, an identifier split at a wrap whose `-` survives.
  **Answered: no code change, and the residual stays the carrier.** The one lever SpecForge holds is the residual's
  second interpretation, keeping a refused declaration's identity and direction without a width.
  `SIGNAL-DECLARATION-ROW-DROP.4d` refused it for identity alone. Narrowed to a declaration that states a direction
  before its `width` keyword, the corpus-wide replay (`specforge replay-declarations`, 78 documents, 0 skipped) would
  admit five identities no other declaration recovers. Two are wires: `RUSERCHK` and CXS's `CXSACTIVEREQ`
  (`output width 1 bit`, a notation `.4d` already measured). Three are not: Avalon's `RESETS` and `BURST` are
  heading rows, and AXI-H's `ACE` is a description row. Two in five does not beat `.4d`'s rule, and naming the
  heading rows as wires is exactly the error it guards. So `RUSERCHK` stays out of the catalog and stays named by
  `semantic_unreadable_declaration_width`. Recovering its width would take a text-layer cross-check at ingest, and a
  population of one cell does not justify that stage. A second instance of the class reopens it.
  Verification: every `structured_tables` header and body cell of the 78 SourceIR artifacts scanned for a parenthesised
  group holding two upper-case parameter operands (one with `_`) separated only by whitespace, beside an operator or a
  call; `specforge replay-declarations --evidence-root generated/evidence_ir --json`, filtered to `width_text_unread`
  refusals whose text states a direction before `width`; `pdftotext -layout` of the AXI PDF (page 271). Read-only, so
  no acceptance checklist applies.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.c.vii`

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.viii`
  Status: `open` (`2026-09-29`; MEASUREMENT then CODE; found by `.3.c.ii`)
  Goal: **a signal table the converter shifted is read or refused as such.** ADIv6 *Table C3-1 JTAG Access Port JTAG port
  signals* (`Signal | Direction | Description | Notes`) arrives with names fused to their direction (`TDI Out`, `TDO In`,
  `RTCK In`, `PORTCONNECTED In`), names missing (`Out | Test Clock | …`, with `TCK TMS` in the Notes column) and the
  direction column holding descriptions. `SIGNAL-DECLARATION-ROW-DROP.1b` adjudicated it a garbled body; `RTCK` (return
  clock, which ADIv6 recommends against using) is declared nowhere. Acceptance: census signal tables whose name cells end
  in a direction word while the direction column holds none, decide whether the fused cell can be split from the
  table's own structure, and keep a JTAG-AP port's direction apart from the DP's same-named signal (`TDI` is an output
  of the JTAG-AP and an input of the DP).
  **Also (`.3.e.i`, `2026-09-29`):** CoreSight TMC *Table 2-1 Memory interface signals* has its name and description
  columns swapped (`Chip enable, active LOW. | MEMCEN`), so `MEMCEN`, `MEMADDR`, `MEMWEN`, `MEMD` and `MEMQ`, the TMC's
  RAM interface, are declared nowhere.
  **Also (`KG-ISF-COMPLETENESS.1g`, `2026-09-29`):** ADIv6 *Table B3-2 JTAG-DP signal connections* merges each
  direction into a neighbouring cell (`DBGTDI | TDI | Input Yes`, `DBGTDO | TDO Output | |`, `DBGTRSTn | TRST Input |
  |`), so `DBGTDO` and `DBGTRSTn` get no declaration from it. `DBGTDO`'s output direction, and `nSRSTOUT`'s from Table
  C3-1, came only from junk relations that `.1g` removes.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iii`
  Status: `done` (`2026-09-29`; CODE; found by `.3.c`; checklist below)
  Goal: **a register field's value table fills the field's enumerated values.** ADIv6 *Table C2-5* heads its code column
  *Value of DbgSwEnable* (`0b0` Disabled, `0b1` Enabled); `CSW@MEM-AP` has a `DbgSwEnable` field whose
  `enumerated_values` are empty. `EXTRACTION-GAP-FIX.11` reads values listed inside a bit-assignment table; this is a
  separate table naming the field in its header. Acceptance: decide structurally when a table's code column names a
  field of a known register (the header names exactly one field), read `value | meaning` pairs into its
  `RegisterFieldEnumRecord`s, never inventing a meaning; measure every chain.
  **Done.** `register_record_surface` ends with `fill_field_values_from_value_tables`: a table whose first header cell
  names one field (`field_named_by_value_header`: the field and at most its register, `CSW.Size`, beside only `value`,
  `of`, `encoding`, `field`) fills that field when exactly one register in scope has it and its values are still empty.
  Each row's first cell is the value (a register literal, or a plain decimal when no cell of the column is two or more
  `0`/`1` digits), with a footnote mark removed as `.10` does for names; the meaning is the second cell's first sentence,
  at most 80 characters; at least two rows must state both. Refused: a second code column (`column_is_mostly_codes`:
  *Table C2-2*, `ERRNPASS` × `ERRSTOP`), a field two registers share, a range cell (`0b110 - 0b111`), a second table
  for a filled field (*Table C2-6*'s `LOW`/`HIGH` for `DbgSwEnable`). The register cites the table.
  **Measured:** EvidenceIR dry-run of all 31 chains: 29 identical, 2 move, only in `register_records`. RISC-V Debug
  `dcsr.cause` gains six values from *Table 8* (`table_0048`); ADIv6 `CSW@MEM-AP` gains `DbgSwEnable` (2, *Table C2-5*),
  `Size` (6, *Table C2-3*) and `AddrInc` (4, *Table C2-1*). Captured-region residuals fall 170 → 169 and 369 → 367
  (`table_0048`; `table_0072`, `table_0077`); `table_0075` was already carried. ADIv6's blocked adapter text gains the
  three field enums and nothing else; RISC-V's is unchanged, since `dcsr.cause` has no bit range (drawn in a figure,
  gap S23, `DOC-INTENT-TAXONOMY.4`).
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.c.iii — a register field's value table fills the field's values`

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv`
  Status: `open` (`2026-09-29`; DESIGN + CODE; found by `.3.c`)
  Goal: **a per-bit encoding is read bit by bit.** AHB *Table 3-5* gives `HPROT` one column per bit (`HPROT[3]
  Modifiable` … `HPROT[0] Data/Opcode`), each row one bit's value and meaning (`HPROT[0]` 0 *Opcode fetch*, 1 *Data
  access*); one enum cannot hold it, since every value repeats across bits (`KG-ISF-COMPLETENESS.5.xiv.a` drops colliding
  values). Acceptance: decide the carrier (a per-bit symbol such as `HPROT[0]`, or a field-level meaning) and whether
  `.isf` can state it; measure AHB and every chain's per-bit tables.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v`
  Status: `open` (`2026-09-29`; DESIGN; found by `.3.c`)
  Goal: **a requirement matrix keeps both of its keys.** USB 3.2 *Table 10-2* (hub upstream connection x downstream port
  type -> VBUS `On` / `May be off`) and OpenCAPI discovery's DVSEC applicability (capability x AFU configuration x
  function -> `Required` / `Prohibited` / `Allowed`) state a modal requirement per pair of keys, and no carrier holds one.
  Acceptance: census such matrices, decide the carrier or state a non-contract disposition (software-visible
  configuration), and measure.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.vi`
  Status: `open` (`2026-09-29`; MEASUREMENT; found by `.3.c`)
  Goal: **today's typed tables no record cites are routed.** `.3.c` closed the 134 of `2026-09-26`. On `2026-09-29` the
  chains list 130 typed tables no record cites, and not all are the same tables: AXI *A12.20 Loopback signal width
  properties* and *A12.21 User signal properties* (parameter tables) are among them, and AXI's presence summaries
  *B2.2*/*B2.3*, which `.3.g` lifted into body rows, still reach no record. Re-derive the population table by table and
  route every table `.3.c` did not.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.e`
  Status: `done` (`2026-09-29`; CODE; continued and closed in this part; checklist below)
  Goal: unchanged (see `program-and-census`). **Re-derived by `.3.c.i`:** the encoding tables whose enum reaches IntentIR
  and are listed only for want of a table citation are **17** (AXI 13, CHI C2C 4: `MsgType`, `ChunkValid`, `Opcode2`,
  `ActTrigger`), not 15; the acceptance's "exactly those 15 residuals go" reads 17. AHB's `HBURST` table (`.3.h`, typed
  `signal_description`) is the same case outside the encoding kind.
  **Done.** Provenance rides on the statement, inside the registered `extracted_statements` field, so no rule field
  and no stage seal moved (all five production digests unchanged): `ExtractedStatement.supporting_table_ids` (omitted
  when empty) is set where an encoding table mints `Enum X M = v.` (`synthesize_encoding_declarations_for_enum`, the one
  minting seam of both enum paths); `SemanticContext` merges it into each statement's table citations;
  `SymbolDefinitionRecord.supporting_table_ids` (omitted when empty) is the union over the *published* members, so a
  member dropped for a colliding value (`KG-ISF-COMPLETENESS.5.xiv.a`) cites nothing; `cited_table_ids` reads symbols.
  A separate EvidenceIR field was rejected: it would have moved the evidence seal of all 31 chains for a fact one
  record type can carry. **Measured:** EvidenceIR dry-run of all 31 current chains: 26 identical, 5 differ only in their
  `Enum` statements (179 of 179 gain their table: AXI 108, CHI C2C 31, AHB 28, ADIv6 8, APB 4). Rebuilt: captured-table
  residuals **-26** (AXI 263 -> 247, CHI C2C 88 -> 84, AHB 60 -> 57, ADIv6 370 -> 367, APB unchanged: its `PPROT`
  members, named `0`/`1`, collide and are withdrawn); symbols otherwise identical; adapters and every `.isf` identical.
  AXI's 16 are the script's 13, plus `BRESP` from the uncaptioned `table_0045` (the script could only call it ambiguous
  with *Table A11.3*), `AWCMO` and `TTL`. **Found:** ADIv6's `In` (members `DPIDR` 0 … `IDENTITY_CODE` 7) is a junk enum
  conflating *DPBANKSEL*'s bank-select rows with *JEP106* field widths; it now cites `table_0030` and `table_0050`, which
  is true provenance and a false "carried": `.3.e.i`.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.e — a symbol cites the table it was read from`

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.e.i`
  Status: `done` (`2026-09-29`; CODE; found by `.3.e`; checklist below)
  Goal: **a stray word does not name an enum.** ADIv6 publishes an enum `In` with members `DPIDR` 0, `DPIDR1` 1,
  `BASEPTR0` 2 (the *DPBANKSEL* register-select table, `table_0030`, whose other columns are bank registers) and
  `CONTINUATION_CODE` 4, `IDENTITY_CODE` 7 (*Table B3-10 JEP106 manufacturer ID code*, `table_0050`, where 4 and 7 are
  field widths). `In` is not a declared ADIv6 signal. **Impact:** a false symbol in IntentIR, and since `.3.e` two
  captured tables count as carried by it. Acceptance: find which path names the enum `In` (the signal-anchor scan
  reads `unknown` tables) and why a non-signal word anchors it, remove it by a structural rule, census every chain's
  anchors that name no declared signal, and measure ADIv6 and the wire golds.
  **Done.** The anchor was a *known signal name*: `collect_signal_names_from_tables` seeds that set with each signal-table
  row's first word, declared or not, and ADIv6's converter-shifted *Table C3-1* (`TDI Out`, a bare `In`) put `In` and
  `Out` in it; the signal-anchor scan then found `In` in two tables' identity text. The seed now skips a word
  `inferred_name_is_an_ordinary_word` recognises (initial capital, lower-case remainder), the orthography rule
  `ACTOR-NOUN-RELATION-DECLARATION.1` measured at 0 of 1,605 table-declared names; its doctrine keeps it off table
  declarations, and the seed declares nothing, while a declared wire re-enters through its declaration statement.
  **Measured:** EvidenceIR dry-run of all 31 chains: 29 identical, 2 move and everything they lose hangs off the
  skipped words. ADIv6: the five `Enum In` statements (by text, nothing else), 76 relations on `In`/`Out`, 21 hints,
  the `Out` polarity, one conflict; downstream 66 actors whose only relations those were (junk phrases, and real
  components such as `MEM-AP` whose one relation was false), 165 behaviours whose every actor was one of them (153 more
  keep their statement and shed only such actors), 2 connectivity records, the `In` symbol and its `.isf` type, and
  `table_0030`/`table_0050` back among the uncarried. TMC (*Table 2-1*, name and description columns swapped): 16
  relations and 13 actors on `Chip`/`Write`/`Address`, two polarities, two conflicts, three behaviours, and its
  `table_0004` uncarried again. Temporal rules and interfaces (apart from renumbered statement ids) unchanged.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.e.i — a word in a signal table's name column is not a signal`

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.f.i` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — I2C after `.3.f`: 8 (table, parameter, mode) keys hold two records each with different
  limits (`I OL` Fast-mode 3 and 6 mA; `t HD;DAT` Standard-mode 5.0 and 0 µs; Table 12's `V OL`, `t rCL`, `t fCL`,
  `t fDA`; Table 13's `C b`), 16 timing tables in 4 current chains head a `Condition(s)` column.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source.rs` `TimingTableColumns` had no condition column
  and `TimingConstraintRecord` no carrier, so `synthesize_timing_constraints` (`ir/evidence.rs`) dropped the cell. RED,
  each alone with the file restored byte-identically: without the heading inheritance
  `a_timing_limit_keeps_the_row_condition_it_is_stated_under` fails; without the predicate's row clause
  `a_per_condition_limit_is_not_a_temporal_rule_input`, that test and
  `a_per_condition_timing_limit_is_carried_but_derives_no_temporal_rule` fail; accepting any header word starting
  `condition` fails `a_timing_table_names_its_condition_column_by_its_header_alone`.
- [x] **ADDRESSED (verified)** — I2C, I2S and both OpenCAPI PHY signalling chains rebuilt stage by stage, each stage
  validated, the adapter last: 93 records gain their condition, I2C's ambiguous keys 8 -> 0.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 current chains against their persisted artifacts (current at
  HEAD): 27 identical, the 4 differ only in `timing_constraints`, no record added, removed or otherwise changed.
  Rebuilt, their SemanticIR and IntentIR differ only there and their adapters only in proof surfaces;
  `check_chain_currency.sh` 31 of 31 current at every stage after the rebuilds. WIRE-BASED-100: all 11 seeds identical to HEAD's (temporary paths masked).
  `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,773 passed, 0 failed.
  Every stage's production digest unchanged. `flow_census.json` rolled forward under this leaf (+2 functions, +10
  decision sites, +36 helper edges, +1 macro; boundary unchanged).
- [x] **GENERICITY (ADR 0006)** — a closed header vocabulary and row structure (same name, no value); no document,
  parameter or condition text.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (the timing-table section gains the row condition,
  with I2C's Table 12 heading rows as the example), `pipeline/semanticir.md` and `pipeline/intentir.md`; KM
  `timing-table-structural-authority` amended. Nothing described the dropped column.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.e` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — AXI IntentIR publishes `AxBURST` from `table_0025` while `captured_region_residuals` lists
  `table_0025` as `no_canonical_carrier_for_captured_region`; `measure_uncarried_encoding_tables.py`: 17 encoding tables
  *minted, reaches IntentIR*.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `synthesize_encoding_declarations_for_enum`
  minted `Enum X M = v.` with no span and no table, and `crates/specforge/src/ir/semantic.rs` `cited_table_ids` read no
  symbol surface. RED, each alone with the files restored byte-identically: the minting seam citing no table fails
  `header_sourced_enum_name_mints_the_document_grounded_members`; the context ignoring the statement field, or citing
  every contributing statement instead of the published members, fails `an_enum_cites_the_tables_of_its_published_members`;
  the gatherer ignoring symbols fails that test and `cited_table_ids_gathers_every_declared_table_provenance_surface`.
- [x] **ADDRESSED (verified)** — five chains rebuilt stage by stage, each stage validated, the adapter last: 179 statements
  name their table, captured-table residuals -26 (AXI 16, CHI C2C 4, AHB 3, ADIv6 3).
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 26 identical, 5 differ only in their `Enum` statements'
  new field; rebuilt SemanticIR/IntentIR differ only in `symbol_definitions[].supporting_table_ids` and
  `captured_region_residuals`; every adapter and `.isf` identical. `check_chain_currency.sh` 31 of 31 current at every stage after the rebuilds. WIRE-BASED-100:
  all 11 seeds identical to HEAD's (temporary paths masked). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 170 / 1,774 passed, 0 failed. Every stage's production digest unchanged.
  `flow_census.json` rolled forward under this leaf (+0 functions, +1 decision site, +44 helper edges, +1 macro).
- [x] **GENERICITY (ADR 0006)** — provenance only: the table a statement was synthesized from; no document, table or
  enum name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` gains *Which table an enum was read from*; `pipeline/semanticir.md`'s
  provenance-vocabulary paragraph names symbols; KM `captured-region-residual-carrier` amended. Nothing described the
  missing citation.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.e.i` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — ADIv6 IntentIR publishes enum `In` (`DPIDR` 0 … `IDENTITY_CODE` 7) and its `.isf` text
  `(type In (bits 3))`; EvidenceIR holds 76 relations and 21 hints on `In`/`Out`; `In` is not a declared ADIv6 signal.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `collect_signal_names_from_tables` kept any
  identifier as a row's first token, so the shifted *Table C3-1*'s `In` and `Out` became known signals and the
  signal-anchor scan named an enum after `In`. RED, with the file restored byte-identically: dropping the new clause fails
  `a_word_in_a_signal_tables_name_column_is_not_a_known_signal`.
- [x] **ADDRESSED (verified)** — ADIv6 and TMC rebuilt stage by stage, each stage validated, the adapter last: no `In`
  symbol, no relation, hint, polarity or connectivity on `In`, `Out`, `Chip`, `Write`, `Address`.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 29 identical; every fact the other 2 lose is read and hangs
  off a skipped word (statements compared by text, relations by triple). `check_chain_currency.sh` 31 of 31 current at every stage after the rebuilds.
  WIRE-BASED-100: all 11 seeds score identically (SWD's report prints 20 stored relations instead of 96, its scores
  unchanged). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D
  warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,775 passed, 0
  failed. `flow_census.json` rolled forward under this leaf (+1 helper edge).
- [x] **GENERICITY (ADR 0006)** — orthography only (an initial capital over a lower-case remainder), no word list, no
  document or table name.
- [x] **LOCKSTEP** — book `pipeline/evidence-failure-modes.md` *A signal called `Input`* gains the known-signal seed;
  `pipeline/isf-enums.md`'s `.3.e` paragraph records the enum's removal; KM `table-name-cells-seed-known-signals` (new)
  and `captured-region-residual-carrier`.


## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.iii` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — ADIv6 IntentIR `CSW@MEM-AP` holds `DbgSwEnable` `[31]`, `Size` `[2:0]` and `AddrInc`
  `[5:4]` with no `enumerated_values`, while *Tables C2-5*, *C2-3* and *C2-1* state them. `table_0072` and `table_0077`
  are captured-region residuals, and RISC-V Debug's *Table 8* (`cause encoding`, `table_0048`) is one too.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `register_record_surface` reads a field's
  values only from its own bit table (`EXTRACTION-GAP-FIX.11`), and no reader takes a separate table whose header
  names the field. RED, with the file restored byte-identically after each mutation, each failing
  `a_field_value_table_fills_the_one_field_it_names`: dropping the second-code-column refusal, the filled-field guard,
  the radix-ambiguity guard, the one-owner guard, or the first-sentence cut.
- [x] **ADDRESSED (verified)** — RISC-V Debug and ADIv6 rebuilt stage by stage, each stage validated, the adapter last:
  `dcsr.cause` holds six values, `CSW@MEM-AP` 2 + 6 + 4. ADIv6's `.isf` text gains `(field addrinc (bits 5 4) (enum
  (auto_increment_off 0) …))` and the two others, and nothing else, and its `lowering_status` is unchanged.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 29 identical, 2 move only in `register_records`;
  SemanticIR and IntentIR move only in `register_records` and `captured_region_residuals` (170 → 169, 369 → 367).
  `check_chain_currency.sh` 31 of 31 current at every stage after the rebuilds. WIRE-BASED-100: all 11 seeds identical;
  `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,777 passed, 0 failed.
  `flow_census.json` rolled forward under this leaf (+4 functions, +25 decision sites, +48 helper edges).
- [x] **GENERICITY (ADR 0006)** — a header grammar (one field name beside `value`/`of`/`encoding`/`field`), a
  uniqueness test over the document's own registers and a literal shape test. No document, table or field name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` *A field's value table* (new); KM
  `field-value-table-fills-register-field` (new) and `captured-region-residual-carrier`; gap ledger S2's open list.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.ii.a` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — ADIv6 EvidenceIR holds `system power drives CSYSPWRUPACK` from *the power controller
  powers up the system power domain and then asserts CSYSPWRUPACK*, and through it `Signal CSYSPWRUPACK is output.`, an
  unconditional temporal rule on `system power` and the actor itself; 1 such relation in the 78 persisted chains.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `active_verb_subject` shared a subject only
  across a bare *and*/*or* and only with a governing relation verb; otherwise the plain reading took the determiner
  phrase before the conjunction, the object of *powers up*. RED, with the file restored byte-identically after each:
  dropping the verb-after-phrase test or the *then* look-through fails
  `a_coordinated_relation_verb_takes_no_subject_from_the_object_before_the_conjunction`.
- [x] **ADDRESSED (verified)** — ADIv6 rebuilt stage by stage, each stage validated, the adapter last: no `system power`
  actor, relation, port, rule or behaviour; the `.isf` text is unchanged.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains with the HEAD and new binaries: 30 identical; ADIv6 loses
  one relation and only what hung off it, every removed fact read downstream. A first, wider rule (refuse every such
  verb) also removed I2C's right `target drives SCL`; the landed rule keeps it. `check_chain_currency.sh` 31 of 31
  current at every stage after the rebuild. WIRE-BASED-100: 10 seeds identical; `seed_swd` loses one false-positive
  constraint (`signal_constraint` tp=0 fp=1 fn=1 → tp=0 fp=0 fn=1: the `must_be_high` it predicted was scored against
  a `must_be_asserted` gold, `.40`). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 /
  1,780 passed, 0 failed. `flow_census.json` rolled forward under this leaf (+1 function, +11 decision sites, +33
  helper edges, `.3.c.ii.b` included).
- [x] **GENERICITY (ADR 0006)** — a conjunction, an adverb, closed-class verbs and a word shape; no document, actor or
  signal name.
- [x] **LOCKSTEP** — book `domain/actor-connectivity.md` (a verb after *and*); KM
  `an-ellipted-subject-is-never-the-object-before-and` (new).

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.ii.b` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — with `.3.c.ii`'s reader, ADIv6's *When CSYSPWRUPREQ is asserted by the debugger,
  CDBGPWRUPREQ must also be asserted* gives the actor `debugger CDBGPWRUPREQ`; no persisted actor over the 78 chains
  spans a passive phrase's delimiter today.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `extract_actor_phrase` stripped a word's
  trailing delimiter and kept reading. RED, with the file restored byte-identically: dropping the delimiter stop fails
  `a_passive_actor_phrase_ends_at_its_punctuation`.
- [x] **ADDRESSED (verified)** — replayed with `.3.c.ii`'s reader the sentence reads `debugger drives CSYSPWRUPREQ`.
- [x] **NO REGRESSION** — the same 31-chain dry-run as `.3.c.ii.a`: this leaf moves no chain; the gates, seeds,
  `kg-bench` and currency are `.3.c.ii.a`'s, run on both leaves together.
- [x] **GENERICITY (ADR 0006)** — punctuation only.
- [x] **LOCKSTEP** — book `domain/actor-connectivity.md`; KM `an-ellipted-subject-is-never-the-object-before-and`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.ii` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — ADIv6 declares `CSYSPWRUPACK` only through a junk relation and `CDBGPWRUPREQ`,
  `CDBGPWRUPACK`, `CSYSPWRUPREQ` nowhere, though *CDBGPWRUPREQ is the signal from the debug interface to the power
  controller* (and three sentences like it) state each wire's two ends; 382 dropped rows census in the measured half.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `extract_actor_signal_relations` read no
  *from A to B* form, and read relations only for known signals, which these wires were not. RED, with the file restored
  byte-identically: skipping the reader's relations fails
  `a_signal_from_one_actor_to_another_is_driven_by_the_first_and_sampled_by_the_second`.
- [x] **ADDRESSED (verified)** — ADIv6 rebuilt stage by stage, each stage validated, the adapter last: the four wires
  declared, two conditional temporal rules on `power controller`, no junk actor added.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains with the HEAD and new binaries: 30 identical, ADIv6 only
  additions, every one read, downstream too. The first attempt's junk (`bit`, `debug power`, `debugger CDBGPWRUPREQ`, a
  consequent from the wrong sentence) was fixed first by `.39`, `.3.c.ii.a`, `.3.c.ii.b` and `KG-ISF-COMPLETENESS.1g`.
  `check_chain_currency.sh` 31 of 31 current at every stage after the rebuild. WIRE-BASED-100: 10 seeds identical;
  `seed_swd` regains the one `CSYSPWRUPACK` prediction `.3.c.ii.a` had removed (`signal_constraint` tp=0 fp=1 fn=1, as
  at `be58b45b`; the `must_be_high`/`must_be_asserted` disagreement is `.40`). `kg-bench` 158/158. `cargo fmt --check`
  clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib
  --exclude specforge-production-graph` 474 / 170 / 1,784 passed, 0 failed. `flow_census.json` rolled forward under
  this leaf (+2 functions, +24 decision sites, +24 helper edges, +2 semantic macros).
- [x] **GENERICITY (ADR 0006)** — a copula, an article, *signal from … to …*, and the relation actor gate; no document,
  actor or signal name.
- [x] **LOCKSTEP** — book `domain/actor-connectivity.md` (*Where graph facts come from*); KM
  `a-signal-from-one-actor-to-another-names-both-ends` (new); gap ledger S2's open list; `KG-ISF-COMPLETENESS.1h` opened.

