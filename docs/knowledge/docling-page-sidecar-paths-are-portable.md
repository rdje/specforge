---
id: docling-page-sidecar-paths-are-portable
title: Docling page sidecars are normalized before the staged bundle is promoted
answers:
  - "why did rendered_image.path contain normalized.staging"
  - "are normalized pages page JSON image paths repository relative"
  - "where are Docling page sidecar paths normalized"
  - "does malformed page metadata preserve the last good normalized bundle"
  - "what happens to a page sidecar path when page images are not persisted"
  - "does SpecForge reject a page metadata staging traversal or symlink escape"
date: 2026-08-09
status: current
tags: [docling, source-ir, page-metadata, path-portability, staged-swap]
evidence: crates/specforge/src/ir/source/docling_backend.rs; docs/tasks/CORPUS-COVERAGE.md (.2.34b.ii.a)
reverify: "cargo test -p specforge --lib page_metadata_ && cargo test -p specforge --lib pdf_source_ir_materialization_ && perl scripts/check_persisted_artifact_paths.pl --check"
---

The Docling helper writes each `normalized/pages/page-*.json` while the ingest is running under
`normalized.staging`. Its `rendered_image.path` therefore names the absolute runtime staging PNG. Renaming the
directory relocates the file but cannot rewrite the JSON string. `DoclingBackendSummary::relocate_paths` repairs
the summary returned to SourceIR in memory, and the older metadata rewrite repairs the document-level
`.meta.json`; neither previously touched the page sidecars already on disk. A fresh 51-page USB4 ingest exposed
51 such stale values through the project-locality oracle.

`materialize_pdf` now rewrites every page sidecar after backend success and before replacing the last-good
`normalized/` bundle. A saved-image path must match its `PageArtifact` summary value, contain no traversal, and
resolve below the staging root without a symlink escape. It is then relocated to the final `normalized/`
destination and serialized through the repository-owned persisted-path contract. When page-image persistence is
disabled, both summary and sidecar must agree on `null`, which stays `null`.

A missing, malformed, mismatched, or escaping page record returns `InvalidBackendOutput` and deletes only the
disposable staging directory; the prior normalized bundle remains byte-for-byte available. Focused tests cover
the final relative path, the no-image case, traversal rejection before an outside file is read, malformed JSON
rollback, and complete stub-helper materialization.
