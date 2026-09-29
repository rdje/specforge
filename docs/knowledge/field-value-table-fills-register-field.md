---
id: field-value-table-fills-register-field
title: A table whose header names one register field fills that field's enumerated values; a second code column, a shared field or a filled field refuses it
answers:
  - "where do a register field's enumerated values come from"
  - "why did ADIv6 CSW.AddrInc have no values"
  - "does SpecForge read a field value table like Summary of AddrInc field values"
  - "what does fill_field_values_from_value_tables do"
  - "why is Table C2-2 ERRNPASS ERRSTOP not read as field values"
  - "why does dcsr.cause have values in IntentIR but not in the isf"
  - "which table wins when two tables list the same field's values"
date: 2026-09-29
status: current
tags: [evidence-ir, registers, register-fields, enumerated-values, captured-regions, adr-0006]
evidence: crates/specforge/src/ir/evidence.rs (register_record_surface, fill_field_values_from_value_tables, field_named_by_value_header, column_is_mostly_codes, first_sentence); docs/tasks/extraction-representation-gaps/census-leaves.md (.3.c.iii); docs/book/src/pipeline/register-tables.md (A field's value table)
reverify: "cargo test --offline --workspace --exclude specforge-production-graph --lib a_field_value_table_fills_the_one_field_it_names a_value_header_names_exactly_one_field; then python3 over generated/intent_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/intent_ir.json: register CSW@MEM-AP's fields AddrInc, Size, DbgSwEnable carry 4, 6 and 2 enumerated_values"
---

A register's bit table gives each field a range and a name; the meaning of its values is often a separate table.
ADIv6 prints *Table C2-1 Summary of AddrInc field values* (`AddrInc value | Description | …`, `0b00` Auto-increment
off …). Until `EXTRACTION-REPRESENTATION-GAPS.3.c.iii` (`2026-09-29`) no record read it: `CSW.AddrInc` had its bits
and no values, and the table was a captured-region residual.

`register_record_surface` now ends with `fill_field_values_from_value_tables`. A table qualifies when its first header
cell names one field (`field_named_by_value_header`: the field, optionally qualified by its register as in
`CSW.Size`, beside only `value`, `of`, `encoding`, `field`) and exactly one register in scope has that field. Each row's
first cell is a value: a register literal, or a plain decimal only when no cell of the column is two or more `0`/`1`
digits (`10` could be binary). A footnote mark after it is removed only when the page prints that note. The meaning is
the second cell's first sentence, at most 80 characters, and at least two rows must state both. The register then
cites the table.

Refusals, each tested: a second column of codes (`column_is_mostly_codes`; ADIv6 *Table C2-2* keys a mode by
`ERRNPASS` × `ERRSTOP`), a field name two registers share when the header names no register, radix-ambiguous
decimals, and a field that already has values. So the first table in document order wins: ADIv6 *Table C2-6* maps
`DbgSwEnable` to the `DBGSWENABLE` signal's `LOW`/`HIGH`, after *Table C2-5* gave the field's meaning. A range cell
(`0b110 - 0b111 Reserved`) is not a value and is left out.

Measured over the 31 replayable chains, only two moved: RISC-V Debug `dcsr.cause` (6 values, *Table 8*) and ADIv6
`CSW@MEM-AP` (`DbgSwEnable` 2, `Size` 6, `AddrInc` 4). ADIv6's `.isf` text gains those three field enums. RISC-V's does
not, because `dcsr.cause` has no bit range: RISC-V Debug draws its field positions in a figure (gap S23,
`DOC-INTENT-TAXONOMY.4`), and the adapter lowers only located fields. See also [[captured-region-residual-carrier]].
