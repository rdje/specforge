---
id: document-class-is-determined-by-evidenceir-but-stored-elsewhere
title: document_class is a pure function of five EvidenceIR structural counts, yet it is computed and stored only by validate — and the SourceIR front-matter it seems to need reaches the under-extraction flag, not the class
answers:
  - "what determines a document's document_class"
  - "does document_class depend on the SourceIR front-matter"
  - "where is document_class computed and where is it stored"
  - "why does an EvidenceIR artifact have no document_class"
  - "what does under_extracted_spec mean and what produces it"
  - "what breaks if I clean a document's source_ir"
  - "how many corpus documents are flagged as under-extracted specs"
  - "what is the document_class distribution across the corpus"
  - "where does declared_type come from in validate"
  - "is a census that returns zero a finding"
date: 2026-09-20
status: current
tags: [extraction, validation, document-class, corpus, method, provenance]
evidence: crates/specforge/src/ir/completeness.rs (classify_document); crates/specforge/src/commands/validate.rs (DocumentClassCensus site); docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md (.5)
reverify: "for each generated/evidence_ir/*/validation_report.json read metrics[] by name — expect 78 of 78 to carry document_class, distributed guide 28 / protocol 20 / register 16 / interface 14; then count findings[] whose finding_id is evidence_document_class and whose summary contains UNDER-EXTRACTED — expect 16, all of them guide-class. Read classify_document to confirm the class expression names only registers, declared_signals + actor_signal_relations, signal_constraints, fsm_states and serial_frame_fields"
---

## The class is decided by the EvidenceIR alone

`classify_document` picks the class from **five structural counts and nothing else**:

| class | condition |
| --- | --- |
| `Register` | `registers >= floor` and `registers >=` both the interface surface and `signal_constraints` |
| `Protocol` | `signal_constraints >= floor`, or any `fsm_states` / `serial_frame_fields` |
| `Interface` | `declared_signals + actor_signal_relations >= floor` |
| `Guide` | otherwise |

Every input is an `ir.<field>` read at the census site in `validate.rs`. **So the class is fully
determined the moment the EvidenceIR is written** — and it is computed and persisted only by
`validate`, a post-build command, inside that command's report (`metrics[]` entry `document_class`,
plus an `evidence_document_class` finding). An artifact built by `evidence` and never validated has no
recorded class although its class is already decided. `SIGNAL-CATALOG-CAPTURE-GAP.7` owns moving it.

Measured `2026-09-20` over all 78 persisted artifacts: `guide` 28, `protocol` 20, `register` 16,
`interface` 14 — 78 of 78 classified.

## The SourceIR dependency is real, and it is not on the class

The census site reads `declared_type` from the **sibling SourceIR**'s front-matter (title plus the
first twelve section headings) and its comment warns the value is `Unknown` "when the SourceIR is
unavailable". That reads like a class hazard. **It is not.** The class expression above never mentions
`declared_type`. What it reaches is:

- `under_extracted_spec = class == Guide && declared_type == Specification`, and
- the human-readable rationale.

`message_field_records`, `signal_presence_records`, `front_matter_isa` and `front_matter_phy` are
likewise recognizer-only and explicitly ignored by `classify_document`.

## What that actually costs: a silent recall loss on 16 documents

`under_extracted_spec` is the sentence *"likely an UNDER-EXTRACTED spec (image/table-heavy), not a
true guide → route to VLM rescan, do not dismiss"*. **16 of the 28 guide-class artifacts carry it**
(RISC-V AIA, Bosch CAN, CoreSight BSA, two AMBA CHI C2C revisions, JESD235 HBM, seven OpenCAPI
documents, SMBus 3.3.1, I2S, USB4 inter-domain). `clean.rs` offers a supported per-document
`source_ir` scope, so cleaning a SourceIR and re-validating turns those 16 into plain guides. Nothing
fails; the document simply stops asking to be looked at again. **The class survives that cleanup; the
reason to re-extract does not.**

## A zero from an unvalidated selector is not a measurement

Two censuses run while establishing the above returned **0**, and both were wrong:

- *"no report carries `document_class`"* — it is a `metrics[]` entry, not a top-level key.
- *"no guide-class document is flagged UNDER-EXTRACTED"* — findings carry `finding_id` and `summary`,
  not `code` and `message`.

Either zero, published, would have been a confident false finding with a plausible story attached.
**Before a 0 counts, show the selector matching something** — print one real record and read its keys.

Links: [[direction-suffix-is-one-documents-convention]].
