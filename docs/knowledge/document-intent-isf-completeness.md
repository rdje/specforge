---
id: document-intent-isf-completeness
title: Per-category ISF-lowering completeness (DOC-INTENT-TAXONOMY.2) — registers lower 1:1 to opaque width-only storage but their 12,638 bit-fields reach .isf ZERO times (Gap A), and 1,220 message-field structures never cross EvidenceIR→IntentIR (Gap B); cat1 MATURE, cat2/3 PARTIAL, cat4 THIN, cat5/6 honest non-targets (5 cat-6 guides over-extract)
answers:
  - "what fraction of a chip-spec PDF's intent reaches the emitted .isf, per purpose category"
  - "where do message-field / structure / flit / descriptor layouts go in the pipeline (EvidenceIR only — no IntentIR carrier, never lowered)"
  - "what are the two dominant ISF-lowering completeness gaps (register bit-fields, message-field structures)"
  - "which document categories are mature vs partial vs thin for ISF synthesis (cat1 mature, cat2/3 partial, cat4 thin, cat5/6 non-target)"
  - "how complete is register-IP / platform-IP / CPU-ISA ISF lowering"
  - "do transactions lower to .isf (only those with composed steps; signal-set/channel/phase membership is recognised-but-unlowered)"
  - "why do conditional_rules lower to .isf only partially"
  - "which guides over-extract spurious .isf wire intent (cortex-a76 sw-opt 537 signals, readme, smmu software guide, gic overview, aarch64 debug guide)"
  - "what ISF abstractions does FSMGen need next (field-structured storage / register-with-fields, packet/structure layouts, topology)"
  - "what is the DOC-INTENT-TAXONOMY.2 per-category ISF-completeness gauge"
date: 2026-06-22
tags: [doc-intent-taxonomy, isf-completeness, isf-lowering, register-fields, message-fields, structures, transactions, measured, read-only, adr-0006, fsmgen-feedback, scorecard]
evidence: docs/research/document-intent-isf-completeness.md (full gauge); scripts/measure_isf_completeness.py (reproducer); docs/tasks/DOC-INTENT-TAXONOMY.md (.2 leaf); docs/book/src/document-categories.md (scorecard table); generated/adapters/isf/*/adapter.json (.isf counts); generated/{evidence,intent}_ir/* (surfaces)
reverify: "python3 scripts/measure_isf_completeness.py --extra-adapter 1_0_risc_v_debug_specification=<dryrun> --extra-adapter den0068_2018_07_23_coresight_base_system_architecture=<dryrun>; read-only over generated/{evidence_ir,intent_ir}/*/*.json + adapters/isf/*/adapter.json; key facts: sum(register_records[].fields)=12,638 across 32 docs with isf.storage lowered 1:1 but isf field count 0 (registers emit (var (width N)) only); evidence message_field_records=1,220 across 11 docs while intent_ir has no message_field_records key (has_msgfld_key=false) so 0 carried/0 lowered; cat1 wire mature (WIRE-BASED-100=1.000), cat5/6 honest non-targets with 5 cat-6 guides over-extracting (cortex-a76 sw-opt .isf 537 signals). No validate/adapt write -> zero mutation."
---

**Measured `2026-06-22` (`DOC-INTENT-TAXONOMY.2`, read-only over the 78 persisted IR + 76 `adapter.json` + 2
read-only `adapt --dry-run` for the unmaterialized adapters; no `validate`/`adapt` write → zero mutation.)**

`.isf` is the synthesis target for all 6 purpose categories. The honest gauge measures lowering **per typed
surface** (a single blended % would be gameable — `[[feedback_scoring_rigor]]`). Two surfaces are recovered in
volume yet reach `.isf` **zero** times across every buildable category — these dominate the scorecard:

- **Gap A — register bit-fields.** `3,449` registers lower 1:1 to `(storage (var … (width N)))`, but their
  **`12,638` constituent bit-fields lower 0 times** (registers emit an opaque width-only var; no field
  substructure). Largest absolute loss is cat 3 platform TRMs (CoreSight SoC-600: 833 registers / 2,978 fields
  → storage 833, fields 0). The register *programming model* — the intent of a register IP — does not
  synthesize.
- **Gap B — message-field structures.** `1,220` flit/packet/descriptor/queue/context fields are recovered into
  EvidenceIR (`.10b`–`.10g`) but **IntentIR has no `message_field_records` key** (`has_msgfld_key=false`), so
  0 are carried and 0 lowered. This is the cat-2 structure frontier (NVMe 216, AMD-IOMMU 217) **and** the cat-1
  message-heavy-protocol gap (CHI 106, DTI 159, CHI-C2C 210, CCIX ×4 ≈ 309) — the flit fields ARE the intent of
  those coherent protocols.

Both point at the **same missing ISF abstraction** the owner anticipated: field-structured storage
(register-with-fields) and packet/structure layouts — the "memory banks / single-dual-port memory" family
FSMGen is adding → the headline `.4+` FSMGen FR candidates, filed only after empirical submodule verification
(`[[feedback_verify_fsmgen_before_fr]]`, `[[feedback_isf_no_hacks]]`).

**Per-category scorecard (objectively measured, turning the `.0`/`[[project_doc_intent_taxonomy]]` estimate into
measurement):**

| Cat | Verdict | What lowers | Principal residual |
|---|---|---|---|
| 1 wire-protocol | **MATURE** | signals, relations→ports, constraints/temporal, enums; registers→storage 1:1 | msg-fields (Gap B, 8 docs), transaction bodies (steps-only), conditional tail |
| 2 register-IP | **PARTIAL** | registers→storage 1:1, enums | bit-fields (Gap A, 1,030), structures (Gap B, 433) |
| 3 platform-IP | **PARTIAL** | registers→storage 1:1 (2,472), enums | bit-fields (Gap A, 9,308 — largest), topology stays hint-level, rules ~11% |
| 4 CPU-ISA | **THIN** | register-shaped surface only | bit-fields, CSR/instruction/privilege/exception model has no ISF construct |
| 5 PHY | non-target ✓ | near-empty `.isf` is correct | — (not behavioral wire intent) |
| 6 guide | non-target ✓ | should be near-empty | 5/14 OVER-EXTRACT spurious wire intent (precision matter for `.3`, not a completeness gap) |

Signals are **not** scored as a present/lowered ratio: the `.isf` signal set derives from a different basis
than the IntentIR `interfaces` array (union of interface + actor-port graph + relation/constraint refs), so it
can be larger (A76 TRM 7→189) or smaller (AXI `ihi0022_h_c` 1026→242); the signal lowering path is mature (it
is the WIRE-BASED-100 = 1.000 core). The rule-lowering shortfall (~41/45/11/22% across cats) is dominated by
`conditional_rules` and is **mixed** (part honest residual, part lever) — flagged for per-item triage, not
assumed a pure gap. Read-only; changed no code/canonical artifact, so all golds + `kg-bench` 156/156 orthogonal.
See `[[document-intent-category-census]]` for the `.1` denominator.
