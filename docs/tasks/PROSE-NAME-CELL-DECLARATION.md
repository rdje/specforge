# PROSE-NAME-CELL-DECLARATION: a row whose name cell is a phrase declares its first word as a signal

## Metadata

- Tree ID: `PROSE-NAME-CELL-DECLARATION`
- Status: `active` (`2026-09-12`; `.0` closed by census, `.1`-`.4` opened from its adjudication)
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

- ID: `PROSE-NAME-CELL-DECLARATION.1` · Status: `pending` · Goal: **the phrase refusal rule.** Refuse a
  name cell classified `phrase`, ordered strictly after the four legitimate shapes so each survives by
  construction: `comma-family` (the reader's own `signal_names_in_name_cell` admission),
  `footnote-marked`, `bracket-suffixed`, `repeated-token`. Population it newly refuses, from `.0`:
  **2 of 604 current, 15 of 1,251 joined legacy**. Its value is not those 2 — it is that
  `SIGNAL-DECLARATION-ROW-DROP.2c` stays deferred until a guard exists, and `.0` measured that the
  guard is worth 3 of the 4 rows `.2c` would newly mint in eMMC `table_0020`.
  Prerequisite: `.2` (see its decision — shipping `.1` first would erase `.2`'s only current evidence).
  Verification: corpus-wide refusal count + adjudicated sample; each of the four legitimate shapes shown
  to survive; wire golds re-scored, not assumed.

- ID: `PROSE-NAME-CELL-DECLARATION.2` · Status: `pending` · Goal: **the content-based name-column
  override cannot fire on a short table, and that is the whole current-stratum phrase population.**
  AHB `table_0004` is rotated exactly like `table_0033` (`Name | Source | Width | Description` header
  over a body holding `Clock source | 1 | <description> | HCLK`), but it has **two body rows**, so the
  real name column scores 2 distinct tokens against the header column's 2 and
  `NAME_COLUMN_OVERRIDE_MARGIN = 2` blocks the override. Both current-stratum phrase declarations, and
  both current-stratum prose widths (`.3`), come from this one table.
  Census first, then a rule: the margin exists because a one-token lead is noise, and a short table is
  where a margin expressed in absolute tokens is weakest. Measure how many persisted
  `signal_description` tables have a column that out-scores the header column by a *ratio* rather than
  a count, and adjudicate the selection before proposing anything.
  Prerequisite: none. Verification: every rotation the margin was added for (APB `table_0016` 18 vs 5,
  AHB `table_0033` 19 vs 4) still overrides; no table currently reading its header column starts
  overriding without adjudication.

- ID: `PROSE-NAME-CELL-DECLARATION.3` · Status: `pending` · Goal: **a width cell that is a sentence is
  not a parametric width.** `infer_signal_table_row_width_hint` accepted
  `The bus clock times all bus transfers. All signal timings are related to the rising edge of HCLK .
  See Clock on page 7-72.` as `WidthHint::Parametric`, producing
  `Signal Clock is width The bus clock times all bus transfers. …`. Measured: **2 of 601** current
  width-bearing declarations carry a sentence-shaped width, and both are AHB `table_0004`.
  The mechanism is independent of `.2` even though today's population is not: a parametric width is an
  integrator-set expression (`ceil(DATA_WIDTH/8)`, `clog2(Num_RP_AR)` — 62 legitimate instances in the
  current stratum), and nothing distinguishes it from prose today.
  Prerequisite: none. Verification: all 62 legitimate parametric forms survive; corpus-wide count of
  what is newly refused, with the sample adjudicated.

- ID: `PROSE-NAME-CELL-DECLARATION.4` · Status: `pending` · Goal: **decide the level for a mode matrix,
  because `.0` proved the row is not it.** eMMC `table_0020`'s five rows are
  `Backwards Compatibility with legacy MMCcard`, `High Speed SDR`, `High Speed DDR`, `HS200`, `HS400`.
  A phrase rule refuses the first three. `HS200` and `HS400` are **single-token** cells indistinguishable
  in shape from a wire, so the table mints phantoms under any row-level rule — one today, two if
  `SIGNAL-DECLARATION-ROW-DROP.2c` ships with `.1` in place.
  Candidates: the SourceIR classifier that typed a bus-mode matrix `signal_description`; a
  header-shape refusal at the table level (`WIRE-BASED-100.10e`'s mechanism sees a caption, and this
  table has none — its evidence is its header row); or an accepted residual.
  Prerequisite: `.1`. Verification: whichever level is chosen, corpus-wide selection adjudicated.

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

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `PROSE-NAME-CELL-DECLARATION.2` — the short-table rotation census. It owns the entire current-stratum
   phrase population and must be measured before `.1` refuses the evidence.
2. `PROSE-NAME-CELL-DECLARATION.3` — a sentence is not a parametric width. Independent of `.2`.
3. `PROSE-NAME-CELL-DECLARATION.1` — the phrase refusal rule, re-measured after `.2`.
4. `PROSE-NAME-CELL-DECLARATION.4` — the level question for a mode matrix.

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
- `2026-09-12` — **`.1` is kept despite a current-stratum population of 2.** It is not justified by
  those two. It is the guard `SIGNAL-DECLARATION-ROW-DROP.2c` was deferred waiting for, and `.0`
  measured its worth there: 3 of the 4 rows `.2c` would newly mint in eMMC `table_0020`.

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
- `.0` — `PROSE-NAME-CELL-DECLARATION.0`.

## Changelog

- `2026-09-11` — tree created from `SIGNAL-DECLARATION-ROW-DROP.2c`'s adjudication, which found 4 of 7
  enumerated-width cells to be a misclassified bus-mode matrix already minting `HS400` as a signal.
- `2026-09-12` — `.0` closed. The declared phrase population is 2 of 604 current and 15 of 1,251 joined
  legacy, not the 345 candidate rows the tree opened on; the opening approximation does not re-derive
  and over-states by ~19×. Both Open Questions answered by measurement. `.1`-`.4` opened, with `.2`
  (the short-table rotation override) sequenced ahead of the rule because it owns the entire current
  population and the rule would erase its evidence.
