---
id: fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards
title: Fact-plane capacity is re-derived from its binding structural join, and the landing stops scaling with cards
date: 2026-08-10
status: accepted
scope: documentation, knowledge-map, generated-projection, containment, pressure, continuity
evidence: docs/tasks/FACT-CARD-CAPACITY-HEADROOM.md; scripts/check_fact_card_catalog.pl; doctrine/knowledge_map/shard_contract.json; doctrine/live_document_size/surfaces.jsonl
answers:
  - "what is ADR 0026"
  - "why is the fact-card catalog about to run out of capacity"
  - "how many fact cards can SpecForge hold"
  - "which limit binds first when adding a fact card"
  - "how is max_cards derived from the knowledge_cards surface"
  - "may fact cards be deleted or merged to free catalog capacity"
  - "why does the fact-card landing have one line per card"
---

# ADR 0026: Fact-plane capacity is re-derived from its binding structural join, and the landing stops scaling with cards

## Context

`FACT-CARD-CATALOG-CONTAINMENT` solved the *browse* problem: a monolithic catalog became a bounded landing over
count-packed title parts (ADR 0020–0023). It did not change how much the fact plane can hold, and
`CORPUS-CHAIN-CURRENCY.2` hit that second limit while adding one ordinary card.

Four authorities bound the plane, and they are deliberately pinned in *both* data and code so that no single
edit can widen them:

| Authority | Where | Value | Cards it permits |
| --- | --- | ---: | ---: |
| `max_facts` (cards + `answers:`-bearing decision records) | `doctrine/knowledge_map/shard_contract.json`, cross-checked `!= 200` at `scripts/check_fact_card_catalog.pl:402` | 200 | 197 today (3 records) |
| canonical file ceiling of `docs/knowledge/` | `doctrine/live_document_size/surfaces.jsonl` (`knowledge_cards`), cross-checked `!= 200` at `:374` | 200 | 198 (`max_cards = files − 2`, `:380`) |
| landing line budget | `fixed_limits()->{landing}` health 224 / ceiling 256 | 224 | 221 (measured `lines = cards + 3`) |
| title-part slots | `max_parts` 4 × `cards_per_part` 56 | 224 | 224 |

Measured on `2026-08-10`: **193 cards, 196 facts, 1,439 question keys** (7.37 keys per card, against a
`max_question_keys` of 2,048 that would not bind until roughly 278 cards). The landing is 196 lines, already at
87.5% of its health target, and the checker warns.

So the headroom is **four facts**, and the failure would not be graceful: `KNOWLEDGE-MAP` is a `gate`-tier
doctrine, so the pre-commit hook refuses the commit that writes the 201st fact — in whatever unrelated slice
happens to write it.

The consumption rate decides how urgent that is, so it was measured rather than assumed. Cards created per
active day, from Git: **23 (`2026-08-08`), 25 (`2026-08-09`), 20 (`2026-08-10`)**. Four facts is a few hours of
ordinary work, not two slices.

Compaction was measured too, and it is not a lever: of 193 cards only 57 declare a `status:` at all, and exactly
**two** are `superseded`.

## Decision

**1. Capacity is re-derived from the binding structural join, never raised by preference.** With today's
generated shapes the join is 221 cards — the landing health target (`lines = cards + 3` against 224) is tighter
than the 224 title-part slots, and both are tighter than the question-key budget. The canonical file ceiling
therefore becomes 223 (`max_cards = 221`), and `max_facts` becomes 229: 221 cards plus eight slots for
`answers:`-bearing decision records, a small, slow-growing set (three today). Every pinned cross-check moves to
its new derived value and stays pinned, so widening remains a deliberate, reviewed act rather than an accident.

**2. That is one day of headroom, so the landing must stop scaling with cards.** At the measured rate, 28
additional slots last about one working day. The landing lists one bare ID line per card, so it is O(cards) by
construction and every future raise has to re-derive it. It becomes a bounded router over the title parts —
which already carry each card's id, date, status, and title — naming each part's explicit ID range so a reader
still reaches any card deterministically, in one extra hop. Capacity then grows by adding a title part, and the
landing stays a fixed size forever.

**3. No card is deleted or merged to buy capacity.** Supersession stays the only lifecycle
(`MEMORY_ARCHITECTURE.md` §10); trading durable knowledge for headroom would defeat the point of having a
durable memory. The two superseded cards stay exactly where they are.

## Consequences

- The immediate unblock (`FACT-CARD-CAPACITY-HEADROOM.1`) changes four authorities together — the surface file
  ceiling, the checker's pinned cross-checks and `fixed_limits`, the catalog contract, and the shard contract —
  and regenerates the projection. No card content changes, and no bound is widened beyond the derived join.
- Until decision 2 lands, a capacity raise still has to touch the landing's line budget; afterwards it does not.
- The landing loses its one-hop ID → card property and gains a deterministic two-hop route. That is a real
  trade, accepted because the question shards (`KNOWLEDGE_MAP.md`) are the retrieval path agents actually use,
  and a 193-line ID list is not a browse surface a human reads.
- Capacity pressure becomes visible earlier: the catalog checker already warns at 80% of health, and that
  warning is now the intended early signal rather than a curiosity.

## Links

- Task tree: [`FACT-CARD-CAPACITY-HEADROOM`](../tasks/FACT-CARD-CAPACITY-HEADROOM.md)
- Predecessors: [ADR 0020](0020-bounded-fact-card-browse-projection.md),
  [ADR 0022](0022-fact-catalog-parts-pack-below-warning.md),
  [ADR 0023](0023-fact-catalog-landing-fits-full-capacity.md)
- Surfaced by: [`CORPUS-CHAIN-CURRENCY.2`](../tasks/CORPUS-CHAIN-CURRENCY.md)
