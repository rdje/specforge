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
machinery. The honest residual is also clear: 68+ caption-less fragment chains had no labeled member at
all (their captions were lost in ingest). Many of those captions turned out to be printed on the table's own
page, only not attached to it; the reader now recovers them (see
[A bit table whose caption was left on the page](#a-bit-table-whose-caption-was-left-on-the-page)). A table
whose page prints no caption stays absent — absence, never invention.
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
forks on a mode bit, and eight caption-less chains whose captions ingest did not attach — absence, never
invention. Six AMD command and event-entry layouts (`INVALIDATE_IOMMU_PAGES`, `COMPLETE_PPR_REQUEST`, …) gain
fields from the caption their page prints once AMD's chain is rebuilt; see
[A bit table whose caption was left on the page](#a-bit-table-whose-caption-was-left-on-the-page).
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


A register's name can join two parts with `/`. ADIv6's *B2.2.3 CTRL/STAT, Control/Status register* was not taken as a
container, so its 16 field headings (`CSYSPWRUPACK, bit[31]` … `ORUNDETECT, bit[0]`) were read into the register
before it: `ABORT` held 21 fields, two of them at bit 4, and `CTRL/STAT` held none. A `/` inside the name is now
accepted, and `CTRL/STAT` gets its fields (`EXTRACTION-REPRESENTATION-GAPS.41.a`).

A heading states each field's position, but not the register's width. Until `.41.b` a register without a stated width
took the highest field's bit plus one, so `ABORT`, whose fields end at bit 4, was recorded as 5 bits wide. Now a
register records only a width the document states. On the 31 chains SpecForge can rebuild none of the 201 registers
with located fields states one, so all 201 lost the width that had been inferred for them. `.isf` still needs a width,
so the adapter emits the narrowest one that holds the located fields and lists each such register as its own choice.

RISC-V IOMMU states each register's size in its *Memory-mapped register layout* (`Offset | Name | Size`), and heads each
field table with the register's name in parentheses (`6.6. Command-queue base ( cqb )`). SpecForge now reads the two
together (`EXTRACTION-REPRESENTATION-GAPS.41.c`). The size column states no unit, so the table's offsets decide: each
register starts where the one before it ends, counted in bytes. A field table takes a heading's name only when that
heading is the nearest one before it in reading order and the layout lists the name. It takes the row's width only when
its fields end exactly there.

SourceIR records no reading order for a table. The ingest numbers every item in one sequence but writes the number
only on text, so each figure and table leaves a gap. When the gaps are exactly as many as the figures and tables, the
n-th gap is the n-th of them, and placing a table by its page is not needed. Page placement would be wrong here:
`ddtp`'s field table sits on the page of `cqb`'s heading. On IOMMU, 26 of the 33 register records are named this way,
four tables split across pages join their other halves, and 22 registers reach `.isf` under their own names at the
width the document states. Seven stay without one. Four sit under figure captions the converter took for headings, one
under a heading that names a range (`iohpmctr1-31`), and one is named from its caption. The last is `msi_cfg_tbl`,
whose row sizes a 256-byte table of entries while the fields under its heading fill one 64-bit entry.

The same reading order now names every field table whose register a heading gives
(`EXTRACTION-REPRESENTATION-GAPS.41.f`). The heading reader had taken the last heading on the table's page, and RISC-V
Debug puts several register headings on a page, so 30 of its field tables were named one register late: `dmstatus`'s
fields sat in `dmcontrol`, `abstractcs`'s in `command`, `command`'s in `confstrptr0`. Named from the heading before
them, they land in their own registers, and the `.isf` declares 35 registers where it had declared 30. The records named
`confstrptr0`, `haltsum1`, `custom0` and `dpc`, which held other registers' fields, are gone. One table stays unnamed:
the heading before it is a body sentence the converter took for a heading, and the page rule had given it the next
register's name. A heading whose parenthetical is only an address (*IDCODE (at 0x01)*) now names its register by the
word before the parenthetical, not `at`.

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

### A bit table whose caption was left on the page

The three-column reader above names a register from its table's caption. CoreSight TMC r0p1 has 58
`Bits | Name | Function` tables, and 26 of them reach SpecForge with no caption at all. The caption is still
printed on the page: the converter filed it as an ordinary text element (here a heading) and attached nothing
to the table.

```text
page 52   heading   "Table 3-3 shows the STS Register bit assignments."   <- a sentence about the table
          heading   "Table 3-3 STS Register bit assignments"               <- its caption
          table     Bits | Name | Function      caption: (none)
                    [31:6] | Reserved | Reserved
                    [5]    | MemErr   | …
page 53   heading   "Table 3-3 STS Register bit assignments (continued)"
          table     Bits | Name | Function      caption: (none)
                    [2]    | TMCReady | …
```

On the table's own page, a text in the caption grammar (`Table`, a numbered reference, a title) is a
candidate. A sentence that refers to the table is not: one that ends in a period, or whose title opens with a
plain lowercase word (*"Table 6-13 describes the …"*, *"Table A3-3 shows the AxBURST"*). A caption the converter
did attach to another table on the page claims its own copy of the text. The candidates left over pair with the
page's uncaptioned tables in reading order, and only when there are exactly as many of each; one caption more
or fewer and the page recovers nothing. A caption is read, never composed, and never borrowed from another page.

Across the 78 ingested documents this finds 1,296 captions in 54 of them. Documents number their tables in
order, which gives an independent check: 1,900 neighbouring pairs that involve a recovered caption are in
order and 6 are not. Three of the 6 are right by content (GIC-600's `GICR_PIDR2` table holds `ArchRev`; the
converter listed those tables out of page order), two are artefacts of the check itself (an appendix `A`
after part `D`, roman front-matter numbers), and one page cannot be settled from text. Only the bit-layout
reader uses recovered captions so far; the other caption readers are `EXTRACTION-GAP-FIX.12`.

Reading those tables brought four row-level rules into view, all about *where a cell sits*:

| row as the converter delivers it | read as |
| --- | --- |
| `[31:7] \| Reserved \| Reserved` | field `Reserved` `[31:7]`: two equal cells are two columns, not one repeated cell |
| `[3:0] \| CLAIMSET_W \| On writes: 0 1 \| No effect …` under a `Function` title spanning two columns | field `CLAIMSET_W`, description both sub-cells |
| `[13:11] \| WBUF_DEPTH \| 0x2 0x3 \| Depth …`, the bit and name cells repeated from the row above because they span it | a value sub-row of `WBUF_DEPTH`, not a second field |
| `Bits \| Name \| Function` as the first body row | the title row, as the register-field reader already assumes |

A title row found in the body is read for three-column tables only. A two-column one cannot be told from two
titles the converter merged (MMU-700 prints `Bits | Name Description`), and reading it would make a register
caption name a structure; that family is `EXTRACTION-GAP-FIX.14`. A table whose titles include a reset or
access column belongs to the register-field reader even when the converter merged them into one cell
(CoreSight SoC-600's `Reset Type Description`), so no table is read by both.

A register often reserves several ranges. TMC's AXICTL reserves `[31:12]` and `[6]`, and the rule that keeps
two registers from being merged into one (every field name distinct) used to refuse it, leaving AXICTL's
fields in a record apart from its summary row. Repeated `Reserved` padding now merges when every reserved range
states its bits and no two overlap; two registers collapsed to one name would reserve the same bits and are
still refused.

On TMC, registers with at least one field go from 28 to 49 of 50 and fields from 87 to 170; `STS` gets
`Reserved`, `MemErr`, `TMCReady`, `Triggered` and `Full` across its page break. The one left is CLAIMSET, whose
second field's name the converter moved into the bit column, so the table is refused whole
(`EXTRACTION-GAP-FIX.13`). Of the other 30 documents SpecForge can replay, none changes. Six it cannot replay
until they are re-ingested gain registers when they are: GIC-600 33 → 69, SDC-600 5 → 12, MMU-700 63 → 65,
GIC-400 4 → 6, the GIC architecture 90 → 91, and AMD IOMMU gains 25 command and event-entry fields named by their
recovered captions. Nothing is lost anywhere.
*Authoritative tracking:* `docs/tasks/EXTRACTION-GAP-FIX.md` (`.9`).

### A footnote letter after a name

Register tables footnote their cells, and the PDF prints the footnote letter right after the name, so the
reader receives it as part of the cell:

```text
page 75   table   Bits   | Name       | Function
                  [11:8] | ATBYTESM a | Controls the value of ATBYTESM.
          note    a. The width of the ATBYTES field depends on the configuration.
```

The field is `ATBYTESM`; the `a` points at the note. A trailing single letter is not always a note mark,
though. MMU-700 writes an array of registers as `TCU_NODE_CTRL n`, where `n` is the index, and the text layer
sometimes splits one identifier into two words (RISC-V IOMMU's `fence_w_ip` arrives as `fence_w_i p`). So the
letter is removed only when the document prints the note: a footnote that opens with that letter (`a.` or
`j `) on the table's page or a neighbouring one, because a table that runs over a page break prints its notes
on one of its pages. GIC-600 marks `GICD_ISENABLERn j` on one page and prints note `j` on the next; MMU-700
prints no note `n`, so `TCU_NODE_CTRL n` keeps its index. This applies to every register path's name cells —
a register summary, a field table, a bit-assignment table. An access cell keeps its mark (`RO,RW a`; see the
access rule under *Which register a field row belongs to*).

Across the 78 ingested documents this renames 27 fields and changes nothing else: CoreSight TMC's
`ATBYTESM` and `ATBYTESS`, ADIv6's `TRNCNT`, `MASKLANE`, `TRNMODE` and `STICKYCMP` in the chains SpecForge can
replay, and 21 GIC fields that change when those documents are next re-ingested. Four GIC architecture names
keep their letter because the converter did not keep their notes. What a note *says* — ADIv6's is that those
four fields do not exist in one configuration — does not yet reach the field (`EXTRACTION-GAP-FIX.16`).
*Authoritative tracking:* `docs/tasks/EXTRACTION-GAP-FIX.md` (`.10`).

### A `Default` column is not always a reset

A table becomes register fields when it has a name or bit column and register vocabulary: an access column
or a reset column. `Default` used to count as a reset column, and it also heads the default of a signal, a
parameter or a property. AXI IHI0022 L's signal tables read (*Table A2.9: Signals when using credited
transport*)

| Name | Width | Default | Description |
| --- | --- | --- | --- |
| `AWPENDING` | 1 | `0b1` | … |
| `AWCRDT` | `Num_RP_AWW` | All zeros | … |

and SpecForge made each of them a register (`Valid and Ready signals`, `AxSIZE signals`, 71 in all), which AXI's
`.isf` then declared as 71 storage registers. Now a `Default` column counts only beside a bit, access or
`Reset` column, or under a caption that names a register: MMU-700's `Table 3-10: MMU-700 SMMU_PMCG_CFGR register
architectural options` (`Field | Default value | …`) stays a register's fields. Across the 78 documents this
removes exactly the default-only tables — AXI's 71, eight Avalon parameter tables, four AMBA ATP parameter tables
and one AXI/ACE MPAM mapping — and one real one: MMU-700's caption-less continuation of that options table, which
the next page prints without a caption (`EXTRACTION-GAP-FIX.17`).
*Authoritative tracking:* `docs/tasks/EXTRACTION-GAP-FIX.md` (`.15`).

### Which register a field row belongs to

A table with a bit column lists *fields*, and every field needs a register. The table itself says how to
find it, in one of two ways, and the reader never looks outside the table for the answer.

Some tables name the register on the row, by its offset or in a register column. OpenCAPI's *64-bit Base
Address Register* table is keyed by offset:

| Offset | Bits | Field Name | Description | Attributes |
| --- | --- | --- | --- | --- |
| `x'00'` | 31:4 | BAR Address Low | Address low. | RW |
| | 3 | Prefetchable | … | RO |
| | 2:1 | Type | … | RO |
| | 0 | Address Space | … | RO |
| `x'04'` | 31:0 | BAR Address High | Address high. | RW |

A row whose offset (or register cell) differs from the open register's opens a new one; a blank cell
continues it. So `x'00'` is one register with four fields and `x'04'` another with one. A table that repeats
the offset on every row (the configuration header gives `x'00'` to both *Device ID* and *PCI Vendor ID*)
reads the same way. The table names no register, so each gets a placeholder name
(`register_table_0021_000`) that the eval counts as unnamed, and keeps its offset. A table that opens with
a blank cell, because it continues a table on the previous page, starts a register of its own rather than
joining one from another table.

Other tables name no register at all: RISC-V IOMMU prints one `Bits | Field | Attribute | Description`
table under each register's heading. Such a table *is* one register's fields, so it is read like any field
table: one register per table, named from its caption or its defining heading when either names it, with
the `Attribute` column read as each field's access.

Until `2026-09-26` a field row joined the last register built from **any** earlier table. RISC-V IOMMU's
18 field tables became one register named `version` (the first field of the first table) holding 98 fields,
and OpenCAPI discovery's 16 tables became one `Device ID` register holding 206. Now IOMMU has 33 registers
and 147 fields, all 147 with an access type, and OpenCAPI discovery has 97 registers keyed by offset and
207 fields. No other document changed.

The same tables often name their access column `Type` or `Attributes`, a word that also heads data-type columns.
Such a column carries access when most of its non-empty cells are access literals (`RO`, `RW`, `WO`, …), and then
each row keeps its own access-shaped cell: CoreSight TMC's `RO,RW a`, with its footnote mark, and OpenCAPI's `W` or
`VU`. A grouping row of prose (`Reserved area`), a dash or a blank states no access for its row. Until
`2026-09-26` one such cell made the reader ignore the whole column. Now TMC has an access type for 50 of its 50
registers (20 before), ADIv6 for 51 of 82 (12 before), and OpenCAPI discovery for 205 of its 207 fields (137
before). Every added value is the source cell as printed.
*Authoritative tracking:* `docs/tasks/EXTRACTION-GAP-FIX.md` (`.6`, `.7`).

### A field's value table

A register's field table gives each field a bit range and a name. The meaning of each value often comes in a
separate table after it. ADIv6 prints *Table C2-1 Summary of AddrInc field values*:

| AddrInc value | Description | Support required? |
| --- | --- | --- |
| `0b00` | Auto-increment off | Always. |
| `0b01` | Increment single | Always. |
| `0b10` | Increment packed | If Packed transfers are supported. … |
| `0b11` | Reserved | - |

Until `2026-09-29` no record read such a table: `CSW.AddrInc` had its bits `[5:4]` and no values, and the table
stayed a captured region. Now the table fills the field's values when three things hold:

- The header's first cell names exactly one field of one register: `AddrInc value`, `Value of DbgSwEnable`,
  `Size value, CSW.Size` (the register given by name), RISC-V Debug's `cause encoding`.
- The first column holds the values: register literals (`0b00`, `0x1`), or plain decimals when no cell in that
  column is two or more `0`s and `1`s (`10` could be binary).
- The second column holds the meaning. SpecForge keeps only its first sentence (`Enabled`, not *This value is the
  value after a reset.*), and only when that sentence is at most 80 characters.

A field needs at least two values from the table, and the table is cited by the register it fills. A table without a
header row keeps it as its first body row, so RISC-V Debug's *Table 3. Meaning of cmdtype* (`cmdtype | Command`) fills
`command.cmdtype`. On the 31 chains this fills IOMMU's `eventID`, RISC-V Debug's `cmdtype` and ADIv6's `IDR.TYPE`, and
the IOMMU and ADIv6 `.isf` carry them as field enums.

The reader refuses these shapes:

- **A table with a second value column.** *Table C2-2* heads `Value of CSW.ERRNPASS | Value of CSW.ERRSTOP |
  Functionality`: each row is a pair of values, not one field's value. Only the meaning column decides this. A code
  column after the meaning is an attribute of each value, so IOMMU's `eventID | Event counted | IDT settings
  supported` fills `eventID` (`EXTRACTION-REPRESENTATION-GAPS.3.b.iii.a`).
- **A table that repeats a value.** A field's values are each stated once; IOMMU's `DMASK | DID_GSCID | Comment` is a
  programming example of masks, not `DMASK`'s values.
- **A header that names no field.** `Value` alone does not, and neither does `4-bit IR instruction value`, because
  no register has a field called `4-bit IR instruction`.
- **A range cell.** `0b110 - 0b111 | Reserved` is two values, and the reader leaves it out rather than choose one.
- **A second table for the same field.** Once a field has values, a later table does not replace them. ADIv6's
  *Table C2-6* maps `DbgSwEnable` to the `DBGSWENABLE` signal's levels (`LOW`, `HIGH`), and *Table C2-5* has
  already given that field its meaning (`Disabled`, `Enabled`).

A footnote mark after a value (`0b011 b`) is removed as it is for a name (see *A footnote letter after a name*).

Of the 31 chains SpecForge can replay, two change, and nothing else does. RISC-V Debug's `dcsr.cause` gains its
six causes from *Table 8* (`1` ebreak, `2` trigger, `3` haltreq, `4` step, `5` resethaltreq, `6` halt group).
ADIv6's `CSW` gains `DbgSwEnable`, `Size` (`0b000` Byte up to `0b101` 256-bits) and `AddrInc`. ADIv6's `.isf` now
enumerates those three fields, e.g. `(field addrinc (bits 5 4) (enum (auto_increment_off 0) …))`.

`dcsr.cause` still has no bit range, because RISC-V Debug draws its field positions in a figure (gap S23). Its
values therefore stop at IntentIR's register records and do not reach `.isf`.
*Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.c.iii`).

### A field bound to a wire

Some register fields are the programmer's end of a wire. ADIv6 writes it under each field's heading:

```text
CSYSPWRUPREQ, bit[30]    System powerup request. This bit controls the CSYSPWRUPREQ signal.
CSYSPWRUPACK, bit[31]    System powerup acknowledge. Indicates the status of the CSYSPWRUPACK signal.
SRST_OUT, bit[0]         This field drives the nSRSTOUT signal for the port or ports that are currently selected …
```

Each field now records the wires it is bound to, and which way. A field `drives` what it *drives* or *controls*. It
`reflects` what it *indicates the status of* or *returns* (*the logical AND of the PORTCONNECTED signals*). The
sentence is read under the field's own heading, into the one field with that name and bit range, and never into a
field two registers share. Before this, the relation reader had taken `field` and `bit` in these sentences for
actors (`KG-ISF-COMPLETENESS.1g`). Across the 31 chains SpecForge can rebuild only ADIv6 states such bindings: 10 of
them, 6 on `CTRL/STAT` and 4 on the JTAG Access Port's `CSW`. Nothing downstream reads them yet. They are carried to
IntentIR's register records, and `.isf` does not change (`EXTRACTION-REPRESENTATION-GAPS.41`).

A table can bind a field bit by bit. ADIv6's implementation appendix maps each bit of `CSW.Prot` to a bit of the
read and the write channel's signal:

```text
Table E1-1 CSW.Prot mapping to ARPROT or AWPROT
Bit | ARPROT signal | AWPROT signal | Description
30  | ARPROT[2]     | AWPROT[2]     | Instruction
```

Since `.41.e` such a table is read when its caption names one `REGISTER.FIELD` followed by *mapping to* (or *maps
to*, *mapped to*), its header has one `Bit` column and `<SIGNAL> signal` columns, and exactly one register named
`REGISTER` (or `REGISTER@…`) has that field with every listed bit in its range. Each `ARPROT[2]` under its own column
is a `drives` binding with the register bit and the signal bit (`field_bit` 30, `signal_bit` 2) and the table it came
from; a table the appendix repeats for another bus variant adds its citation to the same binding. ADIv6's
`CSW@MEM-AP.Prot` gains 14 bindings from Tables E1-1 to E1-4, each citing two tables; nothing else on the 31 chains
moves, and `.isf` does not change. The AHB appendix's HPROT map lost its caption in conversion, so it is not read yet
(`.41.g`).
*Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.41`, `.41.a`, `.41.e`).
