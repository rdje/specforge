---
id: split-identifier-name-cell-joins-only-from-the-document
title: A name cell split by the text layer cannot be rejoined by its shape — the column signature selects 120 columns and almost all are prose; only the document's own joined spelling is precise, and for TileLink that spelling lives inside a figure
answers:
  - "why does TileLink declare signals named C, D, V and R"
  - "can SpecForge rejoin a name cell the PDF split into two words"
  - "is a repeated leading token enough to tell a split identifier from a phrase"
  - "how many name columns share a leading token across their rows"
  - "where does TileLink spell a_opcode"
  - "does the ingest lose underscores"
  - "is the underscore loss the same defect as the markdown escape truncation"
  - "why is c opcode read as the signal c"
  - "how do I tell Clock source from c opcode"
  - "what evidence would let two tokens be joined into one identifier"
  - "does a document ever spell the joined form of a split name cell"
  - "is text inside a timing diagram available to the extractor"
  - "why does eMMC declare a signal called t"
  - "what is a subscript split and why can no underscore rule find it"
  - "should a name recovered from a figure ground a table row identity"
date: 2026-09-14
status: current
tags: [source-ir, evidence-ir, ingest, declarations, identifier-tokenization, adr-0006, text-layer, vlm, text-layer-identifier-split]
evidence: scripts/measure_split_identifier_name_cells.py; scripts/measure_name_column_whole_cell_score.py; docs/tasks/TEXT-LAYER-IDENTIFIER-SPLIT.md (.0); generated/evidence_ir/tilelink_1_8_0_specification/evidence_ir.json (visual_0021/0023/0024); .cache/local-references/chipdoc/risc-v/interfaces/tilelink/current/TileLink-1.8.0_Specification.pdf
reverify: "python3 scripts/measure_split_identifier_name_cells.py — expect CURRENT 116 tables / 10 signature columns, 0 chosen as names, 0 with an in-document join; LEGACY 457 tables / 110 signature columns, 9 chosen, 3 with a join (all AMD IOMMU). A current-stratum column that IS chosen means a new document has entered the corpus and the adjudication must be redone."
---

A specification writes `c_opcode`. Its PDF typesets the underscore, the text layer does not carry it, and
the reader sees the cell `c opcode` and keeps the first token. The wire is not dropped and it is not
refused — it is **renamed to `c`**, and so is every other row of that table, so the column offers exactly
one distinct name and loses the table to whichever neighbour offers more. TileLink's four channel tables
go to their one-letter `Type` column this way.

This is **not** `[[evidence-statement-markdown-escape-truncates-identifiers]]`. There the underscore is
present and escaped (`CYC\_O`) and the identifier is truncated at the backslash. Here the underscore is
**absent**, so there is nothing to unescape and nothing to truncate.

It is also **not** the whole-cell column score refusing a name column that carries spaces. TileLink's
column scores **1** under the leading-token rule that score replaced and 0 under the rule it is, while the
`Type` column scores **4** under both — so the rotation predates that score entirely
(`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`).

## The shape is not the fact

The obvious signature is a column that repeats its leading token down every row while its continuations
differ: `c opcode` / `c param` / `c valid` all lead with `c`, whereas `Clock source` / `Reset controller`
repeat nothing. Measured over every table the declaration reader examines:

| stratum | tables | columns with the signature | chosen as the name column | with an in-document join |
| --- | ---: | ---: | ---: | ---: |
| current (proof-carrying) | 116 | 10 | **0** | 0 |
| legacy (inspection-only) | 457 | 110 | 9 | **3** |

**Every one of the current stratum's ten is a description column** whose sentences open with the same
word — `Transaction identifier` / `Transaction address`, `Global clock` / `Global reset`, `User request` /
`User write`. Five of the nine legacy columns the reader actually chooses are not split identifiers
either: `Write address` names an AXI *channel*, `Redundant Data` is a function. The signature describes
the shape of the defect and selects mostly prose, which is this repository's standing finding about cheap
structural rules, arriving for the fourth time in this area.

## Only the document's own joined spelling is precise — and it is usually absent

Asking whether the document's text anywhere spells `<lead>_<continuation>` selects **3 of the 120**: AMD
IOMMU field tables that also write `iommu_info` and `iommu_attributes`. Three correct, none wrong. It is
the only test in this area with no false positive — and it ships nothing, because all three tables declare
nothing at all. A rule whose entire measured effect is zero declarations has no evidence behind it.

## Where TileLink's joined spelling actually is

The TileLink 1.8.0 PDF's text layer carries exactly **36 underscores, on 5 pages**: `a_opcode`, `a_valid`,
`a_ready`, `a_size`, `a_source`, `d_opcode`, `d_valid`, `d_ready`, `d_size`, `d_source`. All of them are
vector text inside three timing diagrams — *Figure 4.1 Ready-Valid Signaling*, 4.3 and 4.4. Every prose and
table occurrence in the same document spells the same names with a space.

The ingest captures those figures as images, which is correct, so the persisted SourceIR holds **zero**
underscore characters anywhere. The joining is therefore stated in the document, absent from every text
surface SpecForge reads, and present in a surface SpecForge already stores:
`picture-0007/0009/0010.png`, whose only recorded observations are the source document's own captions. The
repair, if there is one, is a VLM read — and the question it raises first is whether a name recovered from
a **figure** may ground a table row's identity or only corroborate it.

## A second notation, with no separator at all

eMMC `table_0221` declares from `t PERIOD` and `t TLH , t THL`; the document writes `tPERIOD`, `tTLH`,
`tTHL`. The join is concatenation, not `<a>_<b>`, so no underscore-keyed rule reaches it — and that cell
carries a comma family as well, so one cell needs two readings at once. Tracked separately, because a rule
that concatenates two words leaves no mark of having guessed.

Links: [[evidence-statement-markdown-escape-truncates-identifiers]],
[[a-width-cell-that-is-a-sentence-is-not-a-width]],
[[declared-population-is-not-the-candidate-row-population]].
