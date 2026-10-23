---
id: an-encoding-members-enabling-property
title: An encoding table's enable or property column gates each member by a property; it becomes one enabling condition per enum member or combination row
answers:
  - "where does an enum member's enabling property go (AWCMO CleanSharedPersist needs Persist_CMO)"
  - "what is EXTRACTION-REPRESENTATION-GAPS.61 / .61.a / .61.b / .61.c"
  - "how are AXI's Enable property / Enable columns (A9.4 AWSNOOP, AWCMO, A7.3, A7.4, A4.7) read"
  - "what is SymbolEnumMemberRecord.enabled_by / CombinationRow.enabled_by / EnablingCondition / enabling_column"
  - "which AXI enums have property-gated members (AWSNOOP, AWCMO, AxPAS)"
  - "why is a member's enabling property not joined to the property's record yet (boolean properties, KG-ISF .2a.vii.c.vi)"
  - "does a gated enum member reach the .isf"
  - "what is isf_protocol_enum_enabled_members"
date: 2026-10-23
status: current
tags: [evidence-ir, semantic-ir, encodings, enums, value-combinations, properties, extraction-representation-gaps, measured]
evidence: docs/tasks/extraction-representation-gaps/table-reading-leaves.md (.61, .61.a); crates/specforge/src/ir/evidence.rs (enabling_condition_column); generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json; crates/specforge/src/ir/evidence.rs (SignalPresenceRecord.presence_condition, the precedent)
reverify: "In AXI's SourceIR, list the tables whose first header row has a cell `Enable`, `Enable property` or `Property` — expect table_0061, table_0102-0106, table_0134 and table_0137 (the eight fragments), plus table_0277-0280 (B2.4, a property-constraint summary that mints no encoding)"
---

AXI gates encoding members by interface properties in a table column. The untitled `AWCMO` table (`AWCMO | Label |
Enable property | Meaning`) gives `CleanSharedPersist` the property `Persist_CMO` and `CleanInvalidPoPA`
`RME_Support`. *A7.3* and *A7.4* give each opcode and domain an `Enable` cell, such as `Shareable_Transactions and
Cache_Stash_Transactions` or `WriteNoSnoopFull_Transaction or Shareable_Cache_Support`.

**Measured `2026-10-23` (31 chains).** Of the 48 tables that mint an enum symbol or a value combination, five AXI
tables (eight fragments) have such a column:
- three enums: *A9.4* `AWSNOOP` (3 of 3 members gated), `AWCMO` (4 of 6) and `AxPAS` (4 of 6, from *A4.7*, which
  `.61`'s census counted only as a combination; corrected by `.61.a`), whose members carried only a name and a value;
- three combination tables, *A7.3* (21 of 25 rows), *A7.4* (12 of 19) and *A4.7* (4 of 6), whose cells
  `value_combinations` already kept as written among `meanings`.

Each cell is `-`, one property, or properties joined by one `and` or `or`. The cells name 22 properties, each with its
own *`<Name>` property* table and every one boolean. SemanticIR mints `param` symbols only for integer properties, so
none of the 22 is a record.

**Decided (`.61`).**
- **The column:** in an enum or combination table, a column headed with *enable*, *enabled* or *property* (document
  grammar, like the presence matrix's *property* column) whose every cell has that shape.
- **The carrier (read since `.61.a`):** an `EnablingCondition`, the cell as written with its names and connective
  (`all` or `any`). The enum readers state `Enum AWCMO CLEANSHAREDPERSIST is enabled by Persist_CMO.`, citing the
  table, as they state `also takes`; SemanticIR reads only a cited one into `SymbolEnumMemberRecord.enabled_by`, which
  the member and its symbol cite. A combination row carries `CombinationRow.enabled_by`, and the record names its
  `enabling_column`. All three reach IntentIR unchanged; `manager.isf` did not move.
- **Not lowered:** the `.isf` enum keeps every member, and since `.61.b` the adapter states one
  `isf_protocol_enum_enabled_members_<name>` residual per gated enum, naming each member and its condition (AXI: 3,
  residuals 114 -> 117, `manager.isf` unchanged, `2026-10-23`).
- **The join:** each name joins its property's record once `KG-ISF-COMPLETENESS.2a.vii.c.vi` reads boolean
  properties (`.61.c`), never by spelling alone before then.

**Neighbours, owned elsewhere:**
- a signal's presence property (`presence_condition`, joined by `.2a.vii.c.vi`);
- a parity check's enable (`check_enable`);
- CHI C2C's register property bits;
- ADIv6 *C2-3*'s `Support required?`, a member's conformance requirement (`.70`).
