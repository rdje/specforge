---
id: bounded-fact-card-browse-projection
date: 2026-08-09
status: accepted
scope: documentation, knowledge-map, generated-projection, retrieval, containment
---

# ADR 0020: Fact-card browsing uses a bounded ID landing over deterministic title parts

## Context

At commit `47e915409bbe6065544f86e49739b9e93c1ac75b`, `docs/knowledge/` contains 158 canonical
fact cards plus `README.md` and the generated `INDEX.md`. The generic `knowledge_cards` surface therefore has
160 of its 200 permitted immediate Markdown files and is exactly at its 80% file warning.

The focused catalog checker has two tighter assumptions that no longer compose with that collection contract:
it rejects more than 160 cards, and it renders every detailed row into one index capped at 32,768 bytes. The
current index is 172 lines / 32,634 bytes, Git blob `dcac9e587a26304e15eb15743b04142cb03c5081`, and
SHA-256 `e774d9fcc1b12f9bb83932789ef4268711a7756f491dfd2a492be375b3529d1b`. Its 158 rows range
from 132 to 261 bytes and average 204.7 bytes, leaving only 134 bytes under the focused byte limit.

The separate question projection has 159 facts / 1,136 keys across eight shards after recording this boundary.
It is already partitioned and derive-and-diff checked, but its 209,962 aggregate bytes have crossed the 80%
health warning. It remains below the 90% rollover and 393,216-byte hard ceiling. Browse containment must account
for that independent pressure without changing question semantics or conflating the two generated products.

Eleven current files name the stable catalog path and 13 unique files name that path or focused checker. README,
the generated Knowledge Map, the Knowledge Map contract, route registry, live-size registry, mdBook, ADR/fact/
research evidence, and the focused writer all depend on the stable path. None depends on a Markdown fragment.

## Decision

Keep `docs/knowledge/INDEX.md` as the stable bounded landing and split detailed browse rows into deterministic
generated title parts under `docs/knowledge-catalog/`.

### Stable landing

The landing remains inside the `knowledge_cards` surface and contains:

1. the generated/read-only notice and authoring/question-search routes;
2. one direct link to `docs/knowledge/README.md`;
3. one direct link to every title part; and
4. one direct ID link to every canonical card, sorted by UTF-8 bytes.

The direct ID list preserves the generic membership contract: the surface index still links every immediate card
and its README without relying on transitive interpretation. An ID lookup reaches its card in one hop. A title
browse reaches the matching detailed part and then the card in two bounded hops. The stable path means every
existing reader remains valid.

The root renderer uses one line `- [<id>](<id>.md)` per card. With the existing 64-byte ID limit, a direct route
is at most 137 bytes. At 198 cards those routes occupy at most 27,324 bytes including newlines; four bounded part
routes plus a fixed scaffold capped at 2,048 bytes keep the complete root under 32,768 bytes. The root contract is
224 lines / 32,768 bytes / 256 maximum content-line bytes, with inclusive ceilings of 256 / 32,768 / 320.

### Detailed title parts

Cards sort by ID and pack by count, 64 cards per part, into
`docs/knowledge-catalog/titles-NNNN.md`. Each part carries the existing detailed table columns and row rendering:
ID link, establishment date, status, and title preview. The migration therefore preserves the 158 current table
rows byte-for-byte as an ordered union while moving them out of the crowded landing.

The renderer permits at most 198 cards and four parts. A row remains capped at 320 bytes; a part has at most 64
rows plus a fixed scaffold capped at 1,024 bytes. The dedicated generated-projection surface uses these limits:

| Dimension | Health target | Inclusive ceiling |
| --- | ---: | ---: |
| files | 4 | 4 |
| lines each | 80 | 96 |
| bytes each | 24,576 | 32,768 |
| lines aggregate | 320 | 384 |
| bytes aggregate | 90,112 | 98,304 |
| maximum content-line bytes | 384 | 512 |

The combined landing-plus-parts contract permits at most five files / 512 lines / 122,880 bytes. These are new,
tighter projection controls; no existing surface target, warning/rollover milestone, or enforcement ceiling is
widened.

### Capacity authority

The canonical card maximum becomes a derivation, not an unrelated literal:

```text
knowledge_cards.files ceiling 200
- fixed collection README          1
- stable generated INDEX           1
= maximum canonical cards        198
```

Replacing the focused checker's premature `160` literal with this derived 198 maximum does not change the
authoritative 200-file collection ceiling or manufacture new collection capacity. The question projection's
200-total-fact limit remains independent because it also admits front-mattered decision records; a change must
satisfy both contracts, and neither borrows unused capacity from the other.

### Generated-state and write transaction

Canonical card files remain the only fact authority. The landing and title parts contain no unique facts and are
regenerated as one projection set. `--write` renders the complete expected set under a repository-local workspace,
validates it, replaces all expected parts, removes only stale files matching the dedicated
`titles-NNNN.md` namespace, and writes the landing last. The Git commit is the durable atomic boundary; a partial
working-tree rewrite must fail derive-and-diff and cannot commit.

The migrated checker validates constrained front matter, source fields, ordering, exact direct card and part
routes, exhaustive row union, part packing, root/part/aggregate bounds, dedicated-directory residue, and exact
derive-and-diff content. The generic live-size checker independently proves collection classification and both
the landing's internal card membership and its external membership over all title parts.

The old monolith is generated rather than canonical, so it receives no archive terminal. Its exact committed
Git/blob/SHA/metric identity and ordered row set remain migration provenance in the executable contract and Git;
the canonical cards can reproduce the result.

## Migration stages

1. `FACT-CARD-CATALOG-CONTAINMENT.2.1` lands the schema-closed contract and neutral checker in `legacy_locked`
   state. It pins the committed monolith, validates all 158 source rows and planned three-part rendering, rejects
   every destination part, and exercises both legacy and future migrated states without changing output.
2. `FACT-CARD-CATALOG-CONTAINMENT.2.2` writes the bounded landing plus three title parts, adds their dedicated
   generated surface, switches the contract to `migrated`, updates current readers/documentation, runs the full
   gates, and closes the program.

## Consequences

- Existing bootstrap and Knowledge Map links keep the stable browse path.
- Every current detailed row survives exactly once in bounded title parts; every card remains directly linked
  from the root for generic membership and ID retrieval.
- Ordinary additions can use the collection capacity already declared by the 200-file surface without reviving
  a monolithic browse bottleneck.
- Question search remains a separate sharded projection with its own current warning and hard bounds.
- Any future need beyond 198 cards, four title parts, or the fixed aggregate caps requires a new explicit
  architecture decision; unused per-part room cannot be banked into unbounded growth.

## Links

- `docs/tasks/FACT-CARD-CATALOG-CONTAINMENT.md`
- `docs/knowledge/INDEX.md`
- `docs/knowledge/fact-card-catalog.md`
- `docs/decisions/0009-bounded-knowledge-map-projection-set.md`
- `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`
