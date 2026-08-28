---
id: source-ir-reingest-trades-captions-for-figure-text
title: Re-ingesting the corpus is a trade, not a refresh — it loses caption bindings to gain figure-interior text
answers:
  - "should the drifted SourceIR documents be re-ingested"
  - "is a fresh ingest better than the persisted SourceIR"
  - "what would re-ingesting the corpus cost"
  - "why did a figure caption become null after re-ingest"
  - "why does caption_text disappear on re-ingest"
  - "does current Docling lose captions"
  - "what kind of content does the ingest drift add"
  - "is the ingest drift purely additive"
  - "how many caption bindings does the corpus lose on re-ingest"
  - "which task owns the re-ingest decision"
  - "does re-ingesting lose three paragraphs"
  - "did the ingest drift lose any content"
date: 2026-08-28
status: current
tags: [source-ir, ingest, docling, captions, evidence-quality, measurement-integrity]
evidence: docs/research/source-ir-reproducibility-census.md; docs/research/ingest-content-loss-adjudication.md; docs/tasks/SOURCE-IR-REPRODUCIBILITY.md; scripts/measure_source_ir_reproducibility.py
reverify: "python3 -c \"import json;d=json.load(open('generated/source_ir/102196_0100_01_2022_05_05_aarch64_external_debug_guide/source_ir.json'));print(sum(1 for x in d['structured_tables']+d['visual_assets'] if x.get('caption_text')))\""
---

`SOURCE-IR-REPRODUCIBILITY.1` re-ingested the whole live corpus and found 13 of 24 documents no longer
reproduce their persisted `SourceIR` ([[source-ir-ingest-not-reproducible]]). The obvious repair — re-ingest
everything so the persisted artifacts match what the toolchain produces — is **not** obviously an improvement,
because the drift moves in two directions at once.

**What a re-ingest gains.** 1,804 additional content elements across the 13 drifted documents. All but two are
`body_text`, and the text is unambiguously figure interior: `Core`, `External Debugger +`, `APB`,
`Referenced to`, `Ideal Clock`, `requency (GHZ)` (a truncated plot axis label), `LE`/`BE`, `MSb LSD`,
`OpenCores`. Current Docling reads text out of diagram and plot regions that the earlier run left alone. Most
of it is diagram furniture rather than specification prose.

**What a re-ingest costs.** Caption bindings on `structured_tables` and `visual_assets` fall from 1,191 to
1,152 across the live population — seven documents lose bindings, one gains two. The Arm external-debug guide
drops from eight to five; USB 3.2 from 482 to 458. The mechanism is visible in a single field:
`structured_tables[1].caption_text` is `Figure 5-1: External debugger and core handshake sequence` in the
persisted artifact and `null` in the replay.

**What is lost is the binding, not the text**, and the mechanism is measured rather than inferred.

SpecForge computes no captions of its own. `docling_backend.rs` calls Docling's own accessor,
`normalize_text(element.caption_text(doc))`, which resolves the item's `captions` list of `$ref` pointers into
the document's `texts` array; `normalize_text` maps the empty string to `None`, so an empty `captions` list is
exactly the `caption_text: null` that appears in `SourceIR`.

Comparing the persisted and re-ingested Docling documents for the Arm external-debug guide shows the layout
model is **not** mislabelling captions. Both runs label the same seven texts `caption` — none lost, none
gained. What breaks is the **assignment**: items carrying a caption reference fall from seven to five.
`pictures/4` loses `Figure 3-1: Debug state entry and exit` and `tables/1` loses
`Figure 5-1: External debugger and core handshake sequence`, while both captions still exist, still labelled
`caption`, attached to nothing.

The trigger is visible in the same comparison: `texts` rises 373 → 469, and the entire +96 lands in
`label: text` — the figure-interior fragments. Docling assigns a caption to a figure by proximity and
containment, so additional text regions detected around a figure can displace or defeat that assignment.

**So the census's two results are one cause, not two.** The same upstream change — more text regions detected
inside figures — produces both the added content elements and the lost caption bindings. That matters for
`.5`: the trade may be *intrinsic* rather than incidental, so "take the new elements and keep the old
bindings" may not be an available option.

That is a real regression for this pipeline specifically. SpecForge's roadmap treats captions as first-class
evidence rather than decoration, and caption-mediated coverage is a measured surface
(`scripts/measure_caption_mediated_coverage.py`). Trading a bound caption for an unbound text fragment costs
the binding that made it evidence.

**A re-ingest does not lose any paragraph.** `.1` reported three elements — one each in the USB4 Connection
Manager guide, USB 3.2, and Wishbone — as content the toolchain emits nowhere, distinguishing them from the
other 28 dropped elements, which are re-segmentation. `SOURCE-IR-REPRODUCIBILITY.5` re-measured those three
against the converter's own document and **withdrew that finding**: every word of all three is still present,
in order, each interrupted by an inserted figure fragment (6, 5, and 35 tokens). `.1`'s test asks whether the
persisted text is a substring of the joined replayed element stream, and a sentence with something spliced
into it is not. All 31 dropped elements are re-segmentation and the measured content loss is **zero**
([[ingest-drops-figure-interior-text]]).

So the caption-binding delta is the whole cost, and it is not a decision this tree may offer: the owning tree
withdrew the re-ingest-versus-retain framing on `2026-08-28` because both branches are defective — one retains
artifacts the toolchain cannot reproduce, the other degrades caption evidence — and a defect is not an option.
`SOURCE-IR-REPRODUCIBILITY.6` owns rebuilding the binding deterministically, and `.3` owns declaring the
chain-currency exclusion while it stands.
