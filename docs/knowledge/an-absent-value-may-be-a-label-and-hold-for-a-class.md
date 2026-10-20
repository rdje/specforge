---
id: an-absent-value-may-be-a-label-and-hold-for-a-class
title: A stated absent value may be a label of the signal's own encoding, and a value stated for one class of requests is a qualified value, never the signal's own
answers:
  - "what is qualified_absent_values on a SignalPresenceRecord"
  - "what is applies_to on a StatedAbsentValue / what is StatedValueEncoding"
  - "why does AWDOMAIN carry three absent values (System domain, Shareable domain, Non-shareable)"
  - "how is 'CMO transactions are assumed to be Non-shareable' read"
  - "how is a label like 'Non-shareable' resolved to a code (through the table the document's Enum statements for the signal or its collective name cite)"
  - "why does 'Shareable domain' carry two codes 0b01 and 0b10"
  - "how does a bullet under 'If AxDOMAIN signals are not present:' get its absence clause (the list lead-in)"
  - "why is a table row not read as an absent-value sentence"
  - "why does bind_stated_absent_values run after the convergent loop"
  - "what is isf_protocol_signal_qualified_absent_value_"
  - "which metric counts class-scoped absent values (signal_presence_qualified_absent_values)"
date: 2026-10-19
status: current
tags: [evidence-ir, signal-presence, absent-value, encoding, label, carried-evidence, isf-adapter, adr-0006, adr-0016]
evidence: crates/specforge/src/ir/evidence.rs (StatedValueEncoding; AbsentValueEncodings; stated_absent_value; value_clause_class_subject; absence_list_lead_in; agreed_absent_value; labeled_absent_value_tests); crates/specforge/src/ir/adapters.rs (absent_value_residual_packet); crates/specforge/src/commands/validate.rs (signal_presence_qualified_absent_values); docs/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.ii.b.iii.a)
reverify: "cargo test --offline -p specforge-core --lib labeled_absent_value && python3 -c \"import json; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); print([(r['signal_name'], [(v['applies_to'], v['value'], v['encoding']['codes']) for v in r['qualified_absent_values']]) for r in e['signal_presence_records'] if r.get('qualified_absent_values')])\"   # expect AWDOMAIN and ARDOMAIN, three values each: Non-cacheable requests / System domain / 0b11, Cacheable requests / Shareable domain / 0b01 0b10, CMO transactions / Non-shareable / 0b00"
---

AXI never states a default for `AxDOMAIN` as a whole. It states one per class of requests, as labels of the
signal's own encoding: *If AxDOMAIN signals are not present:* / *- Non-cacheable requests are assumed to be in the
System domain.* / *- Cacheable requests are assumed to be in the Shareable domain.*, and *If AxDOMAIN is omitted from
the interface, CMO transactions are assumed to be Non-shareable.* Binding *Non-shareable* as `AWDOMAIN`'s one
`absent_value` would assign it to every request, which the document contradicts. Since
`EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii.a` (`2026-10-19`) a `SignalPresenceRecord` carries such values in
`qualified_absent_values`, each a `StatedAbsentValue` with `applies_to` (the class as written) and, for a label,
`encoding: StatedValueEncoding { table_id, codes }`.

**A label** is the clause's words after *assumed to be*, without a leading *in the*. It binds only when a row of
the table the document's enum for that signal cites carries it as a cell (compared without case or spacing), and
every such row contributes its code as written. The enum is found through the `Enum <name> <member> = <code>.`
statements, which name `table_0111` for `AxDOMAIN`, and it may be named by the signal or by a collective name it is a
member of ([[a-document-stated-collective-name-stands-for-its-members]]). The convergent loop writes those
statements, so `bind_stated_absent_values` now runs after it. *Shareable domain* labels two rows, so it keeps
`0b01` and `0b10`; choosing one would fabricate.

**A class** is the subject of the clause that states the value, after the sentence's last clause break, when the
absence is stated in another clause or in the list's lead-in. The subject must not be the signal: a pronoun, a final
noun for a signal or its value (*the value*, *these signals*), a name, or a subject itself said to be absent (*the
missing signals*) each means the value is the signal's own. **A list**: a bullet that states no absence takes the
absence of the statement introducing its list, when that statement ends with a colon and states one; only bullets
may stand between them, and both statements are cited. Agreement is settled per class. A **table row** lifted as
prose is not read: AXI's change history restates the CMO default as *… if the AxDOMAIN signals are missing, a CMOis
assumed to be Non-shareable*, and the converter's fused *CMOis* would otherwise have become a class (the fusion is
`TEXT-LAYER-IDENTIFIER-SPLIT.5`'s).

**Measured `2026-10-19`.** Evidence dry-run of the 31 chains with the HEAD and the new binary: 30 identical; AXI moves
only in `signal_presence_records`, `AWDOMAIN` (`presence_0033`) and `ARDOMAIN` (`presence_0112`) each taking the three
values. The rebuilt AXI chain moves only that field at SemanticIR and IntentIR, the adapter gains six
`isf_protocol_signal_qualified_absent_value_<presence>_<n>` residuals, and `manager.isf` is byte-identical. `validate`
reports `signal_presence_qualified_absent_values` (6 on AXI).

Links: [[a-signals-stated-absent-value-rides-its-presence-record]], [[assume-sentences-fall-into-six-shapes]].
