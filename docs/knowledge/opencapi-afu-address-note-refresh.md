---
id: opencapi-afu-address-note-refresh
title: OpenCAPI AFU Address Space Usage refresh rejects prose acronyms and legal behavior
answers:
  - "is the OpenCAPI AFU Address Space Usage refresh complete"
  - "why does the OpenCAPI AFU Address Space Usage note emit no ISF"
  - "why did AFU BAR CFG GB ID MEM MMIO and PASID disappear as signals"
  - "were the OpenCAPI AFU address-space acronyms signal declarations"
  - "how many OpenCAPI AFU address note page paths are repository relative"
  - "how much memory did the guarded OpenCAPI AFU address note ingest use"
  - "what are the current OpenCAPI AFU address note artifact hashes"
  - "what did the OpenCAPI AFU address note prove about legal boilerplate"
  - "how many corpus refreshes remain after OpenCAPI AFU Address Space Usage"
date: 2026-08-09
status: current
tags: [opencapi, corpus-coverage, engineering-note, signal-authority, legal-boilerplate, path-portability, isf]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.41); generated/source_ir/opencapi_afu_address_space_usage; generated/evidence_ir/opencapi_afu_address_space_usage; generated/semantic_ir/opencapi_afu_address_space_usage; generated/intent_ir/opencapi_afu_address_space_usage; generated/adapters/isf/opencapi_afu_address_space_usage
reverify: "Hash the caller-authorized same-SSD PDF and the 12 generated artifacts recorded below; validate the complete chain; require all 14 page image/layout pairs and nine visual paths repository-relative and present; confirm EvidenceIR has 106 statements with zero typed hardware surfaces, SemanticIR has zero actors/interfaces/phases/gates with 15 invariants, IntentIR has zero behaviors with 15 constraints, and the blocked adapter directory contains only adapter.json plus its report; then run the WIRE/KG/FSMGen-strict/path/locality gates recorded in CORPUS-COVERAGE.2.41."
---

**Established `2026-08-09` (`CORPUS-COVERAGE.2.41`).** The caller-authorized same-SSD PDF hashes to
`0ae0c01c8087169d1b782a61906fb2518a63c197011e5988b4c196e910c6fbe3`. Two guarded CPU ingests peak at 20%
and 19% system memory used and reproduce 14 pages, nine visual assets, two tables, 23 sections, 118 content
elements, and zero SourceIR residuals. All 14 page image paths, 14 layout paths, and nine visual paths are final,
present, and repository-relative. Source, repository, rollback, and output share SSD device `16777240`.

EvidenceIR remains at 23 anchors, 106 spans, nine visual records, 50 links, and 106 statements, with zero actor-
signal relations, signal constraints, conditional rules, registers, timing records, or structured extractors
fired. The five retained low-confidence interfaces grouped ordinary explanatory acronyms—`AFU`, `BAR`, `CFG`,
`GB`, `ID`, `MEM`, `MMIO`, and `PASID`—rather than declared wires. Current generic interface authority therefore
removes five interfaces, five derived channel actors, and all eight one-bit adapter outputs without an acronym,
vendor, or document exception.

The first fresh cascade also exposed a separate legal-authority defect: a permissions paragraph under generic
heading `Approved` contained `while` and became a phase, gate, invariants, and Intent behavior. Child
`CORPUS-COVERAGE.2.41a` repaired that universal boundary. EvidenceIR still preserves the legal text, while final
SemanticIR has zero actors/interfaces/phases/gates, 15 engineering invariants, 14 decompositions, and zero
residuals. IntentIR has zero actors/interfaces/behaviors, 15 constraints/temporal invariants, and two honest
canonicalization residuals. The adapter blocks on no declared signals plus no behavioral content, emits no
target, and leaves exactly `adapter.json` plus its validation report.

The complete final hashes are SourceIR/report
`562b1befbba883272932542dba9be9afba121c469c796e80f1c03ffc17a0cb20` /
`a8836598ad77d5f2e5036d644cf1fdc6c3325f6763d3b80ce4d1a14460226dc3`; page/visual manifests
`1ca9ba0ad1f6b4a2023860ca4d04ebbb6ad0157350c70702dd9eb93d084fc428` /
`fc441aca59e0a21dcca142930c08b0218a12d72ca8e4a2c85f4be64a4c40d4b4`; EvidenceIR/report
`60c910c06c3ff3d8ff89f93bfa93f5c6d9a9303b808d30b06a739c479044916d` /
`c6095764d246663ff196d9b35fee1003aff9ce873c2929a29ee9a55dc8c70459`; SemanticIR/report
`2f38bc36d6f451a5785ef4b47e3a9ad4465357388c3ff6b9c8bfe90aa48dedf7` /
`c879198903e8c7db059958100e3d15403f38b2b454b877542dfd66be743ee7d4`; IntentIR/report
`03e9adb9061001f4d4e86d2e524b5ea25205e980349342d62bbfd20af61473a1` /
`d4846b1b8fea8d72611393973d15bc164f27765e40f8db03c394d05f9a460e2e`; adapter/report
`64d5a9f5eca28644045a67d1ac810fda1f772ffd4a880f6a38c28210410343bb` /
`43a2a01373251e8940bceee7149f32de0c6c86ce14e349c45450df13d5883740`.

Nine provider-free WIRE/I2C/SWD datasets retain their declared filtered gates, KG is 156/156, full CI passes
1,790 tests with five ignored, and all 66 current emitted ISFs pass FSMGen strict. Corpus refresh is 41 done / 15
remaining, with stage census 80 SourceIR / ten normalized / 80 EvidenceIR / 79 SemanticIR→IntentIR→adapter chains.
