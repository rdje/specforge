---
id: fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound
title: Fact-plane capacity requires reshaping the projection, because 198 cards is already the exact maximum its shape permits
date: 2026-08-10
status: accepted
scope: documentation, knowledge-map, generated-projection, containment, pressure, continuity
evidence: docs/tasks/FACT-CARD-CAPACITY-HEADROOM.md; scripts/check_fact_card_catalog.pl (pressure_findings, the 198-card capacity and 199-card overflow self-tests); docs/decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md
answers:
  - "what is ADR 0027"
  - "why can the fact-card maximum not simply be raised from 198"
  - "what does mandatory rollover pressure mean for the fact-card landing"
  - "why is 198 exactly the largest fact-card maximum the current projection allows"
  - "does raising max_facts alone create fact-card headroom"
  - "which ADR 0026 decision was wrong and why"
---

# ADR 0027: Fact-plane capacity requires reshaping the projection, because 198 cards is already the exact maximum its shape permits

## Context

[ADR 0026](0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md) measured the fact
plane correctly — 193 cards, 196 facts, `lines = cards + 3`, 20–25 new cards per active day, two superseded
cards — and then derived its decision 1 from the wrong threshold. It compared full capacity against the
landing's 224-line **health target**, when the same checker treats **≥ 90% of health as a mandatory-rollover
error**, not a warning (`pressure_findings`, `scripts/check_fact_card_catalog.pl:1061`).

Applying the real rule to the measured growth law:

| Cards | Landing lines (`cards + 3`) | % of the 224-line health target | Verdict |
| ---: | ---: | ---: | --- |
| 198 | 201 | 89.7% | passes, by 0.3 points |
| 199 | 202 | 90.2% | mandatory rollover — error |
| 221 (ADR 0026 decision 1) | 224 | 100.0% | error |

So **198 is not a chosen limit; it is the exact largest maximum this projection shape permits**, which is what
[ADR 0023](0023-fact-catalog-landing-fits-full-capacity.md) derived when it reduced the landing scaffold. The
checker already pins that boundary from both sides: a full 198-card capacity projection must cross no
mandatory pressure, and a 199-card projection must fail closed (`:1364`, `:1373`). Setting `max_cards` to 199
was tried and does exactly that.

The title parts are tuned just as tightly: 56 cards render a 63-line part (78.8% of the 80-line each-file
health target, which is why [ADR 0022](0022-fact-catalog-parts-pack-below-warning.md) tightened 64 → 56), and
four such parts total 252 lines against a 320-line total target — 78.8% again. A fifth part of the same size
would reach 98% of that total and fail.

Raising `max_facts` alone buys nothing: cards would still stop at 198, because `max_cards` derives from the
`knowledge_cards` surface file ceiling and is independently bounded by the landing.

## Decision

**1. Supersede ADR 0026 decision 1.** There is no capacity to unlock by re-deriving a bound, because every
bound is already at the exact edge its shape allows. Any raise must change the *shape* first.

**2. The landing stops scaling with cards — as the unblock, not the follow-up.** ADR 0026 decision 2 stands and
becomes the critical path. The landing carries one bare ID line per card, so it is O(cards) by construction;
it becomes a fixed-size router over the title parts, which already carry each card's id, date, status, and
title, with each part's ID range named so any card is reachable in one extra deterministic hop.

**3. Capacity is then re-derived across the whole profile, in one transaction.** Once the landing is fixed-size,
the binding dimensions become per-part lines, part totals, part count, and the independent question-key and
fact authorities. They are re-derived together — each against its own 90%-rollover rule — and the exact-boundary
self-tests move with them, so the new maximum stays pinned from both sides exactly as 198/199 is today.

**4. ADR 0026 decisions 2 and 3 are carried forward unchanged**, including: no card is deleted or merged to buy
capacity.

## Consequences

- The frontier reorders: `FACT-CARD-CAPACITY-HEADROOM.1` is superseded, `.2` (the O(parts) landing) becomes the
  unblock, and a new `.3` performs the re-derived raise afterwards.
- Until `.2` lands, the fact plane genuinely cannot accept more than 198 cards, and the honest response to
  hitting it is to finish `.2` — not to widen a literal. The 199-card regression exists precisely to make that
  the only path.
- Recording this correction consumes one of the scarce fact slots it is about, which is itself evidence for
  decision 2's urgency.
- The correction was cheap because it was caught by checking the rule before implementing rather than after: no
  contract, card, or generated surface had been edited under ADR 0026 decision 1.

## Links

- Task tree: [`FACT-CARD-CAPACITY-HEADROOM`](../tasks/FACT-CARD-CAPACITY-HEADROOM.md)
- Supersedes decision 1 of: [ADR 0026](0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
- Derivation history: [ADR 0020](0020-bounded-fact-card-browse-projection.md),
  [ADR 0022](0022-fact-catalog-parts-pack-below-warning.md),
  [ADR 0023](0023-fact-catalog-landing-fits-full-capacity.md)
