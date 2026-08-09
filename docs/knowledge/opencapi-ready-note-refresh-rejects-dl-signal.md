---
id: opencapi-ready-note-refresh-rejects-dl-signal
title: OpenCAPI Ready engineering-note refresh rejects an acronym-derived DL signal
answers:
  - "is the OpenCAPI 3.0 Ready Test Resources engineering note refresh complete"
  - "why does the OpenCAPI Ready engineering note emit no ISF"
  - "why did the OpenCAPI Ready DL signal disappear"
  - "was DL a signal declaration in the OpenCAPI Ready note"
  - "what did statement 0114 say in the stale OpenCAPI Ready evidence"
  - "did the OpenCAPI Ready terms table declare an interface"
  - "how many OpenCAPI Ready page paths are repository relative"
  - "how much memory did the guarded OpenCAPI Ready ingest use"
  - "what are the current OpenCAPI Ready artifact hashes"
  - "how many corpus refreshes remain after the OpenCAPI Ready note"
date: 2026-08-09
status: current
tags: [opencapi, corpus-coverage, engineering-note, signal-authority, glossary, path-portability, isf]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.39); generated/source_ir/opencapi_3_0_ready_test_resources_engineering_note_v1_0; generated/evidence_ir/opencapi_3_0_ready_test_resources_engineering_note_v1_0; generated/semantic_ir/opencapi_3_0_ready_test_resources_engineering_note_v1_0; generated/intent_ir/opencapi_3_0_ready_test_resources_engineering_note_v1_0; generated/adapters/isf/opencapi_3_0_ready_test_resources_engineering_note_v1_0
reverify: "Hash the caller-authorized same-SSD PDF, validate the complete chain, rerun evidence then semantic then intent then adapt --target isf, require 10 repository-relative page image/layout paths and exactly adapter.json plus its report with no ISF, compare stale statement_0114 and interface_explicit_interface_section_0018_terms against current zero-declaration evidence, then run the WIRE/KG/FSMGen-strict/path/locality gates recorded in CORPUS-COVERAGE.2.39."
---

**Established `2026-08-09` (`CORPUS-COVERAGE.2.39`).** The caller-authorized same-SSD PDF hashes to
`7df9af5f1cdebf1a5e21bfc3a994e2024224f4e004cb656d3d0695f1e0240365`; the current release binary hashes to
`5175bf62061b0e1c3b426bc99a8f38e570e9b8ec31538ea7301dab926cd3d8f4`. Two guarded CPU ingests each peak at
18% system memory used and produce the same 10 pages, six visuals, five tables, 23 sections, 105 content
elements, and zero SourceIR residuals. Every page image and layout path is final and repository-relative. The
refreshed external-source provenance names the authorized SSD input instead of the obsolete boot-volume route.

The source content is byte-structurally stable, but EvidenceIR falls 113→112 statements. Exact comparison shows
that the only missing record is the synthetic `statement_0114`, `Signal DL is width 1.` Its source span points
into the `Terms` section. The actual headerless table defines `DL` as the OpenCAPI data link layer alongside
abbreviations such as `AFU`, `DLx`, `DUT`, and `PHY`; it is a glossary, not a signal inventory or formal interface
declaration. Current extraction therefore reports zero extractors fired and does not regenerate that statement.

The stale synthetic statement had created high-confidence SemanticIR interface
`interface_explicit_interface_section_0018_terms`, containing one width-one `DL` record, and the blocked adapter
rendered it as one output. Current SemanticIR keeps two actors, three phases, four invariants, six contracts,
three gates, one assertion, and 13 decompositions while producing zero interfaces, ports, relations, or
connectivity. IntentIR keeps one actor, 12 behaviors, five constraints, and four temporal invariants with the same
zero hardware surface. Lowering blocks on both `no signals declared in interface` and `no behavioral content`,
emits no target, and leaves only `adapter.json` plus its validation report.

Validation classifies the note as a low-structured-design-intent guide. Its purpose category remains explicitly
unresolved at low confidence between physical/electrical and methodology/guide because the front matter supplies
no decisive cue; two partially structured normative statements remain visible candidate misses. This is honest
classification uncertainty, not permission to fabricate a signal from glossary vocabulary.

Two complete guarded ingests and cascades reproduce all 12 hashes: SourceIR/report
`f9b97587df8554ba9f9f4974f1f7f9c81d7a006edb4a1ae59612035b5c0bff40` /
`4d1b3bf96e08fa5be55dc4315c200538095c9364a0cc46e52c058b99fbff0d8d`; page/visual manifests
`b08eca00b3e36e0918cae69d179936a8e442408f692efa6611916b18cc4c2ccd` /
`b382a82ea6c6b28f19c87555ed0b1de0209e11f492dcdb876a0655539f89d7c2`; EvidenceIR/report
`4653ebff7d7a31a87abea3e03ee6d764cdcab6f6ab9673d83eeb3899f5debbc2` /
`ae481b3692cc9d8696cf8e7082c969cdf803ffde0547632029bad74686d81c19`; SemanticIR/report
`886fcd43bc5ccc5f312fb25c24b27bac7c9ce00c8282f380332246839f26e2bb` /
`ac6ef4bd5aacaa4f7004cba9fb570542be5dbac4ba7e369e1377db68d78c39d6`; IntentIR/report
`91d89ec1abfa18692484531653c6744180ebf13dd658534e4de39c57bbd59d54` /
`43f53326d8b69767a59a8875de045c16c13d1931716ed370913a3d69c49fdb82`; adapter/report
`c53d54962bf208d77c09cc8ee5083f2c9b48fe51efd8684348410c2e32a2f16f` /
`37fdea04e15164fea06ee990716fbab4a5408b9683d59316840fe9bd475c2e77`.

All nine provider-free WIRE/I2C/SWD datasets hold their declared filtered gates, including the documented
promotion-only SWD constraint miss. KG is 156/156, all 66 emitted ISFs pass FSMGen strict, and persisted-path plus
project-data-locality gates pass. Corpus refresh is 39 done / 17 remaining, with stage census 80 SourceIR / eight
normalized / 80 EvidenceIR / 79 SemanticIR→IntentIR→adapter chains.
