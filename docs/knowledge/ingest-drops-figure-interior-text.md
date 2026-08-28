---
id: ingest-drops-figure-interior-text
title: Ingest silently discards every text the converter put inside a figure — 31% of converter items are dropped as a defect
answers:
  - "does SpecForge ingest lose content from the PDF"
  - "how much of the Docling document reaches SourceIR"
  - "how much of the converter output does ingest drop as a defect"
  - "is the 46 percent figure the defect rate"
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
  - "what is source_batch in SourceIR"
  - "how do I address exactly one converter item"
  - "why does source_batch not appear on my artifact"
  - "how many persisted artifacts have an ambiguous source_ref"
  - "why did a paragraph gain words after re-ingest"
  - "can a SourceIR element contain text from two places"
  - "which task owns the ingest conservation gap"
  - "is the traverse_pictures mechanism measured or only read from source"
  - "does the drop model agree with docling iterate_items"
  - "how is the figure-interior drop mechanism verified"
  - "what does the ingest traversal oracle check"
  - "how many converter text items does iterate_items yield across the corpus"
date: 2026-08-28
status: current
tags: [source-ir, ingest, docling, conservation, captions, measurement-integrity, provenance]
evidence: docs/research/ingest-content-loss-adjudication.md; docs/tasks/SOURCE-IR-REPRODUCIBILITY.md; scripts/measure_ingest_content_loss.py; crates/specforge/src/ir/source/docling_backend.rs
reverify: "python3 scripts/measure_ingest_content_loss.py --output-root .project-data/tmp/reverify-figure-interior --census-id reverify-figure-interior --owner SOURCE-IR-REPRODUCIBILITY.11 --oracle --document um10204_rev7_0_2021_i2c_bus_specification"
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
Manager guide, USB 3.2, Wishbone), **5,896 of 18,870 converter text items — 31% — are dropped as a defect**:
they reach no `SourceIR` record, earn no residual, and should have reached one. The raw non-carry rate is
8,648 items (46%), but it bundles in exclusions ingest is right to make, so the defect figure is the one that
leads (`SOURCE-IR-REPRODUCIBILITY.12`). Every item resolves to a named predicate, with an empty `unexplained`
bucket:

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

**The mechanism is measured, not inferred.** The paragraph above describes docling-core's traversal;
`SOURCE-IR-REPRODUCIBILITY.11` stopped it from resting on a reading of that library's source. The
producer's `--oracle` mode loads each persisted converter document back through
`DoclingDocument.model_validate` and calls `doc.iterate_items()` with production's own arguments, then
compares what the library yields against what the drop model predicts — both directions, never netted,
per batch. Across **all 24** persisted artifacts whose converter document was retained: 43,614
converter text items, **22,127 yielded and 22,127 predicted, 0 disagreements**, every document
round-tripping through its own serialization. The residue closes too — 39 empty `formula` items the
backend helper drops, leaving exactly the 22,088 content elements the live population holds, with
`unexplained` 0 everywhere. Six observed RED perturbations keep the comparator from agreeing by
construction.

Two related facts fell out of the same measurement:

- **`source_ref` did not identify one converter item under bounded-memory ingest, and now `source_batch`
  does.** A batched conversion writes one converter document per page range and Docling's `self_ref`
  restarts at zero in each, so the bare ref is ambiguous: measured over all 78 persisted artifacts, **14**
  are in that state, holding 111,861 content elements over 28,599 distinct refs — the Arm Debug guide's
  6,784 elements carry 2,252 refs with 1,883 used more than once, and USB 3.2 is nine batches.
  `SOURCE-IR-REPRODUCIBILITY.9` added `source_batch` to the four `source_ref`-bearing record kinds,
  written only when the run is actually batched, so `(source_batch, source_ref)` is exact from that
  revision forward. Artifacts already on disk carry no coordinate and cannot gain one after the fact;
  the consumer reports which key each artifact supports. That the ambiguity is batching and not a
  duplicate-emitting producer is settled by the retained converter bundles: across the 24 artifacts whose
  bundle survives, 22 are unambiguous and unbatched and 2 are ambiguous and batched, with zero
  disagreements either way.
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
