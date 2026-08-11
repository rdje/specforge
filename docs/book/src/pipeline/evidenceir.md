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

## Portable lineage and provenance

Evidence lineage follows the same storage/runtime split as SourceIR. The persisted `source_ir_path`, optional
prior-memory path, artifact layout, and repository-owned provenance use repository-relative strings. Loading the
artifact resolves them against the current root, so validators and later stages receive immediately usable
absolute runtime paths.

Text provenance has the promoted Markdown's origin: section anchors and evidence spans may legitimately retain
an absolute path when the caller explicitly supplied external Markdown. Visual provenance is different—its
images and caption sources come from SourceIR's normalized artifact bundle—so those paths remain
repository-owned and serialize relative even when the text source is external. The adjacent
`source_path_origin` label lets old unlabeled artifacts be loaded compatibly without guessing that an unrelated
external path belongs to a moved repository.

Provenance and live inputs deliberately have different existence rules. Section/span/visual references remain
inspectable after cleanup reclaims a normalized source leaf, as long as the repository reference is contained
and any legacy rebase is unique. The upstream `source_ir_path` is a live pipeline input and remains strict: its
artifact must exist before EvidenceIR can be loaded or rebuilt. This preserves historical lineage without making
missing current-stage inputs look valid.

## Typical evidence-level wins

- source/destination table recovery
- table-grounded widths
- table-synthesized signal declarations with provenance back to the originating structured table
- prose-grounded actor relations
- visual-caption semantic hints
- VLM timing-note observations
- optional typed `FigureRegion` records projected from tick-addressed VLM timing observations
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

A timing observation and a usable typed region are separate facts. `EvidenceIR` always preserves the bounded
raw observation. It adds `VisualEvidenceItem.figure_region` only when the observation contains explicit
tick-addressed lane samples; older artifacts load with that optional field absent. Validation reports both the
available count and timing observations for which the typed projection was unavailable, so raw VLM activity is
never mistaken for contract-ready evidence.

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

### Keeping the agent the document actually means

The other place agents enter the model is as the *subject* of a who-drives-what relation — *"the Subordinate
drives HRESP"*. Reading a subject out of running prose is noisier than reading a definition sentence, so the
same relation also gets two structural clean-ups, both keyed off **universal English grammar, never a name
list** — so they apply to any spec, including the 101st protocol the tool has never seen:

- **A phrase fragment is not an agent.** When the captured subject *starts* with a function word or a verb — *"For
  components that support …"*, *"is recommended that a Manager sets …"*, *"ensures the channel is idle"* — it is a
  clause fragment, not an agent, and is dropped before it can become a junk actor in the `IntentIR` (and hence the
  `.isf`). The rule looks only at the *first* word's part of speech, so a real agent that simply happens to carry
  a leading qualifier ("All Managers") is never thrown away.
- **A trailing scrap belongs on the real agent.** When the subject is a real agent noun with a dangling verb or
  adverb caught alongside it — *"Subordinate extends"*, *"decoder also"* — the scrap is stripped and the relation
  re-attributes onto the agent itself ("Subordinate", "decoder"). The relations that were stranded under the
  fragment then merge onto the genuine agent, so the agent is connected to *all* the signals the document says it
  drives, not just some — the model gets **more complete**, not just cleaner.
- **"Both of them" means both.** When the subject names two agents joined by *and* — *"the Subordinate and decoder
  read HADDR"* — the relation is split so *each* agent is connected to the signal, exactly as the sentence says.
  Only the conjunction *and* triggers this (both act); *or* is left alone, because a disjunction is ambiguous —
  attributing the relation to an agent that might *not* act would be a fabrication, and the tool would rather stay
  honest. This is how, for example, an AMBA AHB *decoder* — mentioned only in coordinated sentences — ends up
  correctly connected to the address signals it reads.

The net effect is an agent surface that names every real agent once and connects it to its signals, with the
prose noise removed — and the four wire-based reference specs stay byte-for-fact identical, so the clean-up never
costs a real fact.

#### Where this is still being sharpened: dense descriptive prose

These clean-ups were designed and measured on protocol specs that *define* their behaviour tersely — "the
Subordinate drives HRESP". A measurement across the whole local corpus (`docs/research/agent-identity-prose-class-measurement.md`)
found one doc class where the same subject-reading is noisier than the clean-ups yet catch: **dense,
descriptive prose** — a JEDEC eMMC or DRAM datasheet, or the long combined AMBA AXI+ACE manual, where a
single explanatory sentence ("take *advantage of* the BACKGROUND operation", "the *basic bus* protocol") can
leave a noun *phrase* sitting where a clean agent name should be. The honest current state is that such a spec
can still surface extra phrase-shaped "agents" in its model. Two things keep this bounded and visible rather
than hidden: it never reaches the emitted `.isf` (the adapter lowers signals and behaviours, not the raw actor
list), and it is being closed the same principled way as everything above — by *grammar*, not a list of
forbidden words. The first, measured-safe step has **already landed**: the trailing-scrap rule now also folds a
trailing preposition or auxiliary onto the leading agent ("host *has*" / "host *to*" → "host", "cache *in*" →
"cache"), which a corpus-wide check confirms never touches a single real, heavily-connected agent — on a JEDEC
eMMC datasheet it folds the four "host …" variants back onto one "host" and removes 29 phrase-shaped names, and
the four wire-based reference specs stay byte-for-fact identical. The deeper, rarer phrase-fragment case (a
single explanatory sentence leaving a bare noun like "basic bus") is held as an explicit, scoped open item
rather than papered over — because the alternative, dropping anything that
merely *looks* like a phrase, would also throw away genuine one-word agents like an AMBA *decoder* or
*controller*, and the tool would rather stay complete than look tidy.

## Capturing an interface's sampling and drive-change edge

Some serial interfaces define their electrical handoff in prose rather than a timing table. The ADI
specification says, in one timing-class statement, that the target samples `SWDIO` on the rising edge of
`SWCLK` and that changes to whether the target drives `SWDIO` happen on that same edge. Treating those as two
unrelated text snippets would lose the coupled clocking contract, while a generic temporal key would not prove
which clock signal the statement named.

`EvidenceIR.interface_edge_timings` therefore carries one complete typed tuple:

```json
{
  "actor_name": "target",
  "signal_name": "SWDIO",
  "clock_signal": "SWCLK",
  "edge": "rising",
  "samples_on_edge": true,
  "drive_changes_on_edge": true,
  "supporting_statement_ids": ["statement_1948"]
}
```

The deterministic reader is intentionally generic. It only examines statements already classified as timing
constraints; recognizes universal `When [the|a|an] <actor> samples|drives <signal>` plus explicit
`rising`/`falling` (`posedge`/`negedge`) grammar; and accepts the data and clock only when both are in the
document's declared-signal inventory. A missing edge, undeclared clock, wrong statement class, or incomplete
operation wording yields no record. Sample and drive-change clauses merge only when actor, data signal, clock,
and edge agree. Production code contains no SWD, vendor, actor, or signal-name list.

The extraction manifest exposes this as `interface_edge_timings[timing.interface_edge_prose]`. On a fresh ADI
re-ingest it reports one eligible, one produced, and one kept record. The record is currently an EvidenceIR
surface: its scored extraction fidelity is described in [Extraction eval](../quality/extraction-eval.md), while
projection into SemanticIR, IntentIR, and `.isf` remains an explicit architecture frontier rather than being
implied by the EvidenceIR result.

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

### `CORPUS-COVERAGE.2.47a` — scalar timing records require scalar values

Timing *category* and scalar timing *layout* are separate contracts. A genuine timing table may contain several
variant-specific `MIN`/`MAX` pairs. That table is timing-bearing, but flattening its variants into one
`TimingConstraintRecord` would lose which limit belongs to which mode. EvidenceIR therefore keeps the table in
the timing category and exposes it through unexplained-table accounting until a dimension-preserving record type
exists; it does not collapse the table or pass it to an unrelated extractor.

For a table that can use the current scalar record, each of `min`, `typ`, and `max` must identify at most one
distinct column. Every emitted row must then contain at least one actual value in those columns. A parameter name,
description, or unit without a min/typ/max value is evidence about a row, but it is not a timing constraint.

For example, an instruction-performance table headed `Instruction group | AArch64 instructions | Exec latency |
Execution throughput` has meaningful performance data, but it does not implement the scalar min/typ/max schema.
SpecForge leaves that table as residual evidence rather than emitting a value-empty timing record or pretending
that `Instruction group` is the unit. By contrast, the structurally trapped I2S table described above remains a
classified timing table and still emits its five genuine value-bearing records.

Across the retained corpus, this boundary changes 2,144 timing records across 39 documents into 608 grounded
scalar records: 38 documents change, I2S remains at five, and every survivor has at least one min/typ/max value.
The Cortex-A76 optimization guide's 151 instruction-derived records become zero. *Authoritative tracking:*
`docs/tasks/CORPUS-COVERAGE.md` (`CORPUS-COVERAGE.2.47a`); Knowledge Map
`[[timing-table-structural-authority]]`.

### `CORPUS-COVERAGE.2.48a` — one spanned source cell cannot ground several scalar roles

PDF table conversion can preserve a merged cell by expanding it into one positional record for every covered
column. Those records may contain identical text and retain the original multi-column span. Positional indexing
alone would then make one footer or group heading look like independent parameter, minimum, typical, maximum,
and unit evidence.

EvidenceIR therefore checks source-cell geometry before emitting a scalar timing row. The parameter and every
populated min/typ/max cell must each originate from exactly one column. Blank or `-` scalar cells are still honest
absences, even when a layout span covers them. Optional unit, description, and comment cells do not establish
scalar authority, so their spans remain legal. Equal values in separate one-column cells remain valid too: the
boundary is source independence, not text uniqueness.

In OpenCAPI table 5-11 this keeps the two real receiver-jitter rows and removes the following eight-column note.
Across 80 retained SourceIR documents and 105 timing-classified tables, the rule changes 608 scalar records to
585 by removing 23 spanned informational/group rows in four documents. No document name, vendor, table ID,
parameter spelling, or note prefix participates in the decision. *Authoritative tracking:*
`docs/tasks/CORPUS-COVERAGE.md` (`CORPUS-COVERAGE.2.48a`); Knowledge Map
`[[timing-scalar-rows-require-independent-cell-geometry]]`.

### `CORPUS-COVERAGE.2.48` — keep physical timing without inventing an interface

A physical-link specification can be rich in scalar electrical timing and still provide no synthesizable signal
inventory. OpenCAPI 32 Gbps PHY Signaling is one such case: current EvidenceIR retains 60 timing constraints, but
the source has no signal-description table, direction relation, signal constraint, or register surface that
would declare a digital interface.

Older artifacts promoted four glossary or measurement acronyms—`CDR`, `DDJ`, `DL`, and `DL3`—into one-bit
signals, grouped them into three interfaces, and emitted a four-port `channel.isf` plus a generic table enum.
Current authority removes those synthetic declarations and the untyped phase/gate projections. IntentIR still
keeps seven actors, 21 behaviors, 75 constraints, four assumptions, and all 60 grounded timings; adapter lowering
blocks because no signal is declared instead of fabricating a runnable topology.

Validation keeps the recall boundary explicit: all 39 visuals still need local visual enrichment, 27 normative
statements remain partially structured, two narrative conditionals lack an actor-signal graph, and eight local
timing/conditional source IDs have no typed temporal rule. These are future visual and language extraction
opportunities, not authority to turn physical quantities or acronyms into ports. *Authoritative tracking:*
`docs/tasks/CORPUS-COVERAGE.md` (`CORPUS-COVERAGE.2.48`); Knowledge Map
`[[opencapi-32g-phy-timing-without-interface-topology]]`.

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

### `EXTRACTION-QUALITY-GAUGE.FIELD.2` — message fields are intent too, but they are not signals

Packet and flit protocols (the CHI family is the canonical example) describe two very different kinds of named
things. **Signals** are physical wires — declared in tables like *"REQ channel interface signals"* with names
like `REQFLITV`. **Fields** are portions of the *message payload* that travels over those wires — declared in
tables like *"Request channel fields"* with names like `TxnID`, `Opcode`, or `DBID`. A field has no direction
and no pin; treating one as a signal produces exactly the garbage constraints the extraction-quality gauge
caught on CHI (a `DBID must_be_value` "signal requirement" about something that is not a signal at all).
Until now those field tables were simply invisible: the fields had no typed home, so the only way field names
entered the pipeline was *wrongly*, through prose.

The reader leans on the document's own vocabulary, twice. First, **the table header names what its rows are**:
a table whose name column is titled `Field` / `Field name` declares fields, just as a `Signal`-titled column
declares signals. Second, **the caption names the container**: *"Table B2.2: Request channel fields"* says these
are fields *of the request channel*, and a follow-on page captioned *"Table B2.2 Continued from previous page"*
carries the same table number, so its rows merge into the same container instead of becoming strays. The same
field name can appear in several containers (`QoS` exists in every CHI channel) and each is honestly its own
record. A width is recorded only when the table carries a single, unqualified width column — a per-variant
width (*"Width (bits) ReqS"*) stays an honest blank rather than a guessed pick.

One subtlety makes this safe on *non*-packet documents: register specifications also write `Field` columns
(a register is made of bit-fields). The discriminator is structural and lives in one place — a field-titled
table that also carries register-access vocabulary (`Access` / `Reset` / `Default` columns) belongs to the
register reader, never to this one; and a caption has to anchor its "fields" to a real container word
(*channel*, *packet*, *message*, *flit*, *header*, *frame*, *request*, *response*) before anything is read at
all. Measured over the whole persisted corpus, that combination fires **only** on the packet-protocol family —
CHI recovers **106 fields across its 4 channels**, the CHI chip-to-chip specs 143–189 fields with most widths,
CCIX 47–51 — while every register document (RISC-V Debug, the IOMMU and MMU manuals, NVMe, eMMC) and every
wire-based bus (APB/AHB/AXI, SWD) yields exactly zero *through this field-titled reader* and is otherwise
byte-identical to before (NVMe and the AMD IOMMU manual now contribute fields through the separate
bit-position strategy of `.10b` above — same surface, different table shape). Two tracked
benchmark fixtures lock both directions: fields land in `message_field_records` and never in the signal
inventory, and a register-shaped table never produces message fields even when its caption says "message
fields". This inventory is the foundation the signal-vs-field ontology builds on: with declared fields known,
the entity-typing gate grounds a field *out* of signal constraints deterministically — and the field-constraint
surface below gives field obligations their own typed home instead of dropping them.
*Authoritative tracking:* `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.FIELD.2`).

### `EXTRACTION-QUALITY-GAUGE.FIELD.4` — obligations on fields get their own constraint surface

Knowing that `TagOp` is a message field keeps it *out* of signal constraints — but the specification still
says things about it that an implementer must honor: *"For all other REQ channel messages, the TagOp field is
inapplicable and **must be 0**"* is a real requirement. Dropping it because its subject is not a wire would
trade one extraction error (mis-typing) for another (silent loss). So the evidence layer carries a parallel
surface, **`message_field_constraints`**: typed obligations whose subject is a declared message field. Each
record holds the field name, the containers the document declares that field in (the same field legitimately
recurs across channels — `TagOp` lives in the Request channel, the Response packet, and the Data packet), the
same constraint-kind vocabulary signal constraints use (what a requirement can *say* is the same; what it is
*about* differs), the grounded condition and value, and the statements it came from. A separate surface —
rather than a "field" flag on signal constraints — means every downstream consumer of `signal_constraints`
keeps seeing wires only, by construction.

The discipline is identical on both surfaces: a field obligation must survive the condition-subject guard, the
permissive-frame guard, source-grounded value recovery, and provenance-merging de-duplication, exactly like a
signal constraint. Measured live on the persisted CHI artifact the split is visibly right: the four flit-valid
wires (`REQFLITV`/`RSPFLITV`/`SNPFLITV`/`DATFLITV`) are correctly kept **out** of the field surface — they are
wires, not fields (their *descriptive* "sets this signal HIGH" reading is a separate over-extraction, filtered
by `.3c` below) — the `TagOp`/`PBHA` content rules become field records with their channel provenance (the
twice-stated `TagOp` fact merged into one record carrying both statements), and every wire-based document
(APB/AHB/AXI) carries an empty field surface with its signal results unchanged. One boundary is recorded honestly rather than guessed at: a field *presence*
requirement (*"the MPAM field must be included on the REQ and SNP channels"*) has no slot in the
constraint-kind vocabulary yet, so such sentences currently yield nothing — a candidate future kind.
*Authoritative tracking:* `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.FIELD.4`).

### `EXTRACTION-QUALITY-GAUGE.3c` — a signal's *description* is not an *invariant*

The deterministic extractor reads "`<actor>` sets `<signal>` HIGH/LOW" as a value binding. That is
right for an obligation — "the Requester **must drive** PSTRB LOW" — but wrong when the sentence is
**describing what a signal does** rather than **constraining it**. Two shapes recur across the corpus:

- a **signal-description cell**: *"Request Flit Valid. The transmitter **sets this signal HIGH** to
  indicate when REQFLIT is valid."* `REQFLITV` is a *valid* strobe — it is HIGH when there is a valid
  flit and LOW otherwise; it is emphatically **not** always-HIGH. Reading "must be HIGH" off its
  description is an over-extraction.
- a **timing-diagram walkthrough**: *"At T2, the power controller **sets PREQ HIGH**. The interface
  state is now P_REQUEST."* This narrates one moment of one example waveform; the global obligation,
  if any, belongs in the typed temporal layer, not a flat invariant.

So a logic-level binding is dropped when it is descriptive narration, recognized **structurally**
(universal grammar, no signal or vendor names — ADR 0006): an *action* verb (`sets`/`drives`/`driven`)
— never the static-invariant verbs `tied`/`held`/`pulled`/`forced`, which *do* encode a real
always-at-this-level fact — together with **no** mandatory modal (`must`/`shall`/`required to`) and a
descriptive marker (the phrase "this signal", a timing anchor `T<n>`, or a figure narration
"Figure … shows"). A mandatory binding ("must drive PSTRB LOW") and a static invariant ("tied HIGH")
are always kept.

The gate lives in the deterministic Pattern extractor — the root mint site — so it cleans **both** the
Pattern surface *and* the LLM-primary surface at once (the LLM-primary extractor only re-reads
sentences the Pattern surface already found, so refusing to mint at the root removes them from its
recall universe too). Measured live: CHI drops 13 → 5 signal constraints, removing exactly the eight
`*FLITV`/`*LCRDV` description cells; every wire-based gold gate (APB/AHB/AXI constraints and temporal,
SWD frame/operation/state) stays at 1.000 and `kg-bench` stays green — probe-confirmed gold-safe,
because no wire-doc obligation is phrased as actor-action narration. One honest note on the quality
gauge: its not-entailed *rate* can even rise when these drop, because the NLI judge textually entails
"sets HIGH" ⇒ "is HIGH"; the gauge is a textual heuristic, while the per-item semantic audit (a valid
strobe is not an always-high invariant) is the ground truth — so the cleaner surface is the correct one.
*Authoritative tracking:* `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.3c`).

A companion gate (`.3d`) handles the sibling **relational-value** frame the same probe surfaced: an
*inter-signal/field equality* — "ALLOW_UW **must be equal to the value of** ALLOW_PW", or a bounded
form "a value **less than or equal to the value of** the NVM Set Identifier Maximum field" — has no
slot in the constraint vocabulary ("this equals another operand's value" is not "this is `<literal>`"),
so the value-binding paths mis-mint a garbage `must_be_value` off a condition token. Both constraint
extractors now refuse a sentence carrying a relational phrase ("… the value of …" / "the same value
as …") — a literal binding like "must be equal to **0**" is untouched, because it names no other
operand. Measured live: DTI sheds 15 such records and NVMe 5, with every wire gold gate unchanged.
*Authoritative tracking:* `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.3d`).

A third companion gate (`.3e`) handles the **descriptive-field-cell spurious-subject** frame — the
deepest of the three, and the original "spurious subject" error the quality gauge first flagged. In a
register or structure spec, a field is *defined* by a table cell that narrates what it is: *"Controller
Base Address (CBA): **This field specifies** the 52 most significant bits …"*, *"MemPoolSpcificMemTypeCap
**This field indicates** … 1h: Indicates SRAM Memory Type."* The field's own name sits in front of the
phrase "This field …"; the words *after* it are description — enumerated value meanings (`SRAM`, `DDR`),
protocol names (`CCIX`, `PCI`), even a bare hex literal (`FFFF` from "A value of FFFFh indicates …").
The deterministic extractor would sometimes grab one of *those* trailing tokens as the constraint's
subject and mint a `must_be_*` about a thing that is not a wire or a field at all. The gate is a clean
structural test (universal grammar, no name lists — ADR 0006): inside a *"This field `<descriptive-verb>`"*
cell — `indicates`/`specifies`/`describes`/`contains`/… , never the obligation lead "This field **shall**" —
the subject must appear *before* the phrase, i.e. be part of the field's name. A subject that shows up
only *after* it was lifted from the description, so it is dropped; the field's real mnemonic (which does
appear before, like `CBA`/`SANICAP`/`ELEN`) is always kept, along with any genuine "… shall …"
obligation it carries. Measured live: CCIX sheds 8 such records (the `SRAM`/`DDR`/`NVDIMM`/`HBM`/`CCIX`/
`PCI` enum-name subjects) and NVMe its lone `FFFF`, while every wire-based gold gate (APB/AHB/AXI
constraints and temporal, SWD frame/operation/state) stays at 1.000 and `kg-bench` stays green —
probe-confirmed gold-safe, because a wire spec declares its signals in signal tables, never in
"This field …" cells. *Authoritative tracking:* `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.3e`).

A fourth companion gate (`.3f`) fixes a smaller but sharper *value* error in the same family. When the
extractor reads a value-binding sentence, it looks for a value the document itself used, sitting behind a
phrase like *"shall be …"*. It used to match that value as a plain substring — which means an *alphabetic*
value such as `NO` would also match inside a longer word: *"this field shall be **no**n-zero"* wrongly
became the fabricated fact *"SANICAP must be NO"*, when the real obligation is *"shall be non-zero"*. The
fix is a one-line-of-reasoning rule (universal grammar, no name lists — ADR 0006): an alphabetic enum
value must match a **whole word** (the character after it cannot continue a word), so `NO` no longer hides
inside `non-zero`. A purely numeric value keeps the looser match on purpose, so a radix-suffixed literal
like *"shall be `0`h"* still binds correctly (a blanket rule would have wrongly dropped those genuine
`0h` obligations — measurement caught that before it shipped). The effect is surgical: exactly one
fabricated record leaves the corpus (NVMe's `SANICAP NO`), every wire-based gold gate stays at 1.000
(the value binder produces byte-identical results on the wire specs), and `kg-bench` stays green —
honest residual over a fabricated fact. *Authoritative tracking:*
`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.3f`).

A fifth companion gate (`.3g`) fixes a related *subject* error. Register and structure specs constantly
cross-reference *another* register's field with dotted notation — *"Buffer Address (BADD): … aligned to
the memory page size (**CC.MPS**). The least significant bits shall be 0."* That sentence carries a real
obligation about `BADD`, but the extractor would also pick up `MPS` (the trailing half of the
cross-reference `CC.MPS`) as a second subject and mint a constraint about it — even though `MPS` is just
a pointer to a *different* register's field, not the subject of this cell. The gate is a clean structural
test (universal `Reg.Field` cross-reference grammar, no name lists — ADR 0006): a subject is dropped only
if *every* place it appears in the text is immediately preceded by `"<identifier>."` — i.e. it is only
ever a dotted reference, never a name in its own right. A subject that shows up standalone even once (its
own declaration) is always kept, so the cell's genuine subject (`BADD`) is untouched. A tempting
shortcut — "treat any all-hex-looking token as a literal" — was tried and *rejected by measurement*,
because real field names like `CBA` (Controller Base Address) and `BADD` happen to be spelled with only
hex letters. The effect is again surgical: NVMe's `MPS` co-subject leaves, the wire specs are byte-
identical (zero wire records touched), and the gold gates stay at 1.000. *Authoritative tracking:*
`docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.3g`).

A sixth companion gate closes the last member of this family: **a passive obligation's subject has to be named
before the obligation**. English puts the subject of *"… must/shall be …"* in front of the modal, so anything the
sentence only reaches *afterwards* is an agent, an aside, or a later mention — never the constrained thing. In
*"The endpoint **shall be held in reset** by an out-of-band OpenCAPI Device Enable (OCDE) signal"* the obligation
is about the endpoint; `OCDE` names the signal that does the enabling, and `CAPI` is not even a word — it is the
uppercase tail of *OpenCAPI*. In *"Lane reversal … **shall be compatible** with all supported lane widths"* the
candidates (`DLX`, `CAPI` again) appear two sentences later. Both deterministic paths could reach such a token —
the pattern path through its full-text fallback, the value path through its whole-statement scan — and mint a
`must_be_*` about something the document never constrains. The gate keeps a candidate only when it occurs, at word
boundaries, *before* the sentence's first `must`/`shall` (optionally *not*/*never*) `be`/`remain`. Three things are
deliberately left alone: an **active** obligation, which states its object after the verb (*"must drive PSTRB
LOW"*, *"must have its WSTRB input tied HIGH"*), carries no passive lead at all; a **table row**, whose other
cells legitimately name the subject its obligation cell then constrains; and every ordinary passive constraint,
single- or multi-signal (*"PADDR, PWDATA, and PWRITE must be stable when PSEL is asserted"*), whose subjects all
precede the lead. Measured live over the whole corpus: 26 fabricated subjects leave (a WISHBONE sentence that had
been minting a constraint about the word `MUST` itself, protocol names like `WISHBONE`/`PCI`/`DTI`, later
conditions like `HRESP` and `CKE`, non-subject fields like `OAS`/`DID`/`IODIR`), the wire golds stay at 1.000,
`kg-bench` stays green, and one emitted `.isf` improves — the I2C target loses a false `SCL = 1` rule that a
timing footnote had produced. *Authoritative tracking:* `docs/tasks/CORPUS-COVERAGE.md` (`.2.50a`).

One last member of the family (`.3h`) closes the mirror-image error: a **value** wearing a subject's clothes. A
value-binding phrase puts the bound literal *after* it — *"all entries … shall have the Controller ID field **set
to FFFFh**"* — so the obligation is about the Controller ID field and `FFFF` is merely what it is set to. The
value-binding path had already learned to skip the value *it* bound, but here the record's own bound value came
from a different phrase later in the paragraph, so nothing stopped the hex literal from becoming the subject. The
gate keeps a candidate unless *every* place it appears sits directly behind a value binder (*set to*, *cleared
to*, *written to*, *programmed to*, …); one standalone mention anywhere and it is treated as a real subject, the
same "standalone wins" rule `.3g` uses. The tempting shortcut — "anything spelled with only hex letters is a
literal" — stays rejected by measurement, because real field names like `CBA` and `BADD` are spelled that way.
Measured live: exactly one record leaves the corpus, and no wire-protocol record moves at all.
*Authoritative tracking:* `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (`.3h`).

### `EXTRACTION-QUALITY-GAUGE.0` — the artifact carries its own quality measurement

Every surface above is about extracting more, and extracting it correctly. This one is about
**knowing how correct the result actually is** — per document, automatically, and in a way that
survives the terminal session that measured it. The EvidenceIR now carries an optional
`extraction_quality_gauge` record: the result of one NLI pass that asked, for every signal
constraint, whether the constraint's own source sentence *entails* it. The record holds the
judging model, how many constraints were entailed / not entailed / abstained-on, and the exact
ids of the not-entailed ones — so a reviewer opens the artifact and goes straight to the
suspect items.

The honesty rules are strict, because a quality number is exactly the kind of thing that goes
stale or gets gamed:

- the gauge is **measurement metadata, never extraction truth** — persisting it changes no
  constraint and no canonical fact, and the NLI oracle is reported everywhere as a *noisy*
  estimate;
- **rebuilding the EvidenceIR drops it** — a new constraint surface requires a new measurement
  (`converge` takes one automatically after every stabilized run);
- a pass that **labeled nothing is not persisted** — a dead model never overwrites a real
  measurement with a vacuous one;
- and `validate` flags a gauge whose surface has **changed underneath it** as stale instead of
  reporting an old number as current.

`nli-verify` writes it, `converge` refreshes it after stabilization, and `validate` reports it
model-free — see the [validation chapter](../quality/validation.md) for the metrics and
warnings. *Authoritative tracking:* `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (leaf `.0`).

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
