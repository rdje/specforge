---
id: heading-as-declaration-needs-a-direction-bearing-body
title: A section heading shaped like a wire name selects 277 titles across 33 documents and is unsafe alone; requiring the section's own body to state a direction selects 27 in one document, all correct
answers:
  - "may a section heading declare a signal"
  - "what licenses a heading-as-declaration"
  - "why is an identifier-shaped section title not enough to declare a wire"
  - "how many identifier-shaped section headings does the corpus have"
  - "which document uses a heading-as-declaration convention"
  - "how many Wishbone wires does the heading rule recover"
  - "why are TGA_O and STALL_O refused by the heading rule"
  - "why does the heading rule ship at 25 when the census measured 27"
  - "why are TGD_I and TGD_O refused by the shipped heading rule"
date: 2026-09-20
status: current
tags: [signal-catalog-capture-gap, source-ir, extraction-recall, census, adjudication, adr-0006]
evidence: scripts/measure_heading_declaration_shape.py; crates/specforge/src/ir/evidence.rs (synthesize_heading_declarations, heading_body_binds_direction); docs/research/heading-as-declaration-shape.md; docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md (.1, .2, .3)
reverify: "python3 scripts/measure_heading_declaration_shape.py — expect 277 identifier-shaped headings across 33 documents, 65 compound across 11, and 27 licensed across 1 (Wishbone); --self-test expects 9/9; --landed expects the SHIPPED rule at 25 in 1 document and 0 elsewhere."
---

Wishbone declares its interface as a **heading-as-declaration convention**: sections whose title IS
the wire name (`CLK_I`, `RST_O`, `DAT_I()`, …), each followed by prose giving direction and meaning.
SpecForge reads declarations from tables and from a formal prose predicate, so it captured no
catalog at all — the single confirmed capture miss among 33 empty-catalog documents.

**The title shape cannot be the rule.** Measured over all 78 persisted artifacts:

| predicate | selects |
| --- | ---: |
| a section title that is a bare identifier token | **277** headings, **33** documents |
| …with an underscore or array/call suffix (a wire name's form) | **65**, **11** documents |
| …**and the section's own body states a direction** | **27**, **1** document |

The refused set is where the evidence is: USB 3.2's 13 are hub port-feature selectors, the SMMU
software guide's 3 are register names, ADIv6's are `IMPLEMENTATION_DEFINED` boilerplate, AMBA DTI's
are message names. A shape-only rule declares every one of them a wire.

The direction test uses the declaration reader's own closed vocabulary — `input`/`output`/`inout`,
full words only, because `[[direction-column-drift]]`'s tree measured the abbreviations at 0 true
positives and 18 false ones — matched **whole-word**, so a section about *"the outputs"* states no
wire's sense. The body is the heading's **own** elements in reading order.

## Adjudicated in full: 27 admitted, all correct; 5 missed, all real

Every admitted heading is a Wishbone wire whose body says so outright (*"The clock input [CLK_I]…"*).
**Precision 27/27, zero fabrications, zero admitted anywhere else in the corpus.**

The five refusals are real wires and a recall miss, for one cause: four name the **type**
(*"Address tag type [TGA_O()]…"*, *"Cycle tag type…"*) and `STALL_O` says *"the pipeline stall
**signal**"* where its siblings say *input* or *output*. **27 of 32 = 84% recall at 100% precision.**

## What shipped: 25, because a declaration needs a sense of its own (`.3`, `2026-09-20`)

The census measured a **selection**; the rule has to emit a **declaration**, and asking *which*
direction the body states changed the answer. Two of the 27 — `TGD_I()` and `TGD_O()` — reach a
direction word only inside a phrase about a **different** signal: *"…associated with the data
**input** array `[DAT_I()]`"* is `DAT_I`'s sense, not `TGD_I`'s. Both are real wires, so 27/27 stands
as an *existence* result; but a declaration rule built on a body-wide search attaches a neighbour's
direction, and it looks correct only because the `_I` suffix agrees.

**The shipped rule binds the direction to the name** — the direction word immediately precedes the
mention, with at most one intervening array noun (*"the data output array `[DAT_O()]`"*). That is the
same head-versus-window boundary `EXTRACTION-GAP-FIX.1` measured at the parenthetical-abbreviation
seam. Measured over all 78 documents: **25 headings in 1 document, 0 elsewhere, all 25 directions
correct.** The empty-catalog census went **33 → 32** and Wishbone's `IntentIR` gained an explicit
document interface of exactly those 25 wires.

**The `_I`/`_O` judgement was REFUSED** — see [[direction-suffix-is-one-documents-convention]] for the
1,202-signal control that settled it. Recall is **25 of 32**, and the 7 refusals stay honest misses.

Links: [[empty-signal-catalog-is-mostly-honest-absence]], [[direction-column-drift]],
[[direction-suffix-is-one-documents-convention]].
