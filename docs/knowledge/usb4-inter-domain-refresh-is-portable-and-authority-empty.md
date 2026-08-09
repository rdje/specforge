---
id: usb4-inter-domain-refresh-is-portable-and-authority-empty
title: USB4 Inter-Domain refresh is path-portable and authority-empty
answers:
  - "is the USB4 Inter-Domain Service corpus refresh complete"
  - "does the USB4 Inter-Domain adapter still emit a USB4 signal"
  - "how many USB4 Inter-Domain page sidecars are repository relative"
  - "did the page sidecar repair change USB4 semantic artifacts"
  - "what are the current USB4 Inter-Domain artifact hashes"
  - "what happened to the USB4 Inter-Domain channel.isf"
  - "how much memory did the guarded USB4 Inter-Domain ingest use"
  - "where does the SpecForge chipdoc source route currently resolve"
  - "how many corpus refreshes remain after USB4 Inter-Domain"
date: 2026-08-09
status: current
tags: [usb4, corpus-coverage, source-ir, path-portability, signal-authority, isf, project-data-locality]
evidence: docs/tasks/CORPUS-COVERAGE.md (.2.34b.ii.b); generated/source_ir/usb4_inter_domain_service_specification_v2_0_2025_11; generated/evidence_ir/usb4_inter_domain_service_specification_v2_0_2025_11; generated/semantic_ir/usb4_inter_domain_service_specification_v2_0_2025_11; generated/intent_ir/usb4_inter_domain_service_specification_v2_0_2025_11; generated/adapters/isf/usb4_inter_domain_service_specification_v2_0_2025_11
reverify: "Resolve .cache/local-references/chipdoc and compare its filesystem identity with the repository. Hash the selected PDF. Validate the USB4 Inter-Domain source/evidence/semantic/intent artifacts, run adapt --target isf, require 51 repository-relative page paths with no normalized.staging value, exactly adapter.json with no *.isf, and run scripts/check_project_data_locality.sh."
---

**Established `2026-08-09` (`CORPUS-COVERAGE.2.34b.ii.b`).** The director-supplied `chipdoc` checkout and
SpecForge are on the same external SSD filesystem. The ignored `.cache/local-references/chipdoc` symlink remains
a discovery route; ingest uses the resolved, caller-authorized read-only external input because treating a
symlink escape as a repository-owned persisted path correctly fails closed. The selected PDF hash is
`ab337460641c1f012a78c63dcf8cf182bf90deb29a7a841726973be79dcd7396`.

Two guarded CPU ingests produce 51 pages, 82 visual assets, 49 structured tables, 603 content elements, and zero
SourceIR residuals. Guarded monitoring saw system memory stay at least 82% free, so peak system use was 18%.
All 51 `normalized/pages/page-*.json` files store their final repository-relative PNG path. Compared with the
preserved pre-fix run, exactly those 51 sidecars differ; deleting `rendered_image.path` makes every pair equal.
No page record contains `normalized.staging`, the repository absolute root, or a generic temporary root.

SourceIR/report hashes are `eddccfb73b2f8fc93ce792a105eb75e132dc0179e655d26ae0dd016dd63d8eb0` and
`b78e76a26b135010b1a3b15fc867f4e64746d03ec8ffa0ee77c9545f1bd9bf96`. Downstream files are byte-identical
to the preserved fresh chain: EvidenceIR/report `111f4dc348cd082b8e9857911a0921c731e35d0ec8343811c76825a38086398e` /
`5bc59e80454316b1f6172ad9ae910e256307663441bce4a080434f9c69786253`, SemanticIR/report
`422f70f56d17af6dfa3aafd4c766fb4810cf6c80aedd8d1c2a391efda4358a1f` /
`df3bb7aefaae64db66ea6315821512692c0ee685fad99d0eeecec45d079808c7`, IntentIR/report
`144cd98134a6aae47e97092b975dce73e903c2ad4acfe43d232390ebf8dc92a5` /
`9ae4c87d61fb7d1c6d5e6095d2364cac66e5fbe5b554dde64b569c73a049bce4`, and adapter
`1b16b30bb29e438d7a4a679c65750568b91482282c73abf4eb395badcb1d2ded`.

The current generic signal-authority rules remove the stale model rather than special-casing USB4. Evidence
relations fall 1→0; SemanticIR/IntentIR interfaces, ports, relations, and connectivity fall 1→0. The adapter moves
from renderable to blocked: signals 1→0, rules 2→0, enums 8→0, six storage records retained, and one explicit
residual with `no signals declared in interface`. Two exact `USB4` conditional consequents remain in EvidenceIR,
SemanticIR, and IntentIR as source evidence without becoming interface authority. The successful blocked write
removes `channel.isf`, so the adapter directory contains exactly `adapter.json`.

All nine provider-free WIRE/I2C/SWD datasets hold their declared 1.000 gates, KG is 156/156, all 69 remaining
emitted ISFs pass FSMGen strict, and full CI/book/doctrine/locality gates pass. After those checks, the
authenticated seven-file rollback and 198-file defective-chain evidence were deleted; the exact task-id residue
census is zero. Corpus refresh is 34 done, 22 remaining, with stage census 80 SourceIR / 3 normalized / 80
EvidenceIR / 79 downstream chains.
