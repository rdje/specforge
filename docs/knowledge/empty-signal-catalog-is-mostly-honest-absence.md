---
id: empty-signal-catalog-is-mostly-honest-absence
title: Of the 33 documents that declare no signals, 32 have no declaration-bearing modality at all; one declares its wires as section headings
answers:
  - "are the 33 empty signal catalogs a capture miss or honest absence"
  - "which corpus documents are real signal-catalog capture misses"
  - "why does Wishbone declare no signals"
  - "why does USB 3.2 declare no signals"
  - "why does the AMBA DTI specification declare no signals"
  - "why does the Bosch CAN specification declare no signals"
  - "is the empty catalog caused by the table authority gate rejecting real signal tables"
  - "do the register-classed empty-catalog documents deserve the same bar as protocol-classed ones"
  - "why do four corpus documents have no document_class"
  - "which command produces the document_class metric"
  - "how many evidence artifacts have no validation report"
  - "is any corpus document ingested at fewer pages than its source PDF"
  - "is the jesd235 HBM artifact a truncated ingest"
  - "does Wishbone declare its signals in a table"
  - "can a section-title shape alone license a signal declaration"
  - "how many page objects does a /Type/Page regex report vs pdfinfo"
date: 2026-08-11
status: current
tags: [signal-inventory, corpus, extraction-breadth, document-class, honest-absence, source-ir, evidence-ir, adr-0006]
evidence: docs/research/empty-signal-catalog-classification.md; docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md (.1); generated/source_ir/*/source_ir.json; generated/evidence_ir/*/evidence_ir.json; generated/semantic_ir/*/semantic_ir.json; crates/specforge/src/commands/validate.rs (document_class back-annotation)
reverify: "Read-only over the 78 persisted artifact chains. For each document with zero interface signal_records, count (A) structured_tables with table_kind == signal_description whose header row carries a signal/name/symbol/pin identity column, (B) document_sections titles matching ^[A-Z][A-Z0-9_]{1,23}(\\(\\)|\\[[^]]*\\])?$ that contain an underscore or array suffix, and (C) extracted_statements matching ^Signal <ID> is (input|output|inout|internal|local|width). Expect A == 0 and C == 0 on all 33, and B == 32 on wbspec_b4_wishbone_b4_specification against a next-highest of 13. Confirm ingest completeness with `pdfinfo <canonical_path>` against document_profile.page_count: 77 available sources, none short."
---

**Established `2026-08-11` (`SIGNAL-CATALOG-CAPTURE-GAP.1`).** Thirty-three of 78 corpus documents carry zero
interface signal records of any confidence. Classified against three presence probes — an identity-header
`signal_description` table, a bare-identifier section heading, and the formal `Signal <ID> is <predicate>`
prose grammar — **32 are honest absence and exactly one is a capture miss.** The full per-document table is in
[the classification measurement](../research/empty-signal-catalog-classification.md).

**The table gate is not the culprit.** Not one of the 33 has a `signal_description` table with an identity
header; catalog-bearing documents carry up to 19 each. All 41 `signal_description` classifications present
across the 33 were read individually and reduce to 18 distinct header shapes — register, flit-field,
packet-format, 8b/10b encoding, status/fault matrix, and table-of-contents — every one correctly refused. The
prima-facie reading in `SIGNAL-CATALOG-CAPTURE-GAP.0`, that a `protocol`-classed document with zero signals is
a probable miss, survives for one document out of five.

**`wbspec_b4_wishbone_b4_specification` is the capture miss.** It uses a **heading-as-declaration
convention**: 32 sections whose title *is* the wire name (`CLK_I`, `RST_O`, `ACK_I`, `CYC_O`, `DAT_I()`,
`TGD_O()`, `STALL_I`, …), each followed by prose giving direction and meaning. 485 of its 1,622 `SourceIR`
content elements name a `*_I`/`*_O` wire. SpecForge reads declarations from tables and from a formal prose
predicate, and this document uses neither. Its truncated demoted subjects are a separate defect —
[[evidence-statement-markdown-escape-truncates-identifiers]].

**The four other `protocol`-classed documents are honest absence, each for a nameable reason:** USB 3.2's
identifier headings are hub port-feature selectors (`PORT_POWER`, `PORT_LINK_STATE`) and its
`signal_description` tables are LMP formats and the VBUS matrix, already measured false in
[[dense-prose-false-signal-loop-reaches-isf]]; AMBA DTI declares a naming *transformation* (`Direction |
Suffix` → `*_DTI_DN`) over AXI-Stream signals declared in another specification; Bosch CAN 2.0 specifies
frame formats and bit timing, with the wires belonging to the companion physical-layer standard; and the USB4
connection-manager guide self-reports `document_intent_category = methodology-guide`.

**Two corpus facts fell out of the same traversal.**

- `document_class` is produced by the **`validate`** command, which back-annotates it into `EvidenceIR`. The
  four unclassified documents carry **zero validation reports**, so there is no metric surface to read — the
  classifier did not fail on them. This is corpus-wide: **14 of 78** evidence artifacts have no validation
  report at all, including `ihi0022_l_…_amba_axi`, `ihi0024_e_…_amba_5_apb`, and `ihi0033_c_…_amba_5_ahb`.
- **Ingest is complete everywhere.** Across all 77 available sources, no document's `document_profile.page_count`
  is below its PDF page count (`pdfinfo`). `jesd235_2013_10_hbm_dram` looks truncated at 6 pages and 82 content
  elements, but the retained source genuinely *is* a 6-page PDF rather than the full HBM standard — a
  corpus-acquisition gap, not an extraction one. A `/Type/Page` regex over the raw bytes reports 125 for that
  file and is not a reliable page count; use `pdfinfo`.

**A title shape alone must not license a declaration.** The heading probe is a candidate generator whose
every hit was read, not an oracle. On Wishbone it goes 32 real wires → 52 uppercase-shape hits → 69 any-case
hits, and the `()` suffix that legitimately marks `DAT_I()` also matches AMBA DTI pseudocode functions such
as `CombineAllocHints()`. Relaxing the shape to any case across all 33 documents adds zero wire declarations
— only section words, glossary entries, and CamelCase protocol identifiers. The rule
`SIGNAL-CATALOG-CAPTURE-GAP.2` must find is corroborative (a title naming an identifier **and** a body
stating a direction for it), not a wider pattern.
