---
id: timing-caption-unit-and-table-provenance
title: Timing constraints preserve explicit caption units and direct structured-table provenance
answers:
  - "how does SpecForge recover a timing unit from a table caption"
  - "which caption grammar can supply a unit to every timing row"
  - "does an explicit timing row unit override a caption unit"
  - "where do TimingConstraintRecord table provenance ids live"
  - "do SemanticIR and IntentIR preserve timing units and table provenance"
  - "are older timing constraint records compatible with supporting_table_ids"
date: 2026-08-12
status: current
tags: [timing, evidence-ir, semantic-ir, intent-ir, tables, provenance, units]
evidence: crates/specforge/src/ir/source.rs (timing_caption_unit, TimingConstraintRecord); crates/specforge/src/ir/evidence.rs (synthesize_timing_constraints); docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6c.i)
reverify: "cargo test -p specforge --lib timing_table_ && bash scripts/check_chain_currency.sh"
---

`timing_caption_unit` supplies a table-wide unit only for the complete caption cue `all value(s) in <unit>`,
where the unit is one of `s`, `ms`, `us`, `ns`, `ps`, `fs`, `cycle(s)`, `clock cycles`, or `UI`. A unit mention
without the cue cannot fill rows. A non-empty explicit unit cell takes precedence, and the chosen source spelling
is preserved.

Every `TimingConstraintRecord` synthesized from a structured table carries that table's id in
`supporting_table_ids`, separately from prose `supporting_statement_ids`. The field defaults to empty and is
omitted when absent, so old artifacts deserialize. SemanticIR and IntentIR clone the record without changing the
unit or provenance. Timing observations derived outside a table keep an empty table list.

The causal defect was SourceIR → EvidenceIR: I2S `table_0004` retained the caption `all values in ns`, but the
timing producer read only a dedicated unit column and the carrier had no direct table-provenance field. Later
stages faithfully cloned those omissions. The `.6c.i` retained-chain reconciliation proves all five reviewed
rows carry exact `ns` plus `table_0004` through IntentIR; the same generic provenance carrier accounts for every
table-derived timing record in the five affected replayable chains without changing their other content.
