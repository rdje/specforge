---
id: fact-catalog-parts-pack-below-warning
date: 2026-08-09
status: accepted
scope: documentation, knowledge-map, generated-projection, containment, pressure
---

# ADR 0022: Fact-card title parts pack below the existing warning milestone

> **Correction:** ADR 0023 later supersedes only the landing scaffold and landing/combined metrics below. The
> 56-card title-part packing and all three current part hashes/metrics remain unchanged.

## Context

The executable `legacy_locked` checker implemented ADRs 0020 and 0021 before writing any migrated output. Its
first 64-card renderer produced nine fixed scaffold lines plus 64 rows: 73 lines, or 91.25% of the accepted
80-line health target. That crosses the project's unchanged 90% mandatory-rollover milestone. Removing two
nonessential scaffold lines reduced a full part to 71 lines, but 88.75% still begins every full part inside the
warning zone.

Byte pressure was not the controlling dimension: even a 64-row current part remained comfortably below the
24,576-byte health target. Raising the line target or ceiling would hide an avoidable design defect and violate
this containment program's non-goal. The partition count can instead be tightened while retaining the accepted
four-part maximum: `ceil(198 / 56) = 4`.

## Decision

Supersede only ADR 0020's 64-card packing count. Sort cards by ID exactly as decided, but pack no more than 56
cards into each deterministic `titles-NNNN.md` part. Keep the minimal seven-line scaffold. A full part is thus 63
lines, 78.75% of the existing 80-line health target, below the 80% warning milestone.

All accepted paths, authorities, semantic and destination preservation, maximum card count, maximum part count,
health targets, rollover milestones, ceilings, writer transaction, and migration stages remain unchanged. The
checker treats 90% health pressure as an error requiring rollover and 80% pressure as a warning; neither is a
data-controlled threshold.

At this decision's boundary, the deterministic 158-card plan was:

| Output | Lines | Bytes | Widest line |
| --- | ---: | ---: | ---: |
| bounded landing | 178 | 12,390 | 113 |
| `titles-0001.md` | 63 | 12,721 | 268 |
| `titles-0002.md` | 63 | 12,025 | 244 |
| `titles-0003.md` | 53 | 10,160 | 254 |

The three title parts total 179 lines / 34,906 bytes. The complete four-file projection is 357 lines / 47,296
bytes, and every measured dimension is below warning.

## Consequences

- A newly full title part does not immediately demand the rollover that created it.
- The 198-card surface capacity still fits in four parts, so no path or aggregate contract changes.
- More rows may move between generated parts after an insertion, but the deterministic output remains bounded.
- The checker catches both the exact 198-card ceiling and any future 90% health crossing before migration or
  commit.

## Links

- `docs/decisions/0020-bounded-fact-card-browse-projection.md`
- `docs/decisions/0021-cross-directory-fact-catalog-links-preserve-destinations.md`
- `docs/tasks/FACT-CARD-CATALOG-CONTAINMENT.md`
- `doctrine/live_document_size/fact_card_catalog.json`
- `scripts/check_fact_card_catalog.pl`
