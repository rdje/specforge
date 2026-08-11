---
id: no-collection-may-declare-an-aggregate-below-its-own-legal-maximum
title: No collection may declare an aggregate below its own legal maximum, and pressure must name the wall
date: 2026-08-11
status: accepted
scope: documentation, containment, live-document-size, pressure, continuity
evidence: docs/tasks/LIVE-DOC-STOP-RISK.md; scripts/check_live_document_size.pl (validate_aggregate_reachability, validate_aggregate_composition_schema, headroom reporting); scripts/test_live_document_size.pl (81 cases); doctrine/live_document_size/surfaces.jsonl; doctrine/live_document_size/ceiling_increase_authorities.jsonl
answers:
  - "what is ADR 0032"
  - "why must a collection aggregate be at least files times per-file"
  - "what is aggregate_composition in surfaces.jsonl"
  - "how does a heterogeneous collection declare its legal maximum"
  - "why does the live-document report show lines below the ceiling"
  - "why did the task_evidence aggregate ceiling become 480000"
---

# ADR 0032: No collection may declare an aggregate below its own legal maximum, and pressure must name the wall

## Context

[ADR 0029](0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md) established
the rule for the fact plane: *pressure belongs on a dimension that has a remedy*, so a canonical collection's
aggregate becomes its file bound times its per-file bound. The closing audit of that work then measured the
same shape on ten other surfaces and recorded it as `LIVE-DOC-STOP-RISK` Finding 1, but the rule itself was
still a fact about one plane, applied by hand, checked by nothing.

Enumerating **every** multi-file surface — 20 of them — found eleven with an aggregate below `files ×
per-file` on at least one band, one more than the original finding, and one of those eleven legitimately so:

| Surface | files × per-file | Aggregate lines before | After |
| --- | --- | ---: | ---: |
| `task_evidence` | 160 × 3,000 | 40,000 | 480,000 |
| `corpus_knowledge_base` | 40 × 1,200 | 4,000 | 48,000 |
| `kg_fixture_documents` | 256 × 80 | 4,000 | 20,480 |
| `research_records` | 64 × 640 | 12,000 | 40,960 |
| `workflow_standards` | 16 × 700 | 3,000 | 11,200 |
| `fsmgen_issue_packets` | 32 × 512 | 5,000 | 16,384 |
| `canonical_collection_indexes` | 9 × 384 | 1,024 | 3,456 |
| `active_task_evidence_parts` | 24 × 896 | 9,600 | 21,504 |
| `corpus_task_evidence_parts` | 16 × 896 | 6,144 | 14,336 |
| `knowledge_map_bundle` | 8 × 512 | 2,000 | 4,096 |
| `achievement_status_archive_segments` | 20 × 131,072 bytes | 2,097,152 (health only) | 2,621,440 |
| `fact_index` | 33 × 384 = 12,672 | 12,384 | **unchanged — see decision 2** |

`task_evidence` shows why this is not academic. Its 40,000-line aggregate binds at a mean of 250 lines per
tree; the measured mean is 229.7 across 131 trees. Its **file count is already at 81.9% warning**, and the
declared remedy for a count is to add capacity — but raising the count to, say, 200 would have produced
capacity the aggregate could not accommodate (200 × 229.7 = 45,940). The advertised capacity would have been
unreachable exactly as the fact plane's 198 cards were.

The tree's Finding 1 proposed keeping a tight aggregate "where a remedy exists — for a surface with an
archive or bounded-parts route, breaching the total is what triggers the migration." Testing that against
each surface, no case survives. A bounded-parts route lets a tree *add* parts; it is not triggered by, and
does not reduce, the total. A completed tree's archive migration replaces a monolith with a bounded root,
which does reduce lines — but it is triggered by the per-file bound, not the aggregate, and it is unavailable
to a collection of many medium-sized legal members, which is what all ten actually are. So the honest answer
is that none of the ten had a remedy its aggregate triggered, and the escape hatch the finding imagined has
no user.

Finding 2 left a second question: whether pressure should be ranked by distance to the enforcement ceiling
rather than by percentage of health target. `ROADMAP.md` reported `142.2% of health` while sitting 20 lines
from a hard stop — the loudest number was the least urgent fact about the surface.

## Decision

**1. The reachability rule is mechanical, not editorial.** For every surface with `locator: collection`, both
bands must satisfy `lines_total ≥ files × lines_each` and `bytes_total ≥ files × bytes_each`. A breach is a
gate failure naming the surface, the band, and the arithmetic. A file locator is exempt because it holds one
document, where the aggregate merely repeats the per-file bound (ADR 0028).

**2. The one exemption is a declared, summed partition — not an assertion.** A heterogeneous collection may
declare `aggregate_composition`: a rationale plus at least two member roles, each with a count and explicit
`health`/`ceiling` line and byte bounds. The gate then requires the counts to sum to the `files` bound, the
products to sum *exactly* to each `lines_total`/`bytes_total`, and the largest member to equal `lines_each` /
`bytes_each`. `fact_index` is the sole user and the reason the exemption exists: its legal maximum is one
96-line landing plus 32 × 384-line shards = 12,384, not 33 × 384 = 12,672. Because the declaration must
reproduce the declared totals arithmetically, it cannot become a place to park a number.

**3. The ten aggregates are re-derived, each with its own consumed authority.** Every raise carries a
`ceiling_increase_authorities` record with the exact old and new ceiling objects and a rationale naming why
that surface's members can never leave. Those records are retired immediately afterwards by
`LIVE-DOC-STOP-RISK.1a`, because an authority that outlives its increase is banked and the gate rejects it.

**4. Pressure reports the distance to the wall, not only the percentage of the target.** Every warning and
rollover line now ends with `— N below its M ceiling`. This is a reporting change, not a milestone-semantics
change: no threshold, band, or lifecycle moves.

**5. Ranking findings by urgency is declined, with the reason recorded.** The report stays ordered by surface
id so consecutive runs diff cleanly and a reader can find a known surface. Absolute headroom already conveys
urgency at each line, which was the actual gap; reordering would have traded a real property for a
presentational one.

## Consequences

- The trap class is closed by construction. A future surface cannot silently declare an aggregate its own
  files can exceed — the gate rejects the registry before any document is measured.
- Aggregate warnings that were noise are gone; the surviving warnings are per-file bounds and file counts,
  which is the intended split from ADR 0029: totals guarantee reachability, per-file bounds and counts carry
  the signal. `task_evidence` can now be given the file capacity its 81.9% warning calls for.
- `scripts/test_live_document_size.pl` grows from 68 to 81 cases. The new ones cover a tight line aggregate,
  a tight byte aggregate, a tight health band with a reachable ceiling, the file-locator exemption, a valid
  composition, and compositions that miss the total, the file bound, or the per-file bound, plus a
  single-role composition, a missing rationale, a repeated role, an unknown field, a member missing a band,
  and the headroom annotation itself.
- The fixture's own `generous_dimensions` had to be repaired: it declared 16 × 100 lines against a 500-line
  total, so the harness that proves the rule was violating it. That is the cheapest possible evidence that
  the shape is easy to write by accident.
- One honest gap, stated rather than hidden: only `enforcement_ceilings` increases require an authority
  record. Raising a *health target* is ungated, and re-deriving these aggregates raised health targets too,
  which lowers reported pressure. That is correct here — the pressure it removes was measuring an
  unreachable bound — but the asymmetry is real and unowned.

## Links

- Task tree: [`LIVE-DOC-STOP-RISK`](../tasks/LIVE-DOC-STOP-RISK.md)
- Generalizes: [ADR 0029](0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
- Builds on: [ADR 0028](0028-a-file-locator-means-one-file-and-aggregate-ceilings-must-warn.md)
- Same rule applied to one surface's sections:
  [ADR 0031](0031-a-bounded-snapshot-bounds-its-sections-not-just-its-file.md)
