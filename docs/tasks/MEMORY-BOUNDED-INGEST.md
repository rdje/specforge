# MEMORY-BOUNDED-INGEST: bounded-memory ingestion of very large PDFs

## Metadata

- Tree ID: `MEMORY-BOUNDED-INGEST`
- Status: `active`
- Roadmap lane: `R2`/`R8` (SourceIR ingestion / Tier-1 capture robustness)
- Created: `2026-06-14`
- Last updated: `2026-06-14`
- Owner: repo-local workflow

## Goal

Make `specforge ingest` effectively **immune to chip-spec PDF file size** — 10 MB, 100 MB,
300 MB, 1 GB, 3 GB and beyond — bounded in **both RAM and DISK**, so no document can exhaust a
host's resources and crash/reboot it, and SPECFORGE runs in the **most restricted RAM/disk
environments**. **Quality is invariant — only SPEED flexes.** Under tight resources SPECFORGE runs
*slower* (smaller batches, incremental disk staging, aggressive freeing) but its SOTA-quality
outcome stays **fully intact**: never lower fidelity, never drop or coarsen captured intent. Owner
directive (`2026-06-14`, reinforced repeatedly, escalated to full size-immunity, then clarified:
"speed might be impacted but the level of top, SOTA quality shall remain untouched"). Hosts have
limited resources no matter how large they look; the 24 GB dev host crashes/reboots in the 90→93%
used-RAM danger zone (`[[feedback_ram_ceiling_monitor]]`, `[[project_big_pdf_memory_bounded_ingest]]`).

The immediate corpus need is docs up to 930 pages (GIC-600 930, CoreSight SoC-600 842, SMMU-700
717, Cortex-A76 620, CHI 585, USB-3.2 548); the standing goal is bounded resources at **any**
size. The two dominant unbounded costs are (a) **RAM** — holding a full-res image for every page
during conversion (peak ∝ page count) — addressed by `.1`; and (b) **DISK** — the `normalized/`
bundle stores a full-res PNG per page, so a multi-thousand-page PDF can write tens of GB —
addressed by the disk-bounding leaves below.

## Non-Goals

- Not changing the SourceIR schema or the downstream EvidenceIR→…→adapter pipeline.
- **NOT trading quality for resources.** Fidelity and captured-intent completeness are invariant;
  the ONLY thing that may flex under tight resources is SPEED (smaller batches, more disk staging).
  Lossy levers — lower image resolution, compression that drops detail, sampling/skipping pages or
  tables — are OUT. Disk-bounding (`.3`) bounds the *working set* (incremental + free), not the
  captured fidelity; full-res figure/table region images and all typed surfaces stay intact.
- Not re-ingesting or perturbing existing gold/intact docs — the batched path activates only above
  a threshold set above every doc we already ingest single-pass (max 500p), so they stay
  byte-identical.
- Not optimizing for raw speed — the aim is bounded RAM/disk at any file size with quality intact;
  slower-under-restriction is acceptable, lower-quality is not.

## Acceptance Criteria

- A big PDF (CHI, 585p) ingests to a complete `SourceIR` + `normalized/` bundle without the
  Docling backend being memory-killed, with peak system RAM held safely below the danger zone.
- Small/medium docs (≤ threshold) keep the **exact current single-pass path** and re-ingest
  byte-identical (proven on a representative gold doc).
- Stays PDF-agnostic (ADR 0006) and preserves the staged-swap (a failed run never destroys the
  last good `normalized/`; `source_ir.json` survives).
- Focused checks + full `scripts/run_ci.sh` green; CHI verification run under the autonomous
  RAM guard; live docs + book updated; committed per `COMMIT.md`.

## Task Tree

- ID: `MEMORY-BOUNDED-INGEST`
  Status: `active`
  Goal: resource-bounded ingestion immune to PDF file size (RAM + DISK)
  Children: `.1`, `.2`, `.3`, `.4`, `.5`

- ID: `MEMORY-BOUNDED-INGEST.1`
  Status: `done`
  Goal: implement page-range batched conversion in the Docling backend helper (+ Rust config),
  gated above a page threshold; extract the per-doc element processing into one shared routine
  so single-pass and batched modes produce the same record shapes.
  Acceptance: code lands behind a page-count gate; small docs unchanged; `run_ci.sh` green; the
  helper frees each batch (`del`+`gc.collect()`) so peak memory is O(batch), not O(page count).
  Verification: `done (2026-06-14)` — extracted `_IngestAccumulator` + `process_converted_document`
  (the two per-doc loops moved verbatim, accumulators externalized); `detect_pdf_page_count`
  (pypdfium2, cheap) + `_env_int`; `main` dispatches single-pass (`convert(path)`, unchanged) when
  `page_count <= SPECFORGE_INGEST_BATCH_THRESHOLD` (default 512) else page-range batches of
  `SPECFORGE_INGEST_BATCH_PAGES` (default 64), freeing each batch. Python syntax py_compiled; Rust
  builds; full `run_ci.sh` GREEN (1587 tests). On a temp 14-page doc (no gold touched): **(A)
  single-pass before-vs-after-refactor `source_ir.json` BYTE-IDENTICAL**; **(B)** forced batched
  (threshold=4, batch=4 → 4 batches) vs single-pass: `page_no` ABSOLUTE across batches (1–14, zero
  duplicate page/table/element ids), tables/visual/sections counts equal, table (id,caption) sets
  equal — the only delta is one boilerplate running-header (`"I 2 S bus specification"`) that
  single-pass merges across one page boundary and batched keeps as two (benign boundary artifact,
  no content/signal/table loss). Commit: `MEMORY-BOUNDED-INGEST.1`.
  Commit: `MEMORY-BOUNDED-INGEST.1`

- ID: `MEMORY-BOUNDED-INGEST.2`
  Status: `in_progress`
  Goal: verify on CHI (585p) end-to-end under the autonomous RAM guard — ingest completes with
  bounded peak RAM, `normalized/` + `source_ir.json` materialize; re-prove small-doc
  byte-identity on a gold doc; then unblock `PDF-VARIANT-DIGESTION.13c`.
  Acceptance: CHI ingest succeeds under the ≥85%-used kill ceiling; a gold doc re-ingests
  byte-identical; measurements recorded.
  Verification: `pending`
  Commit: `pending`

- ID: `MEMORY-BOUNDED-INGEST.3`
  Status: `proposed`
  Goal: bound the **DISK** footprint of the `normalized/` bundle for very large PDFs **without any
  fidelity loss** (the dominant cost is the full-res PNG per page — a multi-thousand-page doc can
  write tens of GB). Quality-preserving lever: defer per-page full-res image generation to
  **on-demand** (rendered full-res at `enrich` time only for the pages a consumer actually reads)
  instead of eagerly for every page at ingest — so ingest disk is O(assets), not O(pages), and any
  page image, when needed, is still full resolution. Keep figure/table region images and all typed
  SourceIR surfaces intact. NO lossy levers (no lower resolution / detail-dropping compression /
  page or table skipping). PDF-agnostic.
  Acceptance: a huge-page-count doc ingests within a bounded disk budget; any page image that is
  later produced is byte-for-byte the full-res image; small docs unchanged.
  Verification: `pending`
  Commit: `pending`

- ID: `MEMORY-BOUNDED-INGEST.4`
  Status: `proposed`
  Goal: complete at **full quality even in the most restricted** RAM/disk environments by trading
  SPEED, not quality — a pre-flight resource check, adaptive batch size (shrink the batch / spill
  more to disk under pressure → slower but identical output), and the autonomous RAM guard as a
  first-class ingestion safeguard. A clear typed error is the ABSOLUTE last resort only when the
  task is genuinely impossible (e.g., disk physically full) — never a quality compromise, never a
  silent partial result.
  Acceptance: ingestion never OOM-crashes the host; under restriction it completes with the SAME
  output (just slower / smaller batches), or — only if truly impossible — fails with a typed
  diagnostic and an intact prior `normalized/` (staged-swap).
  Verification: `pending`
  Commit: `pending`

- ID: `MEMORY-BOUNDED-INGEST.5`
  Status: `proposed`
  Goal: bound the in-RAM + on-disk size of the accumulated summary / `source_ir.json` itself for
  extreme page counts (the lightweight records still grow with page count). Evaluate streaming /
  chunked summary assembly so even a 10000-page doc holds bounded summary state.
  Acceptance: summary assembly memory + `source_ir.json` size stay bounded at extreme page counts.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `MEMORY-BOUNDED-INGEST.2` | `in_progress` | verify on CHI (585p) under the RAM guard, then unblock `.13c` |

`.1` (implement) `done` 2026-06-14.

## Decisions

- `2026-06-14`: **Root cause** — the embedded Docling helper
  (`crates/specforge/src/ir/source/docling_backend.rs`) calls
  `converter.convert(str(input_path))` on the whole PDF with `images_scale = 2.0` +
  `generate_page_images = True`, so a full-res image for every page is held in `doc.pages`
  simultaneously. Peak memory ∝ page count → unbounded; CHI (585p) is killed at conversion.
- `2026-06-14`: **Mechanism** — Docling 2.84's `DocumentConverter.convert(source, …,
  page_range=(lo, hi))` (1-based inclusive; default `(1, MAXINT)`) lets us convert bounded page
  ranges. Internal `settings.perf.page_batch_size = 4` is model-inference batching only and does
  not bound the held document.
- `2026-06-14`: **Threshold design** — batch only when `page_count > SPECFORGE_INGEST_BATCH_THRESHOLD`
  (default **512**, above the 500p max of every currently-ingested doc), so all existing gold /
  intact docs keep the EXACT single-pass call and stay byte-identical. Batch size
  `SPECFORGE_INGEST_BATCH_PAGES` (default **64**, a multiple of Docling's internal 4). Page count
  is detected cheaply before conversion (pypdfium2 / Docling backend page count — verify).
- `2026-06-14`: **Merge** — the `DoclingBackendSummary` manifest (page_artifacts /
  structured_tables / content_elements / document_sections / profile) is the SourceIR-bearing
  product and holds only lightweight records, so it accumulates across batches with continued
  counters (picture/table/reading-order/section). The raw `export_to_dict()` is only a
  provenance *path* (`caption_source_path`), and markdown is a lossy convenience view — both can
  be merged/concatenated per batch without fidelity risk. Page images are saved to disk per batch
  then freed.
- `2026-06-14`: **Safety** — every verification re-ingest runs under the autonomous RAM guard
  (kill at ≥85% used; below the 90% danger floor), one heavy job at a time, model unloaded during
  ingests (`[[feedback_ram_ceiling_monitor]]`).

## Open Questions

- Does Docling report absolute page numbers (`page.page_no`) within a `page_range` batch, or
  range-relative? (Must be absolute, else page IDs collide across batches — verify in `.1`.)
- Is `convert(path)` byte-identical to `convert(path, page_range=(1, page_count))`? (Not relied
  upon — small docs keep the no-page_range call — but informs whether the gate could be removed
  later.)

## Blockers

- None. (`PDF-VARIANT-DIGESTION.13c` is blocked ON this tree; `.13d` likewise for its biggest docs.)

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-14` | `MEMORY-BOUNDED-INGEST.1` | py_compile + `cargo build` + full `run_ci.sh` (1587) + temp-14p before/after | GREEN; (A) single-pass BYTE-IDENTICAL; (B) batched complete+correct (`page_no` absolute, surfaces match, one benign boilerplate-header boundary split) |
| `2026-06-14` | `MEMORY-BOUNDED-INGEST.2` | CHI (585p) ingest under the autonomous RAM guard | `pending` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `MEMORY-BOUNDED-INGEST.1` | `MEMORY-BOUNDED-INGEST.1` | page-range batched ingestion behind a 512p gate |
| `MEMORY-BOUNDED-INGEST.2` | `pending` | `pending` |

## Changelog

- `2026-06-14`: Created task tree (owner-directed, surfaced by the CHI `.13c` OOM). Design
  recorded in Decisions.
- `2026-06-14`: `.1` DONE — implemented page-range batched ingestion in the Docling helper
  (`_IngestAccumulator` + `process_converted_document` + `detect_pdf_page_count` + `_env_int`;
  single-pass unchanged ≤512p, batched >512p freeing each batch). py_compile + full CI green;
  temp-14p proof: single-pass byte-identical, batched complete/correct (one benign boilerplate
  boundary split). `.2` (CHI under the RAM guard) in progress.
