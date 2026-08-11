---
id: evidence-statement-markdown-escape-truncates-identifiers
title: EvidenceIR statement text comes from the normalized markdown, where `_` is escaped, so every underscore-bearing identifier is truncated at the backslash
answers:
  - "why is a constraint subject CLK when the document says CLK_I"
  - "why are extracted signal names truncated at the underscore"
  - "is identifier truncation a separate extractor defect or the same empty-catalog gap"
  - "does the extractor cut identifiers at the underscore character"
  - "why does a Wishbone constraint name CYC instead of CYC_O"
  - "where does EvidenceIR statement text come from"
  - "do EvidenceIR statements read SourceIR content elements or the normalized markdown"
  - "why does SourceIR carry CLK_I but EvidenceIR carries CLK\\_I"
  - "how many corpus documents carry markdown-escaped underscores in their statements"
  - "does markdown escaping cost recall on documents that already have a signal catalog"
  - "what produces the backslash in an EvidenceIR statement text"
date: 2026-08-11
status: current
tags: [evidence-ir, source-ir, ingest, normalization, docling, signal-inventory, identifier-tokenization, recall, adr-0006]
evidence: generated/source_ir/wbspec_b4_wishbone_b4_specification/normalized/wbspec_b4_wishbone_b4_specification.md (line 394); generated/source_ir/wbspec_b4_wishbone_b4_specification/source_ir.json (content_elements — unescaped); generated/evidence_ir/wbspec_b4_wishbone_b4_specification/evidence_ir.json (extracted_statements + signal_constraints.source_text — escaped); docs/research/empty-signal-catalog-classification.md; docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md (.1)
reverify: "Read-only over persisted artifacts. Count EvidenceIR statements matching the regex `[A-Za-z0-9]\\_[A-Za-z0-9]` per document: expect 67 of 78 documents non-zero, Wishbone 455 of 1,501. Run the same regex over that document's SourceIR content_elements and structured-table cells: expect 0 of 1,622 and 0 of 739. Then confirm the loss reaches typed facts: every Wishbone signal_constraints[].subject_signal is a truncated stem (CLK, CYC, STB, RST, STALL) while its own source_text spells the escaped full name ([CYC\\_O])."
---

**Established `2026-08-11` (`SIGNAL-CATALOG-CAPTURE-GAP.1`).** `EvidenceIR` statement text is read from the
**normalized markdown** the ingest writes (`generated/source_ir/<key>/normalized/<key>.md`), not from
`SourceIR`'s typed `content_elements`. That markdown is a Docling export, and it escapes `_` as `\_`. Every
`evidence_spans[]` entry names that file as its `source_path`, with a line range into it.

The two surfaces therefore disagree on the same sentence:

| Surface | Text |
| --- | --- |
| `SourceIR.content_elements` | `… the [ADR_O] and [ADR_I] signals are connected together.` |
| normalized markdown, line 394 | `… the [ADR\_O] and [ADR\_I] signals are connected together.` |
| `EvidenceIR.extracted_statements` | `… the [ADR\_O] and [ADR\_I] signals are connected together.` |

Identifier tokenization stops at the backslash, so the extracted subject is the stem before the underscore.
This is directly observable in typed facts: every `signal_constraints[].subject_signal` on
`wbspec_b4_wishbone_b4_specification` is a truncated stem — `CLK`, `CYC`, `STB`, `RST`, `STALL` — while the
constraint's own `source_text` field preserves the escaped full name:

```text
subject_signal = "CYC"
source_text    = "The cycle output [CYC\_O], when asserted, indicates that a valid bus cycle is in progress."
```

**This answers the open question `SEMANTIC-EMPTY-CATALOG-FILTER` declined to claim.** The truncation is *not*
a rule that cuts identifiers at `_`, and it is not the same event as the missing catalog. It is a lossy text
surface upstream of tokenization. A document whose signal names carry no underscore is unaffected; a document
using an underscore-suffix convention loses the suffix on every prose mention.

**Scope, measured over the persisted corpus:** 67 of 78 documents carry at least one escaped statement.
It is not confined to documents with empty catalogs — 40 of the 67 already have a signal catalog, so this
costs prose recall there too rather than only blocking catalog formation:

| Document | escaped statements / total | declared signals |
| --- | ---: | ---: |
| `ihi0069_g_…_generic_interrupt_controller` | 3,677 / 14,671 | 6 |
| `ihi0070_e_a_…_system_memory_management_unit` | 2,866 / 13,423 | 1 |
| `100798_0401_00_…_cortex_a76_trm` | 947 / 9,534 | 205 |
| `wbspec_b4_wishbone_b4_specification` | 455 / 1,501 | 0 |
| `ihi0022_l_2025_08_amba_axi_protocol_specification` | 359 / 6,407 | 980 |

The fix is structural and ADR-0006-safe — the escape is a property of the markdown serialization, not of any
document's vocabulary — but it is **not** a free change: unescaping shifts statement text on 67 of 78
documents, so it moves the extraction fixed point corpus-wide and must be measured, not assumed. It is owned
as its own leaf rather than folded into a catalog rule.

Related: [[semantic-grounding-filter-is-catalog-independent]] is what made the truncated subjects visible, by
demoting them into a `semantic_ungrounded_records_not_promoted` packet instead of promoting them silently.
[[dense-prose-false-signal-loop-reaches-isf]] is the opposite failure — names that should never have been
admitted — and is the reason the repair here must not simply lower the declaration bar.
