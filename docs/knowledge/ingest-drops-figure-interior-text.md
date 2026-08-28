---
id: ingest-drops-figure-interior-text
title: Ingest silently discards every text the converter put inside a figure — 46% of converter items reach no SourceIR record
answers:
  - "does SpecForge ingest lose content from the PDF"
  - "how much of the Docling document reaches SourceIR"
  - "why do figure labels not appear in content_elements"
  - "what does traverse_pictures False do to SpecForge ingest"
  - "why is text inside a diagram missing from SourceIR"
  - "is PDF to SourceIR conservation measured"
  - "what conserves between the PDF and SourceIR"
  - "which converter items earn no residual"
  - "how many captions does ingest discard"
  - "is the figure-interior gap drift or standing"
  - "why is source_ref ambiguous"
  - "does source_ref identify one Docling item"
  - "what happens to self_ref under batched ingest"
  - "why did a paragraph gain words after re-ingest"
  - "can a SourceIR element contain text from two places"
  - "which task owns the ingest conservation gap"
date: 2026-08-28
status: current
tags: [source-ir, ingest, docling, conservation, captions, measurement-integrity, provenance]
evidence: docs/research/ingest-content-loss-adjudication.md; docs/tasks/SOURCE-IR-REPRODUCIBILITY.md; scripts/measure_ingest_content_loss.py; crates/specforge/src/ir/source/docling_backend.rs
reverify: "python3 scripts/measure_ingest_content_loss.py --output-root .project-data/tmp/reverify-figure-interior --census-id reverify-figure-interior --owner SOURCE-IR-REPRODUCIBILITY.5 --persisted --document um10204_rev7_0_2021_i2c_bus_specification"
---

SpecForge's Docling helper builds `content_elements` from `doc.iterate_items()`, called with the default
`traverse_pictures=False`. In docling-core that default means `iterate_items` **skips every child of a
`PictureItem` except the refs listed in that picture's own `captions`**, and the skip happens at the boundary,
so every descendant of a blocked child goes with it. A `list` group inside a figure takes its list items down
too — which is why detecting this needs the ancestor chain, not the direct parent: in the I2C specification
nine list items sit under `#/groups/41`–`44` and `#/groups/65`–`69`, whose parents are `#/pictures/61` and
`#/pictures/82`.

Nothing records what is skipped. There is no `content_element`, no residual decision, and no counter.

`SOURCE-IR-REPRODUCIBILITY.5` measured the size of it. Across three re-ingested documents (USB4 Connection
Manager guide, USB 3.2, Wishbone), **8,648 of 18,870 converter text items — 46% — reach no `SourceIR` record
and earn no residual**. Every one resolves to a named predicate, with an empty `unexplained` bucket:

| Reason | Items | Correct? |
| --- | ---: | --- |
| `picture_interior_not_traversed` | 5,896 | **no** |
| `content_layer_excluded` (running headers/footers, `furniture` layer) | 2,722 | yes |
| `empty_text` (`formula` items with no text) | 30 | yes |

Most of the 5,896 is diagram furniture (`Tx_0`, `Router A`, `Back to TOC`), but not all: it also discards
9 `caption`, 8 `footnote`, and 4 `section_header` items.

**This is standing, not drift.** The same census runs against the persisted artifacts with no ingest at all
(`--persisted`). The persisted I2C specification — which the `.1` reproducibility census scores as reproducing
**byte-for-byte** — discards 1,372 figure-interior items, 39 of them captions. Reproducibility and
conservation are independent properties, and only the first was measured before now. The gap is also
document-dependent: the Arm external-debug guide discards none, because its converter document places no text
inside figures at all.

Two related facts fell out of the same measurement:

- **`source_ref` does not identify one converter item under bounded-memory ingest.** A batched conversion
  writes one converter document per page range and Docling's `self_ref` restarts at zero in each, so the
  recorded ref is ambiguous. The Arm Debug guide's 6,784 content elements carry only 2,252 distinct
  `source_ref` values, 1,883 used more than once; USB 3.2 is nine batches. Any conservation gate has to join
  converter items to `SourceIR` records, so this blocks the gate (`SOURCE-IR-REPRODUCIBILITY.9`).
- **Preservation and faithfulness have come apart.** The three paragraphs `.1` called lost are all present
  word for word — because current Docling spliced a figure fragment into the middle of each. `SourceIR` now
  carries `… by reading the USB4 Host Enhanced SS Host Controller ROUTER_CS_6. Gen T Full Connectivity Support
  field …`, which is not a sentence the specification contains. A conservation check cannot see this: nothing
  was lost (`SOURCE-IR-REPRODUCIBILITY.10`).

The remedy is **not** to promote figure-interior text into `content_elements` as prose — that is exactly how
the interleaving above happens. It is that the pipeline's own doctrine requires an unresolved thing to become
an explicit residual rather than disappear, and 5,896 items disappear.
`SOURCE-IR-REPRODUCIBILITY.8` owns the carrier, `.6` the unbound captions, and `.7` the gate.

Related: [[source-ir-reingest-trades-captions-for-figure-text]], [[source-ir-ingest-not-reproducible]].
