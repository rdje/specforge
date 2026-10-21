# Presence, Defaults, Assumptions And Software Intent

A specification states more than its wires and their timing. It says which signals exist in which configuration
and what an absent one is taken to be, which signals a check signal covers, what one thing is treated as, what a
subject may do, what is optional, what the hardware assumes of its environment, and what software does to the
hardware. Each entry below is how EvidenceIR came to read one of those statements, what it refuses, and the
measurement that settled it.

These sections were part of [How Each EvidenceIR Rule Was Found And Verified](evidence-rule-chronology.md) until
that chapter outgrew its size bound; they are the same content, reorganized by reader concern rather than rewritten.

### `PDF-VARIANT-DIGESTION.12b` — presence matrices: which signals exist, in which variant, under what condition

Bus specifications routinely answer a question no other table answers: *does this signal exist at all in your
configuration?* They answer it with a **presence matrix** — a table like *"Summary of signal presence for each
interface class"* whose rows are signals and whose columns are protocol variants (interface classes, spec
versions, or the two agent sides), with short letter codes in the cells: `AWSUBSYSID` is `O` in AXI5 but `N`
in ACE5-LiteACP, and it only exists at all when `SUBSYSID_WIDTH > 0`. That is *configuration intent*: it tells
an RTL or VIP author which ports their flavor of the interface actually has. Until now no typed surface
carried it — the matrices were either invisible (their rows are usually header-trapped, the `.12a` story) or
merely "covered" as redundant restatements.

The EvidenceIR now carries a **`signal_presence_records`** surface: one record per matrix row, holding the
signal name exactly as written (a generic-name row like `AxVALID` stays generic — the document's own
convention, never expanded), the literal presence-condition expression from the matrix's `Presence`/`Property`
column (`SUBSYSID_WIDTH > 0`, `Check_Type`; a `-` cell is honest absence), and one `(variant label, code)`
entry per variant column. The codes are **kept as literal strings and never interpreted** — every document
defines its own legend in nearby prose (`O` = optional here, `OC` = optional-conditional there), so assigning
meaning in code would fabricate semantics. The capture survives the messy ways real PDFs render these tables,
each rule measured per-item on the discovering corpus before any code: cyclically *rotated* bodies (the signal
name lands in the last column while the header says first — every column remaps by the same offset), genuine
*sub-header* rows (the LTI version axis `A A.b | B | C | D` stays literal), and *fused pair* columns (a header
fused to `ACE5-Lite ACE5-LiteACP` over cells fused to `OC N` splits pairwise — but only when **every** row
carries exactly matching code counts). Anything less trustworthy refuses rather than guesses: a fused `Y Y`
cell refuses its whole row, and a table whose row labels spill across two columns with no consistent rotation
refuses entirely — a misattributed code would state that a port exists in a variant where it does not.

Measured over the rebuildable corpus, the surface fires on exactly the four matrix-bearing documents — **AXI
306 rows (157 of them carrying 73 distinct presence-condition expressions), APB 20, AHB 40, AXI-Stream 22** —
and every other document reads an honest zero with its extraction byte-identical to before. The completeness
accounting now treats a fully captured matrix as *explained* (it produced its typed records), which closed the
AXI generic-signal family (`A13.3`) and both LTI presence fragments without any rebuild, while the garbled
fragments (an ACE page whose header fused one column pair but whose cells fused a different one) stay honestly
flagged as candidate misses. Two tracked benchmark fixtures lock both directions — literal capture including
the pairwise split, and the refusals — plus the load-bearing honesty property that presence records **never
mint signals**: existence-in-a-variant is configuration intent about a name, not a wire declaration (the
`.12a` gap-fill owns declaration content, with its own width/direction evidence rules).
*Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.12b`).

**A data row a converter flagged as a header is still data.** APB5's check-signal matrix (*Table B-3*, header
`Signal | Width | Property | APB5 | APB4 | APB3 | APB2`) arrives with every row among the header rows, and three of
them with their second cell flagged a header (`PSLVERRCHK | 1 | Check_Type | OC | N N | | N`). Read as genuine header
rows, they gave the matrix its codes as variant labels: `PWDATACHK`, `PSTRBCHK`, `PREADYCHK` and `PBUSERCHK` held
`OC`/`N`/`N`/`N` where the document says `APB5`/`APB4`/`APB3`/`APB2`, and `PAUSERCHK` and `PWUSERCHK` held no row.
Since `EXTRACTION-REPRESENTATION-GAPS.45` (`2026-10-20`) a header row whose first cell opens with an identifier and
two more of whose cells are presence codes is a data row of the matrix, whatever its cells' header flags, and the
header keeps its labels. The fused `N N` rows still refuse, so APB5 holds 26 presence records, 6 of them this matrix's
under its own labels; no other chain moves. *Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md`
(`.45`).

**A signal's stated absent value rides its presence record.** A presence matrix says when a signal exists; the
prose says what it is when it does not: *BRESP is an optional signal. If the BRESP_WIDTH property is 0, it is not
present and assumed to be 0b000 (OKAY).* Since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a` (`2026-10-03`) that value
is bound to the signal's presence records as `absent_value` — the literal as written (`0b000`, `LOW`) and the
statement that states it — and the records cross SemanticIR and IntentIR unchanged:

```json
{ "presence_id": "presence_0082", "signal_name": "BRESP", "presence_condition": "BRESP_WIDTH > 0",
  "variant_presence": [ { "variant_label": "AXI5", "code": "O" }, "…" ], "table_id": "table_0268",
  "absent_value": { "value": "0b000", "supporting_statement_ids": ["statement_1089"] } }
```

A sentence is read when, within one sentence, an absence predicate (*not present*, *not driven*, *omitted*,
*missing*) precedes *assumed to be* and a literal follows — a binary, hexadecimal or decimal number, a level word
in capitals, or *asserted*/*deasserted* — closed by the end of the sentence or a parenthetical label. The signal
is the one presence-record name the whole statement mentions, which is how the second sentence's *it* resolves to
`BRESP` while `BRESP_WIDTH`, a property, is no candidate. A value the sentence goes on to qualify (*assumed to be
0b0 for a DVMComplete request*) is not read, because the record cannot carry the qualification. A statement that
names a class of signals (*the missing signals* covered by a check signal, in APB5 and AHB5) or a generic name no
matrix row carries (*AxSNOOP*, whose rows are `AWSNOOP` and `ARSNOOP`) binds nothing, and two statements that
disagree bind nothing: each is a measured residual with its own owner (`.3.d.ii.a.i`, `.3.d.ii.a.ii`). On the 31
chains only AXI moves: `BRESP` and `RRESP` take `0b000`. The `.isf` adapter keeps one residual per stated value,
because `.isf` has no clause for the value an absent input reads (`sf-isf-optional-signal` R3).
*Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.ii.a`).

**A check-coverage table binds the class of covered signals.** Parity specifications state which signals each
check signal protects in a table headed `Check signal | Signals covered | Width | Granularity | Check enable`
(APB5 *Table 5-1 Check signal descriptions*, AHB5 *Table 12-1 Parity check signals*, AXI *Table A16.4* with the
check column headed `Name`). Since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a.i` (`2026-10-03`) EvidenceIR carries
each row as a `check_signal_records` entry — the check signal, the signals it covers, and the width, granularity
and enable cells verbatim — gated by a header cell whose words are exactly *signals covered* and a preceding
column headed by *signal* or *name*:

```json
{ "check_id": "check_table_0014_001", "check_signal": "PCTRLCHK",
  "covered_signals": ["PPROT", "PWRITE", "PNSE"], "width": "1", "granularity": "1-5",
  "check_enable": "PSEL", "table_id": "table_0014" }
```

A name cell's trailing footnote letter is dropped (`PSELxCHK b`), a covered cell splits on commas and spaces, and
a row whose covered cell carries anything but identifiers is refused and keeps its position out of the ids. The
same reading resolves the class sentence the previous paragraph could not: *If some of the signals covered by a
check signal are not present on an interface, then the missing signals are assumed to be LOW* carries every word
of the table's two headers, so each covered signal's presence record takes `LOW`, citing the statement and, in
`supporting_table_ids`, the table it came through. A sentence naming the signal itself outranks the class. On
the 31 chains AXI gains 123 records, APB5 14 and AHB5 16; APB5 binds
15 absent values and AHB5 22. The records cross SemanticIR and IntentIR unchanged; no
check signal is lowered yet, since `.isf` has no parity construct and a check is expressible only as a
combinational XOR chain. `EXTRACTION-REPRESENTATION-GAPS.46` decided the lowering per shape (`2026-10-20`), and
its first step reads the sense: since `.46.a` every check record carries the parity sense the document states
(*Odd parity is used*; *Odd parity means that check signals are … driven such that there is always an odd number of
asserted bits*) and the statements that state it, read from a sentence that is neither apparatus nor a table row (a
configuration table's *Odd parity checking included for all signals* names a property value); a document stating
both senses gives none. All 153 records read `odd`. *Authoritative tracking:*
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.ii.a.i`, `.46`, `.46.a`).

**A document-stated collective name stands for its members.** AXI writes *Any AxSNOOP bits not driven by an
interface are assumed to be LOW*, and `AxSNOOP` is no matrix row: the rows are `AWSNOOP` and `ARSNOOP`. The
document defines the name itself, twice over — in sentences (*In this specification, AxSIZE indicates AWSIZE and
ARSIZE*; *the term VAxQOSACCEPT refers collectively to the VAWQOSACCEPT and VARQOSACCEPT signals*) and as a rule
(*Lowercase x at the second letter of a signal name denotes a collective term for both Read and Write. For
example, AxCACHE refers to both the ARCACHE and AWCACHE signals*). Since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a.ii`
(`2026-10-03`) EvidenceIR carries each as a `collective_signal_names` record — the name, its members in the
document's order, and the sentence or rule that states them — and the records cross SemanticIR and IntentIR
unchanged:

```json
{ "collective_id": "collective_0027", "collective_name": "AxSNOOP",
  "member_names": ["ARSNOOP", "AWSNOOP"], "supporting_statement_ids": ["statement_0446"] }
```

A sentence defines a name when an identifier with one run of lowercase letters is followed by a defining verb
(*indicates*, *refers to*, *denotes*) and two or more identifiers that are that name with the run replaced by
uppercase letters; the members need not be declared, since the sentence declares them, and the run may sit
anywhere (`VAx`). A statement that says *collective* and carries such an example states a rule: every other
name the document uses with the same run at the same position stands for the same replacements, read only when
all of them are known signals (declared, or a presence-matrix row). A document that states no rule gets no
derived name, so USB 3.2's `RxD` and `TxEQ` stay what they are. The absent-value binder then reaches each member
through the collective name, recording it as `collective_name` on the value; a sentence naming a member itself
outranks it, and a class sentence ranks below both. On the 31 chains two documents define collective names: AXI 37 (6
from sentences, 31 from the rule) and ADIv6 2 (*CxxxPWRUPREQ refers to either CSYSPWRUPREQ or CDBGPWRUPREQ*, with a
three-letter run); `AWSNOOP` and `ARSNOOP` take `LOW`. Wishbone's *When the gap indicates a wait state, the symbols
'-WSM-' or '-WSS-' …* is the shape's trap: a capitalised word is a lowercase run with nothing after it, so a name
must carry an uppercase letter after its run. *Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.ii.a.ii`).

**A label of the signal's own encoding, and a value that holds for one class of requests.** AXI states what a
request's domain is when the interface has no domain signals, and it never states it for the signal as a whole:
*If AxDOMAIN signals are not present:* / *- Non-cacheable requests are assumed to be in the System domain.* /
*- Cacheable requests are assumed to be in the Shareable domain.*, and *If AxDOMAIN is omitted from the
interface, CMO transactions are assumed to be Non-shareable.* Each value is a label of `AxDOMAIN`'s own encoding,
not a literal, and each holds for one class of requests. Since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii.a`
(`2026-10-19`) the binder reads both. Such a value lands in the presence record's `qualified_absent_values`, never in
its `absent_value`, which would extend it to every other request:

```json
{ "presence_id": "presence_0033", "signal_name": "AWDOMAIN", "…": "…",
  "qualified_absent_values": [
    { "value": "System domain", "applies_to": "Non-cacheable requests", "collective_name": "AxDOMAIN",
      "encoding": { "table_id": "table_0111", "codes": ["0b11"] },
      "supporting_statement_ids": ["statement_2358", "statement_2359"] },
    { "value": "Shareable domain", "applies_to": "Cacheable requests", "collective_name": "AxDOMAIN",
      "encoding": { "table_id": "table_0111", "codes": ["0b01", "0b10"] },
      "supporting_statement_ids": ["statement_2358", "statement_2360"] },
    { "value": "Non-shareable", "applies_to": "CMO transactions", "collective_name": "AxDOMAIN",
      "encoding": { "table_id": "table_0111", "codes": ["0b00"] },
      "supporting_statement_ids": ["statement_2701"] } ] }
```

*A label* is the clause's words after *assumed to be*, without a leading *in the*. It binds only when a row of the
table the document's enum for that signal cites carries it as a cell, compared without case or spacing; the enum may
be named by the signal itself or by a collective name it is a member of. Every row carrying the label contributes its
code as written, so *Shareable domain* keeps both of its codes, and choosing one would fabricate. A label no encoding
of the signal names (*Inner shareable*, or any label for a signal with no encoding) binds nothing. *A class* is the
subject of the clause that states the value, when the absence is stated in another clause or in the list's lead-in
and the subject is not the signal itself: not a pronoun (*it is not present and assumed to be 0b000*), not a noun for
a signal or its value (*the value*, *these signals*), not a name, and not itself said to be absent (*the missing
signals*). *A list* gives a bullet that states no absence of its own the absence of the statement that introduces
it, when that statement ends with a colon and states one; only bullets may stand between them, and both statements
are cited. Agreement is settled per class, so *Cacheable requests* and *Non-cacheable requests* never conflict. A
table row lifted as prose is not read at all: it belongs to its table's reader, and AXI's change history restates
the CMO default in a row whose converter-fused *a CMOis* would otherwise have become a class. A value qualified
*after* it (*assumed to be 0b0 for a DVMComplete request*) is still not read. On the 31 chains only AXI moves:
`AWDOMAIN` and `ARDOMAIN` each take three class-scoped values, SemanticIR and IntentIR carry them unchanged, the
adapter gains one residual for each (`isf_protocol_signal_qualified_absent_value_<presence>_<n>`), and the emitted
`.isf` is byte-identical. AXI also disagrees with itself here: its domain-signaling overview puts a cacheable
request in the Non-shareable domain, and *Domain signaling* names no signal, so that sentence is not read.
`EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii.c` owns surfacing the disagreement. *Authoritative tracking:*
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.ii.b.iii.a`).

### Which assume-sentences are abstractions, and which are apparatus

Every sentence holding *assume*, *assuming*, *modeled as*, *abstracted* or *treated as* was classified an explicit
abstraction before any other class, lifted to a SemanticIR abstraction and carried as an IntentIR assumption — and
`EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b` measured that the sentences fall into six shapes, most of which state
nothing about the hardware. Since `.3.d.ii.b.i` (`2026-10-03`) the classifier reads the word as a whole word (a
fragment such as *Assumer* is none of them) and *abstracted* only as a verb, keeps an equivalence rule (*treated
as*) an abstraction whatever else its sentence says, and files three measured shapes as **apparatus**, a statement
class no reader consumes: a sentence whose subject is the document, a figure or the reader (*This guide assumes that
you are familiar with …*; *In this example, the following conditions are assumed:*), a timing-diagram key (*Shaded bus
and signal areas are undefined, so the bus or signal can assume any value within the shaded area*), and the premise
of an example or an analysis (*This limitation assumes that there is a maximum of 3 logic levels*; *RJ and BUJ are
assumed as Gaussian terms*; *The latencies shown assume the memory access hits in the L1*). A hardware consequence
stated with the same word (*it can be assumed that no write occurred*) stays an abstraction.

The class is dedicated for a measured reason. Letting such a sentence fall through to the other classes minted a
conditional rule from a diagram key (*Variations, when they occur, have clear labels. You must not assume any timing
information …*), and filing it as a source fact minted a semantic hint on a clock from an example circuit's
*assumes that output data is valid at the rising [CLK_I] edge*. A sentence that only mentions such a
word, as a fragment (*Assumer*) or an adjective (*an abstracted view of a notional OpenCAPI solution*), is apparatus
too: read as ordinary prose, two of them minted invariants. On the 31 chains 83 sentences in 22 documents become
apparatus, every other evidence record is byte-identical, and the IntentIR assumptions
they produced are gone. *Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.ii.b.i`).

**An assume word that states no assumption.** Some sentences use the word and still assume nothing. Since
`.3.d.ii.b.iii.b` (`2026-10-19`) such a sentence keeps the class it would have without the word, provided **every**
assume word in it is one of four uses. A negated assumption is a rule that forbids one (*the missing input cannot be
assumed to take a fixed value*). An active *assume* whose object is a value, a state or a configuration means *take
on* (*the bits cqb.LOG2SZ-1:0 in cqt assume a valid but otherwise UNSPECIFIED value*; *causes the device to assume
the described configuration*; *Three state buffers can assume a logic low state*). An active *assume* governed by
*shall* is the device's obligation (*The hub shall ignore the route string and assume all packets are routed directly
to the hub*). And *assumed* after *value* states a value (*the DTF value assumed for reporting such faults is 0*). One
ordinary use keeps the sentence an abstraction: USB's *The interrupt endpoint should not assume a fixed spacing …
The interrupt endpoint can assume only that it will receive a transaction* stays one. Three more premise shapes are
apparatus: a figure named in the assume clause (*the conceptual clocking architecture shown in Figure E-15 is
assumed*), a document noun after *this* (*One ×8 cable is assumed for this guideline*; *The user of this document
assumes all responsibility*), and an analysis method in the clause (*The moving average smoothing procedure assumes a
window size of 1 GHz*); so are *assumed to be approximately* and *examples* in the plural. On the 31 chains 19
statements in 7 documents change class and no other evidence record moves. The IntentIR assumptions they produced
are gone (15), and three of the reclassified rules are now carried as invariants and constraints, as every other
rule is: AXI's *the CHK signal cannot be used reliably*, USB's hub routing obligation, and its *client software
cannot assume ordering between bulk and control transfers*. *Authoritative tracking:*
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.ii.b.iii.b`).

### A section that defines the document's notation

A document explains its own notation — the pseudocode it writes behaviour in, its typefaces, its keywords, how it
names signals — and those sentences were read as protocol content: ADIv6's appendix *Pseudocode Definition* gave an
optional feature (*The block of lines consisting of elsif and its indented statements is optional*) and invariants
(*Each of the following simple statements must be terminated with a semicolon*). Since
`EXTRACTION-REPRESENTATION-GAPS.51` (`2026-10-21`) every sentence of a section whose heading names the document's
notation is apparatus: a heading holding *conventions*, *notation* or *typographic*, or one whose topic is pseudocode
(*About ARM pseudocode*), but not pseudocode put to use (*Pseudocode description of the transfers*). The converter
flattens headings, so the outline number decides membership: a numbered notation heading covers the headings
numbered under it and the unnumbered ones among them, and an unnumbered one covers only itself. Readers that
interpret notation still read these sentences: AXI's collective names come from its conventions. A pseudocode
function that describes hardware, outside such a section, keeps its class. *Authoritative tracking:*
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.51`).

**A keyword's definition is typed with the obligations it states.** A notation section can define a keyword that
carries a rule on everything it marks: *Reserved is a keyword indicating reserved bits … A reserved bit, byte, word or
field shall be set to zero by the sender and shall be ignored by the receiver*. Since
`EXTRACTION-REPRESENTATION-GAPS.55.a` (`2026-10-21`) EvidenceIR reads every *X is a keyword …* definition, whatever
class its statement has, into a `keyword_definitions` record — the keyword, its defining sentence, and each
*shall*, *must* or *should* in it as an obligation with its clause, modality and the agents after *by* (*the sender*,
*the receiver*; not *this specification*) — which SemanticIR and IntentIR carry unchanged. The keyword opening its own
definition (*Shall is a keyword …*) is not a modal. Since `.55.b.iv.a` the space a layout reserves cites the
*Reserved* definition: EvidenceIR's `reserved_spans` (see *Reserved space is a typed span* under register tables). No
field record carries a keyword and the spans stop at EvidenceIR, so every adapter still names the obliging keywords in
one `isf_keyword_obligations_not_bound` residual and nothing is lowered. *Authoritative tracking:*
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.55.a`).

**An equivalence rule is a typed record.** A sentence that treats one thing as another states a rule of the
hardware, often under a condition, and it stays an abstraction by class. Since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.ii`
(`2026-10-19`) EvidenceIR also carries each one as an `equivalence_rules` record, which SemanticIR and IntentIR
carry unchanged:

```json
{ "equivalence_id": "equivalence_0000", "subject": "the instruction", "equivalent": "a nop",
  "condition": "If the instruction being stepped over would normally stall the hart",
  "modality": "stated", "supporting_statement_ids": ["statement_0906"] }
```

The subject is the clause before *treated as* and its auxiliaries, from the sentence's last clause break; the
condition is what comes before that break when it opens like one (*If*, *When*, *At reset*, *For the purpose of*);
the equivalent runs to the end of its clause; the modality is the auxiliary's word (*shall*, *must*, *should*, *may*,
*can*, *will*, *always*) or `stated`. Everything is as written, and nothing is resolved against signals or encodings.
A coordinated predicate ends its subject at the subject's own verb (*The ClearPortFeature(PORT_RESET) request shall not
be used by the USB system software and may be treated as a no-operation request by hubs*), a parenthesis holds its own
subject (*(request is treated as a no-operation)*), and a reason clause asserts its equivalence (*since the beginning of
the Warm Reset will be treated as the start of the Loopback LFPS exit signal*). Five uses are refused, because a record
would say more or less than the sentence: a clause contrasted with what came before (*otherwise they are treated as
unsupported requests*; *else the transaction is treated as …*; *in which case the request is treated as …*), a
hypothetical (*if the beginning part of Warm Reset is treated as U3 LFPS exit signal …, and …*), and a coordinated verb
with no copula (*must be ignored and treated as one*). A table row and an apparatus statement are not read. On the 31
chains 34 records in 10 documents, USB 3.2 holding ten and GICv3 six; only that section moves at each stage, the
adapter keeps one `isf_protocol_equivalence_rule_*` residual per record, and the emitted `.isf` texts are
byte-identical. The new rule field re-seals every chain. *Authoritative tracking:*
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.ii.b.ii`).

**A timing constraint states a time.** The timing arm of the statement classifier filed a sentence as a
`timing_constraint` when it held a word from each of two lists, both matched as substrings: *cycles* alone satisfied
both, *ns* matched *transactions*, and *at least* matched a size, so 645 of 813 such statements stated no time (most of
them Wishbone sentences about its bus cycles). Since `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.b` (`2026-10-20`) those
words admit a sentence only when it also states a time, read as whole words: a number and its unit (*12 ms*, *a
2-cycle delay*, *0.3 UI*, *10ms*), a counted cycle, clock or second (*one clock cycle*, *within N cycles*, *three
seconds*, *50 SWCLKTCK cycles*), an event placed at a cycle (*in the same cycle*, *the next clock cycle*, *Cycle 6*),
a parameter written the way timing parameters are named (*tSU*, *tPortConfiguration*), a setup, hold or propagation
time, a clock period, or a clock edge (*the rising ACLK edge*). A bus cycle (*the BLOCK cycles*, *the current cycle*),
a size (*at least 4 bytes*), a possessive (*2's complement*), a run of bits (*write 1s*) and a latency named without a
value state none. On the 31 chains 534 of the 813 statements leave the class for the one they would otherwise take
(374 source facts, 109 normative statements, 51 conditional rules; Wishbone 249, USB 3.2 52, ADIv6 41) and none joins
it. The readers of those classes then find 47 more conditional rules in 11 documents and 9 more signal hints in 3,
and nothing is lost. The adapters lower none of the new rules: Wishbone gains one `isf_obligations_not_lowered`
residual naming five obligations with no level to drive or met by another actor, AXI's and ADIv6's residual texts
only renumber their rule ids, and the emitted `.isf` texts are byte-identical. A sentence
that states a time but holds none of the admitting words keeps its class (*A 2 ms timer shall be started*), where its
own readers find it. *Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.iii.b`).

**A stated permission is a typed record.** Most of what IntentIR lifted as unread behaviors were permissions (*A MASTER
MAY signal Classic Cycle indefinitely*; *If the aborted transaction is a write then the IO bridge may discard the
write*), and no record carried them. Since `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.a` (`2026-10-20`) EvidenceIR carries
each permission a sentence states with *may*, *is permitted to* or *is allowed to* as a `permission_rules` record,
which SemanticIR and IntentIR carry unchanged:

```json
{ "permission_id": "permission_0164", "subject": "the IO bridge", "permitted": "discard the write",
  "condition": "If the aborted transaction is a write", "modality": "may", "supporting_statement_ids": ["statement_1905"] }
```

The subject is the clause before the marker, from the sentence's last clause break; the condition is what comes
before that break when it opens like one, or the *if* clause of *If X then Y may Z*; the permitted part runs to its
clause's end. *It is permitted to …* and *There may be …* name nobody, so the record has no subject. A coordinated
sentence gives each permission its own subject (*Each interrupt source may be configured with a unique vector or a
vector may be shared …* is two records). Refused, because a record would say more or less than the sentence: *may*
followed by a verb of what can happen (*may lead to*, *may change*, *may vary*), *may not*, *may never*, *may or may
not*, *may have to*, *may have completed*; a hypothetical with no *then*; a contrasted clause; a pronoun, the reader
(*You may only use the trademark*), a relative clause or a subject holding one (*the type of transactions that a
device may generate*), a subject opening with *no* or *neither* (a prohibition); a permission about the document
(*This document may include technical inaccuracies*); and the month (*12 May 2020*). *May* is otherwise not told apart
from a possibility (*At this point the eye may be closed* is a record). On the 31 chains 890 records in 29 documents,
USB 3.2 holding 335 and the IOMMU 179; 530 uses refused. Only that section moves at each stage, every adapter gains one
`isf_protocol_permission_rules` residual naming its records by their id range, and the emitted `.isf` texts are
byte-identical. The new rule field re-seals every chain. *Authoritative tracking:*
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.iii.a`).

**An obligation stated with an assume word.** The premise phrases of `.3.d.ii.b.i` (*assume that*, *assumes the*)
filed a statement as apparatus whatever else it said, so *A device shall not assume that transactions occur at the
same location within each service interval* was no rule. Since `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.c`
(`2026-10-20`) a statement keeps the class it would take without the word when every assume word in it is active and
governed by *shall* or *must*, negated or not (*the debugger must not assume that the TAR wraps*), or when another
of its sentences states an obligation (*shall*, *must*, *is not allowed to*) with no such word and nothing about the
document (*A channel must meet the common-mode requirements … the limits defined below assume a budgeting process*).
The reader's directive in a diagram key (*You must not assume any timing information that is not explicit in the
diagrams*), a need (*an extra cycle is required to forward results*), a disclaimer about *this specification* and a
passive *must be assumed* stay apparatus. On the 31 chains 24 statements in 8 documents change class, the readers add
7 conditional rules and 3 permissions, 11 abstractions leave SemanticIR and 23 invariants join it, and the emitted `.isf`
texts are byte-identical. *Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.iii.c`).

**An optional feature is a typed record.** A sentence that calls something optional states that it may be absent
(*All Trigger Module registers (Table 14) are optional*; *The processing of BERT commands is optional for Gen 1
loopback*), and no record carried it. It is not a permission: its subject is the feature, not an actor that may do
something. Since `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.d` (`2026-10-20`) EvidenceIR carries each feature a sentence
calls optional with *is optional* or *are optional* as an `optional_features` record, which SemanticIR and IntentIR
carry unchanged:

```json
{ "option_id": "option_0001", "feature": "support Atomic transactions", "implementer": "a Subordinate component",
  "supporting_statement_ids": ["statement_2161"] }
```

The feature is the clause before the marker, from the sentence's last clause break (inside a parenthesis, its own;
after *because* or *since*, the reason's); a condition follows *optional* (*if …*, *for …*, *on …*) or opens the
sentence before that break. *It is optional for S to V* names V as the feature and S as its implementer, and *it is
optional whether X* names *whether X*. Refused, because a record would say more or less than the sentence: *optional*
joined to another word (*optional-normative*) or qualifying a noun (*are optional open-drain outputs*), a hypothetical
(*If a register is optional*), a contrast before or after the marker (*else the field is optional*; *and is optional
otherwise*), a pronoun (*It is optional*, *All are optional*), a relative clause (*most of which are optional*), a
subject with a verb, and a feature of the document. A feature named by a demonstrative (*This register*, *This
field*) keeps it as written; the grammar does not resolve what it refers to. On the 31 chains 52 records in 13 documents, read from 61 statements; 15 uses refused. Only that
section moves at each stage, every adapter gains one `isf_protocol_optional_features` residual naming its records by
their id range, and the emitted `.isf` texts are byte-identical. *Authoritative tracking:*
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.3.d.iii.d`).

**What the hardware assumes of its environment.** The premise phrases *assume that* and *assumes that* filed a
sentence as apparatus whatever its subject, so *The WISHBONE interface logic assumes that a low-skew clock
distribution scheme is used on the target device* was read by nothing. Since `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.e`
(`2026-10-20`) those phrases make a premise only when every use of them has, in its own sentence (a table cell is
one), a document noun, a figure, a table, a rule or limitation, an analysis, *shown*, *we* or the reader before it, or
no subject at all (*Assume that a target or line error has occurred*). What a component, an interface, an
implementation or a protocol assumes is an assumption. An *example* makes a premise only before the assume word, so
*This MEC implementation assumes that Managers and caches do not perform any MECID checks. For example, …* is an
assumption the example illustrates. *Assumes the* keeps its premise reading for now (`.3.d.iii.f`). On the 31 chains 5
statements in 4 documents leave the apparatus class: Wishbone's clock-skew and read-data assumptions, AXI's MEC,
SMBus's timeout, and RISC-V debug's *Debuggers may assume that a hardware platform has no harts with indexes higher
than the first nonexistent one*. *Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md`
(`.3.d.iii.e`).

**What a subject takes on.** Since `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.f` (`2026-10-20`) *assumes the* takes the
same subject test as *assume that*, and an active *assume* means *take on* also when the head of its object, the
last word before the first preposition or subordinator, is a value, state, configuration, number or size (*a host
assumes the initial transmit or receive sequence number and burst size for bulk pipes after …*). *Of* stays inside
the object, so *else Connection Manager assumes the configuration of Asymmetric Link failed* concludes rather than
takes on, and is an assumption the census routes to the software-intent leaf. On the 31 chains 5 statements in 3
documents change class: the host's sequence state and a Wishbone patent disclaimer become source facts, the two
connection-manager conclusions and a Wishbone example circuit's *Circuit assumes the use of synchronous RAM
primitives* become assumptions. *Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md`
(`.3.d.iii.f`).

**What a software agent does is software intent.** A document often says what software does to the hardware (*A
Connection Manager shall configure DP AUX Path with the Dedicated Flow Control Buffer Allocation scheme*; *the
debugger must use the IDR register in the AP*), and nothing told that from what the hardware does. Since
`EXTRACTION-REPRESENTATION-GAPS.3.d.i` (`2026-10-20`) EvidenceIR carries every statement whose subject is an agent
the document makes software as a `software_intent` record, which SemanticIR and IntentIR carry unchanged:

```json
{ "intent_id": "software_intent_0070", "agent": "A Connection Manager", "software_name": "connection manager",
  "basis": "implemented_in_software", "basis_statement_ids": ["statement_0166"], "modality": "obligation",
  "supporting_statement_ids": ["statement_0433"] }
```

The document makes an agent software in one of three ways, each its own words: an English noun that names software
(*software*, *firmware*, *operating system*, *hypervisor*, *device driver*, *debugger*), a glossary row that defines the
name as software (*| Hypervisor | Software entity that controls virtualization. |*), or a sentence saying the agent is
implemented in software (*A Connection Manager can be implemented in software or firmware*). The subject is read past
list markers, a note label and a leading condition clause, up to the first modal or verb; the name must head it, with
at most three words before it and no preposition (*External accesses from a debugger* is no software intent). The
word after the subject gives the modality: *must*/*shall* an obligation, *should* a recommendation, *may* a permission,
*can* a capability, anything else a description. Software intent is non-executable by design, so every adapter names a
document's records in one `isf_software_intent_not_applicable` residual, the explicit disposition the category
contract requires; nothing is lowered. *Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md`
(`.3.d.i`).

**A software agent's statement mints no hardware rule.** The readers that ran before `software_intent` still read
its statements as hardware: *Software must enable the system register interface before using these registers* was an
invariant, and the USB4 connection-manager guide's procedure steps were invariants and conditional rules, though the
category contract allows a methodology guide no false executable fact. Since `EXTRACTION-REPRESENTATION-GAPS.52`
(`2026-10-20`) a statement `software_intent` reads mints no SemanticIR invariant, and a conditional rule read only from
such statements is withheld (`withhold_software_rules`); the statement stays software intent. A record that also cites
a hardware statement keeps it (a transaction, an interface), and so does an actor relation that makes the software
the hardware's external actor (*the debugger drives TDI*). On the 31 chains 25 conditional rules leave in 6
documents, and 102 invariants in 10, 64 of them the connection-manager guide's; the IntentIR constraints derived from
those invariants go with them, and the emitted `.isf` texts are byte-identical. *Authoritative tracking:* `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.52`).
