# FACT-CARD-CAPACITY-HEADROOM: restore headroom before the fact plane refuses new knowledge

## Metadata

- Tree ID: `FACT-CARD-CAPACITY-HEADROOM`
- Status: `active` (`.0` pending)
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
193 cards. Consumption is not theoretical: a single corpus refresh slice routinely writes one or two cards, so
two ordinary slices can exhaust the remaining four. Failure is not graceful — `KNOWLEDGE-MAP` is a `gate`-tier
doctrine, so the pre-commit hook refuses the commit that writes the 201st fact.

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
  Status: `pending`
  Goal: pin the exact current pressure across both authorities, enumerate every reader and writer of each
  bound, classify the existing cards by lifecycle (current / superseded / candidate for merge), and accept an
  ADR that decides the mechanism before any limit or file changes.
  Acceptance: `an ADR is accepted; the census names every card, decision record, and consumer of max_facts/max_cards; no contract value or generated surface is edited in this leaf`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FACT-CARD-CAPACITY-HEADROOM.0` | `pending` | nothing may change a bound before the pressure and its readers are measured |

## Decisions

- `2026-08-10`: track this as its own tree rather than reopening the closed `FACT-CARD-CATALOG-CONTAINMENT`.
  That tree solved the *browse plane* (a monolithic landing) and closed on its own evidence; this is a
  different problem — the *capacity* of the fact plane itself — and it binds on a contract that tree does not
  own (`max_facts`, in the knowledge-map shard contract).

## Open Questions

- Is the honest fix more capacity, less duplication, or both? A card census may find superseded or
  near-duplicate cards whose merge restores headroom without widening any bound. `.0` answers this with data.
- Should `answers:`-bearing decision records count against the same maximum as cards? They are facts for
  retrieval purposes, which is why they count today; whether they should also consume *card* capacity is a
  question for the ADR.

## Blockers

- None. The frontier is executable now, and the pressure is not yet a failure.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-10` | ownership | `perl scripts/check_fact_card_catalog.pl --report`; `doctrine/knowledge_map/shard_contract.json` | 193 cards / 196 facts; `max_cards` 198, `max_facts` 200; landing at 87.5% of health with an active warning |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FACT-CARD-CAPACITY-HEADROOM` ownership | `FACT-CARD-CAPACITY-HEADROOM — track measured fact-plane capacity pressure` | surfaced by `CORPUS-CHAIN-CURRENCY.2` |
| `FACT-CARD-CAPACITY-HEADROOM.0` | `pending` | `pending` |

## Changelog

- `2026-08-10`: Created task tree from pressure measured while `CORPUS-CHAIN-CURRENCY.2` added a fact card.
