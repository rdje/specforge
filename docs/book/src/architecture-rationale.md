# Architecture Rationale

This chapter explains why `specforge` is built the way it is.

The short version is:

- do not ask a model to "understand the whole spec" in one shot
- preserve source structure first
- lift grounded evidence second
- build typed semantic meaning third
- keep uncertainty visible instead of flattening it away

## The problem `specforge` is trying to solve

Chip and protocol specifications are not just text.
They are mixed evidence fields made of:

- prose
- signal tables
- timing tables
- figures
- captions
- diagrams
- page-local structure

If the tool treats the whole document as plain text, it loses too much of the shape that later meaning depends on.

## Why there is a staged IR pipeline

`specforge` uses:

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`

because different kinds of truth belong in different places.

- `SourceIR` preserves document structure
- `EvidenceIR` records grounded extracted evidence
- `SemanticIR` lifts that evidence into typed domain meaning
- `IntentIR` is the canonical backend-independent product surface

This separation makes the system:

- inspectable
- debuggable
- easier to validate
- less likely to hallucinate

## Why `specforge` does not try to "program understanding English"

The design goal is not to teach code to understand arbitrary English.

The design goal is narrower and more realistic:

- define a typed hardware/protocol world model
- recover evidence that maps into that model
- validate what survives into canonical intent

That is a much stronger engineering target than trying to solve unrestricted language directly.

## Why AI is bounded instead of central

AI is useful in `specforge`, but it should not be the final authority.

The intended role of AI is:

- propose local hypotheses
- help with ambiguous prose
- help with image-heavy evidence such as timing diagrams or figure captions

The intended role of the rest of the pipeline is:

- ground those hypotheses in the current document
- check them against the typed schema
- surface conflicts and uncertainty
- decide what is safe enough to keep

So the guiding rule is:

- models can propose
- validation and arbitration decide

## Why provenance matters so much

Every promoted fact should stay tied to the evidence that justified it.

That matters because the project is trying to produce implementation-facing truth, not just plausible summaries.

Provenance makes it possible to:

- inspect where a fact came from
- compare competing evidence
- debug false positives
- keep ambiguity explicit

## Why residuals and conflicts are first-class

When the evidence is not decisive, `specforge` should not silently guess.

That is why the project preserves things like:

- residual decisions
- semantic conflicts
- connectivity conflicts
- temporal conflicts

This is not only a validator feature.
It is part of the core architecture.

## Why infrastructure semantics are separated

Some hardware signals are not ordinary protocol edges.

Clocks and resets are the clearest example.
They can appear in the same tables as other signals, but they carry system-level meaning:

- clocks define the sequential timing reference
- resets define initialization and recovery discipline
- reset assertion and release timing are polarity-sensitive
- sourcing and distribution are infrastructure concerns, not ordinary producer/consumer protocol relations

That is why `specforge` treats clock and reset handling as system-contract infrastructure instead of flattening it into the same category as payload or handshake connectivity.

The dedicated chapter [Clock And Reset Infrastructure](domain/clock-reset.md) explains that boundary in more detail.

## Why signal roles are evidence-based

Protocol signal names are useful, but they are not enough.

A signal named `XVALID` may look valid-like, but `specforge` should still prefer grounded role evidence from tables, prose, captions, and timing annotations.

That is why semantic-role recovery is modeled through:

- observations
- candidates
- arbitration
- consensus
- explicit blocked fallbacks when name shape is unsafe

The dedicated chapter [Handshake And Semantic Roles](domain/handshake-semantics.md) explains how that role surface works.

## Why connectivity is graph-first

Flat `input` / `output` labels are useful, but they lose actor perspective.

`specforge` prefers to recover structural facts such as:

- `(Requester, Drives, PSEL)`
- `(Completer, Reads, PSEL)`

Those facts explain who produces and consumes a signal, and they can later support actor-relative ports, signal connectivity, and actor-grounded temporal predicates.

The dedicated chapter [Actor Connectivity And Graph Direction](domain/actor-connectivity.md) explains that graph-first direction model.

## Why learning is symbolic and explicit

`specforge` can get stronger across many documents, but the thing that grows is not hidden neural state.

What grows is an explicit typed prior store:

`generated/prior_memory/corpus_memory.json`

That store learns reusable extraction knowledge such as:

- actor vocabulary
- semantic phrase patterns
- temporal phrase patterns
- table-shape patterns

The critical safety boundary is:

- priors widen interpretation of the current document
- priors do not directly author canonical truth

## Why document truth and cross-document experience stay separate

The current document pipeline should remain provenance-pure.

That means:

- canonical IR facts must be justified by the current PDF
- earlier documents may help the extractor interpret new evidence
- earlier documents must not leak their facts directly into new canonical artifacts

This is one of the most important design boundaries in the whole project.

## Why this architecture should scale

The architecture is designed to improve in two directions at once:

- stronger per-document truthfulness
- stronger cross-document extraction experience

That is the reason for the layered shape:

- document-local canonical truth
- cross-document prior memory
- and, over time, a broader corpus knowledge-base layer

`specforge` is trying to become expert-like by accumulating reusable extraction knowledge, not by replacing the document pipeline with a black box.
