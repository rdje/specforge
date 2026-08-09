---
id: opencapi-certified-definition-refresh
title: OpenCAPI Certified Definition refresh removes diagram labels and administrative intent
answers:
  - "is the OpenCAPI 3.0 Certified Definition refresh complete"
  - "why does the OpenCAPI Certified Definition emit no ISF"
  - "why did DL disappear from the OpenCAPI Certified Definition"
  - "were DL and TL diagram labels signal declarations"
  - "how much memory did the guarded OpenCAPI Certified Definition ingest use"
  - "how many OpenCAPI Certified Definition paths are repository relative"
  - "what are the current OpenCAPI Certified Definition artifact hashes"
  - "what changed between stale and current OpenCAPI Certified Definition artifacts"
  - "how many corpus refreshes remain after OpenCAPI Certified Definition"
date: 2026-08-10
status: current
tags: [opencapi, corpus-coverage, certification, diagram-labels, semantic-authority, path-portability, isf]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.42); generated/source_ir/opencapi_3_0_certified_definition_v1_1; generated/evidence_ir/opencapi_3_0_certified_definition_v1_1; generated/semantic_ir/opencapi_3_0_certified_definition_v1_1; generated/intent_ir/opencapi_3_0_certified_definition_v1_1; generated/adapters/isf/opencapi_3_0_certified_definition_v1_1
reverify: "Hash the caller-authorized same-SSD PDF and release binary plus the 12 generated artifacts recorded below; validate the complete chain; require all 15 page image/layout pairs, eight visual records, and 46 project-owned path references repository-relative and present; confirm EvidenceIR has 167 statements and no typed hardware surface, SemanticIR has two actors, zero interfaces/phases/gates, eight invariants and four contracts, IntentIR has four behaviors/eight constraints, and the blocked adapter directory contains only adapter.json plus its report; then run the WIRE/KG/FSMGen-strict/path/locality gates recorded in CORPUS-COVERAGE.2.42."
---

**Established `2026-08-10` (`CORPUS-COVERAGE.2.42`).** The caller-authorized PDF on the same SSD hashes to
`3e7685205b51c3fde858ee6685ef36fe29254500ced1d28bbba7360f66639516` and is 234,448 bytes. Source and repository
share device `16777240`; the owning release binary hashes to
`9692af3e5019b53a6c849270b24a16a1f8bbfa4d8f94162ada23f56534c61b4b`. The normalized sidecar keeps the external source absolute with
`path_origin: external_input` while its promoted Markdown and all project-owned artifacts are repository-relative.
Two guarded CPU ingests peak at 18% and 33% sampled system memory used and reproduce 15 pages, eight visual assets,
five tables, 31 sections, 151 content elements, and zero SourceIR residuals. All 15 page image paths, 15 layout
paths, eight visual paths, and eight caption-source references are present and relative.

The stale SourceIR's 155→151 element delta removes exactly four flattened diagram labels: `DL` twice, `TL/`, and
`OpenCAPI 3.0 Host TL/`. EvidenceIR correspondingly falls 172→167: those four source-derived statements plus the
synthetic glossary declaration `Signal DL is width 1.` disappear. The final 167 statements retain 31 anchors,
eight visual records, eight links, and two conditional rules, with zero actor-signal relations, signal constraints,
registers, or timing records. A diagram or terms-table acronym is evidence vocabulary, not a declared wire.

The stale chain had five actors, one interface, six phases, 22 invariants, six contracts, seven gates, and 22
decompositions; IntentIR had two actors, one interface, 17 behaviors, and 22 constraints. Current generic signal
authority removes the `DL` interface, and child `CORPUS-COVERAGE.2.42a` prevents certification submissions,
listing requests, email review, conflict escalation, and test-lab administration from establishing semantic
intent. Final SemanticIR has two technical actors, zero interfaces/phases/gates, eight invariants, four contracts,
14 decompositions, and zero residuals. IntentIR has two actors, zero interfaces, four genuine device/host
compliance behaviors, eight constraints, and zero residuals.

The adapter has zero signals, transactions, rules, constants, enums, storage, or residuals. It blocks honestly on
`no signals declared in interface`, emits no target, and leaves exactly `adapter.json` plus its validation report.
Validation retains the guide/low-confidence classification, ten partially structured normative statements, and
eight unenriched visuals rather than manufacturing a hardware surface.

The final hashes are SourceIR/report
`6a17ce8ee989bb73e53b89b7adc26c309ffd0119d9040cd350bc3136efbe0d59` /
`edb71484824717ebcc6bc2fa525a6fb4cc84ffb954e347d2af5170629c7076ab`; page/visual manifests
`e9f5b01e1d36c7b2f2d4c969996d04cb358ca4f1030698fb61c9f5f95026fe34` /
`d26f864c11aa1da68f6fd7fffc235b5a85241ca392066ab2f6c773ad4629e442`; EvidenceIR/report
`63a94ec5e44a580372d6cc1fb47faa65d086850cd763d9db3a98d4d79ba87000` /
`692720fba9e7d5adfbcb4b31080fc856f57c8638a09a1505a0fd114bad66bfaf`; SemanticIR/report
`2622a69b821d3af6712be0869da2da2926c2275eb2fe868c9fff0de20784fdbd` /
`346724ddc56f3b6240cfc854cdeda5fbb19a2cea9ac74766ec9381212eda4644`; IntentIR/report
`211d277aa7e485efa1e0b4ea1dc1ba37fd737e472fc3974f19dbf2c2148dfd7d` /
`5824b2c6af786823f961fe22bbb5bf87dc3895687006204f7143d7f8808d7e8d`; adapter/report
`0d55f785509264fc7cc03794a229427ace0549902cac3afea475ef997c76f7cc` /
`deefda25d1b25215609dbbc10d2209359c3eaaafb3a71dcf3a1a81cdf318dc23`.

Two final cascades reproduce the six downstream hashes. Nine provider-free WIRE/I2C/SWD datasets retain their
declared filtered gates, KG is 156/156, and all 66 current emitted ISFs pass FSMGen strict. Corpus refresh is 42
done / 14 remaining, with stage census 80 SourceIR / 11 normalized / 80 EvidenceIR / 79
SemanticIR→IntentIR→adapter chains.
