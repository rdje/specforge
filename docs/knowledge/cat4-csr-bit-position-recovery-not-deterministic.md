---
id: cat4-csr-bit-position-recovery-not-deterministic
title: Cat-4 RISC-V CSR bit-position recovery (DOC-INTENT-TAXONOMY.4d.i) — NOT deterministically recoverable from the Docling text; the bits live in the layout IMAGE (VLM-accuracy-bound), the few flattened tables are garbled/XLEN-symbolic; honest residual, no code, no FR
answers:
  - "is RISC-V Debug register bit-position recovery a deterministic text-table parse or a VLM-image read (VLM-image — 53/56 diagrams are images, the 7 flattened tables are garbled/symbolic; deterministic parse would fabricate)"
  - "did DOC-INTENT-TAXONOMY.4d.i build a deterministic RISC-V CSR bit-position parser (NO — measured non-viable: bits live in the image, flattened tables garbled/XLEN-symbolic, ~0 correct recovery + fabrication risk; honest residual, no Rust code, no FR)"
  - "why was the .4d.i pre-investigation 'deterministically tractable' verdict overturned (gold check: dmstatus flattened table off-by-8 + dropped 7-field band; dmcontrol image-only no table; tdata1 symbolic XLEN-relative positions)"
  - "can the Docling-flattened register diagram table be parsed into bits_high/bits_low (no — garbled: wrong explicit positions, dropped field bands, doubled cells, two stacked halves, or symbolic XLEN-relative positions)"
  - "how many RISC-V Debug register bit diagrams are images vs flattened tables (53/56 images, 34 field tables, only 7 flattened diagram tables; bits live in the image modality)"
  - "do the register_bits.rs tiling gates validate field order (NO — only width-sum + name-multiset; a row-jumbled flattened table could pass both gates with WRONG bits, so a deterministic-table reader is strictly more dangerous than the VLM front-end)"
  - "what is the genuine lever for RISC-V CSR bit recall (a sharper VLM read for the existing recover-register-bits / register_bits.rs path — stronger/cloud model, upscaling, voting, tighter prompt — owned OUTSIDE the .4 ISF-lowering program; bound purely by VLM accuracy)"
  - "why can't RISC-V AIA registers be captured by .4d.i (its normalized bundle is ABSENT — re-ingest RAM/Docling-gated under CORPUS-COVERAGE — and its CSR intent is prose conditional_rules, not register tables)"
date: 2026-06-23
tags: [doc-intent-taxonomy, cat-4, cpu-isa, csr, register, bit-layout, vlm, extraction-recall, honest-residual, fabrication-guardrail, verify-fsmgen-before-fr, isf-no-hacks, adr-0006, measured, decision-packet]
evidence: scripts/measure_cat4_csr_bit_recovery.py; generated/source_ir/1_0_risc_v_debug_specification/normalized/1_0_risc_v_debug_specification.md (dmstatus ~L1042 flattened table garbled; dmcontrol ~L1085 image-only; tdata1 symbolic XLEN-relative); crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json (dmstatus/dmcontrol bit gold); crates/specforge/src/ir/register_bits.rs (the VLM+tiling gates — order-blind); docs/research/cat4-csr-bit-position-recovery-measurement.md
reverify: "python3 scripts/measure_cat4_csr_bit_recovery.py  -> 56 register-with-address headings / 53 image diagrams / 7 flattened tables (dmcs2,dmstatus,dscratch1,tdata1,textra32,tinfo,tmexttrigger); dmcontrol image-only; dmstatus flattened table off-by-8 (ndmresetpending shown 16, gold 24) + middle band dropped + 13 vs 20 fields; tdata1 symbolic XLEN-relative; AIA bundle ABSENT / 0 register_records / 39 prose conditional_rules. Verdict: deterministic-table lever NOT viable -> honest residual, no code, no FR; genuine lever = sharper VLM read for register_bits.rs (VLM-accuracy-bound). Related: [[register-diagram-bit-recovery-via-tiling]], [[cat4-isa-csr-lowering-decision]], [[register-bit-field-isf-lowering-gap]]."
---

`DOC-INTENT-TAXONOMY.4d.i` was spun out of `.4d` as "the genuine buildable cat-4 lever": recover RISC-V Debug's
`0/179` unlocated register field bit positions **deterministically** from the bit-layout column/diagram (parse
into `bits_high`/`bits_low`, join to the field-description table by name; ADR-0006 diagram-shape grammar) so they
auto-lower via `.4a.ii`, plus a RISC-V-shaped register recogniser for AIA. The `.4d.i` pre-investigation reported
*"deterministically tractable — the [Docling-flattened] table carries the positions."* **Checked per-item against
the human-reviewed bit gold, that verdict is FALSE** (`[[feedback_scoring_rigor]]`); `.4d.i` resolves as a
measurement/decision packet — read-only, docs-only, **no Rust code** — concluding the deterministic-table lever
is not viable and the bit recall stays an honest residual.

**Why it does not work (gold-checked, reproducer `scripts/measure_cat4_csr_bit_recovery.py`):**

1. **The bits live in the IMAGE, not in text.** 53 of 56 register-with-address headings carry an `![Image]`
   bit-layout diagram; only 7 were flattened by Docling into a text table. The fact genuinely lives in a
   non-text modality — which is exactly why `EXTRACTION-GAP-FIX.4` built a **VLM** reader for it
   (`[[register-diagram-bit-recovery-via-tiling]]`). The cleanest gold register `dmcontrol` (14 fields, no
   reserved gaps) is captured as `![Image]` **only** — there is no table to parse.

2. **The few flattened tables are garbled or symbolic.** `dmstatus`'s flattened table has explicit high-bit
   positions **off by ~8** (it places `ndmresetpending` at bit 16, gold 24), **drops** the 7-field middle band
   (`allresumeack`(17)…`allrunning`(11)), and mixes doubled cells + two stacked half-rows whose upper half
   needs width-tiling while the lower half needs explicit positions — 13 diagram fields vs 20 field-table
   fields, so the name-multiset gate yields a residual. `tdata1`'s table carries **symbolic XLEN-relative**
   positions (`XLEN-1`, `XLEN-5`); RISC-V CSRs are XLEN-parameterized, so the bits are not concrete integers.

3. **The tiling gates are order-blind.** `register_bits.rs` gates on width-sum + name-multiset but reconstructs
   from order + widths and does **not** validate the order. The VLM reads fields in visual diagram order; a
   row-jumbled Docling table could present a *wrong* order whose widths still sum to 32 and whose names still
   match — passing both gates while emitting **wrong** bits. So the flattened-table path is *strictly more
   dangerous* than the VLM front-end, while modifying the shared register path `.4a.ii` (24 docs / 6,570 fields)
   relies on — high regression risk for ~0 gain.

4. **AIA is blocked + prose-bound.** RISC-V AIA's normalized bundle is **ABSENT** (re-ingest RAM/Docling-gated
   under `CORPUS-COVERAGE`); its IntentIR has 0 `register_records` and its IMSIC/APLIC CSR intent sits in 39
   prose `conditional_rules`. A register recogniser can neither be built nor tested now, and the intent is prose.

**Decision:** no Rust code, no FSMGen FR (ISF already expresses register fields via `.4a.ii`; the gap is recall
and a speculative parser would fabricate — `[[feedback_verify_fsmgen_before_fr]]` / `[[feedback_isf_no_hacks]]`).
The genuine lever is a **sharper VLM read** for the existing gated path (`recover-register-bits`), bound purely
by VLM accuracy, **owned OUTSIDE the `.4` ISF-lowering program**; AIA waits on `CORPUS-COVERAGE` re-ingest +
upstream prose extraction. Honest residual, recorded as a cross-reference — mirrors `[[cat4-isa-csr-lowering-decision]]`
(`.4d`) and `.4c.i` (cat-3 topology, also capture-recall-gated). With `.4d.i` resolved, the
`DOC-INTENT-TAXONOMY.4` actionable frontier is exhausted except the FSMGen-gated `.4b`.
