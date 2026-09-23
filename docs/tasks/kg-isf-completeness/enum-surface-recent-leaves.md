# KG-ISF-COMPLETENESS — enum surface recent leaves

- Part ID: `enum-surface-recent-leaves`
- State: `active`

## Post-migration record

- ID: `KG-ISF-COMPLETENESS.5.vi.a.1` · Status: `done` (`2026-09-21`; CORRECTION, docs-only; opened and
  closed by an audit of `.5.vi.a`'s own lockstep) · Goal: **re-pin the published figures `.5.vi.a`
  moved.** The leaf verified its own oracles — the 27-chain replay diff, `kg-bench` 156/156,
  WIRE-BASED-100 byte-identical on rebuilt golds — and every one of them passed. None of them looks at
  the surfaces that PUBLISH the census, and `.5.vi.a` moved the census: minted enums **545 -> 552**,
  attributed members **18,969 -> 19,009**, fully-stated enums **100 -> 107**. Three surfaces were left
  stating the old numbers, found by re-deriving rather than by any gate:
  1. **the book** (`pipeline/isf-adapter.md`) said in the PRESENT tense that the current pipeline mints
     *"15 enums and 79 members"* and the corpus holds *"545 enums / 21,538 members"*. Re-derived:
     **22 enums / 119 members** replayable, **552 / 21,578** total. It understated by 47% the very
     improvement this leaf shipped, in the owner's only window.
  2. **`[[minted-enum-values-are-mostly-row-indices]]`'s `reverify`** declared 4 of its 8 expected
     values as they stood before this leaf, so the next session running it would have read a
     regression that had not happened. That is precisely the stale-detection leg
     `CLAIM_VERIFICATION.md` requires, defeated by its own owner.
  3. **`docs/research/enum-value-provenance.md`**'s notation table listed binary as 450 cells when the
     landed reader handles **455** — it omitted the 5 grouped-literal cells (`'0000 0000'`), which the
     implementation reads and a unit test pins, so the three rows summed to 824 of 829 quoted cells.
  **THE RULE THIS EARNS, stated for the next producer change: a change that moves a CENSUS must
  re-pin every surface that publishes that census, not only the surfaces the change edited.** A slice's
  own acceptance oracles answer *"did I break the product?"*; they never answer *"did I falsify a
  number someone else published?"* `.5.viii` had the same exposure and escaped it only because its
  replay was byte-identical, which is luck, not method.
  Non-goal: re-opening `.5.vi.a`'s verdict — every measured claim in it re-derives unchanged
  (+40 statements, 2 documents, 0 removed, 26 of 40 exactly correct).
  Verification: `scripts/adjudicate_positional_enum_values.py` re-run — 552 / 545 / 7 / 19,009 /
  10,139 / 224 / 214 / 107, matching the card's re-pinned `reverify` literally; the 829 quoted cells
  tile as 450 + 5 + 353 + 21; `.5.xi` (1,173 / 158), `.5.xii` (77, 0 in the quoted branch),
  `.5.xiii` (262 / 34) and the 357-of-10,139 headline all re-derive UNCHANGED.

- ID: `KG-ISF-COMPLETENESS.5.xi` · Status: `done` (`2026-09-22`; MEASUREMENT — the adjudication is the
  deliverable and it REVERSED the premise the leaf was opened on; it opens `.5.xi.a`) · Goal: decide
  the radix of an UNQUOTED all-`0`/`1` encoding cell on a measurement rather than on the arm ordering.

  **THE OPENING PREMISE WAS WRONG IN ITS CONCLUSION, NOT ITS COUNTS.** `.5.xi` was opened on a
  corpus-wide CELL census — 230 leading-zero (2 replayable) / 943 bare (156 replayable) — and inferred
  that it *"reaches the current producer in quantity"* and is the only one of the four open defects
  that does. Both counts re-derive exactly. **A cell is published only if it lands in the `value_col`
  of a table that MINTS**, and nearly every all-`0`/`1` cell in this corpus is a bit position, bit
  range or slot number. Replaying the producer and attributing each minted member to its row gives
  **336 minted member values, 0 of them replayable** — and the attribution-free upper bound is 0 too,
  so it is not an artefact of the walk. The replayable stratum states its codes as `0b`/`0x` literals
  (94 + 4 of 126 matches) or single digits (28), which are radix-invariant. **Count the population the
  SEAM reads, not the population the corpus holds.**

  **The producer trims `[`/`]` and the opening census did not** — 67 further cells, and they are bit
  POSITIONS, i.e. the decimal-intending class the repair must leave alone. Corrected population:
  **1,240 cells** (230 / 1,010), still 158 replayable.

  **THE DISCRIMINATOR IS A NOTATIONAL WITNESS, AND IT MAY BE IN THE ROW.** Content shape only, no
  header vocabulary (ADR 0006): a zero-padded all-`0`/`1` sibling in the same column **at the same
  width** or in the same **row** carries a bare cell to binary; a same-width digit ≥ 2 refutes it
  (zero-padded decimal) and outranks it; absent a witness, any digit ≥ 2 in the column makes it decimal
  (`0 1 2 … 11` is a decimal RUN whose `10`/`11` are all-`0`/`1` by coincidence). Verdicts over the
  336: **198 binary / 133 decimal / 0 zero-padded-decimal / 5 undecidable**. Width stratification is
  load-bearing — `opencapi_4_0` `table_0092` fuses a width-1 bit-index table with a width-4 encode
  table in ONE column — and the row witness is **17 of the 22** slots a column-only rule cannot decide
  (Cortex-A76 `op0|op1|CRn|CRm|op2` = `11|011|0100|0100|000`). All 18 contestable calls and all 5
  residual slots were read by hand against the source documents; the 5 are genuinely decimal, so the
  residual costs nothing.

  **UNLIKE `.5.viii`, THIS REPAIR CANNOT REACH THE IDENTITY SEAMS, AND THAT IS PROVED NOT PLACED.**
  For an all-`0`/`1` string the decimal arm succeeds only at ten digits or fewer, where the binary
  reading is at most 1023; above ten digits the decimal arm already fails and the binary arm already
  runs. The set of cells that PARSE is identical under either ordering — only the value changes — so
  `derive_header_sourced_enum_name` clause 5, `infer_encoding_column_indices` and
  `looks_like_encoding_literal` see exactly what they see today. Checked over all **71,001** distinct
  trimmed corpus cells: **0** whose parse success would change.
  Non-goal: the quoted notation (`.5.viii`, landed); landing the reader change (`.5.xi.a`).
  Verification: `python3 scripts/adjudicate_decimal_before_binary_encoding_values.py --residual`
  re-derives 230 / 943 / 1,173 / 158 (the opening census), 230 / 1,010 / 1,240 trimmed, 336 / 0,
  198 / 133 / 0 / 5, and 0 of 71,001. WIRE-BASED-100 exposure is **0 slots** — the wire golds use
  `0b`/`2'b`. Report: `docs/research/decimal-before-binary-encoding-values.md`.
  Commit: `KG-ISF-COMPLETENESS.5.xi — 158 replayable cells, and not one of them reaches a value column`

- ID: `KG-ISF-COMPLETENESS.5.vi.a.2` · Status: `done` (`2026-09-22`; CORRECTION, docs-only; opened and
  closed by `.5.xi`'s re-derivation of the same census) · Goal: **finish the re-pin `.5.vi.a.1` began.**
  `.5.vi.a.1` corrected the book, a fact card's `reverify` and a research table after `.5.vi.a` moved the
  census, and stated the rule: *a change that moves a census must re-pin every surface that publishes
  it.* Re-deriving that census while measuring `.5.xi` found **four more surfaces still stating the
  pre-`.5.vi.a` figure**, all in the PRESENT tense:
  1. this tree's own root — *"The current producer mints 15 enums / 79 members"*, and 99.6%;
  2. `[[minted-enum-values-are-mostly-row-indices]]` — same sentence, and internally inconsistent, since
     530 legacy + 15 replayable is 545 against the 552 its own re-pinned `reverify` expects;
  3. `[[minted-enums-are-mostly-cross-table-conflations]]` — same sentence;
  4. `docs/research/enum-value-provenance.md` — a stratum row LABELLED *"the CURRENT producer"*, whose
     row is now dated instead, since the record's measurements are correct as taken.
  **The rule needs a search, not an intention.** `.5.vi.a.1` re-pinned the surfaces it could think of;
  the three it missed are the ones no oracle points at. Re-derived `2026-09-22`: **22 enums / 119
  members** replayable, **530 / 21,459** legacy, **552 / 21,578** corpus-wide, legacy share **99.45%**.
  Non-goal: dated records inside the sealed source region, which are correct as of their dates.
  Verification: `minted_enums` counted directly over all 78 `generated/evidence_ir/*/evidence_ir.json`
  by stratum; agrees with `.5.vi.a.1`'s `adjudicate_positional_enum_values.py` totals (552 / 19,009) and
  with `.5.xi`'s independent replay (22 enums / 119 members / 126 parsing matches).
  Commit: `KG-ISF-COMPLETENESS.5.xi — 158 replayable cells, and not one of them reaches a value column`

- ID: `KG-ISF-COMPLETENESS.5.xi.a` · Status: `done` (`2026-09-22`; CODE) · Goal: **read an unquoted
  all-`0`/`1` value cell in the radix its own table states.** LANDED exactly as `.5.xi` specified.

  `parse_encoding_member_value` decides the radix from content SHAPE alone: a zero-padded all-`0`/`1`
  sibling in the same column **at the same width** or in the same **ROW** carries a bare cell to binary;
  a same-width digit `>= 2` refutes it and outranks it in both branches; absent a witness the cell keeps
  today's decimal reading, which is also the honest residual. Per-slot over all 336 affected member
  values: **198 binary / 133 decimal / 0 zero-padded-decimal / 5 undecidable**.

  **THE PRODUCT IS BYTE-IDENTICAL, AND THAT IS THE PREDICTED RESULT.** `evidence --dry-run` over every
  persisted `source_ir.json` with both binaries: 27 replayed / 51 refused as schema-1, **27/27 `cmp`
  identical**. `.5.xi` measured the replayable reach at **0** and the attribution-free upper bound at 0,
  so a moving value here would have falsified the measurement rather than confirmed the repair. The
  three rebuildable AMBA wire golds sit inside those 27, so **WIRE-BASED-100 = 1.000 holds by
  construction** — the scorer's input cannot have moved.

  **THE INSTRUMENT WAS CROSS-CHECKED AGAINST THE PRODUCT, the `.5.viii` way.** The three decision
  functions were extracted VERBATIM from `evidence.rs` into a standalone harness and run over all 336
  slots with their full column and row context: **`rust_binary=198, mismatches=0`** against the tracked
  adjudicator. `.5.viii` found `.5.xii` by doing exactly this; it is how a mirror and a producer are
  kept from drifting apart in opposite directions.

  **NON-ADDITIVE, PROVED RATHER THAN PLACED.** For an all-`0`/`1` digit string the decimal arm succeeds
  only at ten digits or fewer, where the binary reading is at most 1023; above ten digits the decimal
  arm already fails and the binary arm already runs. The set of cells that PARSE is identical either
  way, so `derive_header_sourced_enum_name` clause 5, `infer_encoding_column_indices` and
  `looks_like_encoding_literal` see exactly what they see today. 0 divergences over 71,001 distinct
  trimmed corpus cells; two tests pin the property and the seams.
  Non-goal: the quoted notation; `value_col`/`name_col` inference (`.5.xiii`); the 5 residual slots.
  Prerequisite: `.5.xi`.
  Verification: acceptance checklist in the checklists part; `kg-bench` 156/156; `cargo fmt` /
  `clippy -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  green (473 / 168 / 1,618, +12 tests).
  Commit: `KG-ISF-COMPLETENESS.5.xi.a — the table states the radix, and 198 of 336 values were read in the wrong one`

- ID: `KG-ISF-COMPLETENESS.5.xiii` · Status: `done` (`2026-09-22`; MEASUREMENT — the adjudication is
  the deliverable and it refuted the repair it was opened to specify; it opens `.5.xiii.a` and `.5.xiv`;
  re-declared live on becoming the frontier, full contract sealed in
  [the sealed-contracts part](enum-surface-sealed-contracts.md)) · Goal: **a prose `Description`
  column outranks the symbolic column that actually names the member.**

  Live declaration, because `KG-ISF-TASK-EVIDENCE-CONTAINMENT.2` made a rule reachable that could not
  bind before it: **a leaf cannot be the frontier unless it is declared in the ACTIVE part.** Until the
  partition every open leaf shared one part with the frontier, so the question never arose; now the
  sealed stratum holds contracts and the live stratum holds current state, and the frontier is current
  state by definition. `check_active_task_evidence.pl` enforces it twice — the frontier must route to an
  `active` part, and its `part_id` must equal its own leaf route's.

  Nothing about the contract changes: `infer_encoding_column_indices` picks `name_col` as the FIRST
  header matching `name|meaning|description|state|transfer|response|type`, so AXI `Table A7.3` names its
  members from `Description` and emits `AWSNOOP ACT_WRITE_ACCESS__INDICATED_BY_AWACTV_ASSERTED = 0`
  where the document's own member name is the `Opcode` cell (`WriteACT`). Upper bound **262 enum-minting
  tables across 34 documents**; it is an UPPER bound because the shape test alone cannot tell a
  member-name column from an incidental one, which is why adjudication is the first step and not the
  conclusion. The fix must be content SHAPE, not a wider word list (ADR 0006), and the same table raises
  a second question the adjudication must answer: a genuine multi-key table (`AWSNOOP` x `AWDOMAIN` ->
  opcode) cannot be a flat enum at all and may have to mint a residual instead.
  Prerequisite: `.5.vi.a` (met). Verification: as sealed — adjudicate a sample of the 262 by hand
  against the documents, then `evidence --dry-run` before/after over the 27 replayable chains with a
  per-member diff, and WIRE-BASED-100 on rebuilt golds with both binaries.

  **THE HANDED-DOWN POPULATION IS NOT REPRODUCIBLE, AND THAT IS THE FIRST FINDING.** The leaf was
  opened on an upper bound of *"262 enum-minting tables across 34 documents"* whose shape test —
  *"another column holds short, single-token, mostly-DISTINCT cells"* — was never recorded.
  Re-deriving from that prose lands anywhere in **255-268 tables / 32-33 documents** depending only on
  the length cut-off and the distinctness fraction; 262/34 sits inside the band and cannot be hit
  deliberately. **A shape test with unrecorded thresholds is not a measurement.**

  **SO IT DECLARES NO THRESHOLD: IT REUSES THE ONE THIS REPOSITORY ALREADY CALIBRATED.**
  `synthesize_signal_declarations` answers the same question for SIGNAL tables and has been corrected
  against the corpus twice — `name_cell_is_read_whole` decides name-versus-prose, the score is DISTINCT
  names, and `NAME_COLUMN_OVERRIDE_MARGIN = 2` is the lead content needs to overrule a header.
  *(CORRECTED by `.5.xiii.a.7`: every population figure in this record was located by a table sharing
  ANY member with the minted enum — 68% of those pairings are tables that never minted it. By
  attribution: 400 pairs / 383 tables / 33 documents; multi-key 55 of 400 (13.8%) and 465 of 3,298
  minting tables (14.1%); split-bit 8 / 8. The twelve hand-read replayable pairs stand.)*
  Transposed to the member seam: **1,177 pairs / 445 tables / 35 documents, 12 replayable pairs — and
  every one of the twelve is in an AXI or AHB WIRE-BASED-100 gold.**

  **ALL TWELVE ADJUDICATED BY HAND, AND THE SCORE IS WRONG IN BOTH DIRECTIONS.** `AxBURST` is the case
  the repair was designed for: `Label` holds `FIXED`/`INCR`/`WRAP`, the document's own names, against
  today's `FIXED_BURST` from `Meaning`. **`AxSIZE` refutes the blanket rule**: its `Label` column holds
  `1 2 4 8 16 32 64 128` — byte COUNTS — which score as eight distinct names and win by the margin,
  yet `Enum AxSIZE 1 = 0.` is strictly worse than `Enum AxSIZE 1_BYTE_PER_TRANSFER = 0.`. A content
  score cannot separate a member-name column from a QUANTITY column, so `.5.xiii.a` must adjudicate per
  table and may not ship the override alone.

  **THE ANCHOR ANSWERS THE LEAF'S OWN SECOND QUESTION.** AXI `Table A7.3` has **13 rows and 8 distinct
  AWSNOOP codes**, `0b00000` on five of them, because `(AWSNOOP, AWDOMAIN)` determines the operation
  and `AWSNOOP` alone does not. Re-pointing the name column would emit five members equal to 0 —
  trading a naming defect for the collision `.5.vii` calls *provably wrong*. **82 of the 1,177 pairs
  (7.0%) in 66 tables are multi-key; corpus-wide it is 654 of 5,456 minting tables — 12.0%, one in
  eight.** A multi-key table must mint a RESIDUAL.

  **AND A DEFECT THIS MEASUREMENT WAS NOT LOOKING FOR — see `.5.xiv`.** 266 enums already publish a
  colliding code and **9 are REPLAYABLE, all in the AMBA wire golds**: `HPROT` 6 of 8 members,
  `AWSNOOP` 5, `BTAGMATCH` 5, `HSIZE` 4, `ARSNOOP` 3. They survive `.5.vii.a` *because the cell does
  parse* — the value is document-stated and read from the wrong COLUMN. APB `PPROT` publishes
  `Enum PPROT 0 = 0.` / `= 1.` and `Enum PPROT 1 = 0.` / `= 1.`; AHB `HSIZE` sits under
  `HSIZE[2] | HSIZE[1] | HSIZE[0] | Size (bits) | Description`, so `value_col` matched the TOP BIT.
  Non-goal: shipping the override (`.5.xiii.a`); the value column (`.5.xiv`).
  Verification: `python3 scripts/adjudicate_enum_member_name_column.py` re-derives 1,177 / 445 / 35 /
  12, 82 / 66 / 7.0%, 654 / 5,456 / 12.0%, 19 / 14 split-bit pairs and 266 / 9 collisions; the twelve
  replayable pairs and the four named tables were read against the documents; the minted statements for
  `AxSIZE`, `AxBURST`, `HSIZE`, `HPROT` and `PPROT` were read from the persisted `evidence_ir.json`.
  Report: `docs/research/enum-member-name-column.md`.
  Commit: `KG-ISF-COMPLETENESS.5.xiii — the score picks AxSIZE's byte counts, and one minting table in eight cannot be an enum`

- ID: `KG-ISF-COMPLETENESS.5.xiii.a` · Status: `done` (`2026-09-22`; CODE; opened by `.5.xiii`) · Goal:
  **name a member from the column the document names it in — per table, not by score alone.** LANDED as
  `adjudicate_member_name_column` at the member seam; `infer_encoding_column_indices` is untouched, so
  the value column and the seam's other caller cannot move.

  **THE `.5.xiii` INSTRUMENT WAS NOT A FAITHFUL TRANSPOSITION, AND THE AXSIZE REFUTATION WAS ITS
  ARTEFACT.** It scored the SANITIZED cell, so `1 2 4 … 128` scored as eight names. The signal seam it
  claimed to transpose also requires an identifier-shaped token (`is_hardware_signal_token`), under
  which that column scores **0**. The faithful score, now shared as `read_whole_name_token`, refuses
  AxSIZE, AHB `HSIZE`'s `Size (bits)` and BTAGMATCH `table_0178`'s `0b0`/`0b1` on its own — all three
  of the replayable pairs the hand adjudication rejected. **A mirror that drops one clause of the
  thing it mirrors can manufacture a finding.**

  **WHAT THE SCORE GENUINELY CANNOT REFUSE WAS IN THE POPULATION, NOT IN THE TWELVE.** Reading a
  sample of the 1,087 non-replayable pairs found two classes the replayable stratum never showed:
  CCIX register-field tables whose `Attributes` column (`RW`/`RO`/`RsvdP`) would win, and register
  FIELD tables whose value column is a bit position (`[31]`), a range or a width. The rule is four
  content-shape gates (ADR 0006): (1) only a `meaning`/`description` header is contestable; (2) every
  minting value cell must read as a code (`looks_like_encoding_literal`); (3) a column scores its
  names that determine ONE code minus the rows whose name labels several, and must lead by
  `NAME_COLUMN_OVERRIDE_MARGIN` — the net term was added when JESD235A `Table 40`, a lane-remap
  grid, would otherwise have replaced `REPAIR_LANE_0..7` with `DMX0`/`XX`; (4) two names on one code
  in the elected column prove the table MULTI-KEY, and it mints nothing. Verdicts over the contested
  LEGACY pairs, located by attribution (`.5.xiii.a.7`): 1,244 — notation 887 / score 325 / **re-pointed
  25 / multi-key 7** (first published as 4,416 / 3,644 / 736 / 27 / 9 by the loose locator)
  (`adjudicate_enum_member_name_column.py --landed`); the replayable stratum is measured by replay.

  **THE REBUILD BROKE THE INSTRUMENTS BEFORE IT BROKE ANYTHING ELSE.** Every enum instrument locates a
  member's source table by replaying the header rule, so the moment AXI's chain carried document names
  its re-named tables stopped matching: attribution declined 7 -> 16 and the landed count fell 33 -> 28
  with no product change behind either. `producer_name_column` now replays each artifact with the rule
  of the producer that MINTED it (schema 3: the landed rule; legacy: the header rule), in all four
  instruments; declined is back to 7. A residual weakness is stated in the report: document names
  recur across enums, so on a rebuilt chain the loose locator over-pairs tables, and the replay — not
  the count — is the authority there.

  **THE MULTI-KEY VERDICT WAITS FOR THE DOCUMENT'S NAMES, BECAUSE PROSE CANNOT PROVE IT.** RED on
  Table A7.3 before the change: five different sentences on `0`, and ONE sentence,
  `CACHE_LINE_SIZED_WRITE`, on codes 1, 3 and 5. The prose column shows both a collision and a
  non-collision and can prove neither; the `Opcode` column proves the key.

  **EFFECT, REPLAYED.** `evidence --dry-run` over all 78 chains, both binaries: 27 replayed, **26
  byte-identical**, AXI `ihi0022_l` **−29 / +46** member statements. Every re-named member is the
  document's own name (`FIXED`/`INCR`/`WRAP`, `CLR`…`UMIN`, `CLEANINVALID`…, `NONE`/`SEPARATE`/
  `FAIL`/`PASS`, `NON_SECURE`…); Tables A7.3/A7.4 are withdrawn as multi-key; and **three enums are
  RECOVERED** that the `.5.ii` spine gate had emptied because every `Meaning` cell is a sentence —
  `RRESP` (8), `ARTAGOP` (4), `AWTAGOP` (4), each read against its table. The instrument cannot see a
  recovery (it enumerates enums already minted), so the replay is the authority on reach.

  **WIRE-BASED-100 IS IDENTICAL, AND THAT IS NOT THE ACCEPTANCE.** `eval-extraction` on the AXI gold
  before and after: constraints 4/4, temporal 3/3, recall 1.000 both ways — but the gold labels 5
  statements and none of the ones that moved. The id-free SemanticIR diff is the oracle that sees
  them: symbols 13 -> 15, invariants −4 (the withdrawn prose members), signal constraints +3, all
  three conditional in the document and published without it (`.5.xiii.a.3`).

  **IT EXPOSED A DEFECT DOWNSTREAM — see `.5.xiii.a.1`.** Rebuilding the chain showed signal
  constraints moving, which a naming change should not do: the document's own names include ordinary
  words, and the discovered-value binder bound `AWATOP`'s `SET` to the verb in *"AWSNOOP must be set to
  all zeros"*. Closed in the same commit because it is this leaf's NO-REGRESSION condition.
  Non-goal: `value_col` inference (`.5.xiv`); the caption gate (`.5.vi.a`, landed).
  Verification: acceptance checklist in the checklists part.
  Commit: `KG-ISF-COMPLETENESS.5.xiii.a — name a member from the document's own column, and a code only where the sentence writes one`

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.1` · Status: `done` (`2026-09-22`; CODE; opened and closed by
  `.5.xiii.a`'s chain rebuild) · Goal: **a discovered enum value binds only where the sentence writes
  it as a code.** `extract_discovered_state_value_from_text` compared lower-cased text to every enum
  member in the document, whatever enum it belonged to, so once `.5.xiii.a` published `AWATOP SET` the
  rebuilt AXI chain gained `AWSNOOP must_be_value SET` and `WSTRB must_be_value SET` — the verb, bound
  as a code. The repair is capitalisation, not vocabulary (ADR 0006): a span that opens with a digit
  is a literal (`0`, `0h`, `0b01` — `is_admissible_state_value`'s own test) and binds as before; any
  other span must carry a capital, because a document writes a code the way it defines it (`INCR`,
  `Invalid`) and prose writes the same letters in lower case.
  **Reach, measured rather than assumed**: on all 27 replayable chains it removes exactly those two
  records and nothing else; statically over the 51 legacy chains it would drop 4 dynamic records, all
  prose (*"there shall be no more than one entry"*, an older binary's *"shall be non-zero"*, *"It is
  set when"*). The first draft treated `0b01` as a word because a radix prefix is a letter; the legacy
  census caught it. Two existing tests pinned the case-blind contract by lower-casing their input —
  one asserted *"shall be valid only under the following conditions"* binds `VALID` — and now pass the
  document's own text.
  Non-goal: subject-scoping the binder (a value from ANY enum still binds to any subject — a larger
  question with its own measurement); the static path's deliberate `must be valid` → `VALID`.
  Verification: acceptance checklist in the checklists part (shared with `.5.xiii.a`).

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.2` · Status: `done` (`2026-09-22`; CODE; opened by `.5.xiii.a.1`'s
  legacy census) · Goal: **a numeric discovered value must end where the literal it is read from
  ends.** `lead_binds_value` required a trailing boundary only for a value ending in a letter, which
  `EXTRACTION-QUALITY-GAUGE.3f` left lenient so *"shall be 0h"* binds `0` — and which also let a number
  bind as the PREFIX of a longer literal. The rule now reads the identifier run continuing past the
  value: it must be empty, or, for a value ending in a digit, exactly one radix-suffix letter (`h`,
  `b`). One statement of both halves; alphabetic behaviour is unchanged.

  **THE OPENING DESCRIPTION WAS NARROW.** It named 7 records in ARM DTI. The census over every
  persisted numeric `dyn_sigcon_*` found **35**, of which **28** end at their literal and **7** are
  prefixes across **four** documents: `0B11` ×3 from *"must be 0b1111"* (DTI), `12` from *"128-bit
  aligned"* (Intel VT-d), `1` ×2 from `16` (NVMe), `0` from `0x0…` (eMMC). DTI's four `0B01` records
  were not in the class — they bind a whole literal and are kept; their SUBJECTS are wrong, which is
  the known `ISF-VALUE-WIDTH-EMIT` spin-out (`[[isf-value-width-operand-contract]]`), not this leaf.
  Verification: acceptance checklist in the checklists part.
  Commit: `KG-ISF-COMPLETENESS.5.xiii.a.2 — a number ends where its literal ends`

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.3` · Status: `done` (`2026-09-23`; MEASUREMENT — it refuted both
  obvious repairs; opens `.5.xiii.a.3.a`) · Goal: **how many constraints lose the condition their
  heading or caption states?** `.5.xiii.a` exposed three AXI constraints published unconditionally from
  atomic-transaction bullets and the DVM Complete table.

  **THE CENSUS IS LAYOUT.** Every `signal_constraints` record in all 78 chains traced to its binding
  statement: a bullet or table row has a SCOPE CARRIER when a colon-terminated lead-in, a
  colon-terminated bullet of the same list (a nested lead-in the text layer flattened), or a `Table …`
  caption precedes it. **186 of 358 records (52%) are scoped — 79 of the 103 the current producer
  mints.** 23 of 97 scoped bullets depend on a nested lead-in whose nesting the text lost.

  **FORTY READ BY HAND (seed 13): 16 carriers RESTRICT and all 16 are lost** — *"During reset …"*
  publishes `RACK must be LOW`; the atomic-transaction rules, DVM Complete, *"When FRESETn is
  asserted:"*, barrier responses, the AXI4-Lite upgrade procedure. **8 are ROW-KEYED** — the condition is
  a sibling key column (`AWTAGOP = Match`, `LTI_MMU = True LTI_GPC = False`), the multi-key shape again,
  and never carried. **16 are NEUTRAL** (*"The User signals protocol rules are:"*, a continuation label).
  Every `condition_text` present comes from the item's own clause, by construction.

  **BOTH OBVIOUS REPAIRS ARE REFUTED BY THE SAME SAMPLE.** Copying the lead-in into `condition_text`
  fabricates a condition from the neutral 40%, and telling neutral from restricting needs meaning a
  word list may not supply (ADR 0006). Residualizing every scoped record costs gold-scored constraints —
  the neutral carriers include AHB `HAUSER`/`HBUSER`, which WIRE-BASED-100 scores (five `seed_ahb.json`
  items; corrected `2026-09-23` — APB `PWUSER`/`PAUSER` were first named here and are NOT gold-scored). So `.a` records the carrier as provenance and measures any lowering gate against the golds.
  Verification: `python3 scripts/measure_constraint_scope_carriers.py --sample 40 --seed 13`, the 40
  carriers adjudicated in `docs/research/constraint-scope-carriers.md`.
  Commit: `KG-ISF-COMPLETENESS.5.xiii.a.3 — half the constraints sit under a scope, and none keeps a scope that restricts`

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.3.a` · Status: `open` (`2026-09-23`; CODE; opened by `.5.xiii.a.3`)
  · Goal: **a constraint record says when the document scoped it.** Record the scope carrier (text and
  kind: lead-in, nested lead-in, caption) on the signal-constraint record as typed provenance,
  reconstructing flattened nested lead-ins the way the census does; no reading of the carrier's
  meaning (ADR 0006). Then MEASURE, on the WIRE-BASED-100 golds with both binaries, whether a scoped
  record without a `condition_text` may still lower unconditionally — the census proved the blanket
  residual would cost gold-scored constraints, so the gate is chosen on the scores, not presumed.
  Non-goal: row-keyed constraints (a sibling key column is `.5.xiii`'s multi-key shape, a separate
  repair); reading a carrier's meaning.
  **SCOPING NOTE (`2026-09-23`, before any code), CORRECTED THE SAME DAY.** It first said a new
  EvidenceIR field *"changes the frozen 38-family / 170-field producer graph and invalidates all 24
  sealed chains at once"*. **That was transposed, not measured**: the 24-chain cost in
  `[[evidence-proof-binds-artifact-location]]` belongs to re-binding `artifact_layout`, which changes every
  derivation's replay topology. The producer graph is ADR 0038's `claim_family_inventory.tsv`, which
  classifies every top-level artifact field once; a new field is ONE classified row (plus IntentIR's
  carried-field list), rejected until registered — bounded, not a re-seal. Adding the carrier to
  `SignalConstraintRecord` itself would touch 169 construction sites, so if persisted it belongs in a
  parallel top-level collection keyed by `constraint_id`. Whether to persist or derive at the consumer is
  decided when this leaf starts, on those real costs.
  Prerequisite: `.5.xiii.a.3`. Verification: replay over the 27 chains showing only the intended
  surface moves; WIRE-BASED-100 and `kg-bench` with both binaries; a fixture that fails without the
  scope.

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.4` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by
  `.5.xiii.a.3`'s hand sample) · Goal: **a table row's constraint must carry the row KEY it holds for.**
  8 of the 40 sampled scoped records are ROW-KEYED: the condition is a sibling key column, not the
  caption — `AWTAGOP = Match` rows of *"Table A12.12: AWTAGOP encodings"* publish `WTAG must_be_value
  VALID`, and *"Table B12.2"*'s `LTI_MMU = True LTI_GPC = False` row publishes `LASECSID[1] must_be_value
  LOW` with the key dropped. 0 of 8 carry the key (2 carry the item's own clause). It is `.5.xiii`'s
  multi-key shape on the constraint surface. Measure the class corpus-wide (rows whose table has a key
  column distinct from the constrained cell), then carry the key as the condition — structurally, the
  row's key cell under its header — never by vocabulary.
  Verification: census over all 78 chains; replay over the 27 with a per-record diff; WIRE-BASED-100 and
  `kg-bench` with both binaries.

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.5` · Status: `done` (`2026-09-23`; MEASUREMENT — the named symptom is
  a legacy artifact, and the logic behind it is live; opens `.a.5.a`-`.a.5.c`) · Goal: **a dotted field
  reference is the subject of its own binding** (DTI `ATTR_OVR.SHCFG must be 0b01` attributed to
  `FLOW`/`ATST`/`DTI`/`ATTR`).

  **THE CURRENT PRODUCER DOES NOT REPRODUCE THE DTI RECORDS.** DTI's evidence is schema 2 (an older
  binary). Running today's `extract_dynamic_signal_constraints` over its persisted statements yields
  **0** dynamic records: DTI declares no signal with the canonical `Signal X is …` grammar, so the
  `WIRE-BASED-100.5i` catalog gate refuses every subject. The four records clear when DTI is re-ingested.

  **BUT A SEAM TEST WITH THE NAMES DECLARED SHOWS THE LOGIC IS LIVE**, three defects deep:
  1. *"When … BYPASS is 1, then TRANS_RNG must be 0b1111"* mints `BYPASS must_be_value 0B1111`: the
     sentence opens with its condition, the text before the condition marker is empty, and the fallback
     searches the whole sentence — so the subject is a CONDITION-clause signal (`.a.5.b`).
  2. `TRANS_RNG` is declared yet never found, because the text says `TRANS\_RNG`: EvidenceIR reads prose
     from Docling's promoted MARKDOWN, where `\_` is escape syntax, and keeps the backslash. Measured on
     the replayable stratum: **Wishbone B4 declares 25 signals, all with an underscore, all 25 escaped in
     prose, 629 statement mentions — and its rebuilt EvidenceIR has 0 signal constraints and 0 actor
     relations** (`.a.5.a`). SourceIR's own `content_elements` hold `ACK_O` unescaped.
  3. `ATTR_OVR.SHCFG must be 0b01` mints nothing: the dotted reference's field is dropped as a subject —
     a recall loss, not a fabrication (`.a.5.c`).
  Verification: a throwaway test over DTI's persisted statements (0 records, catalog empty); a seam test
  with the six names declared (`BYPASS` minted, `SHCFG`/`MTCFG` not); the Wishbone census from the
  persisted artifacts.
  Commit: `KG-ISF-COMPLETENESS.5.xiii.a.5 — the DTI records are legacy, and the three defects behind them are live`

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.5.a` · Status: `done` (`2026-09-23`; CODE) · Goal: **statement text
  holds the document's characters, not Markdown's escapes.** `unescape_markdown_text` at the two seams
  that turn promoted Markdown into text (`normalize_block_text`, `heading_title`): CommonMark's rule, a
  backslash before ASCII punctuation denotes it, keeping `\|` (table structure). The only escape Docling
  emits in the replayable stratum is `\_` (41,231); 114 escaped headings include register names.
  **Its no-regression condition, two polarity rules (`names_the_signal_itself`).** Recovering `RST_I` let
  Wishbone's Rule 2.30 rationale — a reset "could be named [#RST_I], [/RST_I]" — make `RST_I`
  active-low and mint `RST_I must_be_low`. An inversion-marked name denotes the inverted signal. The
  replay then showed the helper's CASE-EXACT match also moved AXI and ADIv6, adopted deliberately after
  reading each: AXI's *"Lowercase n … denotes an active-LOW signal"* no longer makes `N` active-low;
  ADIv6's prose *"in"* no longer resolves to a junk `In`, which removes a false `In` polarity conflict and
  recovers *"nSRST is an active LOW pin"* -> `nSRST active_low`.
  **Reach (27 replayable chains):** 25 change, almost all statement text. Semantic surfaces: Wishbone
  relations 0 -> 41, constraints 0 -> 2 (`CYC_O`, `RST_I must_be_asserted`), hints 0 -> 11; AXI constraint
  source texts unescaped (same 70); ADIv6 −1 junk relation (`details reads TDI`). **WIRE-BASED-100
  byte-identical scores on all 9 scorable gold datasets**, each side scored by its own binary (the
  scorer re-verifies proofs with the running binary, so a cross-binary run scores nothing); NVMe and
  RISC-V debug are legacy and unscorable on either side.
  **WISHBONE'S RECOVERED RELATIONS ARE ~56% PRECISE, AND THAT IS NOW OWNED.** Read by hand: 23 correct, 16
  wrong, 2 debatable. The 9 phrase actors (`interfaces so`, `similar manner`, `bus cycle`) are the class
  `.1c.ii` measured and deferred — its trigger is now met, since it reaches a rebuildable document; the 7
  wrong directions (*"negates its [STB_O]"* -> `MASTER reads STB_O`) had no owner. Opened `.1d` and `.1e`
  in the `relation-precision` part.
  Verification: acceptance checklist in the checklists part.
  Commit: `KG-ISF-COMPLETENESS.5.xiii.a.5.a — the document's underscores, and the polarity rules they needed`

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.5.b` · Status: `open` (`2026-09-23`; CODE) · Goal: **a sentence that
  opens with its condition takes its subject from the main clause.** *"When A, then B must be V"* has an
  empty text-before-condition, and the fallback searches the whole sentence, admitting condition-clause
  signals. `CONSTRAINT-CONDITION-SUBJECT.1` closed the same hole for the pattern path's empty-subject
  fallback, not the dynamic path's.

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.5.c` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE) · Goal:
  **a dotted field reference binds its field.** `ATTR_OVR.SHCFG must be 0b01` yields no subject:
  `EXTRACTION-QUALITY-GAUGE.3g` drops a subject reachable only through a `Reg.Field` reference, which is
  right for a reference in a DESCRIPTION and wrong for the reference that IS the binding's subject. Measure
  how many bindings name their subject only as `X.Y` first.

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.6` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by
  `.5.xiii.a`'s IntentIR diff) · Goal: **two tables that name the same enum must not cancel each other's
  correct members.** AXI `BTAGMATCH`: `Table A12.14` now names `NONE`/`SEPARATE`/`FAIL`/`PASS` = 0..3,
  `Table A12.15` (`BTAGMATCH | BCOMP | Description`, a multi-key response table) still names prose on the
  same codes, and `.5.xiv.a`'s gate withdraws every colliding group — so the enum is withdrawn from
  IntentIR although one of its two tables states it exactly. `.5.vi`/`.5.vi.a` are closed and never
  owned the cross-table case. Measure how many withdrawn groups have ONE side from a table the
  per-table rules (`.5.xiii.a`) would accept and the other from one they refuse, then decide per table
  rather than per group. Verification: collision census before/after; IntentIR per-symbol diff; golds.

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.7` · Status: `done` (`2026-09-23`; CODE, instrument) · Goal: **the
  enum instruments locate a member's source table by attribution, not by one shared member.**
  `located_tables` runs `attribute_members` (`.5.vi`/`.5.vii`), which requires the minted SEQUENCE to be
  ordered runs of each table's replay; both reports in `adjudicate_enum_member_name_column.py` use it.

  **THE LOOSE LOCATOR WAS WRONG ON THE LEGACY STRATUM TOO, NOT ONLY ON REBUILT CHAINS.** This leaf was
  opened believing the loose locator was exact where member names are descriptive. Measured: attribution
  declines 7 of 555 enums (the positional census's own 7), and of the 13,948 pairs the loose locator
  produced, **9,549 (68%) are tables attribution rejects** — generic mega-enums above all (`TABLE`, 962
  members, pairs with 1,371 tables through incidental shared members). Spot-checked: Cortex-A76
  `MVFR2_EL1` = `(OP1, 0), (000, 11)` was paired with `table_0215`, *AMEVCNTRn_EL0 encoding*, through
  `(OP1, 0)` alone; attribution names `table_0153`. **Every population figure `.5.xiii` and `.5.xiii.a`
  published was inflated**, and is corrected where it is stated: `.5.xiii` 1,177 / 445 / 35 -> 400 /
  383 / 33; multi-key 7.0% -> 13.8%, corpus-wide 12.0% -> 14.1%; split-bit 19 / 14 -> 8 / 8; landed
  legacy 4,416 / 3,644 / 736 / 27 / 9 -> 1,244 / 887 / 325 / 25 / 7 (book re-pinned: *27 and 9* -> *25
  and 7*). Collisions (264 / 7) never used the locator and stand.

  **TWO TABLES CAN BE INDISTINGUISHABLE, AND THE REPORT NOW SAYS SO.** On the replayable stratum
  attribution reports 10 re-pointed where the replay shows 9: AXI `table_0045` (unlabelled BRESP
  encodings) replays IDENTICALLY to `table_0163` (*WriteDeferrable response meanings*), attribution takes
  the first, and the replay shows no `BRESP` change, which `table_0045` minting would have caused.
  `twin_tables` marks such a pair AMBIGUOUS; 10 - 1 = the replay's 9. A residualized multi-key table
  mints nothing, so no artifact-based locator can find it — the replay measures those, stated in the
  report.
  Verification: `--landed` legacy 887 / 325 / 25 / 7, replayable 10 re-pointed with 1 ambiguous; the
  default report 400 / 383 / 33; `--collisions` unchanged 264 / 7; the `MVFR2_EL1` and `BRESP` cases read
  against their tables.
  Commit: `KG-ISF-COMPLETENESS.5.xiii.a.7 — two thirds of the located tables never minted the enum`

- ID: `KG-ISF-COMPLETENESS.5.xiii.a.8` · Status: `done` (`2026-09-23`; CODE, gate + projection; opened
  and closed by the verification pass) · Goal: **the kg-bench fixture count is derived, never typed.**
  Raised in `.5.xiii.a` as "a literal on current-facing surfaces" and recorded without an owner; the
  verification pass found the literal was also inside a GATE, and that gate was blind.

  **THE CORPUS-KB PROJECTION WAS STALE AND THE GATE COULD NOT SEE IT.** `check_corpus_kb_currentness.pl`
  checked the tracked fixture inventory against the git index (158) but checked the KG aggregate
  projection and the producer against TYPED literals. `.5.xiv.a` and `.5.xiii.a` each moved one literal
  (156 -> 157 -> 158) and neither refreshed the projection, so `corpus_kb/benchmarks/kg-fixtures.md`
  kept declaring `156` / `156` and listed neither new fixture — and at `4dd07866` the HEAD gate printed
  *"156/156 KG fixtures … are current"* against a 158-fixture inventory. **RED reproduced on HEAD.**
  The gate now takes ONE count, the inventory's, for the aggregate, the producer and its own report;
  the self-test gains the two missing RED cases (a projection declaring a stale count; a producer
  count drifting from the inventory), 15 -> 17 assertions. The projection was refreshed with the
  documented producer: the two fixtures and nothing else, across four pages and the JSON sidecar.

  **AND IT REPORTED ONE STALE PAGE OF FIVE.** `next if @$errors` skipped every later managed output once
  any earlier one had failed, and `return if @$errors` skipped the JSON output. Each output is now judged
  on its own errors; the refreshed tree reported all five before the contract was re-pinned.

  **PROSE.** 21 current-facing lines in 8 files (TOOLBOX, DOCTRINE_ENFORCEMENT, DOCTRINE_INSTANCE, two
  gate scripts, three book chapters) stated `156/156` in the present tense. They now state a count-free
  form ("every fixture passes", `N/N` read from the run) so the next fixture cannot stale them; the
  acceptance signature drops its `156/156` alternative (`kg-bench` already matches). Dated records —
  KM cards, research, ADR 0006, "how it was verified" sections, `kg_bench.rs`'s account of `.5.xiv` —
  are correct as of their dates and stay.

  **FOUND ON THE WAY, OWNED NEXT IN THE TREE THAT INTRODUCED IT:** `cargo run --manifest-path Cargo.toml
  -- <cmd>` — the form of README's quick start and 192 occurrences in 21 files — fails with *"could not
  determine which binary to run"*: the workspace has had two binary packages since
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.v.ii` added `specforge-production-graph`. `run_ci.sh` runs clippy,
  test and doc WITHOUT `--workspace`, so a `default-members` fix must add `--workspace` there or it
  silently drops the graph tool from CI.
  Verification: `perl scripts/check_corpus_kb_currentness.pl --check` (158/158) and `--self-test`
  (17/17); `check_book_quantitative_claims.pl` 597/597 after dropping the five records whose quantity
  was removed; `kg-bench` unchanged at 158/158 (no product code touched).
  Commit: `KG-ISF-COMPLETENESS.5.xiii.a.8 — the fixture count is derived, and the projection it hid is refreshed`

- ID: `KG-ISF-COMPLETENESS.5.xiv` · Status: `done` (`2026-09-22`; MEASUREMENT — it corrected its own
  opening contract twice, on the CAUSE and on the ORACLE; it opens `.5.xiv.a`) · Goal: **nine replayable enums publish one code on two members, and all nine are in the
  wire golds.** `infer_encoding_column_indices` picks `value_col` as the first header matching
  `value|encoding|code|binary|hex|bit` or referencing the enum name, which selects **one bit of a
  split field** when a document decomposes the encoding per bit: AHB `Table 3-2` is
  `HSIZE[2] | HSIZE[1] | HSIZE[0] | Size (bits) | Description`, so every member's code is bit 2 and
  `BYTE`, `HALFWORD`, `WORD`, `DOUBLEWORD` all publish as 0. APB `PPROT` is worse — two member names,
  each carrying two different codes.
  **This is the class `.5.vii.a` could not reach**: the fallback removal dropped members whose cell
  does not parse, and these cells DO parse. A collision needs no interpretation (`.5.v`'s standing
  rule: no encoding assigns one code to two members), so the defect is certain even before the repair
  is chosen. Measured: **266 enums collide, 9 replayable** (`HPROT` 6/8, `AWSNOOP` 5, `BTAGMATCH` 5,
  `HSIZE` 4, `ARSNOOP` 3, `ARCACHE`/`SYSCOACK`/`PPROT` 2, `AWATOP` 1); **19 pairs in 14 tables** have a
  value column that is one bit of a split field.
  The measurement must decide between composing the split bits into one code and residualizing the
  table, and must separate the split-bit cause from the multi-key cause (`.5.xiii`), which produces the
  same symptom for a different reason.
  Non-goal: the name column (`.5.xiii.a`); re-ingesting.
  Prerequisite: `.5.xiii`. Verification: per-enum adjudication of all 9 replayable collisions against
  their documents, then `evidence --dry-run` with a per-member diff and WIRE-BASED-100 scored on
  rebuilt golds with both binaries.

  **CORRECTION 1 — THE LEAF WAS OPENED ON ONE CAUSE; THERE ARE THREE.** It claimed `value_col`
  *"picks ONE BIT of a split field"*. Classifying all nine replayable collisions refutes a single
  cause: **split bit field** (`HSIZE` under `HSIZE[2] | HSIZE[1] | HSIZE[0] | Size (bits) |
  Description` — one enum); **one bit of a wider field with the rest absent from the table** (APB
  `PNSE | PPROT[1] | Physical address space`, which is why its members are named `0` and `1` and each
  carries two codes); **genuine multi-key** (`AWSNOOP` = `(AWSNOOP, AWDOMAIN) -> opcode`, plus
  `ARSNOOP`, `ARCACHE`, `SYSCOACK`, `BTAGMATCH`); and **cross-table** (`AWATOP` collides between two
  tables that each determine their own members — that is `.5.vi` conflation, not a column defect).
  The unifying statement is weaker and truer: **the value column does not functionally determine the
  member**, and the repair must not assume which way it fails.

  **CORRECTION 2 — THE VERIFICATION THIS LEAF DECLARED CANNOT FAIL.** Its own contract named
  *"WIRE-BASED-100 scored on rebuilt golds with both binaries"*. Measured: **0 of 156 `kg-bench`
  fixtures mention `enum` or `symbol_definition`**, and `eval-extraction`'s 93 labelled items carry
  **9 task kinds — `signal_constraint`, `actor_signal_relation`, `temporal_rule`, `declared_signal`,
  `register_field`, `serial_frame_field`, `protocol_operation`, `protocol_state`,
  `interface_edge_timing` — none of them an enum.** Both headline gates are structurally blind to
  every enum member value. That is `.5.viii`'s *"a count cannot verify a value"* one level up: **an
  oracle that cannot fail is not verification**, and it explains the whole `.5` programme's history,
  in which every defect was found by a census rather than by a test.
  [[enum-member-values-have-no-oracle]]

  **REACH, FOLLOWED RATHER THAN ASSERTED.** 266 enums collide corpus-wide; **9 replayable, and 8 of
  them cross into IntentIR `symbol_definitions`, which `README.md` declares the product boundary** —
  `HPROT` 6 of 8 members, `AWSNOOP` 5, `BTAGMATCH` 5, `HSIZE` 4, `ARSNOOP` 3, `ARCACHE`/`SYSCOACK` 2,
  `AWATOP` 1. APB `PPROT` does NOT cross it: that document promotes no symbols at all. **None reaches
  `.isf`** — AXI, AHB and APB are all `lowering_status: blocked` for temporal, state and width
  residuals unrelated to enums. So the defect is live at the boundary the project publishes and stops
  one stage short of the file FSMGen would consume.
  Non-goal: recovering the value (composing `HSIZE[2:0]`, lowering a multi-key table as a relation) —
  each is additive and needs its own measurement; `.5.vi` conflation; the name column (`.5.xiii.a`).
  Verification: `python3 scripts/adjudicate_enum_member_name_column.py --collisions` re-derives
  266 / 9 / 8-of-9-at-IntentIR with the per-enum cause; the nine were re-derived a second time by
  parsing `Enum … = ….` straight out of each persisted `evidence_ir.json`, independently of the
  replay machinery; `lowering_status` read from each `generated/adapters/isf/*/adapter.json`.
  Report: `docs/research/enum-code-collisions.md`.
  Commit: `KG-ISF-COMPLETENESS.5.xiv — the value column does not determine the member, and no oracle can see it`

- ID: `KG-ISF-COMPLETENESS.5.xiv.a` · Status: `done` (`2026-09-22`; CODE) · Goal:
  **stop publishing a contradiction, and ship the oracle that can prove it.** LANDED.

  **THE ORACLE LANDS WITH THE GATE, NOT AFTER IT.** `.5.xiv` measured that neither standing gate can
  see an enum member value, so this slice adds a `kg_quality` fixture asserting one — the missing RED
  case — and only then the gate. A gate shipped against a bar that cannot fail is the defect
  `.5.xiv` was opened by.

  **THE REPAIR IS SUBTRACTIVE AND NEEDS NO NEW INFERENCE.** An enum whose members carry contradictory
  codes is not faithfully lowerable, so it is held out and recorded as a NAMED residual — the
  mechanism `.2a.iv` already established at the `.isf` boundary (`isf_enum_value_literal_<name>`),
  applied at the seam where the collision becomes visible. `.5.vii.a`'s doctrine, unchanged: an honest
  residual over a fabricated certainty.
  Non-goal: recovering the value; `.5.vi` conflation; the name column.
  Prerequisite: `.5.xiv`. Verification: the new fixture FAILS before the gate and passes after (a red
  case that cannot fail is the thing this leaf exists to stop); `evidence --dry-run` over the 27
  replayable chains with a per-MEMBER diff showing exactly the 8 boundary-crossing enums withdrawn and
  nothing else; `kg-bench` and WIRE-BASED-100 stated as NECESSARY-not-sufficient and reported as
  unmoved; `cargo fmt`/`clippy -D warnings`/`test --workspace --lib` green.

  **THE RULE IS THE MIRROR OF ONE ALREADY IN THE FUNCTION.** `build_symbol_definitions` drops a member
  NAME carrying two different values; it never checked one VALUE carried by two NAMES. Added at the
  same seam and the same granularity: the colliding GROUP is dropped, not the enum, which keeps
  `AWATOP`'s nine correct members and `ARCACHE`'s six. Dropping the enum would discard them; dropping
  one side of a pair would pick a winner, which is the fabrication `.5.vii.a` removed. **An enum with
  no surviving member is no longer published at all** — both per-member rules can empty one, and the
  husk states nothing while looking like a definition.

  **THE ORACLE LANDED WITH THE GATE AND IS RED-VERIFIED.** The fixture schema could not express an
  enum member value, so `CanonicalStageExpectations` gains `symbol_members_include` and
  `symbol_names_exclude` — the hold-out side, a passing state that is an ABSENCE. New fixture
  `enum_code_collision_holdout_gold` pairs a colliding split-bit table with a clean one. **With the
  gate stashed it FAILS** (`expected symbol_names_exclude to exclude TSIZE, but actual set was
  {"TBURST", "TSIZE"}`, at both `semantic` and `intent`) **and passes with it**, while `TBURST`
  survives either way. `kg-bench` **157/157**, where the enum surface previously had 0 of 156.

  **EFFECT, MEASURED PER MEMBER.** `semantic --dry-run` over all 78 persisted chains with both
  binaries — 27 built / 51 refused as schema-1 — then a per-member diff: **40 member values withdrawn
  across 8 symbols in 2 documents**, `HPROT`/`HSIZE`/`SYSCOACK` withdrawn entirely, `BTAGMATCH` −8,
  `AWSNOOP` −7, `ARSNOOP` −5, `ARCACHE` −4, `AWATOP` −2, and **0 gained**. APB `PPROT`'s collision
  never reached this layer: that document promotes no symbols at all.

  **WIRE-BASED-100 = 1.000 IS DERIVED HERE, NOT ARGUED.** Across all 27 rebuilt SemanticIR artifacts
  the only fields that move are `symbol_definitions` (2 documents) and `proof_ledger` (the same 2,
  which must move when the artifact does). **Every fact surface the scorer reads is byte-identical in
  all 27**, so its input cannot have moved — and AHB's `HTRANS` and `HRESP` survive untouched beside
  the withdrawn `HSIZE` and `HPROT`, which is the over-reach check on the gold itself.
  Verification: acceptance checklist in the checklists part; `cargo fmt` / `clippy -D warnings` clean
  (clippy caught a test call site the product build did not); `cargo test --workspace --lib --exclude
  specforge-production-graph` green (473 / 168 / **1,620**, +2 seam tests). Both affected chains were
  **rebuilt and validated per stage in order — semantic, intent, adapt** — so the persisted chain is
  current rather than left stale.
  Commit: `KG-ISF-COMPLETENESS.5.xiv.a — a value must determine one member, and now a fixture can say so`
