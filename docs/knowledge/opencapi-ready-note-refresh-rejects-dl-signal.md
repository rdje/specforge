---
id: opencapi-ready-note-refresh-rejects-dl-signal
title: OpenCAPI Ready and Certified note refreshes reject acronym-derived DL signals
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
  - "is the OpenCAPI 3.0 Certified Test Resources engineering note refresh complete"
  - "why does the OpenCAPI Certified engineering note emit no ISF"
  - "why did the OpenCAPI Certified DL signal disappear"
  - "does the Certified sibling confirm the OpenCAPI Ready glossary result"
  - "what are the current OpenCAPI Certified artifact hashes"
  - "how many corpus refreshes remain after the OpenCAPI Certified note"
date: 2026-08-09
status: current
tags: [opencapi, corpus-coverage, engineering-note, signal-authority, glossary, path-portability, isf]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.39 and .2.40); generated/source_ir/opencapi_3_0_ready_test_resources_engineering_note_v1_0; generated/evidence_ir/opencapi_3_0_ready_test_resources_engineering_note_v1_0; generated/semantic_ir/opencapi_3_0_ready_test_resources_engineering_note_v1_0; generated/intent_ir/opencapi_3_0_ready_test_resources_engineering_note_v1_0; generated/adapters/isf/opencapi_3_0_ready_test_resources_engineering_note_v1_0; generated/source_ir/opencapi_3_0_certified_test_resources_engineering_note_v1_0; generated/evidence_ir/opencapi_3_0_certified_test_resources_engineering_note_v1_0; generated/semantic_ir/opencapi_3_0_certified_test_resources_engineering_note_v1_0; generated/intent_ir/opencapi_3_0_certified_test_resources_engineering_note_v1_0; generated/adapters/isf/opencapi_3_0_certified_test_resources_engineering_note_v1_0
reverify: "Hash both caller-authorized same-SSD PDFs, validate both complete chains, rerun evidence then semantic then intent then adapt --target isf, require every page image/layout path repository-relative and exactly adapter.json plus its report with no ISF, compare stale statements 0114/0174 and terms-section interfaces against current zero-declaration evidence, then run the WIRE/KG/FSMGen-strict/path/locality gates recorded in CORPUS-COVERAGE.2.39 and .2.40."
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

**Independent sibling confirmation (`CORPUS-COVERAGE.2.40`).** The Certified Test Resources note PDF hashes to
`b427a3029c46be2d6fa0e4d965d6bec2360c1d7a24ebdf387a1078c0af422902`. Two guarded CPU ingests peak at 21% and
22% system memory used and reproduce 13 pages, three visuals, two tables, 25 sections, 173 elements, zero
SourceIR residuals, and 13 repository-relative image plus 13 repository-relative layout paths. Evidence 173→172
removes only synthetic `statement_0174`, `Signal DL is width 1.`, from the sibling terms table; the one stale
interface `interface_explicit_interface_section_0016_terms` and adapter output disappear. SemanticIR retains two
actors, four phases, four invariants, one contract, six gates, one assertion, and 15 decompositions; IntentIR
retains one actor, 11 behaviors, four constraints, and four temporal invariants. Lowering blocks on no declared
signals plus no behavior and leaves only `adapter.json` plus its report.

The two Certified cascades reproduce all 12 hashes: SourceIR/report
`165b019ef1e2139140fb468f062c8a2633e8e5f73c30321e8c84c4c46b83eefc` /
`60cbd8f8890344f4d8c7a5e89a71371630299ff52572ece2aed2ff3dbe022bfd`; page/visual manifests
`e0327d06948f82592b2f60af22bbc99c3d677f047adf89e5d708dc6e34d96e47` /
`4e70b9a29d16f78e09a1cfea405ea1a3beb47974af8a5f1d73d16afa25dea441`; EvidenceIR/report
`82f4c62615c795eca745aef0bf91d3e1e457521a4b6e68261a61834a962c722a` /
`181e8c82287aea52243701bb7f6d0a14a5c94bd15761e87b18e10460136d6671`; SemanticIR/report
`cf2db73c2a1f3726ff9d23679471d8f493b88f1e859361335ebf5c49d601602d` /
`edf0803602a26f0405f1c10ac260870b769c405a1fca54e35ad3c03e8fa1c412`; IntentIR/report
`8e5699edf58b87a928a06df0b476e3cdbd266c83603aef681e3f7b8a9d3000ca` /
`c269c6a2ef492dd62893518627f51b3a7c13f6b82cffec1b5f9f3978df0e36a1`; adapter/report
`deb2be06757e3a860cf402f301940fcc6cda4e4ce679252483230b8a83dd5011` /
`12e7bb6f7f0ca0757e53b40a987ebb006bf43bf64be0ce44605396a3ee3c1309`.
WIRE/I2C/SWD, KG 156/156, all 66 emitted ISFs, persisted paths, and locality remain green. Corpus refresh is now
40 done / 16 remaining, with 80 SourceIR / nine normalized / 80 EvidenceIR / 79 SemanticIR→IntentIR→adapter
chains. The sibling agreement confirms glossary rejection is structural rather than document-specific.
