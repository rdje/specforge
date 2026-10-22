# ISF Enums: Which Codes Reach The Adapter

The `.isf` adapter lowers a recovered enumeration to a backing `(type NAME (bits k))` and an `(enums …)`
family; how it renders them is in [ISF Adapter](isf-adapter.md). The type is as wide as the largest member code
(`EXTRACTION-REPRESENTATION-GAPS.59`). It used to be sized from the member count, which left AXI's `AWATOP` 4 bits
wide for code 49 and `ARCACHE` 3 bits for 15; FSMGen checks an enum's values against nothing, so the width has to be
right where it is written. This chapter covers the question a reader of an emitted `.isf` asks next: which table codes qualify as an enum, how each member gets its
name, and where its value comes from, including what happens when the document never states one.

### Only *named* enums reach the `.isf` — no "Table N" junk

An enum is the encoding of a *specific* thing — a signal, a field, an instruction — so it needs a real
name. SpecForge derives that name from the document's own words: it first matches the table's
caption/section/headers against the declared signals, and only if none matches does it fall back to the
first signal-shaped word in the caption. That fallback used to be too eager: a table captioned
*"Table 7 — HBM Mode Register Overview"* would be named **`TABLE`** (the literal caption keyword), and
because every such table shared that name, dozens of unrelated value-tables **fused into one giant junk
`(type TABLE …)` enum** — a 6-bit "enum" claiming to encode lane-remap codes, microbump geometry, and a
footnote all at once. A reader of the `.isf` could not tell it was noise.

SpecForge now **gates that fallback**: a fallback name is kept only when the candidate word is
*independently evidenced* in the document — it is a **declared signal**, or it appears as a **column
header** of that very table — otherwise no enum is minted (an honest absence, never a fabricated one).
This is a structural rule, not a list of banned words (a token like `DATA` is kept where the document
declares it as a real signal, and dropped where it is just a caption word). The genuinely-named enums
(`HTRANS`, `PPROT`, `TKEEP`, …) are untouched; the junk vanishes. Measured when the gate shipped, it took
the generic-named enums from 82 down to 8, counted over the 33 documents that could be rebuilt at the
time; the 8 remaining were tokens the document itself uses as a column header, such as a
"Table of Contents" header. **Re-derived `2026-09-21` over today's 27 rebuildable documents the count is
0** — the gate holds, and holds better than when it shipped. Read that denominator carefully, because
the whole-corpus number looks like the opposite: **67 generic-named enums are still on disk**, and all
67 are in the 51 legacy chains that predate the gate and cannot be rebuilt without re-ingesting the PDF.
They are frozen artifacts of the old behaviour, not evidence of the new one. Alongside that, every
wire-protocol gold's real enums stayed byte-for-byte identical and every emitted `.isf` stayed
FSMGen-strict-clean. As a final
tidy-up, the backing `(type NAME …)` line is now emitted only when its `(enums …)` family is — so a dropped
enum never leaves a dangling type declaration behind.

**A later census found the caption keyword coming back in through a door the gate does not watch, and it
is worth understanding why.** The gate above guards the *fallback* — the path taken when no declared
signal matches. It leaves the earlier path alone, because a name that matches a signal the document
itself declares is supposed to be trustworthy. But the text that path searches is the caption, the
section heading and the headers all joined together, and **a caption begins with the publisher's
numbering label** — `Table 2-1`, `Figure 8.3`, `Chapter 4`. That label is identical on every caption in
the document, so as soon as a word like `Table` reaches the declared-signal catalog from somewhere else
in the document, it names every captioned table at once and they merge again.

Measured across the whole persisted corpus on `2026-09-21`, the caption label is still naming enums:
forbidding it as a name source would reach exactly the enums named `TABLE`, `FIGURE` and `CHAPTER`, and
no others. `CHAPTER` is the useful one to notice — it shows the rule has to be *grammar*
(`<word> <number>` for whatever word the publisher chose) and not a list of two banned captions.

The same census settled a question the obvious fix depends on. When several tables do fuse under one
name, is that ever *legitimate* — one long code table the publisher split across a page break? A
specification can say so in the caption itself — either right after the label
(`Table B2.2 Continued from previous page`) or at the end (`… (continued)`) — and SpecForge already
reads the first form elsewhere. Counting both, across the corpus **two fused enums are a stated
continuation, both in the AXI specification**, and the rest are not. A later census had to correct what
that finding implies: those two enums are legitimate in *identity* — the document really does say the
fragments are one table — and their **contents are junk anyway**. One of them reads
`DISABLED = 0, CONNECT = 0, ENABLED = 1, DISCONNECT = 1`, every value duplicated, for the reason the next
section explains. Knowing which tables an enum came from says nothing about whether its members and
values are right. But the weaker evidence — same header, adjacent pages —
points both ways and cannot be used to fill the gap: an AXI snoop-code table genuinely resumes across
a break while saying nothing, and three RISC-V register-field tables share the header
`Field | Description | Access | Reset` only because a register chapter prints that header once per
register. So "refuse any enum whose members span tables" cannot be made safe, and the fix belongs at
the naming seam instead.

#### The label is now stripped — and stripping a name turns out to *add* codes

SpecForge removes the numbering label before either naming path reads the caption, so
`Table A7.3: AWSNOOP encodings` is read as `AWSNOOP encodings`. The rule is grammar rather than a list
of banned words: a word followed by a numbering token, where the token may open with a volume letter
so Arm's `A7.4` and `B2-57` are matched too. Across the corpus that grammar is well founded —
**5,408 of 5,531 captions carry such a label, and the leading word is one of exactly three**
(`Table`, `Figure`, `Chapter`). One exception protects real names: a first word the document itself
*declares* is never stripped, so a caption like `PPROT 3 encoding` keeps its signal.

The surprising part is the effect. Stripping a name source sounds purely subtractive, and it is not:
**two documents gain 40 encoding statements and none loses any.** The reason is that the enum's name
is also what SpecForge uses to find the table's *value* column. The AXI table above has the columns
`AWSNOOP | AWDOMAIN 1 | Opcode | Enable | Description`. Named `TABLE`, nothing matched a header, so
the word `Opcode` was taken for the value column — its cells are words like `WriteNoSnoop`, they do
not parse as codes, and every member was therefore dropped. Named `AWSNOOP`, the value column is the
`AWSNOOP` column and its `0b00000` codes read correctly. **The label was not just adding a junk name;
it was taking the naming slot and pointing the reader at the wrong column.**

What arrives is mostly exactly right — AXI `AxSIZE` (`1 byte per transfer` = 0 through
`128 bytes per transfer` = 7), `AxBURST` (`FIXED`/`INCR`/`WRAP` = 0/1/2), `AxMMUFLOW`,
`AxMMUSECSID`, `Security`, `AxDOMAIN`, and ADIv6 `Size`. Eleven of the forty were half right when
this landed: the values were the document's own, but the member *names* came from the `Description`
column, because the column-picking rule knew the word `description` and not the word `opcode`. That
was a separate defect in a different part of the reader, visible only because this change stopped
hiding the table, and it is now repaired — see *The member's name comes from the column the document
names it in* below. Every wire-protocol gold score is unchanged — the
before and after eval reports are byte-identical across all five gold datasets, on chains rebuilt with
both versions of the binary.

*(`2026-10-21`.)* Until `EXTRACTION-REPRESENTATION-GAPS.58`, no `AxSIZE` enum reached SemanticIR or the `.isf`.
Every member's name starts with a digit (`1_BYTE_PER_TRANSFER`), and SemanticIR's identifier parser dropped such a
member without a trace, as it did 22 member statements in four documents. Now a member name that starts with a digit
keeps its words behind the enum's name, since an identifier leads with a letter: AXI lowers `(AxSIZE
(AXSIZE_1_BYTE_PER_TRANSFER 0) … (AXSIZE_128_BYTES_PER_TRANSFER 7))`. A "name" that is only a number is no name. That
is a quantity column, or a table read by the wrong key, such as APB's `PPROT` bits, AHB's `HPROT` flags and CHI C2C's
message-credit counts. It mints no enum. Since `.3.c.iv.b.i` a table naming a signal's bits is that signal's layout
instead (see *A signal's bit sub-fields*).

The enums this change *removes* — the 54 still named `TABLE`, `FIGURE` and `CHAPTER` — are all in the
legacy stratum, so they will disappear as those documents are re-ingested rather than today.

### Keeping the real codes when a table fuses with its prose — the member-quality gate

Naming the enum correctly is only half the job. Even a correctly-named enum can be polluted from the
*inside*. Specs often place a clean code table (`OKAY = 0`, `EXOKAY = 1`, `SLVERR = 2`, `DECERR = 3`)
right next to a paragraph that describes those same codes in full sentences, both labelled with the same
signal name. When SpecForge reads the table cells it sanitizes each name to an identifier, so a whole
sentence in a description cell becomes one giant "member" like
`THE_REQUEST_HAS_REACHED_AN_END_POINT_BUT_HAS_NOT_COMPLETED`. The two tables share a name and merge, and
the AXI `BRESP` enum ends up as 16 members — the four real codes drowned among a dozen captured sentences.

You cannot fix this by throwing the whole enum away: that would delete `OKAY`/`EXOKAY`/`SLVERR`/`DECERR`
along with the junk. The fix has to work **one member at a time**, and it does so with a simple, universal
observation: *a real hardware code-name is an identifier, and an identifier never contains the spine of an
English sentence.* So SpecForge drops any member whose words include a copula, auxiliary or modal verb
(`is`, `are`, `be`, `has`, `must`, `shall`), an article or demonstrative (`the`, `this`, `that`), or a
relativizer/subordinator (`which`, `when`, `if`, `because`) — the connective tissue of prose that an
encoding value has no reason to carry. The genuine codes survive; the sentences are dropped; and if *every*
member of a table turns out to be a sentence (a pure description block that was never an enum at all), the
enum simply isn't emitted — an honest absence rather than junk.

This is grammar, not a banned-words list, so it travels to any spec. It deliberately leaves out six words
that double as real hardware names — `a` (a single-letter port suffix like `MASKLANE_A`), `i`, `its` (the
Arm GIC's **ITS** block), `can` (the CAN bus), `may` (the month), `am` — the same care taken elsewhere when
a common English word collides with a chip term. Measured across the whole corpus the rule is exact: it
flagged **none** of a 115-member clean-code reference set (zero false positives) and **all** of a
269-member prose-fragment set (full recall). In practice the AXI `manager.isf` now reads
`(BRESP (OKAY 0) (EXOKAY 1) (SLVERR 2) (DECERR 3) (DEFER 4) (TRANSFAULT 5) (RESERVED 6) (UNSUPPORTED 7))` —
a faithful response enum recovered from what used to be a 16-member tangle — while the wire-protocol golds'
scored facts stayed identical and every emitted `.isf` stayed FSMGen-strict-clean. A few harder cases are
left as honest residuals: glossary "see also…" cross-references, document front-matter, table-caption
fragments, and conflations whose members all *happen* to be clean identifiers (these are kept in full
rather than risk dropping a real code).

### When a width *parameter* sneaks in as a code — the `_WIDTH` gate

There is one more way a correctly-named enum gets polluted from the inside, and it is sneaky because the
intruder *looks* like a clean identifier. A spec often lists, near a signal's encoding table, a
configuration **parameter** that sets that signal's bit-width — written `BRESP_WIDTH`, `AWSNOOP_WIDTH`, and
so on. When that parameter row is read into the value table, the result is a perfectly well-formed member
name like `BRESP_WIDTH = 0`. The sentence-spine gate above waves it through (it carries no prose), so it
reaches the `.isf` — and there it does real harm. `BRESP_WIDTH = 0` lands inside the `BRESP` enum *next to*
`OKAY = 0`, so two different "codes" now claim value `0`. Worse, where the only thing captured for a signal
was its width parameter, the enum becomes pure noise: `(RRESP (RRESP_WIDTH 0))` — a response enum whose
sole "value" is its own width, with the genuine `OKAY`/`SLVERR`/`DECERR` codes nowhere in sight.

The tell is structural and reliable: a member named `<X>_WIDTH`, where `X` is the enum's own name or a
signal the document actually declares, is **that signal's width — a parameter, never one of its encoding
values.** So SpecForge drops it. Crucially, the test is grounded in the document's own list of declared
signals, not a banned-words list, which is what keeps it safe: a genuine width-*value* in a link-width
enum — `FULL_WIDTH`, `HALF_WIDTH`, `QUARTER_WIDTH` — is **kept**, because `FULL`/`HALF`/`QUARTER` are not
signals the spec declares. Measured across the whole corpus the gate fires on exactly the seven real leaks
(all in the AMBA AXI specification) and nothing else. After it, the AXI `manager.isf` reads
`(BRESP (OKAY 0) (EXOKAY 1) (SLVERR 2) (DECERR 3) (DEFER 4) (TRANSFAULT 5) (RESERVED 6) (UNSUPPORTED 7))`
and `(AWCMO (CLEAN_AND_INVALIDATE 0) (CLEAN_ONLY 1))` — no duplicate-value collisions — while the enums
that were *only* a width parameter simply disappear (an honest absence). The wire-protocol golds' scored
facts stayed identical and the emitted `.isf` stayed FSMGen-strict-clean.

### When only the table's own header knows the field's name

The three gates above all *remove* something. This one adds. Naming an enum starts at the table's
caption and section title, and the column headers were used only to confirm a name — never to supply one.
That left an odd blind spot. Consider a table captioned *"Table 3-1: Stream Security determination"* whose
header row reads:

```text
| SEC_SID value | Description |
```

The caption contains no field name, so the fallback found nothing to keep and the table minted no enum at
all — even though the header names the field outright, one column to the left of the meanings it encodes.
SpecForge now reads that header as a **source** of the name, not only as a veto. The clause runs last, after
the declared-signal match and the caption fallback have both declined, so nothing that was already named
changes: this can only recover encodings that were previously lost.

Reading a header is weaker evidence than matching a declared signal, so the table has to look like an
encoding before its header is trusted. Four checks decide it, and each one exists because a real corpus
table failed it:

- **The left column must name a field, not a position.** A header word such as `Bytes`, `Offset`, `Index`
  or `bits` says the column holds *where* something sits — a byte offset in a data structure, a register
  offset in a block, a bit range in a field. That is a layout table, and it has no encoding to name. This
  single check accounts for most of the rejections.
- **No value cell may be a range.** A cell like `03:02` or `[2:0]` spans several bit or byte positions, so
  it is an extent rather than one encoded value — the same layout tables, caught from the other side.
- **At least one value must actually parse as an encoding literal** (`0b10`, `0x3`, `11`, `4'hF`). This is
  the check that keeps glossaries out. An abbreviation table (`Acronym | Description`), a notation legend
  (`Notation | Meaning`), and a terminology table (`Term | Meaning`) all have exactly the two-column shape
  an encoding table has — and none of them encodes anything, so none has a parseable value. The same check
  removes parameter tables such as `Data_Width value | Description`, whose only "value" is a legal range
  like *128, 256, and 512*.
- **The header must leave exactly one field name** once role words (`value`, `description`, `bits`) and
  document-structure words (`Table`, `Figure`, `Section`) are set aside. Two candidate names means the
  shape was not understood, and SpecForge declines rather than guessing.

Nothing here is a list of chip or protocol names: the name comes from the document, and the gates are
ordinary document grammar plus the notation values are written in. Names recovered this way merge across a
document exactly as declared-signal names do, and that turns out to be the right behaviour rather than a
risk — the Arm SMMU specification describes its `SH` shareability field in eleven separate tables, and every
value the eleven share agrees, so merging them reconstructs one correct encoding instead of fusing
strangers. Measured across the corpus, the rule recovers field encodings such as `AWATOP`, `ENDIAN`,
`EXCL`, `RESPERR`, `ARCHID`, `CD2L`, `VMID16` and `PRI` that had no name before; the exact population and
per-class rejection counts live in the measurement record rather than here.

One honest note about what you will see today. Most of the documents this reaches were ingested by an
older version of SpecForge, and their stored pipeline artifacts cannot be replayed by the current build —
they have to be re-ingested from the PDF first. Until then the recovered enums exist in the code path but not in
the artifacts on disk, and every stored `.isf` in the repository is byte-for-byte what it was before. The
`specforge ingest` command is what closes that gap, per document.

### Where an enum's *value* comes from — and what happens when the document never gave one

Every gate above is about **names**: the enum's name, then its members' names. None of them asks where
the *number* on the right-hand side came from, and that turns out to be where most of the damage is.

SpecForge picks a value column for the table, reads the cell, and parses it as an encoding literal —
`0b10`, `0x3`, `11`, `4'hF`. **A member whose cell does not parse is now not minted at all**: the enum
keeps the codes the document states and quietly loses the rest, which is the same honest-absence rule
the name gates above already follow.

It did not always work that way, and the reason the change matters is worth a paragraph. The producer
used to fall back to **the row's position in the table** — first row `0`, second row `1` — and nothing
in the emitted statement distinguished that from a real code. `Enum AWBURST INCR = 1.` read identically
whether the document said `0b01` or said nothing at all and the row simply happened to be second.
**Measured across the whole persisted corpus on `2026-09-21`, 10,189 of 19,023 minted values — 53.6% —
had come from the row position rather than the document**, and only **84 of the 512** symbol
definitions reaching `IntentIR` came from an enum whose every value the document states.

Those two figures describe **the artifacts stored in this repository, not what SpecForge does today**,
and the difference is large enough to matter when you read them. Re-derived `2026-09-29`: forty-seven of
the seventy-eight stored documents were built by older versions and cannot be replayed without
re-ingesting their PDF, and between them they hold **490 of the 522 enums and 20,392 of the 20,571
members**. The current pipeline, across the thirty-one documents it can rebuild, mints **32 enums and
179 members**. It minted 15 and 79 when the row-position fallback was removed; two later fixes described
further down raised that by recovering encodings that had been hidden rather than by relaxing anything
(the caption-label fix, and reading member names from the column the document names them in), and
documents rebuilt on the current pipeline since then joined the count. The defect was real either
way — on those rebuildable documents alone the fallback had invented 52 values in 20 enums — but the
headline percentage is a fact about the corpus on disk.

Two things made it worse than a coin flip. First, the fallback was **silent and per row**, so one enum
could carry a real code and an invented one side by side — 218 enums did. Second, the row position is
**table-local**, so when several table fragments fuse into one enum each fragment starts counting from
zero again and the same value lands on several members. That is why the page-break continuation
mentioned earlier reads `DISABLED = 0, CONNECT = 0, ENABLED = 1, DISCONNECT = 1`. Across the corpus
**5,688 member slots carried a value the same enum already used for a different member** — something no
real encoding does, so no interpretation was needed to call those wrong.

The worked example is the AXI specification's `AWBURST`. Its table is a burst-address *example*
(`AWADDR | AWSIZE | AWLEN | AWBURST | 7 | 6 | … | 0`). The column literally headed `AWBURST` holds
`INCR` and `WRAP` — words, which do not parse — so the value became the row position, and because that
column had been claimed as the value column the member names were taken from the `AWADDR` column
instead. The result is

```text
Enum AWBURST 0X00 = 0.  0X01 = 1.  0X04 = 2.  0X06 = 3.  0X00 = 4.  0X04 = 5.  0X00 = 6.  0X08 = 9.
```

— members named after addresses, values that are row numbers, `0X00` appearing three times with three
different values, and the real `FIXED`/`INCR`/`WRAP` codes nowhere in it. Under the current rule that
enum is gone entirely: no cell in its value column parses, so nothing is minted.

Was the row position ever what the document meant? Almost never: **94.5%** of the invented numbers
appeared nowhere in their own source row, and the one defensible pattern — a table whose rows are
`HPROT[0]`, `HPROT[1]`, … so the position really is the bit number — accounted for 66 members out of
10,189, and even there the *kind* was wrong (`BUFFERABLE` is bit 2 of `HPROT`, not a value of it).

**What you will see on disk, which is less than the 53.6% suggests.** Removing the fallback changed
**54 statements across 4 documents**, not 10,189 — because **99.5% of the invented values are in the 51
legacy chains that cannot be replayed without re-ingesting their PDF**. They are frozen artifacts of the
old behaviour. On the 27 documents that can be rebuilt, five enums disappeared outright (AXI `AWBURST`,
`AWDOMAIN`, `ARDOMAIN`, AHB `HREADYOUT`, Wishbone `Nomenclature`), two of which had reached `IntentIR`
as symbol definitions; four more enums kept their real codes and lost only their invented ones. Nothing
else changed anywhere: no statement was added and no non-enum statement was removed.

#### The quote is not decoration — it says which base to read

**357 of those values *are* stated**, as quoted literals like `'1110'` in the three OpenCAPI
transaction-layer specifications, and the reader did not strip the quotes, so it treated them as
unstated. SpecForge now reads them, and *how* it reads them is the interesting part.

The obvious repair is to strip the quotes and parse what is left. That is wrong, and the documents say
why. Each of them carries a notation section that reads:

> Binary values in sentences are shown in single quotation marks. For example '1010'.
>
> Hexadecimal values are preceded by an x and enclosed in single quotation marks. For example x'0A00'.

**The wrapper carries the base.** Strip it and `'1110'` becomes the bare digits `1110`, which the
encoding parser reads as the decimal number one thousand one hundred and ten — the document meant
binary, which is **14**. The parser tries decimal before it tries binary, so its binary branch is only
ever reached by a digit string too long to be a decimal `u32`, which means eleven digits or more.
Scored against the documents' own rows, stripping the quotes would have been **wrong on 314 of the 357
values, and on 57 of them worse than the row number it replaced** — those rows happened to sit at a
position that matched their code.

So SpecForge reads the wrapper as part of the value: `'1010'` is binary `10`, `x'0A00'` is hexadecimal
`2560`, and a space or underscore between digits is grouping (`'0000 0000'` is `0`). A quoted cell that
is not one literal is still left unread, for the same honest-absence reason as everywhere else — a
range `'0011' - '1110'`, two values in one cell `'0010 0000' '0010 0100'`, a don't-care pattern
`'0000 000x'`, or a quoted word like `'IVRS'`.

Two limits are worth stating plainly. This reading applies to the **value** of an enum member and
nowhere else: the same parser also decides whether a table is an encoding table at all, and teaching
*that* to read quoted literals would make new enums appear rather than repair existing values, which is
a different question with a different risk. And **you will not see any of this on disk yet** — all
three documents are in the legacy stratum, so every rebuildable chain produces byte-identical output.
It lands now because a re-ingest bakes in whatever rules exist when it runs, and these documents are
next in the refresh queue.

### An unquoted code: the table states the radix, not the parser

A document that does not use quotation marks still states the radix — it just states it with the shape
of the column. `0010` beside `0011` and `1010` is binary, because nobody writes a decimal ten as
`0010`; `10` and `11` at the end of `0 1 2 … 9` is decimal, because that is a counting run whose last
two entries happen to contain only zeros and ones.

So SpecForge reads an unquoted cell of only `0`s and `1`s as **binary when the table itself shows
zero-padding at that width**, in the cell's own column or anywhere in its row, and as decimal
otherwise:

```text
MsgType[3:0]:  0001  0010  0011  1001  1010  1011   ->  1, 2, 3, 9, 10, 11   (binary)
Bits:          0  1  2 … 9  10  11                  ->  0, 1, 2 … 9, 10, 11  (decimal)
op0 | op1 | CRn  | CRm  | op2
11  | 011 | 0100 | 0100 | 000                       ->  op0 = 3             (binary, from its row)
```

Two rules keep that from over-reaching. A column that carries a digit above `1` **at the same width**
is refuted — `01 … 09 10 11 12` is zero-padded *decimal*, and its leading zeros mean nothing about
radix. And the test is per width, because a single column can hold both: an OpenCAPI command-flag table
lists bit indices `3 2 1 0` above encodings `0010` and `0011`, and the indices must not decide the
encodings' radix. A cell that no shape decides keeps its decimal reading, which is the same
honest-absence rule as everywhere else.

Measured across the whole persisted corpus on `2026-09-22`: **336 minted member values were read this
way, 198 of them binary, 133 decimal and 5 left undecided** — and, as with the quoted literals, **you
will not see a change on disk**, because every one of the 336 was in the legacy stratum and all 27
rebuildable chains produced byte-identical output. It lands now for the same reason: a re-ingest bakes
in whatever rules exist when it runs.

Unlike the quoted reading, this one is provably safe to state in the shared parser's terms: changing
which radix a digit string is read in never changes *whether* it can be read, so it cannot make a table
start naming an enum it did not name before.

(Tracking: `KG-ISF-COMPLETENESS.5.vii` for the measurement, `.5.vii.a` for the fallback removal,
`.5.viii` for the quoted literal, `.5.xi`/`.5.xi.a` for the unquoted radix, `.5.x`/`.5.xii` for the two
limits above; the full per-class numbers are in the measurement records.)

### A code that names two things names neither

An encoding assigns one code to one meaning. When SpecForge assembles an enum and finds the same code
on two different members, the document has not been read correctly — and the reader cannot tell which
member the code belongs to, so it publishes neither. Both members are dropped; an enum left with no
member at all is not published.

This is the mirror of a rule SpecForge already applied: a member *name* that turns up with two
different codes is dropped too. One name must mean one code, and one code must mean one name.

The cause is almost never the value itself, which the document really does state — it is the column it
was read from. Three shapes account for every case measured on `2026-09-22`:

```text
HSIZE[2] | HSIZE[1] | HSIZE[0] | Size (bits) | Description    <- the code is split across columns,
   0     |    0     |    0     |      8      | Byte              so reading one gives every member
   0     |    0     |    1     |     16      | Halfword          the same code

AWSNOOP  | AWDOMAIN     | Opcode         <- two keys decide the operation together, so AWSNOOP
0b00000  | NSH, SYS     | WriteNoSnoop      alone repeats down the table
0b00000  | SH           | WriteUniquePtl
```

and a third where one enum is assembled from two tables that each number their own members.

**Measured on `2026-09-22`: 40 member values were withdrawn across 8 enums in 2 documents**, three of
those enums withdrawn entirely, and nothing added. AHB's `HTRANS` and `HRESP` are untouched beside the withdrawn
`HSIZE` and `HPROT`, because their codes are distinct.

Since `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.a` (`2026-10-21`) the first shape is read for what it is: one signal's
bit columns are concatenated into its code, so `HSIZE` is published again (see *A table that codes its member in
several signal-titled columns* below). The `enum_code_collision_holdout_gold` fixture now collides on a single
`TSIZE` column, so it still holds the rule.

One shape does share codes for real. AXI *Table A4.4* gives each memory type an `ARCACHE` and an `AWCACHE` code, and
on reads `0b1010` is both Write-Through No-Allocate and Write-Allocate, because a read cannot carry the write-allocate
hint. Since `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.c.i` such a table has its own reader. Its signal-titled code
columns name members of one collective, one key written per channel, and each channel becomes that signal's enum with
every member and its own code. The reader ends the channel with *Enum ARCACHE shares a code between members.* That
sentence, citing its table, is the only thing that exempts an enum from the collision rule. A bracketed code, which
the document calls *permitted but not preferred*, is kept as *Enum ARCACHE WRITE_THROUGH_READ_ALLOCATE also takes 6.*
and not emitted. On `2026-10-22` AXI's `ARCACHE` grew from 6 members to 12, `AWCACHE` from none to 12, and FSMGen
strict accepts both.

(Tracking: `KG-ISF-COMPLETENESS.5.xiv` for the measurement and `.5.xiv.a` for the rule.)

### The member's name comes from the column the document names it in

Many encoding tables give each code both a short name and a sentence. AXI writes

```text
AxBURST | Label | Meaning
0b00    | FIXED | Fixed burst
0b01    | INCR  | Incrementing burst
```

and the column the reader picks from the header is the first one headed like a name, which here is
`Meaning`. So the enum used to read `FIXED_BURST = 0, INCREMENTING_BURST = 1`, and a table whose
`Meaning` cells are all full sentences produced no enum at all, because every sentence is dropped as
prose. SpecForge now looks at the other columns, and **re-points the member name only when the table
itself shows it should**:

- the header's choice was a description column (`Meaning`, `Description`). A column headed `Name` is
  the document saying where the names are, and it is never second-guessed;
- the value column is written as codes (`0b01`, `0x1F`, `0010`). A register-field table has bit
  positions there (`[31]`, `12:7`), and its perfectly tidy `Field` column names fields, not codes;
- another column holds identifier-shaped names that each label **one** code, clearly more of them
  than the description column has, after subtracting any row whose name labels several codes. That
  keeps an access column (`RW`, `RO`, `RW`) and a lane-remapping grid from winning, and a column of
  numbers never counts as names at all — AXI `AxSIZE`'s `Label` column is `1 2 4 … 128`, byte
  counts, and its members stay `1_BYTE_PER_TRANSFER` and so on.

When the document's own name column shows **two different names on one code**, the table is keyed by
more than its value column — AXI's `AWSNOOP` operation is chosen by `AWSNOOP` *and* `AWDOMAIN`
together, as the diagram above shows — so it cannot be a list of codes, and it now contributes no
members at all. Its prose column could not prove this: two different sentences may describe one
member, and one sentence described three different codes.

**Measured on `2026-09-22`**, over every rebuildable document with the old and new binaries, only the
AXI specification changes: 29 member statements go and 46 arrive. The names now match the
specification (`FIXED`/`INCR`/`WRAP`, `ADD`…`UMIN`, `CLEANINVALID`, `NONE`/`SEPARATE`/`FAIL`/`PASS`),
Tables A7.3 and A7.4 are withdrawn as two-key tables, and three encodings appear that were missing
entirely: `RRESP` (`OKAY` … `OKAYDIRTY`, 8 codes), `ARTAGOP` and `AWTAGOP` (4 each). The other 26
documents are byte-identical. In the 51 stored documents built by older versions, the same rule would
re-point 25 table readings and withdraw 7; those will change as the documents are re-ingested.
Since `.62` (`2026-10-22`) A7.3 and A7.4 are value combinations keyed by the snoop code and the domain together (see
*A row keyed by several signals together*).

(Tracking: `KG-ISF-COMPLETENESS.5.xiii` for the measurement and `.5.xiii.a` for the rule.)

### An encoding filed as a signal table

The converter sometimes files a signal's encoding as a *signal description* table, because its first column is
headed by a signal name. AHB's *Table 3-4 Burst signal encoding* is one:

| HBURST[2:0] | Type | Description |
| --- | --- | --- |
| `0b000` | SINGLE | Single transfer burst |
| `0b001` | INCR | Incrementing burst of undefined length |
| `0b010` | WRAP4 | 4-beat wrapping burst |

Neither enum reader used to look at a signal-description table, so AHB carried `HTRANS` and `HRESP` and no `HBURST`.
Such a table is now read as an encoding when it **enumerates the one signal its first header names**: the first
header is that signal (a bit range may follow), most rows open with an encoding literal, and no other column is one
of literals. A signal inventory lists names, not codes, in its first column, so it never qualifies; a table of
*permitted combinations* carries codes in several columns and is keyed by more than one of them, so it does not
either.

AHB now lowers `(HBURST (SINGLE 0) (INCR 1) (WRAP4 2) (INCR4 3) (WRAP8 4) (INCR8 5) (WRAP16 6) (INCR16 7))`, which
FSMGen's strict check accepts. Across every stored document the rule reaches only this table and one permitted-combinations
table in an older AXI revision, which the last condition refuses; AHB's *Table 3-5 Protection signal encoding*
gives each `HPROT` bit its own column, so it is a set of flags rather than one enum, and no enum is read from
it; since `.3.c.iv.a` it is a value combination keyed by the bits (see *A row keyed by several signals together*).

(Tracking: `EXTRACTION-REPRESENTATION-GAPS.3.h`.)

### A table that codes its member in several signal-titled columns

Some encodings title more than one code column by a signal. AXI *Table A4.7* gives each physical address space its
`AxPAS` code beside the `AxPROT[1]` and `AxNSE` bits that jointly key four of the spaces. AHB *Table 3-2* writes
`HSIZE` as three one-bit columns, `HSIZE[2] | HSIZE[1] | HSIZE[0]`. The single-column reader takes one value column,
and a table keyed by more than that column mints nothing. For `HSIZE` it was worse: it read `HSIZE[2]` alone and
published six members on two codes.

An encoding table's signal-titled columns are now read one signal at a time (`EXTRACTION-REPRESENTATION-GAPS.3.g.iii.a`).
A column title names a signal when it is a declared signal or a collective name the document defines (`AxPAS`).

- **Codes.** Each signal's column is its code. One signal's bit columns are concatenated into one code, most
  significant first, and a single bit of such a signal is no longer read alone.
- **When a column is an enum.** It is an enum only when its codes determine the member: two or more rows state one,
  and no code or member name repeats.
- **Member names** come from the first column that is no signal and no code and whose names are distinct.
- **Other cases.** A table whose every row the converter put in its header is read from its second row on. A column
  that only jointly keys its member, or repeats codes, is left alone here: the next section reads the first, and
  the second is read by `.3.g.iii.c.i` as an aliasing encoding whose members share codes (see the collision rule above).

AXI now lowers `(AxPAS (SECURE 0) (NON_SECURE__NS 1) (ROOT 2) (REALM 3) (SYSTEM_AGENT__SA 4) (NON_SECURE_PROTECTED__NSP
5))`. AHB lowers `(HSIZE (BYTE 0) (HALFWORD 1) (WORD 2) (DOUBLEWORD 3) (HSIZE_4_WORD_LINE 4) (HSIZE_8_WORD_LINE 5))`,
3 bits wide since `.58` keeps its two digit-led members. FSMGen's strict check accepts both.

### A row keyed by several signals together

Some tables key each row by several signals' values at once. APB's untitled `PNSE | PPROT[1] | Physical address
space` names an address space for each pair of bits. AXI *Table A15.26* names the four Coherency Connection states
(`Disabled`, `Connect`, `Enabled`, `Disconnect`) by `SYSCOREQ | SYSCOACK`. No column alone is an enum there, yet the
single-column reader took one value column and published it. AXI got a `BTAGMATCH` with seven members on four codes, a
`SYSCOACK` with `DISABLED = 0` and `CONNECT = 0`, and, from *Table A4.7*, an `AxPROT` with `SECURE = 0` and `ROOT = 0`.
SemanticIR dropped all fifteen statements, so none reached an `.isf`. It also dropped AXI's real `BTAGMATCH`
encoding (*Table A12.14*), whose codes the false members contradicted.

Such a table is now an EvidenceIR `value_combinations` record (`EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b.i`):

- **Keys.** Two or more columns are titled by different declared names: a signal, a bit or slice of one, a
  collective, a register's field (`CSW.ERRSTOP`), or a field a register declares, written in capitals (`TYPE`).
  Distinct bits of one signal count too, when a bit cell is no plain bit (`.3.c.iv.a`): AHB *Table 3-5*'s `HPROT[3]
  Modifiable` … `HPROT[0] Data/Opcode`, each row one bit's value and meaning, and *Table 3-7*'s memory types. A title
  may name its bit after the slice. Bits that are all `0` or `1` stay the signal's concatenated code (`HSIZE`).
  Every column is titled and none only by a number. Columns that all name members of one collective (A4.4's
  `ARCACHE | AWCACHE`) are one key written per channel, read as each channel's aliasing enum (`.3.g.iii.c.i`).
  Since `.62` (`2026-10-22`) a title's trailing footnote mark is no part of its name when a note beside the table,
  or beside a fragment its caption names, uses it: AXI *A7.4*'s `ARDOMAIN 1` names `ARDOMAIN`.
- **Member and meanings.** The member column is the first other column whose cells are distinct names; a reserved
  row (`RESERVED`) names no member and may repeat. The rest are meaning columns. Every cell is kept as written (`-`,
  `0b01 0b10`, `Non-secure`).
- **Continuations and refusals.** A continuation page joins the table it continues. A table with an empty key cell
  is refused and counted in the `value_combinations` manifest, because the converter records no span to fill it from.
- **Precedence.** The single-column reader leaves such a table alone. A column whose codes do determine the member is
  still its key's enum, so A4.7 keeps `AxPAS`.
- **States.** A member column whose title ends in *state* names protocol states. A15.26's four join
  `protocol_states`, each citing its table and the combination (`supporting_combination_ids`).

With the false statements gone, AXI's `manager.isf` gains `(BTAGMATCH (NONE 0) (SEPARATE 1) (FAIL 2) (PASS 3))`
from *Table A12.14*, FSMGen strict-clean.

Since `.3.g.iii.b.ii` the record crosses SemanticIR and IntentIR unchanged. A key value that is a label of its key's
encoding carries that encoding's codes (`AxMMUSECSID` `Non-secure` is `0b00`), as a stated absent value does: 34
values in AXI. A member written with its abbreviation carries its name as a label (`Non-secure` is `Non-secure (NS)`,
since `.3.g.iii.b.iv`), while a list of alternatives (`Stall, NoStall, PRI`) and a class of codes (`Cacheable`) join
nothing. The ISF adapter states each record as one `isf_protocol_value_combination_*` residual saying why it stays: 15
across 4 documents on `2026-10-22`, AXI's opcode decodes *A7.3* and *A7.4* among them since `.62`. Their domain lists
(`NSH, SYS`) are labels footnote 1 defines, which no encoding names yet (`.63`).

The table itself is not lowered (`.3.g.iii.b.iii`). On `2026-10-22` an FSMGen strict probe accepted APB's decode as an
enum for the member column and one guarded rule per row writing actor storage, with 0 diagnostics. But every `.isf`
assignment is clocked, so that form adds a register and a one-cycle lag the document does not state, and its storage
target is invented, which ADR 0016 forbids. What a combination offers is a guard vocabulary: a condition naming a
member would resolve to its keys' values. No condition on the current chains needs that, since AXI's rules about a
physical address space name `AxPAS`'s own encoding, so it is built when a consumer appears. A state a combination keys
says in its residual which combination states its encoding.

### A signal's bit sub-fields

AHB's *Table 3-6 Meaning of the HPROT bits* gives each bit a row: `HPROT[1]` *Privileged*, *When asserted, this bit
indicates the transfer is a privileged access*. APB names `PPROT[0]` *Normal or Privileged* (*LOW indicates normal
access*), and AXI's *Table A12.3* maps *PARTID* onto `AxMPAM[9:1]`. No column is an enum there: each row names a part
of one signal. Since `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.i` (`2026-10-22`) such a table is an EvidenceIR
`signal_layouts` record:

- **Shape.** Exactly one column holds a slice of one declared signal or collective in every cell, with at least two
  slices that do not overlap. The name column is the first other column of distinct short cells. A second column of
  slices or binary literals is another table: a register field's bits mapped onto signal bits, or a layout per message
  kind. A column of numbers must restate each slice's width, so ADIv6's caption-less map (a register bit beside each
  `HPROT` bit) is refused and counted.
- **What is kept.** The signal, the caption, and per row the slice with its high and low bit, the name and every
  other cell as written. Layouts are never merged: AXI's two `RME_Support` layouts put `PARTID` at different bits.
- **Levels.** For a one-bit sub-field, a sentence opening *When asserted*, *When deasserted*, *LOW indicates* or *HIGH
  indicates* gives that level its meaning. A cell whose level sentences sit under a condition gives none: AHB
  `HPROT[2]` reads *If both of HPROT[4:3] are deasserted, then when this bit is: …*.
- **Manifest.** The `signal_layouts` manifest entry is recorded only for a document with a candidate table, so the
  others keep their manifests.

On the 31 chains (`2026-10-22`): AHB 1 layout (7 sub-fields, 12 levels), APB5 1 (3 sub-fields, 6 levels), AXI 4
(`AxMPAM`, 3 fields each); ADIv6's candidate is refused.

Since `.3.c.iv.b.ii` a layout crosses SemanticIR and IntentIR unchanged. No port carries a field: `.isf` port options
are a closed set, so a sub-field layout on a port is FSMGen ladder rung F29, *anticipated*. ADR 0033 §4 files no
request, because nothing executable names a sub-field. The adapter states one `isf_protocol_signal_layout_*` residual
per layout and lists every sub-field reaching past its port's elaborated width: AHB's `Lookup [4]`, `Allocate [5]` and
`Shareable [6]` lie past `HPROT_WIDTH`'s default 4.

### A table of marks per pair of keys

AXI *Table A12.11* says which tag operation each read opcode may carry under each `MTE_Support` value (`Y` or `-`).
I2C *Table 3* marks each protocol feature `M`, `O` or `n/a` per configuration, and its caption defines the codes.
AXI's *interoperability* tables say, for a manager's and a subordinate's property values, *Compatible* or *Not
compatible*, often with a reason. No column is an enum there and no row is one signal's code: each cell answers for a
pair. Since `EXTRACTION-REPRESENTATION-GAPS.3.c.v.i` (`2026-10-22`) such a table is an EvidenceIR `matrix_tables`
record:

- **Shape.** The label columns are every leading column before the first whose filled cells are mostly marks (`Y`,
  `-`, `X`, `M`, `O`, `N/A`) or modal words (*Required*, *Permitted*, *Compatible*, *May be off*), or cells opening
  with one. Two or more data columns follow. The header is the leading run of non-data rows, and seven in ten filled
  data cells must be marks or modal words. A footnote mark (`O [2]`, `Y 2`) is not part of the word.
- **What is kept.** The row key titles, each data column's header path (`MTE_Support = Basic` over `Transfer`), each
  row's keys and cells as written, the cell class (*mark* or *modal*), and the codes a caption defines (*M =
  mandatory; O = optional; n/a = not applicable*).
- **Refused, and counted.** A data column titled by a slice (a layout or a value combination), bare-number column
  titles (a bit layout), label columns of code literals (an encoding), and two or more columns titled by declared
  register fields (TMC's *Permitted register writes*, keyed by `TraceCaptEn`, `TMCReady` and `MODE`). A table another
  reader holds keeps it: presence, register, value-combination and layout tables.

On the 31 chains (`2026-10-22`): 48 matrices in 10 documents (AXI 22, USB 3.2 7, ADIv6 5, APB5 4, Cortex-A76 guide 3,
OpenCAPI 3, I2C 2, AHB 1, USB4 CM 1); TMC's three field-keyed tables are refused. Cross-tabs whose cells are values or
prose (USB 3.2 *7-14*, *8-8*) are not matrices of this kind.

Since `.3.c.v.ii` a matrix crosses SemanticIR and IntentIR unchanged, and the adapter states one
`isf_protocol_matrix_table_*` residual per matrix: nothing executable names a matrix's keys yet. A legality matrix
lowers to an always-on assertion per actor, decided by `.3.c.v.iv`: each column guarded by its property value and tag
operation, the marked opcodes decoded to their signals. It waits for three things: the opcode decode (*A7.4*, a
value combination since `.62`, whose domain labels `.63` binds), the property as an `.isf` parameter, and FSMGen's
actor-level assertion (`.3.c.v.v`). A marked cell's note narrows it (*A12.13* `Y 2`: *Not Exclusive write*), which
`.64` reads. A requirement, version or effect matrix states configuration that stays typed in IR (ADR 0033
§6).

Since `.3.c.v.iii` (`2026-10-22`) a matrix can be a legend. AXI *B2.1 Key to signals table* gives each presence code
its meaning on a manager and on a subordinate (`YM`: *Mandatory*, *Optional*), and a presence table whose every code
is one of a legend's keys reads that legend: each code gains its meanings by column, and the record names the legend.
On the 31 chains AXI's 297 *B2.2*/*B2.3* presence rows bind to *B2.1*; *A13.3*'s 9 use `C`, which *B2.1* does not
define, and keep their codes as written. APB5's and AHB's presence codes (`C`, `OC`, `OO`) have no legend matrix.

### Which table an enum was read from

An enum member reaches the IR as a synthesized statement, `Enum AxBURST INCR = 1.`, with no text span, so nothing
downstream used to know which table stated it. The accounting of captured regions therefore listed every encoding
table as reaching no record, even when its enum was published: the census over-counted uncarried intent by exactly
those tables. Each member statement now carries the table it was read from, and the symbol cites the tables of its
*published* members:

```json
{ "symbol_name": "AWATOP", "kind": "enum", "supporting_table_ids": ["table_0096", "table_0097"], "...": "..." }
```

A member dropped for a colliding value (a value that names two members) no longer vouches for its table, and an
enum whose every member collides mints no symbol and cites nothing, which is why APB's `PPROT` table stays listed.
Across the 31 current chains five mint enums from tables: their 179 member statements now name a table, and 26
captured-table residuals go (AXI 16, CHI C2C 4, AHB 3, ADIv6 3); no symbol, member, adapter or `.isf` changed. Two
of ADIv6's were cited by `In`, a junk enum that conflated a bank-select table with field widths; it no longer exists
(`EXTRACTION-REPRESENTATION-GAPS.3.e.i`, see [A signal called `Input`](evidence-failure-modes.md#a-signal-called-input)),
so those two tables are listed as uncarried again, which is what they are. Production knows which table
minted a member, so it also settles a case the measurement scripts could only call ambiguous: AXI's `BRESP` comes
from the uncaptioned `BRESP | Label | Meaning` table (`table_0045`), not the identical-looking *Table A11.3
WriteDeferrable response meanings*.

(Tracking: `EXTRACTION-REPRESENTATION-GAPS.3.e`.)
