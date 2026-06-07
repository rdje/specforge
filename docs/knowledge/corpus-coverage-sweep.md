---
id: corpus-coverage-sweep
title: Whole-corpus coverage sweep (2026-06-08) — Lever A+B uplift measured across the 82-PDF library
answers:
  - "how much intent does SpecForge extract across the whole corpus"
  - "what is the PDF-VARIANT-DIGESTION whole-corpus coverage / re-triage"
  - "which corpus docs still yield nothing (the VLM frontier)"
  - "which docs fail to ingest (giants / timeouts)"
date: 2026-06-08
tags: [corpus, coverage, triage, pdf-variant-digestion, measurement]
evidence: docs/corpus_coverage_2026-06-08.md (per-doc matrix); /tmp/corpus_coverage.tsv (raw)
reverify: "python3 over generated/evidence_ir/*/evidence_ir.json — count Signal-decls / register_records+fields / protocol_actors / signal_constraints / actor_signal_relations per doc-key"
---

**ISA / instruction-set specs are OUT OF SCOPE** (owner `2026-06-08`: chip design uses hardware interface/register/protocol specs, not the software programming model — the 7 ISA-class PDFs are excluded; effective corpus ~75; their generated artifacts were removed; most of the 5 ingest-timeouts were these giant ISA manuals → moot).

PDF-VARIANT-DIGESTION.1 re-triage: ran the DETERMINISTIC pipeline (ingest → evidence, no VLM; 480s/doc
timeout, `normalized` reclaimed per doc) across the 82-PDF library to measure the Lever A+B uplift.

**Status:** 75 OK (+ Wishbone, which a stats-script glitch mislabelled — it ingested fine → ~76 effective);
**5 INGEST-TIMEOUT** (the giant combined-volume ISA manuals: ARM A32/T32, ARM A64, ARM registers, Intel SDM,
USB4 v2.0 spec — need a longer ingest budget / chunking); **1 EVID-FAIL** (USB 3.2 — evidence build bug to
investigate).

**Aggregate over OK docs (massive uplift vs the original triage, where most non-AMBA docs were 0):**
62 docs yield signals, 34 registers, 16 prose actors, 33 constraints, 49 relations — totals **1,908 signals
/ 2,953 registers / 10,632 register fields / 612 constraints / 3,077 relations**. Per-doc matrix:
`docs/corpus_coverage_2026-06-08.md`.

**#3 (prose capture) confirmed across vendors:** CCIX (all 4 versions: was 0 → sig 3–5 / reg 8 / fld 11 /
con 5–6 / rel 4–5), I2C (sig 10 / con 15 / rel 17 / act 2), OpenCAPI (most versions extract), USB4
connection-manager (rel 13) + inter-domain (reg 6), RISC-V privileged (reg 39, act 8), IOMMU (reg 33 / fld
147). NVMe reg 44 / fld 199.

**Remaining frontier:** ~9 OK docs still yield 0 — mostly GUIDES / ISA / overview docs (AArch64 External
Debug Guide, A64 ISA Guide, SMMU Software Guide, GIC Overview, CoreSight Base System Arch, Cortex-A76
Software-Optimization Guide, RISC-V AIA, OpenCAPI PHY-mech/AFU). These are prose-heavy or image-table-heavy —
the VLM classification/grid-repair territory ([[vlm-table-strategy]]). Giants need a longer ingest budget;
USB 3.2 evidence-fail is a bug.

**Method note:** measure per-doc from `generated/evidence_ir/*/` (dirs are keyed by doc-key — identity
preserved) rather than a sweep script that prints the path (the first sweep had a `rel` variable collision —
relations-count vs relpath — that corrupted the doc column; aggregates were still correct).
