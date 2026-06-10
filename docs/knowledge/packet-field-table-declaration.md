---
id: packet-field-table-declaration
title: Packet/flit protocols declare message FIELDS in field-titled tables — the header vocabulary types the rows
answers:
  - "how do packet/flit protocols (CHI-class) declare message fields vs signals"
  - "what structural cue separates a message-field table from a register-field table"
  - "where do DBID / TxnID / ReturnNID style names come from in CHI"
  - "why were CHI fields mis-typed as signals (the .gauge spurious-subject class)"
  - "what is the EXTRACTION-QUALITY-GAUGE.FIELD design"
  - "which corpus docs declare fields with a Field-titled column"
date: 2026-06-10
tags: [extraction-quality, ontology, fields, packet-protocols, tables, adr-0006]
evidence: docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.FIELD.1); generated/source_ir/*/source_ir.json structured_tables (probe)
reverify: python3 -c "import json; s=json.load(open('generated/source_ir/ihi0050_g_2024_03_amba_chi_architecture_specification/source_ir.json')); ts=[t for t in s['structured_tables'] if any((c.get('text','')).strip().lower().startswith('field') for r in (t.get('header_rows') or [])[:1] for c in r)]; print(len(ts), [t['caption_text'] for t in ts[:4]])"   # → 36, Request channel fields...
---

Probed over all 49 persisted SourceIRs with structured tables (`2026-06-10`): packet/flit
protocols declare **message fields** in tables whose own name column is field-titled
(`Field` / `Field name`) — CHI has 36 such tables ("Request channel fields", "Response packet
fields", "Snoop request fields", "Data packet fields"; 79 distinct names including the exact
class the `.gauge` caught mis-typed as signals: `DBID`, `TxnID`, `ReturnNID`, `Addr`,
`Opcode`); CHI-C2C adds widths (`Field name|Width (bits)|Value`); CXS ("Packet control
fields"), DTI, USB carry the shape too. **Signals** stay in `Signal`-titled tables (CHI:
"<channel> interface signals" — `REQFLITV`, `REQLCRDV`...). The document's own header
vocabulary types its rows — a universal structural cue, no name lists (ADR 0006).

Register docs share the field-titled column (RISC-V Debug `Field|Description|Access|Reset`,
Intel VT-d `Bits|Access|Default|Field|Description`): the **discriminator is register-access
vocabulary columns (`Access`/`Reset`/`Default`) and/or in-register bit-position columns** —
those are REGISTER-field tables (register surface). CCIX (`Bit Location|Field Description`)
and OpenCAPI (`Operand mnemonic|Field width|Description`) need different strategies later.

As of `2026-06-10` the field tables are inert (`table_kind: unknown`) — fields have no typed
home, which is why field obligations could only surface as wrong signal constraints; and
`entity_prompt` (`ir/entity_typing.rs`) defines `signal = a wire/pin/field carrying a value`,
conflating the ontology at the judgment point. Fix plan: `EXTRACTION-QUALITY-GAUGE.FIELD.2`
(capture surface `message_field_records`) → `.FIELD.3` (`EntityType::Field` grounding) →
`.FIELD.4` (field-scoped constraints + CHI re-measure). Related:
[[llm-primary-constraint-dedup]], [[conformal-tier-agreement-degenerate]].
