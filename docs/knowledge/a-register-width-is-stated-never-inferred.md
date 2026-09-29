---
id: a-register-width-is-stated-never-inferred
title: A register's size_bits is only a width the document states; the adapter chooses the narrowest width that holds the fields and lists it (isf_register_width_not_stated)
answers:
  - "why do register records carry no size_bits"
  - "where does an .isf register var width come from when the document states none"
  - "what is isf_register_width_not_stated"
  - "why is ADIv6 ABORT emitted 5 bits wide"
  - "was register_size_from_fields removed"
  - "does any current chain state a register width"
date: 2026-09-29
status: current
tags: [evidence-ir, registers, isf-adapter, residuals, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (the five register readers set size_bits: None); crates/specforge/src/ir/isf_ir.rs (register_var_width, register_width_chosen_residual_packet); crates/specforge/src/ir/completeness.rs (registers_with_unresolved_width); docs/tasks/extraction-representation-gaps/recent-leaves.md (.41.b)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_register_width_the_document_does_not_state_is_listed_as_the_adapters_choice section_header_registers_capture_register_container; then no register_records entry in the 31 retained evidence_ir.json files carries size_bits while also having located fields, and ADIv6's adapter.json holds a residual_decisions packet isf_register_width_not_stated"
---

`PDF-VARIANT-DIGESTION.2c` filled a register's `size_bits` from its fields' extent (the highest `bits_high` plus one)
at five places in `ir/evidence.rs`. On `2026-09-29`, all 201 registers with located fields on the 31 current chains
had their width that way, and none stated a different one. So the IR stated widths the document never did: ADIv6's
`ABORT`, whose fields end at bit 4, read 5 bits wide.

Since `EXTRACTION-REPRESENTATION-GAPS.41.b` the readers record no width, and `register_size_from_fields` is gone.
An unset width is what `registers_with_unresolved_width` (`ir/completeness.rs`) reports as a gap. The adapter still
needs one: `register_var_width` takes `size_bits` when stated, else the highest field bit plus one, else 32. It lists
every register whose width it chose in one `isf_register_width_not_stated` packet, as `.7.d` lists a chosen reset
kind. No `.isf` text moved. Five adapters gained the packet: ADIv6, CoreSight TMC, OpenCAPI discovery, RISC-V IOMMU,
and RISC-V Debug, whose EvidenceIR did not move.

A width some documents state: RISC-V IOMMU's *Table 15. IOMMU Memory-mapped register layout* gives each register's
`size` in bytes, but its field-table registers are unnamed (`register_table_0033`) and cannot be matched to it (`.41.c`).
See also [[register-field-table-extraction]] and [[register-reset-isf-emit]].
