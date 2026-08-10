# FACT-CARD-CAPACITY-HEADROOM: restore headroom before the fact plane refuses new knowledge

## Metadata

- Tree ID: `FACT-CARD-CAPACITY-HEADROOM`
- Status: `active` (`.0` done; `.1`/`.2` pending)
- Roadmap lane: repository durability and portability (sibling of `FACT-CARD-CATALOG-CONTAINMENT`)
- Created: `2026-08-10`
- Last updated: `2026-08-10`
- Owner: repo-local workflow

## Goal

Keep writing a Knowledge Map fact card possible. Two independent authorities bound the fact plane, and the
tighter one is now **four facts** from refusing the next card — while the working discipline
(`AGENTS.md`, `COMMIT.md`) tells every slice to write a card whenever it establishes a durable fact. Restore
deliberate headroom before a routine slice hits a hard gate failure it did not cause and cannot locally fix.

## Non-Goals

- Do not widen a ceiling merely to make the gate green; a limit is raised only with its own measured rationale,
  the way `ADR 0022`/`ADR 0023` tightened and re-derived these bounds.
- Do not delete or silently rewrite existing cards. Superseding is the documented lifecycle
  (`MEMORY_ARCHITECTURE.md` §10); compaction, if chosen, must preserve the audit trail.
- Do not change what a fact card *is*, or how the Knowledge Map derives its landing and shards.

## Measured pressure (`2026-08-10`, from `CORPUS-CHAIN-CURRENCY.2`)

Discovered while adding this session's retention card, not predicted from theory.

| Authority | Contract | Current | Maximum | Headroom |
| --- | --- | ---: | ---: | ---: |
| Fact count (cards + `answers:`-bearing decision records) | `doctrine/knowledge_map/shard_contract.json` `max_facts` | 196 | 200 | **4** |
| Card count | `doctrine/live_document_size/fact_card_catalog.json` `max_cards` | 193 | 198 | 5 |
| Landing lines | catalog `limits.landing.health_targets.lines` | 196 | 224 | 28 (87.5% — the checker already warns) |
| Title parts | catalog `limits.title_parts` | 4 parts / 193 cards | 4 parts / 224 slots | not binding |

The binding authority is therefore `max_facts`, and it counts three `docs/decisions/` records alongside the
193 cards. Failure is not graceful — `KNOWLEDGE-MAP` is a `gate`-tier doctrine, so the pre-commit hook refuses
the commit that writes the 201st fact, in whatever unrelated slice happens to write it.

### `.0` census (`2026-08-10`)

**Every bound is pinned in code as well as data**, so no single edit can widen one. `max_cards` is *derived*
(`scripts/check_fact_card_catalog.pl:380`) as the `knowledge_cards` surface file ceiling minus README and INDEX,
and both the file ceiling (`:374`) and the shard contract's `max_facts` (`:402`) are pinned to the literal 200.
`fixed_limits()` (`:97`) additionally pins `cards_per_part` 56, `max_parts` 4, and the landing's 224-line health
target / 256-line ceiling. Its self-test already covers fixed-limit inflation and question-capacity drift, so a
raise must move the pinned literals, their fixtures, and the data together.

**Landing growth is exactly linear**: 196 lines at 193 cards, i.e. `lines = cards + 3`, because the landing
carries one bare ID line per card. So the landing health target permits 221 cards, the 4 × 56 title-part slots
permit 224, and the 2,048-key question budget (1,439 keys today, 7.37 per card) would not bind until roughly
278. The binding structural join is therefore **221 cards**.

**Consumption is fast, and was measured rather than assumed** — cards created per active day, from Git:
23 (`2026-08-08`), 25 (`2026-08-09`), 20 (`2026-08-10`). Four facts is a few hours of ordinary work.

**Compaction is not a lever**: of 193 cards only 57 declare a `status:` at all, and exactly two are
`superseded`. There is no backlog of retired knowledge to reclaim.

[ADR 0026](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
records the resulting decisions: re-derive the bounds to the measured join now, stop the landing scaling with
cards, and never delete a card for capacity.

## Acceptance Criteria

- The binding authority is re-derived from measured evidence rather than raised by preference, in an ADR.
- Whatever mechanism is chosen (a re-derived maximum, a fifth title part, supersession compaction, or a
  combination) keeps every existing card and decision record reachable and byte-honest.
- `scripts/check_fact_card_catalog.pl`, `knowledge-map/scripts/check_knowledge_map.sh`, and
  `scripts/check_live_document_size.sh` pass with no pressure warning at the new state.
- The book's live-docs / doctrine-enforcement chapters and `FACT-CARD-CATALOG-CONTAINMENT`'s recorded bounds
  agree with the new authority before the tree closes.

## Task Tree

- ID: `FACT-CARD-CAPACITY-HEADROOM`
  Status: `active`
  Goal: restore deliberate headroom in the fact plane before it blocks an unrelated slice
  Children: `FACT-CARD-CAPACITY-HEADROOM.0`

- ID: `FACT-CARD-CAPACITY-HEADROOM.0`
  Status: `done` (`2026-08-10`, PROBE/DOC)
  Goal: pin the exact current pressure across both authorities, enumerate every reader and writer of each
  bound, classify the existing cards by lifecycle, and accept an ADR that decides the mechanism before any
  limit or file changes.
  Acceptance: `an ADR is accepted; the census names every bound, its pinned readers, the landing growth law, the measured burn rate, and the compaction surface; no contract value or generated surface is edited in this leaf`
  Verification: the census above; ADR 0026 accepted; nothing under `doctrine/`, `docs/knowledge/`, or
  `docs/knowledge-catalog/` changed in this leaf.
  Commit: `FACT-CARD-CAPACITY-HEADROOM.0 — measure the fact plane and decide its capacity law`

- ID: `FACT-CARD-CAPACITY-HEADROOM.1`
  Status: `pending`
  Goal: raise the four pinned authorities together to the measured join (ADR 0026 decision 1) — canonical file
  ceiling 223 so `max_cards` derives to 221, and `max_facts` 229 — moving the checker's pinned literals, its
  fixtures, the catalog contract, and the surface registry in one transaction, then regenerating the
  projection.
  Acceptance: `catalog, knowledge-map, and live-document gates pass with no pressure warning; no card content changes; every widened literal is the ADR's derived value and remains pinned`
  Verification: `pending`
  Commit: `pending`

- ID: `FACT-CARD-CAPACITY-HEADROOM.2`
  Status: `pending`
  Goal: stop the landing scaling with cards (ADR 0026 decision 2) — the landing becomes a bounded router over
  the title parts with each part's explicit ID range, so capacity grows by adding a part.
  Acceptance: `the landing is fixed-size and range-complete; every card is reachable in one extra deterministic hop; derive-and-diff, membership, and residue rules still hold; the book and FACT-CARD-CATALOG-CONTAINMENT bounds agree`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FACT-CARD-CAPACITY-HEADROOM.1` | `pending` | four facts of headroom against a measured 20–25 cards/day; the unblock is the only thing standing between an ordinary slice and a gate refusal |
| 2 | `FACT-CARD-CAPACITY-HEADROOM.2` | `pending` | `.1` buys about one working day; only the O(parts) landing removes the recurrence |

## Decisions

- `2026-08-10`: track this as its own tree rather than reopening the closed `FACT-CARD-CATALOG-CONTAINMENT`.
  That tree solved the *browse plane* (a monolithic landing) and closed on its own evidence; this is a
  different problem — the *capacity* of the fact plane itself — and it binds on a contract that tree does not
  own (`max_facts`, in the knowledge-map shard contract).
- `2026-08-10` (ADR 0026): re-derive the bounds to the measured structural join, make the landing O(parts)
  rather than O(cards), and never delete or merge a card to buy capacity.

## Open Questions

- Answered by `.0`: compaction is not a lever (two superseded cards out of 193), and the binding join is the
  landing's linear growth, not the file ceiling everyone would reach for first.
- Still open for `.2`: should a title part's ID range be published in the landing as literal first/last ids, or
  as a stable alphabetic partition rule? The literal range is exact but changes on every part rewrite; the rule
  is stable but needs its own derive-and-diff proof.
- Still open: `max_facts` counts `answers:`-bearing decision records alongside cards. ADR 0026 reserves eight
  slots for them (three exist). If decision records ever grow quickly, that reservation needs its own measure.

## Blockers

- None. The frontier is executable now, and the pressure is not yet a failure.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-10` | ownership | `perl scripts/check_fact_card_catalog.pl --report`; `doctrine/knowledge_map/shard_contract.json` | 193 cards / 196 facts; `max_cards` 198, `max_facts` 200; landing at 87.5% of health with an active warning |
| `2026-08-10` | `.0` census | catalog report, shard contract, `check_fact_card_catalog.pl` pinned literals, `git log --diff-filter=A` over `docs/knowledge/`, `status:` distribution | four authorities enumerated with their pinned readers; landing law `lines = cards + 3`; binding join 221 cards; burn 23/25/20 cards per active day; two superseded cards |
| `2026-08-10` | `.0` no mutation | `git status --short` | only the tree, ADR 0026, its index row, and the resume pointer changed; no contract, card, or generated projection touched |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FACT-CARD-CAPACITY-HEADROOM` ownership | `FACT-CARD-CAPACITY-HEADROOM — track measured fact-plane capacity pressure` | surfaced by `CORPUS-CHAIN-CURRENCY.2` |
| `FACT-CARD-CAPACITY-HEADROOM.0` | `FACT-CARD-CAPACITY-HEADROOM.0 — measure the fact plane and decide its capacity law` | measurement + ADR 0026 only |
| `FACT-CARD-CAPACITY-HEADROOM.1` | `pending` | `pending` |
| `FACT-CARD-CAPACITY-HEADROOM.2` | `pending` | `pending` |

## Changelog

- `2026-08-10`: Created task tree from pressure measured while `CORPUS-CHAIN-CURRENCY.2` added a fact card.
- `2026-08-10`: `.0` closed with the four-authority census; ADR 0026 accepted; `.1` and `.2` added.
