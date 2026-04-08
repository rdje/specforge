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

## Why this stage matters

If `SourceIR` is lossy, the downstream KG cannot recover what was lost reliably.

That is why earlier project investment went heavily into ingest and source preservation:

- table structure
- captions
- sections
- page assets
- figure identity

Those are deterministic, high-leverage wins.

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

