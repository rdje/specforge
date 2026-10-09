---
id: an-assume-sentence-about-the-document-is-apparatus
title: An assume-sentence whose subject is the document, a diagram key or an example is classified apparatus, a statement class no reader consumes
answers:
  - "what is StatementClass::Apparatus / the apparatus statement class"
  - "why is 'This guide assumes that you are familiar with' no longer an IntentIR assumption"
  - "why not reclassify a convention as a source fact (a hint reader minted CLK_I handshake_valid_like from an example circuit)"
  - "why not let a convention fall through the classifier (a diagram key minted a conditional rule)"
  - "what is abstraction_class / is_non_contract_assumption"
  - "does 'it can be assumed that no write occurred' stay an abstraction (yes)"
  - "is 'treated as' still an abstraction after .3.d.ii.b.i (yes, whatever the sentence says)"
  - "why does Assumer no longer classify as an abstraction (whole-word match)"
  - "which readers exclude the apparatus class (is_invariant_like, and every class-filtered reader by construction)"
date: 2026-10-03
status: current
tags: [evidence-ir, statement-class, apparatus, assumptions, classifier, measured, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (StatementClass::Apparatus; abstraction_class; is_non_contract_assumption; ABSTRACTION_WORDS; MODELLING_PHRASES; DOCUMENT_SUBJECTS); crates/specforge/src/ir/semantic.rs (is_invariant_like); crates/specforge/src/commands/validate.rs (the class-name table); docs/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.ii.b.i)
reverify: "cargo test --offline -p specforge-core --lib non_contract_assumption && python3 -c \"import json; e=json.load(open('generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json')); print(sum(1 for s in e['extracted_statements'] if s['class']=='apparatus'))\"   # expect 3 (two diagram keys and a parity-budget premise)"
---

`classify_statement` tried its abstraction arm first: any sentence containing *assume*, *assuming*, *modeled as*,
*abstracted* or *treated as* became `ExplicitAbstraction`, then a SemanticIR abstraction, then an IntentIR
assumption ([[assume-sentences-fall-into-six-shapes]]). Since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.i`
(`2026-10-03`) that arm is `abstraction_class`: the words are matched as whole words (`Assumer` is none),
*abstracted* only as a verb (*is abstracted as*, not *an abstracted view*), and a sentence that uses one is **an
abstraction** when it says *treated as* or states something about the hardware, and **apparatus** when it is one of
three measured shapes (`is_non_contract_assumption`): its subject is the document, a figure or the reader (*This
guide assumes that you …*, *In this example, the following conditions are assumed:*, *we assume*), it is a
timing-diagram key (*shaded*, *timing diagram*, *waveform*), or it states the premise of an example or an analysis
(*example*, *assuming*, *assumes that*, *assumed as*, *shown assume*). *It can be assumed that no write occurred* is
a hardware consequence and stays an abstraction; *assumed that* is deliberately not a premise marker.

**Why a dedicated class.** Both cheaper remedies were tried and measured on the 31 chains. Letting the sentence fall
through the classifier sent 33 of 83 into the rule, constraint and timing classes, and the conditional-rule reader
minted rules from six of them, a diagram key among them (*Variations, when they occur, have clear labels. You must
not assume …*). Filing it as `SourceFact` kept every other record byte-identical except Wishbone's, where the prose
hint reader (which reads source facts) minted `CLK_I` → `handshake_valid_like` from an example circuit's *assumes
that output data is valid at the rising [CLK_I] edge*. `StatementClass::Apparatus` is consumed by no reader:
class-filtered readers never select it, and `is_invariant_like` excludes it beside `ExplicitAbstraction`.

**Measured `2026-10-03`.** 83 sentences in 22 of the 31 chains become apparatus, 6 of them sentences that only
mention the word as a fragment or an adjective (read as ordinary prose, two of those minted invariants, so they are
apparatus too); every other
evidence record is byte-identical; their IntentIR assumptions are gone (78 fewer across the 22 chains, no other IntentIR section moved, adapters and ISF texts byte-identical). The class is outside the
EvidenceIR seal closure, so no chain re-seals; the changed chains are rebuilt stage by stage.

Links: [[assume-sentences-fall-into-six-shapes]], [[a-signals-stated-absent-value-rides-its-presence-record]].
