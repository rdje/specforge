# SemanticIR

`SemanticIR` is where evidence is assembled into typed protocol meaning.

## What belongs in `SemanticIR`

- actors
- interfaces
- actor-relative ports
- signal connectivity
- typed transaction phases; legacy generic section phases remain schema-compatible audit data
- invariants
- typed conditional/temporal rules; legacy generic whole-statement gates remain schema-compatible audit data
- timing constraints
- temporal rules
- losslessly projected serial-frame, protocol-operation, protocol-state, and interface-edge observations
- semantic candidates
- semantic arbitration
- residual decisions

This is the stage where the pipeline begins to act like a protocol compiler rather than a document extractor.

## Canonical authority and proof

Current SemanticIR is schema 2. A JSON object matching that schema is not authoritative by itself. Canonical
load, serialization, persistence, and IntentIR construction first verify the complete cumulative EvidenceIR
ledger, rebuild SemanticIR with the current registered implementation, and compare every public field exactly.

The proof covers all 49 public fields across 12 families. Every field has a root claim, including absent optional
values and empty collections; every populated collection also has stable per-record claims. The upstream
EvidenceIR ledger is retained as an exact ordered prefix. A registered stage-replay node depends on every
upstream claim, and every SemanticIR claim depends on its exact field or record replay. This represents the full
contributor graph without repeating every upstream premise on every output.

Genuinely carried evidence is checked more directly. A carried collection cites the corresponding EvidenceIR root, and a
carried record cites an EvidenceIR record with the same exact bytes. Semantic filtering may change array indices,
so record matching uses the conclusion digest rather than assuming that source and destination indices remain
equal.

The distinction is structural, not nominal. `timing_constraints`, `signal_constraints`, and `conditional_rules`
are evidence projections because SemanticIR may filter ungrounded records and may add records derived from typed
visual evidence. They therefore use the registered `semantic.evidence_projection` family and cannot claim
lossless-carry authority. Serial-frame fields, protocol operations/states, interface-edge timings, and register
records remain the exact-carry family.

Optional corpus memory remains advice, not document truth. When used, its exact payload is recorded as an
identity-independent validated prior grounded in the current verified EvidenceIR. It can contribute only to the
registered semantic families that consult prior guidance; it cannot decide document identity or bypass replay.
The only production post-build mutation is validation backannotation, whose closed authority covers
`validation_reports` and no semantic field.

Schemas older than 2 are available only through the inspection API. They cannot feed IntentIR and are never
wrapped in synthetic proof. A current-schema artifact missing proof, carrying a stale ruleset, or differing from
the registered replay fails closed. Repository-owned lineage paths are resolved and normalized before proof
comparison, so moving the repository does not invalidate otherwise identical authority; ambiguous or external
rebasing still rejects. In the retained population, 24 chains currently have verifiable EvidenceIR
and SemanticIR; 54 historical chains remain explicitly unmeasurable until their upstream capture can be
re-ingested.

## Why this stage exists

`EvidenceIR` can tell you what the document said and where it came from.
`SemanticIR` is where the tool starts asking:

- who drives this signal?
- who reads it?
- what role does it play?
- what timing behavior is being asserted?
- is the meaning decisive or contested?

That means `SemanticIR` is the bridge between raw evidence and canonical intent.
It is where multiple evidence fragments are combined into a typed world model.

## Important semantic principles

### Graph-first direction

Direction should come from structural actor-signal relations when possible, not from flat heuristics alone.

The dedicated [Actor Connectivity And Graph Direction](../domain/actor-connectivity.md) chapter explains how `Drives` / `Reads` relations become actor-relative ports and signal connectivity.

### Meaning before spelling

Handshake recovery uses grounded role evidence; raw identifier shape supplies no fallback.

The dedicated [Handshake And Semantic Roles](../domain/handshake-semantics.md) chapter explains the observation, candidate, arbitration, and consensus surfaces behind that rule.

### Explicit conflict surface

Competing semantic candidates should stay visible.
Unsafe forced winners are worse than honest contestation.

Interface-shape conflicts follow the same rule.
If declarations disagree about a signal's direction or width, `SemanticIR` records the conflict and keeps the canonical hint unresolved; later repeated declarations cannot resurrect the old direction or width just because they appear again.
The KG fixture harness can now assert those `interface_signal_conflicts` directly by signal, conflict kind, and preserved observation values, so this contract is executable rather than only described as a validator count.

### Declared records stay canonical

When a signal is already explicitly declared, a single-signal prose sentence should enrich that declared record rather than minting a second heuristic interface record.
For example, `CS_N is asserted when LOW` can attach active-low polarity to the declared `CS_N` surface, but it should not create a duplicate low-confidence `CS_N` interface just because the same signal was mentioned again.
The same rule applies to polarity-only co-mentions of declared signals: `CS_N and WE_N are active LOW signals` should enrich the declared `CS_N` / `WE_N` records, not create a second low-confidence interface group that double-counts polarity coverage.

Declared signal records also keep table support when the declaration was synthesized from a structured signal table.
`InterfaceSignalRecord.supporting_table_ids` records the `SourceIR` table ids that backed the declaration, so `SemanticIR` and the carried `IntentIR` can explain that a canonical signal came from a specific signal-description table rather than from free-floating prose.
`specforge validate` reports this as `with_table_support`, giving users a compact coverage view while leaving exact signal-to-table correctness to canonical IR inspection and `kg-bench` expectations.

Structured register records follow the same provenance principle. SemanticIR clones register-level
`access_type`, per-field access, `supporting_table_ids`, and `supporting_statement_ids` from EvidenceIR without
reinterpretation. Register access is never inferred from a field and a table id is never relabeled as a
statement id.

Timing constraints follow the same lossless boundary. A table-derived `TimingConstraintRecord` keeps its
explicit row unit or closed-grammar caption fallback and its direct `supporting_table_ids`; SemanticIR neither
normalizes the source unit spelling nor converts a table id into statement provenance. Timing observations
recovered from other modalities remain valid with an empty table-support list.

Applicability is carried losslessly too. A decibel-domain timing/limits observation remains in
`SemanticIR.timing_constraints` with its source values, table authority, reason, failing boundary, and replay
route, but only canonical timing constraints enter temporal-rule derivation. Validation uses the same boundary:
a document containing only deliberately non-applicable physical observations is not warned that an executable
temporal-rule surface is missing. This separates capture completeness from executable-digital authority without
turning a physical measurement into a silent drop.

### Heuristic grouping needs positive authority

Statement-level signal co-mentions can group only signals that are already authoritative. `SemanticIR` first
establishes authority from formal `Signal X is ...` declarations and the document system contract, then
intersects candidates with that surface. Deontic prose can enrich those declarations, but cannot make an
arbitrary token a signal. A declaration-free document therefore produces no named interface merely because its
prose resembles signal behavior.

This fail-closed rule matters for prose-heavy and table-heavy documents. A row such as
`| F0 84 | 01 | A0 | A4 | D5 |` may contain hardware-shaped uppercase tokens, but without a typed declaration
there is no evidence that those tokens are top-level interface wires. Before this boundary was enforced, 21
retained documents accumulated 5,527 low-confidence interfaces / 18,397 records and fed 4,060 signals into ISF
adapters. The same documents now produce no interface because none satisfies the positive behavior grammar.

For example, `request_flag must remain asserted until accept_flag is observed` enriches the pair only when the
document has declared both identifiers as signals. Without those declarations it remains source behavior, not a
fabricated wire surface. A raw encoding row, ordinary prose, or an actor relation does not declare a wire.

Formal declarations are unaffected because they take the explicit interface path. System clock and reset are
also preserved: the system contract inserts them into the authority set and emits an explicit document
interface with typed input direction and width one. Actor/signal relations can describe use of an authorized
signal, but a relation alone is not a signal declaration.

### Grounding does not switch off when there is nothing to ground against

`SemanticIR` promotes an `EvidenceIR` signal constraint only when its subject signal is one the document
declares, and a conditional rule only when its consequent signal is declared — or when the rule names no signal
at all, which is how a genuine system-level behavioral rule is preserved.

The rule that matters most is what happens when a document declares **nothing**. A document with no signal
catalog has no authority to check a named subject against, and the tempting reading is that a filter with an
empty catalog should stand aside. `specforge` does the opposite: one predicate governs every document, and an
empty catalog simply grounds no named subject.

That is deliberate, because the alternative inverts the guarantee. Under a stand-aside rule the filter would be
strongest on documents that *have* signal authority and absent on documents that have none — exactly where an
ungrounded record is least likely to be real. The effect was measurable: a document that declared one real
signal filtered every prose-derived record against that one-element catalog, while a near-miss document that
declared none promoted its entire prose-derived record set as canonical. Across the corpus that admitted
subjects such as `NOTICE`, `PDF`, `IMPLEMENTATION`, `UNPREDICTABLE`, `MUST`, and `FFFF` — document metadata,
boilerplate, English modals, and table noise, none of them wires.

A refresh made the inversion visible in the most counter-intuitive way available: removing two false signals
from a document emptied its catalog, and its promoted conditional rules went **up**, from zero to two.

**Nothing is deleted.** The records stay in `EvidenceIR` with full provenance — that is the honest capture
layer, and this rule governs promotion into canonical `SemanticIR`, not what the evidence stage may observe.
What is withheld is canonical authority, and the withholding is stated rather than silent (see
[Residual decisions](#residual-decisions)).

### Generic section phases are legacy compatibility data

The schema still contains `phases`, but current SemanticIR producers leave it empty. The historical producer
treated a section topic as one semantic phase when its title or prose contained broad words such as `reset`,
`read`, `write`, `mode`, `timing`, `when`, or `after`. The record then said only `semantic phase derived from
section <title>`; it did not identify a phase name, order, entry condition, signal membership, or actor role.

An exact 79-document audit found 11,286 retained records. Sentence-level sequencing words alone authorized
8,106. Of the remaining 3,180 title-authorized records, 720 were `Reset value`; common others were registers,
timing diagrams, results, and transaction topics. Only six titles literally used `phase`, and five were CAN
multi-segment/error terminology or a Wishbone glossary definition. The sole clean SWD heading added only a
tautological section summary, while its actual `data` phase was already present in the typed inventory.

Old SemanticIR artifacts remain loadable and round-trippable with populated `phases`, preserving provenance for
audit. Current IntentIR builders deliberately give those legacy records no behavioral or actor authority. This
prevents a stale artifact from broadcasting a section summary to every actor or preserving an otherwise
pure-inferred actor.

The active phase surface is `transaction_phases`. Its bounded `<qualifier> phase` recognizer reads explicit
prose, keeps statement provenance, and limits signal membership to declared signals. A surviving qualifier is
not sufficient by itself: at least one occurrence must positively use the phrase as a phase—for example as a
heading, a numbered phase, the subject/object of a local predicate, or under a temporal/naming construction.
This rejects `edge phase errors`, `dynamic phase tolerance`, and passive `are called phases` without any
protocol-name list. Once `address` is authorized by `During the address phase ...`, every grammatical
`address phase` mention remains provenance and can still contribute declared-signal membership.

See the [transaction-phase precision measurement](../../research/transaction-phase-qualifier-precision-measurement.md),
the [sentence-authority measurement](../../research/semantic-phase-authority-measurement.md), and the
[retirement measurement](../../research/generic-section-phase-retirement-measurement.md).

### Generic whole-statement gates are legacy compatibility data

The schema still contains `gates`, but current SemanticIR producers leave it empty. The historical producer
copied an entire retained statement into one `GateRecord` whenever the prose contained the word-bounded cue
`if`, `when`, `unless`, `while`, `after`, `before`, or `until`. The record preserved statement/section provenance
and interface co-mentions, but it did not parse an antecedent, consequent, effect, actor role, signal action, or
executable operation. A cue-bearing sentence was therefore evidence *about a possible condition*, not a typed
canonical gate.

An exact replay over all 79 retained SemanticIR artifacts found 27,168 such records in 77 documents. Only 2,676
overlapped an independently typed conditional or temporal rule, and 24,492 did not. The old IntentIR projection
turned the source sentences into 21,206 deduplicated behaviors assigned 642,401 times across every retained
actor; 5,974 interface co-mentions also became generic constraints with 366,087 interface assignments. Removing
only that authority changed no rendered ISF source, renderability decision, lowering status, or executable count.

Old SemanticIR artifacts remain loadable and round-trippable with populated `gates`, preserving their provenance
for audit. Current IntentIR builders deliberately ignore those legacy records, so stale artifacts cannot restore
the all-actor projection. Conditional meaning is not discarded: the source statement remains in EvidenceIR,
technical obligations can remain invariants and IntentIR constraints, and independently grounded
`conditional_rules[]` and `temporal_rules[]` keep their typed operands. See the
[generic-gate retirement measurement](../../research/generic-gate-authority-retirement-measurement.md).

### Legal conditions are evidence, not protocol intent

`EvidenceIR` keeps source-grounded copyright, license, warranty, liability, patent, and administrative text so
the document remains auditable. `SemanticIR` does not promote that material into actors, phases, invariants,
gates, interfaces, or decompositions merely because it contains words such as `if`, `when`, `while`, or `until`.
This distinction matters because PDF front matter can appear under generic headings such as `Approved`; a
section-title denylist alone cannot establish semantic authority.

The boundary uses compound legal context rather than isolated vocabulary. For example, copyright plus document
or notice context, warranty/liability plus product or specification context, and permission plus revocation or
successor context are excluded from semantic assembly. A technical access-right field, write permission,
protocol version, reliability condition, register named `license`, or literal ASCII string `Copyright` remains
eligible. The rule does not name a vendor, specification, document key, source sentence, or section index.

As a result, a sentence about permissions remaining valid *while* a specification is current stays in
EvidenceIR but cannot become engineering intent. A real condition such as `While READY is low, VALID must remain
asserted` remains a technical invariant and downstream constraint; where extraction establishes typed operands,
the independent conditional/temporal rule surfaces preserve them as well. It does not need a generic
whole-sentence gate to survive.

### Administrative workflows are evidence, not device behavior

Organizational process receives the same evidence/authority separation. Reference lists, instructions for
submitting a product, email review, listing and certification-mark requests, institutional escalation, and
third-party test-lab administration remain in EvidenceIR for audit. They do not create SemanticIR phases, gates,
actors, invariants, or downstream IntentIR behaviors.

The boundary is structural rather than vocabulary-only. A request must pair with an administrative channel or
listing/mark object; conflict resolution must pair with institutional escalation; test-lab prose must describe
certification-program administration; and attestation must name an organizational actor. This preserves
engineering uses of the same words: a Page Request and its response, an electrical contact, an arbiter resolving
a write conflict, a measured test report, and a certified controller asserting a signal all remain eligible.

The real OpenCAPI Certified Definition demonstrates the distinction. Its 167 evidence statements are unchanged,
while five administrative phases, six gates, and 11 of 15 behaviors disappear. The four surviving behaviors are
device/host product-compliance contracts, not application or listing procedure. The rule contains no vendor,
consortium, document-key, organization-name, exact-sentence, or single-token exception.

### Negative-knowledge cautions

If a carried conflict or residual packet shape matches learned negative knowledge, `SemanticIR` validation may report `negative_knowledge_prior_matches`.
That is a caution surface only.
It can also emit `semantic_negative_knowledge_rescan_guidance` plus rescan/corroboration metrics so later tooling knows which current surfaces deserve targeted re-extraction.
It does not remove the current conflict, remove the residual, change arbitration, or promote a fact from memory.

### Infrastructure is special

Clocks and resets are treated as infrastructure semantics, not ordinary protocol edges.

That matters because a clean semantic model should not flatten:

- clock distribution
- reset discipline
- system-contract infrastructure

into the same category as ordinary payload or handshake signals.

The dedicated [Clock And Reset Infrastructure](../domain/clock-reset.md) chapter explains that domain boundary in more detail.

Current `SemanticIR` therefore has two related surfaces:

- `signal_connectivity`, where infrastructure signals carry `system_clock` or `system_reset` connectivity class
- `infrastructure_signals`, where source status, recovered distribution status, and explicit topology hints are recorded without inventing ordinary protocol producers

Topology hints are intentionally bounded.
They can preserve current-document evidence for gated clock branches, reset synchronizer stage counts, or reset-tree targets, but they do not claim a complete physical tree proof.

## Residual decisions

Residual packets exist because the project would rather preserve unresolved ambiguity than fabricate a clean but wrong canonical answer.

They also carry the *refusals*. When the grounding rule above declines to promote a record,
`SemanticIR` emits a single `semantic_ungrounded_records_not_promoted` packet for that document rather than
letting the records disappear quietly. The packet states how many signal constraints and conditional rules were
refused, how many signals the document actually declares, and a sorted, capped sample of the undeclared names:

```text
packet_id:  semantic_ungrounded_records_not_promoted
question:   Should records naming a signal this document never declares carry canonical authority?
why:        16 signal constraint(s) and 78 conditional rule(s) name a signal that is not in this
            document's declared-signal catalog (0 declared), so SemanticIR did not promote them:
            ALLOW, AMBA, ATTR, BYPASS, COMB, DTI, EL2, FAULT, FLOW, IMPLEMENTATION, IPA, MECID,
            and 29 more. …
```

The sample is bounded on purpose — one corpus document refuses 216 rules, and an unbounded list would ride into
every `IntentIR` and adapter artifact that carries residual decisions. A document whose records are all grounded
gets no packet at all, so this surface stays proportionate rather than becoming ambient noise. The two candidate
interpretations spell out the trade honestly: require a declared subject, or promote the name on the strength of
the prose alone.

Reading the packet is also the fastest way to find a document whose **signal catalog was never captured**. If
the names it lists look like real wires rather than boilerplate, the gap is upstream in signal extraction, not
in this filter.

## What this stage is trying to resolve

`SemanticIR` is where the pipeline tries to answer questions such as:

- which actor really drives this signal?
- which actor reads it?
- is this signal request-like, accept-like, data-like, or still ambiguous?
- which constraints become typed temporal rules?
- which conflicts are genuine and which are only apparent?

These are the kinds of questions that are too semantic for `EvidenceIR` but still too provisional to be flattened straight into final intent.

The dedicated [Temporal Semantics And Timing](../domain/temporal-semantics.md) chapter explains the typed temporal-rule surface in more detail.

## Why arbitration lives here

Meaning recovery is often not binary.

The same signal may accumulate:

- multiple semantic-role candidates
- multiple source modalities
- conflicting prose and table evidence
- alias-dependent or fallback-only interpretations

`SemanticIR` keeps that competition visible through:

- candidates
- consensus
- arbitration
- grounding strength
- residual decisions

That is much safer than pretending every signal already has one obvious final role.

## What a good `SemanticIR` artifact looks like

A good semantic artifact should have:

- graph-backed direction wherever the evidence supports it
- typed semantic roles grounded in observations rather than spelling alone
- actor-relative ports and connectivity that stay inspectable
- temporal rules that keep guards, phases, and conflicts explicit
- ambiguity preserved honestly instead of hidden behind forced simplification

When those observations come from captions or VLM-enriched diagrams, the dedicated [Multimodal Evidence And Visual Grounding](multimodal-evidence.md) chapter explains how that visual provenance is preserved.

For a typed timing `FigureRegion`, `SemanticIR` first grounds every lane against the document's known signal
catalog, converts the region to a `PartialTrace`, generalizes conservative figure-provenance `ActorContract`
candidates, and retains lowerability only after the trace verifier returns `Pass`. The candidates then enter the
same fusion and fidelity path as prose-derived contracts. This is a production builder path; current corpus
artifacts remain empty until enrichment supplies timing notes.

## Portable lineage paths

`SemanticIR` keeps two representations of repository-owned paths. A loaded or newly built Rust value exposes
absolute `input_artifact` and artifact-layout paths rooted at the repository's current location, so validators
and downstream builders can open them directly. Its JSON stores those same values relative to the repository:

```text
generated/evidence_ir/<document_key>/evidence_ir.json
generated/semantic_ir/<document_key>/semantic_ir.json
```

The loader first verifies that the outer artifact really is `semantic_ir`, then resolves its internal paths.
Legacy absolute paths from a retired repository root rebase only when exactly one present in-repository target
matches; ambiguous or escaping values are rejected. Reserializing a successfully loaded legacy artifact emits
the relative form, so no retired workstation root propagates downstream.

## Why this stage matters so much

If `SourceIR` is where structure is preserved and `EvidenceIR` is where grounded hints are harvested, `SemanticIR` is where the project either becomes trustworthy or starts to hallucinate.

That is why so many truthfulness-hardening slices land here:

- handshake-role arbitration
- polarity-aware temporal comparison
- infrastructure handling
- conflict surfacing
- graph-first direction recovery

This is the main semantic safety boundary before canonical intent.

## Lossless frame and protocol observations

Four evidence-grounded protocol collections now cross into `SemanticIR` unchanged:

- `serial_frame_fields`
- `protocol_operations`
- `protocol_states`
- `interface_edge_timings`

This projection is intentionally lossless rather than interpretive. Record order, ids, optional values, and
every `supporting_statement_ids` entry are cloned exactly from EvidenceIR. The fields are serde-defaulted and
omitted while empty, so SemanticIR artifacts written before this addition still load and documents without
these facts keep their existing JSON shape.

The schema is input-neutral. A frame field carries an opaque document-stated `name`, optional bit extent,
optional source-stated `phase_name`, optional `{source_actor, destination_actor}`, source order, and provenance.
An operation carries optional source-stated `branch_label` and `operation_name`, an explicit `phase_count`,
optional source-stated phase names, and provenance. No fixed response values, phase enum, actor-role mapping,
protocol identity, or signal spelling is part of the production type.

EvidenceIR schema 3 is the canonical authority for these records. SemanticIR construction first verifies the
complete executable EvidenceIR derivation, including its exact SourceIR proof prefix, before it can read any
record. EvidenceIR schemas 1 and 2 remain available only through the inspection API: they cannot feed SemanticIR,
and re-ingestion from retained or newly recaptured SourceIR is required to create current authority. Future
schema or proof versions fail closed.

The records do not claim more semantics merely because they crossed a stage. In particular, a protocol-state
observation is not a transition graph, and a serial-frame field is not automatically an executable transaction.
SemanticIR preserves the source-backed structure so later stages can make an explicit lowering decision without
re-reading EvidenceIR or weakening provenance. ADR 0016 forbids inventing missing transitions, guards, initial
state, encodings, values, ports, activation conditions, or storage targets.

`specforge validate <semantic_ir.json>` exposes the boundary directly:

```text
=== Serial / Protocol Surface Projection ===
  serial_frame_fields: N
  protocol_operations: N
  protocol_states: N
  interface_edge_timings: N
```

At this delivery point, these collections stop at SemanticIR; IntentIR carry-through and adapter residual
accounting are the next independently committed slices. That distinction keeps a successful SemanticIR
projection from being mistaken for complete `.isf` lowering.

## Closed task trees — how each was implemented and verified

### `R6-SEMANTIC-HARDENING` — close mutation-testing gaps in `semantic.rs`

`semantic.rs` is the canonical actor/signal graph + conflict
surface; mutation testing identified branches that survived
mutation without test detection. This tree closed those gaps
systematically, again per-symbol so a missed mutant carries a
clear localization. Verified by cargo-mutants delta-to-zero on
the targeted symbols + the full `scripts/run_ci.sh`.
*Authoritative tracking:* `docs/tasks/R6-SEMANTIC-HARDENING.md`.

### `DEMPSTER-FUSION-COMBINER` — when sources agree, trust grows

When the same protocol fact is recovered from more than one place — say
prose *and* a timing table both say "the completer drives `PREADY`" —
`specforge` fuses those contracts into one. The question is: what
confidence should the fused contract carry?

The old rule was *the minimum* — only as confident as the **weakest**
source. That is safe, but it throws away something real: **agreement is
evidence.** Two independent sources saying the same thing should leave
you *more* sure than either alone, not merely as sure as the shakier one.

`DEMPSTER-FUSION-COMBINER` fixes that with **Dempster's rule of
combination** (Dempster, 1967), the classic way to fuse independent
belief. Each confidence becomes a belief mass (High `0.9`, Medium `0.7`,
Low `0.5`), and independent agreeing sources combine to `1 − ∏(1 − mᵢ)`,
which is always at least the strongest source. So two **Medium** sources
that agree now fuse to **High** (`1 − 0.3·0.3 = 0.91`); two **Low**
sources fuse to **Medium**. A single source is unchanged, and **High
never inflates past High**.

Crucially, this boost applies *only when sources agree*. If they
disagree (one says drive `1`, another drive `0`), `specforge` does not
average them into a false consensus — it routes the conflict to a
residual decision, exactly as before, and keeps the conservative
confidence. Corroboration is for agreement; honesty is for conflict.
(Dempster's full machinery also tracks a *conflict mass* for
partially-conflicting evidence, but that case cannot arise here: the
fusion separates agreement from disagreement up front, so the conflict
mass is always zero on the path that combines confidence — noted so the
math is not mysterious.) *Authoritative tracking:*
`docs/tasks/DEMPSTER-FUSION-COMBINER.md`.

### `SEMANTIC-EMPTY-CATALOG-FILTER` — the grounding filter no longer switches itself off

**The defect.** Both grounding filters — signal constraints and conditional rules — were wrapped in the same
guard: *if the document declares no signals, promote everything unchecked.* The intent was generous ("there is
no catalog, so do not drop records unnecessarily"), but the effect was an inversion. The filter was strongest on
documents that had signal authority and absent on documents that had none. A document declaring one real signal
filtered every prose-derived record against that one-element catalog; a document declaring zero filtered
nothing. It was found the hard way: a refresh that retired two *false* signals emptied a document's catalog and
its promoted conditional rules rose from **0 to 2**, both of them prose noise (`PWR` cut from `PWR_GOOD`, `OPEN`
cut from `OPEN_CAPI`).

**How the blast radius was bounded before anything changed.** A read-only census over all 78 persisted
`SemanticIR` artifacts found 33 empty-catalog documents, 29 of them riding the unfiltered branch with **1,423
conditional rules and 100 signal constraints** promoted unchecked. Every emitted `.isf` was then checked against
that set: all 44 come from populated-catalog documents, and an empty catalog blocks the adapter on
`no signals declared in interface` before any rule renders. So the defect polluted canonical `SemanticIR` /
`IntentIR` and the surfaces reading them — never the product boundary.

**The fix, and the one thing that made it non-obvious.** Deleting the special case is the whole repair: one
predicate, and an empty catalog satisfies no named subject. But measuring the *populated* branch first changed
what "no regression" could mean — 41 of the 45 populated-catalog documents were **already** dropping records
silently (1,230 rules, 47 constraints). Applying the same rule uniformly therefore could not leave them
byte-identical; it necessarily surfaces what they had been discarding. The project took uniform demotion anyway
and revised the bar to *"no populated-catalog document loses a promoted record; the only permitted change is
added residuals."* An asymmetric rule — demote here, drop silently there — would have reintroduced the very
discontinuity the tree existed to remove.

**Verified.** A read-only `semantic --dry-run` replay of all 78 documents against their persisted artifacts:
**11 identical · 41 changed only in `residual_decisions` · 26 content-moved**, and every one of the 26 has an
empty declared catalog — so the revised bar is met exactly. Empty-catalog promotion falls `1,423 → 780`
conditional rules (the 780 naming no signal are system-level rules and are kept) and `100 → 0` signal
constraints. All 78 downstream chains were then rebuilt from unchanged `EvidenceIR` and the whole
179-artifact validation population re-validated: all **44 emitted `.isf` are byte-identical** and pass FSMGen
`--strict --check` with **zero** diagnostics, `kg-bench` holds `156/156`, the nine provider-free evals sit at
baseline, and `CHAIN-CURRENCY` is green at 24/78/78/78. *Authoritative tracking:*
`docs/tasks/SEMANTIC-EMPTY-CATALOG-FILTER.md`.
