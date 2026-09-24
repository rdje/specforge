---
id: docling-tables-live-under-documents-in-a-batched-bundle
title: A page-batched Docling bundle keeps its tables under documents[], so a census that reads top-level tables misses most of the corpus
answers:
  - "how many Docling tables does the persisted corpus have"
  - "why does my census of backend.json tables find only 477"
  - "where are the tables in a batched backend.json"
  - "what is the layout of a page-batched normalized backend.json"
  - "how do I load a persisted Docling document to replay table extraction"
  - "why does DoclingDocument.model_validate fail with name Field required"
  - "which corpus documents were ingested in page batches"
date: 2026-09-24
status: current
tags: [source-ir, docling, ingest, tables, census, method]
evidence: generated/source_ir/*/normalized/*.backend.json; crates/specforge/src/ir/source/docling_backend.rs (page-batched conversion, extract_table_grid); docs/tasks/kg-isf-completeness/relation-precision.md (.1d.i.a.i)
reverify: "python3 -c \"import json,glob; n=[len(t) for p in glob.glob('generated/source_ir/*/normalized/*.backend.json') for d in [json.load(open(p))] for t in ([x['tables'] for x in d['documents']] if d.get('batched') else [d['tables']])]; print(len(n), sum(n))\" — expect `45 1256` (document parts, tables); reading only `d.get('tables', [])` sums to 477"
---

**Measured `2026-09-24` (`KG-ISF-COMPLETENESS.1d.i.a.i`).** A persisted `normalized/<key>.backend.json` has two
layouts. A single-pass ingest writes one serialized `DoclingDocument` whose tables are at `tables`. A large
document is converted in page batches, and its bundle is a wrapper instead:

```json
{"batched": true, "page_batches": [...], "documents": [<DoclingDocument>, <DoclingDocument>, ...]}
```

The wrapper has **no top-level `tables`**, so `doc.get("tables", [])` returns nothing for it silently. Over the
retained bundles that reads **477 tables**. Reading each batch's `documents[i].tables` too gives **1,256**. The
three batched documents (AXI `ihi0022_l`, ADIv6 `ihi0074_a`, USB 3.2) hold the other **779**. `.1d.i.a.i`'s
first calibration was taken on the 477 and was re-run on the full 1,256.

The wrapper also refuses `DoclingDocument.model_validate` with `name: Field required`. Validate each element of
`documents` instead. Every batch validates on its own, and its `tables` are real `TableItem`s with the
`.data.grid` that `extract_table_grid` reads. This is how a table repair was replayed over the corpus without
re-running Docling's models.

Batch-local indices restart in each part, so a table's position in the concatenated list is a census index,
not a Docling `self_ref`; [[ingest-drops-figure-interior-text]] records how references behave under batching.
