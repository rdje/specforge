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

Handshake recovery should prefer grounded role evidence over raw `*VALID*` / `*READY*` name shape.

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

### Heuristic grouping needs positive authority

Statement-level signal co-mentions normally group signals that are already authoritative. `SemanticIR` first
establishes authority from formal `Signal X is ...` declarations and the document system contract, then
intersects candidates with that surface. A declaration-free document has one narrow positive path: a
multi-signal statement may ground its own group when it starts with one candidate signal and immediately makes a
deontic signal action (`must` or `shall` plus assertion, deassertion, or stability).

This fail-closed rule matters for prose-heavy and table-heavy documents. A row such as
`| F0 84 | 01 | A0 | A4 | D5 |` may contain hardware-shaped uppercase tokens, but without a typed declaration
there is no evidence that those tokens are top-level interface wires. Before this boundary was enforced, 21
retained documents accumulated 5,527 low-confidence interfaces / 18,397 records and fed 4,060 signals into ISF
adapters. The same documents now produce no interface because none satisfies the positive behavior grammar.

The distinction preserves real heuristic-only behavior. `VALID must remain asserted until READY is observed`
grounds the `VALID`/`READY` pair even without a separate declaration. In contrast, a raw encoding row, ordinary
uppercase prose, or an actor relation such as `TBU reads DOWNSTREAM` does not declare a wire surface.

Formal declarations are unaffected because they take the explicit interface path. System clock and reset are
also preserved: the system contract inserts them into the authority set and emits an explicit document
interface with typed input direction and width one. Actor/signal relations can describe use of an authorized
signal, but a relation alone is not a signal declaration.

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
prose, keeps statement provenance, and limits signal membership to declared signals. Consequently, `During the
address phase the manager drives HADDR` can establish a typed `address` record without creating a generic
section phase. See the [sentence-authority measurement](../../research/semantic-phase-authority-measurement.md)
and the [retirement measurement](../../research/generic-section-phase-retirement-measurement.md).

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

## Lossless serial and protocol observations

Four evidence-grounded protocol collections now cross into `SemanticIR` unchanged:

- `serial_frame_fields`
- `swd_operations`
- `protocol_states`
- `interface_edge_timings`

This projection is intentionally lossless rather than interpretive. Record order, ids, optional values, and
every `supporting_statement_ids` entry are cloned exactly from EvidenceIR. The fields are serde-defaulted and
omitted while empty, so SemanticIR artifacts written before this addition still load and documents without
these facts keep their existing JSON shape.

The records do not claim more semantics merely because they crossed a stage. In particular, a protocol-state
observation is not a transition graph, and a serial-frame field is not automatically an executable transaction.
SemanticIR preserves the source-backed structure so later stages can make an explicit lowering decision without
re-reading EvidenceIR or weakening provenance. ADR 0016 forbids inventing missing transitions, guards, initial
state, encodings, values, ports, activation conditions, or storage targets.

`specforge validate <semantic_ir.json>` exposes the boundary directly:

```text
=== Serial / Protocol Surface Projection ===
  serial_frame_fields: N
  swd_operations: N
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
