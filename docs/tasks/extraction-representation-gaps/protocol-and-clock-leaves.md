# EXTRACTION-REPRESENTATION-GAPS — protocol and clock leaves

- Part ID: `protocol-and-clock-leaves`
- State: `active`

Open records continue here after `table-reading-leaves` crossed its 90% rollover on `2026-10-28`. `.68.a` also lands
here so the old part can seal at `1dec2b0a9`; every moved record and its checklist is preserved exactly. New leaves
and dated entries go to this part.

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
- ID: `EXTRACTION-REPRESENTATION-GAPS.72`
  Status: `open` (`2026-10-23`; DESIGN; opened by `.65.b`)
  Goal: **a note an address or timing table cites reaches its record.** **Reproduce:** I2C's address table (page 16:
  `general call address [1]`, `CBUS address [3]`; page 27) and its timing tables (pages 43-49: `data hold time [1]`,
  `C b = 400 pF [2]`, a cell of `[1]` alone) cite list notes `[1]`-`[11]` printed beside them, which `.65.b` reads,
  and I2S's timing table (page 7) cites its notes in cells that are a mark alone; only matrices and combination keys
  bind a note. Acceptance: census the tables, outside matrices and
  combinations, whose title or cell closes with a mark a nearby note explains, decide each record's carrier (a timing
  constraint's condition, a reserved address's note) and measure every chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.73`
  Status: `open` (`2026-10-23`; DESIGN; opened by `.65.b`)
  Goal: **a matrix's continuation joins it.** **Reproduce:** I2C *Table 3* continues on page 8 (`table_0005`, caption
  `... ...continued`), but the converter put its column titles in the first header row (`| Single controller |
  Multi-controller | Target [1]`) and the `Feature | Configuration` row second, so the matrix reader neither joins it
  to `matrix_table_0001` nor reads it, and its `Device ID` row is lost. Acceptance: census matrix continuations
  refused or unjoined, decide when an inverted header still names the same columns, and measure every chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.71`
  Status: `open` (`2026-10-23`; DESIGN; opened by `.65`)
  Goal: **a cell that names a note reaches the note.** **Reproduce:** OpenCAPI's 25 Gbps and 32 Gbps PHY signaling
  specifications put `Note 1` … `Note 3` in table cells and print the notes as a `1.` list after the table (pages
  20-21 and 29-30), and nothing reads them. Acceptance: census cells naming a note and the lists that follow their
  tables, decide the binding (a `Note N` cell and an `N.` list item after the same table, never a step list) and the
  carrier, and measure every chain.
- ID: `EXTRACTION-REPRESENTATION-GAPS.68.a`
  Status: `done` (`2026-10-28`; CODE; opened by `.68`; checklist below)
  Goal: **the matrix reader keeps prose and value cells.** Acceptance: implement `.68`'s fallback and the two cell
  classes, preserving every existing matrix; read exactly the ten measured tables (45 rows, 213 cells) in the six
  chains, carry them to IntentIR and name each in its adapter residual, leave every `.isf` unchanged, and measure all
  31 chains.
  **Done `2026-10-28`.**
  - **Reader.** `read_value_and_prose_matrix_tables` runs after the unchanged mark/modal reader. Two header levels
    and their largest leading blank run give independent keys; `MatrixCellClass` gains `Value` and `Prose`, and each
    `MatrixRow.cells[i]` remains the exact statement at `row.keys` x `column_keys[i]`. Timing layouts, navigation,
    one-header maps and tables another reader claims are refused; continuations and notes use the existing paths.
  - **Measured.** Fresh before/after EvidenceIR dry-runs: 25 chains identical, six gain exactly `.68`'s ten matrices,
    45 rows and 213 non-empty cells. Existing matrices and unrelated fields are unchanged. The six rebuilt chains
    carry them to IntentIR and gain ten adapter residuals (AHB 43 -> 44, OpenCAPI DLL 2 -> 4, discovery 5 -> 6,
    I2C 9 -> 10, USB 3.2 102 -> 106, Wishbone 8 -> 9). Every prior residual and every `.isf` is byte-identical.
  Verification: focused test and seven RED controls; 31-chain replay, six ordered rebuilds and currency; 11 seeds;
  `kg-bench`; product tests and clippy; measurements in `.project-data/tmp/erg-68a/`
  Commit: `EXTRACTION-REPRESENTATION-GAPS.68.a — the matrix reader keeps prose and value cells`
- ID: `EXTRACTION-REPRESENTATION-GAPS.68.a.i`
  Status: `done` (`2026-10-28`; DOC; opened by `.68.a`'s archive rollover)
  Goal: **retire the consumed archive ceiling authority.** Acceptance: once `.68.a` has sealed the new archive
  bounds, remove its single-use `increase` record, prove the clean-tree live-document gate green, and advance to
  `.69`. The registry must not bank the authority after the bounds it admitted are committed.
  **Done `2026-10-28`.** With the new bounds committed, `LIVE-DOC-SIZE` reported
  `erg_task_evidence_archived_parts has unused or banked ceiling-increase authority` (RED). Removing that record left
  only the registry control record, and the same gate passed (GREEN). The ten archived parts and their measured
  bounds are unchanged.
  Verification: `bash scripts/check_doctrines.sh --only LIVE-DOC-SIZE` RED before, GREEN after removal
  Commit: `EXTRACTION-REPRESENTATION-GAPS.68.a.i — retire the consumed archive ceiling authority`
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
  Status: `done` (`2026-10-28`; DESIGN; opened by `FSMGEN-REQUESTS.9.a`)
  Goal: **a protocol whose only clock is a bus signal grounds its actor's clock, or says why not.** **Reproduce:**
  RISC-V debug, ADIv6, I2C and I2S are blocked by nothing but *no source-grounded system clock/reset contract*; each
  documents a bus clock (`TCK`, `SWCLK`, `SCL`, `SCK`) but no system clock, so the adapter writes
  `(clock __specforge_unresolved_clock)`. A bus clock is an input on one side and a generated output on the other (an
  I2C controller drives `SCL` from a clock the specification does not name). Acceptance: census the 26 blocked chains'
  clock evidence, decide per role whether a bus clock is the actor's `.isf` clock (and what reset then applies, given
  `FSMGEN-REQUESTS.9.a`'s refusal of an invented reset), and measure every chain's adapter.
  **Measured `2026-10-28`** (all 26 blocked adapters; `.project-data/tmp/erg-69/census.py`, `census.json`). Twenty-two
  have no bus-clock candidate. The other four have no complete role-and-reset contract:
  - RISC-V debug's `TCK` is driven by the debug adapter; `nRESET` resets cores and peripherals, not the debug logic,
    while `nTRST` initializes the DTM separately. No typed interface signal or single applicable reset exists.
  - ADIv6's `TCK`/`SWCLK` are protocol clocks; the SWD interface includes no reset, `DBGTRSTn` resets only the TAP
    state machine, and `nSRST` is a system reset with different domain behavior. No complete contract follows.
  - I2C's controller generates `SCL` (and the mode-specific `SCLH`/`USCL`) from an unnamed implementation clock, and
    no reset is stated. The initiator is still the phrase `optional_open_drain`, owned by `.7e.i`.
  - I2S's controller generates `SCK`, no actor is related to it, and no reset is stated.
  **Decided.** No current bus clock is an initiator's own grounded `.isf` clock with an applicable reset, so no new
  contract or emission is justified and no production code changes. `.7e.i` remains the implementation route after
  `KG-ISF-COMPLETENESS.1` grounds these actors. No FSMGen reset default is invented.
  Verification: all 26 adapter manifests reviewed, five emitted chains excluded from the blocked set; no source change
  Commit: `EXTRACTION-REPRESENTATION-GAPS.69 — no bus clock is an initiator's grounded system clock (DESIGN)`
- ID: `EXTRACTION-REPRESENTATION-GAPS.61.c`
  Status: `open` (`2026-10-23`; CODE; opened by `.61`; waits for `KG-ISF-COMPLETENESS.2a.vii.c.vi`)
  Goal: **a member's enabling property joins the property's own record.** Acceptance: once boolean properties are
  records, join each name in an `enabled_by` to its record, leave a name no record declares unjoined, and measure AXI's
  22 properties.
- ID: `EXTRACTION-REPRESENTATION-GAPS.70`
  Status: `done` (`2026-10-28`; DESIGN; opened by `.61`; opened `.70.a`)
  Goal: **an encoding member's support requirement is read.** **Reproduce:** ADIv6 *C2-3* (`Size value, CSW.Size |
  Memory access size | Support required?`) states `Yes a` for `Word` and `No` for the other sizes; the `Size` enum
  keeps the codes and drops the column. Acceptance: census encoding tables with a support-requirement column, decide
  the carrier (and how the footnote mark joins its note), and measure every chain.
  **Measured `2026-10-28`** (31 chains). Two ADIv6 tables have such a column: *C2-1* `AddrInc` (three non-reserved
  rows) and *C2-3* `Size` (six), nine support requirements in all. `Size` has three enum members in IntentIR;
  `AddrInc` has none. C2-3's `Yes a` is note `a` (only word supported: the field is read-only and returns `0b010`);
  larger rows carry note `b` (supported by the Large Data Extension). C2-1 states `Always.` and a conditional fallback
  to `0b00` when packed transfers are unsupported, not an enabling property.
  **Decided.**
  - **One row-level `SupportRequirementRecord` per non-reserved row**, in EvidenceIR `support_requirements`: table ids,
    encoding and meaning as written, the support cell as written, and `notes` (`mark` and exact note text). A `-` row
    states no requirement. SemanticIR and IntentIR carry the records; an optional symbol id and member name join only
    by exact table and literal equality, never row position or spelling. C2-1 needs no such link to be useful.
  - **Notes join by their own marks.** Every cell in the row can cite a note: `Yes a` on the support cell, `0b011 b`
    on the encoding. Join each trailing mark to the footnote with that mark on the table's pages, never by position.
  - **Not lowered.** An `.isf` enum has no support-condition form, so no requirement becomes unconditional membership
    or a guessed enabling property. Each record gets an adapter residual naming its encoding, meaning, support, notes
    and any member link; `.isf` stays unchanged.
- ID: `EXTRACTION-REPRESENTATION-GAPS.70.a`
  Status: `open` (`2026-10-28`; CODE; opened by `.70`)
  Goal: **support requirements and their notes reach IntentIR.** Acceptance: implement the record, its optional exact
    member join and note binding, register the new field, carry it to IntentIR and name each record in its adapter
    residual; read exactly C2-1's three and C2-3's six rows, preserve every existing member and `.isf`, and measure all
    31 chains.
## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.68.a` (CODE) — DONE `2026-10-28`

- [x] **REPRODUCE / MEASURE** — `.68`'s 31-chain census: 32 candidates, 10 two-way value/prose matrices in 6
  documents (45 rows, 213 non-empty cells), none carried by `matrix_tables`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` admitted only marks and modal words, and
  `MatrixCellClass` had no value/prose variant. RED 7/7, the file restored byte-identically: claimed-table precedence
  removed, one-header maps admitted, header-derived labels discarded, timing/navigation admitted, continuations not
  joined, values and prose misclassified; each fails `a_two_way_table_of_values_or_prose_is_a_matrix`.
- [x] **ADDRESSED (verified)** — exactly the ten measured matrices above, with every cell and key exact, carried to
  IntentIR and named in the six adapters.
- [x] **NO REGRESSION** — fresh before/after EvidenceIR dry-runs: 25 chains identical; existing matrices and all
  unrelated fields unchanged. Six ordered rebuilds validated; every prior residual and `.isf` byte-identical.
  `check_chain_currency.sh` 31 of 31 current at every stage. All 11 seed reports identical to the baseline (temp paths
  masked); `kg-bench` 158/158. `cargo fmt --all -- --check` and clippy clean; product tests 475 / 171 / 1,945 pass;
  the two xtask checks whose authorities moved pass after refresh. `mdbook test` and `mdbook build` pass with the
  existing `<operation>` warning unchanged. `flow_census.json` rolled (+3 functions, +46 decision sites, +100 helper
  edges, +1 macro); the genericity boundary is unchanged.
- [x] **GENERICITY (ADR 0006)** — header levels, blank runs, cell grammar, timing roles and reader ownership; no
  document, table or vendor vocabulary and no synthesized statement.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A two-way table and the statement at each pair of keys*); research
  `category-family-census.md`; KM `a-two-way-table-of-marks-is-a-matrix`; root frontier `.68.a.i`, then `.69`,
  and the evidence contract.
  The old table-reading part sealed at `1dec2b0a9`; this leaf and the open records continue in
  `protocol-and-clock-leaves`.
