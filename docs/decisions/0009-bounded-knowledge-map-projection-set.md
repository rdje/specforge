# 0009 — Knowledge Map uses a bounded landing page and generated question shards

- Date: 2026-08-08
- Status: accepted
- Deciders: project owner, SpecForge repository workflow

## Context

The question-keyed `KNOWLEDGE_MAP.md` is deterministic and current, but the committed `.5c.iii`
baseline is one generated file containing 138 facts / 978 rows / 977 unique questions at 2,231 lines /
1,038,010 bytes. Its question section is
about 796 KB because every row repeats date and reverify metadata; its fact section duplicates the
separate bounded fact-card catalog. The widest row is 7,778 bytes. One exact question also mapped to
two cards until the `.5d.i` collision audit assigned it to the focused card.

The projection must remain question-searchable, portable, generated, and free of unique facts while
obeying the repository's bounded-read and same-volume path contracts.

## Decision

`KNOWLEDGE_MAP.md` remains the stable landing path but becomes a bounded generated index. It links a
deterministic set of `docs/knowledge-map/questions-NNNN.md` shards and the separate
`docs/knowledge/INDEX.md` fact-card catalog.

Question rows are sorted in UTF-8 byte order, require globally unique question text, directly link one
canonical fact source, omit repeated date/evidence/reverify metadata, and wrap the preserved question
text under fixed physical-line limits. The generator packs whole entries into sequential shards under
independent line and byte bounds; the root records fact/question/shard counts and a SHA-256 identity
over the participating canonical fact files. The entire root-plus-shards set remains one generated
projection with derive-and-diff freshness and complete root membership.

The contract and its simulation gate land before the generator migration. The migration must atomically
write the complete new set, remove only obsolete generated shards, stage every output from the hook,
and update every bootstrap/bundle reader in the same slice.

## Consequences

- Ordinary readers open a small stable landing page; humans browse facts by id/title in the fact-card
  catalog and search questions with one repository-relative `rg` command across shards.
- Adding a fact can add or rebalance deterministic generated shards but cannot create an unbounded root,
  a duplicate question destination, an over-wide line, or unique generated prose.
- Canonical fact cards and participating decision records remain unchanged in role and location.
- `.5d.i` changes no generator output topology; `.5d.ii` performs the atomic migration and removes the
  current transition debt only after exact reconstruction, membership, reader, and gate proofs pass.

Implementation note (`2026-08-08`): `.5d.ii` landed this topology. The portable generator also
enforces an explicit 4,096-line / 393,216-byte aggregate projection bound, tightening the original
mathematical product of per-shard limits so unused part capacity cannot become banked growth.

## Links

- `doctrine/knowledge_map/shard_contract.json`
- `scripts/check_knowledge_map_shard_contract.pl`
- `knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md`
- `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md`
