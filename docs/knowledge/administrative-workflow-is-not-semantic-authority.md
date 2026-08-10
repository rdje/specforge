---
id: administrative-workflow-is-not-semantic-authority
title: Organizational administration remains evidence but cannot establish engineering intent
answers:
  - "can certification administration become a SemanticIR phase or gate"
  - "why did OpenCAPI email review become an IntentIR behavior"
  - "how does SpecForge distinguish protocol requests from product listing requests"
  - "are reference lists allowed to establish semantic intent"
  - "where are administrative workflows filtered"
  - "does SpecForge preserve certification workflow source evidence"
  - "how are test laboratory procedures distinguished from hardware test requirements"
  - "does the administrative classifier name OpenCAPI or a vendor"
  - "how many administrative workflow statements were measured"
date: 2026-08-10
status: current
tags: [semantic-ir, intent-ir, authority, administration, certification, corpus-coverage]
evidence: docs/research/administrative-workflow-semantic-authority-measurement.md; docs/tasks/CORPUS-COVERAGE.md (.2.42a); crates/specforge/src/ir/semantic.rs; docs/book/src/pipeline/semanticir.md
reverify: "cargo test -p specforge administrative_ --quiet && jq '{statements:(.extracted_statements|length)}' generated/evidence_ir/opencapi_3_0_certified_definition_v1_1/evidence_ir.json && jq '{actors:(.actors|length),phases:(.phases|length),gates:(.gates|length)}' generated/semantic_ir/opencapi_3_0_certified_definition_v1_1/semantic_ir.json && jq '{behaviors:(.behaviors|length),constraints:(.constraints|length)}' generated/intent_ir/opencapi_3_0_certified_definition_v1_1/intent_ir.json"
---

`EvidenceIR` retains source-grounded organizational process: reference lists, certification submissions,
product-listing requests, email review, escalation, mark licensing, and third-party test-lab administration.
Retention does not authorize those statements to create engineering actors, phases, invariants, contracts,
gates, interfaces, or decompositions.

`CORPUS-COVERAGE.2.42a` extended the shared `SemanticContext::from_evidence_ir` authority boundary with compound,
word-bounded administrative signatures. The rule requires organizational partners: a submission plus an
administrative channel/listing object, a conflict plus institutional escalation, or a test lab plus
certification-program administration. Exact generic reference-list headings are excluded as sections. There is
no OpenCAPI, vendor, organization, document, section-index, exact-sentence, or single-token exception.

The retained-corpus projection found 238 candidate administrative/reference statements across 23 documents.
On the real OpenCAPI Certified Definition, unchanged 167-statement EvidenceIR rebuilt from five semantic phases,
six generic gates, and 15 Intent behaviors to zero phases, zero gates, and four genuine product-compliance
behaviors. Generic gate production was subsequently retired corpus-wide; see
[[legacy-generic-gates-are-audit-only]].
Technical controls—including Page Request/response, electrical contacts, write-conflict resolution, measured
test reports, and certified-device electrical requirements—remain eligible. See the
[measurement](../research/administrative-workflow-semantic-authority-measurement.md) and the
[SemanticIR chapter](../book/src/pipeline/semanticir.md).
