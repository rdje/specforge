---
id: actor-taxonomy-grows-in-pairs-not-terms
title: The actor-role taxonomy grows one PAIR at a time, and half a pair is worse than none — it masks the flow-arrow reader and gives both senses of a link the same direction
answers:
  - "should I add a term to builtin_actor_taxonomy_role_in_text (measure it first with scripts/measure_actor_taxonomy_blast_radius.py; the answer for every candidate the corpus has produced so far is NO)"
  - "what is the blast radius of a new actor-role term (four surfaces: the direction cell of a signal row, a section heading ending in ' signals', a relation-actor name in the by-role map, and the complementary-reader's exactly-one-opposite-name condition, which is NOT monotone)"
  - "does SpecForge read a direction cell containing an arrow as an actor name (no - .2g guards it: infer_signal_direction_from_actor_text declines any cell carrying a FLOW_ARROW_FORMS or FLOW_ARROW_DISQUALIFIERS marker, ahead of even the literal input/output substring readings, so the flow reader judges it)"
  - "why does adding one actor term make a flow-arrow cell WORSE rather than better (infer_signal_direction_from_actor_text is tried at priority 2 on the WHOLE cell and infer_signal_direction_from_flow_arrow at priority 4; one known endpoint matches the whole cell, answers first, and returns the same port sense for both senses of the link)"
  - "why does adding both endpoints of a flow to the taxonomy NOT mask the arrow (the cell then matches a requester term AND a completer term, the (true,true) arm returns None, and the arrow reader is reached)"
  - "can a new actor-role term DESTROY existing relations (yes - unique_complementary_reader_actor_name mints a Reads relation only when the opposite role holds exactly one name, so a term that takes that set from one to two deletes every complementary relation the document had; measured on AMBA GFB with 'source')"
  - "why does SpecForge not read GIC-600's Distributor / Redistributor / ITS / SPI Collator arrow rows (they are product block names - ADR 0006 - and the document heads those tables Interblock / Interdomain / Interchip signals, so the flow is between two peer blocks and has no port sense without a chosen subject)"
  - "why does SpecForge not treat Source and Sink as requester and completer (the pair recovers 8 Avalon-ST rows and costs 12 wrong ones - Clock source, Reset source, Interrupt source are not actors - plus one document's complementary relations; 60 percent false positives)"
  - "can a table's own subject give a flow a port sense without any taxonomy (no - measured: the actor common to every arrow cell is unique in only 2 of 11 arrow tables, because a two-party table names both parties in every cell, and against the 18 cells the taxonomy already admits the rule scores 0 agreements and 1 disagreement)"
  - "why do GIC-600's two-arrow direction cells stay closed even though a Forward or reverse column sits beside them (that column is REDUNDANT with the arrow wherever its meaning is observable - a single-arrow cell already states the row's own resolved flow - so no corpus row shows it selecting between two listed arrows)"
  - "can a learned prior make an actor name requester-like with no vocabulary (yes - learn_priors derives the role from the semantic role of the signals that actor drives; one decisive HandshakeValidLike consensus and no competing role is enough)"
  - "how many corpus rows does the literal actor-text direction reading answer (476, and 0 of them carry a flow marker - which is why the .2g guard moved no artifact, NOT evidence that the reader was right: with the guard removed it answers the real cell Interconnect -> Slave with input, and only the Direction column header kept the chain from asking it)"
date: 2026-09-15
status: current
tags: [evidence-ir, declarations, signal-tables, actors, adr-0006, signal-declaration-row-drop, census]
evidence: scripts/measure_actor_taxonomy_blast_radius.py; crates/specforge/src/ir/evidence.rs (builtin_actor_taxonomy_role_in_text; actor_taxonomy_role_in_text; infer_signal_direction_from_actor_text; infer_signal_direction_from_flow_arrow; actor_name_and_role_from_section_heading; collect_local_actor_names_by_taxonomy_role; unique_complementary_reader_actor_name; synthesize_signal_declarations); crates/specforge/src/commands/learn_priors.rs (infer_actor_taxonomy_role; infer_actor_taxonomy_role_from_term); crates/specforge/src/ir/evidence.rs (cell_states_a_flow; a_cell_that_states_a_flow_is_not_read_as_an_actor_name; the_corpus_flow_arrow_forms_are_all_declined_by_the_actor_text_reading); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.2d, .2g, .2h)
reverify: "cargo test --offline -p specforge-core --lib a_cell_that_states_a_flow_is_not_read_as_an_actor_name; then python3 scripts/measure_actor_taxonomy_blast_radius.py — expect 9 discovered candidates, sites S1 3940 / S2 300 / S3 771, sink 0 sites changed, and the table-subject rule agrees 0 / disagrees 1; then python3 scripts/measure_actor_taxonomy_blast_radius.py --vocabulary 'distributor=requester' — expect 56 rows answered before the arrow and 3 flow-sense collapses over 28 rows"
---

`builtin_actor_taxonomy_role_in_text` maps a piece of actor text to `RequesterLike` or
`CompleterLike` from four requester words and six completer words. It looks like a list you extend
when a document names an actor it does not know. It is not, for two measured reasons.

## A term is not the unit; a pair is

The flow-arrow reader (`[[flow-arrow-direction-grammar]]`) needs **both** sides of `A → B` to resolve
and agree. So a term buys nothing unless the other endpoint is already known: adding `sink` on its own
changes **zero** sites corpus-wide, because every cell that names `Sink` also names `Source`, which the
builtin list does not know either. Anything that sizes one term at a time is measuring the wrong thing —
hence `scripts/measure_actor_taxonomy_blast_radius.py --vocabulary`, which sizes a set.

## Half a pair is worse than no pair, and the mechanism is a priority order

`synthesize_signal_declarations` decides a row's direction in a fixed order. Priority 2 is
`infer_signal_direction_from_actor_text` on the **whole** source/destination cell; priority 4 is
`infer_signal_direction_from_flow_arrow` on the same cell. A cell naming both endpoints of a flow
matches on whichever endpoint the taxonomy happens to know — so **one** term of a pair answers before
the arrow is ever consulted, and answers **the same for both senses of the link**.

Measured with `--vocabulary 'distributor=requester'`: **56** of the corpus's 83 arrow rows are answered
at `actor_text:source`, and **3 actor pairs / 28 rows collapse** — `Distributor→ Remote chip` and
`Remote chip→ Distributor` both `output`. With `remote chip` alone: 1 pair / 12 rows. With the
**complete** six-term vocabulary the cell matches a requester term *and* a completer term, the
`(true, true)` arm returns `None`, the arrow is reached, and **0 rows collapse** while 58 of 83 agree
with the stated flow.

The mirror condition the arrow reader was built around is bypassed entirely, because the arrow reader
never runs.

**`SIGNAL-DECLARATION-ROW-DROP.2g` closed this (`2026-09-15`):** `infer_signal_direction_from_actor_text`
declines any cell carrying a marker from `FLOW_ARROW_FORMS` or `FLOW_ARROW_DISQUALIFIERS`, ahead of even
the literal `input`/`output` substring readings, so the flow reader is the one that judges a flow cell.
**All 78 persisted documents rebuild byte-identical**, because none of the **476** corpus rows the
literal reading answers is a flow cell.

**"Latent" was half right, and the correction matters.** The CHAIN moves nothing, but the FUNCTION was
already wrong: measured with the guard removed,
`infer_signal_direction_from_actor_text("Interconnect → Slave", DestinationLike, None)` returns
`Some("input")`. That is a real Avalon `table_0014` cell. The only reason no artifact carried the wrong
answer is that Avalon heads that column `Direction`, so `source_col` and `dest_col` are `None` and the
chain never asks this reader — a column header, not a rule, was the thing keeping it out.

The hazard was also reachable without any builtin edit, through the learned-prior fall-through:
`actor_taxonomy_role_in_text` consults `CorpusMemory`, and `learn_priors` derives a role prior for an
actor NAME from the semantic role of the signals it drives — one decisive `HandshakeValidLike`
consensus and no competing role makes that name requester-like, no vocabulary required.

## The four surfaces, and the one that is not monotone

| surface | where | sites (`2026-09-15`) | what a term changes |
| --- | --- | ---: | --- |
| S1 direction cell | `infer_signal_direction_from_actor_text` | 3,940 rows | a declaration's `direction` |
| S2 section heading | `actor_name_and_role_from_section_heading` | 300 | the DEFAULT direction of every row under it, and a `Drives` relation per row where the table has no relation column |
| S3 relation actor | `collect_local_actor_names_by_taxonomy_role` | 771 | membership of the per-document by-role map |
| S4 complementary reader | `unique_complementary_reader_actor_name` | derived from S2+S3 | a `Reads` relation, but **only while the opposite role holds exactly one name** |

S4 is the trap. A term that takes an opposite-role set from one name to two **deletes** every
complementary relation that document was minting. Measured: adding `source` takes AMBA GFB's
requester set from `['manager']` to `['clock source', 'manager', 'reset source']` — a regression in a
document that contains no arrow cell at all and had nothing to gain.

## Every candidate the corpus has produced, and why each is refused

Nine candidates, discovered from the 49 fail-closed arrow sides rather than listed:

- **`distributor`, `redistributor`, `its`, `spi collator`, `wake request`, `remote chip`** — GIC-600
  product block names, which ADR 0006 forbids. The document agrees they are not module ports: the four
  tables sit under *A.7 Interblock signals*, *A.8 Interdomain signals* and *A.9 Interchip signals*, and
  a flow between two peer blocks has a port sense only relative to a subject those headings decline to
  pick. (`its` is also the English possessive pronoun, matched whole-token, in a taxonomy that section
  headings and prose relation subjects read.)
- **`source` + `sink`** — the one ADR-0006-admissible pair, and it fails adjudication **12 to 8**.
  The pair buys 8 Avalon-ST rows — `sink` alone buys none and costs nothing — and every one of the
  cost is `source`'s: 12 further rows, all wrong, with the document contradicting the rule
  *in the same row*: `[<domain>]clk | Input | Clock source | Clock input.`
  would be declared `output`, as would AXI's `ACLK` and GFB's `FCLK`. Plus the S4 destruction above.
- **`interconnect`** — 92 sites moved (43 AXI relation cells, 3 MMU-700 headings) to buy **one** cell,
  and an interconnect is neither requester nor completer: it sits between them.

## The taxonomy-free alternative, refuted by its own oracle

"Read the flow relative to the table's own subject" has one mechanical form — the actor appearing on
one side of **every** arrow cell in the table. It resolves **2 of 11** arrow tables, because a
well-formed two-party table names both parties in every cell and the intersection is never unique.
Graded against the 18 cells the taxonomy already admits: **agrees 0, disagrees 1, no-subject 17**. The
single cell where both rules can speak, they conflict — Avalon `table_0014` picks `slave` as subject,
because a third party (`Interconnect`) appears in the other cell, and flips `Master → Slave` from
`output` to `input`. It is a competing authority, not a fallback.

## A correction this census forced

The 16 two-arrow cells were recorded as *"genuinely bidirectional groups"*. They are not: the cell
lists both senses of one link, and a sibling column `Forward or reverse` sits beside every row. What
keeps them under-determined is that **the sibling column is redundant with the arrow wherever its
meaning is observable** — GIC `table_0173` writes `icdctready | Reverse | SPI Collator→ Distributor`
beside `icdctvalid | Forward | Distributor →SPI Collator`, each cell already the row's own resolved
flow — so no corpus row demonstrates it selecting between two listed arrows, and the listed pair
carries no forward-first convention. Fail-closed stands; the reason was wrong.

Links: [[flow-arrow-direction-grammar]], [[declaration-reader-drops-uninterpretable-rows]],
[[a-dropped-declaration-row-is-usually-not-a-signal]].
