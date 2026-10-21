---
id: a-signal-titled-code-column-determining-its-member-is-an-enum
title: An encoding table's signal-titled code column is that signal's enum when its codes determine the member; one signal's bit columns concatenate into one code
answers:
  - "what is synthesize_signal_column_encodings / is_one_bit_of_a_split_signal / encoding_member_name"
  - "why does AXI have an AxPAS enum (Table A4.7, every row in the header)"
  - "why does AHB's HSIZE come from three columns HSIZE[2] | HSIZE[1] | HSIZE[0]"
  - "why did AHB's HSIZE once read BYTE=0, HALFWORD=0, ... (one bit read alone)"
  - "why do AxPROT[1] and AxNSE not mint an enum from A4.7 (they key the space only jointly)"
  - "why does A4.4 not mint AWCACHE (its codes repeat)"
  - "why are AxSIZE members named AXSIZE_1_BYTE_PER_TRANSFER and HSIZE's HSIZE_4_WORD_LINE (encoding_member_identifier, .58)"
  - "why does a member named only by a number mint nothing (16 message credits, a PPROT bit's 0)"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.g.iii.a"
date: 2026-10-21
status: current
tags: [evidence-ir, encodings, enums, multi-column-tables, collective-names, isf-enums, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (synthesize_signal_column_encodings; is_one_bit_of_a_split_signal; encoding_member_name; encoding_member_identifier; synthesize_encoding_declarations_for_enum; the convergence loop's signal_titles and minted sets); docs/tasks/extraction-representation-gaps/packet-and-memory-leaves.md (.3.g.iii, .3.g.iii.a, .58)
reverify: "cargo test --offline -p specforge-core --lib a_signal_titled_code_column_that_determines_its_member_is_an_enum"
---

An encoding table can title several code columns by signals. AXI *A4.7 Physical address space encodings* gives each
space an `AxPAS` code next to the `AxPROT[1]` and `AxNSE` bits that key four of them jointly. AHB *3-2 Transfer size
encoding* writes `HSIZE` as three one-bit columns. The single-column reader (`synthesize_encoding_declarations_for_enum`)
takes one value column, and `adjudicate_member_name_column` refuses a table keyed by more than it. So A4.7 minted
nothing: every row sits in its header, which is a converter defect (`.57`). AHB read `HSIZE[2]` alone and published
`BYTE = 0`, `HALFWORD = 0`, … on two codes, which SemanticIR refused to fuse.

**Reading (`EXTRACTION-REPRESENTATION-GAPS.3.g.iii.a`, `2026-10-21`).** `synthesize_signal_column_encodings` runs in
the evidence convergence loop after the signal-anchor scan, on tables the converter typed *encoding*.

- **Signal titles.** A column title names a signal when it, less a trailing index, is a known signal or a collective
  name the document defines. The collectives come from `collective_signal_name_surface` over the pass's statements.
- **Codes.** Each signal's column is its code (`parse_encoding_member_value`). One signal's one-bit columns are
  concatenated, most significant first, into a binary code.
- **When it is an enum.** A signal mints its enum only when its codes determine the member: two or more rows state
  one, and no code or member name repeats. The member gates still apply (`is_prose_fragment_member_name`,
  `is_width_parameter_leak_member`).
- **Member names** come from the first non-signal column that is no code and no number and whose names are distinct.
- **Header-only tables.** A table whose rows all sit in its header is read from its second row on.
- **No double reading.** A `(table, enum)` an earlier reader minted is left to it. The single-column reader now
  refuses a value column that is one bit of a split signal (`is_one_bit_of_a_split_signal`).

Left alone: joint keys (`AxPROT[1]` + `AxNSE`) are read as a value combination since `.3.g.iii.b.i`
([[a-jointly-keyed-table-is-a-value-combination]]), repeating codes (A4.4's `ARCACHE`/`AWCACHE`)
`.3.g.iii.c`'s, and untyped mapping tables are not read.

**Measured `2026-10-21` (31 chains):** 29 identical. AXI gains `AxPAS` (6 members), and AHB's `HSIZE` statements now
carry the concatenated codes, so `HSIZE` reaches SemanticIR and the `.isf`. Each `manager.isf` gains the type and the
enum, and FSMGen's strict check accepts both. SemanticIR's identifier parser used to drop a digit-leading member
name. Since `.58`, `encoding_member_identifier` keeps the words behind the enum's name (`HSIZE_4_WORD_LINE`,
`AXSIZE_1_BYTE_PER_TRANSFER`, so AXI's whole `AxSIZE` enum now lowers, and `HSIZE` is 3 bits). A name that is only a
number (APB `PPROT` `0`, AHB `HPROT` `1`, CHI C2C message credits `16`) mints nothing.
