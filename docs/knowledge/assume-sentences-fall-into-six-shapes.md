---
id: assume-sentences-fall-into-six-shapes
title: Every remaining assume-sentence that IntentIR files as an assumption is one of six shapes, and one classifier is why
answers:
  - "why does IntentIR carry so many assumptions that are not design intent"
  - "what shapes do the unread assumed_by_word records take (convention, diagram_convention, narrative, treated_as, stated_default, other)"
  - "where does the ExplicitAbstraction class come from / which words make a statement an abstraction"
  - "why is 'LPIs are always treated as Non-secure Group 1 interrupts' an assumption"
  - "why is a timing-diagram key sentence an IntentIR assumption"
  - "how do I re-measure the assume-sentence shapes (measure_unread_intent_records.py, shape)"
  - "what are EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.i, .b.ii and .b.iii"
date: 2026-10-03
status: current
tags: [intent-ir, assumptions, abstraction, classification, census, measured, extraction-representation-gaps]
evidence: scripts/measure_unread_intent_records.py (SHAPES, shape); crates/specforge/src/ir/evidence.rs (classify_statement, the ExplicitAbstraction arm); crates/specforge/src/ir/semantic.rs (build_abstractions); crates/specforge/src/ir/intent.rs (build_assumptions); docs/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.ii.b)
reverify: "python3 scripts/measure_unread_intent_records.py | rg 'assumed_by_word|shapes'   # 2026-10-03: 106; treated_as 31, narrative 30, stated_default 25, diagram_convention 10, convention 6, other 4"
---

`classify_statement` in `crates/specforge/src/ir/evidence.rs` files any sentence holding *assume*, *assuming*,
*modeled as*, *abstracted* or *treated as* as `StatementClass::ExplicitAbstraction`, and that arm is tried before
every other class. `build_abstractions` in SemanticIR lifts every such statement to an `AbstractionRecord`, and
`build_assumptions` in IntentIR makes each one an `IntentAssumption`. So an IntentIR assumption is not a class of
intent; it is the set of sentences that contain one of five words (`.3.d`, `2026-09-30`).

**The population, measured `2026-10-03`** (`EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b`). After the five absent values
were bound (`.3.d.ii.a`, `.a.i`, `.a.ii`), 106 such records remain on the 31 chains, and
`scripts/measure_unread_intent_records.py` now gives each a shape, first match wins:

| shape | records | reads as | disposition |
| --- | ---: | --- | --- |
| `treated_as` | 31 | an equivalence rule the hardware applies, often under a condition | typed carrier (`.3.d.ii.b.ii`) |
| `narrative` | 30 | an example, a modelling premise, an analysis assumption, a *must not assume* for software | non-contract (`.3.d.ii.b.i`; the software obligations with `.3.d.i`) |
| `stated_default` | 25 | AXI's domain and flow defaults, USB4's lane default, IOMMU's fault value — and physical-link analysis premises and table notes | the defaults reach a reader (`.3.d.ii.b.iii`); the premises are non-contract |
| `diagram_convention` | 10 | *Shaded bus and signal areas are undefined, so the bus or signal can assume any value* | non-contract |
| `convention` | 6 | *This guide assumes that you are familiar with …* | non-contract |
| `other` | 4 | a fragment; *abstracted* as an adjective | non-contract |

**Outcome so far.** `.3.d.ii.b.i` (`2026-10-03`) files the convention, diagram-key, narrative and other shapes as
the `apparatus` statement class ([[an-assume-sentence-about-the-document-is-apparatus]]); the equivalence rules
and the genuine defaults remain abstractions until `.3.d.ii.b.ii` and `.3.d.ii.b.iii` carry them.

**What this fixes the direction of.** A rule stated with *treated as* is a conditional behaviour and a stated default
is a value; neither belongs beside a guide's prerequisite. The remedy is split by what each shape needs rather than
by removing the words from the classifier wholesale, because a reclassified sentence falls through to the other
classes in order (`DerivedRule`, `LocalDesignDecision`, `SignalValueConstraint`, …) and each child must measure
what those readers then take.

Links: [[a-signals-stated-absent-value-rides-its-presence-record]], [[untyped-uncited-tables-fall-into-fifteen-families]].
