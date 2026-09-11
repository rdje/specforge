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
  - "which notations of the 482 dropped rows have been recovered"
  - "does SpecForge read an enumerated width set like 8, 16, 32, 64"
  - "why are 15 of Avalon's 26 declarations width-only"
  - "can synthesize_directions_from_relations be deleted without losing real signals"
  - "which of the 482 dropped rows does the arrow form actually recover"
  - "how many enumerated width cells exist corpus-wide"
  - "does reading the arrow form fix the four documents that lose every row"
  - "which of Avalon's eight signals does the arrow form actually recover"
date: 2026-09-11
status: current
tags: [evidence-ir, signal-tables, wire-based-100, signal-declaration-row-drop, recall, adr-0037]
evidence: scripts/measure_declaration_row_notations.py; crates/specforge/src/ir/evidence.rs (synthesize_signal_declarations_from_table, the `_ => continue` arm; infer_signal_direction_from_actor_text; infer_signal_table_row_width_hint); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.0); docs/tasks/WIRE-BASED-100.md (.10f); generated/source_ir/683091_18_1_2021_12_25_avalon_interface_specifications/source_ir.json (table_0012)
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

`.1` published the missing denominator as `extraction_manifest.declaration_row_accounting` — rows
considered, declarations emitted, and the verbatim name cell plus a shape reason for every row that
produced nothing. It is carried on the **extraction manifest, not on `EvidenceIr`**, and that is not
a stylistic choice: a new registered evidence rule field restamps the stage ruleset digest and
un-seals every persisted artifact, and the re-seal cannot complete because AXI, APB and AHB have no
`normalized/` bundle and the retained population is frozen at 24
(`[[retained-bundle-population-is-frozen]]`). Measured both ways — registered field: the proof-seal
gate refuses evidence, semantic and intent; manifest: the corpus stays sealed. **Adding an EvidenceIR
rule field is currently structurally unlandable**, which is worth knowing well beyond this tree.

**Correction (`2026-09-11`, `.1b`): "manifest: the corpus stays sealed" was measured with an
instrument that cannot see the defect.** The seal did stay put, and the corpus still stopped loading:
changing the *content* of an already-registered rule field moves `inputs_sha256` for every claim in
the artifact and stales every persisted proof without touching the ruleset digest. Four of the 27
proof-carrying documents — AXI, APB, AHB and ADIv6, every wire-bearing one — refused to load for three
commits, so `eval-extraction` refused all eight golds. The proof-seal gate reported green because it
probes one document per distinct seal, and the seal is homogeneous precisely because it ignores
content. Repaired by rebuilding those four chains with the delta attributed; the mechanism and the
repair procedure are `[[evidence-rule-field-content-stales-every-proof]]`, and the sampling blind spot
is `SIGNAL-DECLARATION-ROW-DROP.1c`.

**The two unread notations have since been censused, and the arrow form is much smaller than it
looked.** `python3 scripts/measure_declaration_row_notations.py` reads every persisted SourceIR and
finds **83** flow-arrow cells in direction-bearing columns of `signal_description` tables — in exactly
**two documents**, over 13 distinct cell forms, so the adjudicable sample is the whole population.
**18 admit** under a mirror test that requires both sides to resolve through the actor taxonomy and to
agree (`Master → Slave` → `output` ×11, `Slave → Master` → `input` ×7); **65 fail closed** — 16 cells
stating two opposite flows at once (a bidirectional group, correctly refused) and 49 whose actor names
(`Distributor`, `ITS`, `Source → Sink`, `Interconnect`) are outside the builtin taxonomy. Enumerated
width cells number **7**, in two documents.

Two consequences correct this card's own framing. **The four documents that lose every row contain no
arrow cell at all**, so the notation work cannot move them off zero — their loss has some other cause,
still unidentified. And 18 of 482 is **3.7%**: reading both notations is a fidelity repair for two
documents, not a recovery of the 18.3%. `WIRE-BASED-100.10f`'s unblock is also narrower than stated
above — the admitted rows carry `address`, `byteenable`, `readdata`, `writedata` and `burstcount`, so
**5 of the 8** signals, not 8; `CHANNEL`, `DATA` and `ERROR` reach EvidenceIR some other way and that
path must be re-measured before the deletion. Adjudicating that same 18-row selection is what exposed
`[[bracketed-metavariable-name-cell]]`: 2 of the 7 rows the arrow form would newly admit are template
metavariables, which had to be refused first. The arrow grammar itself shipped in `.2b` with the
mirror test that keeps it from over-firing — `[[flow-arrow-direction-grammar]]` — so `readdata` and
`writedata` now declare with directions, and only their enumerated width remains unread.
