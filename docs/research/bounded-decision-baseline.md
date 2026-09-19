# Bounded-decision baseline — the frozen per-row set, arm A's score, and the pre-registered bar

Owning leaf: `BOUNDED-DECISION-PROVIDER.1` (MEASURE + PRE-REGISTER).
Producer: `python3 scripts/build_bounded_decision_baseline.py`.
Frozen set: `docs/research/bounded-decision-adjudication.jsonl`,
sha256 `d3c5898f657ee6c12c3ef062dbd3d33fe34931a48c08f256ada55a25cf403d05`.

## Why this exists, and why it goes first

`BOUNDED-DECISION-PROVIDER` starts from rejection. A remote decision provider has to beat the best
thing SpecForge can do locally, by a margin fixed **before** any model runs — and today the two
candidate defects are quantified as aggregate percentages (`18.3%` of declaration rows discarded,
`739` captions admitted) rather than as a per-row set a later change can be scored against. Without
that set, "the model improved things" is unfalsifiable and the threshold is whatever the result
turns out to be.

So this record delivers four things, in this order, and the fourth is the one that binds:

1. a frozen, labelled, repository-local adjudication set with per-row gold labels;
2. what the current deterministic rules — **arm A** — score on it;
3. every row they disagree with the gold on, enumerated rather than summarised;
4. the **pre-registered adoption bar**, fixed here, before any model has seen this data.

It also answers the question that can end the tree early: are the disagreements *ambiguous*, or is
the rule simply *wrong*? The answer is in [§6](#6-are-the-disagreements-ambiguity-or-defect), and it
is not the answer that favours a provider.

## 1. The two decisions

Both are the shape a constrained decision model could answer — a yes/no over a candidate the
deterministic pass already located, with no string for the model to author:

| id | question | owning tree | primitive |
| --- | --- | --- | --- |
| `declaration_row` | does this signal-table row declare an interface signal? | `SIGNAL-DECLARATION-ROW-DROP` | Noul |
| `caption_admission` | does this figure/table caption state a normative constraint? | `INVARIANT-SHAPE-ADMISSION` | Noul |

## 2. The population, and why it is these four documents

`extraction_manifest.declaration_row_accounting` is the declaration reader's own denominator, and
only the documents rebuilt since `SIGNAL-DECLARATION-ROW-DROP.1` shipped carry it — **4 of the 27**
proof-carrying chains, which are also the four wire-bearing ones. Both decisions use the same four
documents, so the frozen set is one population under one digest.

| document | `declaration_row` rows | tables | `caption_admission` rows |
| --- | ---: | ---: | ---: |
| AXI `ihi0022_l` | 464 | 76 | 337 |
| APB `ihi0024_e` | 61 | 9 | 36 |
| AHB `ihi0033_c` | 95 | 12 | 83 |
| ADIv6 `ihi0074_a` | 24 | 4 | 157 |
| **total** | **644** | **101** | **613** |

`declaration_row` is every body row the reader was handed. `caption_admission` is every extracted
statement matching the production caption shape `^(Figure|Table)\s+[A-Za-z]?[0-9]` — the population
`is_invariant_like` decides on, before it decides.

The frozen set carries each row's header, caption, and cells **verbatim** (bounded at 96 characters
per cell, and truncation is recorded), so scoring re-derives from the tracked file alone and never
depends on untracked `generated/` state. `--verify-currency` is the separate question of whether
the corpus still produces that population; it reproduces it byte-for-byte today.

## 3. Scoring the product's decision, not the reader's intermediate one

A row counts as declared only when a declaration from it **survives** to
`table_signal_declaration_provenance`. That distinction was measured, not assumed, and it changed
the answer: AXI `table_0011` (*Table A2.3: Credited channel signals*) has
`declarations_emitted == 6` in the accounting and **zero** provenance records, because
`withhold_base_name_template_declarations` (`WIRE-BASED-100.10b`) removes a base-name template
table's declarations once every table producer has run. Scoring the accounting alone would have
published six false positives the product does not make, and would have reported arm A's precision
as `0.9895` instead of `1.0000`.

The frozen set therefore records three reader states — `emitted`, `withheld`, `dropped` — and only
`emitted` scores positive. `--self-test` case `withheld-is-not-emitted` is the control that keeps
it that way.

## 4. The gold, and the rubric that produced it

### 4.1 `declaration_row`

> **`signal`** iff the row states the existence of at least one named interface signal of the
> interface this document specifies. Otherwise `not_signal`.

The label is about the **row**, not about the token the reader happened to lift out of it. That is
deliberate: a row that declares a real wire whose name the reader could not find is a miss, and
hiding it inside the gold would hide the defect this set exists to measure. Where the name is
unfindable from the reader's chosen column, [§6](#6-are-the-disagreements-ambiguity-or-defect)
classifies it as such rather than relabelling it.

**Every one of the 101 tables is uniformly `signal` or uniformly `not_signal`. Not one is mixed.**
That reproduces `[[a-dropped-declaration-row-is-usually-not-a-signal]]`'s ten-table finding on a
population ten times larger, and it is the most consequential fact in this record — see
[§7](#7-what-this-means-for-arm-c-before-arm-c-runs).

The eight `not_signal` tables, each refused on a property of its own shape or on what the document
states about it, never on which protocol it is:

| table | what it is | rows |
| --- | --- | ---: |
| AXI `table_0011` | base-name template; the document's own introducing sentence reads *"Signal names are the base name, when instantiated each includes a prefix to indicate which channel"* | 6 |
| AXI `table_0092` | metavariable grid — `AxLEN`, `AxSIZE`; `AXLEN` is undeclared while `AWLEN`/`ARLEN` are | 4 |
| AXI `table_0183` | width parameters — header `Name \| Values \| Default`, values are ranges (`0..8`) | 2 |
| AXI `table_0184` | width parameters, same shape | 3 |
| AXI `table_0199` | encoding matrix; the name column holds address-space values, the signals are in the header | 18 |
| AXI `table_0265` | a legend — `Y \| Mandatory \| Mandatory` | 6 |
| AHB `table_0014` | encoding rows for the signal named in the header (`HBURST[2:0]`) | 8 |
| AHB `table_0019` | encoding rows for the signal named in the header (`HPROT[..]`) | 8 |

Six of the eight were already adjudicated in `[[a-dropped-declaration-row-is-usually-not-a-signal]]`
and are carried over. `table_0011` and the two `HPROT`/`HBURST` encodings are adjudicated here.

### 4.2 `caption_admission`

> **`constraint`** iff the text contains a **finite main clause** stating an obligation,
> prohibition, or permission restriction about the specified system. A title — a label plus a noun
> phrase, with no finite main clause — and a sentence whose main verb only reports what a figure or
> table shows are both `not_constraint`, however many deontic words they carry.

The two halves of that rubric are what separate `Table A8.2: Opcodes which must be cache line sized
and Regular` (a title — refused) from `Other combinations are not permitted.` (a prohibition —
admitted), and neither turns on a vocabulary list. **6 of 613** captions are `constraint`:

| document | statement | the clause |
| --- | --- | --- |
| AXI | `statement_1426` | *An interface can include AxPAS or AxPROT/AxNSE signals, not both.* |
| AXI | `statement_3781` | *Other combinations are not permitted.* |
| AHB | `statement_0570` | *The bit combinations that Table 3-7 does not show, are not permitted.* |
| ADIv6 | `statement_2092` | *No additional SWDIOTMS LOW cycles are allowed.* |
| ADIv6 | `statement_2139` | *No additional SWDIOTMS LOW cycles are allowed.* |
| ADIv6 | `statement_4660` | *All listed registers are required in every debug component implementation.* |

**How the other 607 were screened, so "none of them is a requirement" is a measurement and not an
impression.** The producer passes every caption through a deontic net deliberately **wider** than
production's route `r1` — adding `should`, `recommended`, `mandatory`, `not permitted`,
`prohibited`, `forbidden`, `allowed`, `only`, `ensure`, `guarantee`, `is not`, `are not` and more —
and reports the decomposition rather than asserting it:

| screen | captions |
| --- | ---: |
| carries deontic vocabulary — read one at a time | 27 |
| pure title, no main clause at all | 478 |
| sentence whose main verb is a reporting verb (`shows`, `lists`, `summarizes`, …) | 105 |
| other sentence — printed in full by the producer; all three are title-like lines | 3 |
| **total** | **613** |

The 27 are where the six `constraint` labels come from. `--self-test` case
`caption-screen-is-exhaustive` is what keeps the screen from quietly accounting for fewer captions
than the set contains.

**Eight rows carry a recorded `note` naming where a second annotator would most plausibly disagree**
— every one of them a deontic word sitting inside a noun phrase or a subordinate clause, which this
rubric refuses and a looser reading would admit. `statement_2170` (*"shows … the sequence that a
JTAG device must recognize"*) is the closest call in the set. They are recorded rather than hidden
because a single-annotator gold is a known reliability hazard here
(`[[eval-gold-interannotator-kappa]]`), and because arm B and arm C will be scored against exactly
these calls.

## 5. Arm A — what the current rules score

```bash
python3 scripts/build_bounded_decision_baseline.py
```

| decision | rows | gold + | tp | fp | fn | tn | precision | recall | macro-F1 | errors |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `declaration_row` | 644 | 589 | 567 | 0 | 22 | 55 | `1.0000` | `0.9627` | `0.90715` | **22** |
| `caption_admission` | 613 | 6 | 2 | 5 | 4 | 602 | `0.2857` | `0.3333` | `0.65014` | **9** |

Macro-F1 averages the positive and negative classes. It is the headline rather than positive-class
F1 because the two sets are imbalanced in **opposite** directions — 589 of 644 against 6 of 613 —
and positive-class F1 alone would read `0.98` for a rule that loses 22 real wires and `0.31` for one
that gets 602 of 607 refusals right.

### 5.1 `declaration_row` — all 22 disagreements

Arm A publishes **no false positive** on this set; every error is a lost row.

| document | table | rows | the reader's candidate name | why it dropped |
| --- | --- | ---: | --- | --- |
| APB | `table_0018` | 1 | *(empty)* | the name is in the last column of a **one-row** table; the content-based name-column override needs `best_distinct >= 2` and one row cannot reach it. The real name is `PWAKEUPCHK`. |
| ADIv6 | `table_0039` | 3 | `CDBGPWRUPREQ`, `CDBGPWRUPACK`, `CSYSPWRUPREQ` | a two-column `Signal \| Programmers' model` table states no direction and no width anywhere |
| ADIv6 | `table_0039` | 1 | `Bit[31` | the fourth row is rotated the **other way** inside the same table; the real name `CSYSPWRUPACK` is in column 0 |
| ADIv6 | `table_0041` | 2 | `DBGTDO`, `DBGTRSTn` | the direction is real but merged into the neighbouring cell by the text layer (`TDO Output`, `TRST Input`) |
| ADIv6 | `table_0058` | 5 | `SWDIOTMS`, `SWDCLKTCK`, `TDO`, `TDI`, `TRSTn` | a pin-routing table: three name columns, no direction and no width stated anywhere |
| ADIv6 | `table_0108` | 4 | `Out`, `Out`, `Out`, `In` | column garble; the real names (`TCK`, `TMS`, `TRST`, `SRSTCONNECTED`) are in the **Notes** column |
| ADIv6 | `table_0108` | 5 | `TDI`, `TDO`, `RTCK`, `PORTCONNECTED`, `PORTENABLED` | name found; the same garble leaves no readable direction or width in the row |
| ADIv6 | `table_0108` | 1 | `nSRSTOUT` | the direction **is** present and unambiguous — the cell reads `Out` in a column the header names `Direction` — and the reader refuses the abbreviation |

**The loss is not diffuse.** AXI and AHB score perfectly — 0 errors across 559 rows. APB loses one.
**ADIv6 loses 21 of its 24 rows, and all 24 are real signals**: an 87.5% loss in a single
wire-bearing document, which no gold score and no currency gate can see.

### 5.2 `caption_admission` — all 9 disagreements

| kind | statement | text (elided) | why |
| --- | --- | --- | --- |
| FP | AXI `statement_2299` | *Table A8.2: Opcodes which **must** be cache line sized and Regular* | a modal inside a title |
| FP | AXI `statement_2976` | *Figure A9.3: **Required** sequence of communication …* | a deontic adjective inside a title |
| FP | AHB `statement_0670` | *Figure 4-2 shows the … structure **required** to implement …* | a deontic adjective under a reporting verb |
| FP | ADIv6 `statement_2170` | *Figure B5-11 … shows … the sequence that a JTAG device **must** recognize.* | a modal in a relative clause under `shows` |
| FP | ADIv6 `statement_6129` | *Table E2-1 summarizes the **required** and recommended components …* | deontic adjectives under a reporting verb |
| FN | AXI `statement_3781` | *… Other combinations are **not permitted**.* | the form is outside route `r1`'s phrase list |
| FN | AHB `statement_0570` | *… The bit combinations that Table 3-7 does not show, are **not permitted**.* | same |
| FN | ADIv6 `statement_2092` | *… No additional SWDIOTMS LOW cycles **are allowed**.* | same |
| FN | ADIv6 `statement_2139` | *… No additional SWDIOTMS LOW cycles **are allowed**.* | same |

The two failure modes are exact mirrors: route `r1` fires on a deontic **word** wherever it sits,
and misses a deontic **clause** whose wording it does not list.

## 6. Are the disagreements ambiguity, or defect?

This is the question `.1` exists to force, because if the rows are repairable the tree closes with a
better extractor and no vendor. Every one of the 31 errors is classified:

| decision | cause | rows | is it a decision a ranking model could make? |
| --- | --- | ---: | --- |
| `declaration_row` | **name-cell selection** — the name is not in the column the reader chose | 6 | no: this is a *different* decision (`PROSE-NAME-CELL-DECLARATION`'s "which cell is the name"), and a Noul over the row cannot answer it |
| `declaration_row` | **no attribute stated anywhere** — the row offers identity and nothing else | 8 | no: the same question was put three times and answered NO three times (`[[a-dropped-declaration-row-is-usually-not-a-signal]]`); admitting identity alone is 24% precise |
| `declaration_row` | **attribute lost by ingest** — the text layer merged or scattered the direction | 7 | no: the evidence is gone before any decision is reached; a model ranking these cells ranks garble |
| `declaration_row` | **abbreviated direction refused** — `Out` as a whole cell under a `Direction` header | 1 | no: a deterministic notation the reader could read |
| `caption_admission` | **deontic word, not a deontic clause** (the 5 FP) | 5 | no: a deterministic fix exists — require the deontic to head a finite main clause, and refuse a title |
| `caption_admission` | **deontic clause outside route `r1`'s list** (the 4 FN) | 4 | no: a deterministic fix exists, with an over-firing risk to adjudicate |
| — | **genuine ambiguity** | **0** | — |

**Zero of the 31 errors is genuine ambiguity.** Every one is a rule defect with a deterministic
repair, a different decision, an upstream ingest defect, or a standing contract limit this
repository has already adjudicated and refused. That is the classification `.1` was built to
surface, and it is evidence against arm C **before arm C has run**.

Stated against its own weakness: this is a 31-error sample in four documents, and the classification
is the same annotator's. It bounds what arm C could win *here*; it does not prove no corpus row is
ambiguous.

## 7. What this means for arm C, before arm C runs

Three consequences follow from the set itself and are recorded now so they cannot be renegotiated
once results exist:

1. **The `declaration_row` decision is a table property, in 101 of 101 tables.** A per-row ranking
   model is being asked a question whose answer is settled one level up. Its per-row votes can
   therefore only add noise within a table, and its whole achievable win is re-deciding eight
   tables. The three table-level discriminators already tried and refuted
   (`[[a-dropped-declaration-row-is-usually-not-a-signal]]`) remain the honest frontier.
2. **Arm A's precision on `declaration_row` is `1.0000`.** There is no precision to win, only
   recall — and 21 of the 22 recoverable rows need a *different* cell, a *different* decision, or an
   un-garbled text layer. A provider that cannot read the source better than the ingest did cannot
   recover them.
3. **`caption_admission` is where the headroom is** — macro-F1 `0.65014`, and both failure modes are
   ordinary grammar. That is also precisely where a deterministic fix is cheapest, which is arm B's
   case, not arm C's.

## 8. The pre-registered adoption bar

**Fixed `2026-09-19`, before any model — local or remote — has been run against this set. A
threshold chosen after seeing a result is not a threshold. `BOUNDED-DECISION-PROVIDER.6` is held to
these numbers and no others.**

For a decision to be adjudicated by a remote provider, **arm C must satisfy all four of the
following on that decision's frozen set, simultaneously, in one run at the pinned version
`jev-1.13.0`, with the result re-derivable offline from its cached record**:

- **C1 — macro-F1 margin.** Arm C beats **arm A and arm B each** by **≥ 0.05 absolute macro-F1**.
  Not the better of the two: each of them.
- **C2 — error reduction.** Arm C's total errors (`fp + fn`) are at most **60%** of the better of
  arm A's and arm B's, and the absolute improvement is at least `ceil(0.4 × arm A errors)` rows,
  with a floor of 4. On today's numbers: **`declaration_row` ≤ 13 errors and ≥ 12 rows corrected**;
  **`caption_admission` ≤ 5 errors and ≥ 4 rows corrected**.
- **C3 — no precision regression.** Arm C's false positives do not exceed the better of arm A's and
  arm B's. A phantom declaration is worse than a missed one in this product: admitting identity
  without an attribute measured **24% precise**, and a phantom propagates silently into the KG while
  a miss stays countable.
- **C4 — contested rows excluded.** The margin must hold with the 8 `note`-carrying contested rows
  **removed** from the set as well as with them included. A win that exists only on the rows this
  record already flagged as arguable is not a win.

**A marginal or ambiguous result is a rejection**, per this tree's standing rule: the cost side is a
`ROADMAP.md:37` amendment, corpus spans leaving the volume, a permanent offline-cache obligation,
and a vendor with no stated version-availability policy.

**And the bar above is necessary, not sufficient.** Disqualifiers D1–D4 (identity dependence,
numeric reading, no offline re-derivation, egress refused) remain fatal independently, and a win
over arm A alone means shipping arm B instead.

## 9. What this baseline does not establish

- It is **4 of 27** proof-carrying documents, and the four are wire-bearing AMBA/ADI specifications.
  A document class outside that stratum may disagree about everything here.
- The gold is **one annotator's**, screened by a wider net than production's and recorded with its
  contested rows, but not independently re-annotated. `[[eval-gold-interannotator-kappa]]` is the
  standing reason that matters.
- `declaration_row` is dominated by 567 easy positives. The informative view is the **77-row
  contested stratum** — the 71 rows the reader dropped plus the 6 it withheld — on which arm A is
  right 55 times and wrong 22.
- It measures **admission**, not correctness of what is admitted. A row admitted with the wrong
  width is a true positive here and a defect elsewhere.

## 10. Findings this record opens

- **`SIGNAL-DECLARATION-ROW-DROP.2j`** (opened by this leaf): `Out`/`In` as a whole cell in a column
  the header **names** as direction is unambiguous, and the reader refuses it. `.2h.0`'s measured
  refusal of the abbreviations (`0` true positives against `18` false) was a census of the
  *fallback* population — columns whose header carries **no** direction keyword — where a presence
  matrix writing `O` for *Optional* is the hazard. That hazard cannot arise under a `Direction`
  header. One row of this frozen set (ADIv6 `table_0108`, `nSRSTOUT`) is lost to it; the corpus-wide
  population is unmeasured.
- **ADIv6 `table_0108` is column-garbled in the text layer**, which costs 9 of the 22 lost rows on
  its own. That is an ingest defect surfacing as an extraction miss, and no decision layer can
  repair it. Tracked in the same leaf as the population to hand upstream.

## 11. Re-derivation

```bash
python3 scripts/build_bounded_decision_baseline.py                  # arm A, with every disagreement
python3 scripts/build_bounded_decision_baseline.py --check          # digest + pinned result agree
python3 scripts/build_bounded_decision_baseline.py --self-test      # 10/10 RED cases
python3 scripts/build_bounded_decision_baseline.py --verify-currency # corpus still renders the set
```

`--check` is what makes the numbers above un-editable: it refuses any drift in the frozen set's
digest, in either population size, or in either confusion matrix. Production moving is a real event
and has to be re-derived here, not absorbed.
