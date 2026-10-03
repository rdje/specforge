---
id: a-signals-stated-absent-value-rides-its-presence-record
title: A signal's stated absent value is bound to its presence record and carried, literal and cited, to IntentIR
answers:
  - "where does SpecForge carry the value a signal takes when it is absent or not driven"
  - "what is absent_value on a SignalPresenceRecord / what is StatedAbsentValue"
  - "how is 'not present and assumed to be 0b000' read / what is bind_stated_absent_values"
  - "why does AXI's BRESP presence record carry 0b000"
  - "why is 'Any AxSNOOP bits not driven by an interface are assumed to be LOW' not bound"
  - "why was 'the missing signals are assumed to be LOW' (signals covered by a check signal) not bound by .3.d.ii.a alone"
  - "why is 'assumed to be 0b0 for a DVMComplete request' not read as an absent value"
  - "do signal_presence_records cross SemanticIR and IntentIR"
  - "what does the isf adapter do with a stated absent value / what is isf_protocol_signal_absent_value_"
  - "why did adding signal_presence_records to SemanticIR and IntentIR re-seal every chain"
  - "which metric counts stated absent values (signal_presence_absent_values)"
  - "is an absent value interpreted against the signal's polarity"
date: 2026-10-03
status: current
tags: [evidence-ir, signal-presence, absent-value, tie-off, carried-evidence, isf-adapter, adr-0006, adr-0016, fsmgen]
evidence: crates/specforge/src/ir/evidence.rs (StatedAbsentValue; bind_stated_absent_values; stated_absent_value_literal; signal_presence_surface); crates/specforge/src/ir/semantic.rs and intent.rs (signal_presence_records); crates/specforge/src/ir/adapters.rs (absent_value_residual_packet); docs/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.ii.a); docs/upstream/fsmgen/sf-isf-optional-signal/README.md (R3)
reverify: "cargo test --offline -p specforge-core --lib stated_absent_value && python3 -c \"import json; e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json')); print([(r['signal_name'], r['absent_value']) for r in e['signal_presence_records'] if r.get('absent_value')])\"   # expect BRESP and RRESP, value 0b000, one statement each"
---

A presence matrix says when a signal exists (`BRESP_WIDTH > 0`); the prose says what it is when it does not:
*BRESP is an optional signal. If the BRESP_WIDTH property is 0, it is not present and assumed to be 0b000 (OKAY).*
Since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a` (`2026-10-03`) that value lives on the signal's
`SignalPresenceRecord` as `absent_value: Option<StatedAbsentValue { value, supporting_statement_ids }>`, bound by
`bind_stated_absent_values` after the matrix capture inside the one registered `signal_presence` surface, so no
new extractor entry changes any other document's manifest. The value is the literal as written (`0b000`, `LOW`);
it is never interpreted against the signal's polarity or widened, because the sentence states a literal and the
consumer that ties the signal off decides what it means on the wire. `signal_presence_records` then cross
SemanticIR (`semantic.carried_evidence`) and IntentIR (`intent.carried_semantics`) unchanged.

**What is read.** In one sentence, an absence predicate (`not present`, `not driven`, `omitted`, `missing`, `not
included`, `absent`) precedes `assumed to be`, and what follows is a literal — `0b…`, `0x…`, a decimal, a word of
two or more capitals, or `asserted`/`deasserted` — closed by the end of the sentence, a clause break, or one
parenthetical label (`0b000 (OKAY)`). The signal is the one presence-record name the whole statement mentions
(`identifier_words` against the document's own presence names), which lets the second sentence's *it* resolve to
`BRESP` while `BRESP_WIDTH`, a property, is no candidate. Several statements stating the same value are all cited;
disagreeing statements bind nothing.

**What is not read, each a measured residual with an owner.** A value the sentence goes on to qualify (AXI's
*ARMMUVALID is assumed to be 0b0 for a DVMComplete request*) — the record cannot carry the qualification. A class
of signals (*If some of the signals covered by a check signal are not present …, the missing signals are assumed
to be LOW*, APB5 and AHB5) — bound since `.3.d.ii.a.i` the same day through the check-coverage table
([[a-check-coverage-table-binds-the-class-of-covered-signals]]). A generic name no matrix row carries (*Any AxSNOOP bits not
driven … are assumed to be LOW*; the rows are `AWSNOOP` and `ARSNOOP`, and the alias map is empty for AXI) —
the document's prefix convention has no carrier (`.3.d.ii.a.ii`).

**Measured `2026-10-03`.** Evidence dry-run of the 31 chains with the HEAD and the new binary: 30 identical; AXI
moves only in `signal_presence_records`, `BRESP` (`presence_0082`, `statement_1089`) and `RRESP` (`presence_0143`,
`statement_1118`) each taking `0b000`. The unread `assumed_by_word` family falls 111 -> 109. The `.isf` adapter
keeps one `isf_protocol_signal_absent_value_<presence_id>` residual per stated value: `.isf` has no clause for
the value an absent input reads (`(absent-value V)`, `sf-isf-optional-signal` R3, acknowledged by FSMGen).

**Cost of the carrier.** `signal_presence_records` joins `SEMANTIC_RULE_FIELDS` and `INTENT_RULE_FIELDS`, which
are inside those stages' seal closures ([[stage-seal-closure-membership]]), so every chain's SemanticIR and
IntentIR re-seal; the three presence-bearing chains (AXI, APB5, AHB5) are rebuilt by hand stage by stage and the
other 28 by `scripts/rebuild_stage_cascade.sh --write`, content-identical. `validate` reports
`signal_presence_absent_values` on EvidenceIR and a `signal_presence_records` count on the two carried stages.

Links: [[signal-presence-matrix-capture]], [[a-timeouts-next-state-is-a-stated-transition]].
