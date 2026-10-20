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
  - "what are the 21 remaining stated_default records / what are EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii.a, .b and .c"
  - "why is AXI's 'CMO transactions are assumed to be Non-shareable' not AWDOMAIN's absent value (it holds for a class of requests only)"
  - "does the AXI specification disagree with itself about a cacheable request's domain without domain signals (yes: statement_2337 says Non-shareable, statement_2360 says Shareable)"
date: 2026-10-19
status: current
tags: [intent-ir, assumptions, abstraction, classification, census, measured, extraction-representation-gaps]
evidence: scripts/measure_unread_intent_records.py (SHAPES, shape); crates/specforge/src/ir/evidence.rs (classify_statement, the ExplicitAbstraction arm); crates/specforge/src/ir/semantic.rs (build_abstractions); crates/specforge/src/ir/intent.rs (build_assumptions); docs/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.ii.b)
reverify: "python3 scripts/measure_unread_intent_records.py | rg 'assumed_by_word|shapes'   # 2026-10-19 after .3.d.ii.b.iii.b: 37; treated_as 31, stated_default 5, narrative 1 (after .3.d.ii.b.iii.a 50 and 18; before it 53 and 21; 2026-10-03, before .3.d.ii.b.i: 106; treated_as 31, narrative 30, stated_default 25, diagram_convention 10, convention 6, other 4)"
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

**The stated defaults, read one by one `2026-10-19`** (`.3.d.ii.b.iii`). 21 remain after `.3.d.ii.b.i`, and only 3
are an absent signal's default: AXI's `AxDOMAIN` defaults, each stated for a **class of requests** (*Non-cacheable
requests … System domain*, *Cacheable requests … Shareable domain*, *CMO transactions … Non-shareable*) and written
as a label of the signal's encoding (`.3.d.ii.b.iii.a`, bound the same day as class-scoped values,
[[an-absent-value-may-be-a-label-and-hold-for-a-class]]). 13 use an *assume* word without stating an abstraction —
premises the apparatus test misses, *assume* meaning *take on*, a negation, and IOMMU's two field values under a fault
(`.3.d.ii.b.iii.b`, done the same day: [[an-assume-sentence-about-the-document-is-apparatus]]). 4 are defaults whose subject is no presence-record signal (a flow, an interface state, a
removed signal, *Domain signaling*; `.3.d.ii.b.iii.c`), and one is a USB4 software procedure (`.3.d.i`). The AXI
document disagrees with itself: `statement_2337` puts a cacheable request in the Non-shareable domain when domain
signaling is absent, `statement_2360` in the Shareable domain.

**What this fixes the direction of.** A rule stated with *treated as* is a conditional behaviour and a stated default
is a value; neither belongs beside a guide's prerequisite. The remedy is split by what each shape needs rather than
by removing the words from the classifier wholesale, because a reclassified sentence falls through to the other
classes in order (`DerivedRule`, `LocalDesignDecision`, `SignalValueConstraint`, …) and each child must measure
what those readers then take.

Links: [[a-signals-stated-absent-value-rides-its-presence-record]], [[untyped-uncited-tables-fall-into-fifteen-families]].
