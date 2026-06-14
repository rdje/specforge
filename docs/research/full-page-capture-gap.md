# Full-Page Intent-Capture Gap — measured report (`FULL-PAGE-INTENT-CAPTURE.1`)

> Owner-directed (`2026-06-14`, after `MEMORY-BOUNDED-INGEST.4c`). Answers the
> question: **does SpecForge use the full scope of a page's visual information, or is
> intent-bearing content slipping through because nothing reads the full page?**
> This is the probe-first `.1` deliverable — **measurement, no pipeline/IR code change**.
> Method matches the `PDF-VARIANT-DIGESTION.9.x` discipline (probe the persisted corpus
> before building). Read-only and deterministic (same corpus → same numbers).

## TL;DR — recommendation: **NO-GO**

On the persisted corpus (79 docs / 14,762 pages, plus a rigorous pixel-level audit of 16
backend-available docs spanning all three document classes — AMBA wire, serial, large
register/protocol), **there is no material body of intent-bearing page content that escapes
both capture paths.** Body text, tables, and figures are segmented by Docling into
`content_elements` / `structured_tables` / `visual_assets` (figure/table region crops fed to
the VLM arm). What falls *outside* every segmented region is, on inspection, exclusively:

- **decoration** — section-heading underline rules, full-page border boxes (1991 CAN), table
  grid/rule lines;
- **admonition shading** — the light-gray fill behind "Note"/"Caution" callout boxes (the box
  *text* is still captured as a `body_text` element);
- **furniture** — running headers, copyright footers, page numbers (correctly excluded as
  non-intent);
- **genuinely blank pages** — print-layout blank versos and part-divider pages.

None of these carry design intent. The honest answer to the owner's question is:
**SpecForge already uses the full scope of a page's *intent-bearing* visual information;
the residual is decoration/furniture/blank pages, not lost intent.** A whole-page VLM read
(`.2`) would add non-determinism plus RAM/disk cost (a 930-page doc is exactly the
`MEMORY-BOUNDED-INGEST` size-immunity frontier) in exchange for ≈zero recoverable intent.
**The tree closes NO-GO; `.2` stays unbuilt.**

## What "capture" means here

A page reaches the typed pipeline through exactly two paths today:

1. **Structured-element path** — Docling's typed text/table/section elements
   (`content_elements`, `structured_tables`, `document_sections`).
2. **Region-crop path** — figure/table region crops (`visual_assets[].image_path`) fed to the
   VLM arm (`enrich`, `audit-extraction`, `recover-register-bits`).

The per-page **full-page raster** (`page-NNNN.png`) is read by **no consumer**
(KM `page-image-disk-bounding`); `MEMORY-BOUNDED-INGEST.3` skips persisting it for large docs
**without reducing captured information** (region crops stay full-res). The gap this report
measures is content that reaches **neither** path — i.e. ink on the page inside **no**
segmented bounding box.

## Method (two-stage, resource-bounded)

The persisted `source_ir.json` **summary carries no bounding boxes** (only counts, page-ids,
page dimensions, reading order — confirmed in the `.1` recon). True page-*area* coverage
therefore needs the raw Docling `*.backend.json` (`export_to_dict`), which retains element
`prov` bboxes **and embeds the page raster as base64 PNG**. Those files are large (5.9 MB →
330 MB), so a blind corpus-wide raw parse would stress RAM (`feedback_ram_ceiling_monitor`).
Hence two stages:

- **Stage 1 — cheap count/density proxy over all 79 docs** (from the summary alone): per page,
  count `content_elements` + `structured_tables` + `visual_assets`; flag **structurally-blank**
  pages (0 elements & 0 tables) and, of those, **fully-blank** pages (also 0 visual crops →
  escape both summary paths). Surfaces candidate under-captured pages corpus-wide.
- **Stage 2 — rigorous ink-outside-bbox over the backend-available docs** (RAM-monitored,
  smallest-first, abort-if-free%<25): per page, decode the embedded raster, count **ink**
  (grayscale < 245) and **dark ink** (< 150 — excludes light-gray shading and anti-aliasing),
  build the union of all element bboxes (texts+pictures+tables, dilated 2 px), and measure the
  fraction of ink falling **outside** every box. Dark-ink-outside is the clean signal for
  genuine text/line-art content escaping segmentation. Plus a **bounded human eyeball**: render
  the worst pages with an overlay that paints escaped ink red.

RAM stayed healthy throughout (≥48% free; the 85%-used kill line was never approached);
the doctrine's monitor-and-bound rule held. The probe scripts were throwaway and live only in
`/tmp` (never committed) — matching the project's probe convention.

## Stage 1 — corpus proxy (all 79 docs, 14,762 pages)

| Metric | Count | Share of pages |
| --- | ---: | ---: |
| total `content_elements` | 185,816 | — |
| total `structured_tables` | 10,767 | — |
| total `visual_assets` (region crops) | 18,578 | — |
| structurally-blank pages (0 elements & 0 tables) | 192 | 1.30% |
| — of which region-only (≥1 crop) | 22 | 0.15% |
| — of which **fully-blank** (escape both summary paths) | **170** | **1.15%** |
| `visual_assets` with `diagram_kind = unknown` | 16,180 | (87% of crops) |

Two facts decide the rest of the probe:

- **The 16,180 "unknown diagram_kind" crops are NOT a capture gap.** A region crop exists and
  **is** fed to the VLM; the `diagram_kind` simply defaults to `unknown` when no disambiguating
  caption is present. This is a *classification* observation, not lost content.
- **The 170 fully-blank pages are the only true candidates.** Highest counts: AXI+ACE 27,
  CoreSight-arch 22, Cortex-A76 18, ARM-Debug-Interface 15, GIC-arch 11 (+3 region-only),
  ATB 7 / AHB 7 / CXS 7. Stage 2 tests whether these (and any other high-ink-outside pages)
  actually carry content.

## Stage 2 — ink-outside-bbox (16 backend docs measured; CHI raster-absent)

`dark px-weighted_out` = total dark-ink pixels outside any box ÷ total dark-ink pixels (the
honest corpus aggregate — weights by absolute content, not by sparse-page fractions).

| Doc | size | all-ink out (mean) | **dark px-weighted out** | covered area (mean) |
| --- | ---: | ---: | ---: | ---: |
| risc_v_debug | 74 MB | 0.007 | **0.0000** | 0.538 |
| amba_axi (ihi0022_l) | 167 MB | 0.032 | **0.0002** | 0.334 |
| amba_lti (ihi0089_d) | 55 MB | 0.063 | **0.0005** | 0.264 |
| i2c (um10204) | 44 MB | 0.018 | **0.0006** | 0.450 |
| can (bosch 1991) | 30 MB | 0.146 | **0.0010** | 0.332 |
| swp (etsi) | 31 MB | 0.010 | ~**0.006** | 0.387 |
| i2s (um11732) | 6 MB | ~0.05 | ~**0.01** | 0.18 |
| apb_d / apb_e / axi-stream / atb | 13–18 MB | 0.03–0.05 | <**0.02** | 0.18–0.21 |
| nvme | 330 MB | 0.014 | **0.0250** | 0.492 |
| smbus | 51 MB | 0.018 | **0.0308** | 0.490 |
| amba_axi+ace (ihi0022_h_c) | 202 MB | 0.027 | **0.0430** | 0.274 |
| arm_debug_interface (ihi0074_a) | 158 MB | 0.027 | **0.0454** | 0.249 |
| amba_ahb (ihi0033_c) | 35 MB | 0.038 | **0.0490** | 0.244 |

Read this two ways:

- **The clean signal is near-zero.** Genuine dark text/line-art outside every box is ≤5% even
  for the worst doc, and ≈0 for most (RISC-V-Debug 0.0000, AXI 0.0002, LTI 0.0005, I2C 0.0006,
  CAN 0.001). The big *all-ink* numbers (CAN mean 0.146; LTI max 0.420) are entirely light-gray
  shading + decorative borders/rules — they vanish under the dark-ink threshold.
- **The elevated docs (AHB/ARM-Debug/AXI+ACE ~0.045) are a sparse-page artifact.** Their worst
  pages are the **fully-blank divider pages** themselves: 1,807–2,442 dark pixels each (vs
  ~50k+ on a real content page), where the only dark ink is header/footer furniture text, so the
  *fraction* is high but the *absolute* is a few hundred pixels. `picture_region=False` confirms
  these are not figure pages.

**Fully-blank pages contain 0–2,442 dark pixels** (content pages carry ~50k+) — i.e. they are
near-empty, exactly as the eyeball confirms.

## Bounded eyeball sample (5 pages, red = ink outside every box)

| Page | all-ink out | What escaped (visual inspection) | Verdict |
| --- | ---: | --- | --- |
| **LTI p49** | 0.420 | gray "Note" admonition **shading** + table rule lines + chapter header / copyright footer. Note **body text** verified present as a `body_text` `content_element` (`"No-allocate is treated as Allocate…"`). | decoration + furniture — **captured** |
| **CAN p72** | 0.363 | decorative **full-page border box** + header/footer table grid (1991 typesetting, on every page). All body text captured (blue). | decoration — **captured** |
| **SWP p7** | 0.092 (dark 0.18) | two red **section-heading underline rules** + header rule + footer "ETSI". Every paragraph captured. | decoration + furniture — **captured** |
| **APB_d p12** | 0.215 | running header ("Preface/Feedback") + copyright footer + page number only. A print-layout **blank verso**. | furniture on blank page — **correct exclusion** |
| AHB/ARM-Debug/AXI+ACE fully-blank pages | — | dark_px 0–2,442 (furniture only); near-empty dividers. | blank dividers — **correct exclusion** |

The Note-admonition case is the one that looked like a candidate and is the most instructive:
the red overlay was the **gray box shading**, not lost text — the callout's body text is a
captured `body_text` element. Shaded callouts are captured; only their cosmetic fill is not.

## Honest limits

- **CHI's page raster is absent.** `ihi0050_g` (CHI) was ingested via the bounded batched path
  (`MEMORY-BOUNDED-INGEST.2`/`.3`), whose backend JSON does not embed per-page rasters, so the
  pixel audit covered 16 of 17 backend docs. CHI's *content* is still captured as
  elements/tables; only this audit's raster view of it is unavailable.
- **62 docs have no backend JSON** (host-local sources / cleaned `normalized` bundles), so the
  pixel stage covered 16 docs. Those 16 deliberately include the **two highest fully-blank-count
  docs that still have rasters** (AXI+ACE 27, ARM-Debug 15) and all three document classes — the
  conclusion rests on the worst candidates, not the easy docs. Stage 1 covered all 79.
- **The pixel metric is a high-recall proxy.** It sees only ink the raster shows; a watermark or
  obscured region would register as blank-or-covered. The eyeball found no such case.
- **This tree asked only whether content reaches a capture path — it does.** Whether the VLM
  then *reads a captured figure correctly* is a separate VLM-quality question owned elsewhere,
  not a capture gap.

## Decision

**NO-GO.** Close `FULL-PAGE-INTENT-CAPTURE` with the measured answer that SpecForge already
uses the full scope of a page's intent-bearing visual information. `.2` (whole-page VLM capture
design) stays **proposed/unbuilt** — re-open only if a future doc class (e.g. heavy hand-drawn
inline schematics with no Docling picture region) empirically shows dark content escaping all
boxes on a real page. This matches the standing honesty doctrine
(`feedback_scoring_rigor`, `project_intent_completeness_research`): the residual is real and
named (decoration/furniture/blank pages), and we do not build speculative machinery against a
gap the corpus does not exhibit.
