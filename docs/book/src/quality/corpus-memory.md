# Corpus Memory And Priors

`specforge` has a cross-document learning plane, but it is not a neural network and it is not hidden model state.

The thing that grows over time is a typed symbolic prior store:

`generated/prior_memory/corpus_memory.json`

## Two separate planes

The architecture keeps document truth and learned experience separate.

Document plane:

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`

Learning plane:

`CorpusMemory`

The document plane stays grounded in the current PDF.
The learning plane stores reusable extraction knowledge learned from earlier validated artifacts.

## What the system learns

Today the prior store can contain families such as:

- actor-taxonomy priors
- semantic phrase priors
- semantic modality-reliability priors
- temporal phrase priors
- table-shape priors
- visual-motif priors
- negative-knowledge priors

These are not copied document facts.
They are reusable interpretation patterns.

Examples:

- a local actor term is requester-like
- a phrase shape is ready-like
- a timing phrase implies a one-cycle window
- a header signature often means a signal-description table
- a captioned visual asset is a timing-diagram motif
- a repeated conflict or residual class should make future extraction more cautious

## What the system does not learn

The learning plane must not smuggle facts from one spec into another.

So the safe rule is:

- priors widen interpretation of the current document
- priors do not directly author canonical truth

That is especially important for visual-motif and negative-knowledge priors.
Visual-motif priors already have a first bounded consumer: if a current visual asset is still classified as `unknown` but has a local caption matching one unique learned motif, `EvidenceIR` may add a prior-memory `Classification` observation and use the recovered diagram kind for the visual evidence role.
That does not rewrite `SourceIR` and does not create semantic facts by itself.

Negative-knowledge priors remain cautionary memory.
They are intended to guide rescans, ranking, or stronger-corroboration thresholds, but they still need current-document evidence before anything becomes canonical.

That is why `specforge` can get smarter over time without collapsing into cross-document contamination.

## Why this is explicit memory instead of hidden weights

The learning system is symbolic and inspectable.

That means:

- the code defines how learning works
- the prior store captures what has been learned so far
- the contents can be read, debugged, filtered, and regenerated

This is a strong fit for a provenance-first compiler pipeline because the memory stays visible instead of disappearing into opaque model parameters.

## Validation-gated feedback

Not every artifact should feed the learning plane.

The intended discipline is:

1. recover local truth from the current document
2. validate it
3. only then harvest reusable priors from sufficiently trusted output

So the learning plane depends on validation rather than bypassing it.

## Why this matters

This is how `specforge` can become better on PDF `N + 1` after analyzing PDFs `1..N`:

- not by leaking old facts into the new document
- but by learning how chip specs usually express meaning

That distinction is one of the most important architectural choices in the project.
