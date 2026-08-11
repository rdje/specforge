---
id: semantic-grounding-filter-is-catalog-independent
title: One grounding predicate governs every document; rejected records are demoted, not dropped
answers:
  - "does an empty declared-signal catalog disable the SemanticIR grounding filter"
  - "where did my signal constraint go between EvidenceIR and SemanticIR"
  - "why is a conditional rule in EvidenceIR but not in SemanticIR"
  - "what is the semantic_ungrounded_records_not_promoted residual packet"
  - "are ungrounded SemanticIR records dropped silently or demoted"
  - "which conditional rules survive when a document declares no signals"
  - "did the symmetric grounding filter change any emitted .isf"
  - "how many documents lost promoted records when the empty-catalog special case was deleted"
  - "why does a SemanticIR residual packet list only some undeclared signal names"
date: 2026-08-11
status: current
tags: [semantic-ir, grounding, signal-authority, conditional-rules, signal-constraints, residual-decisions, adr-0006]
evidence: crates/specforge/src/ir/semantic.rs (declared-signal gating + `ungrounded_promotion_residual_packet`); docs/tasks/SEMANTIC-EMPTY-CATALOG-FILTER.md; docs/book/src/pipeline/semanticir.md
reverify: "Replay every document read-only and classify the result: for each generated/evidence_ir/<k>/evidence_ir.json run `specforge semantic <path> --dry-run`, strip `validation_reports`, and diff against generated/semantic_ir/<k>/semantic_ir.json — expect it to be current at 78/78 (`bash scripts/check_chain_currency.sh`). To re-derive the landed rule's effect, recompute promotion from EvidenceIR against each document's non-low-confidence declared-signal set: 11 documents unchanged, 41 changed only in `residual_decisions`, 26 content-moved and all 26 with an empty declared catalog."
---

**Established `2026-08-11` (`SEMANTIC-EMPTY-CATALOG-FILTER.1`).** `SemanticIr::build` promotes an `EvidenceIR`
signal constraint only when its `subject_signal` is in the document's declared-signal catalog, and a conditional
rule only when its `consequent_signal` is declared **or absent** (a genuine system-level behavioral rule). The
catalog is built from interface `signal_records` excluding `AutomationConfidence::Low`.

**There is no empty-catalog escape hatch.** The former `if declared_signal_names.is_empty() { … clone() }` guard
is gone. An empty catalog now simply grounds no named subject — the intended outcome rather than a separate
branch. This supersedes [[semantic-empty-catalog-disables-grounding-filter]], which recorded the inverted
behavior: the filter used to be strongest on documents *with* signal authority and absent on documents with
none, which is exactly where an ungrounded record is least likely to be real.

**Rejected records are demoted, not dropped.** They stay in `EvidenceIR` with full provenance, and `SemanticIR`
emits one proportionate `semantic_ungrounded_records_not_promoted` residual packet per affected document, stating
both counts, the declared-catalog size, and a **sorted, capped** sample of the undeclared names
(`UNGROUNDED_PROMOTION_SAMPLE_LIMIT = 12`, remainder elided as "and N more" — one document rejects 216 rules, and
an unbounded list would ride into every `IntentIR` and adapter artifact). A document with nothing rejected gets
no packet. The packet flows to `IntentIR` and `adapter.json` like any other residual decision.

**Measured effect over the 78-document corpus** (read-only `semantic --dry-run` replay against the persisted
artifacts): **11 identical · 41 changed only in `residual_decisions` · 26 content-moved**, and all 26 have an
empty declared catalog — so no populated-catalog document lost a promoted record. Empty-catalog promotion falls
`1,423 → 780` conditional rules (the 780 naming no signal are kept) and `100 → 0` signal constraints. The
populated branch was *already* dropping 1,230 rules and 47 constraints silently on 41 of 45 documents; uniform
demotion surfaces those as residuals, which is why the tree's acceptance bar was revised from byte-identity to
"added residuals only".

**The product boundary did not move.** After rebuilding all 78 downstream chains from unchanged `EvidenceIR`,
all **44 emitted `.isf` are byte-identical** and pass FSMGen `--strict --check` with zero diagnostics;
`kg-bench` holds `156/156`; `CHAIN-CURRENCY` is green at 24/78/78/78. An empty catalog already blocked the `.isf`
adapter on `no signals declared in interface`, so no unfiltered record had ever reached an emitted target.

**What the packet is good for.** If the names it lists read as real wires rather than boilerplate, the document's
**signal catalog was never captured** and the gap is upstream in signal extraction, not in this filter. All 33
empty-catalog corpus documents carry **zero** interface signal records of any confidence — none has a merely
low-confidence catalog — and their rejected subjects mix obvious noise (`DATASHEET`, `MUST`, `PCI`,
`IMPLEMENTATION`, `PDF`) with real tokens truncated at an underscore or suffix (Wishbone `CLK`/`CYC`/`STB` for
`CLK_I`/`CYC_O`/`STB_O`, AMBA DTI `TDATA`/`TKEEP`/`TLAST`, USB `ACK`/`ERDY`/`NRDY`). Related:
[[dense-prose-false-signal-loop-reaches-isf]], [[axi-constraint-subject-must-be-declared]].
