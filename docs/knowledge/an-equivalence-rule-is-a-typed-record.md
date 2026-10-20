---
id: an-equivalence-rule-is-a-typed-record
title: A sentence that treats one thing as another is an equivalence_rules record — subject, equivalent, condition and modality as written — carried to IntentIR
answers:
  - "what is equivalence_rules / EquivalenceRuleRecord / EquivalenceModality"
  - "how is 'LPIs are always treated as Non-secure Group 1 interrupts' read"
  - "what is stated_equivalences / equivalence_rule_surface"
  - "why is 'otherwise they are treated as unsupported requests' not an equivalence record (a contrasted clause is refused)"
  - "why is 'if the beginning part of Warm Reset is treated as U3 LFPS exit signal' not read (a hypothetical)"
  - "how does a coordinated predicate find its subject ('X shall not be used ... and may be treated as Y')"
  - "what is isf_protocol_equivalence_rule_"
  - "why did adding equivalence_rules re-seal every chain"
  - "is a treated-as sentence still an explicit abstraction (yes; the typed record cites it)"
date: 2026-10-19
status: current
tags: [evidence-ir, equivalence, treated-as, carried-evidence, isf-adapter, adr-0006, adr-0016, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (EquivalenceRuleRecord; EquivalenceModality; equivalence_rule_surface; stated_equivalences; equivalence_rule_tests); crates/specforge/src/ir/semantic.rs and intent.rs (equivalence_rules); crates/specforge/src/ir/adapters.rs (protocol_residual_decisions); doctrine/production_genericity/claim_family_inventory.tsv and rule_family_inventory.tsv (evidence.normative); docs/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.ii.b.ii)
reverify: "cargo test --offline -p specforge-core --lib equivalence_rule && python3 -c \"import json,glob; print(sum(len(json.load(open(p)).get('equivalence_rules',[])) for p in glob.glob('generated/intent_ir/*/intent_ir.json')))\"   # expect 34 on the 31 current chains (2026-10-19)"
---

`.3.d.ii.b` found 31 IntentIR assumptions that are rules of the hardware stated with *treated as* (*LPIs are always
treated as Non-secure Group 1 interrupts*; *If the instruction being stepped over would normally stall the hart, then
instead the instruction is treated as a nop*; *The Deferred DPH shall be treated as a TP for buffering and credit
purposes*). Since `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.ii` (`2026-10-19`) EvidenceIR carries each as an
`EquivalenceRuleRecord { equivalence_id, subject, equivalent, condition, modality, supporting_statement_ids }` in the
`evidence.normative` family beside `conditional_rules`, and SemanticIR and IntentIR carry the list unchanged. The
sentence keeps its `explicit_abstraction` class; the typed record citing it is what makes the census count it read.

**Why not `conditional_rules`.** A `ConditionalRuleRecord` is signal-centric: an antecedent, a consequent signal and an
action. Most equivalences have neither a condition nor a signal (interrupt classes, instructions, responses, hub
requests), so forcing them in would tell every consumer of conditional rules something the sentence does not say.

**Grammar (`stated_equivalences`).** Before *treated as*, a run of auxiliaries that includes a copula (*is*, *are*,
*be*, *being*, *been*); the subject is the clause before them, from the sentence's last clause break; the condition is
the text before that break when it opens like one (*If*, *When*, *At*, *For*, *Unless*, *While*, *During*, *Once*,
*Upon*, *On*, *In*); the equivalent runs to its clause's end; the modality is the first of *shall*, *must*, *should*,
*may*, *can*, *will*, *always* among the auxiliaries, else `stated`. A coordinated predicate ends its subject at the
subject's own verb; a parenthesis holds its own subject; a reason clause (*since*, *because*) asserts its equivalence.
**Refused:** a clause with *otherwise*, *else* or *in which case* (the record would drop the contrast), a hypothetical
clause opening with *if*, *when*, *unless* or *whether*, a coordinated predicate with no verb to end at, and *treated
as* with no copula (*must be ignored and treated as one*). A table row and an apparatus statement are not read; a rule
stated twice is one record citing both statements.

**Measured `2026-10-19`.** 39 *treated as* uses on the 31 chains; 34 records in 10 documents (USB 3.2 10, GICv3 6,
IOMMU and ADIv6 4 each, RISC-V debug and AXI 3, SMMU guide, TMC, CHI C2C and the USB4 connection-manager guide 1
each), 5 refused. Each changed chain moves only `equivalence_rules` at every stage; the adapter keeps one
`isf_protocol_equivalence_rule_<id>` residual per record (`.isf` has no construct that treats one thing as another);
the emitted `.isf` texts are byte-identical. The new rule field moves each stage's rule fields, so every chain
re-seals: the 10 were rebuilt stage by stage, then `rebuild_stage_cascade.sh --write` re-sealed all 31
content-identical.

Links: [[assume-sentences-fall-into-six-shapes]], [[an-assume-sentence-about-the-document-is-apparatus]],
[[stage-seal-closure-membership]].
