# Register And Bit-Layout Table Shapes

A register map is rarely one table shape. The same specification will describe its registers as a
four-column bit table on one page, a two-column layout grid on another, a run of headings with no
table at all on a third — and a *structure* that looks identical to a register but is not one.
Getting each of those right is what separates a recovered register map from a plausible-looking
invention, so each shape below is a separate reader concern with its own recognition rule, its own
refusals, and the measurement that settled it.

These sections were part of [EvidenceIR](evidenceir.md) until that chapter outgrew its size bound;
they are the same content, reorganized by reader concern rather than rewritten.

## Table shapes, one at a time

### `PDF-VARIANT-DIGESTION.10a` — a register table whose field names hide inside the description

A corpus-wide census of every table the ingest classifier left "unknown" pointed at one family as the
single biggest untapped register source: tables headed `Bit Location | Register Description | Attributes`
(the CCIX base specifications carry ~150 of them *each*, across four tracked versions — about 600 tables).
The rows clearly describe register fields — `15:0`, *"CCID This field indicates the CCIX Consortium ID
value…"*, `RO` — but the reader recovered almost nothing from them (8 registers and 11 fields per
document), for two reasons. First, the header never says `Field` or `Name`, so the table didn't look like
a register-field table at all. Second, and more interesting: these tables have **no name column**. The
field's name is fused into the description cell as its first word, the way a dictionary entry starts with
the word being defined.

The first half of the fix is vocabulary: `Bit Location` joins `Bits` / `Bit Range` / `Position` as
bit-position column titles, and a column titled `Register Description` or `Field Description` is treated
as a *description*, never as the name column. The second half is reading the dictionary-entry shape
honestly. A field name is accepted from the front of a description only when it actually looks like an
identifier (`DVSECRevID`, `ID0_20_2F`) rather than an English word (*"See Table 7-1…"*, *"Indicates
the…"* stay residuals), and four bleed shapes measured on the real corpus are filtered out: a leading
token that merely repeats the row's own access value (`RO Reserved bit…`), a remainder that is really a
*Reserved* definition, a token that leads several rows of the same table (a real mnemonic is unique; a
repeated one is prose), and a wrapped cell whose *real* definition starts mid-text with its own
*"`SomeName` This field…"* marker — there the bleed prefix must not become the name. Specs that write the
defined term in parentheses — *"Log Length (LogLen) This field describes…"* — get their own reading, and
the caption's *"…at Byte Offset 04h"* locator finally lands in the register's offset instead of being
ignored (a *"from … through …"* range is never collapsed to a guessed point).

Every gate was measured per-item over the persisted corpus before any code changed, and the live result
matches the measurement exactly: the CCIX 2.0 document goes from **8 registers / 11 fields to
143 registers / 389 fields**, 259 of them with real names and 130 kept as honest bit-range residuals
(`Reserved` rows, cross-references), with 40 registers carrying recovered byte offsets. Two CoreSight
manuals upgrade exactly 13 fields each from bit-range placeholders to their real names (`ATDATA127`,
`AFVALID`, the `ID0_…` filter bits). Everything else is untouched — all 12 rebuildable corpus documents
(NVMe, RISC-V Debug, the AMBA buses, SWD, I²C, SMBus, I²S, CAN, SWP) rebuild **byte-identical**, and the
known residual is quantified rather than hidden: exactly one row corpus-wide keeps a wrong name (a
page-wrap bleed indistinguishable from a real row without page-level context), and the ~60
`Byte Location | Size | Register Description` tables were refused outright — forcing their byte offsets
into bit ranges would fabricate. That refusal was right twice over: when the follow-up leaf probed those
tables per-item, their captions turned out to describe in-memory *structures*, not registers at all, and
they are now read honestly into the message-field inventory (see the `.10e` section below).
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.10a`).

### `PDF-VARIANT-DIGESTION.10b` — two-column bit-layout tables describe structures, not registers

The second family the corpus census isolated looks deceptively like a register table: just two columns,
`Bits | Description`, one bit range per row — 334 such tables across six documents, with the AMD IOMMU
specification carrying 163 and the NVMe base specification 156. But reading the rows shows they are not
registers at all. They describe **in-memory structures**: AMD's 256-bit Device Table Entry, NVMe's command
dwords and completion-queue entries, page-table and interrupt-table entries. Nothing in the family carries
the access/reset vocabulary that makes something a register — and pretending otherwise would *fabricate*
MMIO semantics the documents never wrote. So these rows go where structured content already lives: the
message-field inventory (`message_field_records`), as a second reading strategy beside the field-titled one
below. Each recovered field carries its container, its name, and — new with this family — its literal bit
position (`Command Identifier (CID)` at bits `31:16` of `Command Dword 0`; `vImuEn` at bit `207` of the
`Device Table Entry (DTE)`).

Three measured gates make the reading honest. First, a **strict cell parser**: a bit cell is accepted only
when it is *purely* a bit position (`255:248`, `247`, `[7:4]`) — AMD's dword-relative cells (`31:28 +04`)
and NVMe's symbolic ones (`31 + (Element Length*8):32`) reject the *whole* table, because capturing the
range while dropping the offset would misrepresent where the field actually sits. Second, the **fragment
chain**: AMD splits its Device Table Entry across nine page fragments, most of them caption-less — and the
chain head itself has no caption. A caption-less fragment is adopted only on *bit-exact adjacency* (its
first row continues exactly where the previous fragment stopped, `207` after a fragment ending at `208`)
within one page, and a chain takes its container name from its captioned members. Measured over the corpus
this is unambiguous: 23 caption-less fragments continue exactly, 80 fresh structures restart at a width
boundary and adopt nothing — there is no gray zone in between. Third, field names reuse the same
dictionary-entry grammar the register reader gained in `.10a` (`Full Name (CID): …` and the gated leading
identifier `vImuEn: virtualize IOMMU enabled…`), so `Reserved` padding and prose-led rows yield nothing
rather than a wrong name.

Live, NVMe gains **216 typed fields across 113 containers** and AMD **82 across 15** (the stitched DTE
alone carries 31), while the four smaller documents in the family (USB 3.2, USB4 inter-domain, a CoreSight
TMC manual, eMMC) yield exactly **zero** — their matching tables are value-encoding tables whose rows
recover no field names, which is the correct honest outcome. Everything else is provably untouched: all 12
rebuildable corpus documents rebuild with every extraction surface **byte-identical** (the only difference
anywhere is the run manifest recording that the new strategy exists), and NVMe's measured register gold
re-measures exactly its documented state. The known residuals are quantified, not hidden: the 41
offset-suffixed AMD tables awaited an offset-aware reading (delivered by `.10d` below), caption-less chains
with no captioned member stay unread, and one true name (`SnoopAttribute`) is sacrificed to the mid-cell
bleed guard because its own description later says *"…guest PTE. This field is meaningful…"* — the gate
that protects every wrapped cell from a false name rejects this one true one, a trade the per-item audit
makes explicitly.
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.10b`).

### `PDF-VARIANT-DIGESTION.10c` — three-column bit tables: the caption decides register versus structure

The third family from the corpus census is the classic ARM-TRM table: `Bits | Name | Function` (or
`Bits | Field | Description`), three columns and — again — no access/reset vocabulary anywhere. 270 such
tables across eleven documents, and unlike the two-column family they split *by meaning*: most describe
**registers** (`Table 4-21: TCU_CTRL register bit descriptions`, `Table 4-3 GICD_CTLR bit assignments`),
but some describe **in-memory structures** (Intel VT-d's `Root-Entry Format`, the GIC architecture's
`LPI Configuration table entry bit assignments`). The reader decides per table, from the document's own
caption vocabulary. A caption grounds a register when an identifier-shaped name sits right before the word
*register*, or alone before the locution *bit assignments* — including the array conventions
`GICD_CHIPR<n>` and the space-written `TCU_NODE_CTRL n`. A plain English word there does not count:
NVMe's *"Reservation Register"* is the name of a *command* (the act of registering a reservation), and
`Reservation` being an ordinary Titlecase word is exactly what keeps it from becoming a phantom register.
A multi-word head like *"LPI Configuration table entry"* is structure, not register, evidence — those
fields join the structure inventory of `.10b`, where VT-d's entry formats also land (their fused name
cells, `CTP: Context-table Pointer`, yield the mnemonic).

Register-captioned chains become real registers: one caption-named `RegisterRecord` per chain, fields
from the name column with their bit extents, access and reset left honestly empty (the tables never state
them — the completeness gauge reports the absence instead of the reader inventing it). Page fragments
reuse the `.10b` chain rule — bit-exact adjacency only. The tempting alternative, adopting a caption-less
fragment because it sits under the same section heading, was **rejected by measurement**: headings are
only page-granular in the ingest data, and the probe showed same-heading adoption would rescue exactly one
table corpus-wide while wrongly merging fifteen whose bit ranges overlap. Two more caption gates earned
their place the same way: a "label" that ends in a sentence period is caption bleed (two of 314 family
captions, both prose like *"shows the bit assignments."*), and *"Continued from previous page"* is not a
label at all — freeing such fragments to chain into their true home is precisely how one chip-to-chip
property register collected its full 22 fields. A register-*worded* caption that grounds no identifier
stays an honest residual on both surfaces, rather than being re-housed as a fake structure.

Live, the TRM class finally opens up: the GIC-600 manual goes from 15 to 33 registers (gaining
`GICD_CTLR`, `GICD_TYPER`, the `GICD_CHIPR<n>` array…), the MMU-700 from 13 to 63 (+180 fields), the
CoreSight TMC from 2 to 30, the SDC-600 from 0 to 5, and VT-d gains 15 typed structure fields — while
all 12 rebuildable corpus documents keep every extraction surface byte-identical (the run manifest alone
records the new strategy) and the NVMe/AMD results of `.10b` re-verify unchanged through the shared
machinery. The honest residual is also clear: 68+ caption-less fragment chains have no labeled member at
all (their captions were lost in ingest), so the registers they describe stay absent until a better ingest
recovers the captions — absence, never invention.
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.10c`).

### `PDF-VARIANT-DIGESTION.10d` — dword-relative bit cells: keep the offset, never derive the position

Some structure tables don't number their bits from the top of the structure. AMD's IOMMU commands and
event-log entries are 16-byte records documented one **dword at a time**: a bit cell reads `31:28 +04`,
meaning bits 31:28 *of the dword at byte offset 4*. The `.10b` reader deliberately rejected whole tables
containing such cells — capturing `31:28` while silently dropping the `+04` would have placed the field in
the wrong dword — so 43 tables (309 rows) sat as honest residuals. This slice reads them, and the first
question it had to answer was *what an honest capture even looks like*.

The tempting move is to derive an absolute position: bit 28 of the dword at byte 4 "is" bit 60 of the
entry. The corpus said no. The probe cross-checked every row whose description carries its own bracket
notation (`Store Data[63:32]`, `DomainID[15:0]`) and found those brackets are **value slices, not
positions** — 54 of 66 disagree with the would-be derived position. Deriving and storing absolute bits
would have made the IR silently contradict the document's own notation on most rows. So each field keeps
the document's literal statement, in two parts: the dword-relative `bit_range` *and* a new `byte_offset`
field. Anyone downstream who needs an absolute ordering can compute `byte_offset × 8 + bit` — the IR
itself never asserts more than the page does. The same literalism handles a quirk found mid-table: one row
lost its `+04` suffix in ingest, and that field records its bit range with `byte_offset` honestly *absent*
rather than inferred from its neighbors.

The cell grammar is strict the same way `.10b`'s is: digits, an optional colon, whitespace, `+`, decimal
digits — nothing else. That single shape decision keeps every other `+` cell in the corpus rejected
(GIC-600's register-count formulas like `4 + (ITSnum × 2)`, NVMe's variable-length `15+HL:16`, USB's
`9+N`), measured per-item before the parser was written. Page-fragment chains extend naturally: a
continuation now matches when its first row is the *(offset ascending, bit descending)* successor of the
previous fragment's last row — either the next bit down in the same dword, or bit 31 of the next dword
after the previous one closed at bit 0 — and the two position conventions never chain into each other.
Two name forms, also probe-measured against the whole corpus, unlock the rows themselves: the **verbatim
bracket-slice name** (`DeviceID[15:0] .` — kept with its slice, so `Address[31:0]` and `Address[63:32]`
stay the two distinct statements the document made; the bracket-plus-boundary frame is strong enough to
admit plain English heads like `Vector` and `Destination` that the bare leading-identifier form rightly
rejects) and the **framed single letter** (`f: flush queue`, `U . The U bit…` — a lone letter is a name
only inside a colon or dot frame, once per table). A caption marker fused by lost spacing
(`… Fields(Continued)`) — caught live in the per-item audit — is now stripped by both caption readers, so
the fragment it labels merges into its true family.

Live, the AMD-class document goes from **82 to 217 message fields across 30 containers** — `COMPLETION_WAIT`
gains its `f`/`i`/`s` control bits at `+00`, `IO_PAGE_FAULT` its full 16-field entry through
`Address[63:32]` at `31:0 +12`, and the bracket-slice form also names 21 previously-residual rows on
ordinary `.10b` tables (`GDeviceID[15:0]`, the page-table `A`/`D`/`G`/`U` bits) — with **zero**
pre-existing records changed and every other extraction surface byte-identical. All 12 rebuildable corpus
documents rebuild **fully byte-identical** (no new strategy was registered; the family rides the existing
bit-position reader with a wider literal grammar). What stays out is documented: opcode value rows
(`01h . COMPLETION_WAIT command number.` states a value, not a field name), two-word heads
(`Store Address[31:3]`), one malformed-cell table, two conditional-layout tables whose description column
forks on a mode bit, and eight caption-less chains whose captions ingest lost — absence, never invention.
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.10d`).

### `PDF-VARIANT-DIGESTION.10e` — byte-location tables: the probe overturned the working hypothesis

This slice is a small case study in why SpecForge probes before it builds. When the `.10a` register work
refused the `Byte Location | Size (Bytes) | Register Description` tables (turning byte offsets into bit
ranges would fabricate), the working hypothesis it recorded was "these are register-*placement* maps —
a future leaf should turn each row into a register at an offset." The future leaf arrived, probed all 60
tables per-item across the four CCIX-class document versions, and found the hypothesis was wrong: **not
one of the 60 captions says "register."** Every caption names an in-memory record — *"CCIX PER Memory
Error Type Structure"*, *"Cache Error Type Structure"*, *"Vendor-Specific Log Info"* — and the rows are
the byte-granular fields of those structures (an error log's FRU ID at byte 4, its `Length` at byte 6),
complete with mandatory/optional record vocabulary. Minting MMIO registers from them would have invented
hardware that the document never describes. So the typed home follows the same caption-decides rule as
the `.10c` work: these fields land in the **message-field inventory**, beside the other "structured
content, not wires" layouts.

The capture stays literal. Each field records the byte offset exactly as stated (`byte_offset: 4`) and
its width converted exactly from the stated size (`Size (Bytes)` `4` → 32 bits — unit arithmetic, the
same class as computing a width from a bit range; a symbolic size like *"(indicated by VenLen)"* is a
variable-length tail and keeps its width honestly absent). No bit positions are stated, so none are
derived — `bit_range` stays empty, and the byte-offset reading is unambiguous precisely because of that.

The field names live fused at the front of each description cell, but unlike the `.10a` families these
are the document's own **multi-word English names** — *Validation Bits*, *Operation Type*, *FRU ID* —
exactly the shape the shared identifier grammar rightly rejects elsewhere (and would truncate here:
`FRU ID` would become `FRU`). The family therefore gets its own measured grammar, leak-proof because the
header gate admits only these tables: the name is the text before the universal definitional frame
(*"… This field indicates …"*), a head ending in the document's own parenthesized mnemonic prefers it
(*"Card or Channel Number (Chan) This field…"* → `Chan` — trusted even past wrapped-cell bleed, like the
shared mid-cell form), and a bare short cell whose description page-wrapped away is itself the name. A
head containing a sentence period is wrapped-cell bleed and is refused — the two corpus rows shaped that
way stay honest residuals rather than receiving stitched-together names.

Page fragments chain by **byte-exact adjacency**: a caption-less fragment continues a structure only when
its first byte offset is exactly the previous fragment's last offset plus that field's size — measured 31
of 31 true continuations, zero ambiguous joins, with fresh structures always restarting at byte 0 and
variable-length tails closing their chain to further adoption. Live, the four CCIX-class versions gain
**35–45 byte-location fields across 4–6 structure containers each** (≈161 fields total) with zero
pre-existing records changed, and all 12 rebuildable corpus documents keep every extraction surface
byte-identical (the run manifest alone records the new strategy). The residual ledger is explicit: the
Port error structure lost its caption in ingest in all four versions (its chain extracts nothing until a
re-ingest recovers the caption), two more structures lost captions in individual versions, and the two
period-bleed rows above stay name-less — absence, never invention.
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.10e`).

### `PDF-VARIANT-DIGESTION.10f` — a message field written as a heading, not a table row

Every `.10` family above reads a *table*. Some specifications don't put their field layouts in tables at
all. A message protocol like AMBA DTI describes each message — `DTI_TBU_TRANS_REQ`, `DTI_TBU_CONDIS_REQ` —
as a numbered section, and inside it gives each field **its own little sub-heading**: a line that reads
`STAGES, bits [27:26]`, then `SPD, bit [25]`, then `M_MSG_TYPE, bits [3:0]`, each followed by a paragraph
of prose. There is no grid for the table readers to find, so before this slice DTI produced **zero**
message fields, and obligations about those fields (*"the MMUV field must be 0"*) had nowhere to live —
they leaked sideways into the signal-constraint surface as undeclared, dotted subjects.

The scope note that opened this work guessed the fields lived in bullet-list prose. Measuring the real
document corrected that: they are **section headings**, which is good news, because a heading is a far
cleaner anchor than a sentence. The reader walks the document's section list in order, and a field is
recognised only when a heading matches `<NAME>, bit [N]` / `<NAME>, bits [hi:lo]` exactly end-to-end — so
a descriptive paragraph that merely *mentions* "bits [3:0]" (it's body text, not a heading) and a
cross-reference like *"TOK_TRANS_REQ[7:0] is bits [19:12]."* never qualify. Enum-value sub-headings
(`STATE = 0`) and unnamed reserved ranges (`Bits [27:25]`, with no comma and no mnemonic) fall outside the
shape and are left alone.

The interesting part is **where each field belongs**, because the *same* heading shape appears in register
manuals too — GIC, SMMU, CoreSight all describe register fields exactly this way. The rule is the one the
`.10b`/`.10c`/`.10e` work already established: a layout is a register only when it carries
register-attribute vocabulary. So the container decides. A field's container is the nearest numbered
heading above it; if that heading's caption says "register", or the section carries an `Attributes` or
`Accessing …` sub-heading (a register's access-attributes block), the fields are register fields and this
slice leaves them for a future register-routing leaf. DTI's message sections carry none of that (they have
`Source`, `Usage constraints`, `Flow control result` instead), so their fields route to the **message-field
inventory** — and across the whole corpus this reader fires on exactly one document, DTI, recovering
**159 message fields across 17 message containers**. The register-shaped documents (GIC's 612 headings,
SMMU's 434, and the rest) correctly yield **zero** message fields here.

Two cleanups came straight out of reading the real output. The backend sometimes tokenizes the same field
two ways — `TRANSLATION_ID [11:8]` and `TRANSLATION_ID[11:8]` — so a stray space before a value-slice
bracket is normalised away and the two merge into one record. And a heading whose "name" is literally the
unit word *Bits* (`Bits, bit [5:4]`) is an unnamed reserved range, not a mnemonic, so it is dropped rather
than recorded as a field called `Bits` — absence over invention, the same instinct as everywhere else.
Overlapping bit ranges, by contrast, are *kept*: DTI documents the Manager-side and Subordinate-side view
of the same position under different names (`M_MSG_TYPE[3:0]` and `S_MSG_TYPE[3:0]`), and both are real.

Because the reader only ever adds to the message-field surface, an old-vs-new comparison across the wire
gold specs and the table-derived message-field documents (NVMe's 216 fields, AMD-IOMMU's 217) is
byte-identical except for the one new line the run manifest records — nothing else moves. Closing the
recognition gap also lets the downstream LLM constraint reader put a DTI field obligation where it belongs:
once `MMUV` is a known field, *"the MMUV field must be 0"* is typed into the message-field constraint
surface instead of masquerading as a signal constraint.
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.10f`).

### `PDF-VARIANT-DIGESTION.10g` — the same heading shape, for registers this time

The `.10f` reader deliberately left one thing on the table. The very same `<NAME>, bits [hi:lo]`
heading shape that DTI uses for *message* fields is how ARM's **architecture specifications** lay out
*register* fields — GIC, the SMMU, CoreSight, the Advanced Communications Channel, ARM Debug v6 all
give each register a numbered heading (`B2.2.1 ABORT, Abort register`, `6.3.1 SMMU_IDR0`), a
`Field descriptions` anchor, and one sub-heading per field (`ORUNERRCLR, bit[4]`,
`TERM_MODEL, bit [26]`). `.10f` recognised those containers as registers and routed them away; `.10g`
finishes the job by reading them into the **register inventory** instead of dropping them. The two
readers share a single walk of the document's headings, so a container is classified as a register or
a message in exactly one place and the two surfaces can never disagree.

The genuinely hard part turned out to be a naming problem the documents create themselves. A short
register mnemonic gets *reused* across a chip's access-port blocks: ARM Debug describes an
`AUTHSTATUS`, a `CSW`, an `IDR`, a `CLAIMSET` for several different ports, and the section heading
carries only the short name, never the block. Worse, those repeats are a mix — sometimes the very
same register cross-referenced, sometimes a subset view, and sometimes *genuinely different
registers* (the MEM-AP `CSW` and the JTAG-AP `CSW` have completely different fields). Emitting all of
them would either count one register several times or, if we tried to stitch them, fuse two different
registers into one that the document never describes. Neither is honest. So `.10g` emits a register
only when its name is **unique within the document**; a reused mnemonic is held back as an honest
residual, to be recovered later once we can attach the block it belongs to. Unique names that match a
register the document already named elsewhere (but without a field layout) simply *fill in* that
register's fields rather than creating a duplicate.

Across the corpus this reader fires on exactly those five architecture specs and recovers **180
registers with 934 fields** (GIC 73, the SMMU 88, CoreSight 5, the ACC 2, ARM Debug 12), each field
carrying its bit range while access, reset and offset stay honestly empty — a heading states a
field's name and position, nothing more. Everything else fires zero: DTI's message containers stay
message fields, and the register-gold documents (NVMe, CCIX, RISC-V Debug) and the wire-protocol
specs carry no heading-shaped fields at all, so their output is byte-for-byte identical except for the
one line the run manifest adds to record that the new reader ran.
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.10g`).

### `PDF-VARIANT-DIGESTION.10h` — recovering the reused mnemonics by reading their field lists

The residual `.10g` left behind — a register name reused across several access-port blocks — is not
hopeless; it just needs a way to tell *"the same register written twice"* apart from *"two different
registers that happen to share a short name."* The honest signal turns out to be the **field list
itself**. If you look at how a chip's documentation actually repeats a register, three patterns
appear. Sometimes the two copies are identical — a plain cross-reference (`IDR` shown under two
ports with the same fields). Sometimes one copy lists a few more fields than the other — the same
register, drawn more completely in one chapter and more sparsely in another (`AUTHSTATUS`, whose
field list grows from two entries to five as you move between chapters). And sometimes the two
copies share *no* fields at all — the MEM-AP `CSW` and the JTAG-AP `CSW`, which are genuinely
different registers that merely reused a three-letter name.

`.10h` reads that signal directly. When every copy of a reused name is contained inside one fullest
copy — identical, or a neat subset of it — they are clearly views of a single register, so the
reader keeps the **fullest copy** (a real layout the document actually printed, never an invented
blend) and emits it once. When the copies don't nest — disjoint or partially-overlapping field
lists — that is the tell-tale of two different registers, and the reader leaves them as an honest
residual rather than risk fusing them. (We measured why a fancier scheme wasn't possible: the PDF
backend flattens every heading to the same level, so there is no "MEM-AP" block heading sitting
above the register to qualify it with — only the field lists are reliable.)

The payoff is pure recall with no risk: ARM Debug gains three registers (`AUTHSTATUS`, `DEVARCH`,
`IDR`, taking it from 12 to 15 heading-shaped registers / 57 to 69 fields) and CoreSight gains one
(`AUTHSTATUS`, 5 to 6 / 24 to 29), while the genuinely-different `CSW` and the mixed `CLAIMSET` stay
residual. Crucially, *nothing already extracted changes*: a side-by-side rebuild of every register,
wire and message-field gold document is byte-for-byte identical, and the only two documents that
move gain records without losing or altering a single existing one.
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.10h`).

### `PDF-VARIANT-DIGESTION.10i` — naming the genuinely-different registers by the block they live in

`.10h` recovered the registers that were really *one* register written twice, but it deliberately
gave up on the opposite case: the MEM-AP `CSW` and the JTAG-AP `CSW`, which are genuinely different
registers that happen to share a three-letter name. Dropping both is safe but lossy — a faithful
model should know that *each* access port has its own `CSW`. The `.10h` write-up worried that the PDF
backend flattens every heading to one level, so there was no obvious place to find a block name to
tell the two `CSW`s apart.

`.10i` is the measurement that proved that worry too pessimistic. The headings lose their *nesting*,
but the document still prints the block name in plain sight: each register description sits under a
parent section whose title is literally *"C2.6 MEM-AP register descriptions"* or *"C3.5 JTAG-AP
register descriptions"*. Even though that parent no longer sits *above* the register as a heading
level, it is still there as its own line, and its **dotted number** (`C2.6` is the parent of
`C2.6.7`) leads straight back to it. So the reader builds a small map from every dotted number to its
heading text, walks from a register up to its parent number, and reads the block name out of that
parent's title — keeping only the single clean word in front of *"register descriptions"* (`MEM-AP`,
`JTAG-AP`, `AP`). A parent that names no block — a bare *"D4.5 Register descriptions"* — honestly
yields nothing, and its register stays a residual rather than being guessed at.

With a block name in hand, the genuinely-different copies are no longer ambiguous: each is emitted
under a qualified name, `CSW@MEM-AP` and `CSW@JTAG-AP`, so both real registers survive without ever
being fused. (The same move recovers the three per-port copies of `CLAIMSET`; the fourth, under the
block-less `D4.5`, stays an honest residual.) Downstream, when these registers are lowered to `.isf`,
the `@` and `-` are sanitised into ordinary identifier characters (`csw_mem_ap`, `csw_jtag_ap`) that
stay distinct, and FSMGen accepts the result with zero strict-mode complaints.

The win is, once again, pure recall with no collateral change. Exactly one document in the corpus has
genuinely-different reused registers — ARM Debug — and it gains five records (its heading-shaped
register count rises 15 → 20, its fields 69 → 93). Every other document, including the CoreSight spec
whose only reused name was the *collapsible* `AUTHSTATUS`, rebuilds byte-for-byte identically, and
ARM Debug itself gains those five records without altering a single one it already had.
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.10i`).
