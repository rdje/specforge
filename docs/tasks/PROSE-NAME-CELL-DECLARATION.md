# PROSE-NAME-CELL-DECLARATION: a row whose name cell is a phrase declares its first word as a signal

## Metadata

- Tree ID: `PROSE-NAME-CELL-DECLARATION`
- Status: `active` (`2026-09-12`; `.0`, `.1`, `.2`, `.4` done; `.3` deferred on a measured blocker)
- Roadmap lane: `R2` (extraction correctness / false-positive control)
- Created: `2026-09-11`
- Last updated: `2026-09-12`
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

- ID: `PROSE-NAME-CELL-DECLARATION` · Status: `active` (`2026-09-12`) · Children: `.0`-`.4`

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

- ID: `PROSE-NAME-CELL-DECLARATION.3` · Status: `deferred` (`2026-09-12`, by its own adjudication;
  blocked on `[[ACTOR-NOUN-RELATION-DECLARATION]]`) · Goal: **a width cell that is a sentence is
  not a parametric width.** `infer_signal_table_row_width_hint` accepted
  `The bus clock times all bus transfers. All signal timings are related to the rising edge of HCLK .
  See Clock on page 7-72.` as `WidthHint::Parametric`, producing
  `Signal Clock is width The bus clock times all bus transfers. …`. Measured: **2 of 601** current
  width-bearing declarations carry a sentence-shaped width, and both are AHB `table_0004`.
  The mechanism is independent of `.2` even though today's population was not: a parametric width is an
  integrator-set expression (`ceil(DATA_WIDTH/8)`, `clog2(Num_RP_AR)` — 62 legitimate instances in the
  current stratum), and nothing distinguishes it from prose today.
  **`.2` took this leaf's current-stratum population to 0**, because both instances were AHB
  `table_0004` and that table now reads its real name column. The leaf stays open and the honesty about
  it is the point: a defect with no live instance is still a defect, and the next rotated table the
  column score cannot reach will produce one again. Re-measure before designing, and be willing to
  close this as *accepted, unexercised* rather than invent a population for it.
  Prerequisite: none. Verification: all 62 legitimate parametric forms survive; corpus-wide count of
  what is newly refused, with the sample adjudicated.

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

## `.3` — the rule is right, its only live effect is wrong (`2026-09-12`)

`.3` was expected to close as *accepted, unexercised*: `.2` had taken its declared population to
**0 of 601** current width-bearing declarations. It did not close that way, because measuring the
mechanism rather than the declarations found a live population and then a blocker.

### The population, at two levels

| level | current stratum | legacy |
| --- | ---: | ---: |
| declarations carrying a sentence-shaped width | **0** of 601 | 108 of 1,085 |
| width **cells** the parser admits as `Parametric` | 178 | 294 |
| of those, cells that read as prose | **6** | ~9 |

The declared population is zero only because other gates happen to stop those six rows today. That is
not stability: two of the six *were* declarations yesterday, and what removed them was `.2`, an
unrelated change to which column a table's names are in.

### The discriminator, and its measured margin

Two conditions, both required, following the `WIRE-BASED-100.10e` precedent:

1. more than six expression tokens;
2. a sentence terminator followed by whitespace.

On the current stratum **the two select exactly the same 6 cells**, and all 6 are description sentences
from rotated AHB tables. The cost side is a margin rather than a tuned threshold: the widest legitimate
expression is **5** tokens (`ceil((ID_W_WIDTH + int(Unique_ID_Support))/8)`) and the narrowest prose is
**7**; no legitimate form carries a terminator followed by a space, because a footnote marker rides the
expression directly (`ceil(ADDR_WIDTH/8) a`).

Written, and **observed RED**: without the guard the control declares
`Signal ZETA_ALPHA is width The zeta clock times all zeta transfers. …`.

### Why it is not shipped

The rule was applied and AHB's chain rebuilt to measure the blast radius rather than assume it.
Declarations: 79 → 79, identical set. But **AHB's IntentIR interface count went 42 → 62**, and all 24
additions are `…_manager` groupings.

The cause is not the rule. AHB already declares `Manager` — an actor role, not a wire — through the
relation→declaration path. Its statement read
`Signal Manager is output width Exclusive okay, selected by the decoder. a.` and became
`Signal Manager is output.` **Removing the garbage width made the phantom well-formed, and a
well-formed phantom propagates.**

So the honest result is a deferral with the rule in hand, not a rule shipped. Publishing a 48% increase
in one document's interface count, all phantom, to fix a width that no declaration currently carries is
the wrong trade in both directions. The blocker is tracked as
`[[ACTOR-NOUN-RELATION-DECLARATION]]`; when it closes, `.3` re-measures and ships.

This is `.2`-before-`.1` again, and it is becoming the tree's characteristic finding: **when two
changes touch the same rows, the one that repairs the cause goes first, because the other one only
changes how the defect looks.**

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

## Current Frontier

No eligible leaf. `.3` is the only one left and it is **deferred on a measured blocker**: its rule is
written, measured and observed RED, and its only live effect in the corpus is to make an existing
phantom declaration well-formed, which takes AHB's IntentIR interface count from 42 to 62. The blocker
is `[[ACTOR-NOUN-RELATION-DECLARATION]]`; when that closes, re-measure and ship `.3`.

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
- `.3` — `PROSE-NAME-CELL-DECLARATION.3` (deferred).

## Changelog

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
