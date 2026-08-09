---
id: fact-catalog-landing-fits-full-capacity
date: 2026-08-09
status: accepted
scope: documentation, knowledge-map, generated-projection, containment, pressure
---

# ADR 0023: The fact-card landing scaffold fits the full declared capacity

## Context

The first migrated rendering had 20 fixed landing lines plus one direct-route line per canonical card. It was
safe for the current 158 cards at 178 lines, but not for the declared maximum: 198 cards would produce 218 lines,
97.32% of the accepted 224-line health target. The checker fails any dimension at the unchanged 90% mandatory-
rollover milestone, so this shape could admit at most 181 cards before contradicting the derived 198-card
capacity.

The earlier capacity proof established the byte ceiling but did not apply the pressure milestone to the maximum-
card line count. Raising the line target or ceiling would hide the contradiction. Packing multiple card links per
line would complicate ID scanning and still could not guarantee pairs under the accepted 64-byte ID maximum.

## Decision

Keep one direct card link per line and compress only the generated landing scaffold to three lines:

1. the catalog H1;
2. one generated/write line with README, question-search, and decision routes; and
3. one line containing all bounded title-part routes.

At 198 cards the landing is exactly 201 lines, or 89.73% of the unchanged 224-line health target: warning is
allowed, mandatory rollover is not crossed. With four parts, the title-route line is 199 raw bytes, 77.73% of the
unchanged 256-byte line target. The existing 32,768-byte target remains ample under the accepted worst-case ID
calculation.

The checker must render a synthetic 198-card projection using maximum-width IDs and title cells plus the longest
status, and reject the implementation if either landing or title parts cross mandatory pressure. The separate
199-card case must continue to fail the exact capacity ceiling.

For the current 158 cards, the migrated landing is 161 lines / 11,984 bytes / 190 widest-line bytes at SHA-256
`f67ee214e3622b700c818ad9b8d72e4ae19af15a02836c2b127545db9d10b5f5`. The unchanged three title parts bring
the complete projection to 340 lines / 46,890 bytes.

## Consequences

- The advertised 198-card capacity is executable under both ceilings and mandatory-rollover policy.
- Direct ID membership, stable paths, title browsing, navigation depth, generated notices, and writer commands
  remain present; only verbose explanatory scaffold prose is removed from the generated landing.
- No health target, warning/rollover milestone, enforcement ceiling, card limit, or part limit changes.
- Future scaffold growth must keep the exact-capacity pressure case green or make a new bounded decision.

## Links

- `docs/decisions/0020-bounded-fact-card-browse-projection.md`
- `docs/decisions/0022-fact-catalog-parts-pack-below-warning.md`
- `docs/tasks/FACT-CARD-CATALOG-CONTAINMENT.md`
- `doctrine/live_document_size/fact_card_catalog.json`
- `scripts/check_fact_card_catalog.pl`
