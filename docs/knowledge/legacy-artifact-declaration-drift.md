---
id: legacy-artifact-declaration-drift
title: 22 of the 51 legacy evidence artifacts disagree with the current declaration reader, so a legacy artifact is evidence about itself and never about the reader — and the largest class of disagreement is one name-case change, not lost wires
answers:
  - "is a persisted legacy evidence_ir what the current binary produces"
  - "how many legacy artifacts have drifted from the reader"
  - "may I cite a persisted artifact as evidence about the current reader"
  - "why can a legacy evidence artifact not be replayed"
  - "which declaration producers need the normalized markdown bundle"
  - "what does UNMEASURABLE mean in check_chain_currency"
  - "how do I replay the table declaration pass on a legacy document"
  - "why must a replay comparison apply the base-name template withholding"
  - "what is the control for a persisted-versus-reader comparison"
  - "does SpecForge mint a signal named Input or Output"
date: 2026-09-20
status: current
tags: [corpus-chain-currency, evidence-ir, legacy-stratum, measurement, claim-verification]
evidence: crates/specforge/src/ir/evidence.rs (mod corpus_chain_currency_11); docs/research/legacy-artifact-declaration-drift.md; docs/tasks/CORPUS-CHAIN-CURRENCY.md (.11)
reverify: "cargo test -p specforge-core --lib corpus_chain_currency_11 -- --ignored --nocapture — expect 0 of 27 proof-carrying documents divergent (the control) and 22 of 51 legacy; 415 tables compared, 272 identical, 143 differing, 6 excluded; of the differing, 73 tables (527 declarations) identical but for name case, then 304 artifact-only, 163 reader-only and 24 same-name-different-sentence; plus 0 direction-word-named declarations, which read 14 across 5 table/document pairs until SIGNAL-DECLARATION-ROW-DROP.5 shipped on the same day and is kept as a standing control."
---

`check_chain_currency.sh` reports **27 replayed, 27 current, 51 UNMEASURABLE** and never claimed the
51 were current. The hazard is downstream of that honesty: a persisted `evidence_ir.json` exists for
**all 78** documents and censuses read them — `[[direction-column-drift]]` takes its `declared`
column out of exactly these files.

**Measured: 22 of the 51 legacy documents disagree with the current reader; 0 of the 27
proof-carrying ones do.**

## A whole-artifact comparison is impossible, and that is part of the answer

`build_unproved_from_source_ir` reaches the declaration seed only after
`assemble_evidence_statements`, which needs the document's **normalized markdown bundle** — reclaimed
for all 51. Two of the three declaration producers read those base statements: the sparse-catalog
prose fallback, and the trapped-row pass whose inventory gate is keyed on the declared universe.
Neither replays. **`synthesize_declarations_from_tables` does**, because it reads
`source_ir.structured_tables` and nothing else — and it is the producer whose output the censuses
quote.

## The control fired once, and that is why the number can be trusted

The 27 proof-carrying documents must agree, or the METHOD is wrong. They did not: AXI `ihi0022_l`
`table_0011` showed six reader-only declarations (`VALID`, `PENDING`, `RP`, `CRDT`, `CRDTSH`,
`SHAREDCRD`) — the base-name TEMPLATE table `WIRE-BASED-100.10b` withholds. The artifact is what the
pass PUBLISHES, so the comparison must apply `withhold_base_name_template_declarations`. With it, the
control reads **0 of 27**.

## The disagreement, characterised before it is counted

415 tables compared, 272 identical, **143 differing**, 6 excluded. Of the 143:

| class | size |
| --- | --- |
| identical but for **name CASE** (`CHIP_ID` vs `chip_id`) | **73 tables, 527 declarations** |
| in the artifact, not the reader | 304 |
| in the reader, not the artifact | 163 |
| same name, different sentence | 24 |

The largest class is **one normalisation change, not lost wires**; counting those 527 as differences
would overstate the drift by several documents' width. The rest split into the reader finding wires
the artifact has none of (GIC-600 `table_0160`/`0162`), the artifact carrying phantoms the reader has
since refused (`table_0170`'s `ALLOW`/`ERROR`/`FAULT`/`ID`), and a direction the reader no longer
asserts on register rows (Cortex-A76 `table_0063`).

## The decision

**A legacy artifact may be cited about itself, never about the reader.** Any census quoting a count
out of one must say so, or replay the reader. Deliberately **not gated**: a check would have to
recognise intent rather than a file, and would fire on every legitimate read of the stratum. The
controls are the producer, the research record, and the caveat now carried by the drift census.

## Surfaced in passing, and owned

GIC-600 `table_0163`/`0164` published `Signal Input is input.` — the CURRENT reader minting the
direction word as a name. **14 declarations across 5 table/document pairs**, all one document. Owned
by `[[SIGNAL-DECLARATION-ROW-DROP]].5`, **shipped the same day**: the cause was `.2e`'s content
override electing the DIRECTION column as the name column where the real names are all
metavariables, and the count is now **0**, kept as a standing control by this module.

Links: [[direction-column-drift]], [[per-row-column-drift-rule]].
