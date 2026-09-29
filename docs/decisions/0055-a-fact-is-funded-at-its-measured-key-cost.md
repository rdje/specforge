---
id: a-fact-is-funded-at-its-measured-key-cost
title: A fact is funded at its measured key cost, so the Knowledge Map bundle's fact and key caps move and the plane grows to nine title parts
date: 2026-09-29
status: accepted
scope: knowledge-map, fact-plane, capacity, live-document-size, doctrine
evidence: docs/tasks/live-document-pressure-headroom/warning-assignment.md (LIVE-DOCUMENT-PRESSURE-HEADROOM.43); scripts/check_fact_card_catalog.pl; scripts/check_knowledge_map_shard_contract.pl; .knowledge_map.conf; doctrine/live_document_size/surfaces.jsonl
answers:
  - "why are there ten question keys per fact in the Knowledge Map budget"
  - "why did the Knowledge Map bundle's 512-fact and 4,096-key caps move"
  - "how many fact cards can the fact plane hold"
  - "how many decision records can docs/decisions hold"
  - "how is max_shards derived for the Knowledge Map projection"
  - "what is LIVE-DOCUMENT-PRESSURE-HEADROOM.43"
---

# ADR 0055: A fact is funded at its measured key cost

## Context

On `2026-09-29` three fact-plane axes stood at rollover or warning with no owner
(`LIVE-DOCUMENT-PRESSURE-HEADROOM.43`): 385 of 394 card files (383 of 392 cards), 414 of 449 facts and 3,616 of
4,096 question keys. Cards arrive at about one per code slice: 305 files on `2026-09-16`, 385 thirteen days later.

ADR 0029 funds question keys at eight per fact, the ratio of the bundle's 512-fact / 4,096-key hard caps. A fact
costs more: 7.33 keys when ADR 0029 was written, 8.47 at `.24a`, **8.73** today (3,616 keys over 414 facts). So
the declared fact count is reachable only through the 512-key quantum's rounding slack, and `.24a` recorded seven
title parts as the last raise the bundle permits: an eighth part declares more facts than 4,096 keys fund.
ADR 0026 decision 3 rules out merging cards to buy capacity. The remaining lever is the one ADR 0029 named: past
roughly 470 cards, the bundle's caps must move first.

## Decision

**1. A fact is funded at its measured key cost.** The keys-per-fact constant is the measured cost funded at the
90% rollover band, `ceil(8.73 / 0.90) = 10`, and `max_question_keys` stays `max_facts ×` that constant rounded up
to the 512-key quantum. `scripts/check_fact_card_catalog.pl` states the constant once
(`question_keys_per_fact`) and derives every key bound from it, fixtures included.

**2. The bundle's fact and key caps move to 1,024 facts and 10,240 keys**, ten keys per fact.
`scripts/check_knowledge_map_shard_contract.pl` keeps every other hard cap: 64 shards, 32,768 projection lines and
2,097,152 projection bytes. At the measured density (2.04 lines and 190 bytes per key) 10,240 keys fit in 41
shards of the 512-line hard bound.

**3. The profile is re-derived by the laws already in force.** ADR 0029's card law (population below 80%, one
peak day of 25 cards below 90%) needs 479 slots, so `max_parts` 7 → 9 and `max_cards` 504, with 506 card files.
ADR 0041's decision law, applied with this record written (57 files, peak day 9), needs 74 files, hence 73
answers-bearing slots. So `max_facts` = 504 + 73 = **577** and `max_question_keys` = 5,770 rounded up = **6,144**.
The projection must hold that budget: at 147 keys per 300-line shard, `max_shards` = `ceil(6,144 / 147)` =
**42**. That gives 16,224 lines and 2,072,576 bytes, within the unchanged hard caps.

**4. ADR 0026 decision 3 and ADR 0041's stable paths are carried forward.** No card or decision record is edited,
merged or moved to buy capacity.

## Consequences

- Cards stand at 76.0% of 504 slots, 81.0% after a peak day. Facts are at 71.9% of 577, keys at 59.0% of
  6,144 and shards at 25 of 42. No fact-plane axis warns.
- The profile is re-proved as one transaction. Four single-use `ceiling_increase_authorities` records
  (`knowledge_cards`, `fact_card_titles`, `decision_records`, `fact_index`) carry the raise;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.43a` retires them in the next commit.
- The next binding cap is the bundle's projection bytes. Forty-two shards declare 2,072,576 of its 2,097,152
  bytes, so 42 is the last shard count the 49,152-byte per-shard bound allows. A 43rd shard, about 617 facts at
  ten keys each, needs that per-shard bound lowered toward the 31,577 bytes the largest shard measures today, or
  the projection cap moved by its own record.
- If the measured key cost passes ten, the funded count reported by
  `check_knowledge_map_shard_contract.pl --report` falls below `max_facts`. The constant is then re-derived by the
  same rule, never widened by hand.
