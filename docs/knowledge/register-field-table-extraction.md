---
id: register-field-table-extraction
title: Register-FIELD tables (Field|…|Access|Reset) the classifier left "unknown" are recovered into RegisterRecords
answers:
  - "how does SpecForge extract register fields from tables"
  - "why were RISC-V/TRM register tables unextracted (unknown table_kind)"
  - "what is synthesize_register_field_tables"
  - "how flexible is the register model / what register-table shapes are handled"
date: 2026-06-07
tags: [registers, tables, evidence, pdf-variant-digestion, lever-a]
evidence: crates/specforge/src/ir/evidence.rs (synthesize_register_field_tables, is_register_field_header, register_name_from_caption); docs/tasks/PDF-VARIANT-DIGESTION.md
reverify: ./target/debug/specforge evidence generated/source_ir/1_0_risc_v_debug_specification/source_ir.json && python3 -c "import json;e=json.load(open('generated/evidence_ir/1_0_risc_v_debug_specification/evidence_ir.json'));print(len(e['register_records']),'regs',sum(len(r['fields']) for r in e['register_records']),'fields')"
---

PDF-VARIANT-DIGESTION.2 (Lever A, deterministic strategy). Many register tables in TRMs / architecture /
register specs (RISC-V, CoreSight, OpenCAPI) were classified `unknown` and dropped — usually because Docling
did not mark the column-title row as a header, so `Field|Description|Access|Reset` landed in `body_rows[0]`.

`synthesize_register_field_tables` (evidence stage, Rust) recovers them: it resolves the header (a marked
header row, else `body_rows[0]`), recognizes a register-FIELD table via `is_register_field_header` (a
field/bits/name column AND an access/reset column — universal register vocabulary, ADR 0006), and emits one
`RegisterRecord` per table with a `RegisterFieldRecord` per row. **Additive** — it never touches
signal/constraint/relation extraction and skips tables already handled by `synthesize_register_records`, so
APB/AHB/AXI/SWD stay at 100% (verified). Result on the RISC-V Debug spec: **60 registers / 179 fields** from
previously-`unknown` tables.

Flexibility (designed from a corpus survey — [[project_flexible_register_model]]): handles the surveyed
shapes `Field|Description|Access|Reset`, `Bits|Type|Reset|Description` (no name col → bit-range identifies the
field; "Type" is the access col), `Offset|Bits|Field name|…|Attributes`; access/reset kept as FREE strings
(RO/RW/WARL/W1C/-/Reserved/…); bit ranges parse zero-padded forms ("02:00" → 2:0). Header-keyword echoes
(legend rows) are dropped. Multi-strategy: the VLM (Qwen2.5VL on rendered table images) is the planned second
strategy, best-wins-per-PDF ([[feedback_multi_strategy_best_wins]]).

**Known follow-ups:** register NAME is synthetic (`register_table_NNNN`) when the table has no caption — the
real name is often in a preceding heading (not yet wired); field VALUE ENUMERATIONS and register width/size
are not yet modeled ([[project_flexible_register_model]]).
