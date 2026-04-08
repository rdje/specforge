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

## The mindset of this stage

This is still an evidence layer, not the final semantic truth.

So the right behavior is:

- extract what the document supports
- preserve provenance
- keep conflicting evidence visible
- avoid over-promoting generic examples into canonical interface truth

## Typical evidence-level wins

- source/destination table recovery
- table-grounded widths
- prose-grounded actor relations
- visual-caption semantic hints
- VLM timing-note observations
- polarity extraction

## Typical evidence-level failure modes

- field tables leaking fake signals
- abstract example tables pretending to be real interfaces
- payload nouns being promoted to actors
- signal names creating semantic meaning by spelling alone

Many of the project’s recent truthfulness slices have been about tightening exactly those boundaries.

