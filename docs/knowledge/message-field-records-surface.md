---
id: message-field-records-surface
title: message_field_records — the typed home for packet/flit message fields (CHI 106, C2C ≤189, CCIX ~50)
answers:
  - "where do packet/flit message fields (TxnID / DBID / Opcode) live in EvidenceIR"
  - "what is MessageFieldRecord / message_field_surface / message_fields manifest entry"
  - "how are message-field tables distinguished from register-field tables"
  - "why does a register doc captioned 'message fields' yield zero message fields"
  - "how is a message field's width kept honest (per-variant widths stay None)"
  - "how do continuation tables (Table B2.2 Continued) merge into one container"
  - "how to re-measure the message-field corpus yield"
date: 2026-06-10
tags: [extraction-quality, fields, packet-protocols, evidence-ir, extractor-framework, adr-0006]
evidence: crates/specforge/src/ir/evidence.rs (MessageFieldRecord, message_field_surface, extract_container_message_fields); crates/specforge/test_data/kg_quality/message_field_table_gold + message_field_register_table_negative; docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.FIELD.2)
reverify: cargo test -p specforge --lib message_field -- --include-ignored --nocapture 2>&1 | tail -12   # unit gates + live corpus sweep (CHI 106/4 containers)
---

`EvidenceIr.message_field_records` (`EXTRACTION-QUALITY-GAUGE.FIELD.2`, `2026-06-10`) captures
packet/flit protocols' declared MESSAGE FIELDS — `TxnID`, `DBID`, `Opcode`, the exact class the
gauge caught mis-typed as signals on CHI. Extractor `message_fields.container_field_table` via
`run_surface` (key = container+name; manifest entry `message_fields`). Structural gates: exact
`Field`/`Field name` name column; the one-place register discriminator `is_register_field_header`
(Access/Reset/Default/Type columns → the REGISTER surface owns it); caption must anchor
"field(s)" to a container noun at distance ≤2 (channel/packet/message/flit/header/frame/request/
response — grammar vocabulary, no chip names); continuations inherit the container via the
`Table <ref>` token and merge provenance; width only from a single unqualified width column
(plain count or `[hi:lo]`), per-variant widths stay honest `None`; restriction/status tables
declare nothing.

Live corpus yield (real extractor, `#[ignore]`d sweep test): CHI **106 fields / 4 containers**,
CHI-C2C 149/143/189 (widths 89/93/164), CCIX 1.x 47/50/51, CXS 1 — zero on every register/wire
doc (12-doc stash-diff byte-identical except the additive manifest entry). Residual shapes for
later strategies: CCIX 2.0 `Bit Location|Field Description`, OpenCAPI `Operand mnemonic`, USB
descriptors. Consumed by `.FIELD.3` (`2026-06-10`, same day): `EntityType::Field` +
`EntityEvidence.declared_in_field_table` ground a declared field deterministically (no LLM call;
signal-table declaration outranks) and `is_valid_signal_subject(Field)` is false — the
`DBID`/`TxnID` class can no longer be a constraint subject ([[packet-field-table-declaration]]). kg-bench keys: `message_field_count` / `message_fields_include`
(`bit_width`, `bit_width_absent`) / `message_field_names_exclude`. `.FIELD.4` (same day) routes
field-subject OBLIGATIONS to the parallel `message_field_constraints` surface instead of
dropping them ([[message-field-constraints-surface]]).
