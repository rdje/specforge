---
id: docling-metadata-sidecar-paths-are-portable
title: Docling metadata paths are normalized before the staged bundle is promoted
answers:
  - "does the Docling metadata sidecar store repository relative paths"
  - "why did a fresh ingest expose normalized staging in persisted metadata"
  - "where is normalized staging removed from Docling metadata"
  - "how is an external PDF labeled in Docling metadata"
  - "does a metadata rewrite failure preserve the previous normalized bundle"
date: 2026-08-09
status: current
tags: [docling, source-ir, metadata, path-portability, staged-swap]
evidence: crates/specforge/src/ir/source/docling_backend.rs; scripts/check_persisted_artifact_paths.pl; docs/tasks/SWD-SERIAL-EXTRACTION.md (.7e.ii.a)
reverify: "perl scripts/check_persisted_artifact_paths.pl --check && cargo test -p specforge --lib backend_metadata_"
---

Docling writes `normalized/<document>.meta.json` from the embedded helper while a PDF normalization is still
running under `normalized.staging`. The helper therefore sees absolute runtime arguments, including the staged
Markdown destination. Renaming the directory relocates files but cannot rewrite JSON values. This producer had
remained latent after normalized bundles were reclaimed; a fresh canonical ADI ingest restored the sidecar and
the persisted-artifact oracle rejected its two absolute path values.

`materialize_pdf` now rewrites the staged sidecar before replacing the prior normalized bundle. `input_path`
passes through the source's explicit `PersistedPathOrigin`, `promoted_markdown_path` passes through the
repository-owned storage boundary using the final `normalized/` destination, and `path_origin` is serialized
alongside them. A repository source therefore persists both paths relative to the current root. An explicitly
authorized external PDF may keep its absolute identity only with `path_origin: "external_input"`.

The rewrite occurs after backend success but before the staged-directory swap. A missing, malformed, unsafe, or
unwritable sidecar returns a typed backend error and removes the staging tree; it cannot replace the last-good
canonical bundle. The canonical path oracle classifies this exact sidecar shape, permits only the labeled
external `input_path`, and continues to reject every repository-owned absolute value.
