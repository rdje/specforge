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

The *text* of each statement is what the Markdown denotes, not its escape syntax. Markdown writes a literal
underscore as `\_`, so the promoted file says `ACK\_O` where the document says `ACK_O`; EvidenceIR reads the
backslash escape as the character it stands for (keeping only `\|`, which is table structure). Before this, every
signal name containing an underscore was unmatchable in prose — Wishbone B4, whose 25 signals all have one, yielded
no signal constraints and no actor relations at all.

Provenance and live inputs deliberately have different existence rules. Section/span/visual references remain
inspectable after cleanup reclaims a normalized source leaf, as long as the repository reference is contained
and any legacy rebase is unique. The upstream `source_ir_path` is a live pipeline input and remains strict: its
artifact must exist before EvidenceIR can be loaded or rebuilt. This preserves historical lineage without making
missing current-stage inputs look valid.

## Executable EvidenceIR authority

EvidenceIR schema 3 is proof-carrying. It first verifies the complete SourceIR schema-3 ledger and keeps those
claims as an exact ordered prefix. It then appends a root claim for every one of its 39 public fields across 11
families, including empty fields, plus a stable claim for every non-empty collection record and alias-map entry.

The proof is executable, not an internal checksum convention. Canonical load, JSON serialization, writing, and
the EvidenceIR-to-SemanticIR seam independently rebuild the deterministic stage from:

- the verified SourceIR artifact and proof;
- the exact normalized current-document Markdown captured in the proof context;
- the exact validated global prior, when one was consulted; and
- an ordered set of closed, typed post-build proposals for approved enrichment or backannotation paths.

Each conclusion is compared byte-for-byte with its current registered replay. The cumulative ledger must contain
the exact verified SourceIR prefix and the current EvidenceIR suffix in order. Editing a field and recomputing its
JSON digest does not help: the current executable replay still disagrees. A stale ruleset, changed prefix,
unauthorized mutation field, missing support statement, proofless current artifact, or future schema rejects.

Post-build commands do not receive a generic "trust this object" escape hatch. NLP enrichment, condition and
constraint extraction, contract extraction, signal relation resolution, register-bit recovery, NLI quality
measurement, and validation backannotation each have a closed mutation kind. They may replace only that kind's
declared fields, must reproduce the complete typed artifact from the verified predecessor, and must extend the
ledger before persistence. Rebuilding EvidenceIR no longer imports hidden NLP state from an older artifact; state
without an explicit replay event cannot silently regain authority.

Schemas 1 and 2 remain available through the inspection API only. They cannot feed SemanticIR. In the local
corpus, the 27 documents with a verified current SourceIR capture have schema-3 EvidenceIR, and no extracted
field changed during the migration that established 24 of them; owned re-ingest returns the rest one document
at a time. The other 51 EvidenceIR files remain historical, proofless, inspection-only
artifacts until their SourceIR inputs can be recaptured under an owned refresh. Proof currency and bundle
RETENTION are separate properties: 24 of the 27 also have a declared retained normalized bundle, so the APB,
AHB and AXI chains are canonical and scoreable while their EvidenceIR replays read unmeasurable
(`RETAINED-BUNDLE-POPULATION-FROZEN`). The currency gate reports their
EvidenceIR-to-SemanticIR replay as unmeasurable rather than pretending it is current. SemanticIR and IntentIR now
enforce the same proof frontier: the 27 reachable chains are current through the product boundary, while the 51
historical later-stage files cannot regain authority from their storage location or old stage-local bytes.
EvidenceIR implementation identity uses the shared
[production-semantic digest closure](../architecture-rationale.md#current-implementation-status-the-invariant-is-not-yet-met),
so test/comment churn is inert while a referenced production verifier dependency changes proof currency.

### Relocating a proof-carrying artifact

The registered replay is taken over the artifact's public fields, and one of those fields is the artifact's own
`artifact_layout`. The proof therefore covers *where the artifact is stored*, not only what it says. Copy an
`evidence_ir.json` to another directory, rewrite only its `artifact_layout`, leave every other byte identical,
and canonical load refuses it:

```text
error: invalid stage artifact: EvidenceIR proof verification failed: registered derivation
'evidence.claim.schema_version.root' output or input topology is stale
```

That is the seal working, not a bug — but read-only work still has to move artifacts. Scoring the extraction
commands, above all, must run them against a copy so the corpus is never mutated. So relocation is a supported
operation rather than a field rewrite: `EvidenceIr::load_relocated_to_artifact_base_root` verifies the artifact
where it currently lives, moves it to `<new base>/<document_key>/evidence_ir.json`, and re-derives the proof for
that location from the same verified SourceIR prefix and the same sealed proof context, mutation chain included.

It cannot launder authority in either direction. The artifact must already verify before it may move, and
writing the relocated artifact re-verifies it against an independent rebuild from its SourceIR, so content that
no longer replays cannot be persisted at the new path. Only the storage location, and the proof that binds it,
differ from the original.

Note what this does *not* affect: moving the whole repository. Persisted paths are repository-root-relative and
resolved against the current root at runtime, so a repository that moves — even to another filesystem — keeps
every seal valid. It is relocation *within* a repository that needs the seam.

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
A document may also state polarity for a **scope** of signals rather than for one (`EXTRACTION-REPRESENTATION-GAPS.17`), and three scopes are read, each citing the rule's own statement:

- a rule over a class of interface signals, Wishbone's *All WISHBONE interface signals MUST use active high logic*, reaches every signal the document places in an interface of the class: named in a *the MASTER interface MUST include the following signals* list, or placed by the role's own signal section, for a role that *WISHBONE MASTER and SLAVE interfaces* makes of the class (23 signals on `2026-09-26`; the SYSCON module's `CLK_O` and `RST_O` are not in it);
- an exclusive statement, AHB's *The reset signal, HRESETn, is the only active LOW signal in the protocol*, states the other polarity for every other signal of the declared catalog (40 signals on `2026-09-26`, including the `HRESET` that AHB's Table A-2 prints);
- a section rule, ADIv6's *All signals that are described in this section are active-high*, reaches every declared signal the statements of its own section name (`CSYSPWRUPACK`; the section's other three wires are rows the catalog does not declare, `SIGNAL-DECLARATION-ROW-DROP.1e`).

A name's spelling is never a scope: AXI's *Lowercase n … denotes an active-LOW signal* is a rule about spelling, which ADR 0037 forbids reading, so AXI's 17 obligations, 14 temporal rules and 1 guard that need a polarity stay listed (`2026-09-26`).
If a polarity phrase is detached from an explicit signal, the statement stays unresolved instead of borrowing an implicit subject. A signal counts as mentioned only when it is written as the document declares it: `#RST_I` or `/RST_I` is inversion notation for a *different*, active-low signal, so Wishbone's explanation that a reset "could be named [#RST_I], [/RST_I] or [N_RST_I]" says nothing about `RST_I` itself; and a prose word that merely spells a catalog entry in another case — `n` for a signal `N`, `in` for a signal `In` — is not a mention either.
The level a signal is *deasserted* to states the opposite polarity: AXI's *When AxMMUATST is deasserted LOW, the Stall flow is used* makes `AxMMUATST` active-high, agreeing with *When AxMMUATST is asserted HIGH, the ATST flow is used*. A polarity phrase must start a word, so `asserted LOW` is never read inside `deasserted LOW`; its end stays open because the text layer fuses the next word on (`active LOWand`) (`EXTRACTION-REPRESENTATION-GAPS.3.g.i`).
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

Measured `2026-09-23`, this now reaches a document SpecForge can rebuild. Once Wishbone B4's underscore names
became readable, its prose yielded 41 actor relations; read by hand, 23 are right, 16 are wrong and 2 are
debatable. Nine of the wrong ones are phrase-shaped actors ("interfaces so", "similar manner", "bus cycle"), and
seven have the wrong direction — *"the MASTER latches DAT_I and negates its STB_O"* was read as the MASTER
*reading* STB_O. The phrase actors are open work (`KG-ISF-COMPLETENESS.1d`), and the wire-protocol golds are unchanged.

Six of the seven wrong directions came from one reading error: a verb's object was taken to run to the end
of its clause. English marks two earlier ends, and the reader now honours both. A **coordinated verb phrase**
starts a new object — in *"latches DAT_I and negates its STB_O"*, STB_O belongs to *negates* — and a **causal
adjunct** names the trigger, not the object: in *"the SLAVE asserts ACK_I … in response to every assertion of
STB_O"*, the SLAVE does not drive STB_O. The coordinated verb has no subject of its own, so it takes the
subject of the verb it is coordinated with (*"the Subordinate asserts LMOPENACK …, and asserts LACREDIT"* keeps
`Subordinate drives LACREDIT`). Only a finite *-s* verb opens such a phrase: after *and*, a bare form is
routinely a noun or a signal name (*"…, and read after the rising edge of clk"* in Avalon, where `read` is a
signal). Measured `2026-09-23`: Wishbone went from 41 relations to 35, all six wrong ones gone and nothing
correct lost; across the whole corpus's stored statements, 34 wrong relations leave and no correct one does. The seventh wrong
direction is a pronoun subject (*"… when it monitors …"*), owned as `.1f`. *Authoritative tracking:*
`docs/tasks/KG-ISF-COMPLETENESS.md` (`.1e`).

Many phrase-shaped actors turned out not to be a question about actors at all. The relation reader recognises
a verb by its spelling, and several relation verbs are also ordinary nouns: *read*, *access*, *writes*. In
*"the value of the read pointer"*, *"a read transaction"* or *"any access to the local cache"*, the word is a
noun, yet each occurrence licensed a relation whose "actor" was whatever phrase stood before it — `value`,
`shareability`, `Any`. English settles it without a word list: a word an article or determiner governs is a
noun, so an occurrence directly after *the*, *a*, *an*, *this*, *these*, *those*, *each*, *every*, *any*, *no*
or *another* no longer licenses a relation. Two words are deliberately left out — *that*, which also opens a
relative clause (*"the Manager that drives HTRANS"*), and *its*, which is also the name of the GIC's ITS — and
a capital *A* is treated as a label, because *"Master A asserts address"* is a real relation. Measured
`2026-09-23` over every document's stored statements, 124 relations leave and none is added; rebuilt with both
versions, 25 of the 27 rebuildable documents are unchanged, and the wire-protocol golds score identically.
One casualty is recorded rather than hidden: AHB's `Subordinate reads HREADY` is true, but its only support was
*"drives the read data bus"*, so it now waits for a sentence or table that actually states it (`.1d.i.a`).
*Authoritative tracking:* `docs/tasks/KG-ISF-COMPLETENESS.md` (`.1d.i`).

A subject phrase can also swallow its own verb. The reader takes up to two words as the subject, so in
*"Arm recommends that sources are programmed to write SPI IDs"* it read the subject as `Arm recommends` and
published `Arm recommends drives SPI`. Cutting the phrase back to `Arm` would be worse, not better: Arm
writes nothing here — the sentence only recommends something about *sources* — and a plausible-looking
`Arm drives SPI` is harder to catch than an obviously broken one. So when a two-word subject ends in a word
shaped like a verb (*recommends*, *sees*, *has*) and more of the clause stands between it and the relation
verb, the relation is **refused**, not re-attributed. The phrase keeps its head in just two cases, where
English makes the head the relation verb's subject:

- **a to-infinitive** — in *"the Manager needs to drive HTRANS"*, the Manager drives HTRANS;
- **coordination with a real verb** — in *"The Manager sees that LMOPENACK is asserted and in the next
  cycle, asserts LRCREDIT"*, the Manager asserts LRCREDIT.

The second case needs proof that the word really is a verb, because *-s* also ends plural nouns: in *"the
Distributor registers in base offset order and provides a reference"*, it is the table that provides the
reference, not the Distributor. So a word counts as a verb only when it is an auxiliary (*has*, *does*,
*was*: *"The device has sampled PREQ LOW … and sets PACCEPT LOW"* keeps `device drives PACCEPT`) or
is followed by *that* and a new clause's own subject (*"sees that LMOPENACK is"*). A *that* followed directly
by a verb (*"encodings that are used"*) is a relative pronoun and proves nothing. The rule has a price, stated
rather than hidden: a two-word plural subject that a relative clause separates from its verb (*"the
Subordinate interfaces, which are optional, drive HREADYOUT"*) is refused too, because there the *-s* word
cannot be told from a verb; no true relation of that shape exists in the corpus today. Measured `2026-09-23` over
every document's stored statements, 50 relations leave, every one read and every one wrong or a duplicate,
and one correct relation (`Manager drives LRCREDIT`) arrives. Rebuilt with both versions, 25 of the 27
rebuildable documents are unchanged; ADIv6 and I²C each lose only wrong relations.
*Authoritative tracking:* `docs/tasks/KG-ISF-COMPLETENESS.md` (`.1d.ii`).

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

## Reading a normative obligation

A requirement sentence carries five separable things — subject, kind, value, condition, negation — and
each is read from some span of the source. The rules that decide those readings, and the documents that
forced each one, are collected in [Reading a normative obligation](obligation-reading.md).

## Typical evidence-level failure modes

The recognisable ways extraction goes wrong, what each looks like in the output, and what SpecForge
does about it, are catalogued in [Evidence-level failure modes](evidence-failure-modes.md).

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

Each of those rules was written after a specific document was read the wrong way, and the record of
how each one was found, measured and verified is its own chapter:
[How Each EvidenceIR Rule Was Found And Verified](evidence-rule-chronology.md).
