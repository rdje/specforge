---
id: opencapi-discovery-configuration-refresh
title: OpenCAPI Discovery Configuration refresh removes false topology but remains under-extracted
answers:
  - "what happened in OpenCAPI Discovery Configuration refresh 46"
  - "why does OpenCAPI Discovery Configuration no longer emit ISF"
  - "why were BDF DL and VPD removed from OpenCAPI Discovery"
  - "what remains under-extracted in OpenCAPI Discovery Configuration"
  - "what are the final OpenCAPI Discovery artifact hashes"
  - "was OpenCAPI Discovery ingest deterministic and memory safe"
  - "are OpenCAPI Discovery normalized paths portable"
  - "how many corpus refreshes remain after OpenCAPI Discovery"
date: 2026-08-10
status: current
tags: [corpus-refresh, opencapi, discovery, configuration, source-ir, evidence-ir, semantic-ir, intent-ir, isf]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.46); .cache/local-references/chipdoc/cxl/opencapi/current/OpenCAPI_Discovery_Configuration_v201.pdf; generated/source_ir/opencapi_discovery_configuration_v201; generated/evidence_ir/opencapi_discovery_configuration_v201; generated/semantic_ir/opencapi_discovery_configuration_v201; generated/intent_ir/opencapi_discovery_configuration_v201; generated/adapters/isf/opencapi_discovery_configuration_v201
reverify: "Authenticate the repo-local source alias at .cache/local-references/chipdoc/cxl/opencapi/current/OpenCAPI_Discovery_Configuration_v201.pdf as SHA-256 bc767d6eb354f9664459fdaa4c36e945cc1c68baece1b4b381f4dbef7dcfe103 and 403646 bytes. Build release, ingest it three times with DOCLING_DEVICE=cpu and the built-in RAM guard, then rebuild and validate SourceIR through ISF twice. Require identical SourceIR and downstream hashes, all 188 normalized path values relative and present, no emitted ISF target, and the stage/count/delta surfaces recorded below. Run scripts/check_persisted_artifact_paths.pl, scripts/check_project_data_locality.sh, mdbook test/build, and scripts/check_doctrines.sh."
---

**Established `2026-08-10` (`CORPUS-COVERAGE.2.46`).** The authenticated 403646-byte source has SHA-256
`bc767d6eb354f9664459fdaa4c36e945cc1c68baece1b4b381f4dbef7dcfe103`. Three guarded CPU ingests reproduce one
SourceIR hash and produce 40 pages / 54 visuals / 49 tables / 50 sections / 172 elements at high confidence with
zero SourceIR residuals. A 51-sample memory trace holds system memory at 47–53% free; the built-in 85%-used abort
never fires. The normalized bundle contains 139 files / 40288658 bytes. All 188 project-owned path values (80
page/layout and 108 visual/caption paths) are repository-relative, present, and final-rooted.

The stale 181-element SourceIR loses exactly nine flattened visual labels: one on page 1 and eight on page 13;
both pages retain their visual evidence. EvidenceIR changes 831→754 statements with no addition. The 77 removals
are those nine labels, 65 synthetic `CONTENTS`/`REPRESENTATION`/`TABLE`/`VITAL` enum statements, and synthetic
one-bit declarations for `BDF`, `DL`, and `VPD`. Existing shared signal authority removes `BDF`/`DL`; child
`CORPUS-COVERAGE.2.46a` closes the distinct bare-`data` ambiguity that admitted `Vital Product Data (VPD)`.
Final EvidenceIR has 50 anchors / 754 spans and statements / 54 visuals / 643 links / zero actor-signal
relations / four conditionals / one register / six timings.

Current shared semantic authority removes the mixed-vintage interfaces, port, relation, phases, and gates.
Final SemanticIR has four actors / zero interfaces, ports, relations, phases, or gates / 55 invariants / ten
contracts / five assertions / 37 decompositions / six abstractions / six timings. IntentIR has three actors /
zero interfaces, ports, or relations / ten behaviors / 55 constraints / six assumptions / six timings. The
adapter correctly blocks on `no signals declared in interface`; it has zero signals, transactions, rules,
constants, or enums, retains one storage record, and reports one `isf_register_fields_not_lowered` residual for
206 fields. It leaves `adapter.json` plus its validation report and emits no `.isf` target.

This is a faithful refresh, not a completeness claim. Validation identifies 37 partially structured normative
statements, 15 unexplained intent-bearing tables, 54 unenriched visuals, one register with overlapping fields,
ten ambiguous statements, and eight temporal-source IDs without typed temporal rules. Those remain the honest
table/vision/temporal capture frontier; they do not authorize guessed topology or behavior.

The final committed-binary artifact hashes are:

- SourceIR `639a98233b4fe5eff3147bc0cc1f43512b39c30da551162979dda682cd9c6393`; source report
  `44127e250f9570d2c68662040bfee41f83f15376fd509089caafe2a0911cb417`; page manifest
  `47c6816d87925e7c4d662226fbb805acb855c8f43bc8511197e42851ef7a7a4f`; visual manifest
  `1610415e7affe5a32141950a2667a870c6370407fbc7fe589a608608d577248d`.
- EvidenceIR `6ea1430b430c32f533ba3193990dd8b88710e73fdbf6e43eab823d20f8af6846`; report
  `606995fc77d5db93f2b63faee3dde88c7372109df2e297119c40ea5a5f256177`.
- SemanticIR `9486286f8ed4265d47250c2efddbb8ad7bfa14b5a777e92f4526ac53cba7d7f1`; report
  `c51e6a1f886496da16cceb5f081cbcae38c75bcb664f9906888fa82ee7be91f3`.
- IntentIR `fb296f0e7463f1f05742f6c4549444798473209ee54cfb96455195b6e800ac10`; report
  `f4995c03c5e1db3f77bf3fb8c1ec765e3a10c810e5bb323f55e89aeab665e8a8`.
- Adapter `b5bb6626c2bbc58ef78621dcf23d7a16ecc3b1f635a5faa7cf7e0dae0122d7eb`; report
  `41f7ee85f328cc92e145d778b67d18dc8d5b7a720350abb9e5f4d8543c5d8f54`.

Two repaired cascades reproduce the eight downstream hashes, and the final release replay validates all five
stages. After this refresh the corpus census is 80 SourceIR / 18 normalized / 80 EvidenceIR / 79 downstream
chains, 46 refreshes are done, ten authenticated chip-spec candidates remain, and all 61 emitted ISFs are
FSMGen-strict clean.
