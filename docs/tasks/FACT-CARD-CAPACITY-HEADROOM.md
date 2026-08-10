# FACT-CARD-CAPACITY-HEADROOM: restore headroom before the fact plane refuses new knowledge

## Metadata

- Tree ID: `FACT-CARD-CAPACITY-HEADROOM`
- Status: `active` (`.0` done; `.1` superseded; `.2`/`.3` pending)
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
carries one bare ID line per card. The 4 × 56 title-part slots permit 224 cards, and the 2,048-key question
budget (1,439 keys today, 7.37 per card) would not bind until roughly 278 — so the landing is the binding
dimension.

**How binding, exactly** (corrected by ADR 0027 after `.0` first compared against the health target instead):
the checker treats **≥ 90% of a health target as a mandatory-rollover error**, not a warning
(`pressure_findings`, `:1061`). At 198 cards the landing is 201/224 lines = 89.7% — inside by 0.3 points — and
at 199 it is 90.2% and fails. An existing regression asserts exactly that (`:1364`, `:1373`), and setting
`max_cards` to 199 reproduces it. **198 is therefore not a chosen limit but the exact maximum this projection
shape permits**, and no bound can be raised until the shape changes.

**Consumption is fast, and was measured rather than assumed** — cards created per active day, from Git:
23 (`2026-08-08`), 25 (`2026-08-09`), 20 (`2026-08-10`). Four facts is a few hours of ordinary work.

**Compaction is not a lever**: of 193 cards only 57 declare a `status:` at all, and exactly two are
`superseded`. There is no backlog of retired knowledge to reclaim.

[ADR 0026](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
records the census and two standing decisions — stop the landing scaling with cards, and never delete a card
for capacity. Its third (raise the bounds to a 221-card join) is superseded by
[ADR 0027](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md),
which shows there is no join to raise to until the projection is reshaped.

## Acceptance Criteria

- The binding authority is re-derived from measured evidence rather than raised by preference, in an ADR, and
  every raised literal stays pinned from both sides by an exact-boundary regression.
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
  Children: `FACT-CARD-CAPACITY-HEADROOM.0`, `.1` (superseded), `.2`, `.3`

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
  Status: `superseded` by `FACT-CARD-CAPACITY-HEADROOM.3`
  Goal: (withdrawn) raise the four pinned authorities to ADR 0026 decision 1's 221-card join.
  Reason: the premise is false. ADR 0027 shows 198 is already the exact maximum this projection shape permits —
  199 cards puts the landing at 90.2% of its health target, which the checker treats as a mandatory-rollover
  **error**, and an existing regression asserts the 199-card projection fails closed. Nothing can be raised
  before the shape changes, so this leaf's work moves behind `.2` and is re-scoped as `.3`.

- ID: `FACT-CARD-CAPACITY-HEADROOM.2`
  Status: `pending`
  Goal: stop the landing scaling with cards (ADR 0026 decision 2, promoted to the critical path by ADR 0027) —
  the landing becomes a bounded router over the title parts with each part's explicit ID range, so capacity is
  no longer bounded by a per-card landing line.
  Acceptance: `the landing is fixed-size and range-complete; every card is reachable in one extra deterministic hop; derive-and-diff, membership, residue, and exact-boundary rules still hold; the book and FACT-CARD-CATALOG-CONTAINMENT bounds agree`
  Verification: `pending`
  Commit: `pending`

- ID: `FACT-CARD-CAPACITY-HEADROOM.3`
  Status: `pending`
  Goal: re-derive the whole projection profile once the landing is fixed-size (ADR 0027 decision 3) — per-part
  lines, part totals, part count, the surface file ceiling that `max_cards` derives from, and the independent
  `max_facts` / `max_question_keys` authorities — each against its own 90%-rollover rule, in one transaction.
  Acceptance: `every raised literal is derived and re-pinned from both sides (a full-capacity projection crosses no mandatory pressure and one card beyond it fails closed); no card content changes; catalog, knowledge-map, and live-document gates pass`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FACT-CARD-CAPACITY-HEADROOM.2` | `pending` | the landing's per-card line is the binding dimension; nothing can be raised until it stops scaling with cards |
| 2 | `FACT-CARD-CAPACITY-HEADROOM.3` | `pending` | the re-derived raise is only possible after `.2`, and only with both-sided boundary pins |

## Decisions

- `2026-08-10`: track this as its own tree rather than reopening the closed `FACT-CARD-CATALOG-CONTAINMENT`.
  That tree solved the *browse plane* (a monolithic landing) and closed on its own evidence; this is a
  different problem — the *capacity* of the fact plane itself — and it binds on a contract that tree does not
  own (`max_facts`, in the knowledge-map shard contract).
- `2026-08-10` (ADR 0026): re-derive the bounds to the measured structural join, make the landing O(parts)
  rather than O(cards), and never delete or merge a card to buy capacity.
- `2026-08-10` (ADR 0027, correcting ADR 0026 decision 1): there is no join to re-derive to. The checker treats
  ≥ 90% of a health target as a mandatory-rollover **error**, so full capacity at 198 cards (201/224 lines,
  89.7%) is already the exact edge, 199 fails closed by an existing regression, and a raise requires reshaping
  the projection first. Caught before any contract, card, or generated surface was edited.

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
| `2026-08-10` | `.0` census | catalog report, shard contract, `check_fact_card_catalog.pl` pinned literals, `git log --diff-filter=A` over `docs/knowledge/`, `status:` distribution | four authorities enumerated with their pinned readers; landing law `lines = cards + 3`; landing-derived join first read as 221 cards, corrected to the exact 198 by the next row; burn 23/25/20 cards per active day; two superseded cards |
| `2026-08-10` | `.0` no mutation | `git status --short` | only the tree, ADR 0026, its index row, and the resume pointer changed; no contract, card, or generated projection touched |
| `2026-08-10` | `.0` correction (ADR 0027) | set `max_cards => 199` in `fixed_limits()`, run `perl scripts/check_fact_card_catalog.pl --self-test`, restore | `199-card ceiling did not fail closed` — the raise is rejected by an existing exact-boundary regression; restored file is byte-identical, and 41/41 cases pass again |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FACT-CARD-CAPACITY-HEADROOM` ownership | `FACT-CARD-CAPACITY-HEADROOM — track measured fact-plane capacity pressure` | surfaced by `CORPUS-CHAIN-CURRENCY.2` |
| `FACT-CARD-CAPACITY-HEADROOM.0` | `FACT-CARD-CAPACITY-HEADROOM.0 — measure the fact plane and decide its capacity law` | measurement + ADR 0026 only |
| `FACT-CARD-CAPACITY-HEADROOM.1` | `superseded` | premise falsified by ADR 0027 before any implementation |
| `FACT-CARD-CAPACITY-HEADROOM.2` | `pending` | `pending` |
| `FACT-CARD-CAPACITY-HEADROOM.3` | `pending` | `pending` |

## Changelog

- `2026-08-10`: Created task tree from pressure measured while `CORPUS-CHAIN-CURRENCY.2` added a fact card.
- `2026-08-10`: `.0` closed with the four-authority census; ADR 0026 accepted; `.1` and `.2` added.
- `2026-08-10`: ADR 0027 corrected ADR 0026 decision 1 against the checker's mandatory-rollover rule; `.1`
  superseded, `.2` promoted to the frontier, `.3` added for the re-derived raise.
