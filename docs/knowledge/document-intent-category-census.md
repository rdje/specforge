---
id: document-intent-category-census
title: The 78-doc corpus splits 36/7/15/2/4/14 across the 6 purpose categories (wire-protocol/register-IP/platform-IP/CPU-ISA/PHY/guide); typed-surface counts alone CANNOT separate cat2↔3, recover cat4(ISA), or split cat5↔6, and 8 register-heavy docs are actually protocols (the structural-recognizer trap)
answers:
  - "what are the 6 chip-spec document intent categories / purpose taxonomy"
  - "how is the corpus distributed across the document intent categories"
  - "how many docs are wire-protocol vs register-IP vs platform vs ISA vs PHY vs guide"
  - "why is the structural document_class (protocol/register/interface/guide) too coarse for the purpose taxonomy"
  - "can typed-surface counts alone determine a PDF's purpose category (no — 2 vs 3 not separable, ISA has no signature, 5 vs 6 indistinguishable)"
  - "which protocols look like register IPs structurally (CCIX, AXI, CHI, DTI, CHI-C2C — the register-heavy-protocol trap)"
  - "what does a fast category recognizer need beyond surface counts (wire-relation shape, front-matter/self-declared type, topology cue)"
  - "what is the DOC-INTENT-TAXONOMY.1 corpus census"
date: 2026-06-22
tags: [doc-intent-taxonomy, document-class, category, census, measured, read-only, isf-completeness, adr-0006, recognizer]
evidence: docs/research/document-intent-category-census.md (full census); docs/tasks/DOC-INTENT-TAXONOMY.md (.1 leaf); docs/book/src/document-categories.md (the 6-category table); generated/{evidence,intent}_ir/* (surface counts)
reverify: "python3 read-only over generated/evidence_ir/*/evidence_ir.json + intent_ir/*/intent_ir.json: per doc count actor_signal_relations/signal_constraints/register_records/message_field_records/signal_presence_records/conditional_rules + intent transactions; ground-truth label each into 6 categories -> distribution 36/7/15/2/4/14 over 78; a structural-only bucket rule collapses cat2+3 into reg/struct, splits the 2 ISA docs across prose-only and reg/struct, and merges cat5+6 into near-empty; 8 cat-1 protocols (4x CCIX, AXI ihi0022_l, CHI ihi0050_g, DTI ihi0088, CHI-C2C ihi0098_b) are register/message-dominant."
---

**Measured `2026-06-22` (`DOC-INTENT-TAXONOMY.1`, read-only profile of the 78 persisted docs; no `validate`
run → zero artifact mutation).**

Every chip-spec PDF has a dominant **purpose**; the 6-category taxonomy (`DOC-INTENT-TAXONOMY.0`,
`docs/book/src/document-categories.md`) is the guiding lens. The corpus distribution:

| # | Category | Docs |
|---|---|---:|
| 1 | Wire-level bus / interconnect protocol | 36 |
| 2 | Programmable register / memory-mapped IP | 7 |
| 3 | Platform / system-IP topology & integration | 15 |
| 4 | CPU ISA / privileged architecture | 2 |
| 5 | Physical / electrical / link layer | 4 |
| 6 | Methodology / language / EDA standard / guide | 14 |

**Typed-surface counts alone (the basis of the 4-way `document_class`) cannot recover the purpose category:**
(a) cat 2 ↔ cat 3 are not separable — register/structure-dominant docs span register-IP, platform/system-IP,
register-heavy protocols, and ISA; (b) cat 4 (ISA) has no distinct signature — the 2 ISA docs split across the
`prose-only` (RISC-V AIA: reg 0/cond 39) and `reg/struct` (RISC-V Debug: reg 44/rel 20) buckets; (c) cat 5 ↔
cat 6 share the `near-empty` bucket. **The deepest trap:** 8 cat-1 *protocols* are register/structure-dominant
(all 4 CCIX reg 131–143; AXI `ihi0022_l` reg 71 but rel 348; CHI `ihi0050_g` msg 106; DTI `ihi0088` msg 159;
CHI-C2C `ihi0098_b` reg 81/msg 210) — so "has registers ⇒ register-IP" is wrong; the dominant surface is not
the purpose.

**Implication for the `.3` recognizer (ADR-0006 structural, no name lists):** beyond surface counts it needs
the wire-relation/transaction *shape* (to rescue register-heavy protocols), document front-matter /
self-declared type (`document_type_declared`, PDF-VARIANT-DIGESTION.5c — to split guide/PHY/ISA), and a
topology/component cue (to split platform cat 3 from single-IP register programming model cat 2). For `.2`
(per-category ISF-completeness gauge) the denominator is now fixed (36/7/15/2/4/14); cat 5/6 are honest
non-targets (a thin `.isf` there is correct, not a gap). See [[corpus-coverage-buildout]],
[[stage-staleness-validate-detector]], and the north star [[project_kg_isf_completeness]].
