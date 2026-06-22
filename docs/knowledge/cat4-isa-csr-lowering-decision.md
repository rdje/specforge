---
id: cat4-isa-csr-lowering-decision
title: Cat-4 (CPU ISA) ISF-lowering decision (DOC-INTENT-TAXONOMY.4d) — CSR intent REUSES the existing register/storage abstraction (no new ISF construct); the cat-4 register gap is EXTRACTION RECALL (unlocated fields), not abstraction; instruction/privilege/exception are honest non-targets
answers:
  - "does CPU-ISA (category 4) intent need a new ISF construct or does it map onto the existing register/storage abstraction (CSRs map onto storage/register — no new construct; non-register ISA semantics are honest non-targets)"
  - "why is category 4 (CPU ISA) ISF-thin (the only lowerable cat-4 intent is CSRs, which are registers; their bit-fields are UNLOCATED so they don't reach .isf — an extraction-recall gap, not a missing ISF construct)"
  - "do RISC-V CSRs map onto the ISF register/storage abstraction (YES — RISC-V Debug captures its 44 CSRs as register_records; FSMGen titles (storage (var … (fields …))) the register-map/CSR construct, 13a-actor-interface.md:419/:468)"
  - "why don't RISC-V Debug register bit-fields reach .isf (all 179 fields are UNLOCATED — field_name/access/reset/description captured but 0 carry bits_high/bits_low/bit_width; the field_table strategy did not parse the bit-layout column)"
  - "why does RISC-V AIA capture 0 registers (its IMSIC/APLIC CSR intent is in prose conditional_rules/behaviors; no current register strategy matches RISC-V's CSR layout — the .10g <NAME>, bits [hi:lo] section-heading family fires only on ARM ihiXXXX arch specs)"
  - "is the cat-4 register-field gap an ISF-abstraction gap or an extraction gap (EXTRACTION RECALL — fields are unlocated / AIA registers uncaptured; ISF already expresses register fields via .4a.ii; spun out as .4d.i RISC-V CSR bit-position recovery)"
  - "should SpecForge file an FSMGen FR for CPU-ISA instructions/privilege/exceptions (NO — software-visible ISA semantics are not synthesizable hardware intent; ISF has no construct + FSMGen lists none; honest non-target; conditional-future only if FSMGen's SV/UVM path scopes ISA-model verification)"
  - "what is the buildable category-4 lever (.4d.i — recover RISC-V CSR field bit positions + a RISC-V-shaped register recogniser for AIA; once located, fields auto-lower via .4a.ii, no emitter change)"
  - "which corpus documents are category 4 CPU-ISA (exactly 2: 1_0_risc_v_debug_specification and 1_0_2025_03_12_risc_v_advanced_interrupt_architecture; the RISC-V IOMMU doc is category 2)"
date: 2026-06-23
tags: [doc-intent-taxonomy, cat-4, cpu-isa, csr, register, isf-adapter, extraction-recall, honest-residual, verify-fsmgen-before-fr, isf-no-hacks, adr-0006, measured, decision-packet]
evidence: generated/intent_ir/1_0_risc_v_debug_specification/intent_ir.json (44 register_records / 179 fields / 0 located); generated/intent_ir/1_0_2025_03_12_risc_v_advanced_interrupt_architecture/intent_ir.json (0 register_records; 39 conditional_rules IMSIC/APLIC prose; 211 empty interfaces); subs/fsmgen/docs/book/src/13a-actor-interface.md:419 (storage = register maps/CSRs) /:468 (declarative storage fields) /:511 (future = typed fields/banks/packet layouts, NO instruction construct); crates/specforge/src/ir/isf_ir.rs (register_storage_fields gate: located fields only — DOC-INTENT-TAXONOMY.4a.ii); docs/research/cat4-isa-csr-lowering-decision.md
reverify: "Cat-4 docs: python3 -c to count register_records + located fields over generated/intent_ir/1_0_risc_v_debug_specification (expect 44 regs / 179 fields / 0 located: no field carries bits_high|bits_low|bit_width) and 1_0_2025_03_12_risc_v_advanced_interrupt_architecture (expect 0 register_records; CSR intent in conditional_rules/behaviors). FSMGen: grep -niE 'register map|CSR|instruction|privilege|exception' subs/fsmgen/docs/book/src/13a-actor-interface.md -> (storage) named the register-map/CSR construct; NO instruction/privilege/exception construct. Decision: CSRs reuse storage/register (.4a.ii); gap is extraction recall -> .4d.i; instruction/privilege/exception honest non-targets (no FR — feedback_verify_fsmgen_before_fr / feedback_isf_no_hacks). Docs-only leaf -> golds/kg-bench orthogonal. Related: [[register-bit-field-isf-lowering-gap]], [[document-intent-isf-completeness]], [[document-intent-category-census]]."
---

`DOC-INTENT-TAXONOMY.4d` is the cat-4 (CPU ISA / privileged architecture) ISF-lowering decision packet. It
resolves the standing Open Question — *does cat-4 intent need a new ISF construct, or map onto the existing
register/storage abstraction?* — with measured evidence over the corpus's 2 cat-4 docs and the current FSMGen
pin (`d327129b7`). It is a read-only, docs-only leaf (no Rust code).

**Cat-4 intent splits into three, and only one is a buildable SpecForge lever:**

1. **CSR / register intent → REUSE the existing ISF register/storage abstraction.** CSRs are structurally
   registers: RISC-V Debug captures its 44 Debug-Module CSRs (`dmcontrol`/`dmstatus`/…) as `register_records`,
   and FSMGen explicitly titles `(storage (var … (fields …)))` the "register map / CSR" construct
   (`13a-actor-interface.md:419`/`:468`) — the very construct `.4a.ii` lowers cat-2/cat-3 register fields into.
   **No new ISF construct; no FSMGen FR for CSRs.** See [[register-bit-field-isf-lowering-gap]].

2. **The cat-4 register gap is EXTRACTION RECALL, not abstraction.** RISC-V Debug captures field
   *identity* well (179 fields with name/access/reset/description) but **0/179 are located** — no field carries
   `bits_high`/`bits_low`/`bit_width` (the bit-layout column was not parsed), so under the `.4a.ii`
   located-fields gate **0 reach `.isf`**. RISC-V AIA captures **0 registers** at all (its IMSIC/APLIC CSR
   intent sits in prose `conditional_rules`/`behaviors`; its 211 "interfaces" are empty prose shells — 0
   constraints / 0 relations). No current register strategy matches RISC-V's CSR layout; the `.10g`
   `<NAME>, bits [hi:lo]` section-heading family fires only on ARM `ihiXXXX` architecture specs. → spun out as
   **`.4d.i`: recover RISC-V CSR field bit positions** (+ a RISC-V-shaped register recogniser for AIA); once a
   field is located it auto-lowers via `.4a.ii`, no emitter change. This is the genuine cat-4 build lever.

3. **Instruction / privilege-mode / exception / memory-ordering → honest NON-TARGET.** Software-visible ISA
   semantics, not synthesizable hardware actor/signal/storage/transaction intent; ISF has no construct and
   FSMGen lists none (its future ISF directions are storage/structure abstractions, `13a:511`). Filing an FR
   would breach [[feedback_verify_fsmgen_before_fr]]; forcing them into ISF would fabricate
   ([[feedback_isf_no_hacks]]). Recorded as a **conditional-future**: appropriate only if FSMGen's new SV/UVM
   verification path explicitly scopes ISA-model verification (FSMGen's call). A near-empty `.isf` for the
   non-register ISA semantics is correct, like cat-5 PHY.

Reinforces [[document-intent-isf-completeness]] (the per-category scorecard — cat-4 THIN now explained) and
[[document-intent-category-census]] (the 2 cat-4 docs). Objectively measured, per-item demonstrated
([[feedback_scoring_rigor]]); no fabrication.
