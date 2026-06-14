---
id: full-page-capture-gap
title: SpecForge captures a page's intent-bearing content; the residual outside Docling regions is decoration/furniture (measured NO-GO)
answers:
  - "does SpecForge use the full scope of a page's visual information"
  - "is intent-bearing content slipping through because nothing reads the full page"
  - "should SpecForge add a whole-page VLM read / full-page capture path"
  - "how much page content escapes both the structured-element path and the region-crop path"
  - "what falls outside Docling's segmented bounding boxes on a page"
  - "why was FULL-PAGE-INTENT-CAPTURE.2 not built / closed NO-GO"
  - "are the 'unknown diagram_kind' visual assets a capture gap"
date: 2026-06-14
tags: [intent-capture, completeness, docling, source-ir, vlm, multimodal, no-go]
evidence: docs/research/full-page-capture-gap.md; docs/tasks/FULL-PAGE-INTENT-CAPTURE.md (.1, NO-GO)
reverify: "ls generated/source_ir/*/source_ir.json | wc -l  # corpus size; then re-run the Stage 1/2 method in docs/research/full-page-capture-gap.md against the raw *.backend.json prov bboxes + embedded page rasters"
---

A page reaches SpecForge's pipeline through **two paths only**: (a) Docling structured
text/table/section elements (`content_elements` / `structured_tables` / `document_sections`), and
(b) figure/table **region crops** (`visual_assets[].image_path`) fed to the VLM arm. The raw
**full-page raster** is read by **no** consumer (see [[page-image-disk-bounding]]). The owner asked
(`2026-06-14`, after `MEMORY-BOUNDED-INGEST.4c`) whether intent-bearing content slips through the
gap between segmented regions. `FULL-PAGE-INTENT-CAPTURE.1` measured it — **NO-GO**.

**Measured on the persisted corpus (probe-first, no pipeline/IR code):**

- **Stage 1** (count/density proxy, all 79 docs / 14,762 pages): only **170 fully-blank pages
  (1.15%)** escape both summary paths, and they are print-layout blank versos + part dividers. The
  **16,180 `diagram_kind = unknown` crops are NOT a gap** — the region IS cropped and VLM-fed; the
  kind merely defaults to `unknown` without a disambiguating caption (a classification observation,
  not lost content).
- **Stage 2** (rigorous **ink-outside-bbox** over 16 backend-available docs; raster decoded from the
  Docling backend JSON's embedded base64, bboxes from raw `prov`): the **dark-ink (<150)
  px-weighted-outside aggregate is ≤5% worst-case and ≈0 typical** (RISC-V-Debug 0.0000, AXI 0.0002,
  LTI 0.0005, I2C 0.0006, CAN 0.001). The large *all-ink* figures (CAN mean 0.146, LTI max 0.420)
  are entirely light-gray admonition shading + decorative borders/rules + furniture — they vanish
  under the dark threshold. Elevated per-page dark fractions (AHB/ARM-Debug ~0.045 weighted) trace to
  the **sparse fully-blank divider pages** themselves (1,807–2,442 dark px = header/footer furniture
  only; `picture_region=False`), not content.
- **5-page eyeball** (LTI p49, CAN p72, SWP p7, APB_d p12, fully-blank dividers): every escape is
  decoration (section-heading rules, full-page borders, table grid), callout-box shading (the box
  **text** is a captured `body_text` element), header/footer furniture, or a blank divider — never
  lost intent.

**Conclusion:** SpecForge already uses the full scope of a page's *intent-bearing* visual
information. A whole-page VLM pass (`FULL-PAGE-INTENT-CAPTURE.2`) was deliberately **not** built — it
would add non-determinism + RAM/disk cost (a 930-page doc is exactly the `MEMORY-BOUNDED-INGEST`
size-immunity frontier) for ≈zero recoverable intent. Re-open only if a future doc class empirically
shows dark content escaping all boxes on a real page.

**Honest limits:** CHI's page raster is absent (it was ingested via the bounded batched path which
does not embed per-page rasters; its content is still captured as elements/tables); 62 docs lack a
backend JSON, so the pixel stage covered 16 — but those include the two highest fully-blank-count
docs with rasters (AXI+ACE 27, ARM-Debug 15) and all three document classes. Whether the VLM then
*reads* a captured figure correctly is a separate VLM-quality question, not a capture gap. Related:
[[page-image-disk-bounding]], [[project_intent_completeness_research]].
