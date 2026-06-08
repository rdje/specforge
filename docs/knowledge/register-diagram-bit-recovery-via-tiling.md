---
id: register-diagram-bit-recovery-via-tiling
title: Register bit positions from a diagram image — reconstruct from VLM-read order+widths via tiling, not the VLM's absolute positions
answers:
  - "how to recover register field bit positions that live in the layout graphic, not the table"
  - "why does the VLM misread register-diagram bit positions and how is it fixed"
  - "is a better VLM needed to read register bit-layout diagrams"
  - "how does the tiling gate keep register-bit recovery honest (no fabrication)"
  - "why is the Docling table capture of a register diagram unreliable"
  - "what is recover-register-bits / how does the recover-register-bits command work"
  - "where is the tiling-gated register bit recovery implemented"
  - "are the register-bit-recovery plumbing gaps (unknown diagrams, fragmented field tables) fixed"
  - "why does recover-register-bits still recover 0 bits after the plumbing is complete"
date: 2026-06-08
tags: [registers, vlm, bit-layout, pdf-variant-digestion, extraction-gap-fix, honesty-guardrail]
evidence: crates/specforge/src/ir/register_bits.rs; crates/specforge/src/commands/recover_register_bits.rs; docs/tasks/EXTRACTION-GAP-FIX.md; crates/specforge/test_data/llm_eval/seed_riscv_debug_registers.json
reverify: cargo test -p specforge --lib register_bits
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

**Implemented (`EXTRACTION-GAP-FIX.4a`, built `2026-06-08`):** pure core `ir/register_bits.rs`
(`reconstruct_bits_by_tiling` + the two gates `proposal_names_match_fields`/standard-width +
`recover_bits_for_register`) and the `recover-register-bits <evidence-ir>` command
(`commands/recover_register_bits.rs`) — EvidenceIR-in/out, reads `(name,width)` off the register's
`RegisterBitfield` diagram via the shared VLM image transport + the `SPECFORGE_VLM_HELPER` hermetic hook,
default `--vlm-provider skip` no-op. Hermetic tests on the real dmcontrol (14/14)/dmstatus (residual)/
name-mismatch (residual) data; the live VLM is never a CI dependency. General/agnostic (ADR 0006): only the
structural "fields tile a register" law + universal register widths, no chip names in the runtime (see
[[no-hardcoded-chip-vocabulary]]). Live bit-extent measurement on the gold is the follow-on leaf `.4b`. See
[[register-field-table-extraction]].

**Live measurement (`.4b`, `2026-06-08`):** ran it live (`qwen2.5vl:7b`) on the re-ingested RISC-V Debug
evidence → all 60 register records `residual_no_diagram`, bit-extent UNCHANGED at 0/179, zero fabrication.
Direct VLM probes showed the local model reads field names+order correctly but mis-sizes cells (dmcontrol gets a
spurious width-5 reserved → sum 37 ≠ 32; dmstatus off-by-one → 33 ≠ 32) → the standard-width gate rejects every
slip → honest residual. So the tiling math is proven (hermetic 14/14 on clean widths) and the guardrail catches
every real VLM error, but the metric stays flat because the local read isn't clean enough on these dense
diagrams AND two upstream plumbing gaps block auto-resolution: the diagrams are ingest-classified `unknown` (not
`RegisterBitfield`) and a register's field table is fragmented across several Docling tables. Closing the metric
is the proposed follow-up (`.4c`): a stronger/sharper VLM read + those two plumbing fixes.

**Both plumbing gaps NOW closed (`.4c` + `.4d`, `2026-06-08`):** `.4c` `consolidate_register_field_fragments`
merges a register's Docling-split field tables into one record (a conservative all-field-names-distinct gate;
[[register-field-table-defragmentation]]), so gate (b) has the full field set; `.4d`
`resolve_diagram_image_for_register` now uses the unique image on a register's page regardless of `diagram_kind`
(>1 image → residual, never guess), so register diagrams left `unknown` are found. The live RISC-V Debug run
(`qwen2.5vl:7b`, all 44 de-fragmented registers) still recovers **0 / residuals 44 / zero fabrication**, but
several registers now REACH the VLM and gate honestly (`residual_non_standard_width` for `hartinfo`/`sbcs`/`dpc`/
`textra64`; `residual_name_mismatch`/`no_proposals` for `mcontrol`) instead of `.4b`'s uniform
`residual_no_diagram` — the guardrail is validated **end-to-end** through the resolver, not only via direct probes.
**So the `EXTRACTION-GAP-FIX.4` bit-recovery machinery is now COMPLETE and the metric boundary is PURELY VLM
accuracy:** the local `qwen2.5vl:7b` is not accurate enough on these dense diagrams, and a stronger VLM (larger/
cloud model, voting, image upscaling, sharper prompt) is the sole remaining lever — recovery still fabricates
nothing.
