---
id: opencapi-ready-definition-refresh
title: OpenCAPI Ready Definition refresh removes diagram labels and legacy semantic scaffolds
answers:
  - "is the OpenCAPI 3.0 Ready Definition refresh complete"
  - "why does the OpenCAPI Ready Definition emit no ISF"
  - "why did TL disappear from the OpenCAPI Ready Definition"
  - "were TL and DL diagram labels signal declarations"
  - "how much memory did the guarded OpenCAPI Ready Definition ingest use"
  - "how many OpenCAPI Ready Definition paths are repository relative"
  - "what are the current OpenCAPI Ready Definition artifact hashes"
  - "what changed between stale and current OpenCAPI Ready Definition artifacts"
  - "how many corpus refreshes remain after OpenCAPI Ready Definition"
date: 2026-08-10
status: current
tags: [opencapi, corpus-coverage, diagram-labels, semantic-authority, path-portability, isf]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.43); generated/source_ir/opencapi_3_0_ready_definition_v1_1; generated/evidence_ir/opencapi_3_0_ready_definition_v1_1; generated/semantic_ir/opencapi_3_0_ready_definition_v1_1; generated/intent_ir/opencapi_3_0_ready_definition_v1_1; generated/adapters/isf/opencapi_3_0_ready_definition_v1_1
reverify: "Hash the caller-authorized same-SSD PDF and release binary plus the 12 generated artifacts recorded below; validate the complete chain; require all 15 page image/layout pairs, six visual records, and 42 project-owned path references repository-relative and present; confirm EvidenceIR has 173 statements and no typed hardware surface, SemanticIR has three actors, zero interfaces/phases/gates, 21 invariants and 19 contracts, IntentIR has 19 behaviors/25 constraints, and the adapter blocks only on no signals; then run the WIRE/KG/FSMGen-strict/path/locality gates recorded in CORPUS-COVERAGE.2.43."
---

**Established `2026-08-10` (`CORPUS-COVERAGE.2.43`).** The caller-authorized PDF on the same SSD hashes to
`cf553953859c1e737622c1836711cad310e8455c5ab11ef11457c1a1e7de80c0` and is 249,595 bytes. Source and repository
share device `16777240`; the owning release binary hashes to
`5a43f1b5894ba98ffd332f56b5e83823b02efa475b06d5743d574adc54d7c11f`. The external source remains explicitly
labeled while all project-owned artifact paths are repository-relative. One guarded CPU ingest peaks at 62%
sampled system memory used and produces 15 pages, six visual assets, three tables, 30 sections, 169 content
elements, and zero SourceIR residuals. All 15 page-image paths, 15 layout paths, six visual paths, and six
caption-source references are present and relative.

The stale SourceIR's 171→169 element delta removes exactly two flattened diagram labels: `TL/` and
`OpenCAPI 3.0 Host TL/`. EvidenceIR correspondingly falls 176→173: those two source-derived statements plus the
synthetic glossary declaration `Signal DL is width 1.` disappear. The final 173 statements retain 30 anchors,
six visual records, six links, and two conditional rules, with zero actor-signal relations, signal constraints,
registers, or timing records. Diagram and glossary acronyms do not declare a wire.

The stale chain had six actors, one interface, seven phases, five gates, 30 invariants, 20 contracts, seven
assertions, and 20 decompositions; IntentIR had three actors, one interface, 30 behaviors, and 34 constraints.
Current shared authority removes the acronym-derived interface and false actors. Children `.2.43a`/`.2.43a.i`
retire untyped section phases, while `.2.43b` retires whole cue-bearing gate copies. Final SemanticIR has three
grounded actors, zero interfaces/phases/gates, 21 invariants, 19 contracts, five assertions, two abstractions,
17 decompositions, two typed conditional rules, and zero residuals. IntentIR has three actors, zero interfaces,
19 genuine compliance behaviors, 25 constraints, two assumptions, and zero residuals.

The adapter has zero signals, transactions, rules, constants, enums, storage, or residuals. It blocks honestly
on `no signals declared in interface`, emits no target, and leaves only `adapter.json` plus its validation report.
The final hashes are SourceIR/report
`40104b012424a787befc572511018756875f2148a680793641a7bec3c7190a13` /
`48a4348038a945d738248767ab3362b8cf4926bda1a184f39b2a16d78c2d28c3`; page/visual manifests
`59c7d3096a0fb311d118f08e06fa0a271b0f2dfa3cdc23bb0ecd77dba93bf5b2` /
`dedd1a54a661bfa5d353fd19a8f01132044d35761492519754644d0e948bf0b4`; EvidenceIR/report
`410548a7067b815ab75378f90d0ef8b6705c0d9e3505c2e579ae8935abf4f34c` /
`c7a0012d5cdbbf7e37502f08aa3dd349700cb3611d05ee8993ae1b317239b88a`; SemanticIR/report
`0f111b6610f4e70ad23f07a0b8b31f2164e3a7285e5df6340f3963295044b601` /
`53fcb3f51882bf7af04f38319d01b4a311780f33e1e12089e929bbe2a6fe5ceb`; IntentIR/report
`91797f8f1052360a770f217e3cc66b322831011636596a6ec3b201aebc5e9300` /
`91ec2866703efa1a98d7b8b7c694a225e23f6ef75038ccd2f98db9d19d689f20`; adapter/report
`180271836a6e8348771ad18c2444fc02d1c61a4d0114a8845c45791c30c561b6` /
`0964bbe5132d245a7fedd0b01124a1e463f6c593c579fc4b37209b1ccbceee3d`.

Two final downstream cascades reproduce all six hashes. Nine provider-free WIRE/I2C/SWD datasets retain their
declared gates, KG is 156/156, and all 66 current emitted ISFs pass FSMGen strict with zero diagnostics. Corpus
refresh is 43 done / 13 remaining, with stage census 80 SourceIR / 12 normalized / 80 EvidenceIR / 79
SemanticIR→IntentIR→adapter chains.
