---
id: a-table-naming-a-signals-bits-is-its-layout
title: A table whose one column lists slices of one declared signal, each named, is that signal's layout (signal_layouts); a level is read only outside a condition
answers:
  - "what is signal_layouts / SignalLayoutRecord / SignalSubField (.3.c.iv.b.i)"
  - "how is AHB Table 3-6 Meaning of the HPROT bits read"
  - "how is APB's PPROT table (Normal or Privileged, LOW indicates …) read"
  - "how are AXI's MPAM field tables (PARTID onto AxMPAM[9:1]) read"
  - "why does HPROT[2] Bufferable carry no level meaning"
  - "why is ADIv6's caption-less HPROT map not a signal layout"
  - "does a signal's sub-field layout reach the .isf (FSMGen ladder F29)"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b and .3.c.iv.b.i"
date: 2026-10-22
status: current
tags: [evidence-ir, signals, sub-fields, layouts, encodings, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (read_signal_layouts; parse_name_slice; sub_field_levels; sentence_level; signal_layout_surface); docs/tasks/extraction-representation-gaps/encoding-and-state-leaves.md (.3.c.iv.b); docs/tasks/extraction-representation-gaps/layout-and-binding-leaves.md (.3.c.iv.b.i); docs/gaps/fsmgen-ladder.md (F29)
reverify: "cargo test --offline -p specforge-core --lib a_table_naming_a_signals_sub_fields_is_its_layout"
---

Specifications name the bits of a control signal in a table with a row per bit or slice. AHB *Table 3-6* (*Bit | Name
| Description*) gives `HPROT[0]` … `HPROT[6]` a name and a meaning. APB5's untitled `PPROT` table gives each bit a name
and a *LOW indicates … HIGH indicates …* comment. AXI *A12.3*-*A12.6* map *MPAM_NS*, *PARTID* and *PMG* onto `AxMPAM`
slices, in two layouts per MPAM width chosen by `RME_Support`. Until `.3.c.iv.b.i` these reached EvidenceIR only as
raw row statements.

**The reader (`EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.i`, `2026-10-22`).** `read_signal_layouts` takes a table with
exactly one column whose every cell is a slice of one declared signal or collective (`parse_name_slice`), at least
two non-overlapping slices, and a name column (the first other column of distinct cells of at most five words, no
number). It refuses a table with another column holding a slice or a binary literal, which is a register-to-signal
map (`.41.e`) or a per-message layout (`.3.c.iv.c`). It also refuses a number column that does not restate each
slice's width: ADIv6's caption-less map puts a register bit beside each `HPROT` bit. A continuation page joins its
table. Each table is one `SignalLayoutRecord`, never merged with another.

**Levels.** `sub_field_levels` reads, for a one-bit sub-field, each sentence opening *When asserted*, *When
deasserted*, *Asserted,*, *Deasserted,*, *LOW indicates* or *HIGH indicates* (`LOWindicates` too: the text layer
drops the space). A cell whose level sentences follow a sentence ending in `:` or opening *If*/*Unless* gives none.
That is why AHB `HPROT[2]` (*If both of HPROT[4:3] are deasserted, then when this bit is: …*) has no levels.

**Measured `2026-10-22` (31 chains):** AHB 1 layout (7 sub-fields, 12 levels), APB5 1 (3, 6 levels), AXI 4 (3 fields
each), ADIv6 one candidate refused; 27 chains identical. The `signal_layouts` manifest entry is recorded only when a
table has the slice column, so other documents keep their manifests.

**Not lowered.** `.3.c.iv.b.ii` carries the layouts to IntentIR. An `.isf` port takes no field (port options are a
closed set at the pin and upstream; `(fields …)` is storage-only), and ADR 0033 §4 files an FSMGen request only for a
semantic in IntentIR that executable intent needs. So FSMGen ladder rung F29 is *anticipated*.
