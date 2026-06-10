---
id: corpus-register-table-shape-gap
title: The corpus's biggest structural digestion gap is register-shaped unknown tables (CCIX/AMD-IOMMU class)
answers:
  - "where is the biggest register extraction gap across the corpus"
  - "which header signatures are unrecovered register tables"
  - "why does AMD IOMMU extract no register fields"
  - "why do the CCIX specs extract almost no register fields"
  - "how many unknown-kind tables does the corpus carry"
  - "what is the next big PDF-variant digestion lever after the serial class"
  - "does register extraction require the table_kind register classification"
date: 2026-06-10
tags: [registers, tables, corpus, digestion, probe, table-kind]
evidence: docs/tasks/PDF-VARIANT-DIGESTION.md (the .10 probe table); generated/source_ir/*/source_ir.json (structured_tables.table_kind + header_rows); generated/evidence_ir/*/evidence_ir.json (register_records)
reverify: python3 over generated/source_ir/*/source_ir.json — tally table_kind=='unknown' header signatures (digits→#, lowercase) and cross per-doc register-shaped-unknown count against evidence_ir register_records/fields (the .10 probe script shape)
---

Probe (`2026-06-10`, all 77 persisted SourceIRs): **7,012 `unknown`-kind structured tables**;
the dominant normalized header signatures are register-field variants (~2,000 tables) —
`bits | name | reset | type | description` (580), `bit location | register description |
attributes` (481, the CCIX family ×4), `bits | description` (287, AMD IOMMU class),
`bits | name | function` (129, GIC-600 +4 docs), `bits | access | default | field |
description` (102, VT-d), and kin. All structural header vocabulary — no chip names (ADR 0006).

**Register extraction does NOT require the `table_kind` register classification** — healthy
docs extract richly from unknown-kind tables (CoreSight-0701: 598 register-shaped unknown
tables → 833 registers / 2,978 fields; NVMe 199→42/201; VT-d 152→103/318; RISC-V Debug
59→44/179). The REAL gaps are vocabulary/shape families the register grammar does not yet
read: **AMD IOMMU 167 tables → 0 fields; CCIX ×4 versions ~151 each → 11 fields each;
CoreSight TMC 56→50; GIC-600 72→214 (partial); MMU-700 68→91 (partial)**. Owned as
`PDF-VARIANT-DIGESTION.10` (`.10a` CCIX `bit location` vocabulary ≈600 tables, `.10b` AMD
two-column `bits | description` ≈290, `.10c` `name | function` synonym ≈129) — each
probe-first on its family, recovery only when rows genuinely parse as bit-range + field
semantics, residual otherwise. Related: [[timing-table-trapped-row-recovery]],
[[corpus-coverage-sweep]].
