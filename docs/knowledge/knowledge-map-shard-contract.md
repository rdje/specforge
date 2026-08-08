---
id: knowledge-map-shard-contract
title: The Knowledge Map shard migration is locked by a bounded executable contract
answers:
  - "how will the million-byte Knowledge Map be sharded without losing question retrieval"
  - "what prevents one Knowledge Map question from pointing to multiple fact cards"
  - "what identifies the canonical inputs to generated Knowledge Map shards"
  - "why did the Knowledge Map shard simulator and generator report different canonical input hashes"
date: 2026-08-08
status: current
tags: [knowledge-map, generated-projection, sharding, retrieval]
evidence: doctrine/knowledge_map/shard_contract.json; docs/decisions/0009-bounded-knowledge-map-projection-set.md
reverify: perl scripts/check_knowledge_map_shard_contract.pl --check
---

ADR 0009 preserves `KNOWLEDGE_MAP.md` as a bounded landing path and moves full question retrieval into
deterministically packed `docs/knowledge-map/questions-NNNN.md` shards. Each UTF-8 byte-sorted question
entry links exactly one canonical fact, omits repeated fact metadata, and wraps under fixed physical-
line limits. The landing page links every shard and the separate fact-card catalog.

The executable contract rejects duplicate question destinations, unsafe or oversized inputs, unstable
ordering, too many facts/questions/shards, and independent landing/shard/aggregate line, byte, and
line-width overflow. It also computes a SHA-256 identity over every participating canonical fact path
and content. The contract and simulation gate landed in `.5d.i`; `.5d.ii` implements the generator
and exact output topology with reader-complete staging, derive-and-diff, and obsolete-shard cleanup.

The `.5d.ii` integration found and closed an identity-oracle defect: Perl `decode_utf8` with a check
flag may consume its input scalar, so hashing that scalar afterward produced the empty-content hash
for every source. The simulator now hashes raw bytes before decoding a copy, a ninth regression case
locks content sensitivity, and its identity exactly matches the portable generator's landing value.
