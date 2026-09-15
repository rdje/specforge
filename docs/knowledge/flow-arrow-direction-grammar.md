---
id: flow-arrow-direction-grammar
title: A direction column may state the signal's flow rather than its port sense, and only a mirror test makes that readable without over-firing
answers:
  - "does SpecForge read `Master -> Slave` as a direction"
  - "what is the flow-arrow direction grammar"
  - "which arrow spellings does the declaration reader accept"
  - "why does a cell with two arrows fail closed (it lists both senses of one link and the sibling Forward-or-reverse column does not select between them - that column is redundant with the arrow wherever its meaning is observable)"
  - "why must both sides of a direction arrow resolve to a role"
  - "why does SpecForge not read a leftward arrow as a flow"
  - "how many arrow-form direction cells exist corpus-wide"
  - "does reading the arrow form recover Avalon's eight signals"
  - "why do GIC-600's Distributor and Redistributor arrow rows produce no direction (a taxonomy gap that .2d answered NO - the six names are product blocks, and the tables are headed Interblock / Interdomain / Interchip signals)"
  - "where does the flow-arrow reading sit in the direction priority chain"
date: 2026-09-15
status: current
tags: [evidence-ir, declarations, signal-tables, adr-0006, recall, signal-declaration-row-drop, wire-based-100]
evidence: crates/specforge/src/ir/evidence.rs (FLOW_ARROW_FORMS; FLOW_ARROW_DISQUALIFIERS; split_on_single_flow_arrow; infer_signal_direction_from_flow_arrow; the_corpus_flow_arrow_forms_admit_only_the_mirrored_ones; a_flow_arrow_needs_both_sides_to_resolve_and_agree; the_corpus_arrow_direction_rows_declare_their_signals); scripts/measure_declaration_row_notations.py; docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.2b, .2d); scripts/measure_actor_taxonomy_blast_radius.py
reverify: "python3 scripts/measure_declaration_row_notations.py — expect 83 flow-arrow cells, 18 admitted (11 output, 7 input), 16 closed:multiple_arrows, 49 closed:unresolved_actor; then cargo test -p specforge-core --lib the_corpus_flow_arrow_forms_admit_only_the_mirrored_ones"
---

A signal table's direction column does not always say `input` or `output`. Some documents write the
**flow** — `Slave → Master` — and the reader read neither that nor the enumerated width beside it, so
rows that plainly name a wire produced nothing at all
(`[[declaration-reader-drops-uninterpretable-rows]]`).

The arrow is the grammar; the two actors either side of it go through the same role taxonomy every
other direction path uses, so this adds a notation and not a vocabulary (ADR 0006). It is tried after
the three literal column readings and before the description-prose fallback: a column the table
designates for direction outranks a sentence, whichever notation it uses.

**The rule admits only a mirrored cell.** Reading the left side as a source and the right side as a
destination must yield the *same* port sense. One recognised role beside one unrecognised name states
a flow relative to an actor whose role is unknown; two sides that disagree contradict each other; two
arrows in one cell state two flows and leave the row under-determined. All fail closed, as does any
reverse or bidirectional marker (`←`, `↔`, `<-`, `<=`). Accepted spellings are the right-flowing forms
only — `⟶ ⇒ → ==> --> => ->`, matched longest-first so `-->` is one arrow rather than `->` after a
stray `-`. Leftward arrows are **deliberately unread**: no corpus direction cell uses one, and the one
family that writes `←` writes it as assignment (`ATVALID ← 0`), so reading it as flow would ship a
rule with no population behind it.

## What it selects, measured

Over all 78 persisted SourceIR artifacts there are **83** arrow-bearing cells in direction-bearing
columns of `signal_description` tables, in **2 documents** and **13 distinct forms** — few enough that
the adjudicable sample is the population, which is why the in-crate control enumerates all thirteen.

| verdict | cells | what they are |
| --- | ---: | --- |
| admitted `output` | 11 | `Master → Slave` |
| admitted `input` | 7 | `Slave → Master` |
| closed, two flows | 16 | `ITS →Distributor Distributor →ITS` — both senses of one link, under-determined |
| closed, unresolved actor | 49 | `Distributor→ Remote chip`, `Source → Sink`, `Interconnect → Slave` |

All 18 admitted are genuine direction statements: zero false positives. Seven of them are rows that
produced nothing before; after `[[bracketed-metavariable-name-cell]]` refuses two template rows, five
are real wires. The other eleven already declared a width and gain a direction.

The 49 closed rows are a **taxonomy** gap, not an arrow gap: `builtin_actor_taxonomy_role_in_text`
knows four requester-like and six completer-like terms, and none of `Distributor`, `Redistributor`,
`ITS`, `SPI Collator`, `Wake Request`, `Remote chip`, `Source`, `Sink` or `Interconnect` is among
them. Widening that list is not local — the same function decides section-heading direction and prose
relation direction — so `SIGNAL-DECLARATION-ROW-DROP.2d` measured it before guessing, and **answered
NO**: six of the nine are product block names (ADR 0006), `source`/`sink` is 60 % false positives and
destroys a third document's relations, `interconnect` moves 92 sites to buy one cell, and — the reason
that generalises — **half a pair is worse than none**, because one known endpoint makes the literal
actor-text reading answer before this one and give both senses of the link the same direction
(`[[actor-taxonomy-grows-in-pairs-not-terms]]`).

**The two-flow row's recorded reason was wrong, and `.2d` corrected it.** These 16 cells are not
"bidirectional groups": the cell lists both senses of a two-party link, and a sibling column
`Forward or reverse` sits beside every row. What keeps them under-determined is that the sibling column
is **redundant** with the arrow wherever its meaning is observable — GIC `table_0173` writes
`icdctready | Reverse | SPI Collator→ Distributor` beside
`icdctvalid | Forward | Distributor →SPI Collator`, each single-arrow cell already the row's own
resolved flow — so no corpus row shows it selecting between two listed arrows, and the listed pair
carries no forward-first convention. The verdict stands on better evidence than it had.

## Two predictions this graded

`SIGNAL-DECLARATION-ROW-DROP.2` predicted, before implementation, that the arrow form would "move the
four all-zero documents off zero". It does not: AMD IOMMU, HBM2, ATP and CHI C2C contain **no arrow
cell at all**, so their total loss has a different, still-unidentified cause. It also predicted
recovery of "Avalon's eight signals"; the admitted set carries `address`, `byteenable`, `readdata`,
`writedata` and `burstcount` — **5 of the 8**, with `CHANNEL`, `DATA` and `ERROR` absent, so
`WIRE-BASED-100.10f` must re-measure before assuming its unblock. The surviving half of the
prediction holds comfortably: 18 of 482 rows is **3.7 %**, nowhere near a majority.
