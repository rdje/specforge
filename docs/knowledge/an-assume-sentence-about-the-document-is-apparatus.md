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
  - "why is 'the missing input cannot be assumed to take a fixed value' no longer an abstraction (a negated assume is a rule)"
  - "why is 'the bits … assume a valid but otherwise UNSPECIFIED value' no longer an abstraction (assume means take on)"
  - "what is every_assume_word_states_no_abstraction / premise_of_a_figure_a_guideline_or_an_analysis"
  - "why is 'the hub shall ignore the route string and assume all packets are routed directly to the hub' an invariant now"
  - "why is 'The WISHBONE interface logic assumes that a low-skew clock distribution scheme is used' an assumption, not apparatus"
  - "what is states_only_premises_with_a_premise_phrase / names_an_example_before_its_assumptions / PREMISE_SUBJECT_WORDS"
  - "why does 'These rules assume that the TS Delays are zero' stay apparatus (a rule's premise)"
  - "why does a 'For example' after an assumption not make it an example's premise"
  - "what is object_head / OBJECT_ENDS (why is 'a host assumes the initial … sequence number and burst size' a take-on)"
  - "why is 'Connection Manager assumes the configuration of Asymmetric Link failed' an assumption (of stays in the object)"
date: 2026-10-20
status: current
tags: [evidence-ir, statement-class, apparatus, assumptions, classifier, measured, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (StatementClass::Apparatus; abstraction_class; is_non_contract_assumption; ABSTRACTION_WORDS; MODELLING_PHRASES; DOCUMENT_SUBJECTS); crates/specforge/src/ir/semantic.rs (is_invariant_like); crates/specforge/src/commands/validate.rs (the class-name table); docs/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.ii.b.i); docs/tasks/extraction-representation-gaps/environment-assumption-leaves.md (.3.d.iii.e, .3.d.iii.f)
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


**`.3.d.ii.b.iii.b` (`2026-10-19`): an assume word that states no assumption, and three more premises.** A
sentence leaves the abstraction class, keeping the class it would have without the word, when **every** assume word
in it is negated (*cannot be assumed*, *should not assume*), is an active *assume* whose object ends in a value, a
state or a configuration (*take on*), is an active *assume* governed by *shall* in its clause, or is *assumed* after
*value* (`every_assume_word_states_no_abstraction`); one ordinary use keeps it an abstraction, and a modelling phrase
always does. `premise_of_a_figure_a_guideline_or_an_analysis` adds a figure named in the assume clause, a document
noun after *this* (with *guideline*), and an analysis noun in the clause (*procedure*, *curves*); *assumed to be
approximately* and *examples* join the premise phrases. Measured on the 31 chains: 19 statements in 7 documents change
class (13 targets, two example premises, two Wishbone legal sentences, a three-state buffer that *can assume a logic
low state*, an endpoint that *cannot assume a rate*); no other evidence record moves; 15 IntentIR assumptions go, and
three reclassified rules become ordinary invariants and constraints (AXI 1, USB 2). Unlike `.3.d.ii.b.i`'s refused
fall-through, nothing junk is minted, because these sentences are rules or behaviour.

**`.3.d.iii.e` (`2026-10-20`): what the hardware assumes of its environment is an assumption.** *Assume that* and
*assumes that* were premise phrases whatever their subject, so *The WISHBONE interface logic assumes that a low-skew
clock distribution scheme is used on the target device* was apparatus. Since then
(`states_only_premises_with_a_premise_phrase`) a sentence is a premise by those phrases only when **every** use has,
in its own sentence (a table cell counts as one), a document noun, an analysis noun or a `PREMISE_SUBJECT_WORDS` word
before it (*rules*, *limitation*,
*descriptions*, *shown*, *we*, *we'll*, *you*, *reader*), or no subject at all (*Assume that a target or line error
has occurred*). An *example* counts only before an assume word (`names_an_example_before_its_assumptions`): AXI's
*This MEC implementation assumes that Managers and caches do not perform any MECID checks. For example, …* is
illustrated, not premised. *Assumes the* joined the test in `.3.d.iii.f`. Measured on the 31 chains: 5 statements in 4
documents become abstractions (Wishbone's clock-skew and FASM read-data sentences, AXI's MEC, SMBus's timeout, RISC-V
debug's *Debuggers may assume that a hardware platform has no harts with indexes higher than the first nonexistent
one*, which `permission_rules` also reads); no other evidence record moves.

**`.3.d.iii.f` (`2026-10-20`): *assumes the* and an object's head.** *Assumes the* (with *these*, *then*, *there*, as
its old substring did) takes the same subject test, and an active *assume* takes on (`TAKE_ON_NOUNS`, now with
*number(s)* and *size(s)*) when the clause's last word **or** its object's head is one of them: `object_head` is the
last word before the first of `OBJECT_ENDS`, the prepositions and subordinators; *of* is not one, so *assumes the
configuration of Asymmetric Link failed* is a conclusion, not a take-on. Measured on the 31 chains: 5 statements in 3
documents (USB 3.2's host sequence state and a Wishbone patent disclaimer -> source facts; the USB4 connection-manager
guide's two conclusions and a Wishbone example circuit's table row -> assumptions); nothing else moves.

Links: [[assume-sentences-fall-into-six-shapes]], [[a-signals-stated-absent-value-rides-its-presence-record]].
