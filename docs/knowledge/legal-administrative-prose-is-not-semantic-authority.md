---
id: legal-administrative-prose-is-not-semantic-authority
title: Legal and administrative prose is retained as evidence but cannot establish semantic intent
answers:
  - "can legal boilerplate become a SemanticIR gate"
  - "why did an OpenCAPI permissions paragraph become an IntentIR behavior"
  - "how does SpecForge distinguish legal conditions from protocol conditions"
  - "does SemanticIR keep copyright and license text"
  - "where is legal administrative prose filtered"
  - "why is section-title boilerplate filtering insufficient"
  - "does the legal statement classifier use a vendor or document denylist"
  - "what prevents the word while in a license notice from becoming a gate"
  - "how many retained legal semantic gates were measured"
date: 2026-08-09
status: current
tags: [semantic-ir, intent-ir, authority, legal-boilerplate, gates, corpus-coverage]
evidence: docs/research/legal-administrative-semantic-authority-measurement.md; docs/tasks/CORPUS-COVERAGE.md (.2.41a); crates/specforge/src/ir/semantic.rs; docs/book/src/pipeline/semanticir.md
reverify: "cargo test -p specforge legal_ --quiet && jq '{statements:(.extracted_statements|length)}' generated/evidence_ir/opencapi_afu_address_space_usage/evidence_ir.json && jq '{phases:(.phases|length),invariants:(.invariants|length),gates:(.gates|length)}' generated/semantic_ir/opencapi_afu_address_space_usage/semantic_ir.json && jq '{behaviors:(.behaviors|length),constraints:(.constraints|length)}' generated/intent_ir/opencapi_afu_address_space_usage/intent_ir.json"
---

`EvidenceIR` preserves source-grounded legal and administrative text. Preservation does not grant semantic
authority. Before `CORPUS-COVERAGE.2.41a`, SemanticIR excluded known boilerplate section titles but trusted every
statement under an unrecognized or generic heading. A rights-and-permissions paragraph under `Approved`
contained `while`; the conditional-cue gate builder promoted it into a SemanticIR gate and IntentIR behavior.

The fix is a compound, word-bounded legal/administrative predicate at
`SemanticContext::from_evidence_ir`, before every semantic consumer. Copyright, licensing, warranty/liability,
patent/IP, revocable-permission, commercial-term, and IPR concepts require contextual partners. Single words such
as permission, rights, version, reliability, license, or `Copyright` do not trigger the boundary. The production
rule contains no vendor, organization, document key, exact source sentence, or section index.

The retained-corpus before projection found 32 legal/administrative gates across 21 documents among 27,180
total gates; 26 lacked a related interface. The real #41 rebuild preserves all 106 EvidenceIR statements while
removing its false phase/gate and both legal Intent behaviors. A legitimate `While READY is low, VALID must remain
asserted` condition survives in the regression. See the [SemanticIR chapter](../book/src/pipeline/semanticir.md)
and the [measurement](../research/legal-administrative-semantic-authority-measurement.md) for the public contract
and exact evidence.
