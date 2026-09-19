---
id: caption-repair-corpus-selection
title: The caption repair removes 71 admissions and adds 176 + 6 corpus-wide, loses no requirement, and the census had two populations where the rule has three
answers:
  - "what does the caption admission repair do to the whole corpus"
  - "how many invariant admissions does the caption repair remove"
  - "how many statements does the caption repair newly admit"
  - "does refusing a caption title lose a real requirement"
  - "why was was-not-permitted dropped from the negated permission form"
  - "why is the no-x-allowed window tightened to exclude commas"
  - "does the caption repair admit serialized table rows"
  - "what is the TileLink Figure 3.1 caption removal"
  - "is the caption repair shipped"
  - "what does shipping the caption repair still need"
  - "how many statements and captions does the persisted corpus hold"
  - "why does the caption repair census have a third stratum"
  - "how many captions does R3 newly admit"
  - "does a caption route r1 refuses ever get admitted by the caption repair"
  - "which caption additions are table-reading descriptions rather than requirements"
date: 2026-09-19
status: current
tags: [invariant-shape-admission, semantic-ir, caption, admission, census, adjudication, bounded-decision-provider]
evidence: scripts/measure_caption_admission_repair.py; docs/research/caption-admission-repair-census.md; docs/research/bounded-decision-arm-b.md; docs/tasks/INVARIANT-SHAPE-ADMISSION.md (.6); crates/specforge/src/ir/semantic.rs (is_invariant_like, statement_is_a_caption)
reverify: "python3 scripts/measure_caption_admission_repair.py --self-test — expect 15/15, pinning 71 removals (15 title / 56 cross-reference), 176 non-caption additions (168 not_permitted / 8 no_x_allowed, 38 of them serialized table rows) and 6 caption additions. A corpus rebuild that moves those counts fails loudly and the adjudication must be redone."
---

`BOUNDED-DECISION-PROVIDER.1a.1` scored a deterministic repair of `is_invariant_like`'s caption
handling on four documents. `INVARIANT-SHAPE-ADMISSION.6` put the same rules to **all 78 persisted
documents — 261,508 statements, 13,136 caption-shaped** — because a cheap structural rule over-fires
until its selection is read ([[a-cheap-structural-rule-overfires-until-you-read-its-selection]]).
**Nothing is shipped**; `is_invariant_like` is untouched.

| half | rows | composition |
| --- | ---: | --- |
| removals (precision) | **71** | 15 titles, 56 cross-references |
| additions (recall) | **176** | non-caption: 168 `is/are not permitted`, 8 `no … is/are allowed`; 38 are serialized table rows, 138 prose; 27 documents, 129 distinct texts |
| caption additions (recall) | **6** | captions route `r1` refuses whose SECOND sentence R3 admits; 5 documents. Added by `.6a.1` — the first census measured two cells of a two-by-two and `continue`d past this one, and the rule was unit-tested on the shape while no corpus row of it was enumerated. **4 are real prohibitions** (*"Other combinations are not permitted."*, *"The bit combinations that Table 3-7 does not show, are not permitted."*, ADIv6's *"No additional SWDIOTMS LOW cycles are allowed."* twice); **2 are table-reading descriptions** whose main verb is `excluded`/`indicate`, admitted and named rather than refused, because two rows out of 261,508 are not a grammar |

## The precision half costs no requirement

Sixty-nine removals carry no self-contained requirement — a label, or a sentence whose main verb is
`shows`/`lists`/`summarizes` and whose deontic word describes the referent. **Two do**: TileLink
`1.7.1` and `1.8.0` both caption a figure *"Figure 3.1: Valid must be driven LOW for at least 100
cycles during reset"*, which is a finite clause, and R1's proxy (*no sentence terminator, therefore a
title*) refuses it. The disposition is what matters: **each document states the same rule in prose**
— *"Before deasserting reset, a valid, c valid, and e valid must be driven LOW by the master…"* —
which route `r1` admits on `must`, so the requirement survives in both. The proxy's blind spot is
recorded as a known limit rather than repaired: one distinct sentence in two editions is not a
grammar.

## The corpus narrowed the recall rule three times

1. **`was`/`were` dropped.** The past-tense arm admits exactly two rows corpus-wide, both *"Prior to
   Issue G, … were not permitted"* — a superseded edition's rule, which is document history.
2. **The negated-existential window tightened** from `[^.]{0,80}` to `[^.,;:]{0,60}`. The wide form
   matched once across a clause break — SMMU's *"No\_snoop == 1 flag, it indicates that the
   transaction **is allowed** to 'opt-out'"* — where the `no` is part of a signal name and the
   permission is **granted**. A negated subject and its verb share one clause.
3. **Serialized table rows counted as their own stratum, not refused.** Route `r1` already admits a
   table row carrying `must`, and `INVARIANT-SHAPE-ADMISSION.0` measured 769 such rows and kept them
   because 699 carry content found nowhere else. The 38 enter on the same footing; they are separated
   so the recall change is not read as 176 sentences of prose when 138 of it is.

Two residual classes are **named rather than fixed**, because no shape rule separates them without
refusing real requirements: about 4 revision-history entries (*"| Correction: Use of SnpDVMOp is not
permitted |"*), and subjectless bullet continuations (*"- Is not permitted to send DVMReq…"*), whose
subject sits in the preceding bullet — that is the tree's separate subject question.

## Shipping still needs all of this

No production code changed; the golds are not re-scored; and
`INVARIANT-SHAPE-ADMISSION.1` warns a caption-route change rebuilds the chain for most documents,
a cost that is unchanged and unpaid. `BOUNDED-DECISION-PROVIDER.1a.1`'s published scores are
deliberately unaffected — its producer is pinned evidence and is left alone, the narrowings live in
the shipping census, and a RED case asserts both rows its frozen set depends on still admit under the
narrowed forms. See [[local-repair-closes-the-caption-decision]].
