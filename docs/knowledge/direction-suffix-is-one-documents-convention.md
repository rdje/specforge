---
id: direction-suffix-is-one-documents-convention
title: An _I/_O name suffix is one document's convention, not digital-design grammar — 1,202 independently declared directed signals across 30 documents carry the suffix zero times
answers:
  - "may a signal name suffix imply its direction"
  - "can I read _I and _O as input and output"
  - "is an _IN/_OUT suffix universal digital-design grammar"
  - "does ADR 0006 allow a suffix-to-direction mapping"
  - "how do I decide whether a naming convention is universal grammar or one document's habit"
  - "why does the heading-as-declaration rule stop at 25 of 32 Wishbone wires"
  - "what independent authority declares a signal's direction in this corpus"
  - "how many corpus signals carry a table-declared direction"
date: 2026-09-20
status: current
tags: [adr-0006, genericity, signal-direction, census, signal-catalog-capture-gap, method]
evidence: docs/research/heading-as-declaration-shape.md (the `.3` section); docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md (`.3`); docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md
reverify: "scan every generated/evidence_ir/*/evidence_ir.json for statements matching '^Signal <NAME> is (input|output|inout)\\.$' and count how many NAMEs end in _I/_O/_IN/_OUT — expect 1202 such declarations across 30 documents and 0 with a direction suffix"
---

`SIGNAL-CATALOG-CAPTURE-GAP.2` left one judgement to the leaf that would land its rule: Wishbone's
five missed wires all carry their direction in the name's `_I`/`_O` suffix, and reading it would take
recall from 27/32 to 32/32. Is that **universal digital-design grammar**, which ADR 0006 permits, or
**one document's naming convention**, which it forbids?

**It is decidable by measurement, and the measurement is one-sided.**

The corpus already holds an independent authority on direction: signal-description tables, which
declare a sense without reference to any name shape. Across all 78 persisted `EvidenceIR` artifacts:

| | |
| --- | ---: |
| signals with a table-declared direction | **1,202** |
| documents contributing them | **30** |
| …that carry an `_I`, `_O`, `_IN` or `_OUT` suffix | **0** |

GIC-600 writes `SAMPLE_REQ` for an input and `SAMPLE_ACK` for an output. MMU-700 and AXI name 347
directed signals each and use the convention in none of them. Wishbone's section headings are the
only place in the corpus it appears at all.

## Why this is the right test for an ADR 0006 question

ADR 0006 admits exactly one naming-shaped thing — logic levels — and states the reason: they are
*"assumed by every digital specification and defined by none."* That is a claim about universality,
so the way to settle a candidate is to ask the corpus how universal it is, not to argue from how
natural it looks. `_I`/`_O` is assumed by **one document in 78** and corroborated by **none** of the
1,202 independent declarations, so it fails the test the ADR itself sets.

**Decided (`SIGNAL-CATALOG-CAPTURE-GAP.3`): the suffix is not read.** The heading-as-declaration rule
ships at 25 of 32 and the remaining 7 stay honest misses, per this repository's standing guardrail
that you cannot extract what is not there.

## The trap this closes

The suffix is dangerous precisely because it *agrees*. `.3` found that a body-wide direction search
attributes `DAT_I`'s sense to `TGD_I` — a borrowed direction that looks correct only because
`TGD_I`'s suffix happens to match. A rule can be right for a reason that is not evidence, and a
convention that correlates with the truth on one document is the easiest way to build one.

Links: [[heading-as-declaration-needs-a-direction-bearing-body]], [[direction-column-drift]].
