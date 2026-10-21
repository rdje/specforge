---
id: a-software-agents-statement-is-software-intent
title: A statement whose subject is an agent the document makes software is a software_intent record — agent, basis and modality — carried to IntentIR as an explicit non-applicable disposition
answers:
  - "what is software_intent / SoftwareIntentRecord / SoftwareAgentBasis / SoftwareIntentModality"
  - "how does SpecForge know an agent is software (an English software noun, a glossary row, implemented in software)"
  - "why is the Connection Manager software in the USB4 connection-manager guide but not in the inter-domain spec"
  - "why is 'External accesses from a debugger' not software intent (a preposition before the noun)"
  - "why are 'output drivers' and 'Re-driver' not software (driver alone is no software noun)"
  - "what is software_intent_surface / software_subject / glossary_software_name / implemented_in_software_names"
  - "what is isf_software_intent_not_applicable"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.d.i and .52"
  - "why did 102 invariants leave the USB4 connection-manager guide and ADIv6 (withhold_software_rules, software_statement_ids)"
date: 2026-10-20
status: current
tags: [evidence-ir, software-intent, non-applicable, carried-evidence, isf-adapter, census, category-contract, adr-0006, adr-0016, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (SoftwareIntentRecord; SoftwareAgentBasis; SoftwareIntentModality; software_intent_surface; software_subject; software_agent; glossary_software_name; implemented_in_software_names; without_lead_markers; SOFTWARE_NOUNS; software_intent_tests); crates/specforge/src/ir/semantic.rs and intent.rs (software_intent); crates/specforge/src/ir/adapters.rs (software_intent_residual_packet); doctrine/spec_to_intent_category_contract.json (software_architecture_semantics, non_contract_disposition); docs/archive/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.i)
reverify: "cargo test --offline -p specforge-core --lib software_intent_tests && python3 -c \"import json,glob; es=[json.load(open(p)) for p in glob.glob('generated/intent_ir/*/intent_ir.json')]; print(sum(len(e.get('software_intent',[])) for e in es if e.get('proof_context')))\"   # 2026-10-20: 342 on the 31 proof-carrying chains"
---

No IR field said whether an agent is software, so a document's software procedure (*A Connection Manager shall
configure DP AUX Path …*; *the debugger must use the IDR register in the AP*) was indistinguishable from the
hardware's behavior. Since `EXTRACTION-REPRESENTATION-GAPS.3.d.i` (`2026-10-20`) EvidenceIR carries every
non-apparatus, non-table statement whose subject is an agent the document makes software as a `SoftwareIntentRecord {
intent_id, agent, software_name, basis, basis_statement_ids, modality, supporting_statement_ids }` (family
`evidence.normative`); SemanticIR and IntentIR carry it unchanged, and each adapter names a document's records in one
`isf_software_intent_not_applicable` residual: the category contract's explicit non-applicable disposition
(`software_architecture_semantics`, `non_contract_disposition` now name the surface).

**How an agent is software — the document's own words.** `SoftwareAgentBasis`: `software_noun` (English:
*software*, *firmware*, *operating system(s)*, *hypervisor(s)*, *device driver(s)*, *debugger(s)*; not *driver* alone,
which also names an I/O pad and *Re-driver* hardware), `glossary` (a row whose definition opens with *software* +
*entity*, *component*, *agent*, *program*, *module*, *that*, *which*, *resident*, *running* or *responsible*: the
IOMMU's *Hypervisor*, USB 3.2's *Client*), and `implemented_in_software` (*A Connection Manager can be implemented in
software or firmware*: up to four capitalized words after an article). The inter-domain spec names only *the Local
Connection Manager software*, a compound that also names hardware's software (*Host Controller software*), so its
Connection Manager is not read as software there.

**Grammar (`software_subject`, `software_agent`).** Past list markers, a note label and a leading condition clause
(*if*, *when*, *whenever*, *while*, *unless*, *after*, *before*, *once*, *during*, *until* up to its comma), the
subject runs to the first `,`/`:`/`;`/`(` or one of `SOFTWARE_SUBJECT_ENDS` (modals, auxiliaries, procedure verbs);
before a relative pronoun the subject ends and the modal is the first of the sentence after it. The name must end the
subject with at most three words before it, none a preposition or condition opener. Modality is the word after the
subject. The same statement text twice is one record.

**Measured `2026-10-20`.** 342 records in 11 chains (USB4 connection-manager guide 232, ADIv6 44, RISC-V debug 16,
IOMMU 15, external debug guide 13); 185 descriptions, 61 obligations, 54 capabilities, 36 permissions, 6
recommendations; a Python port produced the same 342. Each changed chain moves only `software_intent` at every stage;
each of the 11 adapters gains the residual; `.isf` byte-identical; the new field re-sealed every chain. Unread IntentIR
records 145 -> 138 (census `software_agent` 12 -> 5: four of its list's hardware or human names, one participle).
**`.52` (`2026-10-20`): no hardware rule from a software statement.** A statement `software_intent` reads mints no
SemanticIR invariant (`build_invariants` skips `SemanticContext::software_statement_ids`), and a conditional rule read
only from such statements is withheld (`withhold_software_rules`): -102 invariants and temporal invariants, -86 derived
constraints, -25 conditional rules; `.isf` byte-identical. Kept: 8 records that also cite hardware statements and
ADIv6's 2 actor relations (*the debugger* drives `TDI`, samples `TDO`).

Links: [[a-stated-permission-is-a-typed-record]], [[unread-intent-records-fall-into-six-families]],
[[stage-seal-closure-membership]].
