---
id: a-stated-permission-is-a-typed-record
title: A sentence stating what a subject may do is a permission_rules record — subject, permitted part, condition and modality as written — carried to IntentIR, with one adapter residual per document
answers:
  - "what is permission_rules / PermissionRuleRecord / PermissionModality"
  - "how is 'If the aborted transaction is a write then the IO bridge may discard the write' read"
  - "what is stated_permissions / permission_rule_surface"
  - "why is 'may lead to' or 'may change' not a permission (a verb of what can happen)"
  - "why is 'This document may include technical inaccuracies' not a permission (about the document)"
  - "why does 'It is permitted to signal an error' have no subject"
  - "why is 'the type of transactions that a device may generate' refused (a subject holding a relative clause)"
  - "what is isf_protocol_permission_rules"
  - "why did the unread IntentIR records fall from 315 to 151"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.d.iii.a"
date: 2026-10-20
status: current
tags: [evidence-ir, permission, may, carried-evidence, isf-adapter, census, adr-0006, adr-0016, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (PermissionRuleRecord; PermissionModality; permission_rule_surface; stated_permissions; may_states_no_permission; names_the_month; speaks_about_the_document; permission_rule_tests); crates/specforge/src/ir/semantic.rs and intent.rs (permission_rules); crates/specforge/src/ir/adapters.rs (permission_residual_packet); scripts/measure_category_family_census.py (TYPED_SURFACES); docs/tasks/extraction-representation-gaps/default-and-assumption-leaves.md (.3.d.iii.a)
reverify: "cargo test --offline -p specforge-core --lib permission_rule && python3 -c \"import json,glob; es=[json.load(open(p)) for p in glob.glob('generated/intent_ir/*/intent_ir.json')]; print(sum(len(e.get('permission_rules',[])) for e in es if e.get('proof_context')))\"   # 2026-10-20: 890 on the 31 proof-carrying chains"
---

`.3.d.iii` found that 154 of the 176 unread IntentIR behaviors grant something. Since
`EXTRACTION-REPRESENTATION-GAPS.3.d.iii.a` (`2026-10-20`) EvidenceIR carries every permission a sentence states with
*may* (any case), *is/are (also) permitted to* or *is/are (also) allowed to* as a `PermissionRuleRecord {
permission_id, subject, permitted, condition, modality, supporting_statement_ids }` in the `evidence.normative` family
beside `equivalence_rules`; SemanticIR and IntentIR carry the list unchanged. Example:
`{"subject": "the IO bridge", "permitted": "discard the write", "condition": "If the aborted transaction is a write",
"modality": "may"}`.

**Grammar (`stated_permissions`).** The subject is the clause before the marker from the sentence's last clause break
(bullets, *Note that*, *Therefore*, *since* read past); the condition is the part before that break when it opens like
one (`PERMISSION_CONDITION_OPENERS`, *after* and *before* included), or the *if* clause of *If X then Y may Z*; the
permitted part runs to the first `. `, `; `, `, ` or `(`, and stops before a second permission's coordination (*… or a
vector may be shared* is a second record). *It is permitted to* and *There may be* have no subject. **Refused** (530
uses on the 31 chains): *may* + a verb of what can happen (`POSSIBILITY_VERBS`: *lead*, *result*, *change*, *vary*,
*require* …), *may not*, *may never*, *may or may*, *may have to*, *may have* + participle, *may be due*; a
hypothetical or concession with no *then*; *otherwise*/*else*; a pronoun subject or *you*; a subject that holds *that*,
*which*, *whom* or *whose* after its first word or ends with one; a subject opening with *no*, *neither*, *none*, *nor*
(a prohibition); a subject with a verb no later *and*/*or*/*but* ends; a copula in the permitted part with no
subordinator before it (*may draw is limited*); a document noun after *this* in subject or permitted part; *May*
beside a number (a date). Table rows and apparatus are not read; a permission stated twice is one record.

**Not decided by grammar.** *May* is not told apart from a possibility: *At this point the eye may be closed* and *the
time to complete the request may be longer* are records with `modality: may`. A subject is the clause as written, so
one with an unpunctuated lead-in (*Subsequent to reading each command the IOMMU*) keeps it.

**Measured `2026-10-20`.** 890 records in 29 chains (USB 3.2 335, IOMMU 179, USB4 connection-manager guide 65,
RISC-V debug 61, Wishbone 49); 786 *may*, 81 *permitted*, 23 *allowed*; 87 records have a
software subject (*Software*, *the Connection Manager*, *a hypervisor*), which `.3.d.i` owns as software intent. A
Python port of the grammar produced the same 890 records. Each changed chain moves only `permission_rules` at every
stage; each adapter gains one `isf_protocol_permission_rules` residual naming its records by id range; `.isf` texts are
byte-identical; the new rule field re-sealed every chain (`rebuild_stage_cascade.sh --write`, 31 content-identical).
Unread IntentIR records 315 -> 151 (behaviors 274 -> 112): `permission_or_description` 176 -> 51 (permission 154 ->
32), `software_agent` 43 -> 9, `apparatus` 26 -> 21. *Is optional* (76 statements) is `.3.d.iii.d`.

Links: [[an-equivalence-rule-is-a-typed-record]], [[unread-behaviors-are-permissions-and-the-timing-class-misfires]],
[[stage-seal-closure-membership]].
