---
id: document-class-from-structure
title: Document class (protocol/register/interface/guide) inferred from structural intent surfaces, not the doc name
answers:
  - "how does SpecForge tell a guide from a real spec / report low-yield docs honestly"
  - "what document class is a chip-spec PDF (protocol / register / interface / guide)"
  - "how does validate classify a document / what is document_class"
  - "why are conditional_rules excluded from the document-class decision"
  - "how is a guide reported so it is not a silent 0-yield extraction miss"
  - "what structural surfaces discriminate document class"
date: 2026-06-08
tags: [document-class, validate, completeness, pdf-variant-digestion, agnostic, adr-0006]
evidence: crates/specforge/src/ir/completeness.rs (classify_document); docs/tasks/PDF-VARIANT-DIGESTION.md (.5a)
reverify: "./target/debug/specforge validate generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification/evidence_ir.json 2>/dev/null | grep -A2 'Document Class'   # → protocol"
---

`PDF-VARIANT-DIGESTION.5a`: `specforge validate <evidence-ir>` reports a **document class** so a
low-structured-design-intent document (a programming guide / overview / image-heavy datasheet) is reported
HONESTLY as a `guide`, not as a silent 0-yield extraction failure. Four classes, inferred ONLY from which typed
intent surfaces the extraction produced — never from the document title/vendor (ADR 0006, agnostic):

- **protocol** — `signal_constraints ≥ 3`, OR an FSM / serial frame exists. (APB/AHB/AXI/AXI-Stream/I²C/CHI/SWD)
- **register** — `registers ≥ 2` AND registers outnumber BOTH the connectivity surface (relations + declared
  signals) AND the behavioral surface (signal constraints). (RISC-V Debug 60≥30; NVMe 46≥26)
- **interface** — a signal inventory + actor-signal connectivity at the floor, no behavior, not
  register-dominated. (Avalon / Wishbone / TileLink / OpenCAPI-PHY)
- **guide** — the HONEST floor: none of the reliable surfaces present. Counts shown, conditional-rule count
  flagged "not class-determining".

Pure function `crate::ir::completeness::classify_document(DocumentClassCensus) -> DocumentClassification`;
surfaced as a `document_class` metric + an `evidence_document_class` Info finding (always recorded).

**KEY real-data finding (why the threshold model was corrected by measurement, not guessed):** the
`conditional_rules` extractor (*"if X then Y"*) is OVER-PRODUCED — it fires **12× on a GIC interrupt-controller
overview *guide***, 7× on the SMMU software guide, 78× on the RISC-V Debug *register* spec, and 250× on NVMe.
So it does NOT discriminate class and is **EXCLUDED from the decision** (kept in the census, shown in a guide's
rationale only). The decision routes only on low-noise surfaces. Decision order Register → Protocol → Interface
→ Guide.

Live distribution over all 74 persisted EvidenceIR docs (`2026-06-08`): **protocol 25 / register 11 /
interface 22 / guide 16.** Agnostic by construction (only generic numeric floors 2/3/3 — `DOC_CLASS_*_MIN`);
APB/AHB/AXI/SWD wire-based extraction unchanged; kg-bench 151/151. Owner-suggested follow-up `.5c`: read the
document's own front-matter (title / ToC / preface — usually stated in the early pages of chapter 1) to
corroborate the class and separate a TRUE guide from a spec we under-extracted (the VLM frontier).
