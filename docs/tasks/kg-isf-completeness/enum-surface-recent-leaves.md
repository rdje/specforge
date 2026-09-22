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

- ID: `KG-ISF-COMPLETENESS.5.xiii.a` · Status: `open` (`2026-09-22`; CODE; opened by `.5.xiii`) · Goal:
  **name a member from the column the document names it in — per table, not by score alone.** Land the
  calibrated override at the member seam for the cases `.5.xiii` adjudicated, with the two refusals it
  measured built in: a column of QUANTITIES is not a name column (AxSIZE `Label` = `1 2 4 … 128`), and a
  MULTI-KEY table mints a residual instead of an enum (7.0% of the population).
  **The oracle is per-MEMBER and the wire golds are in scope**, unlike `.5.xi.a`: all 12 replayable
  pairs are AXI/AHB golds, so WIRE-BASED-100 must be scored before and after with both binaries rather
  than argued from a byte-identical replay.
  Non-goal: `value_col` inference (`.5.xiv`); the caption gate (`.5.vi.a`, landed).
  Prerequisite: `.5.xiii`. Verification: `evidence --dry-run` over the 27 replayable chains with a
  per-member diff; WIRE-BASED-100 = 1.000 scored on rebuilt golds with both binaries; kg-bench 156/156;
  the adjudicator re-run showing the intended members move and AxSIZE's do not.

- ID: `KG-ISF-COMPLETENESS.5.xiv` · Status: `open` (`2026-09-22`; MEASUREMENT then CODE; opened by
  `.5.xiii`) · Goal: **nine replayable enums publish one code on two members, and all nine are in the
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
