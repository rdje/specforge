# Literature grounding — Document structured extraction

*Aspect 1 of `LITERATURE-GROUNDING`. SpecForge's input layer: turning a PDF into a
richly-structured representation (layout, reading order, tables, figures/captions).
All citations below were web-verified (real venue/arXiv id); none are guessed.*

## Prior art (verified)

- **Docling — Technical Report**, Auer et al., 2024. arXiv:2408.09869. The open-source
  toolkit SpecForge ingests with (pinned `docling==2.84.0`). Pipeline: page images →
  layout analysis → table structure recovery → unified structured document.
- **Docling: An Efficient Open-Source Toolkit for AI-driven Document Conversion**,
  2025. arXiv:2501.17887.
- **TableFormer: Table Structure Understanding with Transformers**, Nassar et al.,
  CVPR 2022. arXiv:2203.01017. The vision-transformer table-structure model Docling
  uses; handles borderless/empty cells, spans, hierarchical headers; raises TEDS from
  ~91%→98.5% (simple) and 88.7%→95% (complex) over the prior PubTabNet end-to-end model.
- **DocLayNet: A Large Human-Annotated Dataset for Document-Layout Analysis**,
  Pfitzmann et al., KDD 2022. arXiv:2206.01062 (DOI 10.1145/3534678.3539043). 80,863
  hand-annotated pages, 11 layout classes, COCO format; Docling's layout model
  (RT-DETR-based) trains on it. Explicitly motivated by the *layout-variability* gap in
  prior datasets (PubLayNet, DocBank — both sourced only from PubMed/arXiv scientific
  papers).
- **PubTables-1M: Towards comprehensive table extraction from unstructured documents**,
  Smock et al., Microsoft, CVPR 2022. arXiv:2110.00061. ~1M tables; the Table
  Transformer (TATR) baseline; introduces the **GriTS** metric and a *canonicalization*
  procedure that fixes ground-truth **oversegmentation**; covers three tasks —
  detection, structure recognition, and **functional analysis** (header vs data roles).

Standard evaluation metrics named above: **TEDS** (Tree-Edit-Distance-based Similarity)
for table structure, **GriTS** for grid table similarity.

## Alignment (where SpecForge already matches the literature)

SpecForge does **not** reinvent document AI — its `SourceIR` ingest layer *is* the
current SOTA stack: Docling = RT-DETR/DocLayNet layout analysis + TableFormer table
structure recognition. The design choice (`bootstrap_docling.sh` → `.venv-docling`) is
validated by the literature: this is the leading open, commodity-hardware toolkit for
exactly the "PDF → structured page/table/figure" problem SpecForge's input stage solves.

## Adopt (proven techniques worth borrowing)

- **TEDS / GriTS as table-structure eval metrics.** SpecForge has no supervised metric
  for *how well it recovers a table's structure* (its new `eval-extraction` measures
  downstream signal/relation extraction, not table grids). Borrowing TEDS/GriTS would let
  SpecForge quantify ingest-stage table fidelity per corpus doc.
- **Functional analysis (header vs data) — PubTables-1M.** SpecForge's own corpus work
  surfaced table-role confusion (the CHI DVM "encoding" tables and column-less signal
  tables — see `REGISTER-CLASSIFIER-ENCODING-FP`, `SIGNAL-TABLE-COLUMNLESS-RECALL`).
  PubTables-1M's functional-analysis formulation (classifying cells/rows into header vs
  data roles) is established prior art SpecForge could lean on to disambiguate header
  rows from data rows before its domain `classify_table_kind` runs.
- **Canonicalization against oversegmentation.** PubTables-1M's oversegmentation insight
  maps onto a real SpecForge failure mode (Docling marking *every* row a header → empty
  `body_rows`, the CHI Class-A tables); the canonicalization framing is a reference for a
  future fix.

## Extend / genuine novelty (the out-of-the-box part)

The document-AI literature **stops at the structured document** (layout boxes + table
grids + reading order). SpecForge's novelty is everything *above* that: it treats the
structured document as **evidence**, then lifts typed *protocol intent* through staged
IRs (`SourceIR → EvidenceIR → SemanticIR → IntentIR`) — actors, signals, relations,
temporal contracts. No document-AI paper does protocol-intent recovery; that is
SpecForge's contribution, and document AI is (correctly) only its front-end.

## Gaps / opportunities → candidate future trees

1. **No ingest-stage table-fidelity metric.** Adopt TEDS/GriTS to measure table-structure
   recovery on the corpus → a new eval surface (sibling to `LLM-EXTRACTION-EVAL`).
2. **Table-role/functional analysis is domain-bolted, not principled.** SpecForge's
   `classify_table_kind` heuristics (which the corpus work repeatedly had to fix) could be
   re-grounded on the PubTables-1M functional-analysis formulation (header/data roles) as a
   precise pre-step.
3. **Oversegmentation (all-header tables) is unhandled** (CHI Class-A residual). The
   canonicalization literature is the reference for closing it.

## Links

- Task-tree: `LITERATURE-GROUNDING`. Related SpecForge trees: `REGISTER-CLASSIFIER-ENCODING-FP`,
  `SIGNAL-TABLE-COLUMNLESS-RECALL`, `CORPUS-HARDENING`, `LLM-EXTRACTION-EVAL`.
