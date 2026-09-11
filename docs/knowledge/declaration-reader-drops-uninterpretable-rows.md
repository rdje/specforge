---
id: declaration-reader-drops-uninterpretable-rows
title: The authoritative signal-declaration reader silently discards 18.3% of the rows it is handed, and the wire golds score 1.000 across the loss
answers:
  - "why do Avalon's readdata and writedata never become declarations"
  - "why is synthesize_directions_from_relations load-bearing for Avalon"
  - "how many signal_description rows are dropped corpus-wide"
  - "what does the `_ => continue` arm in synthesize_signal_declarations_from_table cost"
  - "why does AXI score 1.000 while losing 103 declaration rows"
  - "does SpecForge read the Slave -> Master arrow direction form"
  - "does SpecForge read an enumerated width set like 8, 16, 32, 64"
  - "why are 15 of Avalon's 26 declarations width-only"
  - "can synthesize_directions_from_relations be deleted without losing real signals"
date: 2026-09-11
status: current
tags: [evidence-ir, signal-tables, wire-based-100, signal-declaration-row-drop, recall, adr-0037]
evidence: crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations_from_table, the `_ => continue` arm; infer_signal_direction_from_actor_text; infer_signal_table_row_width_hint); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.0); docs/tasks/WIRE-BASED-100.md (.10f); generated/source_ir/683091_18_1_2021_12_25_avalon_interface_specifications/source_ir.json (table_0012)
reverify: "re-run the .0 census over generated/source_ir + generated/evidence_ir: count signal_description body rows whose first cell is a single identifier, minus table_signal_declaration_provenance entries for that table; expect 482 of 2637"
---

Table declarations are the authoritative source of signal declarations — they win over everything
derived. `synthesize_signal_declarations_from_table` builds each declaration from a
`(direction, width)` pair and ends the match with `_ => continue`: a row with neither is dropped. The
drop is total and silent — no declaration, no residual, no counter, no validation-report entry, and no
rows-in/declarations-out ratio anywhere for a gate to bound.

Measured over all 78 persisted artifact pairs, **482 of 2,637 rows (18.3%)** are discarded this way,
counting only `signal_description` tables whose name cell is a single identifier — a lower bound, since
multi-name cells (`read read_n`) and bracketed forms (`response [1:0]`) are excluded. Four documents
lose **every** row: AMD IOMMU (59), HBM2 (32), AMBA ATP (25), CHI C2C (15). The largest absolute losses
are Cortex-A76 (107) and AXI (78 + 25 across two specifications).

Two unread notations account for the Avalon case. Direction is written as the arrow form
`Slave → Master`, which `infer_signal_direction_from_actor_text` does not read — confirmed by
**15 of Avalon's 26 surviving declarations being width-only**. Width for `readdata` is the enumerated
set `8, 16, 32, 64, 128, 256, 512, 1024`, which `infer_signal_table_row_width_hint` does not read.
Neither is inferable, so `readdata` and `writedata` — plainly present in `table_0012`, a table
SpecForge itself typed `signal_description` with first column `Signal Role` — never become declarations.

The consequence reaches further than recall. `WIRE-BASED-100.10f` exists because
`synthesize_directions_from_relations` turns a relation into a declaration, violating the SemanticIR
authority doctrine, and could not be deleted without losing Avalon's eight genuine signals. Those eight
are load-bearing on that path **only because the authoritative path drops their rows**. Repairing the
reader dissolves the trade rather than resolving it: no discriminator, and no entity-typing tier, is
needed to tell the junk from the real names.

Three gates are all green across this. The scores cannot see it — AXI scores `1.000` on every aspect
while losing 103 rows, because a gold is evidence only about the facts it names
(`[[a-green-score-is-evidence-only-about-its-gold]]`). Chain-currency cannot see it — a dropped row is
not drift; the persisted artifact is exactly what the current binary produces, so the loss being
deterministic is precisely why replay stays green. And no check can bound a ratio that is never
computed. Owned by `[[SIGNAL-DECLARATION-ROW-DROP]]`; see also
`[[property-table-is-not-a-signal-inventory]]` for the opposite error in the same reader.
