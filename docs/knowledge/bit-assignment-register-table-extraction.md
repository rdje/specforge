---
id: bit-assignment-register-table-extraction
title: Three-column bits|name|function tables split by CAPTION evidence — register-captioned chains become caption-named registers, structure captions become message fields
answers:
  - "how are bits | name | function tables extracted"
  - "how are TRM register bit assignments tables without access/reset columns handled"
  - "when does a caption ground a register name"
  - "how are GICD_CHIPR<n> / TCU_NODE_CTRL n array registers named"
  - "why was nearest-heading anchoring rejected for capless table adoption"
  - "what happens to a register-worded caption that grounds no identifier"
  - "how do Continued from previous page fragments find their home"
  - "why does a sentence-period caption label yield nothing"
date: 2026-06-10
tags: [registers, message-fields, tables, bit-position, captions, chains, digestion, trm-class]
evidence: crates/specforge/src/ir/evidence.rs (bit_assignment_layout_header, bit_assignment_register_name, bit_assignment_register_ident, collect_bit_layout_tables, stitch_bit_layout_chains, extract_bit_assignment_registers); docs/tasks/PDF-VARIANT-DIGESTION.md (.10c)
reverify: cargo test -p specforge --lib bit_assignment -- --nocapture ; plus old-vs-new `evidence --dry-run` parity over generated/source_ir/*/ and the stub-copy protocol for GIC-600/MMU-700/TMC/VT-d
---

`PDF-VARIANT-DIGESTION.10c` (2026-06-10). The three-column `bits | name |
function/description/meaning` family (270 `unknown`-kind tables / 11 docs; GIC-600 71,
MMU-700 66, TMC ddi0461 55, VT-d 39, SDC-600 16…) carries NO access/reset vocabulary, so
`is_register_field_header` can never claim it. The typed home is decided PER TABLE by the
document's own caption vocabulary, on the shared `collect_bit_layout_tables` +
`stitch_bit_layout_chains` machinery (one collector for the `.10b` 2-col and `.10c` 3-col
shapes, so chain adjacency always tests the true document predecessor):

- **Register-grounded captions** → the register surface, third strategy
  `registers.bit_assignment_table`, ONE caption-named `RegisterRecord` per chain
  (`bit_assignment_register_name`): an identifier before the word `register`
  (`TCU_CTRL register bit descriptions`) or a SINGLE identifier heading `bit
  assignments|descriptions|fields` (`GICD_CTLR bit assignments`); array forms
  `GICD_CHIPR<n>` (extended ident charset `_<>*`) and space-`n` (`TCU_NODE_CTRL n`,
  verbatim) included. A plain Titlecase head (`Reservation Register` — an NVMe COMMAND
  name) is prose, never a register. A register-WORDED caption that grounds no ident is
  register-shaped-but-unnamed → residual on BOTH surfaces (never re-housed as a structure
  container — the leak class found live on MMU-700). Access/reset stay honestly absent;
  `-`/empty name cells keep bit-range residual names; `(continued)` re-starts consolidate
  via the `.4c` same-name fragment merge.
- **Any other caption** → the `.10b` message-field strategy with an explicit name column;
  colon-fused name cells (`CTP: Context-table Pointer`, VT-d) yield the leading mnemonic
  (`R: Reserved` skips). `LPI Configuration table entry bit assignments` (multi-word head)
  correctly lands here — entries are structures.
- **Measured caption gates**: a label ending in a sentence period is caption BLEED (2 of
  314 family captions, both GIC-400 prose) → no label; `Continued from previous page` is no
  label (1 of 314) → the fragment chains bit-exactly to its true home (how `C2C_Prop*Tx1`
  reached 22 fields).
- **Heading-anchoring REJECTED by measurement**: same-nearest-heading adoption of capless
  tables would add 1 safe table corpus-wide vs 15 bit-OVERLAPPING wrong ones (page-granular
  headings mis-assign when registers share pages). Bit-exact adjacency remains the only
  caption-free joiner.

Live (stub protocol): GIC-600 15→33 regs (+18/79 fields), MMU-700 13→63 (+50/180), TMC
2→30 (+28/87), SDC-600 0→5, GIC-400 +1, C2C-b +5 property registers; VT-d +15 structure
fields. Parity: 12 intact docs manifest-only; NVMe's 216 `.10b` fields byte-identical
through the refactor. Residuals: 68+ capless no-label chains (re-ingest lever), 15
heading-overlap tables, `RES0` kept (the document's own name). GOTCHA: `cargo test` does
NOT rebuild the bin — copy a fresh `cargo build` binary before live measurement. Related:
[[bit-position-structure-field-extraction]], [[bit-location-register-field-vocabulary]],
[[corpus-register-table-shape-gap]].
