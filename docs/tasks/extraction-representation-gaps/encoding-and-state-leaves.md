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
  Status: `open` (`2026-10-21`; CODE; split from `.3.g.iii.b`)
  Goal: **a jointly keyed table is a value combination, and the states it keys are protocol states.**
  **Reproduce:** APB `table_0010`, AXI-L `table_0178` and `table_0239` + `table_0240`, and LTI `table_0040` have no
  record, and AXI-L's `BTAGMATCH` and `SYSCOACK` enum statements repeat codes. Acceptance:
  - add EvidenceIR `value_combinations` (family `evidence.protocol_structure`) as `.3.g.iii.b` decides, with its
    manifest surface;
  - refuse the single-column reading of a table read here;
  - merge the keyed states into `protocol_states` with `supporting_combination_ids`;
  - make the census script count a cited table as typed;
  - measure the 55 tables, the 11 withdrawn statements and the 12 states on every chain, then rebuild and re-seal.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.ii`
  Status: `open` (`2026-10-21`; CODE; split from `.3.g.iii.b`)
  Goal: **a value combination reaches IntentIR and the ISF adapter.** **Reproduce:** after `.3.g.iii.b.i`,
  `value_combinations` stops at EvidenceIR. Acceptance:
  - carry the record verbatim through SemanticIR and IntentIR;
  - join each label value to its key's encoding codes where the key's enum states the label (`StatedValueEncoding`);
  - state each combination as an ISF adapter residual packet that names what it lacks;
  - run an FSMGen strict probe to decide whether a combination that names a member lowers to a decode, and open
    that leaf.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.c`
  Status: `open` (`2026-10-21`; DESIGN; split from `.3.g.iii`)
  Goal: **a code column that maps several members to one code is carried as what it is.** **Reproduce:** AXI *A4.4*:
  `ARCACHE` `0b1010` is Write-Through No-Allocate and Write-Allocate on reads; today's `ARCACHE` enum keeps 6 of 12
  rows and `AWCACHE` none. Acceptance: decide whether a repeating column is an aliasing encoding (code -> members) or
  stays a residual. Decide whether today's 6-member enum is kept, re-read or withdrawn, and how the parenthesized
  alternates (*0b1110 ( 0b0110 )*) read. Measure AXI and the wire golds.

- ID: `EXTRACTION-REPRESENTATION-GAPS.57`
  Status: `open` (`2026-10-21`; DESIGN; found by `.3.g.iii`)
  Goal: **a table whose every row the converter put in its header has one header row.** 137 tables in 16 documents
  have more than one header row and no body row (the Cortex-A76 guide 37, AXI 26, USB 3.2 23, ADIv6 14), so every
  table reader sees no data. AXI *A4.7* is one. **Reproduce:** `generated/source_ir/ihi0022…/source_ir.json`
  `table_0061` (7 header rows, 0 body rows). Acceptance: census the 137 by shape (front matter, data tables, true
  multi-row headers), decide whether SourceIR normalizes them or each reader does, from what the converter states,
  and measure every chain.
