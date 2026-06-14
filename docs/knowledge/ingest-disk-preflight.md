---
id: ingest-disk-preflight
title: ingest disk pre-flight refuses before launching when free disk is below a source-size-scaled requirement
answers:
  - "how does ingest avoid filling the disk on a very large PDF"
  - "what is the ingest disk pre-flight check"
  - "what does SPECFORGE_INGEST_MIN_FREE_DISK_MB do"
  - "how is the required free disk for an ingest estimated"
  - "why did ingest stop with 'ingest aborted before launching'"
  - "what is AppError::IngestAbortedForDisk"
  - "how does specforge read free disk space without a new dependency"
  - "why doesn't the disk estimate use the page count"
date: 2026-06-14
tags: [ingest, docling, disk, memory-bounded, preflight, safety, source-ir]
evidence: crates/specforge/src/ir/source/docling_backend.rs (DiskPreflightRequirement / preflight_ingest_disk / parse_df_available_kb / estimate_required_disk_mb); crates/specforge/src/error.rs (AppError::IngestAbortedForDisk); docs/tasks/MEMORY-BOUNDED-INGEST.md (.4b)
reverify: grep -n "preflight_ingest_disk\|DiskPreflightRequirement\|SPECFORGE_INGEST_MIN_FREE_DISK_MB\|IngestAbortedForDisk" crates/specforge/src/ir/source/docling_backend.rs crates/specforge/src/error.rs
---

`MEMORY-BOUNDED-INGEST.4b` adds the DISK dimension of the ingest pre-flight (the RAM dimension was
already delivered by `.4a`'s pre-spawn memory sample). `materialize_pdf` checks free disk at its very
FIRST statement — BEFORE any staging directory is created — so a refusal touches nothing on disk and
any previous `normalized/` bundle + `source_ir.json` are trivially intact.

**Why source size, not page count:** a precise per-document bundle-size estimate is ill-posed
pre-ingest — the figure/table asset count is unknown, and the page count is only computed INSIDE the
Docling subprocess (`detect_pdf_page_count`), so Rust has no cheap pre-ingest page count without
re-parsing the PDF (a dependency / drift / an extra subprocess). The one cheap signal Rust already
has is the **source PDF file size** (`fs::metadata(source).len()`). So
`estimate_required_disk_mb(source_bytes) = 128 (base headroom for markdown/raw-JSON/region-crops) +
source_mb × 4` (saturating). It is deliberately conservative; the `.3` disk bounding makes a large
doc's bundle closer to `O(source)` than ×4 assumes, and the staged-swap is the backstop for the
imprecise middle ground.

**No new dependency:** free disk is read via POSIX `df -P -k <path>` (`-P` forces single-line rows;
Available is field index 3). `parse_df_available_kb` is a pure parser unit-tested on macOS+Linux
sample output. `available_disk_mb` returns `None` when `df` is unavailable/unparseable, and
`check_disk_preflight` is then PERMISSIVE — it never refuses on missing data (mirrors the RAM guard).
`nearest_existing_ancestor` gives `df` a real target before the artifact root exists (first ingest).

**Config:** one knob `SPECFORGE_INGEST_MIN_FREE_DISK_MB` parsed by `parse_disk_preflight_requirement`
into `DiskPreflightRequirement`: unset/empty → `EstimateFromSource`; a positive integer →
`Floor(mb)` (fixed MB requirement overriding the estimate); `off`/`none`/`disabled`/`0` → `Disabled`;
garbage → `EstimateFromSource` (never silently disable on a typo). New typed
`AppError::IngestAbortedForDisk { path, free_mb, required_mb }` with an actionable Display (free disk
/ set a floor / disable).

**Test note:** the 3 source.rs stub-helper ingest tests set `SPECFORGE_INGEST_MIN_FREE_DISK_MB=off`
(a CI host low on disk could otherwise false-abort them); the unix tests that spawn `df`/`sleep`/`true`
by PATH lookup hold `env_var_lock()` because the `inspect_docling_runtime` tests set `PATH=""`.
Related: [[ingest-ram-guard]], [[page-image-disk-bounding]].
