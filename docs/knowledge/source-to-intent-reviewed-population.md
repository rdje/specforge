---
id: source-to-intent-reviewed-population
title: The first source-to-IntentIR vertical population locks twelve reviewed documents without extractor tuning
answers:
  - "which documents are in the first source-to-IntentIR vertical evaluation population"
  - "how many reviewed documents are locked per source category"
  - "was the first vertical population historically unseen"
  - "how many source-to-intent evaluation inputs are repository sources versus external read-only sources"
  - "where is the reviewed source-to-intent dataset"
  - "does the reviewed population publish product support results"
  - "which source modalities are represented in the first vertical population"
date: 2026-08-11
status: current
tags: [spec-to-intent-alignment, evaluation, reviewed-gold, portability, selection-boundary]
evidence: crates/specforge/test_data/source_to_intent_vertical/reviewed_dataset.json; crates/specforge/test_data/source_to_intent_vertical/build_fixture.py; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.4b)
reverify: "python3 -B crates/specforge/test_data/source_to_intent_vertical/build_fixture.py --check && cargo test -p specforge --lib ir::source_to_intent_eval::tests::reviewed_population_is_complete_balanced_portable_and_deterministic"
---

`SPEC-TO-INTENT-ALIGNMENT.4b` freezes exactly twelve documents: two in each of the six dominant-purpose
categories. Four PDFs live in the repository; eight are necessary caller-authorized read-only external inputs
represented only by portable id, digest, and necessity. The 14 reviewed cells cover prose, tables, and figures;
the two physical-link documents each contribute separate digital and analog-disposition cells.

| Category | Reviewed documents |
| --- | --- |
| wire protocol | Arm AMBA APB; NXP I2S |
| register IP | Arm GIC-400 TRM; AMD IOMMU |
| platform/system IP | Arm CoreSight Base System Architecture; RISC-V IOMMU |
| CPU/ISA | RISC-V Advanced Interrupt Architecture; Arm Debug Interface v6 |
| physical/link | OpenCAPI 25 Gbps PHY Signaling; OpenCAPI 32G PHY Signaling |
| methodology/guide | Cortex-A76 Software Optimization Guide; Generic Interrupt Controller Overview Guide |

The selection boundary is commit `a3e9757d63ca5499a2393864fb503d6537de0035`, before gold construction and
before extractor changes. This is a retrospective corpus baseline, not a claim that the documents were
historically unseen. Three SourceIR hashes were re-pinned after correcting only their obsolete external path
metadata to the verified SSD source; the PDF bytes and all downstream stage bytes were unchanged.

Each document pins the source identity and full SourceIR, EvidenceIR, SemanticIR, and IntentIR hashes, then
stores only a bounded reviewed projection. Every cell states exhaustive gold for its explicit source region,
modality, semantic family, provenance, and canonical/residual/non-applicable disposition. The deterministic
builder refuses drift in source bytes, four-stage bytes, reviewed regions, or evidence capture.

`.4b` locks review authority and proves the population is complete, balanced, portable, measurable, and
deterministic. It deliberately does not publish category status or exact product scores. Those conclusions
belong to `.4c`; evaluator semantics remain in `[[source-to-intent-vertical-evaluator]]`.
