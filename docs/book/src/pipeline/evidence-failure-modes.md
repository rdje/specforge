# Evidence-Level Failure Modes

Extraction fails in a small number of recognisable ways, and naming them is what makes a bad
artifact diagnosable instead of merely disappointing. This chapter is the catalogue: what each
failure looks like in the output, what causes it, and what SpecForge does about it.

It is the companion to [Typical evidence-level wins](evidenceir.md#typical-evidence-level-wins) in
the [EvidenceIR](evidenceir.md) chapter, and to the rules in
[Reading a normative obligation](obligation-reading.md).

## Typical evidence-level failure modes

- field tables leaking fake signals
- abstract example tables pretending to be real interfaces
- payload nouns being promoted to actors
- descriptive relative-clause phrases such as `mixture of` being promoted to actors
- signal names creating semantic meaning by spelling alone

Many of the project’s recent truthfulness slices have been about tightening exactly those boundaries.

### The failure mode that list was missing: rows that are simply dropped

Every entry above is something *fake getting in*. Measured on `2026-09-11`, the larger loss runs the
other way — real rows *silently not getting in*.

Signal declarations built from tables are the authoritative ones: a table declaration wins over
anything derived downstream. The reader builds each declaration from a `(direction, width)` pair, and
a row that yields neither is skipped — no declaration, no residual, no counter, no entry in the
validation report. Over every persisted artifact, **482 of 2,637 rows (18.3 %)** are discarded that
way, counted only over tables SpecForge itself typed as signal descriptions whose name cell is a
single clean identifier. Four specifications lose **every** such row; the largest single losses are a
processor TRM at 107 rows and one bus specification at 103 across its two editions.

The Avalon case shows how ordinary the cause is. Its signal table plainly lists `readdata` and
`writedata`, but that document writes direction as an arrow (`Slave → Master`) and writes one width as
the set of legal widths (`8, 16, 32, 64, 128, 256, 512, 1024`). SpecForge reads neither notation, so
both rows disappear — and 15 of that document's 26 surviving declarations carry a width but no
direction, which is the same gap showing up from the other side.

Three things worth drawing out, because they are why this went unnoticed for so long:

- **A perfect score is compatible with it.** The bus specification that loses 103 rows scores `1.000`
  on every aspect of its gold, because a gold is evidence only about the facts it happens to name.
- **The currency check is *supposed* to stay green.** A dropped row is not drift. The stored artifact
  really is exactly what today's binary produces; the loss being perfectly repeatable is precisely why
  replay cannot flag it.
- **No ratio exists to bound.** The reader never reports rows-considered against declarations-emitted,
  so there was no number for any check to watch.

This also settled an open design question rather than answering it. One path in the pipeline turns a
*relation* ("the slave drives readdata") into a *declaration*, which contradicts the rule that the
next stage down owns that authority; it had been kept because deleting it appeared to cost those eight
genuine Avalon signals. It does not: they are carried by that path only because the authoritative path
drops their rows. Repairing the reader removes the reason the exception existed — no heuristic and no
model tier is needed to tell the junk names it also mints from the real ones. Tracked as
`SIGNAL-DECLARATION-ROW-DROP`, which makes the drop visible before making it smaller.

### A template row is not a declaration

Repairing that drop started somewhere unexpected. Before teaching the reader a notation it cannot
read, the rows it *already* reads had to be worth reading — and one class of them is not.

Specifications routinely describe a family of signals by writing one row about a stand-in name. A
tristate conduit is documented as three rows — `<name> _in`, `<name> _out`, `<name> _outen`, with
descriptions like *"the input signal of a logical tristate signal"* — where `<name>` is whatever the
integrator calls the signal. That is a template the reader of the document expands, not a wire the
document declares.

SpecForge declared it anyway. The name cell's leading token is trimmed of its non-identifier
characters before being judged, and that trim is exactly what turns `<name>` into the perfectly
ordinary identifier `name`. The identifier test then sees nothing unusual and the row becomes a
signal called `name`. With a direction also readable — which is what the arrow notation above would
have supplied — the same table would have declared that phantom **twice, with opposite senses**,
because a tristate template states one row per sense.

The rule is to judge the leading token *as written*: a token wrapped in a matched bracket pair
(`<…>`, `(…)`, `[…]`, `{…}`) is a metavariable, and the row declares nothing. This is shape only —
the delimiters decide, never the word inside them, so it carries no vendor or protocol vocabulary and
no list of placeholder spellings. The row is recorded rather than discarded, with its own reason
(`name_is_a_placeholder`) and its cell text kept verbatim, so a reader of the artifact can see the
template that was refused.

The test runs *after* the identifier test, deliberately. A bracketed token that was never an
identifier in the first place — a bit range such as `[15:8]` under a `Bits` header — keeps the reason
it already had, so the new rule changes the fate of a row rather than the label on a row whose fate
was already settled.

Measured on `2026-09-11` across 78 artifacts, exactly **five** rows in two documents reach the new
rule, and **two phantom declarations** disappear: `name` and `any`, both from one interface
specification's template tables. Nothing else moves: each of 24 documents that can be rebuilt
produces a byte-identical `EvidenceIR` before and after, so no score and no stored chain is touched.
Tracked as `SIGNAL-DECLARATION-ROW-DROP.2a`; the population is re-derivable with
`python3 scripts/measure_declaration_row_notations.py`.

### Direction written as a flow, not as a port sense

With template rows refused, the notation that started this can be read. A signal table's direction
column does not always say `input` or `output`. Some documents state the **flow** instead:

```text
Signal Role   Width                              Direction        Description
readdata      8, 16, 32, 64, 128, 256, 512, 1024 Slave → Master   The readdata driven from the slave …
writedata     8, 16, 32, 64, 128, 256, 512, 1024 Master → Slave   Data for write transfers.
```

Both rows produced nothing at all before: no direction was readable, the width was a list of legal
widths rather than one width, and a row with neither is the drop the section above measures.

The arrow is the grammar. The two actors either side of it are read with the same role taxonomy every
other direction path already uses, so this adds a notation rather than a vocabulary. What makes it
safe is that **both sides must resolve, and they must agree**: reading the left side as a source and
the right side as a destination has to yield the same port sense. A cell naming one recognised role
beside one unrecognised name states a flow relative to an actor whose role is unknown; a cell whose
two sides disagree contradicts itself; a cell containing two flows describes a bidirectional group,
which has no single answer. All three fail closed, as does any reverse or bidirectional marker.

That mirror is not caution for its own sake — it is this reader's recurring lesson, that a cheap
structural rule over-fires until a second condition is added. The numbers say how much it costs and
what it buys. Across all 78 stored artifacts there are 83 arrow-bearing direction cells, in just two
documents and 13 distinct wordings — few enough that the sample *is* the population. 18 are admitted,
and every one of them is a genuine direction statement. The other 65 fail closed: 16 state two
opposite flows in one cell, and 49 name actors the built-in role taxonomy does not know
(`Distributor`, `ITS`, `Source → Sink`). Those 49 are a taxonomy question, deliberately left open
rather than answered by widening a list that also decides how headings and prose are read.

Of the 18, seven are rows that produced nothing before — five real wires recovered, and two that the
template rule above now refuses. The rest already declared a width and simply gain their direction.

Two honest limits. First, this recovers about 3.7 % of the measured row loss, not a majority: the
four specifications that lose *every* row contain no arrow cell at all, so their loss has another
cause that is still unidentified. Second, `readdata` and `writedata` come back as
`Signal readdata is input.` — with a direction but no width, because the enumerated width set is a
separate notation and a separate decision about how to represent a set of legal widths without
inventing one of them. Tracked as `SIGNAL-DECLARATION-ROW-DROP.2b`; the population is re-derivable
with `python3 scripts/measure_declaration_row_notations.py`.


### A name cell that is a phrase — and how large that population really is

Both rules above *recover* rows. The opposite question — which rows this reader accepts that it
should not — opened as its own tree, and its first leaf was a census rather than a rule, because the
number the tree opened on turned out to be measuring something else.

A row's name is its name cell's first whitespace token, trimmed and tested as an identifier. When the
cell is a phrase, that mints its first word: `Backwards Compatibility with legacy MMCcard` would
declare a signal called `Backwards`. Counting the rows *shaped* like that — a signal-description row
whose name cell holds two or more tokens starting with an identifier, excluding the comma families —
finds 318 rows across 27 documents. Counting the declarations the reader actually produced from such
a cell finds seventeen.

The gap is the `(direction, width)` drop measured above: a candidate row that offers neither is
discarded long before the shape of its name matters. The population therefore has to be read back
from what the reader emitted, never from what the source offers it. Joining every entry of the
persisted `table_signal_declaration_provenance` to the source row carrying its name, and classifying
that cell by shape, measured on `2026-09-12`:

| stratum | declarations | joined to a name cell | minted from a phrase |
| --- | ---: | ---: | ---: |
| current (proof-carrying) | 604 | 604 (100.0%) | 2, now 0 |
| legacy (inspection-only) | 2,085 | 1,251 (60.0%) | 15, now 11 |

The two strata are never added, because a persisted artifact is evidence about the producer that
wrote it. The legacy join rate is itself that principle showing through: an older emitter folded
names to upper case, and a folded name is no longer the cell's own spelling, so it cannot be matched
back to the row it came from.

The same caution applies to the legacy count itself, and more strongly than it first appears. A
persisted table also records the *kind* an older classifier gave it, and 124 legacy tables carry
`signal_description` where the current classifier assigns nothing of the sort — register summaries,
Extended-CSD field tables, a bus-speed-mode matrix. Those tables mint 598 of the legacy stratum's
2,085 declarations. In the proof-carrying stratum the disagreement is zero. A legacy number is a fact
about files.

Five shapes separate the population with no vocabulary at all — a comma-separated family
(`AWSIZE, ARSIZE`), a footnote marker (`HSELx a`), a bit-range suffix (`ARMPAM [10:0]`), a text-layer
split of one identifier (`waitrequest waitrequest _ n`), and a phrase. Every legitimate form lands in
its own class, which is why the corpus's largest wire specification contributes nothing to the
phrase count: all 72 of its multi-word name cells are comma families this reader already reads
correctly.

### The two phantoms were a column defect, and refusing them would have hidden it

Both current phantoms came from one AHB table, and the name reader was not their cause. That table is
rotated — its header reads `Name | Source | Width | Description` while its body carries the signal in
the last column — and the content-based correction that fixes the other rotated tables declined to
fire, because the table has two body rows: too few to clear the margin that protects an explicit
`Name` header from being overruled by noise. `Clock source` and `HCLK` each offered one leading
identifier, so nothing could tell a prose column from a name column.

Refusing the phrase would have silenced those rows. It would also have deleted the only evidence that
the column was wrong — so the reader was changed where the mistake was. **A cell scores for its column
only when the reader consumes it whole.** The four multi-token forms above leave nothing over and
still score; prose leaves a sentence over and scores nothing. The margin is untouched, and so is what
a row in the chosen column declares: this decides only *which* column holds the names.

That table now declares `Signal HCLK is output width 1.` and `Signal HRESETn is width 1.` — two
phantoms gone, two prose widths gone, and `HRESETn`, the document's own active-LOW spelling, reaching
IntentIR for the first time. AHB's interface signal set goes from 40 to 41 with nothing removed.

Corpus-wide the new score moves the name column on 7 of 602 signal tables in the current stratum: that
one repair, and six USB 3.2 register and field tables mis-typed as signal tables, which declare
nothing either way. Every rotation the margin was introduced for still overrides — APB `table_0016`
18 against 5, AHB `table_0033` 19 against 4 — and of the 27 proof-carrying documents, only AHB's
EvidenceIR changed at all.

### A width cell that is a sentence is not a width

The same table reader takes a width from a column and asks very little of it: a whole number is a
width, and anything holding at least one letter is a *parametric* width — an expression the
integrator sets, such as `ceil(DATA_WIDTH/8)` or `clog2(Num_RP_AR)`. That is right for every form a
specification actually writes, and it is also why a table whose width column has been handed a
**description** declares the description as the wire's width:

```text
Signal C is width Operation code. Identifies the type of message carried by the channel. (Table 5.2).
```

Two conditions now separate an expression from prose, and neither works alone. A cell is prose when
it carries **more than six whitespace tokens** *and* holds **a sentence terminator followed by a
space**. Measured over every width cell the reader examines, each condition on its own refuses a real
width: the token bound alone would refuse `ceil((ID_R_WIDTH+1)/8) if ARIDUNQ is not present:
ceil(ID_R_WIDTH/8)` at seven tokens, and the terminator bound alone would refuse the ternaries
`LTI_GPC == True ? 2:1` and `LTI_MMU ? 8 : ceil(LTI_LRADDR_WIDTH/8)`, where `?` is an operator and
not a question. Together they select 47 cells corpus-wide, every one of them prose, and no legitimate
expression in either stratum. A footnote marker rides its expression directly — `ceil(ADDR_WIDTH/8) a`
— so it carries no terminator and is untouched.

**A length threshold would not have done this, and the earlier reading of the margin is corrected
here.** When only the proof-carrying stratum was in view the two classes looked separable by length,
the longest expression being five tokens and the shortest prose seven. Across the whole corpus real
expressions reach **eight** tokens while prose starts at **seven**, so the classes overlap and it is
the conjunction doing the work.

The population is the other half of the result, and it is worth stating plainly: in the
proof-carrying stratum this refuses **nothing at all** — 195 parametric width cells, none of them
prose, and no artifact moves. All 47 are in documents frozen at an older generation, and 43 of those
are one shape: a four-column `Signal | Type | Width | Description` table whose signal names arrive as
two tokens (`c opcode`, `d param`) because the identifier's underscore is not in the document's text
layer. Every row's leading token is then the same letter, so that column can offer only one distinct
name, the one-letter `Type` column wins it, and the width rotates onto the description. Refusing the
sentence there removes a declaration named `C`, not a wire. The two rows where a real identity does go
with it — one GIC and one CXS signal, both declared with a fabricated width and nothing else — become
rows the accounting counts as unread, which is the whole point of counting them.

Two readings of that rotation are worth separating, because the wrong one is the tempting one. It is
**not** the whole-cell column score refusing a name column whose cells carry a space: that column scores
one under the rule the score replaced and zero under the rule it is, while the `Type` column scores four
under both, so the rotation predates the score entirely. Measured over the whole corpus the whole-cell
rule zeroes eleven name columns, every one of them genuinely prose, and six of those are then moved onto
the real name column — `VOH`/`VOL`/`VIH`/`VIL` in place of `Output HIGH voltage`, a pin list in place of
`Point to Point`. It costs no recall anywhere. The split identifier is a separate class, and the eMMC
timing table that writes `t PERIOD` for `tPERIOD` carries it too.

Both censuses are re-derivable: `python3 scripts/measure_parametric_width_cell_shapes.py` for the width
cells, whose join control replays the reader's own column selection and must reproduce every declared
width in the proof-carrying stratum, and `python3 scripts/measure_name_column_whole_cell_score.py` for
the column score, which imports that same mirror rather than keeping a second copy of it.

### The same loss one stage later: a width the reader cannot finish reading

Everything above is about the EVIDENCE stage, where a table row becomes a declaration. A second,
independent reader runs one stage later: SemanticIR turns each `Signal X is <direction> width <W>.`
statement into the interface record that becomes the document's signal catalog. It ends with the same
kind of all-or-nothing refusal — the sentence must parse to its last token, or nothing is kept — and
the consequence is larger than a missing width, because the catalog is what decides which obligations
are promoted at all. A signal that is not in it has every requirement the document states about it
demoted to a residual.

AXI is the case that shows the size of it. The specification declares

```text
Signal WSTRB is output width DATA_WIDTH / 8.
```

and the width reader consumed exactly one whitespace token, so `/` and `8` were left over and the
whole declaration — identity, direction and all — was thrown away. Written `DATA_WIDTH/8`, with no
spaces, the very same width would have been read. The tell was in the catalog all along: `WSTRBCHK`,
the parity signal that checks `WSTRB`, has a plain parametric width and is present; the wire it checks
is not.

A width is now read as an *expression* — numbers, parameters, `+ - * /`, balanced parentheses, and the
call form specifications write as `ceil(…)` — and the prose a document writes after it
(`… if ARIDUNQ is not present: …`) no longer discards the declaration in front of it. Two conditions
keep it from reading a width out of text that is not one: the expression must end where a token ends,
so a Verilog literal such as `3'b000` cannot be read as the number `3`; and trailing material is
tolerated only after a structured expression, so an ingest-mangled `log 2 (DATA_WIDTH) -` is not read
as a width of `log`.

Measured over the corpus on `2026-09-13`: **83 declared signals across ten documents never reached
the catalog**, of which 17 state an arithmetic width. Fourteen of those seventeen now read, and AXI's
own catalog grows from 288 signals to 296 — `WSTRB` among them, which immediately restores the
requirement *"An attached Subordinate must have its WSTRB input tied HIGH"* from a residual to a
published constraint, and carries it into the emitted `.isf`. The remaining 69 are tracked as
`SIGNAL-DECLARATION-ROW-DROP.4b`: most of them have a declaration whose width text the source row
never stated as a width at all, which is a defect upstream of this reader — but the refusal is still
silent, which is not.

### What no shape can decide

A bus-mode matrix that SourceIR typed as a signal table has rows named `HS200` and `HS400`, single
tokens indistinguishable from a wire. A row-level rule refuses the phrases in that table and stops
there, which is the measured reason the question moves up to the table or to the classifier rather
than being answered here. Tracked as `PROSE-NAME-CELL-DECLARATION.0` and `.2`; the population is
re-derivable with `python3 scripts/measure_declaration_name_cell_shapes.py`.
