# EvidenceIR

`EvidenceIR` is where the pipeline starts lifting grounded facts out of the source.

## What belongs in `EvidenceIR`

- extracted statements
- section anchors
- evidence spans
- visual evidence
- figure/caption links
- actor-signal relations
- early signal facts
- semantic hints
- polarity evidence

This is the first stage where the system begins to say:

- "this table row looks like a signal declaration"
- "this prose sentence looks like a constraint"
- "this caption appears to ground a semantic role"
- "this visual observation may support a timing fact"

## The mindset of this stage

This is still an evidence layer, not the final semantic truth.

So the right behavior is:

- extract what the document supports
- preserve provenance
- keep conflicting evidence visible
- avoid over-promoting generic examples into canonical interface truth

That last point matters a lot.
`EvidenceIR` is not supposed to be clever in the sense of inventing final meaning.
It is supposed to be disciplined in the sense of preserving recoverable evidence without silently flattening ambiguity.

## Typical evidence-level wins

- source/destination table recovery
- table-grounded widths
- table-synthesized signal declarations with provenance back to the originating structured table
- prose-grounded actor relations
- visual-caption semantic hints
- VLM timing-note observations
- polarity extraction, including explicit asserted-when-level prose such as `CS_N is asserted when LOW`, unambiguous collective prose such as `CS_N and WE_N are active LOW signals`, and safe clause-local mixed prose such as `CS_N is active LOW and ENABLE is active HIGH`
- negative-knowledge caution surfacing during validation

These wins are valuable because they give later stages something much stronger than free-form text:

- typed hints
- grounded spans
- table-linked facts
- visual-evidence references
- early KG edges
- explicit caution signals

The dedicated [Multimodal Evidence And Visual Grounding](multimodal-evidence.md) chapter explains the visual part of that evidence flow in more detail.

## What this stage is allowed to do

`EvidenceIR` is allowed to extract and classify.

It is allowed to say:

- this sentence is a constraint-like statement
- this signal appears in a relation-like table row
- this caption text supports a semantic-role hint
- this table suggests a width, polarity, or source/destination relation

It can also apply a collective polarity statement to multiple declared controls when the wording has one unambiguous active level.
For example, `CS_N and WE_N are active LOW signals` can ground both controls as active-low.
Mixed compound wording such as `CS_N is active LOW and ENABLE is active HIGH` can be recovered only when the clause-local parser can split and validate every signal-level pair safely.
If a polarity phrase is detached from an explicit signal, the statement stays unresolved instead of borrowing an implicit subject.
When polarity is resolved, later canonical stages expose it per signal, not only as a validation total.
If prose and a signal-description table disagree about the same signal's active level, `EvidenceIR` keeps a typed `signal_polarity_conflicts` record instead of silently picking the prose or table side.

When `EvidenceIR` synthesizes a formal declaration from a signal-description table, it also records a `table_signal_declaration_provenance` entry.
That entry links the synthetic statement id back to the structured `SourceIR` table id, so later stages can preserve table support on canonical interface signals instead of losing the fact that the declaration came from a real table.
Validation reports this count as `table_signal_declaration_provenance`, which makes the evidence-stage table bridge visible before `SemanticIR` and `IntentIR` decide what survives into canonical signal inventory.

It is not supposed to decide the final canonical meaning of the whole interface.
Likewise, negative-knowledge priors may make validation more alert to a repeated evidence-stage conflict pattern, but they do not suppress the current evidence or decide the conflict.
When that happens, validation can also mark the matched current conflict as rescan/corroboration guidance through `evidence_negative_knowledge_rescan_guidance`.
Later `SemanticIR` and `IntentIR` validation carry the same caution idea forward for repeated conflict and residual shapes.

## Capturing the agents a spec defines in prose

A protocol is a conversation between *agents* — a manager and a subordinate, a host and a device, a controller
and a target. Many specs never put those agents in a table; they introduce them in a sentence: *"A controller
is the device that initiates a data transfer and generates the clock."* `EvidenceIR` captures those
definitions as first-class `ProtocolActorRecord`s, so the agent model is grounded directly from the document's
own words rather than only inferred as the subject of a relation.

The grammar is deliberately general: *"`<NAME>` is a/an/the `<agent-class>` that/which `<capability>`"*, where
`<agent-class>` is one of a small, generic set of agent/component words (device, component, agent, module,
controller, manager, bridge, host, node, …) — never a chip, vendor, or protocol name. The defining
"that/which" clause is what distinguishes a real definition from a passing "is a component **of** the system"
mention.

Broadening a grammar like this is exactly where false positives creep in, so the safety here is **structural**,
not a hand-maintained list of forbidden words:

- a trailing cross-reference such as *"An I/O controller (refer to section 3.1.2.1) is a controller that …"* is
  stripped before the agent name is read, so the agent is "controller", not "section";
- the name is only looked for inside the **current sentence**, so *"… host system. It is the entity that …"*
  cannot reach back across the full stop and mis-name the system;
- if a **preposition** sits in the subject — *"a use case **for** multiple HSEL signals is a peripheral that …"*
  — the last noun ("signals") is a prepositional object, not the subject, so the sentence is skipped.

These are closed, principled signals (punctuation, sentence boundaries, a closed class of prepositions), which
is why the capture can widen across many documents without minting noise. On the corpus it lifts the number of
documents with a recovered agent meaningfully, and every newly-recovered agent is a genuine one — a CPU core, a
trace unit, a debug-access port, an interconnect manager — while the four wire-based reference specs are
unaffected.

## Typical evidence-level failure modes

- field tables leaking fake signals
- abstract example tables pretending to be real interfaces
- payload nouns being promoted to actors
- descriptive relative-clause phrases such as `mixture of` being promoted to actors
- signal names creating semantic meaning by spelling alone

Many of the project’s recent truthfulness slices have been about tightening exactly those boundaries.

## Why provenance is critical here

`EvidenceIR` is where the project first needs to defend itself against "plausible but wrong" extraction.

That is why evidence records carry things like:

- supporting statement ids
- supporting table ids
- supporting visual evidence ids
- automation confidence

Without that provenance, later semantic arbitration would not have enough context to judge which evidence is strong, weak, conflicting, or merely suggestive.

## What a good `EvidenceIR` artifact looks like

A good evidence artifact is not one that looks clean at all costs.

It is one that:

- extracts a lot of grounded candidate knowledge
- preserves where that knowledge came from
- keeps disagreement visible
- avoids creating false structure from generic or noisy inputs

That makes `EvidenceIR` the main staging area for truthfulness before canonical semantics begin.

## Closed task trees — how each was implemented and verified

### `R6-EVIDENCE-HARDENING` — close zero-coverage assertion gaps on `EvidenceIR`

This tree added regression-only test assertions to populated-but-
untested fields on `EvidenceIr`, `ExtractedStatement`, and
`FsmSignalCandidate` (the latter retained through historical
cycles and re-purposed for signal-binding context). The leaves
worked symbol-by-symbol so failure isolation stayed sharp.
Verified by mutation testing reducing missed mutants to zero on
the targeted symbols + `scripts/run_ci.sh`. *Authoritative
tracking:* `docs/tasks/R6-EVIDENCE-HARDENING.md`.

### `R15C-CONVERGENCE-REPORT` — make the anchored-rescan loop inspectable

**What it gives you:** when you run `specforge validate` on an
`EvidenceIR`, you now see a `Convergence` section — how many passes
the extractor ran, how many *genuinely new* facts it recovered, and
whether it actually settled or was cut off.

**Why that matters.** Building `EvidenceIR` is not a single pass.
Newly discovered signals become anchors that unlock more tables,
whose new value atoms unlock more prose constraints — so the
extractor loops, re-scanning with everything it has learned so far,
until a pass turns up nothing new. That last part is the honesty
question: *did the loop genuinely run out of new facts, or did it
just hit its pass limit while still finding more?* Those two
outcomes look identical in the final artifact, but they mean very
different things for how complete your capture is.

So the loop now records an `EvidenceConvergenceReport`: `passes_run`
(of `max_passes`), `new_facts_per_pass` (counting *deduplicated* new
statements — real knowledge growth, not vectors getting longer),
`total_new_facts`, and a `converged` flag. `validate` turns that into
a finding you can act on:

- **Converged** → an `Info` finding: the loop stabilized; the
  anchored rescan is as complete as this document allows.
- **Capped** → a `Warning` finding: the loop stopped at the
  pass limit while still discovering facts, so convergence is *not*
  proven and the capture may be incomplete — a signal to look closer.

The extraction outcomes themselves are unchanged; this is a pure
visibility surface over a loop that was already running. It satisfies
the R15c criterion "convergence reporting counts genuinely new
persisted facts instead of duplicate vector growth," and it gives
future R15c accuracy work a metric to push against. Verified by unit
tests on the recorded report (converged path) and both `validate`
findings (converged `Info` / capped `Warning`) + `scripts/run_ci.sh`.
*Authoritative tracking:* `docs/tasks/R15C-CONVERGENCE-REPORT.md`
(under `R15C-R15G-LEARNING-PLANE-BACKFILL.1`).

### `COMPLETENESS-REGION-ACCOUNTING` — flag intent-bearing tables that produced nothing

**What it gives you:** when you `validate` an `EvidenceIR`, a `Region
Accounting` section tells you whether any table that SpecForge *recognized* as a
register / signal / timing table came out **empty** — i.e. the table's purpose
was identified but no facts were captured from it.

**Why it matters.** A miss is, by definition, something in the document that did
*not* make it into the extraction — which is exactly what you can't see by
looking at the output alone. The reframe that makes misses findable is to check
the *input* side: every intent-bearing source region should produce at least one
fact. So this detector pairs each `SignalDescription` / `RegisterMap` /
`TimingParameter` table with the records it produced — using the existing
provenance (signal-declaration `table_id`, and the `table_id` embedded in
register/timing record ids, e.g. `reg_table_0026_000`) — and flags any such
table that yielded **zero** records as an `UnexplainedTableResidual` (a Warning
finding + the `region_unexplained_tables` metric). It is flag-only: it never
invents a fact, it makes a *gap visible* so you (or a rescan, or a fix) can act.

This is the first slice of the broader region-accounting instrument (prose and
figure regions, and a unified coverage report, follow). Verified by unit tests
(covered table / zero-yield table / non-intent kinds skipped / id-marker
precision) and a `validate` wiring test + `scripts/run_ci.sh`. *Authoritative
tracking:* `docs/tasks/COMPLETENESS-REGION-ACCOUNTING.md`.

#### Duplicate tables don't count as misses (`WIRE-BASED-100.3a`)

**The trap this avoids:** a real spec often presents the *same* signals in more
than one table — a detailed "Signal / Source / Width / Description" table, and
elsewhere a compact version-matrix summary ("which signals exist in v5 vs v4 vs
v3"). SpecForge captures the catalog from the detailed table; the summary table
then produces no new record of its own. If the gauge flagged *that* as a miss it
would be crying wolf — every signal it lists is already captured. Real PDFs make
this worse: the table extractor sometimes shuffles a summary table's columns
(the APB version matrix comes out with the signal-name column rotated to the
*last* position), so the summary yields zero direct records purely as an artifact
of layout, not a true gap.

**What it does:** before flagging a `SignalDescription` table as unexplained, the
gauge asks one more honest question — *is every signal this table lists already
in the document's captured inventory?* If yes, the table is a duplicate
presentation, not a miss, and it is not counted. The signal-name column is found
by **content** (the column with the most distinct hardware-signal names), so the
check survives the column-rotation artifact and is never fooled by a repeated
"Property" value tying the count. It stays strict: if **even one** signal in that
column is *not* in the inventory, the table is still flagged — a genuine catalog
miss can never be hidden, and nothing is ever fabricated to make a counter look
good (the inventory is read, never written). On the AMBA APB spec this drops two
false "unexplained table" misses while leaving a genuinely garbled third table
flagged for review — the honest result. Verified by unit tests (covered-by-
inventory gold, the Property-column tie, an unknown-signal negative).
*Authoritative tracking:* `docs/tasks/WIRE-BASED-100.md`.

#### Captured requirements don't count as residuals (`WIRE-BASED-100.3b`)

The same honesty applies to the **prose** side of the count. A sentence like "the
Requester must drive PSTRB LOW" is recognized as a *normative* statement, and a
downstream extractor turns it into a typed constraint (`PSTRB must be LOW`) — but
the sentence keeps its "normative" label. The completeness summary used to count
every normative-labelled sentence as a "partially-structured residual," so a
requirement that *was* captured got double-counted as a gap. The fix: a normative
statement counts as a residual only when **no** typed record (a constraint or a
conditional rule) cites it. A requirement that produced a fact is captured, not a
miss; a requirement nothing structured (e.g. a system-level "error correction is
required end-to-end" with no per-signal obligation) correctly stays a residual for
review — never fabricated into a fact. Together with the duplicate-table fix above,
this makes the candidate-miss count *accurate*: on the AMBA APB spec it reports only
the genuine review items, with zero false alarms.

### `COMPLETENESS-REPORT-SURFACE` — one completeness headline

**What it gives you:** a `Completeness Summary` at the end of `validate` that
answers "how completely did SpecForge capture this document?" in one place —
a `candidate_misses` total with its breakdown, and the anchored-rescan
convergence status.

The completeness checks each report their own signal (register tiling, region
accounting, prose residuals, convergence); this summary **adds them up** so you
don't have to. `candidate_misses` = register-field overlaps + interior gaps +
unexplained intent-bearing tables + partially-structured normative statements,
and the headline names each component. It is honest by construction: a *count of
candidate misses*, never a claim of completeness — and because it only sums
values the detectors already produced, it can never introduce a gap the
detectors didn't already surface (a wiring test asserts the total equals the sum
of its component metrics). *Authoritative tracking:*
`docs/tasks/COMPLETENESS-REPORT-SURFACE.md`.

### `COMPLETENESS-CLOSURE-INVARIANTS` — register bit-fields that contradict themselves

**What it gives you:** when you `validate` an `EvidenceIR`, a `Register Tiling`
section flags any register whose documented bit-fields **overlap** (two fields
claim the same bit — a contradiction) or leave an **interior gap** (an uncovered
bit *between* the lowest and highest documented field — a likely missed field).

**Why it's a closure invariant.** A register is supposed to tile its bits: each
bit belongs to exactly one field. That's a structural law you can check exactly,
with no ground truth — so a violation is a high-confidence signal that the
extraction (or the spec) is wrong. It is deliberately conservative: the IR
carries no register *width*, so bits *above* the highest documented field are
never flagged (that would require guessing the width); reversed bounds are
normalized; registers with fewer than two bit-bounded fields are skipped.

On the real corpus this detector did double duty as an **extraction-precision
signal** — its overlap finding on an I2C table was what first exposed an
over-eager register-map classifier (since fixed; see SourceIR), and across 84
genuine registers it raised zero false positives. Overlaps surface as a Warning,
interior gaps as Info (a gap is a *candidate* missed field, not a proven defect).
*Authoritative tracking:* `docs/tasks/COMPLETENESS-CLOSURE-INVARIANTS.md`.

### `EXTRACTION-GAP-FIX.4c` — putting a split register back together

PDF backends often break one register's field-definition table across several
table fragments — a long register that spans a page boundary, say. The field
reader then produces *several* register records for one register, each holding a
slice of its fields. That inflates the register count and, worse, means no single
record holds the register's whole field set (so the bit-recovery check below has
nothing complete to match against).

`EvidenceIR` de-fragments these: register records sharing the same recovered name
are merged back into one. The safety rule is deliberately strict — a group is
merged **only when every field name across it is distinct**. That one test is what
keeps it honest. If a fragment repeats a field name (a garbled bit-row the backend
read as the same field over and over), or if two fragments share a field name
(which usually means they are actually *different* registers an upstream
name-association lumped together — a register *array* like `sbaddress0..3`, for
instance), the merge is refused and the fragments are left exactly as they were. A
register's field set is never invented to make it look whole. On the RISC-V Debug
spec this collapses 60 record fragments into 44 real registers — `dmcontrol`'s
thirteen fields come back together in one record — while the genuinely ambiguous
groups stay split and visible.

### `EXTRACTION-GAP-FIX.4a` — recovering bits that live only in the layout diagram

The tiling law above checks a register that *has* bit positions. The mirror-image
problem is a register that has **none** — because the spec drew the bit layout as a
picture and never repeated those numbers in the field table. The field reader then
recovers the field names but cannot fill a single bit position; the numbers are in
a modality (the image) it does not read. The honest baseline is to leave them
empty rather than invent them.

The `recover-register-bits` command closes that gap using the *same* tiling law as
a reconstruction tool. A vision model reads the field **names**, their **order**,
and each cell's **width** off the diagram — the things it reads reliably — while
its (unreliable) absolute bit numbers are discarded. Because a register's fields
tile it MSB→LSB with no gaps, the widths alone determine every field's exact range.

It is gated so it can only *confirm*, never *guess*: the widths must sum to a
standard register size (8/16/32/64/128) **and** the proposed names must match the
register's own field table. A register whose 14 named fields tile all 32 bits
comes back exact; a register with reserved gaps the table does not name, or a
misread width, fails a gate and stays an honest residual — no bit is ever
fabricated. The full operator-facing description is in the
[commands chapter](../commands/quality-and-learning.md#recover-register-bits);
*authoritative tracking:* `docs/tasks/EXTRACTION-GAP-FIX.md`.

### `PDF-VARIANT-DIGESTION.9.3a` — recovering a protocol's state machine from prose

A protocol's **state machine** is often the heart of what an implementer needs —
and many specs never draw it as a clean table or diagram, they just *describe* it
in prose. SpecForge already read one such shape: the JTAG/SWD style, where a state
is a capitalized hyphenated name followed by the word *state* (`Shift-DR state`,
`Test-Logic-Reset state`). But a whole family of serial protocols writes its states
a completely different way, and that reader saw nothing in them.

CAN is the clean example. Its fault-confinement FSM is defined like this: *"a unit
may be in one of three states: `'error active'`, `'error passive'`, `'bus off'`"*,
then *"a node is `'error passive'` when the transmit error count reaches 128"*. The
states are **single-quoted operational modes of an actor** — there is no "`<Name>`
state" phrasing at all — so the old reader recovered zero states from CAN.

SpecForge now reads this second shape too. A state is recognized when a quoted
short name is **bound to a generic actor** — a *node*, *unit*, *station*, or
*device* — either as an adjective (`'error active'` **unit**) or as a predicate
(a **node** *is* `'error passive'`). That one binding rule is what keeps it honest:
it is exactly what separates a real state (a *node* is `'error passive'`) from a
quoted **bit value** (a *bit* is `'dominant'`) or a quoted **bus condition** (the
*bus* is `'idle'`) — neither of which is a node mode, so neither becomes a state.
Two more rules block stray quotes: a name has to **recur** across at least two
sentences, and a document has to yield **at least two distinct states** (a single
quoted phrase is not a state machine). The trailing `when …` clause is kept as the
state's transition condition. Nothing is invented — every state is a mode the prose
literally quotes.

The result on the CAN specification is its exact error-state FSM —
`error active`, `error passive`, `bus off` — recovered from text, with no spurious
states. The reader is purely additive: protocols that don't write states this way
(the parallel buses, NVMe, I2C, RISC-V Debug) recover **zero** here and are
unchanged, and the JTAG/SWD reader keeps its own states. This is the first prose
lever in SpecForge's push to digest a new **serial-protocol class** (CAN, SWP,
SMBus, I2S); frame-field recovery is its sibling follow-up. *Authoritative
tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.9.3a`).

### `PDF-VARIANT-DIGESTION.9.7` — a state machine written as plain `<NAME> state`

There turns out to be a *third* way protocols name their states, and the two
readers above both missed it. The Single Wire Protocol (SWP) never says "state
machine" or draws its FSM — it just narrates transitions: *"the terminal shall set
SWP to the `DEACTIVATED` state"*, *"the CLF puts SWP into the `ACTIVATED` state"*,
*"while in the `SUSPENDED` state …"*. The states are **single, ALL-CAPS words**
followed by *state* — too plain for the JTAG/SWD reader (which needs a hyphenated
name like `Shift-DR`) and not quoted, so `.9.3a` skipped them too. SWP's whole FSM
went unread.

The danger with matching a single word before *state* is obvious: a document also
says "the security state", "the cache state", "the current state" — none of which
is an FSM state. So this reader leans on one structural cue that an actual state
always carries: **you enter it, leave it, or are in it**. A word is only taken as a
state when it sits in `<trigger> [the] <NAME> state`, where the trigger is a
transition or locative word — *enter*, *into*, *leave*, *exit*, *to*, *in*, *from*,
*move*, *transition*, *return*, *remain*. That single binding is what tells a real
state (*the interface moves into the* `SETUP` *state*) apart from a description (*the*
`security` *state controls access*) or a machine name (*the JTAG TAP* `state machine`,
which a guard explicitly drops). As with `.9.3a`, two more rules keep it honest — a
name must **recur** across at least two sentences, and a document must yield **at
least two distinct** such states — so the structural pattern *is* the "is this an
FSM?" gate, with no need to trust the words "state machine" (which SWP never uses).
A short denylist removes logic levels (`HIGH`/`LOW`) and the architectural
pseudo-values `UNKNOWN`/`UNPREDICTABLE`, which are never states.

On SWP this recovers exactly its interface FSM — `ACTIVATED`, `DEACTIVATED`,
`SUSPENDED`, `HALT` — from text alone, where before there were none. And because the
binding is so strict, it stays additive and honest across the corpus: the JTAG/SWD
spec gains **nothing** (its noisy `TAP`/`JTAG` mentions never appear as a transition
*into* a state), CAN keeps its `.9.3a` quoted states unchanged, and where a parallel
bus genuinely *does* describe an operating machine — APB's `SETUP`/`ACCESS`, AXI's
low-power `RUN`/`STOP`/`ACTIVATE`/`DEACTIVATE` — those real states are now captured
too, a correctness gain that touches none of the scored constraint/relation/temporal
surfaces. Across the tracked corpus, 18 documents (CHI, CXS, DTI, CCIX, CoreSight,
eMMC, USB4, …) gain a genuine FSM surface from this one grammar. *Authoritative
tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.9.7`).

### `PDF-VARIANT-DIGESTION.9.8` — a signal named only in a sentence, not a table

Most specs list their wires in a signal table, and `EvidenceIR` reads them straight
from there. Some don't. The Single Wire Protocol (SWP) introduces its two signals in
running prose: *"S1 is a signal in the voltage domain …"*, *"S2 is a signal in the
current domain …"*, and in a glossary line *"S1: signal from the master to a slave"*.
With no table to read, the earlier prose readers — the pin appositive (*"a clock pin,
SWCLK"*) and the parenthetical abbreviation (*"a serial data line (SDA)"*) — both
walked right past them, and SWP entered the pipeline with **zero** signals.

The temptation is to grab any word next to "signal" — but the chase is exactly the
trap: a spec says *"control signal"*, *"these signals"*, *"the signal is asserted"*,
*"signal integrity"* thousands of times, and almost none of those words is a signal
*name*. So this reader does the opposite of greedy matching. It fires only on a
sentence that **defines** a token as a signal — the copula *"&lt;NAME&gt; is a/an
signal …"* or the glossary colon *"&lt;NAME&gt;: signal …"* — and it requires the
candidate to be an **all-uppercase identifier token** (the same `is_hardware_signal_token`
shape used everywhere else). That second rule is what keeps it clean without a
hand-maintained word-blocklist: *"an interrupt is a signal"* and *"it is a signal"*
never qualify, because `interrupt` and `it` aren't identifier-shaped. The grammar was
locked the same way every grammar in this stage is — by **probing all of the stored
documents before writing a line of code**: across the whole corpus this definitional
reader yields exactly `S1` and `S2`, and nothing else.

That precision is also the honesty boundary. SWP's third name, `SWIO`, is **not** a
third signal — it's the single physical contact (C6) that *carries* S1 and S2 (*"S1
shares the same electrical contact as S2"*). It has no definitional sentence, only a
noisy abbreviation-table row whose "Input/Output" wording it shares with non-signals
like `MMIO` and `DMA` in other documents. Rather than guess, SpecForge leaves the
contact as an honest residual — capturing the two real signals it can prove, and not
inventing a third. The reader runs only as a fallback for table-poor documents, so the
table-rich buses (APB/AHB/AXI) and the serial-debug spec (SWD/ADI) are provably
untouched. *Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.9.8`).

### `PDF-VARIANT-DIGESTION.9.3b` — a frame described as a sentence, not a bit-range table

The SWD reader recovers a packet frame written as bit-ranges (`ACK[2:0]`, `WDATA[0:31]`). Other protocols
describe their frame in plain prose. CAN says: *"A DATA FRAME is composed of seven different bit fields: START
OF FRAME, ARBITRATION FIELD, CONTROL FIELD, DATA FIELD, CRC FIELD, ACK FIELD, END OF FRAME"*, and then states
each field's width unevenly, sentences apart (*"the CONTROL FIELD consists of six bits"*, *"the ACK FIELD is
two bits long"*). There's no table to read, so the bit-range reader found nothing — CAN entered the pipeline
with no frame at all.

Two ideas make reading this both general and honest. First, **the composition list scopes the frame.** Only the
fields named in *"composed of N … bit fields: …"* are taken as frame fields. That matters because CAN says "N
bits" about *many* things that are **not** frame fields — error flags, the overload delimiter, intermission —
and a naive "find every N-bit thing" reader would sweep them all in. Anchoring on the one sentence that
enumerates the frame keeps exactly the seven real fields and nothing else; across the whole tracked corpus this
shape fires on CAN alone.

Second, **a width is recorded only when the prose states it directly for the field** — and the reader is
deliberately strict about what "directly" means. *"The CONTROL FIELD consists of six bits"* gives the control
field a width of 6; *"the ACK FIELD is two bits long"* gives the ack field 2. But *"the ARBITRATION FIELD
consists of the 11 bit IDENTIFIER and the RTR-BIT"* does **not** make the arbitration field 11 bits — the
"11 bit" there describes a *sub-field* (the identifier), and the arbitration field is actually wider. The
reader distinguishes the two by requiring the plural *"N bits"* as the field's own count and rejecting the
singular *"N bit \<noun\>"* modifier form, so it never attaches a subtly wrong number. The four fields whose
widths CAN only gives anaphorically, variably, or for a sub-part (start-of-frame, data, CRC, end-of-frame) stay
honest blanks rather than guesses — the same "a residual beats a fabrication" rule used everywhere in this
stage.

CAN now carries its full seven-field frame in order, with the two cleanly-stated widths filled and the rest
left open. The reader is additive and disjoint from the bit-range path (CAN lacks the serial-wire markers that
trigger it; the bit-range specs lack the composition sentence), so SWD keeps its frame unchanged and the
parallel buses stay empty. *Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.9.3b`).

### `PDF-VARIANT-DIGESTION.9.11` — a timing table whose rows got mistaken for headings

A datasheet timing table is usually easy to read: a column for the parameter name, then `MIN` / `TYP` / `MAX`,
one row per parameter. But the *shape* a PDF converter hands us doesn't always match the shape a human sees. In
some specs (the I2S bus spec, and one of the SMBus timing tables) the converter looked at the left-hand column —
the parameter *names* — decided it was a row of headings because it sits at the edge of the table, and filed the
**entire row** under "table header." The numbers came along for the ride. The result: a table that visibly has
five timing parameters reported **zero**, because the part of the code that reads data rows found the data-row
list empty — every row had been re-labelled as a heading.

The fix is to read the table by its *structure*, not its labels. Inside one of these tables, a real heading row
and a real data row look different in one reliable way: in the heading row every cell is marked as a header
(`MIN`, `TYP`, `MAX` are all column titles), whereas in a data row only the left-hand *name* cell is — the value
cells next to it (`360`, `400`, `440`) are plain data. So when the data-row list comes up empty, the reader
looks back through the rows that were filed as headings and rescues any whose value cells are plain data and
whose first cell is a real label. Those are data rows wearing a heading's coat.

This stays deliberately careful. A table that genuinely *does* have a two-level heading — for example a
cross-tab with `TRANSMITTER` and `RECEIVER` spanning groups of columns — keeps real header cells all the way
across, so it correctly fails the test and is left as an honest "couldn't read this one" rather than being
forced into invented parameters. Empty value cells stay empty instead of being filled with a guess. Nothing
about the rescue depends on the parameter *names* or their capitalisation — only on the table's structure — so
it generalises to any spec with this layout instead of to a list of known tables.

With this in place, the I2S spec went from **0 to 5** recovered timing parameters (clock period, clock-high and
clock-low times, set-up and hold times, with their stated limits), while the timing tables that already read
correctly are untouched. *Authoritative tracking:* `docs/tasks/PDF-VARIANT-DIGESTION.md` (`.9.11`).

### `PER-EXTRACTOR-FACT-TAGGING` — who found which fact (recall-gauge groundwork)

This is plumbing for a future **calibrated recall estimate**. To estimate how
much a document states that SpecForge *didn't* capture, you compare independent
extractors: the fraction each finds alone vs. together lets you infer the unseen
remainder (capture–recapture). That needs to know *which* extractor found each
fact — but the pipeline normally merges (and de-duplicates) everything into one
`EvidenceIR`, erasing that.

So `EvidenceIR` now carries a `fact_provenance` index: each entry records that a
tier (**Pattern** = the structural prose/table extractor at build, **Nlp** = the
LLM in `nlp-enrich`) found a fact, under a **canonical key** that normalizes the
fact (e.g. signal + constraint kind + value) so the *same* constraint found by
two tiers maps to the *same* key — which is exactly the overlap the estimate
needs. Crucially, the NLP tier's finds are recorded *before* de-duplication, so
an overlap with a pattern find isn't silently dropped. `validate` shows the
per-tier counts. This slice records the data (signal constraints first); the
recall estimate that consumes it is a separate, designed follow-on. *Authoritative
tracking:* `docs/tasks/PER-EXTRACTOR-FACT-TAGGING.md`.

### `COMPLETENESS-RECALL-GAUGE` — an honest estimate of what's still missing

**What it gives you:** a `Recall Estimate` line in `validate` that puts a number
on the *unseen* — roughly how many signal-constraint facts the document states
that **neither** extractor captured — instead of only listing the misses it can
point to.

It uses **capture–recapture**: if the pattern tier finds a set of facts and the
NLP tier finds another, the size of their *overlap* tells you how much you're
likely still missing (a lot of overlap ⇒ you've probably found most of it; little
overlap ⇒ there's likely a large unseen remainder). With two extractors that is
the Lincoln–Petersen estimator `N̂ = |a|·|b| / overlap`; remaining misses ≈ `N̂ −
distinct-found`.

It is deliberately honest: it computes **only** when both tiers have findings
that actually overlap (so it never invents a "0 misses" out of no data — you'll
see *"insufficient — run nlp-enrich"* instead), and it reports the remaining
misses as a **lower bound** with its assumptions printed (the two tiers read the
same prose, so they're partially correlated and the estimate is optimistic). A
complementary **Chao estimator**, robust to that correlation, is now reported
alongside it (see *`RECALL-CHAO-ESTIMATOR`* below); a future third, independent
extractor would unlock the sharper 3-source model.
*Authoritative tracking:* `docs/tasks/COMPLETENESS-RECALL-GAUGE.md`.

### `COMPLETENESS-RECALL-RELATIONS` — the same gauge, now for actor–signal relations

Signal-constraint facts were the first fact kind to be tagged and gauged, but
they are not the only intent-bearing relation a document states. The **Tier-3
relation extractor** (`signal-resolve`, the LLM that reads "the manager *drives*
`HTRANS`") produces a second, independent fact kind: **actor–signal relations**
(`actor / relation-direction / signal`). Misses there matter just as much — an
undetected "drives/reads" edge is a dropped piece of the interaction model.

So the provenance index and the recall gauge are now **generic over fact kind**.
The `fact_provenance` records carry a `fact_kind` tag, relations get their own
canonical key (`actor | direction | signal`, case- and whitespace-normalized so
the *same* edge found by the pattern build and by `signal-resolve` collapses to
one key), and the Lincoln–Petersen estimator takes the fact kind as a parameter —
the math is identical, only the population changes. `validate` now prints a
per-kind provenance breakdown **and** a second recall line,
`recall_estimate_relation_remaining_misses`, alongside the signal-constraint one.
The same honesty rules apply per kind: each estimate computes only when both tiers
have overlapping finds for *that* kind, and the two kinds never cross-contaminate.
*Authoritative tracking:* `docs/tasks/COMPLETENESS-RECALL-RELATIONS.md`.

### `RECALL-CHAO-ESTIMATOR` — a second, heterogeneity-aware estimate of the unseen

**What it gives you:** the `Recall Estimate` line now prints **two** numbers for the
remaining misses, not one — a Lincoln–Petersen estimate *and* a Chao estimate — so you read
the unseen as an honest **range** instead of a single point.

**Why a second number is worth it.** Lincoln–Petersen assumes both extractors are equally
likely to catch any given fact. SpecForge's two tiers are not like that: they read the same
prose and have different strengths, so some facts are easy for both to catch and some are
hard for both — *unequal catchability*. When that happens, Lincoln–Petersen tends to
**under**-estimate how much is missing. The **Chao estimator** (Chao, 1987) was designed for
exactly this case: it leans on the facts caught by only one tier (the "singletons") to infer
how many were caught by neither, and it tolerates the heterogeneity LP assumes away. On the
same data where LP estimates 8, Chao estimates 10 — so SpecForge prints *"estimated_total
LP 8 / Chao 10, remaining_misses ≥ 2 / 4"*: a wider, more honest bound.

It stays deliberately conservative: Chao is computed only when the gauge already fires (both
tiers, real overlap), it is a **lower bound** like LP (never a claim of the true total), and
nothing about extraction changes — this is pure measurement. The math is the two-source form
of Chao1: with `f1` facts seen by exactly one tier and `f2 = overlap` seen by both, the
estimate is `distinct + f1²/(2·f2)`. This grounds the gauge in the capture–recapture
literature the `LITERATURE-GROUNDING` survey surfaced (Chao, *Biometrics* 1987).
*Authoritative tracking:* `docs/tasks/RECALL-CHAO-ESTIMATOR.md`.

### `SIGNAL-TABLE-COLUMNLESS-RECALL` — capture signals from column-less signal tables

**What it gives you:** interface signals listed in a bare two-column
`Signal | Description` table — with no Width and no Direction/Source column — are
now captured instead of silently dropped.

**Why it mattered.** Running SpecForge across the CHI spec, the region-accounting
detector flagged the REQ/RSP/SNP/DAT *channel interface signal* tables
(`REQFLITPEND`, `REQFLITV`, `REQLCRDV`, …) as producing **zero** records. The
cause: to synthesize a signal declaration the evidence stage required a *direction*
or a *width*, and these tables state neither in a column — so every row was skipped.
That dropped core interface signals on the floor.

The fix reads the **driver from the description prose**, exactly as a human does:
"Request Flit Valid. *The transmitter sets this signal HIGH* …" → the transmitter
drives it → `output`; "*The receiver sets this signal HIGH* …" → `input`. Direction
is framed relative to the channel transmitter (the subject of a `<X> channel
interface signals` table). Crucially, this *adds* a direction where one is provable
from the text — it does **not** relax the contract that a declaration needs a
direction or width, and it does **not** fabricate one: a signal whose driver is not
stated in prose (e.g. `REQFLITPEND`) is left as an **honest residual** rather than
guessed. So recall improves with no precision cost. *Authoritative tracking:*
`docs/tasks/SIGNAL-TABLE-COLUMNLESS-RECALL.md`.

### `CONSTRAINT-SUBJECT-PRECISION` — a constraint binds the signal it's about, nothing more

**What it gives you:** when SpecForge records "signal X must be stable / asserted /
this value", the subject is the signal the sentence is actually *about* — not signals
that merely appear nearby in a condition, a width column, or an earlier sentence.

**Why it mattered, and how it was found.** This was the **first catch of the new
extraction eval** (`quality/extraction-eval.md`): scoring the LLM constraint task on the
AMBA APB seed, precision came out at 0.5 — half the recorded constraints on the labeled
statements were spurious. Diagnosing them (against gold drafted from the prose) showed
three over-extraction patterns:
- *Condition-clause signals as subjects.* "PWAKEUP must remain asserted **until** PREADY
  …, **if** … PSELx are HIGH" recorded `PREADY` and `PSEL` as constrained, though they
  live in the `until`/`if` clauses — the subject is `PWAKEUP`.
- *Width parameters as signals.* A table row's width column (`USER_RESP_WIDTH`) was minted
  as a constrained signal alongside the real subject `PBUSER`.
- *Cross-sentence sweep.* "… PREADY is asserted … at the rising edge of PCLK. PADDR,
  PWDATA … must be stable" recorded `PCLK` (the clock) and `PREADY` (the handshake signal
  that *changes* to complete the transfer) as "must be stable", though only `PADDR`/
  `PWDATA` are.

The fix tightens subject selection three ways: the condition-clause stripper now also
cuts at `until`/`if` (and at the *earliest* condition marker, not the first listed);
width-parameter tokens (`*_WIDTH`) are never subjects; and subjects are collected only
from the **sentence carrying the constraint verb**, so signals in unrelated earlier
sentences aren't swept in — with a fallback to the whole statement so a true subject is
never lost. The three eval-found cases are locked as regression tests; the full suite
stays green (no true subject dropped). *Authoritative tracking:*
`docs/tasks/CONSTRAINT-SUBJECT-PRECISION.md`.
