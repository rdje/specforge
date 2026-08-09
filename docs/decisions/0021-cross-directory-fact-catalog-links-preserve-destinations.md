---
id: cross-directory-fact-catalog-links-preserve-destinations
date: 2026-08-09
status: accepted
scope: documentation, knowledge-map, generated-projection, links, retrieval
---

# ADR 0021: Cross-directory fact rows preserve semantics and destinations, not relative link bytes

## Context

ADR 0020 correctly separates the stable fact-card landing from detailed title parts under
`docs/knowledge-catalog/`, but its byte-preservation clause is internally inconsistent with that move. The legacy
rows live in `docs/knowledge/INDEX.md` and link cards as `(card-id.md)`. Copying those bytes into the sibling
directory would resolve to nonexistent `docs/knowledge-catalog/card-id.md` rather than the canonical
`docs/knowledge/card-id.md`.

Adding redirect stubs, a non-portable HTML base element, or broken links would preserve bytes at the cost of the
retrieval contract. Moving title parts into the canonical-card directory would mix generated projections with
canonical facts under one live-document surface and consume the card collection's file capacity. None is an
acceptable implementation shortcut.

## Decision

Supersede only ADR 0020's claim that detailed Markdown rows remain byte-identical after the cross-directory move.
The stable landing, separate title-part directory, derived 198-card maximum, limits, writer transaction, and
two-stage migration remain accepted. ADR 0022 subsequently supersedes the original 64-card packing count with
56 cards per part without changing those bounds.

For each legacy row, the migration checker extracts and pins this semantic tuple:

```text
(card id, canonical card path, establishment date, status, compacted title)
```

The migrated title part renders the same tuple in the same order and changes only the relative destination:

```text
legacy:   [card-id](card-id.md)
migrated: [card-id](../knowledge/card-id.md)
```

The checker resolves every migrated link from its containing part and requires the exact canonical
`docs/knowledge/<card-id>.md` destination. It also compares ID, date, status, and compacted title values exactly;
no field can disappear or change under the guise of path normalization. The committed legacy monolith remains
byte-exact provenance through its Git blob, SHA-256, metrics, and ordered legacy-row hash.

The link prefix adds 13 bytes per detailed row. The migrated-row ceiling is therefore 333 bytes while the legacy
row ceiling remains 320. This did not change ADR 0020's part surface limits: 64 worst-case migrated rows plus the
then-modeled fixed scaffold remained below 24,576 bytes, and 333 remained below the 384-byte health target. At
this decision's committed boundary, the corrected in-memory result was three parts totaling 176 lines / 34,720
bytes, with a 14,306-byte largest part and 268-byte widest line; the landing was 172 lines / 12,361 bytes. ADR
0022 later found that line pressure, not bytes, required 56-card packing before implementation.

## Consequences

- Every migrated title link works from its actual directory and resolves to the same canonical card as before.
- The migration is information-lossless but not byte-identical at the generated row level; exact legacy bytes
  remain independently pinned rather than masquerading as portable links.
- No new surface, threshold, ceiling, card path, title-part path, or navigation-depth decision changes.
- `FACT-CARD-CATALOG-CONTAINMENT.2.1.2` may implement the checker only after this correction is committed.

## Links

- `docs/decisions/0020-bounded-fact-card-browse-projection.md`
- `docs/decisions/0022-fact-catalog-parts-pack-below-warning.md`
- `docs/tasks/FACT-CARD-CATALOG-CONTAINMENT.md`
- `docs/knowledge/INDEX.md`
- `docs/knowledge/fact-card-catalog.md`
