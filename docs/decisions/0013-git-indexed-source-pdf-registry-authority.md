# 0013 — Git-indexed corpus PDFs define source-registry membership

- Date: 2026-08-08
- Status: accepted
- Deciders: project owner, SpecForge repository workflow

## Context

`corpus/SOURCE_PDF_REGISTRY.md` contains 22 rows and the Git index contains 22 PDFs below `corpus/`.
Every row currently resolves, begins with `%PDF-`, names the correct parent directory, and matches the
`SourceIR` key derived from its filename. That agreement was manually established during imports but
had no currentness verifier; a later PDF or row change could silently break reproducible re-ingest.

The owner also keeps a larger host-local source library, and generated SourceIR directories may exist
for documents not present in a fresh clone. Neither is stable tracked membership authority. Conversely,
making the Markdown table its own denominator would allow a missing tracked PDF and its row to disappear
together without detection.

## Decision

The Git-indexed set of `.pdf` files below `corpus/` is the authoritative membership denominator. The
registry must cover that set exactly once. Each row's repository path must be safe and regular, its
redundant corpus-directory column must equal the path's parent below `corpus/`, and its file must start
with the PDF signature.

The `document_key` is not hand-authoritative metadata. It must equal the real `SourceIR` derivation:
take the final filename stem, retain lowercase ASCII alphanumerics, collapse each run of other
characters to one underscore, trim boundary underscores, and use `source` only if nothing remains.
The currentness contract pins `stable_stem`, `document_key`, and their build seams so a code change
cannot invalidate the registry grammar unnoticed.

The verifier is read-only. It never reads the host-local library or git-ignored generated artifacts,
and its mutation fixtures stay under guarded repository-local `generated/` paths with fail-closed
cleanup.

## Consequences

- Adding or removing a tracked corpus PDF requires the matching registry row in the same change.
- Renaming a PDF requires its path, derived key, and corpus-directory metadata to move together.
- Class labels remain human-maintained descriptive metadata; membership, path, key, and signature are
  mechanical.
- Git continues to version full PDF content. The currentness oracle checks membership and file type,
  not a second redundant list of PDF object hashes.
- A host-local or generated-only document remains outside the durable source corpus until explicitly
  copied and tracked under `corpus/`.

## Links

- `corpus/SOURCE_PDF_REGISTRY.md`
- `doctrine/live_document_size/source_pdf_registry.json`
- `scripts/check_source_pdf_registry_currentness.pl`
- `docs/knowledge/source-pdf-registry-authority.md`
- `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md`
