---
id: document-intent-category-recognizer
title: Document intent-category recognizer (DOC-INTENT-TAXONOMY.3b) — validate reports a chip-spec PDF's 6-category PURPOSE (wire-protocol / register-or-platform / cpu-isa / physical-link / methodology-guide / unresolved) beside document_class, pure + name-list-free, honest confidence + residual; only clean-wire and self-declared-guide are HIGH confidence
answers:
  - "how does SpecForge determine what a chip-spec PDF is about / its purpose category"
  - "what is document_intent_category / the 6-category purpose recognizer"
  - "where is classify_document_intent_category implemented (crates/specforge/src/ir/completeness.rs)"
  - "what are the 6 purpose categories (wire-protocol, register-or-platform, cpu-isa, physical-link, methodology-guide, unresolved)"
  - "how is the purpose category different from document_class (richer 6-way semantic taxonomy vs coarse 4-way structural proxy; consumes document_class as one input, never replaces it)"
  - "why are register-IP (cat 2) and platform-IP (cat 3) reported as one combined register-or-platform category (counts cannot separate them — DOC-INTENT-TAXONOMY.1)"
  - "why is CPU-ISA / PHY only recognized from front-matter (no distinct structural signature; cat 5 vs cat 6 indistinguishable by structure)"
  - "why does a register count NOT veto a wire protocol (wire-vs-structure weight dominance; AXI wire 401 >= struct 229)"
  - "why do message/flit fields count as wire intent only when there is no register map (CHI/DTI reg=0 vs NVMe/AMD/CCIX reg>0)"
  - "when is the purpose category HIGH vs LOW confidence (only clean wire shape + self-declared guide are HIGH; everything else LOW + explicit residual)"
  - "what validate metrics/findings carry the purpose category (document_intent_category, document_intent_category_confidence, evidence_document_intent_category finding)"
  - "what is the corpus distribution of purpose categories (21 wire / 8 guide high; 28 register-or-platform / 16 unresolved / 5 physical-link low; 0 high-confidence false positives)"
  - "how does SpecForge avoid chip/vendor name lists in document classification (structural typed-surface counts + generic front-matter doc-type vocabulary only; ADR 0006)"
date: 2026-06-22
tags: [doc-intent-taxonomy, document-class, document-intent-category, recognizer, validate, completeness-rs, honest-residual, confidence, adr-0006, front-matter, measured]
evidence: crates/specforge/src/ir/completeness.rs (classify_document_intent_category + DocumentIntentCategory/IntentCategoryConfidence/DocumentIntentClassification + front_matter_declares_isa/_phy + 13 in-file tests); crates/specforge/src/commands/validate.rs (census bound once -> both classifiers; document_intent_category metric + evidence_document_intent_category finding); docs/tasks/DOC-INTENT-TAXONOMY.md (.3a design + .3b acceptance checklist); docs/book/src/quality/validation.md + docs/book/src/document-categories.md (user-facing)
reverify: "cargo test -p specforge --lib -- completeness (66/66, incl. the recognizer tests); then for d in generated/evidence_ir/*/evidence_ir.json; do ./target/debug/specforge validate $d | grep document_intent_category:; done | sort | uniq -c -> 21 wire-protocol (high) / 8 methodology-guide (high) / 28 register-or-platform (low) / 16 unresolved (low) / 5 physical-link (low), 0 high-confidence false positives. Key invariants: a clean wire shape OR a self-declared guide => HIGH confidence with no residual; everything else => LOW + residual; flit fields are a cat-1 cue only when registers==0; register/field count never vetoes a clean wire shape (wire_weight >= struct_weight). Pure fn of DocumentClassCensus + generic front-matter doc-type vocabulary, no chip/vendor/protocol-instance name list (ADR 0006). Related: [[document-intent-category-census]], [[document-intent-isf-completeness]]."
---

`specforge validate <evidence-ir>` recognizes a chip-spec PDF's **purpose category** — what the document
is *about* — via the pure `classify_document_intent_category(census)` in
`crates/specforge/src/ir/completeness.rs` (sibling to `classify_document`), reported beside the structural
`document_class` as a `document_intent_category` / `document_intent_category_confidence` metric and an
`evidence_document_intent_category` Info finding.

The six categories are `wire-protocol` (cat 1), `register-or-platform` (the combined cat 2/3 the
[[document-intent-category-census]] proved counts cannot split), `cpu-isa` (cat 4), `physical-link` (cat 5),
`methodology-guide` (cat 6), and an honest `unresolved`. The recognizer is **additive** — it consumes the
same `DocumentClassCensus` as `classify_document` (extended with `message_field_records` /
`signal_presence_records` / `front_matter_isa` / `front_matter_phy`) and never replaces the structural class.

It is honest by construction: **only** a clean wire-behavioural shape and a self-declared guide are reported
at **high** confidence (zero high-confidence mislabels measured over the 78-doc corpus); cat 2↔3, cat 4, and
cat 5↔6 are reported at **low** confidence with an explicit `residual`, never a forced guess. The measured
discriminators (a refinement of the `.3a` flit clause its own per-document data falsified): a register/field
count never vetoes a clean wire shape (a wire-weight vs register/structure-weight dominance test — AXI: wire
401 ≥ struct 229), and message/packet fields count as wire (flit) intent only when the document declares no
register map (CHI/DTI `registers=0` vs NVMe/AMD-IOMMU/CCIX `registers>0`). Every cue is a structural
typed-surface count or generic front-matter doc-type vocabulary — no chip/vendor/protocol-instance name list
(ADR 0006). See [[document-intent-category-census]] (the distribution + the blind spots this encodes) and
[[document-intent-isf-completeness]] (the per-category ISF-lowering scorecard).
