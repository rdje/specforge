# Reference

This section collects the durable reference material for `specforge`.

It is different from the root live docs.
The reference chapters should help a reader understand stable project surfaces without needing to know the current development session.

Start here when you want to understand:

- the durable extraction and canonical-product contracts
- which files `specforge` writes
- which docs are public-facing versus continuity-focused
- how generated artifacts, validation reports, and prior memory fit together
- how to troubleshoot common runtime and pipeline issues

## Reference chapters

- [Extraction Architecture Contract](extraction-architecture.md) defines the evidence modalities, stage obligations, and target-quality boundary.
- [IntentIR Product Contract](intentir-contract.md) defines the canonical output, residual, serialization, adapter, and guardrail boundary.
- [Generated Artifacts](generated-artifacts.md) explains the local artifact tree and why `generated/` is not tracked.
- [Documentation Scope And Continuity](documentation-scope.md) explains the book's role as the public documentation surface.
- [Live Docs And Continuity](live-docs.md) explains the repo-root continuity docs and how they differ from the book.
- [Troubleshooting](troubleshooting.md) gives the first checks to run when the runtime or pipeline behaves unexpectedly.

## Practical rule

Use this section when you need a stable map.
Use the root live docs when you need the latest engineering state.
