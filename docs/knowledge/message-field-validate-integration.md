---
id: message-field-validate-integration
title: validate reports the message-field surfaces (5 metrics + inventory finding); class census and completeness gauge deliberately unchanged
answers:
  - "does validate report message_field_records"
  - "what message field metrics does validate emit"
  - "why are message fields not part of the document_class decision"
  - "why is there no fields-without-positions completeness gap"
  - "which persisted evidence docs carry message_field_records"
  - "when should message fields join the document class census"
date: 2026-06-11
tags: [validation, message-fields, document-class, completeness, reporting]
evidence: crates/specforge/src/commands/validate.rs (validate_evidence_ir — message_field metrics, evidence_message_field_inventory finding); docs/tasks/PDF-VARIANT-DIGESTION.md (.11)
reverify: cargo test -p specforge --lib validate_evidence_ir_reports_message_field_inventory ; plus a temp-root NVMe evidence rebuild + validate (expect 216/113 and the [info:message_fields] finding)
---

`PDF-VARIANT-DIGESTION.11` (2026-06-11). `validate <evidence-ir>` reports the typed
message-field surfaces: metrics `message_field_records`, `message_field_containers`,
`message_fields_with_bit_range`, `message_fields_with_byte_offset`,
`message_field_constraints`, plus the `evidence_message_field_inventory` Info finding
emitted ONLY when the surface is non-empty (absence is not an event). Live: a temp-root
NVMe rebuild reports 216 fields / 113 containers / 216 with bit positions.

TWO MEASURED NO-DECISIONS (the durable knowledge): (1) message fields do NOT join the
`document_class` census — probe over all 78 persisted evidence docs found ZERO carrying
the surface (the field-bearing docs were never rebuilt: canonical NVMe keeps its standing
Pattern gauge, CHI/AMD bundles host-local), and on the two real dry-run data points (AMD
`interface` via 64 connectivity edges, NVMe `register` via 42 regs) NEITHER reclassifies —
n=2 is overfitting territory ([[feedback_genericity_guardrail]]; the `.5a`
`conditional_rules` exclusion is the precedent). REVISIT TRIGGER: when the corpus
re-ingest sweep rebuilds field-bearing docs at scale, re-measure per-doc. (2) NO
completeness-gauge dimension — a width-only field table (CHI-class) states no positions,
so "fields without bit positions" would mislabel the document's own honest absence as an
extraction gap (fabricated expectation). Related:
[[bit-position-structure-field-extraction]],
[[offset-suffixed-dword-relative-bit-cells]].
