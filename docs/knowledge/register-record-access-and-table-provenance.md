---
id: register-record-access-and-table-provenance
title: Register records preserve register-level access and direct structured-table provenance
answers:
  - "where is register level access stored in RegisterRecord"
  - "how does register access differ from register field access"
  - "how do canonical register records retain source table provenance"
  - "why did Arm Debug register access disappear before EvidenceIR"
  - "do SemanticIR and IntentIR preserve register access"
  - "are older register records compatible with access and table provenance fields"
date: 2026-08-12
status: current
tags: [registers, evidence-ir, semantic-ir, intent-ir, access, provenance]
evidence: crates/specforge/src/ir/source.rs (RegisterRecord); crates/specforge/src/ir/evidence.rs (synthesize_register_records, synthesize_register_field_tables, merge_register_fragments); docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6b.ii.a)
reverify: "cargo test -p specforge --lib register_map_preserves_register_access_and_table_provenance && cargo test -p specforge --lib register_fragment_consolidation_4c"
---

`RegisterRecord.access_type` stores optional register-level access exactly as the source table writes it. It is
separate from `RegisterFieldRecord.access_type`: a register map row does not become a synthetic field merely to
carry `RO`, `RW`, `WO b`, or another source notation. Missing register access remains `None`.

`RegisterRecord.supporting_table_ids` stores direct structured-table provenance separately from prose
`supporting_statement_ids`. Register-map rows retain their table, field-table records retain theirs, bit-layout
chains retain every member, and safe same-register fragment merges union/deduplicate the ids. Both new fields are
Serde-defaulted and omitted while absent, so old artifacts load and unrelated empty shapes stay stable.

The causal defect was at SourceIR → EvidenceIR: the generic register-map extractor parsed Arm Debug table
`table_0044`'s Access cells, but the old register carrier had no register-level field and discarded the values;
later stages faithfully cloned the loss. A bounded implementation probe verifies all 12 exact access strings and
the table id at EvidenceIR, SemanticIR, and IntentIR. `SPEC-TO-INTENT-ALIGNMENT.6b.ii.b` owns the clean-revision
whole-population replay and is the authority for any changed product-quality counts.
