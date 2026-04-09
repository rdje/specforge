# SourceIR

`SourceIR` is the document-preservation stage.

Its job is to retain enough faithful structure that later stages do not have to guess unnecessarily.

## What belongs in `SourceIR`

- document identity
- normalization outputs
- page structure
- structured tables
- visual assets
- section layout
- backend metadata

In practical terms, `SourceIR` is where the pipeline decides:

- what the source document is
- how it was normalized
- what tables, figures, and page assets exist
- where those assets came from in the original document

It is the closest thing the system has to a structured "compiled source document".

## Why this stage matters

If `SourceIR` is lossy, the downstream KG cannot recover what was lost reliably.

That is why earlier project investment went heavily into ingest and source preservation:

- table structure
- captions
- sections
- page assets
- figure identity

Those are deterministic, high-leverage wins.

This is why early project effort went so heavily into ingest and preservation rather than immediately chasing higher-level semantics.
If the source layer loses the table grid, the caption link, the section boundary, or the asset identity, later semantic stages can only guess.

## What a good `SourceIR` artifact gives later stages

A strong `SourceIR` should let later stages ask grounded questions like:

- which page did this table come from?
- what was the table header layout?
- which caption belongs to this figure?
- where in the section hierarchy did this statement appear?
- which visual asset was referenced here?

That is a much better foundation than forcing later stages to work from flattened text alone.

The dedicated [Multimodal Evidence And Visual Grounding](multimodal-evidence.md) chapter explains how preserved visual assets flow into visual evidence, VLM observations, and semantic grounding.

## What `SourceIR` is not

It is not the place where protocol semantics should be invented.

It should preserve the document faithfully and expose enough structure for later semantic lifting, but not pretend to know the final meaning already.

## Current maturity boundary

`SourceIR` is relatively mature in architecture, but not "done forever".

It is strong enough to be the foundation.
It is not yet assumed universal for every ugly real-world PDF.

Remaining work there is mostly:

- robustness hardening
- benchmarking on messy PDFs
- fallback behavior
- failure detection

not broad new concept invention.

## Typical `SourceIR` failure modes

The main risks at this stage are structural, not semantic.

Examples:

- OCR-heavy or scanned PDFs
- multi-column reading-order drift
- rotated or split tables
- weak caption-to-figure linkage
- unusual appendix layouts
- backend-dependent table-kind misclassification

When those failures happen, the right response is usually:

- preserve the failure honestly
- surface it clearly
- harden normalization or fallback behavior

not to pretend a later semantic stage can recover structure that was never preserved.

## The key rule of this stage

`SourceIR` should maximize faithful preservation and minimize premature interpretation.

That discipline is what allows the later stages to be ambitious without becoming reckless.
