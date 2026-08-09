---
id: aarch64-external-debug-guide-refresh-is-authority-empty
title: AArch64 External Debug Guide refresh is portable and authority-empty
answers:
  - "is the AArch64 External Debug Guide current-binary refresh complete"
  - "why does the AArch64 External Debug Guide emit no ISF"
  - "what happened to the AArch64 External Debug Guide agent.isf"
  - "were the 73 stale AArch64 External Debug signals authoritative"
  - "why did 64 AArch64 External Debug interfaces disappear"
  - "why did the host actor disappear from AArch64 External Debug"
  - "is AArch64 External Debug a methodology guide or under-extracted architecture"
  - "how many AArch64 External Debug page sidecars are repository relative"
  - "what are the current AArch64 External Debug artifact hashes"
  - "how much memory did the guarded AArch64 External Debug ingest use"
  - "how many corpus refreshes remain after AArch64 External Debug"
date: 2026-08-09
status: current
tags: [aarch64, external-debug, corpus-coverage, guide, signal-authority, path-portability, isf]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.37); generated/source_ir/102196_0100_01_2022_05_05_aarch64_external_debug_guide; generated/evidence_ir/102196_0100_01_2022_05_05_aarch64_external_debug_guide; generated/semantic_ir/102196_0100_01_2022_05_05_aarch64_external_debug_guide; generated/intent_ir/102196_0100_01_2022_05_05_aarch64_external_debug_guide; generated/adapters/isf/102196_0100_01_2022_05_05_aarch64_external_debug_guide
reverify: "Hash the caller-authorized same-SSD PDF, validate the complete chain, rerun evidence then semantic then intent then adapt --target isf, require 25 repository-relative page paths and exactly adapter.json with no *.isf, confirm the methodology-guide classification and visual-label-only SourceIR delta, and run the WIRE/KG/FSMGen-strict/locality gates recorded in CORPUS-COVERAGE.2.37."
---

**Established `2026-08-09` (`CORPUS-COVERAGE.2.37`).** The selected same-SSD PDF hashes to
`303504a884d0aa11802920259eb674ee23619b96eaf5fd0c3fc3e1789db38379`; the release binary hashes to
`6e8add7b7497002662afb25d74a4699bbb910f4ed66d933c850e98f1bc22d7ec`. A guarded CPU ingest produces 25
pages, 11 visuals, two tables, 36 sections, 266 content elements, and zero SourceIR residuals. The lowest sampled
system memory free was 81%, so peak sampled system use was 19%. All 25 page sidecars store their final
repository-relative PNG paths, with no staging or off-volume path residue.

The retained SourceIR had 347 elements. The 81 removed records are all `body_text` on pages 7, 8, 10, 11, 13,
18, and 20, each of which contains visual assets. Captions, list items, section headers, pages, visuals, tables,
and sections hold exactly. The longest removals are flattened labels from host/debug-probe/target diagrams, so
the delta is visual-label cleanup rather than structural or narrative loss. It also explains the current actor
count dropping six→five: the removed `host` was derived from diagram-label text.

EvidenceIR contains 36 section anchors, 243 spans, 11 visual records, seven links, 243 statements, five
conditional facts, and two normative facts, with zero actor-signal relations, signal declarations, registers,
or timing records. Validation classifies the self-declared document as a high-confidence methodology guide. This
contrasts with the preceding CoreSight Base System Architecture, whose low typed yield is explicitly
under-extracted: no equivalent architecture/specification warning applies here.

The stale SemanticIR/IntentIR had turned authority-empty evidence into 64 heuristic interfaces, and the adapter
rendered 73 one-bit signals with no rules, transactions, enums, storage, or connectivity. Current generic
authority removes those interfaces while retaining five actors, 22 phases, 54 invariants, 56 gates, two
assertions, 28 decomposition candidates, five conditionals, 78 behaviors, 55 constraints, and one assumption.
Lowering blocks on `no signals declared in interface`, removes the obsolete `agent.isf`, and leaves exactly
`adapter.json` with zero residuals.

Two complete deterministic cascades reproduce all final hashes: SourceIR/report
`f4abaafb0a858d4e3606f4e35d35e665697c9a2b912579eef328b92a38a16bde` /
`e27bd9b4bc057ccaef56da211627b745789f8f162926eccf5c018653e25ad7d9`; EvidenceIR/report
`de14fd80204e93715b0d7e75688f923b733cdbab33045c0d2e1cff7ea6e95571` /
`5ca8fc822d8dfa6b31817d3ed661a240a25e01d0de1759cb7fa3f32721e39148`; SemanticIR/report
`c59ed56761f4ea49c7a519af56103978b8e5bf7539896321a763aaadc0f87e8a` /
`b116788a32ad436b21aed2b12342c96b62afcfbfad9d0f6d4ae165d4d3471169`; IntentIR/report
`25013a40429e8feb326244d544056e7555dbb6e076d8e6f5b11cc175f57ef8f4` /
`b05ef9c062662c44573760b05c26731c1ce2cfb885c8c58cb924d400ca97b1e6`; adapter
`64b29c4a28287a3eb91c9c6ece454caec2784325c758c7f131ccc77397d4f278`.

All nine provider-free WIRE/I2C/SWD datasets hold their declared gates, including the documented promotion-only
SWD constraint miss. KG is 156/156, all 66 remaining emitted ISFs pass FSMGen strict, and persisted-path and
project-data locality checks pass. Corpus refresh is 37 done / 19 remaining, with stage census 80 SourceIR / six
normalized / 80 EvidenceIR / 79 downstream chains.
