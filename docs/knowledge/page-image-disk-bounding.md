---
id: page-image-disk-bounding
title: Per-page full-res PNGs are not read downstream — ingest skips persisting them for large PDFs
answers:
  - "are the per-page full-res page images read by any downstream consumer"
  - "does enrich / audit-extraction / recover-register-bits read full-page images or region images"
  - "how is the ingest DISK footprint bounded for very large PDFs"
  - "why is page_image_path null / None for a large document"
  - "what does SPECFORGE_INGEST_SAVE_PAGE_IMAGES do"
  - "why does ingest still generate page images if it does not save them"
date: 2026-06-14
tags: [ingest, docling, disk, memory-bounded, source-ir]
evidence: crates/specforge/src/ir/source/docling_backend.rs (process_converted_document save_page_images gate); docs/tasks/MEMORY-BOUNDED-INGEST.md (.3)
reverify: grep -n "save_page_images\|SPECFORGE_INGEST_SAVE_PAGE_IMAGES" crates/specforge/src/ir/source/docling_backend.rs
---

The full-res per-PAGE PNG (`normalized/pages/page-NNNN.png`, `PageArtifact.page_image_path`) is
read by **no** downstream consumer. Every vision step reads only figure/table **region** crops via
`VisualAsset.image_path`: `enrich` (`commands/enrich.rs`), `audit-extraction`
(`commands/audit_extraction.rs`), `recover-register-bits` (`commands/recover_register_bits.rs`,
`DiagramKind::RegisterBitfield` assets). The IR stages (`evidence`/`semantic`/`intent`) and
`validate`/`converge` only **count** `page_artifacts`, never open the images.

But region images are cropped from the in-memory page image (`PictureItem/TableItem.get_image(doc)`),
so `generate_page_images=True` must stay (disabling it would break region cropping → quality
regression). The disk cost is the explicit `page.image.pil_image.save(...)` write only.

So `MEMORY-BOUNDED-INGEST.3` keeps page-image **generation** (in memory) and skips **persistence**
for large docs: ingest disk becomes `O(assets)` not `O(pages)`, region images stay full-res and
byte-identical, and `page_image_path`/`rendered_image.path` become an honest `None` (the page's
full-res `width_px`/`height_px`/`dpi` are still recorded). Default: skip above
`SPECFORGE_INGEST_BATCH_THRESHOLD` (same gate as batching); persist at/below it (small docs stay
byte-identical). Override either way with `SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1`/`0`. No Rust struct
change — both path fields are already `Option<PathBuf>`. Measured on CAN (72p): per-page PNGs = 21 MB
vs region crops = 1.8 MB; skip mode drops the 21 MB. Related: [[docling-device-cpu]].
