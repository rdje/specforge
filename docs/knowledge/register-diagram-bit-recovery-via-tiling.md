---
id: register-diagram-bit-recovery-via-tiling
title: Register bit positions from a diagram image — reconstruct from VLM-read order+widths via tiling, not the VLM's absolute positions
answers:
  - "how to recover register field bit positions that live in the layout graphic, not the table"
  - "why does the VLM misread register-diagram bit positions and how is it fixed"
  - "is a better VLM needed to read register bit-layout diagrams"
  - "how does the tiling gate keep register-bit recovery honest (no fabrication)"
  - "why is the Docling table capture of a register diagram unreliable"
date: 2026-06-08
tags: [registers, vlm, bit-layout, pdf-variant-digestion, extraction-gap-fix, honesty-guardrail]
evidence: docs/tasks/EXTRACTION-GAP-FIX.md (.4 investigation); generated/source_ir/1_0_risc_v_debug_specification/normalized/assets/table-0020.png + picture-0020.png; crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json
reverify: "view the diagram images table-0020.png (dmstatus) / picture-0020.png (dmcontrol); probe qwen2.5vl:7b for (name,width) per field; reconstruct ranges by cumulative LSB tiling and compare to seed_riscv_debug_registers.json — dmcontrol = 14/14, dmstatus widths sum != 32 (reserved field misread) -> residual"
---

Some register specs (RISC-V Debug) put each field's **bit position only in the register bit-layout
GRAPHIC**, not in the field-definition table (`Field|Description|Access|Reset` has no bits column). So the
fact lives in a non-text modality — the honesty guardrail says read THAT modality or report a residual,
never fabricate.

**Two wrong paths, ruled out by evidence (vs the gold):**
- *Docling's table capture of the diagram* (`table_0020`) is **garbled**: the low half matches the gold but
  the high half is wrong (`ndmresetpending` captured at bit 16, gold 24) and reserved-bit gaps are dropped.
  Reconstructing from it would synthesize wrong bits → rejected.
- *Trusting the VLM's absolute bit positions* fails too: `qwen2.5vl:7b` reads field **names + order + per-field
  widths reliably**, but misreads the absolute MSB of **wide** fields — a wide cell prints BOTH edge numbers
  (e.g. `25 … 16` for `hartsello`) and the model grabs the wrong one, cascading an off-by-N down the row.
  Confirmed on both dmstatus and dmcontrol.

**The fix is a smarter USE of the VLM, not a bigger VLM:** discard the unreliable absolute positions and
**reconstruct them from order + widths by cumulative LSB tiling** — a register's fields tile it with no gaps,
a structural law the VLM can't violate. Walk fields LSB→MSB, lay each down by its width:
`dmcontrol → 14/14 exact gold` (`hartsello [25:16]`, `hartselhi [15:6]`, `dmactive [0:0]`, …) even though the
VLM's raw positions were wrong.

**A tiling gate keeps it honest:** accept only when the widths tile a standard register width (8/16/32/64/128)
AND the proposed field names match the field-definition table; else the bits stay an honest residual. dmstatus's
wide *reserved* field (width 7) is VLM-misread as width 1 → widths sum to 26 ≠ 32 → **gate rejects → residual**,
never a fabricated bit. A stronger VLM is complementary (it would rescue residual cases by reading the
reserved-field width correctly), not required for the clean diagrams.

Implemented by `EXTRACTION-GAP-FIX.4a` (the tiling-gated recovery). General/agnostic (ADR 0006): the structural
"fields tile a register" law, no chip names. See [[register-field-table-extraction]].
