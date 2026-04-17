# SemanticIR

`SemanticIR` is where evidence is assembled into typed protocol meaning.

## What belongs in `SemanticIR`

- actors
- interfaces
- actor-relative ports
- signal connectivity
- phases
- invariants
- gates
- timing constraints
- temporal rules
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

## Why this stage matters so much

If `SourceIR` is where structure is preserved and `EvidenceIR` is where grounded hints are harvested, `SemanticIR` is where the project either becomes trustworthy or starts to hallucinate.

That is why so many truthfulness-hardening slices land here:

- handshake-role arbitration
- polarity-aware temporal comparison
- infrastructure handling
- conflict surfacing
- graph-first direction recovery

This is the main semantic safety boundary before canonical intent.
