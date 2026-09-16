---
id: fact-plane-capacity-is-funded-not-declared
title: The fact plane's declared capacity is funded by a rounding artifact, not by the measured ratio
answers:
  - "what actually binds the SpecForge fact plane"
  - "is the portable 4096 question-key cap the binding authority on the knowledge map"
  - "why is an eighth fact-card title part infeasible"
  - "how many facts does the knowledge map question-key budget actually fund"
  - "why does max_question_keys land on 4096 exactly"
  - "what warns before the knowledge map refuses on facts or question keys"
date: 2026-09-17
status: current
tags: [knowledge-map, capacity, live-document-size, fact-cards]
evidence: doctrine/knowledge_map/shard_contract.json; .knowledge_map.conf; scripts/check_knowledge_map_shard_contract.pl; docs/tasks/live-document-pressure-headroom/current-and-open-work.md
reverify: perl scripts/check_knowledge_map_shard_contract.pl --report
---

`max_question_keys` is **derived**, not chosen: `check_fact_card_catalog.pl` computes it as
`max_facts x 8` rounded up to a 512-key quantum. At `max_facts: 449` that is `3,592 -> 4,096`, which
lands exactly on the portable bundle's 4,096-key hard cap. The eight-keys-per-fact constant in that
derivation is **below what a fact costs**: the measured population is 2,767 keys over 327 facts, so a
fact carries about 8.46 questions and the ratio has risen over the projection's life (6.93 -> 8.49 mean
across 494 revisions of `KNOWLEDGE_MAP.md`). The declaration is therefore funded by the quantum's
rounding slack, not by the ratio it names.

That distinction decides which authority binds. A 4,096-key budget funds `int(4096 x 327 / 2767)` =
**484 facts**. `KM_MAX_FACTS` is 449, inside that, so **the repository's own fact cap binds first** and
the portable key cap is not the plane's binding authority — a claim `.knowledge_map.conf` and
`LIVE-DOCUMENT-PRESSURE-HEADROOM.24a` both made and `.28` corrected. What binds ahead of both is the
**card** plane: 304 cards of the 392 slots `max_parts: 7 x cards_per_part: 56` declares.

An eighth title part is still refused, for the funded reason rather than the declared one: it raises
`max_cards` to 448 and `max_facts` to `448 + 57` = **505**, which is 21 facts above the 484 the budget
funds. The contract itself would accept it — `505 x 8` still rounds to 4,096, and 505 is under the
portable 512-fact cap — so nothing in the derivation refuses the raise; the projection would refuse
later, at whatever commit crossed 4,096 actual keys. The lever that makes an eighth part reachable is
the **ratio** (it must fall to 4096/505 = 8.11 or below), never a cap raise.

`max_facts` and `max_question_keys` were the only capacity authorities in the repository that refused
with no band: a file, line, or byte dimension is banded by `doctrine/live_document_size/surfaces.jsonl`
and a bounded-registry record count by the central discovery over `doctrine/**/*.jsonl`, but a fact and
a question key are neither, so both fell between the two mechanisms. `.28` gave them the same 80/90
band every surface uses and made the funded count a reported figure, so a raise into the region the
rounding hides is visible in the commit that declares it. `max_shards` is deliberately left unbanded:
the `knowledge_map` surface already bounds the projection at 33 files = 1 landing + 32 shards.
