---
id: fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy
title: Fact-plane capacity is one derived profile, and an aggregate bound with no remedy is a trap
date: 2026-08-11
status: accepted
scope: documentation, knowledge-map, generated-projection, containment, pressure, continuity
evidence: docs/tasks/FACT-CARD-CAPACITY-HEADROOM.md; scripts/check_fact_card_catalog.pl (fixed_limits, the derived-profile and 336/337 boundary self-tests); doctrine/live_document_size/surfaces.jsonl; doctrine/live_document_size/ceiling_increase_authorities.jsonl; doctrine/knowledge_map/shard_contract.json
answers:
  - "what is ADR 0029"
  - "how many fact cards can SpecForge hold now"
  - "how is fact-plane capacity derived"
  - "why is a fact-card aggregate ceiling the file bound times the per-file bound"
  - "when should a live-document capacity bound be raised"
  - "how do I add fact-card capacity"
  - "why did max_facts become 379"
---

# ADR 0029: Fact-plane capacity is one derived profile, and an aggregate bound with no remedy is a trap

## Context

[ADR 0027](0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md) decision 3
deferred one job to `FACT-CARD-CAPACITY-HEADROOM.3`: once the landing is fixed-size, re-derive the whole
profile in one transaction, each dimension against its own rule. Measuring for that re-derivation is what
found the blind authority [ADR 0028](0028-a-file-locator-means-one-file-and-aggregate-ceilings-must-warn.md)
fixed, so this is the first re-derivation performed with every authority visible.

Measured on `2026-08-11`:

| Authority | Contract | Measured | Bound | Pressure |
| --- | --- | ---: | ---: | --- |
| Card files | `knowledge_cards` `files` | 195 | 200 | 97.5% — rollover |
| Card aggregate lines | `knowledge_cards` `lines_total` | 9,773 | 10,000 | 97.7% — rollover |
| Card aggregate bytes | `knowledge_cards` `bytes_total` | 853,926 | 1,048,576 | 81.4% — warning |
| Decision-record files | `decision_records` `files` | 30 | 32 | 93.8% — rollover |
| Question-projection lines | `fact_index` `lines_total` | 2,976 | 3,072 health / 4,096 ceiling | 96.9% — rollover |
| Question-projection bytes | `fact_index` `bytes_total` | 267,938 | 262,144 health / 393,216 ceiling | **102.2%** — over health |
| Title parts | `fact_card_titles` `lines_each` | 63 | 80 | 78.8% — healthy (ADR 0022) |
| Landing | catalog `landing.health_targets.lines` | 10 | 224 | 4.5% — no longer a capacity dimension |

Two facts shape the decision.

**The declared capacity was not reachable.** `max_cards` was 198, but 198 cards at the measured mean of
50.5 lines per card need 10,026 aggregate lines against a 10,000-line ceiling. The plane would have refused
its own advertised last card, and the refusal would have landed on whichever unrelated slice happened to
write it.

**That aggregate had no legal exit.** [ADR 0026](0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
decision 3 forbids deleting or merging a card to buy capacity, and cards are `partitioned_canonical`, not a
rolling ledger, so there is no rollover. A total that 198 individually-legal cards can exceed is therefore a
state reachable by ordinary compliant writes with no compliant way out. `decision_records` (aggregate 4,000
lines against 32 × 512 = 16,384 legal per-file lines) had the same shape.

Growth was measured from Git rather than assumed. Cards created per active day over 20 active days: median 8,
90th percentile 20, peak 25. Decision records over 7 active days: peak 9.

## Decision

**1. Pressure belongs on a dimension that has a remedy.** For a canonical collection that may never be
deleted or rolled over, the only such dimension is the *count*: its remedy is to add capacity. The aggregate
line and byte bounds therefore become the file bound times the per-file bound, so no corpus of individually
legal files can ever be refused by a total no single file can see. The per-file bounds keep their own
warning bands and remain the real quality signal. `fact_card_titles` already satisfied this rule for lines
(320 = 4 × 80), which is where it was read off.

**2. Capacity is set from the measured population against the doctrine's own milestones.** A capacity bound
is chosen so the measured population is below the 80% warning milestone, and so one measured peak active-day
of growth still leaves it below the 90% mandatory-rollover milestone — then rounded up to the surface's
natural quantum. This makes the warning both true and actionable: it fires before the wall, with at least a
day of reaction time, and the reaction is a declared one-parameter change rather than an archaeology.

**3. The profile is one derivation, not a set of coincident literals.** `max_parts` is the only free
parameter. Everything else follows and is asserted mechanically:

| Quantity | Derivation | Value |
| --- | --- | ---: |
| `cards_per_part` | ADR 0022 — 56 + 7 scaffold lines = 63 of an 80-line health target | 56 |
| `max_parts` | decision 2 on the part-file dimension: 4 parts must stay under 80%, and 5 under 90% | 6 |
| `max_cards` | `cards_per_part × max_parts` | 336 |
| `knowledge_cards.files` | `max_cards + 2` (README, landing) — the existing `max_cards = files − 2` join | 338 |
| `knowledge_cards` aggregates | decision 1: `338 × 300` lines, `338 × 36,864` bytes | 101,400 / 12,460,032 |
| `fact_card_titles.files` | `max_parts` | 6 |
| `fact_card_titles` aggregates | decision 1, per band: `6 × 80` / `6 × 96` lines | 480 / 576 |
| `decision_records.files` | decision 2: 30 measured, peak 9 → `(30 + 9)/0.90` | 44 |
| `decision_records` aggregates | decision 1: `44 × 512` lines, `44 × 32,768` bytes | 22,528 / 1,441,792 |
| `max_facts` | `max_cards +` every record but the index (`files − 1`) | 379 |
| `max_question_keys` | `max_facts ×` the 8-keys-per-fact ratio the bundle's own hard caps declare (4,096/512), rounded to the registry's 512 step; measured usage is 7.33 | 3,072 |
| `fact_index` / projection aggregates | `landing bound + max_shards × shard bound` | 12,384 / 1,581,056 |

**4. The derivation is pinned from both sides.** The catalog self-test asserts the identities themselves —
`max_cards = cards_per_part × max_parts`, each aggregate band equals files × that band's per-file bound, and
the projection ceiling equals the landing ceiling plus the part aggregate — so a future raise cannot move one
literal and leave another behind. A full 336-card capacity render must cross no mandatory pressure and a
337th card must fail closed, replacing the old 198/199 pair. `max_facts` is no longer a pinned literal at all:
the catalog checker now derives it from `max_cards` plus the `decision_records` file ceiling and rejects any
other value.

**5. ADR 0026 decision 3 and ADR 0027 decision 4 are carried forward unchanged.** No card is deleted or
merged to buy capacity; every one of the 193 cards and 29 decision records is untouched by this change.

## Consequences

- Headroom is real again: 143 free card slots (195 of 338 files, 57.7%), 14 free decision-record slots
  (68.2%), and 181 free fact slots. Every fact-plane rollover warning is gone; the gate reports 733 files
  across 51 surfaces with no capacity pressure.
- Adding capacity is now one number. Raising `max_parts` re-derives `max_cards`, the surface file ceiling,
  both aggregate bands, `max_facts`, and the projection ceiling, and the self-test proves the set moved
  together.
- The aggregate ceilings are deliberately inert. `knowledge_cards` sits at 9.6% of its line total and 6.9% of
  its byte total, and can only approach them if the average card approaches the per-card ceiling — which the
  per-file warning would report first. That is the intended split: totals guarantee reachability, per-file
  bounds carry the signal.
- One warning survives on purpose: `knowledge_cards` `lines_each` is at 81.0% because
  `transaction-capture-census.md` is 243 lines against a 300-line per-card bound. That is a per-card quality
  signal with a local remedy (split or supersede that card), not capacity pressure, and this leaf changes no
  card content.
- The architecture's own ceiling is now visible: the portable Knowledge Map bundle hard-caps a project at 512
  facts, 4,096 question keys, and 64 shards. This profile uses 74% of the fact cap. A future raise past
  roughly 470 cards must change the bundle's caps first, which is a portable-architecture act, not a project
  one.
- Four `ceiling_increase_authorities` records carry the exact old/new ceiling objects for this transaction.
  They are consumed by the commit that raises the ceilings and retired immediately afterwards by
  `FACT-CARD-CAPACITY-HEADROOM.3a`, because an authority that outlives its increase is banked, and the
  containment checker rejects banked authorities.

## Links

- Task tree: [`FACT-CARD-CAPACITY-HEADROOM`](../tasks/FACT-CARD-CAPACITY-HEADROOM.md)
- Executes decision 3 of: [ADR 0027](0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
- Carries forward: [ADR 0026](0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
  decision 3, [ADR 0022](0022-fact-catalog-parts-pack-below-warning.md)'s 56-card part
- Made measurable by: [ADR 0028](0028-a-file-locator-means-one-file-and-aggregate-ceilings-must-warn.md)
