---
id: source-pdf-registry-authority
title: Git-indexed corpus PDFs are the source-registry membership authority
answers:
  - "what is the authoritative membership set for corpus SOURCE_PDF_REGISTRY"
  - "how is corpus SOURCE_PDF_REGISTRY currentness checked"
  - "how are document keys in the source PDF registry derived from filenames"
  - "do host-local source libraries define tracked corpus membership"
date: 2026-08-08
status: current
tags: [corpus, pdf, registry, currentness, document-key]
evidence: corpus/SOURCE_PDF_REGISTRY.md; doctrine/live_document_size/source_pdf_registry.json; docs/decisions/0013-git-indexed-source-pdf-registry-authority.md
reverify: perl scripts/check_source_pdf_registry_currentness.pl --report
---

The authoritative durable source set is every Git-indexed `.pdf` below `corpus/`. The 22-row
`corpus/SOURCE_PDF_REGISTRY.md` must cover that set exactly once. The owner's larger host-local library
and git-ignored generated artifacts are intentionally outside membership authority because neither is
available in every clone.

For each row, the checker requires a safe tracked repository path, a regular file beginning with
`%PDF-`, and a corpus-directory column equal to the path's parent below `corpus/`. The human-maintained
class label is descriptive; it does not decide membership.

The key is derived, not discretionary. `SourceIR` takes the final filename stem, retains lowercase
ASCII alphanumerics, replaces each run of other characters with one underscore, trims boundary
underscores, and falls back to `source` only for an empty result. The contract pins both Rust functions
and their build seams. `scripts/check_source_pdf_registry_currentness.pl` validates all 22 PDFs without
reading host-local or generated state; its 13 repository-local cases fail closed on membership, row,
key, directory, signature, producer, path, identity, and schema drift.
