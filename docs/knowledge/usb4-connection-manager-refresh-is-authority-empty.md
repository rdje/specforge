---
id: usb4-connection-manager-refresh-is-authority-empty
title: USB4 Connection Manager refresh is portable and honestly non-emitting
answers:
  - "is the USB4 Connection Manager current-binary refresh complete"
  - "why does the USB4 Connection Manager Guide emit no ISF"
  - "what happened to the USB4 Connection Manager device_also.isf"
  - "did SB USB and USB4 survive as USB4 Connection Manager interface signals"
  - "how many USB4 Connection Manager page sidecars are repository relative"
  - "what are the current USB4 Connection Manager artifact hashes"
  - "how much memory did the guarded USB4 Connection Manager ingest use"
  - "how many corpus refreshes remain after USB4 Connection Manager"
date: 2026-08-09
status: current
tags: [usb4, corpus-coverage, methodology-guide, signal-authority, path-portability, isf]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.35); generated/source_ir/usb4_connection_manager_guide_v2_0_2025_11; generated/evidence_ir/usb4_connection_manager_guide_v2_0_2025_11; generated/semantic_ir/usb4_connection_manager_guide_v2_0_2025_11; generated/intent_ir/usb4_connection_manager_guide_v2_0_2025_11; generated/adapters/isf/usb4_connection_manager_guide_v2_0_2025_11
reverify: "Hash the same-SSD PDF reached through .cache/local-references/chipdoc, validate the retained SourceIR/EvidenceIR/SemanticIR/IntentIR, rerun evidence then semantic then intent then adapt --target isf, require 96 repository-relative page paths, exactly adapter.json with no *.isf, and run the WIRE/KG/FSMGen-strict/locality gates recorded in CORPUS-COVERAGE.2.35."
---

**Established `2026-08-09` (`CORPUS-COVERAGE.2.35`).** The selected same-SSD PDF hashes to
`09f4441915eaeb421ed5314156534d61a350f8a5b8eb8a9c63f65b7fbd251d42`. A guarded CPU ingest produces 96
pages, 46 visuals, 23 tables, 1,313 content elements, and zero SourceIR residuals. The lowest sampled system
memory free was 72%, so peak system use was 28%. All 96 page sidecars store their final repository-relative PNG
paths; the complete 252-file source-through-adapter chain has no staging, boot-volume, or generic-temp residue.

The stale chain had 1,509 SourceIR elements and a renderable `device_also.isf`: 13 actor-signal relations became
four interfaces, 11 actor ports, three one-bit outputs (`SB`, `USB`, `USB4`), three rules targeting `USB4`, and
one generic `TABLE` enum. Current extraction keeps four exact `USB4` references in EvidenceIR—one signal
constraint and three conditional consequents—but produces zero actor-signal relations. SemanticIR and IntentIR
therefore have zero interfaces, ports, relations, or connectivity. Validation classifies the self-declared guide
as a high-confidence methodology guide and retains two recognition-only operations without fabricating a signal
boundary. Lowering blocks on `no signals declared in interface`, has zero signals/transactions/rules/enums/storage,
records four residuals, removes `device_also.isf`, and leaves exactly `adapter.json`.

The SourceIR element delta is isolated to body text: body records fall 639→443, while 683 list items, 37
captions, seven footnotes, 143 section headers, 96 pages, 46 visuals, and 23 tables hold. Twelve pages account for
197 removed body fragments and all twelve carry visual assets; one clean body record is gained on another page.
The normalized markdown retains the substantive prose in corrected form. This is current visual-text cleanup,
not evidence loss. Two complete downstream replays are byte-identical.

Final SourceIR/report hashes are `89d3b64c8f69faec7c77d2365cba3d19ac5502e8581a6fcf02c00f20ae193378` /
`d4c3759bc86d35a94f614eebde2bdd5ab3fbbb194fcbb9321723c0794e6efbb2`; EvidenceIR/report
`8f84a51a9631571ece93559df73bd1ba2993f6dd06ff9829061276d44cc9b008` /
`2a745b71bcd00e6becdac03d3fa0429ba527723b1fd473c06526957efaebf992`; SemanticIR/report
`1554907944c2aff181d1a95aa888fa76a6c7af06bb70159224f10f0d78147f03` /
`33fce6c8842be0f136f0d5465505f1b41a037805508b820099373fc52f650d6c`; IntentIR/report
`d1da6c3d7681e998e3e17f1ddb45cd430b36ead58b51c637d6ad428a63c2f62a` /
`79854d6063ec91f0d75dc5061172881c491a4554f8eaab1ca7755a267af84f11`; adapter
`453a3f4ef01029b354ec5003024c1cc7d52d6fa53a240b28431ef4dd9dc7c82e`.

All nine provider-free WIRE/I2C/SWD datasets hold their declared 1.000 gates, including the documented
promotion-only SWD constraint miss. KG is 156/156, all 68 remaining emitted ISFs pass FSMGen strict, mdBook and
project-data locality pass, and the downstream replay is byte-exact. Only then were the exact rollback and
first-run replay deleted; the task-id residue census is zero. Corpus refresh is 35 done / 21 remaining, with stage
census 80 SourceIR / 4 normalized / 80 EvidenceIR / 79 downstream chains.
