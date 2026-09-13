# PROSE-NAME-CELL-DECLARATION: a row whose name cell is a phrase declares its first word as a signal

## Metadata

- Tree ID: `PROSE-NAME-CELL-DECLARATION`
- Status: `active` (`2026-09-14`; `.0`-`.5` done; no eligible frontier — see below)
- Roadmap lane: `R2` (extraction correctness / false-positive control)
- Created: `2026-09-11`
- Last updated: `2026-09-14`
- Owner: repo-local workflow

## Goal

`synthesize_signal_declarations` reads a row's name cell by taking its **first whitespace token**,
trimming the token's non-identifier characters, and asking `is_hardware_signal_token`. That is right
for `HSELx a` (a footnote marker) and for `readdatavali d readdatavali` (a PDF text-layer artefact of
one real signal). It is wrong for a cell that is a **phrase**: `Backwards Compatibility with legacy
MMCcard` declares a signal called `Backwards`, `Row Command/Address` declares `Row`, `OEM ID`
declares `OEM`, `a opcode` declares `a`.

Establish how many declarations in the corpus are minted this way, and separate the phrase cells from
the several legitimate multi-word forms — then decide what, if anything, can be refused without
losing a real wire.

## How it was found

`SIGNAL-DECLARATION-ROW-DROP.2c` was going to teach the reader that a width cell listing legal widths
(`8, 16, 32, 64`) is a width set. Its census found 7 such cells. Adjudicating them — before writing
code, which is this repository's standing rule — showed that **4 of the 7 are not signal widths at
all**: they are the `Bus Width` column of eMMC `table_0020`, a bus-mode matrix
(`Mode Name | Data Rate | IO Voltage | Bus Width | Frequency | Max Data Transfer`) that SourceIR typed
`signal_description`. Its name column holds mode names.

That table already mints a signal today. Its `HS400` row is the one whose Bus Width cell is a single
value (`8`) rather than a list, so it parses, and the persisted EvidenceIR for `jesd84_b50…` carries
**`HS400` in `table_signal_declaration_provenance` for `table_0020`**. The other four rows are held
back only by the width notation `.2c` was about to teach. Reading it would have taken that table from
one phantom signal to five — the exact over-firing this repository keeps re-learning
(`[[base-name-template-table-is-not-a-catalogue]]`, `[[property-table-is-not-a-signal-inventory]]`,
`[[bracketed-metavariable-name-cell]]`).

## Non-Goals

- Do not refuse every multi-word name cell. Several are legitimate and load-bearing: a comma-separated
  family (`AWSIZE, ARSIZE`), a footnote marker (`AWUNIQUE a`), a name plus a bit range
  (`response [1:0]`), and a text-layer split of one identifier
  (`waitrequest waitrequest _ n`). `signal_names_in_name_cell` already handles the first of these.
- Do not solve it at the table level. `WIRE-BASED-100.10e` already refuses a *property* table by its
  caption, and that mechanism does not see a mode matrix. This tree is about the **row**.
- Do not add document, vendor, or protocol vocabulary, and do not add an English word list (ADR 0006).
  A rule that needs to know `Backwards` is a word and `AWSIZE` is not has not been found yet, and the
  first leaf is allowed to conclude that no such rule exists cheaply.

## Acceptance Criteria

- The population is measured **through the reader**, not through an approximation: how many entries in
  the persisted `table_signal_declaration_provenance` come from a name cell that is a phrase, per
  document, with every distinct cell form listed for adjudication.
- Any rule shipped carries a corpus-wide count of what it newly refuses **and an adjudicated sample**,
  and the legitimate multi-word forms above are each shown to survive it.
- No gold score moves down; the wire golds are re-scored, not assumed. AXI is the risk here — 35
  candidate rows, of which the `AWSIZE, ARSIZE` comma families are real.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `PROSE-NAME-CELL-DECLARATION` · Status: `active` (`2026-09-14`) · Children: `.0`-`.5`

- ID: `PROSE-NAME-CELL-DECLARATION.0` · Status: `done` (`2026-09-12`) · Goal: **measure before proposing
  anything.** A read-only census over persisted SourceIR + EvidenceIR pairs: for every entry in
  `table_signal_declaration_provenance`, recover the source row that produced it and classify its name
  cell. Delivered by `scripts/measure_declaration_name_cell_shapes.py` (read-only, deterministic).
  Result below; it corrected the opening approximation and answered both Open Questions.
  Verification: read-only; no artifact written or mutated; `git status` clean apart from the tracked
  deliverables.
  Commit: `PROSE-NAME-CELL-DECLARATION.0`

- ID: `PROSE-NAME-CELL-DECLARATION.1` · Status: `done` (`2026-09-12`) · **No rule shipped, and the
  measurement is why.** The leaf was to refuse a `phrase` name cell, ordered after the four legitimate
  shapes. `.2` took its current-stratum population to 0, so its whole remaining justification was to be
  the guard `SIGNAL-DECLARATION-ROW-DROP.2c` waits on. Measured against exactly that population, it is
  the wrong guard — see `.1` below. Superseded as a guard by `.4`, which dominates it on the same rows.
  Verification: read-only; `python3 scripts/measure_declaration_name_cell_shapes.py --guard-population`.
  Commit: `PROSE-NAME-CELL-DECLARATION.1`

- ID: `PROSE-NAME-CELL-DECLARATION.2` · Status: `done` (`2026-09-12`) · Goal: **the content-based
  name-column override cannot fire on a short table, and that was the whole current-stratum phrase
  population.** AHB `table_0004` is rotated exactly like `table_0033` (`Name | Source | Width |
  Description` header over a body holding `Clock source | 1 | <description> | HCLK`), but it has **two
  body rows**, so the real name column scored 2 distinct tokens against the header column's 2 and
  `NAME_COLUMN_OVERRIDE_MARGIN = 2` could never be cleared.
  The census falsified the leaf's own proposed direction and produced a different rule; both are below.
  Shipped: `name_cell_is_read_whole` — a cell scores for its column only when the reader consumes it
  entirely, reusing `.0`'s shape taxonomy. Scoring only; no row's declaration changes by this test.
  Commit: `PROSE-NAME-CELL-DECLARATION.2 / PRODUCTION-GRAPH-CENSUS-PIN.0`

- ID: `PROSE-NAME-CELL-DECLARATION.3` · Status: `done` (`2026-09-14`, CODE) · Children: `.5` · Goal: **a width cell that
  is a sentence is not a parametric width.** `parse_table_width_hint_text` admits any cell holding one
  ASCII letter, so a table whose width column has been handed a *description* declares the description
  as the wire's width.
  **The re-measurement the deferral demanded was run first, and it moved every number in this node.**
  The leaf was deferred with the rule in hand and one instruction attached — re-measure, because `.2`
  had removed the population it was sized against. Measured through a tracked census
  (`scripts/measure_parametric_width_cell_shapes.py`, which replays the reader's own column selection
  and reproduces **601 of 601** declared widths in the proof-carrying stratum):

  - the current stratum holds **195 parametric width cells and 0 prose** — not the 6 recorded here.
    Nothing in it moves, at either the cell or the declaration level;
  - the wider legacy population **falsifies this node's own margin**. It recorded "the widest
    legitimate expression is 5 tokens and the narrowest prose is 7"; real expressions reach **8**
    (`LTI_MMU == True: 64 LTI_MMU == False: LTI_LRADDR_WIDTH`) and prose starts at **7**, so the two
    classes overlap on length and no threshold separates them.

  **Both conditions ship, and the census measured what each would cost alone** — which is the
  justification the original 5-vs-7 margin was standing in for. The token bound alone refuses
  `ceil((ID_R_WIDTH+1)/8) if ARIDUNQ is not present: ceil(ID_R_WIDTH/8)`; the terminator bound alone
  refuses the three ternaries (`LTI_GPC == True ? 2:1`, `LTI_SSID_WIDTH > 0 ? 1:0`,
  `LTI_MMU ? 8 : ceil(LTI_LRADDR_WIDTH/8)`), where `?` is an operator. Together: **47 cells, every one
  prose, no legitimate expression in either stratum.**
  **The blocker is gone and was verified gone, not assumed.** It was deferred because applying the rule
  made AHB's phantom `Manager` declaration well-formed and took its interface count 42 → 62;
  `[[ACTOR-NOUN-RELATION-DECLARATION]]`.1 removed that declaration, and AHB's EvidenceIR is byte-unchanged
  under the guard.
  **What it refuses, adjudicated**: 43 of the 47 are TileLink rows whose declared NAME is a single
  letter — the cause `.5` measured and `[[TEXT-LAYER-IDENTIFIER-SPLIT]]` owns; 2 are MMU-700 field rows
  carrying a paragraph; 2 are real
  identities (GIC `ARCHREV`, CXS `CXSCNTL`) declared width-only with a fabricated width, which become
  rows the `.1` accounting counts as unread. That is `SIGNAL-DECLARATION-ROW-DROP.2e`'s trade exactly:
  a visible refusal over a width the document never states.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `PROSE-NAME-CELL-DECLARATION.3`

- ID: `PROSE-NAME-CELL-DECLARATION.5` · Status: `done` (`2026-09-14`, PROBE/DOC) · **The leaf's premise was
  false and the census said so before anything was designed.** It opened on `.3`'s reading that
  TileLink loses its name column *because* `.2`'s whole-cell score zeroes a column of two-token names.
  Measured, TileLink's name column scores **1** under the score `.2` replaced and **0** under the one it
  shipped — and the `Type` column scores 4 either way, so `4 >= 1 + NAME_COLUMN_OVERRIDE_MARGIN` fires
  under both. **`.2` did not cause that rotation and removing `.2` would not undo it.**
  What the census does establish is the question `.2` left open: **the whole-cell score's recall cost is
  measured, and it is zero.** Of 573 tables, **11** have a header-named column the rule takes to zero
  while it still leads with two or more identifiers — 1 current, 10 legacy — and every one of the 11 is
  genuinely prose. Six are then overridden and four of those are repairs the corpus wanted (AHB
  `table_0004` → `HCLK`/`HRESETn`, eMMC `table_0213`/`table_0214` → `VOH`/`VOL`/`VIH`/`VIL`, HBM2
  `table_0075` → its `DA…` pin list); the other two are inert tables. **No real signal is lost to it.**
  TileLink's actual cause is a different class and is tracked as `[[TEXT-LAYER-IDENTIFIER-SPLIT]]`: the
  name cell `c opcode` is the single identifier `c_opcode` with its underscore missing, so every row's
  leading token is the same letter and the column can offer only ONE distinct name however it is scored.
  Prerequisite: none. Verification: `python3 scripts/measure_name_column_whole_cell_score.py`, read-only;
  the mirror is imported from `measure_parametric_width_cell_shapes.py` rather than copied, so the two
  cannot drift apart.
  Commit: `PROSE-NAME-CELL-DECLARATION.5`

- ID: `PROSE-NAME-CELL-DECLARATION.4` · Status: `done` (`2026-09-12`) · **The premise was a legacy fact:
  the current classifier already refuses the matrix.** The leaf was to decide whether a bus-mode matrix
  should be refused at the table or at the classifier. Neither: `classified_table_kind` does not type
  eMMC `table_0020` as `signal_description` at all, because every closed role must match a header's
  WHOLE normalized label and the matrix qualifies each with its own subject — `Mode Name`, not `Name`;
  `Bus Width`, not `Width` — while no header or caption word names a signal. The persisted
  `table_kind` is the third premise in this tree to turn out to be evidence about a producer that no
  longer exists.
  Shipped: a control over the real classifier, plus the corpus-wide drift census. No production change.
  Verification: `cargo test -p specforge-core --lib a_matrix_that_qualifies_every_role_…`;
  `python3 scripts/measure_signal_table_classification_drift.py`.
  Commit: `PROSE-NAME-CELL-DECLARATION.4`

## `.0` — census result (`2026-09-12`)

Producer: `python3 scripts/measure_declaration_name_cell_shapes.py`. Read-only; joins every persisted
`table_signal_declaration_provenance` entry back to the source row carrying its name by replaying the
reader's own name-column selection, then classifies that cell by shape.

**Stratify or the number is meaningless.** 51 of the 78 persisted pairs were written by a producer that
no longer exists (`[[declared-spelling-is-the-document-spelling]]`). The boundary is
`EVIDENCE_IR_SCHEMA_VERSION`, read out of the Rust source so a schema bump fails loudly. The strata are
never summed.

| stratum | documents declaring from tables | declarations | joined to a name cell |
| --- | ---: | ---: | ---: |
| current (proof-carrying) | 4 | 604 | **604 (100.0%)** |
| legacy (inspection-only) | 22 | 2,085 | 1,251 (60.0%) |

The join rate is the census's own control. A replay of the reader's name-column selection is worth
nothing unless it recovers the names the reader actually emitted; on the current stratum it recovers
every one. The legacy stratum's 834 misses are the uppercase-folding producer of
`SIGNAL-DECLARATION-ROW-DROP.3` showing through — a folded name is not the cell's spelling, so it
cannot join. That is the same finding from a second direction, not a new defect.

| shape | current | legacy (joined) |
| --- | ---: | ---: |
| `single-token` | 528 | 1,221 |
| `comma-family` | 72 | 0 |
| `footnote-marked` | 2 | 6 |
| `bracket-suffixed` | 0 | 9 |
| `repeated-token` | 0 | 0 |
| **`phrase`** | **2** | **15** |

Per document, the current stratum: AXI-L 462 declarations / **0 phrase**; APB-e 60 / 0; ADIv6 3 / 0;
AHB 79 / **2**. The acceptance criteria named AXI as the risk; it is measured at zero, because all 72
of its multi-token cells are comma families the reader already admits (35 distinct, `AWSIZE, ARSIZE`
through `AWIDUNQ, BIDUNQ, ARIDUNQ, RIDUNQ`).

Both current phrase declarations are AHB `table_0004`: `Clock source` → `Clock`, `Reset controller` →
`Reset`. The legacy 15 are 13 distinct cells across three documents — eMMC (`FFU features`,
`HPI features`, `HPI management`, `FFU status`, `FW configuration`, `TRIM Multiplier`, and a `NOTE 1
Reserved bits should read as '0.'` blob), AXI-H (`ARBURST , INCR`, `ARLOCK zeros,`, `ARSIZE bus`), and
GIC-600 (three description-column sentences).

**The opening approximation does not re-derive, and it over-states the population by ~19×.** The leaf
recorded "345 rows across 28 documents". Restated under its own description — a `signal_description`
row whose name cell has two or more whitespace tokens, whose first token is an identifier, excluding
the comma families — it yields **318 rows across 27 documents**; two other readings of "share a
two-character prefix or suffix" give 310 and 308. None reaches 345, and the original probe is not
tracked. The point is unaffected and is the one the leaf itself made: all of these count *candidate
rows*, and a candidate row is not a declaration. 318 candidate rows correspond to **17 phrase
declarations** corpus-wide (2 current + 15 legacy), because the reader discards a row that offers
neither a direction nor a width long before the name cell's shape matters.

### What the adjudication decided

- **The current-stratum phrase population has one cause, and it is not the name reader.** AHB
  `table_0004` is a rotated table the content-based override declined to correct because the table has
  two body rows and the margin is expressed as an absolute token count. Refusing the phrase would
  suppress `Clock`/`Reset`; correcting the rotation would *read the row properly*. `.2` leads `.1` for
  that reason — and because shipping `.1` first deletes `.2`'s only current-stratum evidence.
- **No wire is lost to `table_0004`.** Its real names `HCLK` and `HRESET` are recovered from AHB
  `table_0033`, where the same rotation wins 19 vs 4 and the override does fire. The harm from
  `table_0004` is two phantom declarations and two prose widths, not a missing signal.
- **A third symptom, same two rows.** Both phantoms carry a whole sentence as their width (`.3`).
  Sixty-two legitimate parametric widths in the same stratum show the mechanism is worth separating.
- **`repeated-token` is 0 in both strata.** The text-layer-split form this tree's Non-Goals protect
  (`waitrequest waitrequest _ n`) never reaches a declaration in the persisted corpus: Avalon, the
  document that carries it, joins 0 of 26 because it is legacy. The protection is still right — the
  shape is real and a re-ingest will surface it — but it costs nothing today, and no rule need be
  weakened to preserve it.
- **eMMC `table_0020` verified exactly as the tree stated**, and the verification changed the tree.
  `HS400` is in `table_signal_declaration_provenance` for `table_0020`; the table is the bus-mode
  matrix; its four other rows are held back only by the enumerated-width notation. But `HS400` and
  `HS200` are **single-token** name cells. No row-shape rule can refuse them, so the row level is
  provably insufficient for this table — `.4`.

## `.2` — census result and shipped rule (`2026-09-12`)

**The leaf's own proposed direction was falsified by measurement, and the census said so before any
code was written.** `.2` was written to test a *ratio* margin (`best >= header x 2`) in place of the
absolute one. Measured over all 602 persisted `signal_description` tables, a ratio rule changes 10
tables, does **not** fix AHB `table_0004` — both columns score 2, and 2/2 is ratio 1.0 — and its one
current-stratum change *breaks* AHB `table_0034`, a rotation that works today. It fixes nothing and
costs one. Recorded rather than quietly replaced, because the next author will otherwise propose it
again.

What the data supports instead: the two columns are not distinguishable by *how many* leading
identifiers they carry, but by whether the reader has anything **left over** after reading each cell.
`Clock source` leaves the word `source`; `HCLK` leaves nothing. That discriminator already existed —
it is `.0`'s shape taxonomy — so the rule reuses it rather than inventing a heuristic:

> A cell contributes to its column's score only when the reader consumes it whole:
> a single token, a comma family `signal_names_in_name_cell` already admits, a footnote marker,
> a bit-range suffix, or a text-layer split. Prose scores nothing.

`NAME_COLUMN_OVERRIDE_MARGIN` is untouched at 2, and a margin of 1 was measured: it changes **nothing**
in the current stratum and 5 legacy tables, so the conservative value stands and only one variable
moved.

### What it changes, corpus-wide

7 of 602 current-stratum tables change their name column:

| table | change | adjudication |
| --- | --- | --- |
| AHB `table_0004` | col 0 → 3 | **the repair** — `Clock source`/`Reset controller` → `HCLK`/`HRESETn` |
| USB 3.2 `table_0106`/`0109`/`0110` | col 2 → 0 | `Width (bits) \| Offset \| Description` register tables mis-typed as signal tables; the override currently hands them their Description column. They declare nothing either way |
| USB 3.2 `table_0208`/`0250`/`0258` | col 1 → 0 | `Key`/`Bit \| Description` state and field tables, same shape, same inertness |

The six USB changes are **proven** inert rather than assumed: the proof seal verifies each persisted
EvidenceIR against a re-derivation, and all 27 proof-carrying artifacts still load. Only AHB's needed
rebuilding, so no other current document's EvidenceIR content moved at all.

In the legacy stratum 39 tables change (19 gain or move an override, 20 lose one). The sample adjudicated:
`MMU-700 table_0202`, `table_0199` and `AXI-H table_0064` currently override a correct `Signal` column in
favour of their **Description** column and stop doing so — a recall repair that a re-ingest will realise;
`HBM2 table_0075` newly finds a real `DA13, DA16, …` pin-list column; the rest are tables mis-typed as
`signal_description`, where both the old and the new column are wrong and neither declares anything.

### What it does to AHB

| | before | after |
| --- | --- | --- |
| `table_0004` declarations | `Clock`, `Reset` | `HCLK`, `HRESETn` |
| their statements | `Signal Clock is width The bus clock times all bus transfers. …` | `Signal HCLK is output width 1.` / `Signal HRESETn is width 1.` |
| document declarations | 79 | 79 |
| IntentIR interface signals | 40 | **41** (`HRESETn` added, nothing removed) |

`HRESETn` — the document's own active-LOW spelling — reaches IntentIR for the first time. `table_0033`
supplies `HRESET`, a text-layer truncation, and supplied it alone until now.

## `.1` — the guard measured against the population it guards (`2026-09-12`)

Producer: `python3 scripts/measure_declaration_name_cell_shapes.py --guard-population`. Read-only.

`.2` took the phrase population in the current stratum to **0**, so `.1` had exactly one remaining
justification: to be the guard `SIGNAL-DECLARATION-ROW-DROP.2c` is deferred waiting for. That guard can
therefore be judged on one population — **the 7 rows an enumerated-width reading would newly admit**,
all of them legacy, in two documents:

| document | table | width cell | name cell | shape | a phrase rule |
| --- | --- | --- | --- | --- | --- |
| Avalon | `table_0011` | `2, 4, 8, 16, 32, 64, 128` | `byteenable byteenable_n` | `phrase` | **refuses a real wire** |
| Avalon | `table_0012` | `8, 16, 32, …, 1024` | `readdata` | `single-token` | admits (correct) |
| Avalon | `table_0012` | `8, 16, 32, …, 1024` | `writedata` | `single-token` | admits (correct) |
| eMMC | `table_0020` | `1, 4, 8` | `Backwards Compatibility with legacy MMCcard` | `phrase` | refuses a phantom |
| eMMC | `table_0020` | `1,4, 8` | `High Speed SDR` | `phrase` | refuses a phantom |
| eMMC | `table_0020` | `4, 8` | `High Speed DDR` | `phrase` | refuses a phantom |
| eMMC | `table_0020` | `4, 8` | `HS200` | `single-token` | **admits a phantom** |

It refuses 4 and admits 3. Three of the four refusals are right and one is a real signal pair; one of
the three admissions is a phantom it cannot see. As a guard it takes `.2c` from 3 real / 4 phantom
(43% precision) to 2 real / 1 phantom (67%) — **and loses `byteenable`.**

### The obvious escape hatch is refused by its own selection

`byteenable byteenable_n` is a pair written with a space instead of a comma, so the natural repair is to
apply `signal_names_in_name_cell`'s shared-affix test to whitespace as well. Measured, that rule selects
**9** cells:

- **real pairs (6)**: Avalon's `reset reset_n`, `read read_n`, `write write_n`, `irq irq_n`,
  `byteenable byteenable_n`, and GIC-600's `icpdtready icpdtvalid`.
- **not signals (3)**: AXI-H `table_0076`'s `WriteClean WriteBack` and `WriteUnique WriteLineUnique`,
  which are **transaction names** sharing the prefix `write`, and USB 3.2's `Enhanced SuperSpeed`, a
  plain phrase whose two words happen to share the suffix `ed`.

The asymmetry is the finding, and it is general: **a comma is an author enumerating; a space is the
default separator between any two words.** The same two-character affix test that is safe after a comma
is a coincidence detector after a space. One in three is not a rule.

### Why `.4` dominates, which is the actual decision

Every phantom in the guarded population is **one table** — eMMC `table_0020`, the bus-mode matrix.
Refuse it at the table or fix the SourceIR classification that typed it `signal_description`, and the
population `.2c` newly admits becomes the three Avalon rows: `readdata`, `writedata`, `byteenable` —
**3 real, 0 phantom, 100% precision**, and a phrase rule at that point does nothing but delete
`byteenable`. A guard that is unnecessary once the real defect is fixed, and harmful in the meantime,
is not a guard.

`.1` therefore ships no rule. The `phrase` class stays what `.0` built it to be — a measurement
instrument and the shape test `.2`'s column score is built on — rather than becoming a refusal.

## `.4` — the persisted `table_kind` is evidence about the classifier that wrote it (`2026-09-12`)

Producer: `python3 scripts/measure_signal_table_classification_drift.py`. Read-only.

The tree opened on a plain reading of a persisted artifact: eMMC `table_0020` carries
`table_kind: signal_description` and mints `HS400`. The conclusion drawn was that the SourceIR
classifier types a characteristics matrix as a signal table. **It does not.** Applying the current
`classified_table_kind`'s two `SignalDescription` paths to every persisted table already carrying that
kind:

| stratum | still typed `signal_description` | would NOT be | declarations minted by the latter |
| --- | ---: | ---: | ---: |
| current (proof-carrying) | 118 (107 caption / 11 header) | **0** | 0 |
| legacy (inspection-only) | 360 (242 caption / 118 header) | **124** | **598** of that stratum's 2,085 |

eMMC `table_0020` is in the legacy 124. It is refused because every closed role must match a header's
**whole** normalized label, and a matrix qualifies each of its roles with its own subject —
`Mode Name` rather than `Name`, `Bus Width` rather than `Width` — while neither a header nor the caption
(`Table 4 - Bus Speed Modes`) names a signal. Two independent conditions refuse it, which is measured
rather than asserted: relaxing `header_has_role` to word containment leaves it refused; forcing
`header_names_signals` to `true` leaves it refused; **only both together admit it**, and that is the
observed RED for the control.

### What this does to the rest of the tree

- **The mode-matrix question needs no new mechanism.** `.4` ships a guard over the real classifier and
  the drift census, and no production change.
- **`SIGNAL-DECLARATION-ROW-DROP.2c`'s deferral reason is dissolved.** The 4 phantom rows it would have
  newly admitted were all eMMC `table_0020`, which the current producer never hands to the declaration
  reader. Its remaining population is the three Avalon rows — `readdata`, `writedata`, `byteenable` —
  all real. `.2c` can be reconsidered on its own merits rather than on a guard.
- **`.1`'s decision is reinforced from a second direction.** Of the 11 legacy `phrase` declarations,
  **8** come from tables the current classifier refuses outright and would simply not exist. The
  **3** that survive are all AXI-H `table_0036` — `ARSIZE bus`, `ARBURST , INCR`, `ARLOCK zeros,` — a
  badly scrambled table whose cells have drifted across columns. Every one of those three names a
  **real signal** with a fragment of its neighbour fused on. So the entire phrase population that
  survives the current producer is real wires, and a phrase-refusal rule would refuse all of it.

### The general form, which outlives the case

`table_kind` is a persisted field, and a persisted field records the producer that wrote it. This tree
has now been wrong about the current producer three times from the same class of evidence: `.3` of the
sibling tree on a folded *spelling*, `.2` here on a *column* choice, and `.4` on a *classification*.
Each time the artifact was read as a statement about behaviour. **Ask which producer wrote the field
before treating it as a defect** — `[[persisted-table-kind-is-a-classifier-generation-artefact]]`.

## `.3` — re-measured, and the re-measurement moved every number (`2026-09-14`)

Producer: `python3 scripts/measure_parametric_width_cell_shapes.py`. Read-only. It measures two
populations and never conflates them: the **declarations** the reader emitted, read straight out of
the persisted `extracted_statements` with no mirror involved, and the **width cells** the parser would
admit, which replays the reader's column selection and therefore carries a join control.

**The boundary was wrong on the first run and the control is what said so.** Scoped to tables whose
persisted `table_kind` is `signal_description`, the join came back at **92.8%**, and every miss was
AXI `table_0251` — `Name | Width | Source | Description`, 24 rows, persisted kind `unknown`, promoted
to a signal table by `effective_table_kind` through corpus memory. The boundary is now that set
**union** the tables named in `table_signal_declaration_provenance`, and the join is **601 of 601**.
A census whose control is a number it can read back is a census that reports its own errors.

### The population, at both levels

| | current (proof-carrying) | legacy (inspection-only) |
| --- | ---: | ---: |
| declaring rows examined | 685 | 2,889 |
| width cells admitted as `Parametric` | **195** | 271 |
| of those, cells that read as prose | **0** | **47** |
| declarations carrying a prose width | **0** of 637 | 2 of 1,272 |
| join control | 601/601 (100.0%) | 631/1,085 (58.2%) |

The legacy join rate is not a failure of the census and is not summed with the current one: an older
emitter folded declared names to upper case, so a folded name cannot be matched back to the cell that
produced it (`[[declared-spelling-is-the-document-spelling]]`, the same finding from a third
direction). Two of its misses are the guard itself, and the census labels them as such.

### What this node recorded, and what is true

`.3` recorded "**6** prose cells in 178 admitted" and a margin of "widest legitimate 5 tokens,
narrowest prose 7". Neither survives:

- the six were all rotated AHB tables, and `.2` repaired the rotation. **The current stratum is 0.**
- legitimate expressions reach **8** tokens (`LTI_MMU == True: 64 LTI_MMU == False: LTI_LRADDR_WIDTH`,
  and `ceil((ID_R_WIDTH+1)/8) if ARIDUNQ is not present: ceil(ID_R_WIDTH/8)` at 7) while the shortest
  prose the rule selects is **7** (`Unique, per-link master source identifier. (Section 5.4)`). The
  classes **overlap on length**. The earlier margin was an artefact of measuring one stratum.

### The two conditions, and what each costs alone

The rule is a conjunction, and the census measured the price of each half rather than asserting it:

| condition applied alone | what it refuses that is REAL |
| --- | --- |
| more than six tokens | `ceil((ID_R_WIDTH+1)/8) if ARIDUNQ is not present: ceil(ID_R_WIDTH/8)`; `LTI_MMU == True: 64 LTI_MMU == False: LTI_LRADDR_WIDTH` |
| a terminator before whitespace | `LTI_GPC == True ? 2:1`; `LTI_SSID_WIDTH > 0 ? 1:0`; `LTI_MMU ? 8 : ceil(LTI_LRADDR_WIDTH/8)` — `?` is an operator, not a question |
| **both** | **nothing. 47 selected, all prose.** |

Each bound is the only thing saving a real width from the other. A footnote marker rides its
expression directly (`ceil(ADDR_WIDTH/8) a`), carries no terminator, and is untouched.

### What it refuses, adjudicated row by row

| what | count | the name the row would declare | verdict |
| --- | ---: | --- | --- |
| TileLink `table_0012`/`0013`/`0014`, both revisions | 43 | `C`, `D`, `V`, `R`, `F` | a phantom removed — and the cause is `.5` |
| MMU-700 `table_0025` | 2 | `mtlbidx`, `mtlbway` | a 30-to-36-token paragraph removed; the row keeps whatever else it states |
| GIC `table_0094`, CXS `table_0012` | 2 | `ARCHREV`, `CXSCNTL` | **the measured cost** — both are declared width-only, so the declaration goes, and the row is counted as `NoDirectionAndNoWidth` |

The two lost identities are the `SIGNAL-DECLARATION-ROW-DROP.2e` trade, taken deliberately: a row
counted as unread over a width the document never states. `ARCHREV`'s "width" is a bulleted list of
architecture revisions; `CXSCNTL`'s is a sentence pointing at another table.

### The current corpus does not move, and that was measured rather than argued

`evidence --dry-run` was replayed for **all 27 rebuildable documents** — the 24 with a retained
normalized bundle, plus AHB, APB and AXI restored from
`generated/preserved/WIRE-BASED-100.10/*-normalized-bundle-held-out` for the run and removed again —
and every one reproduces its persisted artifact byte for byte outside `validation_reports`,
`proof_context` and `proof_ledger`. The preserved bundles are byte-identical before and after
(`ahb a32ad8ab…78b90`) and retention is back at exactly **24**. The four current-stratum documents
that declare from tables still LOAD through the semantic stage, so no proof was staled and no chain
rebuild is owed.

### A document that does NOT reproduce, and it is not this leaf's

The same sweep found **I2C (`um10204_rev7_0_2021_i2c_bus_specification`) does not reproduce**: its
persisted EvidenceIR carries 9 signal constraints and 21 fact-provenance records where the current
binary produces 3 and 15, with 13 conditional rules renumbered. **Proven independent of this change** —
the delta is byte-identical with the guard and with `git stash`'d HEAD — and it predates
`EXTRACTION-QUALITY-GAUGE.3k.1`, which also fails to reproduce it. Tracked as
`CORPUS-CHAIN-CURRENCY.4`; it is a chain-currency finding, not a width one.

## `.5` — the whole-cell score's recall cost, measured (`2026-09-14`)

Producer: `python3 scripts/measure_name_column_whole_cell_score.py`. Read-only. It imports its reader
mirror from `measure_parametric_width_cell_shapes.py` instead of copying it, because two copies of one
producer's logic is the drift this tree has now found in its own instruments twice.

The leaf existed to answer a recall worry `.2` created: if a cell scores for its column only when the
reader consumes it whole, what happens to a column of real names the reader does **not** consume whole?
The population is every table whose header-designated column scores **0** under the whole-cell rule
while still leading with two or more identifiers — exactly the columns the rule zeroes rather than
merely lowers.

| stratum | tables | name column zeroed | then overridden | of those, declaring |
| --- | ---: | ---: | ---: | ---: |
| current (proof-carrying) | 116 | **1** | 1 | 1 |
| legacy (inspection-only) | 457 | **10** | 6 | 1 |

**All eleven are genuinely prose, and the overrides are repairs.** The one current case is AHB
`table_0004` — `Clock source` / `Reset controller` zeroed, the column moved to `HCLK` / `HRESETn`, which
is `.2` working exactly as it was written. In the legacy ten: eMMC `table_0213` and `table_0214` move
`Output HIGH voltage` to the `Symbol` column's `VOH`/`VOL`/`VIH`/`VIL`, HBM2 `table_0075` moves
`Point to Point` to its `DA13, DA16, …` pin list, and DTI `table_0052` moves a direction sentence to a
`*_DTI_DN` suffix column. The remaining four keep their column and declare nothing. **No real signal is
lost to the rule anywhere in the corpus.**

### The premise this leaf was opened on is false, and the correction is the point

`.3` read TileLink's rotation as this rule's doing: a name column of two-token names scoring zero. The
profile says otherwise.

| TileLink `table_0012` column | whole-cell score (`.2`) | leading-token score (before `.2`) |
| --- | ---: | ---: |
| 0 `Signal` — `c opcode`, `c param`, … | 0 | **1** |
| 1 `Type` — `C`, `D`, `V`, `R` | **4** | **4** |

Under *both* scores the `Type` column clears `NAME_COLUMN_OVERRIDE_MARGIN` against column 0, so the
rotation predates `.2` and deleting `.2` would not undo it. The real cause is that every row's leading
token is the same letter: `c opcode` is the single identifier `c_opcode` with its underscore absent, so
the column can offer only **one** distinct name however it is scored.

Corroboration from a second document: eMMC `table_0221` is in the same list with `t PERIOD` and
`t TLH , t THL` — `tPERIOD` and `tTLH`/`tTHL` split the same way — and it **declares**. The class is not
one specification's typesetting.

**Where the underscore goes.** The TileLink PDF's own text layer carries `a_opcode`, `d_valid` and eight
other underscored names — 36 occurrences on 5 pages, all in code listings — while the surrounding prose
and every table cell spell them with a space. SpecForge's SourceIR for that document carries **zero**
underscores anywhere: not in `content_elements`, not in a table cell. So the evidence that would rejoin
the two tokens exists in the document and does not survive ingest, which is why the repair cannot be
designed at this reader and is tracked as its own tree.

## Acceptance Checklist (enforced) — `.2`, the tree's only production change
- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_declaration_name_cell_shapes.py`: current stratum
  604 declarations, **2 phrase**, both AHB `table_0004`. `specforge eval-extraction` + the persisted
  EvidenceIR show that table's two statements carrying a 120-character sentence as their width.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs:10488` (`signal_token_distinct`
  inside `synthesize_signal_declarations`). Measured, not inferred: for `table_0004` the header column
  and the real name column both score 2, so `NAME_COLUMN_OVERRIDE_MARGIN = 2` cannot be cleared by any
  margin value. `bash scripts/check_chain_currency.sh` named the same table as the one stale document.
- [x] **ADDRESSED (verified)** — AHB `table_0004`: `Clock`/`Reset` → `HCLK`/`HRESETn`; both prose widths
  gone; declarations 79 → 79; IntentIR interface signals 40 → 41 with nothing removed. Current-stratum
  phrase declarations **2 → 0**; legacy 15 → 11. Corpus-wide selection: 7 current tables, adjudicated
  above.
- [x] **NO REGRESSION** — `cargo test` green (472 / 168 / 1423 / 4 — the last after
  `PRODUCTION-GRAPH-CENSUS-PIN.0`); `cargo fmt --check` and `cargo clippy --all-targets -D warnings`
  green; `scripts/check_doctrines.sh` green; `scripts/check_chain_currency.sh` **fully current** —
  evidence 24/24, semantic 27/27, intent 27/27, isf-adapter 27/27, retention exactly the declared 24
  bundles. kg-bench 156/156 via `corpus-kb-currentness`. Every rotation the margin exists for still
  overrides (APB `table_0016` 18 vs 5, AHB `table_0033` 19 vs 4, AHB `table_0034` 5 vs 3).
  **Stated limit, because a green score would otherwise be read as proof:** `seed_ahb` holds at
  `signal_constraint` 1.000 and `actor_signal_relation` 0.500, but it **names none of `HCLK`,
  `HRESETn`, `Clock` or `Reset`** (measured: 0 occurrences in the 17-item gold), so that gold was
  structurally unable to move either way. The evidence for this change is the artifact diff and the
  chain oracle, not the gold.
- [x] **GENERICITY (ADR 0006)** — `name_cell_is_read_whole` tests token shape only: a comma family the
  reader already admits, a one-character token, a token with no letter, a recurrence with fragments.
  No document, vendor, protocol or English vocabulary; no list of words.
- [x] **LOCKSTEP** — `docs/book/src/pipeline/evidenceir.md` updated in the same commit: the section that
  described `table_0004` as an open choice between silencing and repairing now states what shipped and
  what it recovered. No production rule was deleted or replaced, so no book text describes a behaviour
  that is now gone. Knowledge-map card `[[declared-population-is-not-the-candidate-row-population]]`
  updated with the new numbers, the new `reverify` expectation, and the mirror-drift finding below.

## Acceptance Checklist — `.4` (test-only Rust change)

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_signal_table_classification_drift.py`: eMMC
  `table_0020` carries `table_kind: signal_description` in the persisted artifact and mints `HS400`;
  124 legacy tables minting 598 declarations carry that kind, 0 in the current stratum.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source.rs` `classified_table_kind` /
  `header_has_role`: a closed role must match a header's whole normalized label, so `Mode Name` offers
  no `name` role; and `header_names_signals` finds no signal noun, so
  `has_direction || (has_explicit_signal && has_width)` fails. Verified by running the real function:
  `cargo test -p specforge-core --lib a_matrix_that_qualifies_every_role_…` passes on the unmodified
  tree. The persisted kind is therefore a *legacy* classifier's output, not current behaviour.
- [x] **ADDRESSED (verified)** — no production change is warranted and none was made; the leaf ships a
  control plus the census. The control is observed RED only when BOTH conditions are relaxed
  (`header_has_role` → word containment, and `header_names_signals` → `true`); either alone leaves the
  matrix refused. It carries a GREEN control so it cannot pass by refusing everything.
- [x] **NO REGRESSION** — `cargo test` 472 / 168 / **1419** / 4 green; `cargo fmt --check` and
  `cargo clippy --all-targets -D warnings` green; `scripts/check_doctrines.sh` green. Test-only change:
  no artifact rebuilt, no persisted content touched, so chain currency is unaffected.
- [x] **GENERICITY (ADR 0006)** — the control asserts the same refusal over the matrix's own words and
  over an alpha-renamed copy with every document word replaced, so the property is structural. No rule
  was added, so no vocabulary was added.
- [x] **LOCKSTEP** — book paragraph added to `docs/book/src/pipeline/evidenceir.md` (a legacy count is a
  fact about files); fact card `[[persisted-table-kind-is-a-classifier-generation-artefact]]` created.
  No production rule deleted or replaced, so no book text describes a behaviour that is now gone.

## Acceptance Checklist (enforced) — `.3` (production change)

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_parametric_width_cell_shapes.py`: current
  stratum 195 parametric width cells / **0 prose** / 0 declarations; legacy 271 / **47** / 2. Join
  control 601/601 on the proof-carrying stratum, which is what caught the census's own first boundary
  (`table_kind`-only: 92.8%, every miss AXI `table_0251`, a `table_kind: unknown` promoted by corpus
  memory).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`
  (`parse_table_width_hint_text`): a cell holding one ASCII letter is returned as
  `WidthHint::Parametric` with nothing else asked of it, so a description in the width column becomes
  the wire's width. Three call sites, all inside the width-hint path.
- [x] **ADDRESSED (verified)** — `width_expression_reads_as_prose` refuses a cell that carries more
  than six tokens **and** a sentence terminator before whitespace. Selection: 47 cells corpus-wide,
  every one prose, adjudicated in the table above; 0 legitimate expressions in either stratum.
- [x] **NO REGRESSION** — `cargo test` 472 / 168 / **1531** / 8 green (+2 controls; the
  production-graph suite green after the flow census was rebased to this leaf); `cargo fmt --check`
  and `cargo clippy --all-targets --all-features -D warnings` green; `scripts/check_doctrines.sh`
  green. **The corpus does not move**: `evidence --dry-run` replayed over all 27 rebuildable documents
  reproduces every persisted artifact, the three held-out bundles are byte-identical before and after
  their restore, retention is back at 24, and the four table-declaring current documents still load
  through the semantic stage — so no proof was staled and no chain rebuild is owed.
  **Stated limit:** no gold can move either way. The refusal's whole population is legacy, and a
  legacy document has no gold and cannot be rebuilt; the evidence here is the artifact replay and the
  in-crate controls, not a score.
- [x] **GENERICITY (ADR 0006)** — token count and punctuation shape only. No document, vendor,
  protocol or English vocabulary; no word list. The controls run the same assertions over
  alpha-renamed copies of every corpus form.
- [x] **LOCKSTEP** — `docs/book/src/pipeline/evidence-failure-modes.md` gains the section for this
  rule, in the chapter that already owns the table reader's failure modes; the fact card
  `[[a-width-cell-that-is-a-sentence-is-not-a-width]]` records the conjunction and its measured
  margin. No production rule was deleted or replaced, so no book text describes a behaviour that is
  now gone.

## Current Frontier

No eligible frontier. All six leaves are closed and the tree's question is answered in both directions:
a phrase name cell must not be refused (`.1`), a prose column must not score (`.2`), a prose width must
not be read (`.3`), the matrix is already refused upstream (`.4`), and the whole-cell score costs no
recall (`.5`). The one live defect this tree found and does not own is the split identifier, which is
`[[TEXT-LAYER-IDENTIFIER-SPLIT]]`; reopen here only if a NAME-CELL question comes back with a measured
population.

## Decisions

- `2026-09-11` — **opened as its own tree rather than a leaf of `SIGNAL-DECLARATION-ROW-DROP`.** That
  tree is about rows the reader *drops*; this is about rows it *should* drop. They meet at one function
  and nowhere else, and folding a precision question into a recall tree would make both frontiers
  unreadable.
- `2026-09-11` — **the first leaf is a census with no code change.** Three separate rules in this
  reader have now over-fired until their selection was inspected, and the one number available today
  (345) is already known to contain false positives. A rule proposed before the adjudication would be
  the fourth.
- `2026-09-12` — **the census ships as a tracked reproducer, not as a number in this file.**
  `.0`'s stated non-goal was "any code change"; a read-only measurement script is not one, and the
  alternative — an untracked probe whose result is quoted here — is exactly how the 345 figure became
  unverifiable within one session. Precedent: `scripts/measure_declaration_row_notations.py`.
- `2026-09-12` — **`.2` is sequenced before `.1`.** The rule and the root cause address the same two
  declarations; whichever ships first makes the other's current-stratum evidence disappear. The root
  cause goes first, because correcting the rotation recovers the row and refusing the phrase only
  silences it.
- `2026-09-12` — **`.2`'s stated direction was dropped after the census contradicted it.** The leaf asked
  for a *ratio* margin. Measured, it fixes none of the case it was written for and breaks a working
  rotation. The alternative was found by looking at what actually distinguishes the two columns, which
  is the same question `.0` had already answered. Written down rather than replaced silently.
- `2026-09-12` — **the margin stayed at 2.** A margin of 1 under the new score changes nothing in the
  current stratum. Two variables were available and only one needed to move.
- `2026-09-12` — **`.1` was kept for the guard, then closed by measuring it against the guarded
  population.** `.0` justified the leaf by "3 of the 4 rows `.2c` would newly mint in eMMC
  `table_0020`". Measured over all 7 rows rather than that one table, the rule also refuses Avalon's
  `byteenable byteenable_n` and still admits `HS200`. The number that justified the leaf was true and
  the leaf was still wrong, because it was counted on a subset of its own population.
- `2026-09-12` — **no whitespace-family rule.** Its selection is 6 real pairs and 3 false positives,
  and the reason is structural rather than a tuning problem: a comma is an author enumerating.

- `2026-09-14` — **`.3` shipped on a MEASURED zero current-stratum effect, not on a live defect.**
  The precedent is `SIGNAL-DECLARATION-ROW-DROP.2e`: the class is demonstrable through the real reader
  and the risk is measured at zero, which is not the same as an unmeasured zero effect. What
  distinguishes it from a change that should wait is that the measurement ran over every rebuildable
  document rather than over the tables the rule was written for.
- `2026-09-14` — **the node's own margin was wrong and the correction is the leaf's main result.**
  "Widest legitimate 5 tokens, narrowest prose 7" was measured on the proof-carrying stratum alone.
  Over the whole corpus the classes overlap on length, so the rule is a conjunction whose two halves
  each save a real width from the other — and each half's cost was measured rather than argued.
- `2026-09-14` — **the census's boundary was corrected by its own control, before any number was
  published.** A `table_kind == signal_description` boundary misses a table corpus memory promotes,
  and the join rate said so at 92.8%. A mirror without a control is an assertion.
- `2026-09-14` — **the TileLink finding was split to `.5` rather than folded into `.3`.** 43 of the 47
  refusals are one cause — a real name column scoring zero — and that is a recall defect in `.2`'s own
  rule. Refusing the prose width there is right whatever `.5` decides, but the two must not be argued
  as one change; this tree's characteristic finding is that the repair of a cause goes first and
  separately from the rule that changes how the defect looks.

- `2026-09-14` — **`.5` closed as a measurement, with no rule, because the rule it was opened to justify
  had no defect to repair.** The leaf inherited its premise from `.3`'s adjudication rather than from a
  measurement, and the first thing the census did was falsify it. That is the fourth premise in this tree
  to come from reading an artifact instead of running the producer; the pattern is now general enough to
  be the tree's closing note.
- `2026-09-14` — **the second census imports the first's mirror rather than copying it.** `.0`'s card
  already records that a mirror which does not move with its producer reports a defect that is its own.
  Two copies guarantee that eventually.

## Open Questions

- ~~Is there a shape-only discriminator at all?~~ **Answered by `.0`, for the forms that occur.** Five
  shapes separate the population without vocabulary: `comma-family`, `footnote-marked`,
  `bracket-suffixed`, `repeated-token`, `phrase`. Each of the legitimate forms this tree set out to
  protect lands in its own class, and `phrase` holds only cells that are prose. What remains
  irreducible is not phrase-vs-split but **phrase-vs-single-token**: `HS400` is a mode name and
  `HRESETn` is a wire, and no property of either cell tells them apart.
- ~~Does the answer belong at the row, the table, or the SourceIR classifier?~~ **All three, and `.0`
  says which where.** The row answers AXI, APB and the 15 legacy phrases. The *column* answers AHB
  `table_0004` (`.2`). Only the table or the classifier can answer eMMC `table_0020` (`.4`).

## Blockers

None.

## Verification Log

- `2026-09-14` — `.5`. `python3 scripts/measure_name_column_whole_cell_score.py` over all 573 boundary
  tables: 11 zeroed name columns, each printed verbatim with its headers, its sample cells, and the
  winning column's injectivity, and each adjudicated above. Read-only: no artifact written, rebuilt or
  mutated; no network, clock or randomness. The falsifying profile was computed by running the census's
  own `column_profile` over TileLink `table_0012`/`table_0013` — column 0 scores 1 under the pre-`.2`
  leading-token rule and 0 under the shipped whole-cell rule, while `Type` scores 4 under both. The PDF
  text-layer counts come from `pypdf` over
  `.cache/local-references/chipdoc/risc-v/interfaces/tilelink/current/TileLink-1.8.0_Specification.pdf`
  (repository-volume, read-only): 36 underscores on 5 pages, 10 distinct underscored names, against 0 in
  the persisted SourceIR.

- `2026-09-14` — `.3`. Controls, both **observed RED** against the exact defect they guard:
  `a_description_sentence_is_not_a_parametric_width` runs the real `synthesize_signal_declarations`
  over TileLink `table_0013`'s shape, alpha-renamed, and without the guard declares
  `Signal C is width Operation code. Identifies the type of message carried by the channel.
  (Table 5.2).`; `a_width_expression_reads_as_prose_only_when_both_conditions_agree` was observed RED
  **once per condition, applied alone** — with only the token bound it refuses
  `ceil((ZETA_R_WIDTH+1)/8) if ZETAIDUNQ is not present: …`, with only the terminator bound it refuses
  `ZETA_GPC == True ? 2:1`. Corpus replay: `evidence --dry-run` over all 24 retained documents plus
  AHB/APB/AXI restored from `generated/preserved/WIRE-BASED-100.10/` and removed again — **27 of 27
  reproduce**, preserved bundles byte-identical (`ahb a32ad8ab…78b90`, `apb 14128bbf…6651`,
  `axi e1f69f08…f113`), retention back at 24, and `semantic --dry-run` still loads all four
  table-declaring current documents. Census: `python3 scripts/measure_parametric_width_cell_shapes.py`,
  read-only, no artifact written or mutated, join control 601/601.
  **Out-of-scope finding, proven independent**: I2C does not reproduce, identically with the guard and
  with HEAD's `evidence.rs` restored by `git stash`, and also fails to reproduce at `1ada364a`
  (`EXTRACTION-QUALITY-GAUGE.3k.1`). Routed to `CORPUS-CHAIN-CURRENCY.4`; no attempt was made to fix it
  inside this leaf.

- `2026-09-12` — `.3`. The rule was implemented, controlled and measured, then **reverted**; the tree
  keeps it so re-applying is mechanical. Controls (both passed with the guard, RED without):
  `a_description_sentence_is_not_a_parametric_width` — without the guard the declaration comes back as
  `Signal ZETA_ALPHA is width The zeta clock times all zeta transfers. …`, while
  `ceil((ID_W_WIDTH + int(Unique_ID_Support))/8)` and `ceil(ADDR_WIDTH/8) a` survive; and
  `a_width_expression_reads_as_prose_only_when_both_conditions_agree` over each form.
  Blast radius measured on the artifact: AHB's bundle restored from
  `generated/preserved/WIRE-BASED-100.10/`, the chain rebuilt in the interleaved order, diffed against
  `generated/preserved/PROSE-NAME-CELL-DECLARATION.3/pre-rebuild/`, then **rolled back to exactly those
  bytes** and `scripts/check_chain_currency.sh` re-run to prove the corpus is current again. The
  preserved bundle is byte-identical before and after and retention stays at the declared 24.
- `2026-09-12` — `.4`. Control: `a_matrix_that_qualifies_every_role_with_its_subject_is_not_a_signal_table`
  runs the real `classified_table_kind` over the matrix's exact shape, over the same shape alpha-renamed
  (ADR 0006), and over a GREEN control with unqualified roles that must still be a signal table. It
  passes on the unmodified tree — a guard, not a fix, and the difference is stated rather than blurred.
  **Observed RED with two probes applied together**: `header_has_role` relaxed from whole-label equality
  to word containment, plus `header_names_signals` forced to `true`. Either probe alone leaves the
  matrix refused, which is the measurement behind "two independent conditions".
  Census mirror cross-check: 100% agreement with the persisted `table_kind` across the whole
  proof-carrying stratum (118/118), so a divergence there would indict the mirror rather than the code.
- `2026-09-12` — `.1`. Read-only decision leaf; no code change, no artifact written or mutated.
  `python3 scripts/measure_declaration_name_cell_shapes.py --guard-population` — 7 enumerated-width
  rows with their name-cell shapes, and the 9 whitespace-family candidates, both printed verbatim for
  adjudication. The default census mode is unchanged and still reports current 604/604, 0 phrase.
- `2026-09-12` — `.2`. Controls, both **observed RED** against the exact defect they guard, not asserted:
  `a_short_rotated_table_is_scored_by_the_cells_the_reader_reads_whole` fails with
  `["Clock", "Reset"]` when the `name_cell_is_read_whole` filter is removed from the score; and
  `a_comma_family_name_column_keeps_its_score` fails with
  `["PROT_Present", "RME_Support", "INSTPRIV_Present"]` when the comma-family clause is removed from
  `name_cell_is_read_whole` — which is what earns that clause its place, since a naive single-token
  score would hand AXI's name column to its Presence column. `a_cell_scores_for_its_column_only_when_the_reader_reads_all_of_it`
  pins the shape test on each form the census named.
  Chain repair: AHB's normalized bundle restored from
  `generated/preserved/WIRE-BASED-100.10/ahb-normalized-bundle-held-out`, the chain rebuilt in the
  documented interleaved order (`[[retained-chain-rebuild-order]]`) with exactly one `validate` per
  artifact, and the bundle removed again — the preserved copy is byte-identical before and after
  (`a32ad8ab…78b90`), retention stays at exactly the declared 24, and the pre-rebuild artifacts are
  held at `generated/preserved/PROSE-NAME-CELL-DECLARATION.2/pre-rebuild/`.
  **A pre/post A/B on the gold was attempted and is impossible by design**: the current binary refuses
  the pre-rebuild EvidenceIR (`registered derivation … output or input topology is stale`), which is
  the seal doing its job. The comparison recorded above is therefore the artifact diff plus the chain
  oracle, and the gold's own inability to move is stated rather than used.
- `2026-09-12` — `.0`. `python3 scripts/measure_declaration_name_cell_shapes.py` over all 78 persisted
  `generated/source_ir` + `generated/evidence_ir` pairs. Read-only: no artifact written, rebuilt, or
  mutated; no network, clock, or randomness. Join control: current stratum 604/604 (100.0%), 0
  ambiguous. Stratum boundary read from `EVIDENCE_IR_SCHEMA_VERSION` in
  `crates/specforge/src/ir/evidence.rs`, not restated. Spot-checks against the persisted artifacts:
  eMMC `table_0020` provenance is exactly `[{HS400, table_0020}]`; AHB `table_0004` provenance is
  exactly `[{Clock}, {Reset}]` and their statements carry the two sentence-shaped widths; AHB `HCLK`
  and `HRESET` both trace to `table_0033`. `scripts/check_doctrines.sh` green.
- `2026-09-12` — `.0` book routing. `docs/book/src/pipeline/evidenceir.md` gains one section: the
  chapter documented the two rules that *recover* declaration rows (`.2a`, `.2b`) and asserted nothing
  about the rows the reader accepts and should not. The live-surface bookkeeping chain
  (`[[live-surface-edit-bookkeeping-chain]]`) was walked in order: nine `book_quantitative_claims.jsonl`
  regions re-anchored **by content digest**, each matching exactly one window in the edited file; three
  new candidate lines registered (`excluded` / `dated_boundary_evidence`) with
  `expected_candidate_lines` 339 → 342; `shipped_behavior`'s `aggregate_change` rebased to
  17,704 lines / 1,151,850 bytes with the new delta; the three `surface_registry` source pins and the
  `claims.jsonl` durability digests refreshed last.

## Commit Log

- Opened in the commit that deferred `SIGNAL-DECLARATION-ROW-DROP.2c`.
- `.0` — `PROSE-NAME-CELL-DECLARATION.0` (`4bb6c1c8`).
- `.2` — `PROSE-NAME-CELL-DECLARATION.2 / PRODUCTION-GRAPH-CENSUS-PIN.0` (`ba9e9a74`).
- `.1` — `PROSE-NAME-CELL-DECLARATION.1` (`8199be47`).
- `.4` — `PROSE-NAME-CELL-DECLARATION.4` (`c6d61393`).
- `.3` — `PROSE-NAME-CELL-DECLARATION.3` (deferred `2026-09-12`; shipped `2026-09-14`).
- `.5` — `PROSE-NAME-CELL-DECLARATION.5`.

## Changelog

- `2026-09-14` — `.5` closed with **no rule**, and its own premise falsified. The whole-cell score zeroes
  11 name columns corpus-wide, all of them prose, 6 then correctly overridden, and **no real signal is
  lost to it**. TileLink's rotation is not its doing: that column scores 1 under the pre-`.2` rule and the
  `Type` column scores 4 under both, so the rotation predates `.2`. The cause is a split identifier whose
  underscore is present in the PDF text layer and absent from SourceIR; opened as
  `[[TEXT-LAYER-IDENTIFIER-SPLIT]]`. The tree now has no eligible frontier.

- `2026-09-14` — `.3` closed, and its own recorded numbers were corrected by the re-measurement it was
  deferred pending. Current stratum: 195 parametric width cells, **0 prose**, 0 declarations, and all
  27 rebuildable documents reproduce byte for byte. Legacy: 47 prose cells refused, 43 of them one
  cause. The "5-token margin" does not survive the wider population — legitimate expressions reach 8
  tokens and prose starts at 7 — so the rule is a conjunction and each half's cost was measured.
  `.5` opened for the TileLink cause; an unrelated I2C chain-currency drift found by the same sweep was
  proven independent and routed to `CORPUS-CHAIN-CURRENCY.4`.

- `2026-09-11` — tree created from `SIGNAL-DECLARATION-ROW-DROP.2c`'s adjudication, which found 4 of 7
  enumerated-width cells to be a misclassified bus-mode matrix already minting `HS400` as a signal.
- `2026-09-12` — `.3` deferred by its own adjudication. The rule is written, measured (6 prose cells in
  178 admitted, with a 5-vs-7 token margin) and observed RED, but applying it takes AHB's IntentIR
  interface count from 42 to 62 by making the phantom `Manager` declaration well-formed. Blocker opened
  as `[[ACTOR-NOUN-RELATION-DECLARATION]]`; the corpus was rolled back to its pre-measurement bytes.
- `2026-09-12` — `.4` closed. The premise was a legacy fact: the current classifier already refuses the
  bus-mode matrix, and 124 legacy tables minting 598 declarations carry a `table_kind` it would not
  assign, against 0 in the current stratum. `SIGNAL-DECLARATION-ROW-DROP.2c`'s deferral reason is
  dissolved; `.1`'s decision is reinforced (the 3 surviving legacy phrases are all real signals).
- `2026-09-12` — `.1` closed with **no rule**. Measured against the 7 rows it exists to guard, a phrase
  refusal loses Avalon's `byteenable byteenable_n` and still admits eMMC `HS200`; the whitespace-family
  escape hatch selects 3 false positives in 9. `.4` dominates it and is promoted to the frontier.
- `2026-09-12` — `.2` closed. The leaf's own ratio proposal was falsified by census (fixes nothing,
  breaks AHB `table_0034`); shipped `name_cell_is_read_whole` instead. AHB `table_0004` now declares
  `HCLK`/`HRESETn`; current-stratum phrase declarations 2 → 0, IntentIR interface signals 40 → 41.
  `.3`'s live population went to 0 as a side effect and the leaf stays open saying so.
- `2026-09-12` — `.0` closed. The declared phrase population is 2 of 604 current and 15 of 1,251 joined
  legacy, not the 345 candidate rows the tree opened on; the opening approximation does not re-derive
  and over-states by ~19×. Both Open Questions answered by measurement. `.1`-`.4` opened, with `.2`
  (the short-table rotation override) sequenced ahead of the rule because it owns the entire current
  population and the rule would erase its evidence.
