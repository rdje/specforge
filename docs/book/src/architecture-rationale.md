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

The dedicated chapter [Multimodal Evidence And Visual Grounding](pipeline/multimodal-evidence.md) explains how visual assets, captions, and VLM observations enter that bounded flow.

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

There is also a document-scope boundary here.
Protocol and chip-interface PDFs are usually shared contracts for RTL designers and verification-IP authors.
They describe the clock/reset semantics that participants can rely on at the boundary, but they rarely define the final chip's physical clock tree or reset tree.
Those trees are normally custom SoC integration work owned by the chip team.

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

## Why timing is typed

Timing prose is too important to remain only as text.

`specforge` turns recoverable timing obligations into typed temporal rules with:

- clock edge
- tick phase
- predicates
- cycle windows
- actor grounding
- explicit conflict records

The dedicated chapter [Temporal Semantics And Timing](domain/temporal-semantics.md) explains that temporal model.

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

## Why the design is grounded in published research

Almost everything `specforge` does — turning PDFs into structured documents, lowering
meaning through staged representations, pulling normative requirements and protocol timing
out of prose, building a knowledge graph of actors and signals, fusing text with tables and
figures, using a bounded LLM, learning across documents, measuring its own recall, and
handing off to hardware tooling — is something a research community has already studied for
years.

That is good news, and we treat it as such. It means the project does not have to invent
its foundations from scratch: where a problem is settled in the literature, `specforge`
should *adopt the proven approach and the standard vocabulary* rather than re-derive a
weaker version. And it means the genuinely new ideas can be named and defended, instead of
being lost in the noise.

So one task-tree (`LITERATURE-GROUNDING`) deliberately maps each part of the design onto its
prior art. For every aspect it records three things: where `specforge` already matches the
established work (validate the design), which techniques are worth borrowing (adopt), and
where `specforge` genuinely does something the literature does not (claim it). A short
unifying map and a prioritized "what to improve next" backlog live alongside the per-aspect
write-ups under `docs/research/grounding/`.

A few threads recur across every aspect — these are the parts worth claiming as genuinely
ours:

- **We work forwards from the spec.** Most of the related work (program-invariant mining,
  assertion generation, specification mining) works *backwards* from an existing
  implementation — traces, RTL, or code. `specforge` recovers intent from the
  human-authored specification *before* any implementation exists.
- **When the evidence is not decisive, we keep a structured record instead of guessing.**
  The wider field usually stops at a confidence number or simply drops the uncertain case.
  `specforge` emits a first-class, provenance-carrying residual decision — uncertainty stays
  visible and auditable.
- **We estimate the misses we cannot see.** Borrowing capture–recapture from software
  inspection, `specforge` gives an honest lower bound on what it *failed* to extract, not
  just a score on what it found.

There is one non-negotiable rule behind all of this: **every citation must be real and
verifiable.** A literature-grounding effort is only trustworthy if its references resolve.
Each source is checked against a resolvable identifier (an arXiv id, DOI, RFC number, ISBN,
IEEE standard, or stable URL), the riskiest recent works are double-checked by hand, and any
source that cannot be confirmed is dropped — never guessed. The goal is to help you trust
that `specforge` stands on real ground, and to show you exactly where it goes beyond it.

The full grounding survey — one document per aspect plus the unifying synthesis map — lives
in `docs/research/grounding/` (`README.md` is the synthesis and index).
